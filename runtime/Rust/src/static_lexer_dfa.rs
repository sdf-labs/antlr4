//! Statically-precomputed lexer DFA tables (`-Xstatic-dfa` on a lexer
//! grammar) and the table walker that tokenizes off them.
//!
//! When the tool can prove a lexer needs no runtime input-dependent
//! machinery (no semantic predicates, no position-dependent lexer actions),
//! it fully expands the DFA the lazy [`crate::lexer_atn_simulator`] would
//! otherwise build at runtime - by driving the reference simulator over the
//! whole input alphabet, so maximal-munch, rule-priority, non-greedy, and
//! lexer-action semantics are identical by construction. Fully-static
//! lexers embed *only* the tables - no ATN at all (see
//! [`StaticLexerTables`]); lexers generated before that split carry them
//! appended to the serialized ATN blob (see the tool's `LexerDFABuilder`
//! and `SerializedStaticLexerDFAs` for the stream format).
//!
//! The walker in [`crate::lexer_atn_simulator::LexerATNSimulator`] then
//! runs one tight loop per token: a dense per-state row indexed directly by
//! the next input byte (symbols 0..=127), a single EOF edge, and a short
//! sorted interval list for symbols above 127. No ATN simulation, no
//! config-set allocation, no scratch arena, and no lazily-built DFA states.
//! Lexers generated without tables (or with esoteric features) keep the
//! general simulator path unchanged.

const FORMAT_VERSION: i32 = 3;

/// Dense-row width: symbols 0..=127 are direct-indexed.
pub(crate) const ASCII_ROW: usize = 128;

/// "No edge" sentinel in the u16 target rows. Distinct from every valid
/// entry: state ids are capped at 0x7FFE, so `ACCEPT_BIT | state` peaks at
/// 0xFFFE.
pub(crate) const NO_EDGE: u16 = u16::MAX;

/// Bit OR-ed into every serialized edge target whose target is an accept
/// state: the per-char walk loop reads target state and accept info from
/// one table entry.
pub(crate) const ACCEPT_BIT: u16 = 0x8000;

/// Mask extracting the target state from an edge entry.
pub(crate) const STATE_MASK: u16 = 0x7FFF;

/// All static lexer DFA tables of a grammar, one per lexer mode.
#[derive(Debug)]
pub struct StaticLexerDFATables {
    modes: Vec<StaticLexerDFA>,
}

/// One mode's table.
#[derive(Debug)]
pub struct StaticLexerDFA {
    start_state: u16,
    /// `num_states * 128` dense targets for symbols 0..=127; NO_EDGE = none.
    ascii: Vec<u16>,
    /// EOF target per state; NO_EDGE = none.
    eof: Vec<u16>,
    /// Index of each state's first triple in `hi_edges`; `num_states + 1`.
    hi_offsets: Vec<u32>,
    /// (lo, hi, target) triples, lo-sorted per state, lo > 127.
    hi_edges: Vec<(u32, u32, u16)>,
    /// Token type per accept state; -1 = not an accept state. 0 is a real
    /// accept type: rules with a `more`/`type(...)` command get no token
    /// type assigned by the tool.
    accept_type: Vec<i32>,
    /// Offset of each accept state's action list in `action_lists`; -1 = none.
    accept_actions: Vec<i32>,
    /// Length-prefixed lists of lexer action indices (into the enclosing
    /// `ATN::lexer_actions` for ATN-embedded tables, or into
    /// [`StaticLexerTables::lexer_actions`] for standalone ones).
    action_lists: Vec<i32>,
}

impl StaticLexerDFATables {
    /// No tables: every mode lookup fails and the lexer keeps its lazy
    /// simulator. The only state for blobs generated without `-Xstatic-dfa`
    /// or for lexers the tool could not prove static.
    pub fn empty() -> Self {
        Self { modes: Vec::new() }
    }

    /// Build the tables from the (already varint-decoded) logical int
    /// stream, consuming it to the end - used by
    /// `ATNDeserializer::deserialize_compact` for the section that follows
    /// a lexer ATN in the combined blob.
    ///
    /// Panics on malformed input or a format-version mismatch: the blob is
    /// generated together with the lexer that embeds it, so any failure is
    /// a build inconsistency, not a runtime condition.
    pub fn from_int_stream(ints: &mut impl Iterator<Item = i32>) -> Self {
        let mut next = || ints.next().expect("truncated static lexer DFA table data");
        let version = next();
        assert_eq!(
            version, FORMAT_VERSION,
            "static lexer DFA table format version mismatch: lexer was generated \
             with a different ANTLR tool version, please regenerate"
        );
        Self::read_modes(&mut next)
    }

