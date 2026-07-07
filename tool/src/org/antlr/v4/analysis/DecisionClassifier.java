/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.analysis;

import org.antlr.v4.runtime.Token;
import org.antlr.v4.runtime.atn.ATN;
import org.antlr.v4.runtime.atn.ATNConfig;
import org.antlr.v4.runtime.atn.ATNConfigSet;
import org.antlr.v4.runtime.atn.ATNState;
import org.antlr.v4.runtime.atn.DecisionState;
import org.antlr.v4.runtime.atn.EmptyPredictionContext;
import org.antlr.v4.runtime.atn.LoopEndState;
import org.antlr.v4.runtime.atn.NotSetTransition;
import org.antlr.v4.runtime.atn.PrecedencePredicateTransition;
import org.antlr.v4.runtime.atn.PredicateTransition;
import org.antlr.v4.runtime.atn.PredictionContext;
import org.antlr.v4.runtime.atn.PredictionMode;
import org.antlr.v4.runtime.atn.RuleStopState;
import org.antlr.v4.runtime.atn.RuleTransition;
import org.antlr.v4.runtime.atn.SingletonPredictionContext;
import org.antlr.v4.runtime.atn.StarLoopEntryState;
import org.antlr.v4.runtime.atn.Transition;
import org.antlr.v4.runtime.atn.WildcardTransition;
import org.antlr.v4.runtime.misc.Interval;
import org.antlr.v4.runtime.misc.IntervalSet;
import org.antlr.v4.tool.Grammar;
import org.antlr.v4.tool.Rule;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.BitSet;
import java.util.Collection;
import java.util.Deque;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedHashMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;

/**
 * Classifies every parser decision by the kind of <em>static</em> lookahead
 * machinery required to predict it without invoking {@code adaptivePredict}
 * at runtime.
 *
 * <p>The classifier works by exhaustively constructing, at tool time, the SLL
 * prediction DFA that {@link org.antlr.v4.runtime.atn.ParserATNSimulator}
 * would otherwise build lazily at runtime. SLL prediction start states are
 * context-free (the runtime computes them with a wildcard outer context and
 * caches the resulting DFA across all invocation contexts), so the complete
 * SLL DFA is statically computable. Conflict detection reuses the runtime's
 * {@link PredictionMode} machinery so the verdicts match runtime SLL-mode
 * behavior exactly.</p>
 *
 * <p>Termination of the exhaustive construction is guaranteed by widening:
 * when closure would push a rule-invocation follow state that already appears
 * {@link #maxContextStateOccurrences} times in a configuration's calling
 * context chain (i.e., recursion), the deeper stack is collapsed to the
 * wildcard context. This over-approximates the exact SLL configuration space
 * (it is what turns unbounded-lookahead decisions into <em>cyclic</em> DFAs
 * instead of infinite ones), and results that relied on widening are marked
 * {@link Result#usedWidening}.</p>
 */
public class DecisionClassifier {
	public enum Category {
		/** SLL(1) with disjoint sets; the existing LL(1) codegen fast path applies. */
		LL1,
		/** Acyclic SLL DFA; decidable with at most {@link Result#k} tokens of lookahead. */
		LLK,
		/** Cyclic SLL DFA; decidable with unbounded but regular lookahead (ANTLR3-style LL(*)). */
		LLSTAR,
		/**
		 * DFA construction reached one or more states where all viable
		 * alternatives conflict identically (an exact ambiguity), and none of
		 * the conflicting configurations descend from a widened (recursion-
		 * collapsed) context. Exactness over untainted configurations is
		 * preserved under any real outer context (the wildcard bottom is
		 * substituted uniformly across all alternatives), so this is a true
		 * grammar ambiguity: runtime SLL and full-context LL both resolve it
		 * to the minimum alternative, and a static min-alt resolution is
		 * behavior-preserving.
		 */
		EXACT_AMBIG,
		/**
		 * DFA construction reached a conflict that cannot be trusted as an
		 * exact ambiguity: either a non-exact SLL conflict (runtime ALL(*)
		 * would retry with full context and may pick a different alternative
		 * than min-alt), or a conflict involving configurations tainted by
		 * widening (the conflict may be an analysis artifact that the runtime
		 * - even in SLL mode - never encounters, cf. PERMUTE(a, b) vs
		 * pattern-variable ambiguities). Static min-alt resolution at such
		 * states can reject or misparse inputs the runtime handles.
		 */
		CONTEXT_SENSITIVE,
		/** Lookahead traverses a semantic predicate; not statically decidable. */
		PREDICATED,
		/** Non-greedy decisions are never statically predicted. */
		NON_GREEDY,
		/** The precedence loop decision of a left-recursive rule; stays on adaptivePredict. */
		LR_PRECEDENCE,
		/** DFA construction exceeded {@link #maxDfaStates}. */
		OVERFLOW
	}

