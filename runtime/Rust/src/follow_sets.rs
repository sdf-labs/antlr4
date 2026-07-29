//! Real-stack postfix-chase FIRST sets for the guarded descent dispatch
//! (`-Xstatic-dfa` shared-descent groups with a nullable member).
//!
//! A shared-descent group's tail dispatch commits to a member alternative on
//! the post-R lookahead token. When the group has a member that can complete
//! right after the shared rule (a *nullable* member, e.g. a bare column
//! reference), the commit is sound only when that member is dead on the real
//! parse stack - i.e. when the dispatch token cannot follow the decision
//! rule on this stack. The static analysis can only answer that with the
//! grammar-wide FOLLOW set, which is far too coarse (every alias position
//! accepts every keyword). The dispatch therefore evaluates the *real*
//! postfix chase: walking the parser's actual context frames, the token can
//! follow iff it starts the continuation of some frame's rule body (and
//! every frame below it completes token-free).
//!
//! This module precomputes, lazily and once per ATN, for every rule
//! invocation site the FIRST set of the caller-body continuation from the
//! call's follow position to the rule end (`first`), and whether that
//! continuation can complete token-free (`nullable`). Both are plain
//! context-free fixpoints over the ATN (no widening, no recursion cut), so
//! the chase is exact: `first` is a superset of the true continuation set
//! (a predicate could only narrow it), which is the sound direction - the
//! chase may defer a commit it could have taken, never take one it must
//! not.

use std::sync::OnceLock;

use crate::atn::ATN;
use crate::token::TOKEN_EPSILON;
use crate::transition::TransitionType;

/// Per invocation-site continuation sets (see module docs). Indexed by
/// call-site ATN state number; unpopulated slots are (empty, false).
pub struct FollowSets {
    /// Bitset over token types (`TOKEN_EOF` = -1 is never included) of the
    /// continuation's FIRST tokens, one row per call-site state.
    first: Vec<Vec<u64>>,
    /// Whether the continuation from the call's follow position can reach
    /// the rule's end consuming no token.
    nullable: Vec<bool>,
}

fn set_bit(words: &mut [u64], token: i32) -> bool {
    let old = (words[token as usize / 64] >> (token as usize % 64)) & 1;
    words[token as usize / 64] |= 1 << (token as usize % 64);
    old == 0
}

impl FollowSets {
    /// The continuation sets of every invocation site of the grammar,
    /// computed on first use.
    pub fn of<'a>(cell: &'a OnceLock<FollowSets>, atn: &ATN) -> &'a FollowSets {
        cell.get_or_init(|| Self::compute(atn))
    }

    /// Compute the continuation sets for every invocation site.
    fn compute(atn: &ATN) -> FollowSets {
        let n_states = atn.states_count();
        let max_token = atn.max_token_type;
        let words = (max_token as usize + 64) / 64;

        // Per-state fixpoint: first(s) = FIRST of the language from s to
        // its rule's end; nullable(s) = the language contains epsilon.
        // Transitions contribute:
        //   token label l:        first += l
        //   epsilon:              first += first(target), nullable |= nullable(target)
        //   call C with follow f: first += FIRST(C); if nullable(C) {
        //                         first += first(f); nullable |= nullable(f) }
        let mut first: Vec<Vec<u64>> = vec![vec![0; words]; n_states];
        let mut nullable: Vec<bool> = vec![false; n_states];
        let mut changed = true;
        while changed {
            changed = false;
            for s in 0..n_states {
                let mut acc: Option<Vec<u64>> = None;
                let mut acc_nullable = false;
                for tr in atn.get_state(s as i32).get_transitions() {
                    let target = tr.get_target().get_state_number() as usize;
                    let acc = acc.get_or_insert_with(|| first[s].clone());
                    match tr.transition_type() {
                        TransitionType::Atom | TransitionType::Set | TransitionType::Range => {
                            if let Some(label) = tr.get_label() {
                                for iv in label {
                                    for tok in iv.a..=iv.b.min(max_token) {
                                        if tok >= crate::token::TOKEN_MIN_USER_TOKEN_TYPE && set_bit(acc, tok) {
                                            changed = true;
                                        }
                                    }
                                }
                            }
                        }
                        TransitionType::NotSet => {
                            if let Some(label) = tr.get_label() {
                                for tok in 1..=max_token {
                                    if !label.contains(tok) && set_bit(acc, tok) {
                                        changed = true;
                                    }
                                }
                            }
                        }
                        TransitionType::Wildcard => {
                            for tok in 1..=max_token {
                                if set_bit(acc, tok) {
                                    changed = true;
                                }
                            }
                        }
                        TransitionType::Rule => {
                            let rule_tr = tr.try_as::<crate::transition::RuleTransition>().unwrap();
                            let follow = rule_tr.follow_state.get_state_number() as usize;
                            for w in 0..words {
                                let add = first[target][w] & !acc[w];
                                if add != 0 {
                                    acc[w] |= add;
                                    changed = true;
                                }
                            }
                            if nullable[target] {
                                for w in 0..words {
                                    let add = first[follow][w] & !acc[w];
                                    if add != 0 {
                                        acc[w] |= add;
                                        changed = true;
                                    }
                                }
                                if nullable[follow] && !acc_nullable {
                                    acc_nullable = true;
                                    changed = true;
                                }
                            }
                        }
                        _ => {
                            // epsilon, action, predicate, precedence
                            // predicate (evaluated as epsilon here - the
                            // superset direction, sound for the guard)
                            for w in 0..words {
                                let add = first[target][w] & !acc[w];
                                if add != 0 {
                                    acc[w] |= add;
                                    changed = true;
                                }
                            }
                            if nullable[target] && !acc_nullable {
                                acc_nullable = true;
                                changed = true;
                            }
                        }
                    }
                }
                if let Some(a) = acc {
                    first[s] = a;
                    if acc_nullable {
                        nullable[s] = true;
                    }
                }
            }
        }

        let mut site_first: Vec<Vec<u64>> = vec![Vec::new(); n_states];
        let mut site_nullable = vec![false; n_states];
        for s in 0..n_states {
            for tr in atn.get_state(s as i32).get_transitions() {
                if matches!(tr.transition_type(), TransitionType::Rule) {
                    let rule_tr = tr.try_as::<crate::transition::RuleTransition>().unwrap();
                    let follow = rule_tr.follow_state.get_state_number() as usize;
                    site_first[s] = first[follow].clone();
                    site_nullable[s] = nullable[follow];
                }
            }
        }
        FollowSets { first: site_first, nullable: site_nullable }
    }

    /// Does the continuation after the invocation at call-site `state`
    /// start with `token`? (Empty row = not a call site: nothing follows.)
    #[inline]
    pub fn contains(&self, state: usize, token: i32) -> bool {
        state < self.first.len()
            && !self.first[state].is_empty()
            && token >= 0
            && (self.first[state][token as usize / 64] >> (token as usize % 64)) & 1 != 0
    }

    /// Can the continuation after the invocation at call-site `state`
    /// complete token-free?
    #[inline]
    pub fn nullable(&self, state: usize) -> bool {
        state < self.nullable.len() && self.nullable[state]
    }
}
