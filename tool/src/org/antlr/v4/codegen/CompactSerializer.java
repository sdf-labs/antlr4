/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen;

import java.util.ArrayList;
import java.util.Base64;
import java.util.List;

/**
 * Compact serialization of integer streams for embedding in generated code:
 * each value is zigzag-mapped ({@code (v << 1) ^ (v >> 31)}, so small
 * negative values stay small), encoded as an unsigned LEB128 varint, and the
 * resulting byte stream is base64-encoded (standard alphabet, with padding)
 * and split into fixed-width text lines that render as one string literal
 * each - editor-friendly, and each literal stays far below Java's
 * 65535-byte string constant limit.
 *
 * <p>Used for the Rust target's {@code _serializedATN} and for the static
 * DFA prediction tables ({@code -Xstatic-dfa}); decoders live in the target
 * runtimes ({@code serialized_ints} in Rust,
 * {@code StaticDFATables} in Java).</p>
 */
public class CompactSerializer {
	/** Width of the emitted base64 lines. */
	public static final int LINE_CHARS = 80;

	/** Encode the values into base64 text lines of at most {@link #LINE_CHARS} chars. */
	public static List<String> encode(int[] values) {
		ByteBuffer buf = new ByteBuffer();
		for (int value : values) {
			int v = (value << 1) ^ (value >> 31); // zigzag
			while ((v & ~0x7F) != 0) {
				buf.add((byte)((v & 0x7F) | 0x80));
				v >>>= 7;
			}
			buf.add((byte)v);
		}
		String base64 = Base64.getEncoder().encodeToString(buf.toArray());
		List<String> lines = new ArrayList<String>((base64.length()+LINE_CHARS-1)/LINE_CHARS);
		for (int at = 0; at < base64.length(); at += LINE_CHARS) {
			lines.add(base64.substring(at, Math.min(at+LINE_CHARS, base64.length())));
		}
		return lines;
	}

	private static final class ByteBuffer {
		private byte[] data = new byte[8192];
		private int size;

		void add(byte b) {
			if (size == data.length) {
				byte[] bigger = new byte[data.length*2];
				System.arraycopy(data, 0, bigger, 0, size);
				data = bigger;
			}
			data[size++] = b;
		}

		byte[] toArray() {
			byte[] result = new byte[size];
			System.arraycopy(data, 0, result, 0, size);
			return result;
		}
	}
}