	public static class Result {
		public final DecisionState decisionState;
		public Category category;
		/** Max lookahead depth (tokens) if the DFA is acyclic; -1 if cyclic or unknown. */
		public int k = -1;
		public int numDfaStates;
		public boolean usedWidening;
		/**
		 * Context-depth bound at which DFA construction converged
		 * (Integer.MAX_VALUE = exact construction, widened only at direct
		 * call-site recursion).
		 */
		public int wideningDepth = Integer.MAX_VALUE;
		public boolean sawPredicate;
		public boolean sawPrecPredicate;
		/** Trusted exact ambiguities: exact conflicts over untainted configs; min-alt is behavior-preserving. */
		public final List<BitSet> exactAmbigConflicts = new ArrayList<BitSet>();
		/** Non-exact conflicts over untainted configs: genuine SLL/LL divergence candidates. */
		public final List<BitSet> contextSensitiveConflicts = new ArrayList<BitSet>();
		/** Conflicts involving widening-tainted configs: possibly analysis artifacts; untrusted. */
		public final List<BitSet> approxConflicts = new ArrayList<BitSet>();

		public Result(DecisionState decisionState) {
			this.decisionState = decisionState;
		}
	}

	/**
	 * Taint bits stored in {@link ATNConfig#reachesIntoOuterContext} marking
	 * configurations derived through an over-approximation. The field is
	 * chosen because every ATNConfig copy constructor propagates it,
	 * ATNConfigSet's merge takes the max of it, and equals/hashCode ignore it
	 * - so the taint flows to closure descendants and survives merging
	 * without affecting DFA state identity. (This classifier never uses the
	 * field's runtime meaning; note max-merge is bitwise-correct here only
	 * because tainted values compare greater than untainted ones and we only
	 * test for non-zero.)
	 *
	 * <p>Over-approximations are one-sided: they can only keep alternatives
	 * alive longer than the runtime would, so uniquely-predicted accepts
	 * remain correct, but a conflict involving tainted configurations may be
	 * an analysis artifact the runtime never encounters and must not be
	 * trusted as an exact ambiguity.</p>
	 */
	protected static final int WIDENED_TAINT = 1;

	/**
	 * Taint for configurations that traversed a precedence predicate as
	 * epsilon (i.e., assumed {@code precpred(...)} true). Same one-sided
	 * soundness argument as {@link #WIDENED_TAINT}.
	 */
	protected static final int PRECPRED_TAINT = 2;

	/** All taint bits that make a conflict untrusted. */
	protected static final int ANY_TAINT = WIDENED_TAINT | PRECPRED_TAINT;

	/** Hard cap on DFA states per decision (per construction attempt). */
	public int maxDfaStates = 2000;

	/**
	 * Iterative-deepening schedule for the calling-context depth bound. The
	 * first attempt is exact (widening only at direct call-site recursion,
	 * which is required for termination and is what yields cyclic LL(*)
	 * DFAs); when an attempt overflows {@link #maxDfaStates}, construction is
	 * retried with progressively tighter widening.
	 *
	 * <p>Deep-but-not-recursive-through-the-same-call-site context chains
	 * (typical of expression precedence ladders) make the exact SLL
	 * configuration space explode combinatorially; bounding the depth
	 * collapses deeper stacks to the wildcard context. Depth 0 is the fully
	 * context-free approximation (ANTLR3-style linear approximate lookahead
	 * over token classes): every rule invocation keeps only its immediate
	 * return frame, which bounds the configuration space by construction.
	 * Widening errors are one-sided: a widened DFA can only produce spurious
	 * conflicts (resolved min-alt), never a wrong uniquely-predicted
	 * alternative.</p>
	 */
	public int[] wideningDepths = { Integer.MAX_VALUE, 0 };

