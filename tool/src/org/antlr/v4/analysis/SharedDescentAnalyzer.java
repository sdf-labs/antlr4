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
import java.util.LinkedHashMap;
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
		public int callSiteState = -1;
		/** Leading token types shared by every member alternative
		 *  before the epsilon descent to {@link #rule} (empty for a
		 *  pure zero-token descent). */
		public final List<Integer> prefixTokens = new ArrayList<Integer>();
		/** The block's full mask: member alternatives plus prefix-sharing
		 *  non-members (they fall through to the widen arm, which defers
		 *  to adaptivePredict - so their conflicts never need resolving,
		 *  but their states are accepted). */
		public final BitSet blockAlts = new BitSet();
		/** The R-call follow states of the member alternatives: the
		 *  prefix-side boundary of the first-consumer proof (frames
		 *  below the R segment are validated only down to these). */
		public final Set<Integer> boundaryStates = new HashSet<Integer>();
	}

	/** Per-decision plan: the shared-descent groups of the decision. */
	public static class Plan {
		public final List<Group> groups = new ArrayList<Group>();

		public boolean hasGroups() { return !groups.isEmpty(); }

		/** The group whose block alternatives cover {@code live}, or null. */
		public Group groupCovering(BitSet live) {
			for (Group grp : groups) {
				BitSet covered = (BitSet)(grp.blockAlts.isEmpty() ? grp.alts.clone() : grp.blockAlts.clone());
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
		// one DFS per alternative yields every (rule, prefix) candidate;
		// cluster alternatives sharing the same (rule, prefixTokens)
		Map<String, BitSet> byKey = new LinkedHashMap<String, BitSet>();
		Map<String, List<AltPath>> pathsByKey = new LinkedHashMap<String, List<AltPath>>();
		if ("descent".equals(System.getProperty("antlr.dfa.debug"))) {
			for (int a = 1; a <= s.getNumberOfTransitions(); a++) {
				for (AltPath p : altPaths(s, a)) {
					System.err.printf("CAND d=%d alt=%d rule=%s prefix=%s%n", s.decision, a,
						g.getRule(p.rule).name, p.tokens);
				}
			}
		}
		for (int a = 1; a <= s.getNumberOfTransitions(); a++) {
			for (AltPath p : altPaths(s, a)) {
				String key = p.rule + "|" + p.tokens;
				byKey.computeIfAbsent(key, k -> new BitSet()).set(a);
				pathsByKey.computeIfAbsent(key, k -> new ArrayList<AltPath>()).add(p);
			}
		}
		for (Map.Entry<String, BitSet> e : byKey.entrySet()) {
			if (e.getValue().cardinality() < 2) continue;
			List<AltPath> paths = pathsByKey.get(e.getKey());
			AltPath first = paths.get(0);
			Group grp = new Group(first.rule, e.getValue());
			grp.prefixTokens.addAll(first.tokens);
			for (AltPath p : paths) {
				if (grp.callSiteState < 0) grp.callSiteState = p.callSite;
				grp.boundaryStates.add(p.boundary);
			}
			analyzeDispatch(s, grp);
			boolean dj = disjointOk(s, grp);
			if ("descent".equals(System.getProperty("antlr.dfa.debug"))) {
				System.err.printf("DISPATCH d=%d rule=%s alts=%s default=%d arms=%s disjoint=%s%n",
					s.decision, g.getRule(grp.rule).name, grp.alts, grp.defaultAlt, grp.explicitArms, dj);
			}
			if (grp.codegenable && !dj) grp.codegenable = false;
			// the block's mask widens to prefix-sharing non-members (they
			// fall through to the widen arm, so their states are accepted
			// without any conflict resolution of their own)
			if (grp.codegenable && !grp.prefixTokens.isEmpty()) {
				for (int a = 1; a <= s.getNumberOfTransitions(); a++) {
					if (!grp.alts.get(a) && postPrefixState(s, a, grp.prefixTokens) != null) {
						grp.blockAlts.set(a);
					}
				}
			}
			grp.blockAlts.or(grp.alts);
			plan.groups.add(grp);
			if ("descent".equals(System.getProperty("antlr.dfa.debug")) && grp.codegenable) {
				System.err.printf("DESCENT-PLAN d=%d rule=%s prefix=%s alts=%s default=%d arms=%s%n",
					s.decision, g.getRule(grp.rule).name, grp.prefixTokens, grp.alts, grp.defaultAlt, grp.explicitArms);
			}
		}
		return plan;
	}

	/**
	 * The ambiguity disjointness check for an expression-first
	 * dispatch: some non-member alternative sharing the prefix could
	 * also complete with the same tail. That happens exactly when a
	 * single token completes both the non-member alternative and R
	 * itself (e.g. {@code (TABLE)} as a parenthesized expression vs as
	 * a query): then a clean R parse with that tail is not unique to
	 * the member and the group must be deferred. The check is precise:
	 * for every content-first token of every prefix-sharing non-member
	 * alternative, require that it cannot complete BOTH that
	 * alternative and R.
	 */
	private boolean disjointOk(DecisionState s, Group grp) {
		LL1Analyzer ll1 = new LL1Analyzer(atn);
		for (int a = 1; a <= s.getNumberOfTransitions(); a++) {
			if (grp.alts.get(a)) continue;
			ATNState post = postPrefixState(s, a, grp.prefixTokens);
			if (post == null) continue; // does not share the prefix: irrelevant
			for (int t : ll1.LOOK(post, null).toArray()) {
				if (t <= 0) continue;
				boolean cr = canBeExactly(atn.ruleToStartState[grp.rule], t);
				boolean ca = canBeExactly(post, t);
				// a genuine collision is fatal only when the runtime's
				// min-alt resolution would prefer the non-member over
				// every member of the group
				if (cr && ca && a < grp.alts.nextSetBit(0)) {
					if ("descent".equals(System.getProperty("antlr.dfa.debug"))) {
						System.err.printf("DISJOINT-FAIL d=%d alt=%d token=%d%n", s.decision, a, t);
					}
					return false;
				}
			}
		}
		return true;
	}

	/** The state of alternative {@code alt} after the exact prefix
	 *  token sequence (epsilon + those tokens in order), or null when
	 *  the alternative does not begin with the prefix. */
	private ATNState postPrefixState(DecisionState s, int alt, List<Integer> prefixTokens) {
		Set<String> seen = new HashSet<String>();
		Deque<Object[]> work = new ArrayDeque<Object[]>();
		work.add(new Object[]{s.transition(alt-1).target, 0});
		while (!work.isEmpty()) {
			Object[] item = work.poll();
			ATNState st = (ATNState)item[0];
			int k = (Integer)item[1];
			String key = st.stateNumber + ":" + k;
			if (!seen.add(key) || st instanceof RuleStopState) continue;
			if (k == prefixTokens.size()) return st;
			int want = prefixTokens.get(k);
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					work.add(new Object[]{t.target, k});
					work.add(new Object[]{((RuleTransition)t).followState, k});
				}
				else if (t.isEpsilon()) {
					work.add(new Object[]{t.target, k});
				}
				else if (t.label() != null && t.label().contains(want)) {
					work.add(new Object[]{t.target, k+1});
				}
			}
		}
		return null;
	}

	/** Can the language from {@code start} be exactly token {@code t}
	 *  (consume {@code t}, then reach a rule stop with only epsilon
	 *  steps)? */
	private boolean canBeExactly(ATNState start, int t) {
		Set<String> seen = new HashSet<String>();
		Deque<Object[]> work = new ArrayDeque<Object[]>();
		work.add(new Object[]{start, Boolean.FALSE});
		while (!work.isEmpty()) {
			Object[] item = work.poll();
			ATNState st = (ATNState)item[0];
			boolean consumed = (Boolean)item[1];
			String key = st.stateNumber + ":" + consumed;
			if (!seen.add(key)) continue;
			if (st instanceof RuleStopState) {
				if (consumed) return true;
				continue;
			}
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition tr = st.transition(i);
				if (tr instanceof RuleTransition) {
					// descending into a callee stays epsilon only when its
					// interior is token-free: in the consumed phase its
					// token transitions are pruned below, leaving only the
					// nullable paths
					work.add(new Object[]{tr.target, consumed});
					work.add(new Object[]{((RuleTransition)tr).followState, consumed});
				}
				else if (tr.isEpsilon()) {
					work.add(new Object[]{tr.target, consumed});
				}
				else if (!consumed && tr.label() != null && tr.label().contains(t)) {
					work.add(new Object[]{tr.target, Boolean.TRUE});
				}
				// any other token consumption breaks exactness
			}
		}
		return false;
	}

	/**
	 * Does the decision's emitted table contain a mask-accept covered
	 *  by one of the plan's descent groups (any multi-alt subset of a
	 *  group's block alternatives)? The block is only worth emitting
	 *  then; a plan whose groups never fire in the table must not
	 *  introduce adaptive widen paths into an otherwise fully static
	 *  parser.
	 */
	public static boolean hasDescentMask(Grammar g, int decision) {
		StaticDFA dfa = g.staticDecisionDFAs != null ? g.staticDecisionDFAs.get(decision) : null;
		Plan plan = g.staticDescentPlans != null ? g.staticDescentPlans.get(decision) : null;
		if (dfa == null || plan == null) return false;
		for (long m : dfa.acceptMasks) {
			if (m == 0 || (m & (m-1)) == 0) continue; // singletons take ordinary arms
			for (Group grp : plan.groups) {
				if (!grp.codegenable) continue;
				BitSet cover = grp.blockAlts.isEmpty() ? grp.alts : grp.blockAlts;
				long bits = 0;
				for (int a = cover.nextSetBit(0); a >= 0; a = cover.nextSetBit(a+1)) bits |= 1L << (a-1);
				if ((m & ~bits) == 0) return true;
			}
		}
		return false;
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
			ATNState start = grp.prefixTokens.isEmpty()
				? s.transition(a-1).target
				: postPrefixState(s, a, grp.prefixTokens);
			IntervalSet t = start != null
				? mandatoryTailFrom(s, a, grp.rule, ll1, nul, start)
				: new IntervalSet();
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
		return mandatoryTailFrom(s, alt, ruleR, ll1, nullable, s.transition(alt-1).target);
	}

	/** Variant of {@link #mandatoryTail} starting from the post-prefix
	 *  state of the alternative (prefix groups: the tail lies after the
	 *  shared prefix tokens). */
	private IntervalSet mandatoryTailFrom(DecisionState s, int alt, int ruleR, LL1Analyzer ll1,
										  Set<Integer> nullable, ATNState startState) {
		IntervalSet tokens = new IntervalSet();
		Set<ATNState> calls = new HashSet<ATNState>();
		Set<ATNState> seen = new HashSet<ATNState>();
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(startState);
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
		return segStartsViaR(c.state.ruleIndex, c.context, ruleR, callSites, null,
			java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()));
	}

	/** First-consumer proof for a group with a leading token prefix:
	 *  every wrapper segment below the outermost R segment must lie on
	 *  the epsilon descent from the decision, down to a boundary frame
	 *  (an R-call follow state of a member alternative - the prefix is
	 *  consumed below it and needs no validation). */
	public boolean startsViaR(ATNConfig c, DecisionState s, Group grp) {
		Set<Integer> callSites = epsilonCallSites(s, grp);
		return segStartsViaR(c.state.ruleIndex, c.context, grp.rule, callSites, grp.boundaryStates,
			java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()));
	}

	/** Does the sub-chain from ({@code stateRule}, {@code ctx}) down
	 *  transit R as first consumer? Deepest-first: the outermost R
	 *  invocation is the deepest R segment on the stack, and only its
	 *  below-path is validated (segments above it are inside R's own
	 *  parse, its business). */
	private boolean segStartsViaR(int stateRule, PredictionContext ctx, int ruleR,
								  Set<Integer> callSites, Set<Integer> boundary, Set<PredictionContext> visited) {
		// deepest-first: try the frames below for a deeper R segment
		if (ctx != null && visited.add(ctx)) {
			for (int i = 0; i < ctx.size(); i++) {
				int rs = ctx.getReturnState(i);
				if (rs != PredictionContext.EMPTY_RETURN_STATE
					&& segStartsViaR(atn.states.get(rs).ruleIndex, ctx.getParent(i), ruleR, callSites, boundary, visited)) {
					return true;
				}
			}
		}
		// no deeper R segment validates: is THIS the outermost R segment?
		if (stateRule == ruleR) {
			return allCallSitesToBoundary(ctx, callSites, boundary,
				java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()));
		}
		return false;
	}

	/** Every wrapper segment below the R segment lies on the epsilon
	 *  descent (in {@code callSites}), down to a boundary frame or the
	 *  chain end. */
	private boolean allCallSitesToBoundary(PredictionContext ctx, Set<Integer> callSites,
										   Set<Integer> boundary, Set<PredictionContext> visited) {
		if (ctx == null || ctx.isEmpty()) return true;
		if (!visited.add(ctx)) return true;
		for (int i = 0; i < ctx.size(); i++) {
			int rs = ctx.getReturnState(i);
			if (rs == PredictionContext.EMPTY_RETURN_STATE) continue;
			if (boundary != null && boundary.contains(rs)) return true;
			if (callSites.contains(rs)
				&& allCallSitesToBoundary(ctx.getParent(i), callSites, boundary, visited)) {
				return true;
			}
		}
		return false;
	}

	/** Group-aware call-site computation: epsilon paths from the
	 *  descent start states (the member alternatives' post-prefix R-call
	 *  source states, or the decision's alternative starts when the
	 *  prefix is empty) to the group's rule. */
	private Set<Integer> epsilonCallSites(DecisionState s, Group grp) {
		String key = s.decision + ":" + grp.rule + ":" + grp.prefixTokens.hashCode();
		Set<Integer> sites = callSitesCache.get(key);
		if (sites == null) {
			sites = new HashSet<Integer>();
			Set<ATNState> seen = new HashSet<ATNState>();
			Deque<ATNState> work = new ArrayDeque<ATNState>();
			if (grp.prefixTokens.isEmpty()) {
				for (int i = 0; i < s.getNumberOfTransitions(); i++) work.add(s.transition(i).target);
			}
			else {
				for (int a = grp.alts.nextSetBit(0); a >= 0; a = grp.alts.nextSetBit(a+1)) {
					// the member call sites are the post-prefix R-call
					// source states; the descent begins there
					work.add(atn.states.get(grp.callSiteState));
				}
			}
			while (!work.isEmpty()) {
				ATNState st = work.poll();
				if (!seen.add(st) || st instanceof RuleStopState) continue;
				for (int i = 0; i < st.getNumberOfTransitions(); i++) {
					Transition t = st.transition(i);
					if (t instanceof RuleTransition) {
						RuleTransition rt = (RuleTransition)t;
						sites.add(rt.followState.stateNumber);
						if (rt.target.ruleIndex != grp.rule) work.add(t.target);
					}
					else if (t.isEpsilon()) {
						work.add(t.target);
					}
				}
			}
			callSitesCache.put(key, sites);
		}
		return sites;
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
	/** The path of one alternative to R: the leading token sequence
	 *  consumed before the (epsilon) descent to R, the R-call's source
	 *  state ({@link #callSite}), and the R-call's follow state
	 *  ({@link #boundary} - the prefix-side end of the first-consumer
	 *  proof). */
	private static final class AltPath {
		final List<Integer> tokens = new ArrayList<Integer>();
		int rule = -1;
		int callSite = -1;
		int boundary = -1;
	}

	/**
	 * The leading token sequence of alternative {@code alt} before it
	 * descends to R with zero further tokens, or null when no such path
	 * exists. The path shape is [tokens*] then epsilon-only to R.
	 */
	/** Bound on the leading token prefix considered (real factor
	 *  prefixes are one to three tokens; longer loops are not useful
	 *  and explode the search). */
	private static final int MAX_PREFIX_TOKENS = 2;

	private List<AltPath> altPaths(DecisionState s, int alt) {
		List<AltPath> out = new ArrayList<AltPath>();
		Deque<Object[]> work = new ArrayDeque<Object[]>();
		work.add(new Object[]{s.transition(alt-1).target, new ArrayList<Integer>(), Boolean.FALSE});
		Set<String> seen = new HashSet<String>();
		while (!work.isEmpty()) {
			Object[] item = work.poll();
			ATNState st = (ATNState)item[0];
			@SuppressWarnings("unchecked") List<Integer> tokens = (List<Integer>)item[1];
			boolean inRule = (Boolean)item[2];
			String key = st.stateNumber + "|" + tokens + (inRule ? "r" : "");
			if (!seen.add(key) || st instanceof RuleStopState) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					RuleTransition rt = (RuleTransition)t;
					AltPath p = new AltPath();
					p.tokens.addAll(tokens);
					p.rule = rt.target.ruleIndex;
					p.callSite = st.stateNumber;
					p.boundary = rt.followState.stateNumber;
					out.add(p);
					work.add(new Object[]{t.target, tokens, Boolean.TRUE});
				}
				else if (t.isEpsilon()) {
					work.add(new Object[]{t.target, tokens, inRule});
				}
				else if (!inRule && t.label() != null && tokens.size() < MAX_PREFIX_TOKENS) {
					for (int tok : t.label().toList()) {
						List<Integer> next = new ArrayList<Integer>(tokens);
						next.add(tok);
						work.add(new Object[]{t.target, next, Boolean.FALSE});
					}
				}
			}
		}
		return out;
	}

	/** Can R be reached from {@code start} following epsilon
	 *  transitions and rule calls only (zero tokens)? */
	private boolean descendsToR(ATNState start, int ruleR, Set<ATNState> seen) {
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(start);
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
