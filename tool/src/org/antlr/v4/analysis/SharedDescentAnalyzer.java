/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this source code is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.analysis;

import org.antlr.v4.runtime.atn.ATN;
import org.antlr.v4.runtime.atn.ATNConfig;
import org.antlr.v4.runtime.atn.ATNState;
import org.antlr.v4.runtime.atn.BlockEndState;
import org.antlr.v4.runtime.atn.BlockStartState;
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
		 *  non-members admitted by {@link #widenHarmless} (their spans
		 *  can never coincide with a member's neutral-R span, so when
		 *  the neutral parse succeeds the tail dispatch cannot overrule
		 *  adaptivePredict's min-alt resolution, and when it fails the
		 *  block falls back to the adaptive engine). */
		public final BitSet blockAlts = new BitSet();
		/** The R-call follow states of the member alternatives: the
		 *  prefix-side boundary of the first-consumer proof (frames
		 *  below the R segment are validated only down to these). */
		public final Set<Integer> boundaryStates = new HashSet<Integer>();
		/** The recorded wrapper call chains of each member alternative
		 *  (outermost call first, the R call last): the dispatch's tail
		 *  analysis pops along these chains - not grammar-wide - so
		 *  phantom returns (R's own follow positions, other members'
		 *  call sites) cannot pollute a member's continuation set. */
		public final Map<Integer, List<List<RuleTransition>>> chainsByAlt = new HashMap<Integer, List<List<RuleTransition>>>();
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
	private final Map<Integer, List<Integer>> candidateCache = new HashMap<Integer, List<Integer>>();
	private final Map<String, Set<Integer>> callSitesCache = new HashMap<String, Set<Integer>>();

	public SharedDescentAnalyzer(Grammar g, ATN atn) {
		this.g = g;
		this.atn = atn;
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
				if (!firstConsumerUnique(s, a, p)) {
					if ("descent".equals(System.getProperty("antlr.dfa.debug"))) {
						System.err.printf("CAND-REJ d=%d alt=%d rule=%s prefix=%s%n", s.decision, a,
							g.getRule(p.rule).name, p.tokens);
					}
					continue;
				}
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
				grp.chainsByAlt.computeIfAbsent(p.alt, k -> new ArrayList<List<RuleTransition>>())
					.add(p.chain);
			}
			analyzeDispatch(s, grp);
			if ("descent".equals(System.getProperty("antlr.dfa.debug"))) {
				System.err.printf("DISPATCH d=%d rule=%s alts=%s default=%d arms=%s%n",
					s.decision, g.getRule(grp.rule).name, grp.alts, grp.defaultAlt, grp.explicitArms);
			}
			// the block's mask widens to prefix-sharing non-members whose
			// own spans cannot be confused with a member's (their states
			// then need no conflict resolution of their own)
			if (grp.codegenable && !grp.prefixTokens.isEmpty()) {
				for (int a = 1; a <= s.getNumberOfTransitions(); a++) {
					if (!grp.alts.get(a) && widenHarmless(s, grp, a)) {
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
	 * May the block's mask safely cover states where the prefix-sharing
	 * non-member alternative {@code a} is live? The neutral R parse
	 * plus tail dispatch commits to a member alternative, so a live
	 * non-member is dangerous exactly when the input can complete BOTH
	 * a member (span = prefix + an R string) and {@code a} itself AND
	 * adaptivePredict's min-alt resolution would prefer {@code a} over
	 * the member the dispatch commits to (e.g. {@code ( query )} next
	 * to a {@code ( expression )} group: {@code (SELECT 1)} is both a
	 * parenthesized expression over a subquery and a direct subquery,
	 * and the direct subquery alternative wins). Two conservative
	 * sufficient conditions for harmlessness: every member is
	 * lower-numbered than {@code a} (the min-alt rule then always
	 * prefers a member, whichever one the dispatch commits to), or no
	 * string can be both an R derivation and {@code a}'s post-prefix
	 * content - over-approximated by disjoint FIRST sets (this also
	 * covers the {@code (TABLE)}-style exact-single-token collision,
	 * which is a special case of FIRST overlap). Otherwise the
	 * alternative is left out of the mask and its states fall back to
	 * the adaptive engine.
	 */
	private boolean widenHarmless(DecisionState s, Group grp, int a) {
		ATNState post = postPrefixState(s, a, grp.prefixTokens);
		if (post == null) return false; // does not share the prefix: irrelevant
		if (a >= grp.alts.length()) return true; // every member beats a on min-alt
		if (System.getProperty("antlr.dfa.disableWidenGate") != null) return true;
		LL1Analyzer ll1 = new LL1Analyzer(atn);
		IntervalSet look = ll1.LOOK(post, null);
		IntervalSet firstR = ll1.LOOK(atn.ruleToStartState[grp.rule], null);
		for (int t : look.toArray()) {
			if (t > 0 && firstR.contains(t)) {
				if ("descent".equals(System.getProperty("antlr.dfa.debug"))) {
					System.err.printf("WIDEN-VETO d=%d alt=%d rule=%s first=%d%n",
						s.decision, a, g.getRule(grp.rule).name, t);
				}
				return false;
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
	 * the continuation is nullable), computed by popping along the
	 * member's recorded descent chain - not grammar-wide - so phantom
	 * returns (R's own follow positions, other members' call sites)
	 * cannot pollute the continuation sets. Explicit arms are the
	 * non-nullable continuations; the default is the single nullable
	 * one. Over-approximating (full callee LOOK, grammar-wide FOLLOW at
	 * the decision rule's stop) can only reject sound groups, never
	 * accept unsound ones.
	 */
	private void analyzeDispatch(DecisionState s, Group grp) {
		LL1Analyzer ll1 = new LL1Analyzer(atn);
		Map<Integer, IntervalSet> mandatory = new HashMap<Integer, IntervalSet>();
		Set<Integer> nullable = new HashSet<Integer>();
		for (int a = grp.alts.nextSetBit(0); a >= 0; a = grp.alts.nextSetBit(a+1)) {
			Set<Integer> nul = new HashSet<Integer>();
			IntervalSet t = new IntervalSet();
			List<List<RuleTransition>> chains = grp.chainsByAlt.get(a);
			if (chains != null) {
				for (List<RuleTransition> chain : chains) {
					t.addAll(mandatoryTailChain(s, chain, ll1, nul));
				}
			}
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
	 * FIRST tokens of the mandatory continuation after R along one
	 * recorded descent chain (over-approximated by full callee LOOK);
	 * {@code nullable} is set when the walk reaches the decision rule's
	 * stop, in which case the decision rule's FOLLOW is included. The
	 * walk starts at the innermost R-call follow and pops frame by frame
	 * along the chain (a wrapper rule's stop continues at the next
	 * outer frame's follow, never grammar-wide).
	 */
	private IntervalSet mandatoryTailChain(DecisionState s, List<RuleTransition> chain,
										   LL1Analyzer ll1, Set<Integer> nullable) {
		IntervalSet tokens = new IntervalSet();
		Set<ATNState> seen = new HashSet<ATNState>();
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(chain.get(chain.size()-1).followState);
		while (!work.isEmpty()) {
			ATNState p = work.poll();
			if (!seen.add(p)) continue;
			if (p instanceof RuleStopState) {
				int idx = chainFrameOf(chain, p.ruleIndex);
				if (idx > 0) {
					work.add(chain.get(idx-1).followState);
				}
				else if (p.ruleIndex == s.ruleIndex) {
					nullable.add(1);
					tokens.addAll(ll1.LOOK(p, null));
				}
				continue;
			}
			for (int i = 0; i < p.getNumberOfTransitions(); i++) {
				Transition t = p.transition(i);
				if (t instanceof RuleTransition) {
					tokens.addAll(ll1.LOOK(t.target, null));
					work.add(((RuleTransition)t).followState);
				}
				else if (t.isEpsilon()) {
					work.add(t.target);
				}
				else if (t.label() != null) {
					tokens.addAll(t.label());
				}
			}
		}
		return tokens;
	}

	/** Index of the chain frame whose caller rule is {@code ruleIndex}
	 *  (the frame whose follow state lies in that rule), innermost
	 *  first; -1 when the rule is not a chain caller. */
	private static int chainFrameOf(List<RuleTransition> chain, int ruleIndex) {
		for (int i = chain.size()-1; i >= 0; i--) {
			if (chain.get(i).followState.ruleIndex == ruleIndex) return i;
		}
		return -1;
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
		if (segStartsViaR(c.state.ruleIndex, c.context, grp.rule, callSites, grp.boundaryStates,
			java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()))) {
			return true;
		}
		// Chain-descended group, no R segment in the chain.
		int v = checkDescent(c.state, grp.rule,
			new HashMap<Long, Integer>(), new HashSet<Long>());
		// Inert configuration: the position cannot reach R at all and
		// every deviation from it was approved by the member gate's
		// side-exit discipline - it can never interact with the descent.
		if (v == V_NOCLEAN) return true;
		// Post-first-R configuration: reaching the position consumed at
		// least one token, so the body's first R call (at the decision
		// start, served by the resume) already happened - later R calls
		// parse normally.
		if (!reachableWithoutConsumption(c.state)) return true;
		// A wrapper frame whose invocation is a post-R position (the
		// call site is reachable only through an R call) proves the
		// config's first-R span already happened, e.g. the selectItem
		// alias block, which follows the expression invocation.
		if (chainFramesPostR(c.context, grp.rule,
			java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()))) {
			return true;
		}
		// The remaining cases need the wrapper chain below to lie on
		// the epsilon descent.
		if (!allCallSitesToBoundary(c.context, callSites, grp.boundaryStates,
			java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()))) {
			return false;
		}
		// Pre-R configuration: the position still descends to R - on
		// the commit region it enters R at a clean call site (the
		// member gate's side-exit discipline keeps the body's first R
		// call aligned with the neutral span).
		if (v != V_VETO) return true;
		// Post-R configuration (e.g. a wrapper rule's stop): the
		// position is reachable only through an R call, so the config's
		// R span already happened - aligned by construction.
		return postRPosition(c.state, grp.rule);
	}

	/** Any context frame whose call site is a post-R position (the
	 *  site is reachable from its rule's start only through an R
	 *  call): the frame's invocation then happened after the first R
	 *  span, whatever the chain below. */
	private boolean chainFramesPostR(PredictionContext ctx, int ruleR, Set<PredictionContext> visited) {
		if (ctx == null || ctx.isEmpty()) return false;
		if (!visited.add(ctx)) return false;
		for (int i = 0; i < ctx.size(); i++) {
			int rs = ctx.getReturnState(i);
			if (rs == PredictionContext.EMPTY_RETURN_STATE) continue;
			ATNState site = siteStateByFollow(rs);
			if (site != null
				&& (postRPosition(site, ruleR) || !reachableWithoutConsumption(site))) {
				return true;
			}
			if (chainFramesPostR(ctx.getParent(i), ruleR, visited)) return true;
		}
		return false;
	}

	/** Is {@code state} reachable from its rule's start consuming zero
	 *  tokens? The walk follows epsilon transitions, descends into
	 *  callees, and continues past a callee only when the callee can
	 *  complete token-free (nullable). When it is not, the position
	 *  postdates the body's first R call at the decision start. */
	private boolean reachableWithoutConsumption(ATNState state) {
		Set<ATNState> seen = new HashSet<ATNState>();
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(atn.ruleToStartState[state.ruleIndex]);
		while (!work.isEmpty()) {
			ATNState st = work.poll();
			if (!seen.add(st)) continue;
			if (st == state) return true;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					RuleTransition rt = (RuleTransition)t;
					work.add(rt.target);
					if (nullableOf(rt.target.ruleIndex)) work.add(rt.followState);
				}
				else if (t.isEpsilon()) {
					work.add(t.target);
				}
			}
		}
		return false;
	}

	/** Rule nullability (can complete with zero tokens), cached. */
	private final Map<Integer, Boolean> ruleNullable = new HashMap<Integer, Boolean>();
	private boolean nullableOf(int rule) {
		Boolean n = ruleNullable.get(rule);
		if (n == null) {
			n = new LL1Analyzer(atn).LOOK(atn.ruleToStartState[rule], null)
				.contains(org.antlr.v4.runtime.Token.EPSILON);
			ruleNullable.put(rule, n);
		}
		return n;
	}

	/** Lazily built: a rule call's follow state number -> the source
	 *  state owning the {@link RuleTransition} (unique calls only; a
	 *  follow shared by multiple call sites maps to no entry). */
	private Map<Integer, ATNState> siteStateByFollow;
	private ATNState siteStateByFollow(int followState) {
		if (siteStateByFollow == null) {
			siteStateByFollow = new HashMap<Integer, ATNState>();
			Set<Integer> dup = new HashSet<Integer>();
			for (ATNState st : atn.states) {
				if (st == null) continue;
				for (int i = 0; i < st.getNumberOfTransitions(); i++) {
					Transition t = st.transition(i);
					if (t instanceof RuleTransition) {
						int f = ((RuleTransition)t).followState.stateNumber;
						if (siteStateByFollow.put(f, st) != null) dup.add(f);
					}
				}
			}
			siteStateByFollow.keySet().removeAll(dup);
		}
		return siteStateByFollow.get(followState);
	}

	/** Is {@code state} reachable from its rule's start only through an
	 *  R invocation? The walk explores epsilon and token transitions
	 *  (R-free consumption) and non-R calls (continuing past a callee
	 *  only when it may complete without R - an ALWAYS_R callee called
	 *  R to get there); R calls are barriers. */
	private boolean postRPosition(ATNState state, int ruleR) {
		Map<Long, Integer> memo = new HashMap<Long, Integer>();
		Set<Long> inProgress = new HashSet<Long>();
		Set<ATNState> seen = new HashSet<ATNState>();
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(atn.ruleToStartState[state.ruleIndex]);
		while (!work.isEmpty()) {
			ATNState st = work.poll();
			if (!seen.add(st)) continue;
			if (st == state) return false;
			if (st instanceof RuleStopState) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					RuleTransition rt = (RuleTransition)t;
					if (rt.target.ruleIndex == ruleR) continue; // R is a barrier
					work.add(rt.target);
					if (checkDescent(rt.target, ruleR, memo, inProgress) != V_ALWAYS) {
						work.add(rt.followState);
					}
				}
				else {
					work.add(t.target);
				}
			}
		}
		return true;
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
		if ("descent".equals(System.getProperty("antlr.dfa.debug")) && !callSitesCache.containsKey(key)) {
			System.err.printf("CALLSITES-G d=%d rule=%s%n", s.decision, g.getRule(grp.rule).name);
		}
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
						// the descent continues after the call too: chains
						// like identifier -> over -> identifier pass through
						work.add(rt.followState);
						if (rt.target.ruleIndex != grp.rule) work.add(t.target);
					}
					else if (t.isEpsilon()) {
						work.add(t.target);
					}
				}
			}
			callSitesCache.put(key, sites);
			if ("descent".equals(System.getProperty("antlr.dfa.debug"))) {
				System.err.printf("CALLSITES-GS d=%d rule=%s sites=%s%n", s.decision,
					g.getRule(grp.rule).name, sites);
			}
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
						work.add(rt.followState);
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
	 *  state ({@link #callSite}), the R-call's follow state
	 *  ({@link #boundary} - the prefix-side end of the first-consumer
	 *  proof), and the full wrapper call chain from the decision
	 *  alternative to the R call (outermost first, the R call last -
	 *  the dispatch analysis pops along it after the neutral parse). */
	private static final class AltPath {
		final List<Integer> tokens = new ArrayList<Integer>();
		int alt = -1;
		int rule = -1;
		int callSite = -1;
		int boundary = -1;
		final List<RuleTransition> chain = new ArrayList<RuleTransition>();
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
		work.add(new Object[]{s.transition(alt-1).target, new ArrayList<Integer>(), Boolean.FALSE,
			new ArrayList<RuleTransition>()});
		Set<String> seen = new HashSet<String>();
		while (!work.isEmpty()) {
			Object[] item = work.poll();
			ATNState st = (ATNState)item[0];
			@SuppressWarnings("unchecked") List<Integer> tokens = (List<Integer>)item[1];
			boolean inRule = (Boolean)item[2];
			@SuppressWarnings("unchecked") List<RuleTransition> chain = (List<RuleTransition>)item[3];
			String key = st.stateNumber + "|" + tokens + (inRule ? "r" : "");
			if (!seen.add(key) || st instanceof RuleStopState) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					RuleTransition rt = (RuleTransition)t;
					List<RuleTransition> nextChain = new ArrayList<RuleTransition>(chain);
					nextChain.add(rt);
					AltPath p = new AltPath();
					p.alt = alt;
					p.tokens.addAll(tokens);
					p.rule = rt.target.ruleIndex;
					p.callSite = st.stateNumber;
					p.boundary = rt.followState.stateNumber;
					p.chain.addAll(nextChain);
					out.add(p);
					work.add(new Object[]{t.target, tokens, Boolean.TRUE, nextChain});
				}
				else if (t.isEpsilon()) {
					work.add(new Object[]{t.target, tokens, inRule, chain});
				}
				else if (!inRule && t.label() != null && tokens.size() < MAX_PREFIX_TOKENS) {
					for (int tok : t.label().toList()) {
						List<Integer> next = new ArrayList<Integer>(tokens);
						next.add(tok);
						work.add(new Object[]{t.target, next, Boolean.FALSE, chain});
					}
				}
			}
		}
		return out;
	}

	/** Is R the first consuming position of alternative {@code alt}
	 *  after its prefix on every input where a neutral R parse succeeds
	 *  (the descent block's commit region)? The walk descends from the
	 *  post-prefix start through epsilon transitions and wrapper-rule
	 *  calls; an R call ends a path successfully. Every side exit - a
	 *  token or a non-R call that would let the body consume (or
	 *  complete) before its first R call - must be unreachable on the
	 *  commit region:
	 *
	 *  <ul>
	 *  <li>a side token disjoint from FIRST(R) is never consumed when
	 *      the neutral R parse succeeds;</li>
	 *  <li>a side call whose FIRST is disjoint from FIRST(R)'s
	 *      likewise;</li>
	 *  <li>an overlapping side token is still never taken when the
	 *      enclosing block's R-ward alternative precedes it: both are
	 *      viable on the commit region, so adaptivePredict's min-alt
	 *      rule picks the R-ward one (e.g. valueExpression's
	 *      primaryExpression base alternative beats the MINUS/PLUS
	 *      unary alternatives, which overlap FIRST(primaryExpression)
	 *      only on signed numerics);</li>
	 *  <li>everything else - an overlapping side call, an R-less
	 *      completion, an overlapping token without R-ward priority -
	 *      mis-aligns the body's first R call from the neutral parse's
	 *      span and vetoes the member (cf. the TRIM from-form family:
	 *      BOTH starts both trimsSpecification and a valueExpression,
	 *      so the neutral parse can consume as R content a token the
	 *      body consumes before its first R call).
	 *  </ul>
	 *
	 *  With {@code -Dantlr.dfa.disableChainDescent=1} the walk reverts
	 *  to the shallow discipline (no wrapper descent: every token, non-R
	 *  call, or rule end before R vetoes). */
	private boolean firstConsumerUnique(DecisionState s, int alt, AltPath p) {
		ATNState start = postPrefixState(s, alt, p.tokens);
		if (start == null) return false;
		if (System.getProperty("antlr.dfa.disableChainDescent") != null) {
			return firstConsumerUniqueShallow(start, p.rule);
		}
		return checkDescent(start, p.rule,
			new HashMap<Long, Integer>(), new HashSet<Long>()) == V_ALWAYS;
	}

	/** The pre-chain-descent member gate: every epsilon path from
	 *  {@code start} must reach an R call before any token, any non-R
	 *  call, or the rule end. */
	private boolean firstConsumerUniqueShallow(ATNState start, int ruleR) {
		Set<ATNState> seen = new HashSet<ATNState>();
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(start);
		boolean any = false;
		while (!work.isEmpty()) {
			ATNState st = work.poll();
			if (!seen.add(st)) continue;
			if (st instanceof RuleStopState) return false;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					if (((RuleTransition)t).target.ruleIndex != ruleR) return false;
					any = true;
				}
				else if (t.isEpsilon()) {
					work.add(t.target);
				}
				else {
					return false;
				}
			}
		}
		return any;
	}

	/** Descent verdicts of {@link #checkDescent}. */
	private static final int V_VETO = 0;
	private static final int V_ALWAYS = 1;
	private static final int V_SOMETIMES = 2;
	private static final int V_NOCLEAN = 3;

	/** Verdict of the descent from {@code start} (a state inside one
	 *  rule body, no consumption so far) to the body's first R call:
	 *  V_ALWAYS when every path that survives the side-exit checks calls
	 *  R before completing, V_SOMETIMES when R is called on some path
	 *  but a survivable R-less completion also exists, V_NOCLEAN when no
	 *  clean path to R exists but every side exit was approved (the
	 *  position is unreachable on the commit region - exempt, not a
	 *  veto), V_VETO when a side exit can be taken on the commit region.
	 *  Memoized per (R, start state); genuine recursion on the clean
	 *  descent vetoes (conservative). */
	private int checkDescent(ATNState start, int ruleR,
							 Map<Long, Integer> memo, Set<Long> inProgress) {
		Long mkey = (((long)ruleR) << 32) | start.stateNumber;
		Integer m = memo.get(mkey);
		if (m != null) return m;
		if (!inProgress.add(mkey)) return V_VETO;
		IntervalSet firstR = firstOfRule(ruleR);
		boolean foundR = false;
		boolean rLess = false;
		boolean veto = false;
		Set<ATNState> seen = new HashSet<ATNState>();
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(start);
		while (!work.isEmpty() && !veto) {
			ATNState st = work.poll();
			if (!seen.add(st)) continue;
			if (st instanceof RuleStopState) { rLess = true; continue; }
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					RuleTransition rt = (RuleTransition)t;
					int callee = rt.target.ruleIndex;
					if (callee == ruleR) { foundR = true; continue; }
					if (firstOfRule(callee).and(firstR).isNil()) continue; // disjoint side call
					int v = checkDescent(rt.target, ruleR, memo, inProgress);
					if (v == V_VETO) { veto = true; break; }
					foundR = true;
					if (v == V_SOMETIMES) work.add(rt.followState);
				}
				else if (t.isEpsilon()) {
					work.add(t.target);
				}
				else if (t.label() == null) {
					work.add(t.target); // actions: no token consumption
				}
				else if (t.label().and(firstR).isNil()) {
					continue; // disjoint side token
				}
				else if (!tokenPriority(st, ruleR)) {
					veto = true;
				}
			}
		}
		inProgress.remove(mkey);
		int v = veto ? V_VETO : !foundR ? V_NOCLEAN : (rLess ? V_SOMETIMES : V_ALWAYS);
		memo.put(mkey, v);
		return v;
	}

	/** FIRST set of a rule (epsilon stripped), cached. */
	private final Map<Integer, IntervalSet> ruleFirst = new HashMap<Integer, IntervalSet>();
	private IntervalSet firstOfRule(int rule) {
		IntervalSet f = ruleFirst.get(rule);
		if (f == null) {
			f = new LL1Analyzer(atn).LOOK(atn.ruleToStartState[rule], null);
			f.remove(org.antlr.v4.runtime.Token.EPSILON);
			ruleFirst.put(rule, f);
		}
		return f;
	}

	/** Min-alt priority for an overlapping side token at {@code st}:
	 *  the innermost block enclosing the token must have an R-ward
	 *  alternative (reaching R with zero token consumption) preceding
	 *  the token's alternative; on the commit region both are viable
	 *  and adaptivePredict's min-alt rule then takes the R-ward one,
	 *  keeping the body's first R call aligned with the neutral span. */
	private boolean tokenPriority(ATNState st, int ruleR) {
		Long ba = enclosingAlt(st);
		if (ba == null) return false;
		int blk = (int)(ba >> 32);
		int altIdx = (int)(ba & 0xffffffffL);
		ATNState bs = atn.states.get(blk);
		if (!(bs instanceof BlockStartState)) return false;
		int clean = cleanAltOfBlock((BlockStartState)bs, ruleR);
		return clean != 0 && clean < altIdx;
	}

	/** Minimum alternative index of block {@code b} through which R is
	 *  reachable with zero token consumption, or 0 when none. */
	private int cleanAltOfBlock(BlockStartState b, int ruleR) {
		for (int i = 0; i < b.getNumberOfTransitions(); i++) {
			if (reachesRuleEpsilon(b.transition(i).target, ruleR, new HashSet<ATNState>())) {
				return i + 1;
			}
		}
		return 0;
	}

	/** Can R be reached from {@code start} following epsilon
	 *  transitions and rule calls (descending into callees and
	 *  continuing past their returns) with zero token consumption? */
	private boolean reachesRuleEpsilon(ATNState start, int ruleR, Set<ATNState> seen) {
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		work.add(start);
		while (!work.isEmpty()) {
			ATNState st = work.poll();
			if (!seen.add(st) || st instanceof RuleStopState) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition) {
					RuleTransition rt = (RuleTransition)t;
					if (rt.target.ruleIndex == ruleR) return true;
					work.add(rt.target);
					work.add(rt.followState);
				}
				else if (t.isEpsilon()) {
					work.add(t.target);
				}
			}
		}
		return false;
	}

	/** Innermost enclosing (BlockStartState state number, alternative
	 *  index) pair per ATN state, packed into a long; states outside
	 *  any block have no entry. */
	private final Map<Integer, Map<ATNState, Long>> blockAltCache = new HashMap<Integer, Map<ATNState, Long>>();
	private Long enclosingAlt(ATNState st) {
		Map<ATNState, Long> m = blockAltCache.get(st.ruleIndex);
		if (m == null) {
			m = computeEnclosingAlts(st.ruleIndex);
			blockAltCache.put(st.ruleIndex, m);
		}
		return m.get(st);
	}

	/** One stack-carrying DFS over a rule's own states, recording the
	 *  innermost (block, alt) of every state. */
	private Map<ATNState, Long> computeEnclosingAlts(int rule) {
		Map<ATNState, Long> m = new HashMap<ATNState, Long>();
		Deque<Object[]> work = new ArrayDeque<Object[]>();
		work.add(new Object[]{atn.ruleToStartState[rule], new ArrayList<Long>()});
		Set<ATNState> seen = new HashSet<ATNState>();
		while (!work.isEmpty()) {
			Object[] it = work.poll();
			ATNState st = (ATNState)it[0];
			@SuppressWarnings("unchecked") List<Long> stack = (List<Long>)it[1];
			if (!seen.add(st)) continue;
			if (!stack.isEmpty()) m.put(st, stack.get(stack.size()-1));
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof RuleTransition || !t.isEpsilon()) continue; // intra-rule only
				List<Long> next = stack;
				if (st instanceof BlockStartState) {
					next = new ArrayList<Long>(stack);
					next.add(((long)st.stateNumber << 32) | (i + 1));
				}
				else if (st instanceof BlockEndState) {
					if (!stack.isEmpty()) {
						next = new ArrayList<Long>(stack);
						next.remove(next.size()-1);
					}
				}
				work.add(new Object[]{t.target, next});
			}
		}
		return m;
	}

	/** Can R be reached from {@code start} following epsilon
	 *  transitions and rule calls only (zero tokens)? */

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