	/** Wall-clock budget per construction attempt; exceeding it counts as overflow. */
	public long attemptBudgetMs = 5000;

	protected final Grammar g;
	protected final ATN atn;
	protected final IntervalSet allTokens;

	/** Depth bound of the construction attempt in progress. */
	protected int currentDepthBound = Integer.MAX_VALUE;
	/** Per-attempt memo for {@link #contextDepth} (contexts share structure). */
	protected Map<PredictionContext, Integer> depthCache;

	public DecisionClassifier(Grammar g) {
		this.g = g;
		this.atn = g.atn;
		this.allTokens = IntervalSet.of(Token.MIN_USER_TOKEN_TYPE, atn.maxTokenType);
	}

	/** Classify all decisions of the grammar, indexed by decision number. */
	public List<Result> classifyAll() {
		List<Result> results = new ArrayList<Result>(atn.decisionToState.size());
		for (DecisionState s : atn.decisionToState) {
			results.add(classify(s));
		}
		return results;
	}

	public Result classify(DecisionState s) {
		Result res = new Result(s);
		if (s.nonGreedy) {
			res.category = Category.NON_GREEDY;
			return res;
		}
		if (isPrecedenceLoopDecision(s)) {
			res.category = Category.LR_PRECEDENCE;
			return res;
		}
		if (g.decisionLOOK != null && s.decision < g.decisionLOOK.size()
			&& AnalysisPipeline.disjoint(g.decisionLOOK.get(s.decision))) {
			res.category = Category.LL1;
			res.k = 1;
			return res;
		}

		// iterative deepening: exact first, then progressively wider approximation
		Result last = res;
		for (int depthBound : wideningDepths) {
			Result attempt = new Result(s);
			attempt.wideningDepth = depthBound;
			if (!buildDFA(s, attempt, depthBound)) {
				return attempt;
			}
			last = attempt;
		}
		last.category = Category.OVERFLOW;
		return last;
	}

	/**
	 * Replicates the runtime's ATNDeserializer#markPrecedenceDecisions logic
	 * on the tool-side ATN: the star loop entry of the operator loop that the
	 * left-recursion transformer generates.
	 */
	protected boolean isPrecedenceLoopDecision(DecisionState s) {
		if (!(s instanceof StarLoopEntryState)) return false;
		if (!atn.ruleToStartState[s.ruleIndex].isLeftRecursiveRule) return false;
		ATNState maybeLoopEndState = s.transition(s.getNumberOfTransitions()-1).target;
		if (!(maybeLoopEndState instanceof LoopEndState)) return false;
		return maybeLoopEndState.epsilonOnlyTransitions
			&& maybeLoopEndState.transition(0).target instanceof RuleStopState;
	}

	// ---------------------------------------------------------------------
	// Static SLL DFA construction
	// ---------------------------------------------------------------------

