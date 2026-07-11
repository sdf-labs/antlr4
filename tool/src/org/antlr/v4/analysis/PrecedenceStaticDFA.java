/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.analysis;

/**
 * The statically-precomputed prediction tables of one left-recursive
 * precedence loop decision (the {@code StarLoopEntryState} the left-recursion
 * transformer generates), produced by {@link DecisionClassifier}.
 *
 * <p>Such a decision cannot be served by a single static DFA because its
 * behavior depends on the current precedence (the {@code _p} argument of the
 * rewritten rule): each operator alternative is guarded by
 * {@code precpred(_ctx, n)}, which holds iff {@code n >= _p}. The runtime's
 * adaptive engine handles this by maintaining a separate DFA start state per
 * precedence value ({@code DFA.isPrecedenceDfa()}). Statically, the guard
 * constants {@code n1 < ... < nk} appearing in the rule partition all
 * precedence values into at most {@code k+1} equivalence classes with
 * identical guard outcomes, so the complete behavior is captured by one
 * {@link StaticDFA} per class.</p>
 *
 * <p>Dispatch: the class of precedence {@code p} is the number of cutoffs
 * strictly less than {@code p}; the walker computes it from
 * {@code parser.getPrecedence()} (the top of the precedence stack, exactly
 * the value the adaptive runtime keys its per-precedence start states on).
 * Class 0 (p at most n1) has every operator viable; the last class
 * (p greater than nk) has none, so its table accepts the loop-exit
 * alternative with zero lookahead - together these make the generated loop
 * behave like a hand-rolled precedence-climbing (Pratt) parser.</p>
 */
public class PrecedenceStaticDFA {
	public final int decision;
	/** Sorted distinct precedence-guard constants; class(p) = |{c : c &lt; p}|. */
	public final int[] cutoffs;
	/** One table per precedence class; length {@code cutoffs.length + 1}. */
	public final StaticDFA[] tables;

	public PrecedenceStaticDFA(int decision, int[] cutoffs, StaticDFA[] tables) {
		this.decision = decision;
		this.cutoffs = cutoffs;
		this.tables = tables;
	}
}
