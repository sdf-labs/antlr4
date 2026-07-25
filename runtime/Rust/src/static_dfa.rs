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
//!   decision                       (-1: referenced only via a dispatch below)
//!   numStates
//!   numEdgeInts
//!   accepts[numStates]             (predicted alt per state; 0 = non-accept;
//!                                    -1 = escape: defer to adaptive prediction)
//!   fallbacks[numStates]           (error-avoidance alt per state; 0 = none)
//!   edgeOffsets[numStates+1]       (index of each state's first edge i32)
//!   edges[numEdgeInts]             ((lo, hi, target) triples, lo-sorted per state)
//! numPrecedenceDispatches
//! for each dispatch:               (a left-recursive precedence loop decision)
//!   decision
//!   numCutoffs
//!   cutoffs[numCutoffs]            (sorted; class(p) = #cutoffs < p)
//!   tableIndex[numCutoffs+1]       (table of each precedence class;
//!                                    -1 = class dispatches to adaptive prediction)
//! ```
//!
//! Precedence dispatches serve the operator loops of left-recursive rules:
//! the loop's viable-operator set depends on the current precedence (the
//! `_p` argument of the rewritten rule), so one table per precedence
//! equivalence class is precomputed and the walker selects by
//! `get_precedence()` - the static analogue of the adaptive runtime's
//! per-precedence DFA start states, and the reason the generated loop
//! behaves like a hand-rolled precedence-climbing (Pratt) parser.

/// Format version understood by this runtime; must match the tool's
/// `SerializedStaticDFAs.FORMAT_VERSION`. v6 adds the per-table
/// alternative-mask section (prefix-factor groups).
pub const FORMAT_VERSION: i32 = 6;

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
    /// Per-table alternative-mask accepts (bit 1<<(alt-1) per live
    /// alternative of the covering prefix-factor group; 0 = none).
    masks: Vec<Vec<u64>>,
    /// decision number -> table index (`>= 0`), `-1` (no table), or
    /// `-(dispatch index) - 2` (precedence-dispatched decision).
    decision_to_table: Vec<i32>,
    /// Flattened dispatch entries: `numCutoffs, cutoffs..., tableIndex...`
    /// (`numCutoffs + 1` table indices).
    dispatch_data: Vec<i32>,
    /// Per-dispatch start offset into `dispatch_data`.
    dispatch_at: Vec<usize>,
    /// Per-table expanded edge lookup, built at load time from the compact
    /// interval triples in `data` (memory-only; the serialized blob is
    /// unchanged). `edge_min[t][s]` is state `s`'s base token, `edge_row_at[t]`
    /// its CSR offsets (`num_states + 1`), and `edge_targets[t]` the flat
    /// per-state rows: target state for each token in `[min, min+len)`,
    /// `-1` where the state has no edge. Gives O(1) `edge()`.
    edge_min: Vec<Vec<i32>>,
    edge_row_at: Vec<Vec<u32>>,
    edge_targets: Vec<Vec<i32>>,
}

/// Borrowed view of one decision's table.
#[derive(Copy, Clone)]
pub struct StaticDFATable<'a> {
    /// Predicted alternative per state; 0 = not an accept state.
    pub accepts: &'a [i32],
    /// Alternative-mask accept per state; 0 = none. A nonzero entry marks
    /// a terminal state where the walker returns the mask: the live
    /// alternatives are covered by one prefix-factor group, so the
    /// generated parser executes the group's shared prefix and resolves
    /// the choice with its tail decision.
    pub masks: &'a [u64],
    /// Error-avoidance fallback per state; 0 = none. Returned when a
    /// lookahead token matches no edge, so the parser fails later with a
    /// more precise error at the mismatch point - mirroring
    /// `adaptive_predict`'s finished-decision-entry-rule recovery.
    pub fallbacks: &'a [i32],
    /// State `s`'s base token: `edge()` indexes `edge_targets` at
    /// `edge_row_at[s] + (t - edge_min[s])`.
    pub edge_min: &'a [i32],
    /// CSR offsets into `edge_targets`, one per state plus a terminator.
    pub edge_row_at: &'a [u32],
    /// Flattened per-state direct-indexed edge rows: target state for each
    /// token in the state's `[min, min+len)` range, `-1` where no edge.
    pub edge_targets: &'a [i32],
}