	/** One construction attempt at the given context-depth bound; true = overflow. */
	protected boolean buildDFA(DecisionState s, Result res, int depthBound) {
		this.currentDepthBound = depthBound;
		this.depthCache = new java.util.IdentityHashMap<PredictionContext, Integer>();
		Map<Set<ATNConfig>, Integer> stateIds = new HashMap<Set<ATNConfig>, Integer>();
		List<Set<ATNConfig>> states = new ArrayList<Set<ATNConfig>>();
		List<List<Integer>> edges = new ArrayList<List<Integer>>();
		List<Boolean> terminal = new ArrayList<Boolean>(); // accept or conflict; no expansion

		Set<ATNConfig> start = new LinkedHashSet<ATNConfig>();
		Set<ATNConfig> startBusy = new HashSet<ATNConfig>();
		for (int i = 0; i < s.getNumberOfTransitions(); i++) {
			closure(new ATNConfig(s.transition(i).target, i+1, EmptyPredictionContext.Instance),
					start, startBusy, res);
		}
		start = canonical(start);
		stateIds.put(start, 0);
		states.add(start);
		edges.add(new ArrayList<Integer>());
		terminal.add(Boolean.FALSE);

		Deque<Integer> work = new ArrayDeque<Integer>();
		work.add(0);
		boolean overflow = false;
		long deadline = System.currentTimeMillis() + attemptBudgetMs;

		while (!work.isEmpty() && !overflow) {
			if (System.currentTimeMillis() > deadline) { overflow = true; break; }
			int d = work.remove();
			Set<ATNConfig> configs = states.get(d);
			ATNConfigSet cs = toConfigSet(configs);

			BitSet alts = PredictionMode.getAlts(cs);
			if (alts.cardinality() <= 1) {
				terminal.set(d, Boolean.TRUE); // accept (or dead) state
				continue;
			}
			if (PredictionMode.hasSLLConflictTerminatingPrediction(PredictionMode.SLL, cs)) {
				terminal.set(d, Boolean.TRUE);
				Collection<BitSet> altSubsets = PredictionMode.getConflictingAltSubsets(cs);
				boolean exact = PredictionMode.allSubsetsConflict(altSubsets)
					&& PredictionMode.allSubsetsEqual(altSubsets);
				boolean tainted = false;
				for (ATNConfig c : cs) {
					if ((c.reachesIntoOuterContext & ANY_TAINT) != 0) { tainted = true; break; }
				}
				BitSet conflicting = PredictionMode.getAlts(altSubsets);
				if (tainted) res.approxConflicts.add(conflicting);
				else if (exact) res.exactAmbigConflicts.add(conflicting);
				else res.contextSensitiveConflicts.add(conflicting);
				continue;
			}

			for (Set<ATNConfig> succ : successors(configs, res)) {
				Integer id = stateIds.get(succ);
				if (id == null) {
					if (states.size() >= maxDfaStates) { overflow = true; break; }
					id = states.size();
					stateIds.put(succ, id);
					states.add(succ);
					edges.add(new ArrayList<Integer>());
					terminal.add(Boolean.FALSE);
					work.add(id);
				}
				edges.get(d).add(id);
			}
		}

		res.numDfaStates = states.size();
		if (overflow) {
			res.category = Category.OVERFLOW;
			return true;
		}
		if (res.sawPredicate) {
			res.category = Category.PREDICATED;
			return false;
		}

		boolean cyclic = isCyclic(edges);
		if (!cyclic) res.k = longestPath(edges);

		if (!res.contextSensitiveConflicts.isEmpty() || !res.approxConflicts.isEmpty()) {
			res.category = Category.CONTEXT_SENSITIVE;
		}
		else if (!res.exactAmbigConflicts.isEmpty()) res.category = Category.EXACT_AMBIG;
		else if (cyclic) res.category = Category.LLSTAR;
		else res.category = Category.LLK;
		return false;
	}

