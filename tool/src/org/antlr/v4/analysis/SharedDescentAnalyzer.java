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
import org.antlr.v4.runtime.atn.LL1Analyzer;
import org.antlr.v4.runtime.atn.Transition;
import org.antlr.v4.runtime.misc.IntervalSet;
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
		/** Explicit tail arms (member alt -> tail token set); the default
		 *  alt ({@link #defaultAlt}) catches everything else. */
		public final Map<Integer, IntervalSet> explicitArms = new HashMap<Integer, IntervalSet>();
		/** The member alt whose continuation after R is nullable (it may
		 *  complete right after R); the tail dispatch's default arm. 0
		 *  when none (all-explicit dispatch, with an adaptive fallback). */
		public int defaultAlt;
		/** The dispatch plan is sound: exactly one nullable default and
		 *  pairwise-disjoint explicit arms (each disjoint from the
		 *  default's viable set). */
		public boolean codegenable;

		public Group(int rule, BitSet alts) {
			this.rule = rule;
			this.alts = alts;
		}

		/** A state on the epsilon descent whose transition invokes
		 *  {@link #rule}: a valid call-site for the neutral parse (the
		 *  generated block sets the parser's invoking state to it before
		 *  calling the common rule, so error recovery sees a proper
		 *  call frame). */
		public int callSiteState;
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
	private final Map<String, Set<Integer>> callSitesCache = new HashMap<String, Set<Integer>>();

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
			if (members.cardinality() >= 2) {
				Group grp = new Group(r, members);
				grp.callSiteState = firstCallSite(s, r);
				analyzeDispatch(s, grp);
				plan.groups.add(grp);
				if ("descent".equals(System.getProperty("antlr.dfa.debug")) && grp.codegenable) {
					System.err.printf("DESCENT-PLAN d=%d rule=%s alts=%s default=%d arms=%s%n",
						s.decision, g.getRule(r).name, members, grp.defaultAlt, grp.explicitArms);
				}
			}
		}
		return plan;
	}

	/**
	 * The dispatch analysis of one group: every member alt's mandatory
	 * continuation after R (its FIRST tokens, FOLLOW tokens included iff
	 * the continuation is nullable). Explicit arms are the non-nullable
	 * continuations; the default is the single nullable one. Over-
	 * approximating (grammar-wide pops, full callee LOOK) can only
	 * reject sound groups, never accept unsound ones.
	 */
	private void analyzeDispatch(DecisionState s, Group grp) {
		LL1Analyzer ll1 = new LL1Analyzer(atn);
		Map<Integer, IntervalSet> mandatory = new HashMap<Integer, IntervalSet>();
		Set<Integer> nullable = new HashSet<Integer>();
		for (int a = grp.alts.nextSetBit(0); a >= 0; a = grp.alts.nextSetBit(a+1)) {
			Set<Integer> nul = new HashSet<Integer>();
			IntervalSet t = mandatoryTail(s, a, grp.rule, ll1, nul);
			mandatory.put(a, t);
			// an empty mandatory tail is a degenerate continuation: the
			// alternative completes with no discriminating token, so it
			// belongs to the default arm, not an explicit one
			if (!nul.isEmpty() || t.isNil()) nullable.add(a);
		}
		if (nullable.size() > 1) return;
		IntervalSet defaultSet = nullable.isEmpty()
			? new IntervalSet() : mandatory.get(nullable.iterator().next());
		IntervalSet acc = new IntervalSet();
		for (int a = grp.alts.nextSetBit(0); a >= 0; a = grp.alts.nextSetBit(a+1)) {
			if (nullable.contains(a)) continue;
			IntervalSet t = mandatory.get(a);
			for (int tok : t.toArray()) {
				if (acc.contains(tok)) return;
				acc.add(tok);
			}
			for (int tok : defaultSet.toArray()) {
				if (t.contains(tok)) return;
			}
		}
		if (!nullable.isEmpty()) grp.defaultAlt = nullable.iterator().next();
		boolean anyTokens = false;
		for (int a = grp.alts.nextSetBit(0); a >= 0; a = grp.alts.nextSetBit(a+1)) {
			if (!nullable.contains(a)) {
				grp.explicitArms.put(a, mandatory.get(a));
				if (!mandatory.get(a).isNil()) anyTokens = true;
			}
		}
		// a dispatch needs something to discriminate on: a default arm,
		// or at least one non-empty explicit arm
		grp.codegenable = grp.defaultAlt != 0 || anyTokens;
	}

	/**
	 * FIRST tokens of alternative {@code alt}'s mandatory continuation
	 * after R (over-approximated); {@code nullable} is set when the
	 * continuation can complete the alternative with no post-R token,
	 * in which case the decision rule's FOLLOW is included.
	 */
	private IntervalSet mandatoryTail(DecisionState s, int alt, int ruleR, LL1Analyzer ll1,
									  Set<Integer> nullable) {
		IntervalSet tokens = new IntervalSet();
		Set<ATNState> calls = new HashSet<ATNState>();
		Set<ATNState> seen = new HashSet<ATNState>();
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(s.transition(alt-1).target);
		while (!work.isEmpty()) {
			ATNState st = work.poll();
			if (!seen.add(st) || st instanceof RuleStopState) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					if (((RuleTransition)t).target.ruleIndex == ruleR) calls.add(((RuleTransition)t).followState);
					else work.add(t.target);
				}
				else if (t.isEpsilon()) {
					work.add(t.target);
				}
			}
		}
		for (ATNState follow : calls) {
			tailFirst(follow, s.ruleIndex, tokens, nullable, ll1, new HashSet<ATNState>());
		}
		return tokens;
	}

	private void tailFirst(ATNState st, int decisionRule, IntervalSet tokens,
						   Set<Integer> nullable, LL1Analyzer ll1, Set<ATNState> seen) {
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(st);
		while (!work.isEmpty()) {
			ATNState p = work.poll();
			if (!seen.add(p)) continue;
			for (int i = 0; i < p.getNumberOfTransitions(); i++) {
				Transition t = p.transition(i);
				if (t instanceof RuleTransition) {
					tokens.addAll(ll1.LOOK(t.target, null));
					work.add(((RuleTransition)t).followState);
				}
				else if (t.isEpsilon()) {
					if (p instanceof RuleStopState) {
						if (p.ruleIndex == decisionRule) {
							nullable.add(1);
							tokens.addAll(ll1.LOOK(p, null));
						}
						else {
							for (RuleTransition rt : callByFollowState.values()) {
								if (rt.target.ruleIndex == p.ruleIndex) work.add(rt.followState);
							}
						}
					}
					else {
						work.add(t.target);
					}
				}
				else {
					tokens.addAll(t.label());
				}
			}
		}
	}

	/**
	 * Does this configuration genuinely start via R - i.e. does its
	 * call chain contain an R invocation as the <em>first consuming
	 * invocation</em> of the decision? Formally: walking the stack
	 * segments from the top, there is a deepest segment whose rule is R
	 * (the outermost R invocation, since configurations reached by the
	 * decision's own closure can only nest R inside R's own parse), and
	 * every segment below it - the wrapper calls from the decision entry
	 * to that invocation - lies on an epsilon-only path from the
	 * decision (its return address is in {@link #epsilonCallSites}).
	 * A configuration that reached R through a token-consuming call
	 * site (a TRIM argument, a subscript bracket) has a non-epsilon
	 * segment below the R invocation and fails; so does one whose
	 * widening joint hides a non-epsilon entry. Merged contexts: any
	 * branch may establish the derivation.
	 */
	public boolean startsViaR(ATNConfig c, DecisionState s, int ruleR) {
		Set<Integer> callSites = epsilonCallSites(s, ruleR);
		return segStartsViaR(c.state.ruleIndex, c.context, ruleR, callSites,
			java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()));
	}

	/** Does the sub-chain from ({@code stateRule}, {@code ctx}) down
	 *  transit R as first consumer? Deepest-first: the outermost R
	 *  invocation is the deepest R segment on the stack, and only its
	 *  below-path is validated (segments above it are inside R's own
	 *  parse, its business). */
	private boolean segStartsViaR(int stateRule, PredictionContext ctx, int ruleR,
								  Set<Integer> callSites, Set<PredictionContext> visited) {
		// deepest-first: try the frames below for a deeper R segment
		if (ctx != null && visited.add(ctx)) {
			for (int i = 0; i < ctx.size(); i++) {
				int rs = ctx.getReturnState(i);
				if (rs != PredictionContext.EMPTY_RETURN_STATE
					&& segStartsViaR(atn.states.get(rs).ruleIndex, ctx.getParent(i), ruleR, callSites, visited)) {
					return true;
				}
			}
		}
		// no deeper R segment validates: is THIS the outermost R segment?
		if (stateRule == ruleR) {
			return allCallSitesToEmpty(ctx, callSites,
				java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()));
		}
		return false;
	}

	private boolean allCallSitesToEmpty(PredictionContext ctx, Set<Integer> callSites,
										Set<PredictionContext> visited) {
		if (ctx == null || ctx.isEmpty()) return true;
		if (!visited.add(ctx)) return true;
		for (int i = 0; i < ctx.size(); i++) {
			int rs = ctx.getReturnState(i);
			if (rs == PredictionContext.EMPTY_RETURN_STATE) continue;
			if (callSites.contains(rs)
				&& allCallSitesToEmpty(ctx.getParent(i), callSites, visited)) {
				return true;
			}
		}
		return false;
	}

	/**
	 * Follow states of every rule call lying on an epsilon-only path
	 * from the decision entry to an R invocation (wrapper calls and the
	 * R-entry call itself), not descending into R.
	 */
	private Set<Integer> epsilonCallSites(DecisionState s, int ruleR) {
		String key = s.decision + ":" + ruleR;
		Set<Integer> sites = callSitesCache.get(key);
		if (sites == null) {
			sites = new HashSet<Integer>();
			Set<ATNState> seen = new HashSet<ATNState>();
			Deque<ATNState> work = new ArrayDeque<ATNState>();
			for (int i = 0; i < s.getNumberOfTransitions(); i++) work.add(s.transition(i).target);
			while (!work.isEmpty()) {
				ATNState st = work.poll();
				if (!seen.add(st) || st instanceof RuleStopState) continue;
				for (int i = 0; i < st.getNumberOfTransitions(); i++) {
					Transition t = st.transition(i);
					if (t instanceof RuleTransition) {
						RuleTransition rt = (RuleTransition)t;
						sites.add(rt.followState.stateNumber);
						if (rt.target.ruleIndex != ruleR) work.add(t.target);
					}
					else if (t.isEpsilon()) {
						work.add(t.target);
					}
				}
			}
			callSitesCache.put(key, sites);
			if ("descent".equals(System.getProperty("antlr.dfa.debug"))) {
				System.err.printf("CALLSITES d=%d rule=%s sites=%s%n", s.decision,
					ruleR >= 0 ? "?" : "?", sites);
			}
		}
		return sites;
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

	/** The source state of the first R-call found on an epsilon-only
	 *  descent from the decision whose first transition is the call -
	 *  the shape of call-site the error recovery requires (it re-reads
	 *  the invoking state's first transition as the invoking call). */
	private int firstCallSite(DecisionState s, int ruleR) {
		Set<ATNState> seen = new HashSet<ATNState>();
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		for (int i = 0; i < s.getNumberOfTransitions(); i++) work.add(s.transition(i).target);
		while (!work.isEmpty()) {
			ATNState st = work.poll();
			if (!seen.add(st) || st instanceof RuleStopState) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					if (((RuleTransition)t).target.ruleIndex == ruleR) {
						if (st.transition(0) == t) return st.stateNumber;
						work.add(t.target);
					}
					else {
						work.add(t.target);
					}
				}
				else if (t.isEpsilon()) {
					work.add(t.target);
				}
			}
		}
		return s.stateNumber;
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