impl StaticDFATables {
    /// No tables: every lookup fails; only reachable from hand-written code
    /// since generated parsers only call `dfa_predict` for decisions they
    /// serialized tables for.
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
            metas: Vec::new(),
            masks: Vec::new(),
            decision_to_table: Vec::new(),
            dispatch_data: Vec::new(),
            dispatch_at: Vec::new(),
            edge_min: Vec::new(),
            edge_row_at: Vec::new(),
            edge_targets: Vec::new(),
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
        let mut masks = Vec::with_capacity(num_tables);
        let mut edge_min: Vec<Vec<i32>> = Vec::with_capacity(num_tables);
        let mut edge_row_at: Vec<Vec<u32>> = Vec::with_capacity(num_tables);
        let mut edge_targets: Vec<Vec<i32>> = Vec::with_capacity(num_tables);
        let mut decision_to_table = vec![-1i32; num_slots];

        for table in 0..num_tables {
            let decision = next();
            let num_states = next() as usize;
            let num_edge_ints = next() as usize;
            if decision >= 0 {
                decision_to_table[decision as usize] = table as i32;
            }

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
            // Expand this table's compact interval triples into per-state
            // direct-indexed rows for O(1) edge lookup at parse time. Kept
            // out of the serialized blob (a memory-only expansion).
            {
                let eo = &data[edge_offsets_at..edges_at];
                let ed = &data[edges_at..edges_at + num_edge_ints];
                let mut t_min = vec![0i32; num_states];
                let mut t_row_at = vec![0u32; num_states + 1];
                let mut t_targets: Vec<i32> = Vec::new();
                for s in 0..num_states {
                    t_row_at[s] = t_targets.len() as u32;
                    let lo_tri = (eo[s] / 3) as usize;
                    let hi_tri = (eo[s + 1] / 3) as usize;
                    if lo_tri < hi_tri {
                        let mut smin = i32::MAX;
                        let mut smax = i32::MIN;
                        for tri in lo_tri..hi_tri {
                            smin = smin.min(ed[tri * 3]);
                            smax = smax.max(ed[tri * 3 + 1]);
                        }
                        t_min[s] = smin;
                        let base = t_targets.len();
                        t_targets.resize(base + (smax - smin + 1) as usize, -1);
                        for tri in lo_tri..hi_tri {
                            let tgt = ed[tri * 3 + 2];
                            for tok in ed[tri * 3]..=ed[tri * 3 + 1] {
                                t_targets[base + (tok - smin) as usize] = tgt;
                            }
                        }
                    }
                }
                t_row_at[num_states] = t_targets.len() as u32;
                edge_min.push(t_min);
                edge_row_at.push(t_row_at);
                edge_targets.push(t_targets);
            }
            // v6: alternative-mask section
            let num_mask_states = next() as usize;
            let mut table_masks = vec![0u64; num_states];
            for _ in 0..num_mask_states {
                let state = next() as usize;
                let num_alts = next() as usize;
                let mut bits = 0u64;
                for _ in 0..num_alts {
                    bits |= 1u64 << (next() - 1);
                }
                table_masks[state] = bits;
            }
            masks.push(table_masks);
            metas.push((accepts_at, fallbacks_at, edge_offsets_at, edges_at));
        }

        let num_dispatches = next() as usize;
        let mut dispatch_data = Vec::new();
        let mut dispatch_at = Vec::with_capacity(num_dispatches);
        for dispatch in 0..num_dispatches {
            let decision = next() as usize;
            decision_to_table[decision] = -(dispatch as i32) - 2;
            dispatch_at.push(dispatch_data.len());
            let num_cutoffs = next() as usize;
            dispatch_data.push(num_cutoffs as i32);
            for _ in 0..2 * num_cutoffs + 1 {
                dispatch_data.push(next());
            }
        }