	/**
	 * Compute the distinct successor configuration sets of a DFA state, one
	 * per equivalence class of tokens (never per token). The labels
	 * themselves are not retained; Phase 0 only needs the DFA's shape.
	 */
	protected Collection<Set<ATNConfig>> successors(Set<ATNConfig> configs, Result res) {
		// collect non-epsilon moves
		List<IntervalSet> labels = new ArrayList<IntervalSet>();
		List<ATNConfig> moveConfigs = new ArrayList<ATNConfig>();
		List<ATNState> moveTargets = new ArrayList<ATNState>();
		List<Boolean> moveIsStop = new ArrayList<Boolean>();
		for (ATNConfig c : configs) {
			if (c.state instanceof RuleStopState) {
				// Stop state with wildcard context: like the runtime's reach
				// computation, such a config survives only the EOF symbol and
				// is preserved verbatim (no closure past EOF).
				labels.add(IntervalSet.of(Token.EOF));
				moveConfigs.add(c);
				moveTargets.add(c.state);
				moveIsStop.add(Boolean.TRUE);
				continue;
			}
			for (int i = 0; i < c.state.getNumberOfTransitions(); i++) {
				Transition t = c.state.transition(i);
				if (t.isEpsilon()) continue;
				IntervalSet label;
				if (t instanceof NotSetTransition) {
					label = ((NotSetTransition)t).set.complement(allTokens);
				}
				else if (t instanceof WildcardTransition) {
					label = allTokens;
				}
				else {
					label = t.label();
				}
				if (label == null || label.isNil()) continue;
				labels.add(label);
				moveConfigs.add(c);
				moveTargets.add(t.target);
				moveIsStop.add(Boolean.FALSE);
			}
		}

		// partition the token space into equivalence classes over move labels
		TreeSet<Integer> boundaries = new TreeSet<Integer>();
		for (IntervalSet label : labels) {
			for (Interval iv : label.getIntervals()) {
				boundaries.add(iv.a);
				boundaries.add(iv.b+1);
			}
		}

		Map<BitSet, Set<ATNConfig>> byMoveKey = new LinkedHashMap<BitSet, Set<ATNConfig>>();
		for (int p : boundaries) {
			BitSet key = new BitSet(labels.size());
			for (int m = 0; m < labels.size(); m++) {
				if (labels.get(m).contains(p)) key.set(m);
			}
			if (key.isEmpty() || byMoveKey.containsKey(key)) continue;
			Set<ATNConfig> succ = new LinkedHashSet<ATNConfig>();
			Set<ATNConfig> busy = new HashSet<ATNConfig>();
			for (int m = key.nextSetBit(0); m >= 0; m = key.nextSetBit(m+1)) {
				ATNConfig c = moveConfigs.get(m);
				if (moveIsStop.get(m)) {
					succ.add(c);
				}
				else {
					closure(new ATNConfig(c, moveTargets.get(m)), succ, busy, res);
				}
			}
			byMoveKey.put(key, canonical(succ));
		}
		// distinct successor sets (different token classes may close to the same set)
		return new LinkedHashSet<Set<ATNConfig>>(byMoveKey.values());
	}

	/**
	 * SLL epsilon closure, mirroring ParserATNSimulator's closure: predicates
	 * are traversed (and recorded), rule invocations push a singleton context
	 * (widened on recursion), rule stops pop or - with a wildcard context -
	 * chase the grammar-wide FOLLOW links the tool attaches to rule stop states.
	 */
	protected void closure(ATNConfig config, Set<ATNConfig> configs, Set<ATNConfig> busy, Result res) {
		if (!busy.add(config)) return;
		ATNState p = config.state;

		if (p instanceof RuleStopState) {
			PredictionContext ctx = config.context;
			if (ctx != null && !ctx.isEmpty()) {
				for (int i = 0; i < ctx.size(); i++) {
					if (ctx.getReturnState(i) == PredictionContext.EMPTY_RETURN_STATE) {
						closure(new ATNConfig(config, p, EmptyPredictionContext.Instance), configs, busy, res);
					}
					else {
						ATNState returnState = atn.states.get(ctx.getReturnState(i));
						closure(new ATNConfig(config, returnState, ctx.getParent(i)), configs, busy, res);
					}
				}
				return;
			}
			// Wildcard context: keep the stop-state config (it participates in
			// stop-state conflict detection and matches EOF), and additionally
			// chase FOLLOW links via the epsilon transitions below.
			configs.add(config);
		}

		if (!p.onlyHasEpsilonTransitions()) {
			configs.add(config);
		}

		for (int i = 0; i < p.getNumberOfTransitions(); i++) {
			Transition t = p.transition(i);
			if (t instanceof RuleTransition) {
				RuleTransition rt = (RuleTransition)t;
				int followStateNumber = rt.followState.stateNumber;
				PredictionContext newCtx;
				boolean widened = false;
				if (currentDepthBound == 0) {
					// context-free approximation: no frames at all, returns
					// resolve through the grammar-wide FOLLOW links
					widened = true;
					newCtx = EmptyPredictionContext.Instance;
				}
				else if (contextDepth(config.context) >= currentDepthBound
					|| hasReturnState(config.context, followStateNumber)) {
					// call-site recursion or depth bound reached: keep the
					// immediate return but widen the remaining stack to the
					// wildcard context
					widened = true;
					newCtx = SingletonPredictionContext.create(EmptyPredictionContext.Instance, followStateNumber);
				}
				else {
					newCtx = SingletonPredictionContext.create(config.context, followStateNumber);
				}
				ATNConfig callee = new ATNConfig(config, rt.target, newCtx);
				if (widened) {
					res.usedWidening = true;
					callee.reachesIntoOuterContext |= WIDENED_TAINT;
				}
				closure(callee, configs, busy, res);
			}
			else if (t instanceof PrecedencePredicateTransition) {
				// traversed as epsilon = assumed true: over-approximation
				res.sawPrecPredicate = true;
				ATNConfig c2 = new ATNConfig(config, t.target);
				c2.reachesIntoOuterContext |= PRECPRED_TAINT;
				closure(c2, configs, busy, res);
			}
			else if (t instanceof PredicateTransition) {
				res.sawPredicate = true;
				closure(new ATNConfig(config, t.target), configs, busy, res);
			}
			else if (t.isEpsilon()) {
				closure(new ATNConfig(config, t.target), configs, busy, res);
			}
		}
	}

