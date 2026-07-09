//! Table-driven static SLL prediction.
//!
//! The ANTLR tool's `-Xstatic-dfa` option precomputes, at code-generation
//! time, the SLL prediction DFAs that `ParserATNSimulator` would otherwise
//! build lazily at runtime - for every decision where the static resolution
//! is provably behavior-preserving (conflict-free LL(k)/LL(*) decisions and
//! trusted exact ambiguities resolved to the minimum alternative).
//!
//! All of a parser's tables are appended to the compact `_serializedATN`
//! blob and decoded together with the ATN in one streaming pass (see
//! `ATNDeserializer::deserialize_compact`), landing in
//! [`crate::atn::ATN::static_dfas`]. Predictions then run through
//! [`crate::parser::BaseParser::dfa_predict`]: a plain table walk over
//! `LA(1..n)` with no ATN simulation and no per-prediction allocation.
//!
//! The blob is encoded with the scheme in [`crate::serialized_ints`];
//! the logical int stream layout of the tables section is:
//!
//! ```text
//! FORMAT_VERSION
//! numTables
//! numDecisionSlots                 (max decision number + 1)
//! for each table:
//!   decision
//!   numStates
//!   numEdgeInts
//!   accepts[numStates]             (predicted alt per state; 0 = non-accept)
//!   fallbacks[numStates]           (error-avoidance alt per state; 0 = none)
//!   edgeOffsets[numStates+1]       (index of each state's first edge i32)
//!   edges[numEdgeInts]             ((lo, hi, target) triples, lo-sorted per state)
//! ```

/// Format version understood by this runtime; must match the tool's
/// `SerializedStaticDFAs.FORMAT_VERSION`.
pub const FORMAT_VERSION: i32 = 3;

/// The deserialized static prediction tables of one generated parser.
///
/// Table data is stored in a single flat buffer; per-decision lookup is a
/// direct index. State `0` of each table is its start state;
/// `accepts[s] > 0` marks a terminal accept state predicting that
/// alternative (numbering identical to `adaptive_predict`'s return value).
/// A lookahead token matching no edge means no viable alternative.
///
/// Cyclic tables (LL(*) decisions, e.g. dotted-name scanning) are walked by
/// exactly the same loop: termination is guaranteed because every step
/// consumes one token of lookahead and the input is finite (EOF included in
/// the token space).
pub struct StaticDFATables {
    /// Concatenated per-table data: accepts, fallbacks, edge_offsets, edges.
    data: Vec<i32>,
    /// Per-table (accepts_at, fallbacks_at, edge_offsets_at, edges_at) indexes into `data`.
    metas: Vec<(usize, usize, usize, usize)>,
    /// decision number -> table index, or -1.
    decision_to_table: Vec<i32>,
}

/// Borrowed view of one decision's table.
#[derive(Copy, Clone)]
pub struct StaticDFATable<'a> {
    /// Predicted alternative per state; 0 = not an accept state.
    pub accepts: &'a [i32],
    /// Error-avoidance fallback per state; 0 = none. Returned when a
    /// lookahead token matches no edge, so the parser fails later with a
    /// more precise error at the mismatch point - mirroring
    /// `adaptive_predict`'s finished-decision-entry-rule recovery.
    pub fallbacks: &'a [i32],
    /// Index of each state's first edge `i32` in `edges`; `num_states + 1` entries.
    pub edge_offsets: &'a [i32],
    /// Flattened `(lo, hi, target)` triples, sorted by `lo` within each state.
    pub edges: &'a [i32],
}

impl StaticDFATables {
    /// No tables: every lookup fails; only reachable from hand-written code
    /// since generated parsers only call `dfa_predict` for decisions they
    /// serialized tables for.
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
            metas: Vec::new(),
            decision_to_table: Vec::new(),
        }
    }

    /// Build the tables from the (already varint-decoded) logical int
    /// stream, consuming it to the end - used by
    /// `ATNDeserializer::deserialize_compact` for the section that follows
    /// the ATN in the combined blob.
    ///
    /// Panics on malformed input or a format-version mismatch: the blob is
    /// generated together with the parser that embeds it, so any failure is
    /// a build inconsistency, not a runtime condition.
    pub fn from_int_stream(ints: &mut impl Iterator<Item = i32>) -> Self {
        let mut next = || ints.next().expect("truncated static DFA table data");

        let version = next();
        assert_eq!(
            version, FORMAT_VERSION,
            "static DFA table format version mismatch: parser was generated \
             with a different ANTLR tool version, please regenerate"
        );
        let num_tables = next() as usize;
        let num_slots = next() as usize;

        let mut data = Vec::new();
        let mut metas = Vec::with_capacity(num_tables);
        let mut decision_to_table = vec![-1i32; num_slots];

        for table in 0..num_tables {
            let decision = next() as usize;
            let num_states = next() as usize;
            let num_edge_ints = next() as usize;
            decision_to_table[decision] = table as i32;

            let accepts_at = data.len();
            for _ in 0..num_states {
                data.push(next());
            }
            let fallbacks_at = data.len();
            for _ in 0..num_states {
                data.push(next());
            }
            let edge_offsets_at = data.len();
            for _ in 0..num_states + 1 {
                data.push(next());
            }
            let edges_at = data.len();
            for _ in 0..num_edge_ints {
                data.push(next());
            }
            metas.push((accepts_at, fallbacks_at, edge_offsets_at, edges_at));
        }
        drop(next);
        assert!(ints.next().is_none(), "trailing data in static DFA blob");

        Self {
            data,
            metas,
            decision_to_table,
        }
    }

    /// The table for a decision; panics if the decision has no static table
    /// (generated code only references decisions it has tables for).
    #[inline]
    pub fn table(&self, decision: i32) -> StaticDFATable<'_> {
        let t = self.decision_to_table[decision as usize];
        assert!(t >= 0, "no static DFA table for decision {}", decision);
        let (accepts_at, fallbacks_at, edge_offsets_at, edges_at) = self.metas[t as usize];
        let end = self
            .metas
            .get(t as usize + 1)
            .map(|m| m.0)
            .unwrap_or(self.data.len());
        StaticDFATable {
            accepts: &self.data[accepts_at..fallbacks_at],
            fallbacks: &self.data[fallbacks_at..edge_offsets_at],
            edge_offsets: &self.data[edge_offsets_at..edges_at],
            edges: &self.data[edges_at..end],
        }
    }
}

impl<'a> StaticDFATable<'a> {
    /// Predicted alternative if `state` is an accept state.
    #[inline]
    pub fn accept(&self, state: usize) -> Option<i32> {
        let alt = self.accepts[state];
        if alt > 0 {
            Some(alt)
        } else {
            None
        }
    }

    /// Error-avoidance fallback alternative of `state`, if any.
    #[inline]
    pub fn fallback(&self, state: usize) -> Option<i32> {
        let alt = self.fallbacks[state];
        if alt > 0 {
            Some(alt)
        } else {
            None
        }
    }

    /// Successor of `state` on token type `t`, if any (binary search).
    #[inline]
    pub fn edge(&self, state: usize, t: i32) -> Option<usize> {
        let mut lo = (self.edge_offsets[state] / 3) as usize;
        let mut hi = (self.edge_offsets[state + 1] / 3) as usize;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if t < self.edges[mid * 3] {
                hi = mid;
            } else if t > self.edges[mid * 3 + 1] {
                lo = mid + 1;
            } else {
                return Some(self.edges[mid * 3 + 2] as usize);
            }
        }
        None
    }
}
