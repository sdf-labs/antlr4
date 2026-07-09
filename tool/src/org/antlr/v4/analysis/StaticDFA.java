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
	public final int decision;
	public final int numStates;
	/** Predicted alternative per state; 0 = not an accept state. */
	public final int[] accepts;
	/** Index of each state's first edge int in {@link #edges}; length numStates+1. */
	public final int[] edgeOffsets;
	/** Flattened (lo, hi, target) triples, sorted by lo within each state. */
	public final int[] edges;
	/** True if the DFA contains cycles (unbounded, LL(*) lookahead). */
	public final boolean cyclic;
	/** Max lookahead depth if acyclic; -1 if cyclic. */
	public final int maxK;

	public StaticDFA(int decision, int numStates, int[] accepts,
					 int[] edgeOffsets, int[] edges, boolean cyclic, int maxK) {
		this.decision = decision;
		this.numStates = numStates;
		this.accepts = accepts;
		this.edgeOffsets = edgeOffsets;
		this.edges = edges;
		this.cyclic = cyclic;
		this.maxK = maxK;
	}
}