	/** Does a return state appear anywhere in a (possibly DAG-shaped) context? */
	protected static boolean hasReturnState(PredictionContext ctx, int returnStateNumber) {
		return hasReturnState(ctx, returnStateNumber,
			java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()));
	}

	private static boolean hasReturnState(PredictionContext ctx, int returnStateNumber, Set<PredictionContext> visited) {
		if (ctx == null || ctx.isEmpty() || !visited.add(ctx)) return false;
		for (int i = 0; i < ctx.size(); i++) {
			if (ctx.getReturnState(i) == returnStateNumber) return true;
			if (ctx.getReturnState(i) != PredictionContext.EMPTY_RETURN_STATE
				&& hasReturnState(ctx.getParent(i), returnStateNumber, visited)) {
				return true;
			}
		}
		return false;
	}

	/** Max chain length over any path of a (possibly DAG-shaped) context; memoized per attempt. */
	protected int contextDepth(PredictionContext ctx) {
		if (ctx == null || ctx.isEmpty()) return 0;
		Integer cached = depthCache.get(ctx);
		if (cached != null) return cached;
		int max = 0;
		for (int i = 0; i < ctx.size(); i++) {
			if (ctx.getReturnState(i) == PredictionContext.EMPTY_RETURN_STATE) continue;
			max = Math.max(max, 1 + contextDepth(ctx.getParent(i)));
		}
		depthCache.put(ctx, max);
		return max;
	}

	protected static ATNConfigSet toConfigSet(Set<ATNConfig> configs) {
		ATNConfigSet cs = new ATNConfigSet(false);
		for (ATNConfig c : configs) cs.add(c);
		return cs;
	}

	/**
	 * Canonicalize a configuration set the way the runtime does: merge the
	 * contexts of configs that differ only in context (via ATNConfigSet), so
	 * DFA state identity matches the runtime's and recursive closures converge.
	 */
	protected static Set<ATNConfig> canonical(Set<ATNConfig> configs) {
		return new LinkedHashSet<ATNConfig>(toConfigSet(configs).configs);
	}

	protected static boolean isCyclic(List<List<Integer>> edges) {
		int n = edges.size();
		int[] color = new int[n]; // 0=white 1=gray 2=black
		Deque<int[]> stack = new ArrayDeque<int[]>(); // {state, next edge index}
		for (int root = 0; root < n; root++) {
			if (color[root] != 0) continue;
			stack.push(new int[]{root, 0});
			color[root] = 1;
			while (!stack.isEmpty()) {
				int[] frame = stack.peek();
				List<Integer> out = edges.get(frame[0]);
				if (frame[1] < out.size()) {
					int t = out.get(frame[1]++);
					if (color[t] == 1) return true;
					if (color[t] == 0) {
						color[t] = 1;
						stack.push(new int[]{t, 0});
					}
				}
				else {
					color[frame[0]] = 2;
					stack.pop();
				}
			}
		}
		return false;
	}

	/** Longest path (#edges) from state 0 in an acyclic DFA = max lookahead depth. */
	protected static int longestPath(List<List<Integer>> edges) {
		int[] memo = new int[edges.size()];
		java.util.Arrays.fill(memo, -1);
		return longestPathFrom(0, edges, memo);
	}

	private static int longestPathFrom(int s, List<List<Integer>> edges, int[] memo) {
		if (memo[s] >= 0) return memo[s];
		memo[s] = 0; // also guards (unreachable in acyclic graphs)
		int max = 0;
		for (int t : edges.get(s)) {
			max = Math.max(max, 1 + longestPathFrom(t, edges, memo));
		}
		memo[s] = max;
		return max;
	}

	// ---------------------------------------------------------------------
	// Report
	// ---------------------------------------------------------------------

	/** Build the human-readable report printed by the -Xdecision-report tool option. */
	public static String report(Grammar g) {
		DecisionClassifier classifier = new DecisionClassifier(g);
		List<Result> results = classifier.classifyAll();

		Map<Category, Integer> counts = new LinkedHashMap<Category, Integer>();
		for (Category c : Category.values()) counts.put(c, 0);
		for (Result r : results) counts.put(r.category, counts.get(r.category)+1);

		StringBuilder buf = new StringBuilder();
		buf.append("=== decision report: grammar ").append(g.name)
		   .append(" (").append(results.size()).append(" decisions) ===\n");
		buf.append("summary:");
		for (Map.Entry<Category, Integer> e : counts.entrySet()) {
			if (e.getValue() > 0) buf.append(' ').append(e.getKey()).append('=').append(e.getValue());
		}
		buf.append('\n');

		int maxKAnyAcyclic = 0;
		int maxKLLK = 0;
		Map<Integer, Integer> llkHistogram = new java.util.TreeMap<Integer, Integer>();
		for (Result r : results) {
			if (r.category == Category.LL1 || r.category == Category.LR_PRECEDENCE) continue;
			if (r.k > maxKAnyAcyclic) maxKAnyAcyclic = r.k;
			if (r.category == Category.LLK) {
				maxKLLK = Math.max(maxKLLK, r.k);
				llkHistogram.merge(r.k, 1, Integer::sum);
			}
			Rule rule = g.getRule(r.decisionState.ruleIndex);
			buf.append(String.format("%-22s d=%-4d %-17s", rule.name, r.decisionState.decision, r.category));
			if (r.k >= 0) buf.append(" k=").append(r.k);
			buf.append(" dfaStates=").append(r.numDfaStates);
			if (!r.exactAmbigConflicts.isEmpty()) {
				buf.append(" exactAmbigAlts=").append(distinct(r.exactAmbigConflicts));
			}
			if (!r.contextSensitiveConflicts.isEmpty()) {
				buf.append(" ctxSensitiveAlts=").append(distinct(r.contextSensitiveConflicts));
			}
			if (!r.approxConflicts.isEmpty()) {
				buf.append(" approxConflictAlts=").append(distinct(r.approxConflicts));
			}
			if (r.usedWidening) {
				buf.append(r.wideningDepth == Integer.MAX_VALUE
					? " [widened]" : " [widened depth="+r.wideningDepth+"]");
			}
			if (r.sawPrecPredicate) buf.append(" [precpred-in-lookahead]");
			buf.append('\n');
		}
		buf.append("LLK k histogram: ").append(llkHistogram).append('\n');
		buf.append("max k: LLK=").append(maxKLLK)
		   .append(", any acyclic non-LL(1) (incl. conflicted)=").append(maxKAnyAcyclic).append('\n');
		return buf.toString();
	}

	private static Set<BitSet> distinct(List<BitSet> sets) {
		return new LinkedHashSet<BitSet>(sets);
	}
}
