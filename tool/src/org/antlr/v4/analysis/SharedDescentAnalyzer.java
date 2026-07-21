/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this source code is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.analysis;

import org.antlr.v4.runtime.atn.ATN;
import org.antlr.v4.runtime.atn.ATNConfig;
import org.antlr.v4.runtime.atn.ATNState;
import org.antlr.v4.runtime.atn.DecisionState;
import org.antlr.v4.runtime.atn.PredictionContext;
import org.antlr.v4.runtime.atn.RuleStopState;
import org.antlr.v4.runtime.atn.RuleTransition;
import org.antlr.v4.runtime.atn.Transition;
import org.antlr.v4.tool.Grammar;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

/**
 * Semantic shared-descent group analysis (the generalization of the
 * prefix-factor mask to rule-level shared descents). A group of
 * alternatives of a decision <em>starts via</em> a common rule R when
 * each member alternative descends to an R invocation with zero token
 * consumption (epsilon-only descent from the decision). At an escape
 * state whose live alternatives are covered by such a group and whose
 * live configurations all genuinely transited R at the decision's first
 * consuming invocation, the static walker may accept with the group's
 * alternative mask: the parser calls {@code R()} once and resolves the
 * choice at the post-R tail - the table never enumerates the descent
 * parse that exhausts its construction budget.
 *
 * <p>The per-configuration proof ({@link #startsViaR}) is the rigorous
 * core: a configuration starts via R iff its call chain contains an R
 * invocation frame (the callee of some context frame is R) whose call
 * site lies on an epsilon-only path from the decision entry, with only
 * epsilon-descent frames below it. Configurations whose chain is
 * truncated by a widening joint before reaching an R frame, or that
 * reach R through a token-consuming call site (e.g. inside a TRIM
 * argument), fail the proof - so the mask is only accepted where the
 * shared descent is genuine.</p>
 */
public class SharedDescentAnalyzer {

	/** One shared-descent group: member alternatives starting via {@link #rule}. */
	public static class Group {
		public final int rule;
		public final BitSet alts;

		public Group(int rule, BitSet alts) {
			this.rule = rule;
			this.alts = alts;
		}
	}

	/** Per-decision plan: the shared-descent groups of the decision. */
	public static class Plan {
		public final List<Group> groups = new ArrayList<Group>();

		public boolean hasGroups() { return !groups.isEmpty(); }

		/** The group whose member alternatives cover {@code live}, or null. */
		public Group groupCovering(BitSet live) {
			for (Group grp : groups) {
				BitSet covered = (BitSet)grp.alts.clone();
				covered.and(live);
				if (covered.equals(live)) return grp;
			}
			return null;
		}
	}

	private final Grammar g;
	private final ATN atn;
	private final Map<Integer, RuleTransition> callByFollowState;
	private final Map<Integer, List<Integer>> candidateCache = new HashMap<Integer, List<Integer>>();

	public SharedDescentAnalyzer(Grammar g, ATN atn, Map<Integer, RuleTransition> callByFollowState) {
		this.g = g;
		this.atn = atn;
		this.callByFollowState = callByFollowState;
	}

	/** Build the decision's shared-descent plan. */
	public Plan buildPlan(DecisionState s) {
		Plan plan = new Plan();
		for (int r : candidates(s)) {
			BitSet members = new BitSet();
			for (int a = 1; a <= s.getNumberOfTransitions(); a++) {
				if (altStartsViaR(s, a, r)) members.set(a);
			}
			if (members.cardinality() >= 2) plan.groups.add(new Group(r, members));
		}
		return plan;
	}

	/**
	 * Is this configuration inside an R invocation - R is its own rule,
	 * or the rule of some frame's return state? Widening joints hide the
	 * R-entry call itself, but the joint's frame still carries R's rule,
	 * and per-state prefix consistency supplies the rest: the static
	 * state is reached by a single consumed prefix, so configurations
	 * that entered R through a token-consuming call site (a TRIM
	 * argument, a subscript bracket) land in different states than
	 * configurations that entered R through the decision's epsilon
	 * descent. Merged contexts: any branch may establish the derivation.
	 */
	public boolean startsViaR(ATNConfig c, DecisionState s, int ruleR) {
		if (c.state.ruleIndex == ruleR) return true;
		if (c.context == null) return false;
		Set<PredictionContext> seen =
			java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>());
		Deque<PredictionContext> work = new ArrayDeque<PredictionContext>();
		work.add(c.context);
		while (!work.isEmpty()) {
			PredictionContext ctx = work.poll();
			if (ctx == null || !seen.add(ctx)) continue;
			for (int i = 0; i < ctx.size(); i++) {
				int rs = ctx.getReturnState(i);
				if (rs != PredictionContext.EMPTY_RETURN_STATE
					&& atn.states.get(rs).ruleIndex == ruleR) {
					return true;
				}
				PredictionContext p = ctx.getParent(i);
				if (p != null) work.add(p);
			}
		}
		return false;
	}

	/**
	 * Rules invocable from the decision via epsilon-only descents, in
	 * BFS order (shallowest shared rules first).
	 */
	private List<Integer> candidates(DecisionState s) {
		List<Integer> rules = candidateCache.get(s.decision);
		if (rules == null) {
			Set<Integer> seen = new java.util.LinkedHashSet<Integer>();
			Set<ATNState> seenStates = new HashSet<ATNState>();
			Deque<ATNState> work = new ArrayDeque<ATNState>();
			for (int i = 0; i < s.getNumberOfTransitions(); i++) work.add(s.transition(i).target);
			while (!work.isEmpty()) {
				ATNState st = work.poll();
				if (!seenStates.add(st) || st instanceof RuleStopState) continue;
				for (int i = 0; i < st.getNumberOfTransitions(); i++) {
					Transition t = st.transition(i);
					if (t instanceof RuleTransition) {
						seen.add(((RuleTransition)t).target.ruleIndex);
						work.add(t.target);
					}
					else if (t.isEpsilon()) {
						work.add(t.target);
					}
				}
			}
			rules = new ArrayList<Integer>(seen);
			candidateCache.put(s.decision, rules);
		}
		return rules;
	}

	/**
	 * Does alternative {@code alt} descend to an R invocation with zero
	 * token consumption (on some path)?
	 */
	private boolean altStartsViaR(DecisionState s, int alt, int ruleR) {
		Set<ATNState> seen = new HashSet<ATNState>();
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(s.transition(alt-1).target);
		while (!work.isEmpty()) {
			ATNState st = work.poll();
			if (!seen.add(st) || st instanceof RuleStopState) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					if (((RuleTransition)t).target.ruleIndex == ruleR) return true;
					work.add(t.target);
				}
				else if (t.isEpsilon()) {
					work.add(t.target);
				}
			}
		}
		return false;
	}

}
