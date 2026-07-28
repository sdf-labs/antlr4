/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.analysis;

/**
 * A statically-precomputed SLL prediction DFA for a single parser decision,
 * produced by {@link DecisionClassifier} for decisions whose static
 * resolution is provably behavior-preserving (conflict-free LL(k)/LL(*)
 * DFAs, plus trusted exact ambiguities resolved to the minimum alternative).
 *
 * <p>This is the compile-time equivalent of the DFA that
 * {@code ParserATNSimulator} would otherwise build lazily at runtime in SLL
 * mode. Code generation serializes it into the generated parser as constant
 * tables; a tiny runtime walker ({@code dfaPredict}) drives prediction with
 * no ATN simulation and no allocation.</p>
 *
 * <p>Encoding: state 0 is the start state. {@link #accepts}{@code [s] > 0}
 * marks an accept state predicting that alternative (alternative numbers are
 * the decision state's transition indexes + 1, exactly like
 * {@code adaptivePredict}'s return value). Accept states are terminal (no
 * outgoing edges). Non-accept states carry sorted, disjoint edges
 * {@code (lo, hi, target)} over token types (including {@code Token.EOF} =
 * -1); a lookahead token matching no edge means no viable alternative.</p>
 */
public class StaticDFA {
	/**
	 * Sentinel in {@link #accepts}: an escape state of a hybrid table. The
	 * walker stops and defers the whole prediction to
	 * {@code adaptivePredict} (the table never consumes input, so the
	 * rescan is trivially sound). See DecisionClassifier's HYBRID category.
	 */
	public static final int ESCAPE = -1;

	/**
	 * Sentinel in {@link #accepts}: a guarded-take state of an
	 * optional-postfix decision ({@code X Y?}). The walker evaluates the
	 * decision's guard - the epsilon-pop chase from the decision's block
	 * end over the real parse stack - and resolves to
	 * {@link #guardedAlts}{@code [s]} when no invoking state on the stack
	 * is in {@link #guardDanger}, deferring to {@code adaptivePredict}
	 * otherwise (see DecisionClassifier#finalizeGuardedTakes).
	 */
	public static final int GUARDED = -3;

	/** True if any state of this table is a guarded-take state: a runtime
	 *  prediction driven by it may defer to {@code adaptivePredict} when
	 *  the parse stack trips the guard. */
	public boolean hasGuards() {
		for (int a : accepts) {
			if (a == GUARDED) return true;
		}
		return false;
	}

	/** True if any state of this table is an escape state: a runtime
	 *  prediction driven by it can defer to {@code adaptivePredict}. */
	public boolean hasEscapes() {
		for (int a : accepts) {
			if (a == ESCAPE) return true;
		}
		return false;
	}

	/** True if any state of this table is a mask-accept state: a runtime
	 *  prediction driven by it can receive an alternative mask it cannot
	 *  execute (no factored arm), and must defer to {@code adaptivePredict}. */
	public boolean hasMasks() {
		for (long m : acceptMasks) {
			if (m != 0) return true;
		}
		return false;
	}

	public final int decision;
	public final int numStates;
	/** Predicted alternative per state; 0 = not an accept state; {@link #ESCAPE} = escape. */
	public final int[] accepts;
	/**
	 * Alternative-mask accept per state; 0 = none. A nonzero entry marks a
	 * terminal state where the walker returns the mask (bit {@code 1<<(alt-1)}
	 * per live alternative) instead of a unique alternative: the live set is
	 * covered by a single prefix-factor group (see PrefixFactorAnalyzer), so
	 * the generated parser executes the group's shared prefix and resolves
	 * the choice with its tail decision. Only ever set on states that would
	 * otherwise be escape states.
	 */
	public final long[] acceptMasks;
	/**
	 * Error-avoidance fallback per state; 0 = none. When prediction dies at
	 * a state on a token matching no edge, the walker returns this
	 * alternative (the minimum one that already finished the decision entry
	 * rule) instead of failing, so the parser reports a more precise error
	 * at the actual mismatch point - mirroring adaptivePredict's
	 * getAltThatFinishedDecisionEntryRule recovery.
	 */
	public final int[] fallbacks;
	/** Index of each state's first edge int in {@link #edges}; length numStates+1. */
	public final int[] edgeOffsets;
	/** Flattened (lo, hi, target) triples, sorted by lo within each state. */
	public final int[] edges;
	/** True if the DFA contains cycles (unbounded, LL(*) lookahead). */
	public final boolean cyclic;
	/** Max lookahead depth if acyclic; -1 if cyclic. */
	public final int maxK;
	/**
	 * Resolution alternative per {@link #GUARDED} state; 0 = not guarded.
	 */
	public final int[] guardedAlts;
	/**
	 * Sorted invoking-state numbers whose follow state's epsilon-reachable
	 * region contains a guard root of this decision; null/empty when the
	 * table has no guarded states. The runtime guard defers when the parse
	 * stack carries one of these invoking states.
	 */
	public final int[] guardDanger;
	/**
	 * Sorted invoking-state numbers whose follow state can reach their own
	 * rule's stop over epsilon; the runtime guard keeps walking the stack
	 * past them (and stops at the first invoking state not in this set).
	 */
	public final int[] guardPass;

	public StaticDFA(int decision, int numStates, int[] accepts, long[] acceptMasks,
					 int[] fallbacks, int[] edgeOffsets, int[] edges, boolean cyclic, int maxK,
					 int[] guardedAlts, int[] guardDanger, int[] guardPass) {
		this.decision = decision;
		this.numStates = numStates;
		this.accepts = accepts;
		this.acceptMasks = acceptMasks;
		this.fallbacks = fallbacks;
		this.edgeOffsets = edgeOffsets;
		this.edges = edges;
		this.cyclic = cyclic;
		this.maxK = maxK;
		this.guardedAlts = guardedAlts;
		this.guardDanger = guardDanger;
		this.guardPass = guardPass;
	}
}
