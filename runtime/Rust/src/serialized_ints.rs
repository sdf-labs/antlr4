//! Decoding of compact serialized integer streams embedded in generated
//! code (the tool's `CompactSerializer`): each value is zigzag-mapped,
//! LEB128-varint encoded, and the byte stream is base64-encoded and split
//! into string segments (rendered as one ~80-column literal per line).
//!
//! Used for the Rust target's `_serializedATN`, which carries the ATN
//! followed by the optional static DFA prediction tables (`-Xstatic-dfa`)
//! as a trailing section; both are decoded in one streaming pass with no
//! intermediate buffers. Decoding failures panic: these blobs are generated
//! together with the code that embeds them, so any failure is a build
//! inconsistency, not a runtime condition.

/// A streaming decoder over base64/zigzag-varint segments, yielding the
/// original integer stream one value at a time.
pub struct Decoder<'a> {
    segments: &'a [&'a str],
    segment: usize,
    at: usize,
    /// Base64 bit accumulator.
    acc: u32,
    bits: u32,
}

impl<'a> Decoder<'a> {
    pub fn new(segments: &'a [&'a str]) -> Self {
        Self {
            segments,
            segment: 0,
            at: 0,
            acc: 0,
            bits: 0,
        }
    }

    /// Next byte of the base64-decoded stream, if any.
    fn next_byte(&mut self) -> Option<u8> {
        while self.bits < 8 {
            let c = loop {
                match self.segments.get(self.segment) {
                    None => return None,
                    Some(segment) => match segment.as_bytes().get(self.at) {
                        None => {
                            self.segment += 1;
                            self.at = 0;
                        }
                        Some(&c) => {
                            self.at += 1;
                            break c;
                        }
                    },
                }
            };
            let v = match c {
                b'A'..=b'Z' => c - b'A',
                b'a'..=b'z' => c - b'a' + 26,
                b'0'..=b'9' => c - b'0' + 52,
                b'+' => 62,
                b'/' => 63,
                b'=' => continue,
                _ => panic!("invalid base64 character in serialized data: {}", c as char),
            };
            self.acc = (self.acc << 6) | v as u32;
            self.bits += 6;
        }
        self.bits -= 8;
        Some((self.acc >> self.bits) as u8)
    }
}

impl<'a> Iterator for Decoder<'a> {
    type Item = i32;

    /// Next logical value: un-zigzagged LEB128 varint.
    fn next(&mut self) -> Option<i32> {
        let mut value: u32 = 0;
        let mut shift = 0;
        let mut first = true;
        loop {
            let b = match self.next_byte() {
                Some(b) => b,
                None if first => return None,
                None => panic!("truncated varint in serialized data"),
            };
            first = false;
            value |= ((b & 0x7F) as u32) << shift;
            if b & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        // un-zigzag
        Some(((value >> 1) as i32) ^ -((value & 1) as i32))
    }
}

/// Decode all segments into a vector (convenience/testing; prefer streaming
/// via [`Decoder`] in production paths).
pub fn decode(segments: &[&str]) -> Vec<i32> {
    Decoder::new(segments).collect()
}

#[cfg(test)]
mod test {
    use super::decode;

    #[test]
    fn round_trip_reference() {
        // encoded by the tool's CompactSerializer for [0, -1, 1, 300, -300, 123456789]
        let segments = ["AAEC2ATXBKq03nU="];
        assert_eq!(decode(&segments), vec![0, -1, 1, 300, -300, 123456789]);
    }

    #[test]
    fn multi_segment() {
        // same stream split mid-varint across segments
        let segments = ["AAEC2AT", "XBKq03nU="];
        assert_eq!(decode(&segments), vec![0, -1, 1, 300, -300, 123456789]);
    }

    #[test]
    fn empty() {
        assert_eq!(decode(&[]), Vec::<i32>::new());
    }
}