    /// Read the `numModes` + per-mode table section from `next` - the
    /// layout shared by the ATN-embedded (v3) stream and the standalone
    /// ATN-less (v4) stream.
    fn read_modes(next: &mut impl FnMut() -> i32) -> Self {
        let num_modes = next() as usize;
        let mut modes = Vec::with_capacity(num_modes);
        for _ in 0..num_modes {
            let num_states = next() as usize;
            assert!(
                num_states <= STATE_MASK as usize,
                "static lexer DFA has {num_states} states, above the table limit {}",
                STATE_MASK as usize
            );
            let start_state = next() as u16;
            let mut accept_type = Vec::with_capacity(num_states);
            let mut accept_actions = Vec::with_capacity(num_states);
            let mut eof = Vec::with_capacity(num_states);
            let mut ascii = Vec::with_capacity(num_states * ASCII_ROW);
            let mut hi_offsets = Vec::with_capacity(num_states + 1);
            for _ in 0..num_states {
                accept_type.push(next());
            }
            for _ in 0..num_states {
                accept_actions.push(next());
            }
            for _ in 0..num_states {
                eof.push(edge(next()));
            }
            for _ in 0..num_states * ASCII_ROW {
                ascii.push(edge(next()));
            }
            for _ in 0..=num_states {
                hi_offsets.push(next() as u32);
            }
            let num_hi = next() as usize;
            debug_assert_eq!(num_hi % 3, 0);
            let mut hi_edges = Vec::with_capacity(num_hi / 3);
            for _ in 0..num_hi / 3 {
                let lo = next() as u32;
                let hi = next() as u32;
                hi_edges.push((lo, hi, next() as u16));
            }
            // Validate everything the walker's unchecked accesses rely on:
            // the start state and every edge target are valid state ids,
            // and the hi-edge offsets are monotonic and in range. After
            // this, `edge`/`accept_type` may index without bounds checks.
            assert!((start_state as usize) < num_states);
            assert!(ascii.iter().all(|&t| t == NO_EDGE || ((t & STATE_MASK) as usize) < num_states));
            assert!(eof.iter().all(|&t| t == NO_EDGE || ((t & STATE_MASK) as usize) < num_states));
            assert!(hi_edges.iter().all(|&(_, _, t)| ((t & STATE_MASK) as usize) < num_states));
            assert_eq!(hi_offsets.first(), Some(&0));
            assert_eq!(hi_offsets.last(), Some(&(num_hi as u32 / 3)));
            assert!(hi_offsets.windows(2).all(|w| w[0] <= w[1]));
            let num_action_ints = next() as usize;
            let mut action_lists = Vec::with_capacity(num_action_ints);
            for _ in 0..num_action_ints {
                action_lists.push(next());
            }
            modes.push(StaticLexerDFA {
                start_state,
                ascii,
                eof,
                hi_offsets,
                hi_edges,
                accept_type,
                accept_actions,
                action_lists,
            });
        }
        Self { modes }
    }

    /// The table for lexer mode `m`, if one was serialized.
    #[inline]
    pub fn mode(&self, m: usize) -> Option<&StaticLexerDFA> {
        self.modes.get(m)
    }

    /// Number of serialized mode tables.
    pub fn num_modes(&self) -> usize {
        self.modes.len()
    }
}

/// Everything an ATN-less generated lexer needs at runtime: the static
/// mode tables plus the (position-independent) lexer actions the accept
/// states reference.
///
/// When the tool proves a lexer fully static (`-Xstatic-dfa`), the
/// generated code embeds *only* this stream - no ATN at all - and drives
/// the simulator off it (see
/// [`crate::atn_simulator::LexerATNSimulatorManager::new_static_lexer`]).
/// Accept-action indices in the tables resolve against
/// [`StaticLexerTables::lexer_actions`] here (for lexers generated before
/// this split - tables embedded after the ATN, format v3 - they resolve
/// against `ATN::lexer_actions` instead).
///
/// Standalone stream layout: [`TABLES_FORMAT_VERSION`], then the same
/// `numModes` + per-mode table section as the embedded v3 stream, then
/// `numActions` and, per action, the `(type, data1, data2)` triple the
/// ATN deserializer uses for lexer actions.
#[derive(Debug)]
pub struct StaticLexerTables {
    dfas: StaticLexerDFATables,
    lexer_actions: Vec<crate::lexer_action::LexerAction<'static>>,
}