        debug_assert!(ints.next().is_none(), "trailing data in static DFA blob");

        Self {
            data,
            metas,
            masks,
            decision_to_table,
            dispatch_data,
            dispatch_at,
            edge_min,
            edge_row_at,
            edge_targets,
        }
    }

    /// Is the decision precedence-dispatched (per-precedence-class tables
    /// of a left-recursive loop)? Such tables are built against the
    /// decision rule's compatible call sites and must not be consulted
    /// when the left-recursive rule itself is the parse entry (no caller
    /// frame): the adaptive engine then explores every FOLLOW link from
    /// the empty stack, a behavior the class tables deliberately prune.
    #[inline]
    pub fn is_precedence_dispatched(&self, decision: i32) -> bool {
        self.decision_to_table[decision as usize] <= -2
    }

    /// The table for a decision; `precedence` (the parser's current
    /// precedence, i.e. the top of its precedence stack) selects the
    /// precedence class of dispatched decisions and is ignored for plain
    /// ones. `None` means the selected precedence class has no static
    /// table and the prediction must run through the adaptive engine.
    /// Panics if the decision itself has no table entry at all (generated
    /// code only references decisions it has tables for).
    #[inline]
    pub fn table(&self, decision: i32, precedence: i32) -> Option<StaticDFATable<'_>> {
        let mut t = self.decision_to_table[decision as usize];
        if t <= -2 {
            // precedence dispatch: class(p) = #cutoffs < p
            let at = self.dispatch_at[(-t - 2) as usize];
            let num_cutoffs = self.dispatch_data[at] as usize;
            let cutoffs = &self.dispatch_data[at + 1..at + 1 + num_cutoffs];
            let class = cutoffs.iter().take_while(|&&c| c < precedence).count();
            t = self.dispatch_data[at + 1 + num_cutoffs + class];
            if t < 0 {
                // this precedence class dispatches to adaptive prediction
                return None;
            }
        }
        assert!(t >= 0, "no static DFA table for decision {}", decision);
        let (accepts_at, fallbacks_at, edge_offsets_at, _edges_at) = self.metas[t as usize];
        Some(StaticDFATable {
            accepts: &self.data[accepts_at..fallbacks_at],
            masks: &self.masks[t as usize],
            fallbacks: &self.data[fallbacks_at..edge_offsets_at],
            edge_min: &self.edge_min[t as usize],
            edge_row_at: &self.edge_row_at[t as usize],
            edge_targets: &self.edge_targets[t as usize],
        })
    }
}

/// `accepts` sentinel: an escape state of a hybrid table - the walker
/// defers the whole prediction to the adaptive engine (the table never
/// consumes input, so the rescan is trivially sound).
pub const ESCAPE: i32 = -1;

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

    /// True if `state` is an escape state: prediction must be handed to
    /// the adaptive engine.
    #[inline]
    pub fn is_escape(&self, state: usize) -> bool {
        self.accepts[state] == ESCAPE
    }

    /// Alternative mask of `state` when it mask-accepts (the live
    /// alternatives are covered by one prefix-factor group; bit
    /// `1<<(alt-1)` per member).
    #[inline]
    pub fn accept_mask(&self, state: usize) -> Option<u64> {
        let m = self.masks[state];
        if m != 0 {
            Some(m)
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

    /// Successor of `state` on token type `t`, if any. O(1): the compact
    /// serialized interval triples are expanded into a per-state direct-
    /// indexed row at load time (see [`StaticDFATables::from_int_stream`]).
    #[inline]
    pub fn edge(&self, state: usize, t: i32) -> Option<usize> {
        let base = self.edge_row_at[state] as usize;
        let end = self.edge_row_at[state + 1] as usize;
        let idx = t - self.edge_min[state];
        if idx < 0 {
            return None;
        }
        let pos = base + idx as usize;
        if pos >= end {
            return None;
        }
        let target = self.edge_targets[pos];
        if target < 0 {
            None
        } else {
            Some(target as usize)
        }
    }
}