const TABLES_FORMAT_VERSION: i32 = 4;

impl StaticLexerTables {
    /// Decode the standalone (ATN-less) tables blob embedded in generated
    /// lexer code, in one streaming pass with no intermediate buffer.
    ///
    /// Panics on malformed input or a format-version mismatch: the blob is
    /// generated together with the lexer that embeds it, so any failure is
    /// a build inconsistency, not a runtime condition.
    pub fn deserialize_compact(segments: &[&str]) -> Self {
        Self::from_int_stream(&mut crate::serialized_ints::Decoder::new(segments))
    }

    /// Build from the (already varint-decoded) logical int stream,
    /// consuming it to the end.
    pub fn from_int_stream(ints: &mut impl Iterator<Item = i32>) -> Self {
        let mut next = || ints.next().expect("truncated static lexer table data");
        let version = next();
        assert_eq!(
            version, TABLES_FORMAT_VERSION,
            "static lexer table format version mismatch: lexer was generated \
             with a different ANTLR tool version, please regenerate"
        );
        let dfas = StaticLexerDFATables::read_modes(&mut next);
        let num_actions = next() as usize;
        let mut lexer_actions = Vec::with_capacity(num_actions);
        for _ in 0..num_actions {
            let (ty, data1, data2) = (next(), next(), next());
            lexer_actions.push(crate::lexer_action::LexerAction::from_serialized(ty, data1, data2));
        }
        Self { dfas, lexer_actions }
    }

    /// The table for lexer mode `m`, if one was serialized.
    #[inline]
    pub fn mode(&self, m: usize) -> Option<&StaticLexerDFA> {
        self.dfas.mode(m)
    }

    /// The lexer actions the tables' accept-action indices reference.
    #[inline]
    pub(crate) fn lexer_actions(&self) -> &[crate::lexer_action::LexerAction<'static>] {
        &self.lexer_actions
    }
}

/// Decode one serialized edge target: -1 is "no edge".
#[inline]
fn edge(v: i32) -> u16 {
    if v < 0 {
        NO_EDGE
    } else {
        v as u16
    }
}

impl StaticLexerDFA {
    #[inline]
    pub fn start_state(&self) -> u16 {
        self.start_state
    }

    /// Raw edge entry of state `s` on input symbol `t` (`-1` = EOF):
    /// `NO_EDGE` when there is no edge, else `target | ACCEPT_BIT?` (the
    /// bit marks accept states, so the per-char walk loop needs only this
    /// one table load per input symbol).
    ///
    /// No bounds checks: every `s` the walker produces is a valid state id
    /// (the start state and all serialized targets are validated at load
    /// time), and the symbol branches self-limit their indexes.
    #[inline]
    pub fn edge_entry(&self, s: u16, t: i32) -> u16 {
        unsafe {
            if t < 0 {
                *self.eof.get_unchecked(s as usize)
            } else if (t as usize) < ASCII_ROW {
                *self
                    .ascii
                    .get_unchecked(s as usize * ASCII_ROW + t as usize)
            } else {
                let t = t as u32;
                let lo = *self.hi_offsets.get_unchecked(s as usize) as usize;
                let hi = *self.hi_offsets.get_unchecked(s as usize + 1) as usize;
                match self.hi_edges[lo..hi].binary_search_by(|&(l, h, _)| {
                    if t < l {
                        std::cmp::Ordering::Greater
                    } else if t > h {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }) {
                    Ok(i) => self.hi_edges[lo + i].2,
                    Err(_) => NO_EDGE,
                }
            }
        }
    }

    /// Token type produced when accepting in state `s`; -1 = not an accept state.
    ///
    /// Same unchecked-index safety argument as [`StaticLexerDFA::edge_entry`].
    #[inline]
    pub fn accept_type(&self, s: u16) -> i32 {
        unsafe { *self.accept_type.get_unchecked(s as usize) }
    }

    /// Lexer action indices to execute when accepting in state `s`
    /// (e.g. `skip`, `mode(X)`), in order; empty slice when none.
    #[inline]
    pub fn accept_actions(&self, s: u16) -> &[i32] {
        let off = self.accept_actions[s as usize];
        if off < 0 {
            &[]
        } else {
            let off = off as usize;
            let len = self.action_lists[off] as usize;
            &self.action_lists[off + 1..off + 1 + len]
        }
    }
}
