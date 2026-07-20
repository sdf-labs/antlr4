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
import org.antlr.v4.runtime.atn.PlusBlockStartState;
import org.antlr.v4.runtime.atn.PrecedencePredicateTransition;
import org.antlr.v4.runtime.atn.PredicateTransition;
import org.antlr.v4.runtime.atn.StarBlockStartState;
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
		OVERFLOW,
		/**
		 * A partial (depth- and size-bounded) static DFA with escape states:
		 * lookahead prefixes the table fully proves are predicted
		 * statically; everything else - untrusted conflicts, and the
		 * frontier beyond the depth/size budget - reaches an escape state,
		 * where the walker defers to {@code adaptivePredict} (which rescans
		 * from the decision start; the table never consumes input).
		 * Behavior-preserving by construction: the table only
		 * short-circuits what it proved, the reference engine handles the
		 * rest. This serves the hot paths of decisions whose complete SLL
		 * DFA is impractically large (unbounded phantom-FOLLOW scans) or
		 * contains genuinely context-sensitive corners.
		 */
		HYBRID
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
		/**
		 * The serialized prediction table, populated only when the decision
		 * is statically decidable with behavior preservation (category LLK,
		 * LLSTAR, or trusted EXACT_AMBIG).
		 */
		public StaticDFA dfa;
		/**
		 * For {@link Category#LR_PRECEDENCE} decisions: the per-precedence
		 * table group, populated only when every precedence class is
		 * statically decidable (see {@link PrecedenceStaticDFA}).
		 */
		public PrecedenceStaticDFA precDfa;
		/** For LR_PRECEDENCE decisions: report detail (per-class outcomes). */
		public String precNote;
		/** For HYBRID tables: number of escape states. */
		public int escapes;
		/** For LR_PRECEDENCE decisions: precedence classes without a table
		 *  (their dispatch entry defers to adaptivePredict). */
		public int adaptiveClasses;
		/**
		 * For HYBRID tables: fraction of the start state's outgoing token
		 * space whose successor subgraph contains at least one accept
		 * state - i.e., the share of first tokens for which the table can
		 * complete some predictions (typically the short-lookahead
		 * majority; longer scans down the same edges escape). A static
		 * proxy for hot-path coverage: edges that lead exclusively to
		 * escapes contribute nothing and a table whose start state resolves
		 * nothing is pure overhead.
		 */
		public double coverage;
		/**
		 * The decision's prefix-factor plan (see
		 * {@link PrefixFactorAnalyzer}), populated when the decision has
		 * factorable groups. Dry-run for the alt-mask mechanism: no
		 * effect on emitted tables.
		 */
		public PrefixFactorAnalyzer.Plan factorPlan;
		/** Dry-run: DFA states whose live-alt set is covered by one
		 *  factor group (would accept with an alternative mask). */
		public int factorMaskStates;
		/** Dry-run: escape states that a mask accept would cure. */
		public int factorEscapesCured;
		/** Dry-run: conflict states where the optional-postfix take rule
		 *  fires (mirror + vetoes pass): would accept take (alt 1). */
		public int takeRuleFires;
		/** Dry-run: the decision is an optional-postfix (X Y?) shape. */
		public boolean hasPostfixShape;

		public Result(DecisionState decisionState) {
			this.decisionState = decisionState;
		}
	}

	/**
	 * Taint bits stored in {@link ATNConfig#reachesIntoOuterContext} marking
	 * how configurations were derived. The field is chosen because every
	 * ATNConfig copy constructor propagates it, ATNConfigSet's merge takes
	 * the max of it, and equals/hashCode ignore it - so the taint flows to
	 * closure descendants and survives merging without affecting DFA state
	 * identity. (This classifier never uses the field's runtime meaning.)
	 *
	 * <p>Because merging takes the numeric max rather than the bitwise or,
	 * the "hard" bits ({@link #PRECPRED_TAINT}, {@link #WIDENED_TAINT}) are
	 * assigned above {@link #BOUNDARY_TAINT}: a merge can only lose the
	 * boundary bit in favor of a hard bit, and hard-tainted conflicts are
	 * untrusted regardless. When no hard taint is present in a
	 * configuration set, the boundary bits are exact.</p>
	 *
	 * <p>Over-approximations (widening, assumed-true precedence predicates)
	 * are one-sided: they can only keep alternatives alive longer than the
	 * runtime would, so uniquely-predicted accepts remain correct, but any
	 * conflict involving such configurations may be an analysis artifact the
	 * runtime never encounters and must not be trusted as an exact
	 * ambiguity.</p>
	 */
	/**
	 * Taint for configurations derived by popping through the decision-entry
	 * wildcard context (the SLL boundary): their continuations come from the
	 * grammar-wide FOLLOW links rather than the real parse stack. This
	 * mirrors the runtime's own {@code reachesIntoOuterContext} bookkeeping.
	 *
	 * <p>An exact conflict is context-independent - and therefore a true
	 * ambiguity resolvable to the minimum alternative - only if boundary
	 * usage is <em>uniform</em>: either no conflicting configuration popped
	 * through the boundary (the conflict is internal to the decision's
	 * sub-language) or all of them did (the real outer context substitutes
	 * into every alternative identically). Mixed usage is the dangling-else
	 * shape: one alternative reaches the conflict only via phantom follow
	 * contexts, and full-context prediction may kill it.</p>
	 *
	 * <p>Boundary-tainted configurations also identify the alternatives
	 * that "finished the decision entry rule", used for the per-state
	 * error-avoidance fallback (see {@link StaticDFA#fallbacks}), mirroring
	 * ParserATNSimulator#getAltThatFinishedDecisionEntryRule.</p>
	 */
	protected static final int BOUNDARY_TAINT = 1;

	/**
	 * Taint for configurations that traversed a precedence predicate as
	 * epsilon (i.e., assumed {@code precpred(...)} true).
	 */
	protected static final int PRECPRED_TAINT = 2;

	/**
	 * Taint for configurations that, after popping through the decision
	 * boundary of the left-recursive rule under a per-precedence-class
	 * construction, consumed a token outside the rule's operator loop -
	 * i.e. not at a loop-block state and without a loop-block frame in the
	 * calling context (an operand descent). Such consumption is
	 * "foreign": the phantom outer context eating the tokens is not an
	 * enclosing invocation of the same loop, so the enter/exit
	 * substitution argument (see {@link #classifyPrecedence}) does not
	 * apply and the conflict must escape to the adaptive engine. The
	 * genuinely context-sensitive shapes - e.g. {@code BETWEEN e AND e}
	 * colliding with a boolean {@code AND} loop, where the phantom
	 * consumer is a primary alternative of the rule, not its loop - are
	 * exactly the foreign ones.
	 */
	protected static final int FOREIGN_CONSUME_TAINT = 8;

	/** Taint for configurations whose calling context was widened. */
	protected static final int WIDENED_TAINT = 4;

	/** Taint bits that make any conflict untrusted outright. */
	protected static final int HARD_TAINT = WIDENED_TAINT | PRECPRED_TAINT;

	/** Hard cap on DFA states per decision (per construction attempt). */
	public int maxDfaStates = 2000;

	/**
	 * Hybrid-table construction bounds (see {@link Category#HYBRID}): the
	 * lookahead depth beyond which states become escapes, and the state
	 * budget after which the remaining frontier becomes escapes. The work
	 * queue is FIFO, so construction is breadth-first by lookahead depth
	 * and the emitted prefix is exactly the shallowest - hottest - states.
	 */
	public int hybridDepthCap = Integer.parseInt(System.getProperty("antlr.dfa.hybridDepthCap", "4"));
	public int hybridStateCap = Integer.parseInt(System.getProperty("antlr.dfa.hybridStateCap", "64"));
	/**
	 * Minimum start-state coverage (see {@link Result#coverage}) for a
	 * hybrid table to be worth its size: below this, nearly every
	 * prediction would escape to adaptivePredict anyway and the table is
	 * pure overhead.
	 */
	public double hybridMinCoverage = 0.1;

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

	/**
	 * When set (table-building mode), abandon a decision as soon as an
	 * untrusted (tainted or non-exact) conflict is found: no widening level
	 * can make such a decision table-eligible, so there is no point burning
	 * the state/time budget on it. Leave false for classification reports,
	 * where complete conflict inventories are wanted.
	 */
	public boolean abortOnUntrustedConflict = false;

	protected final Grammar g;
	protected final ATN atn;
	protected final IntervalSet allTokens;

	/** Depth bound of the construction attempt in progress. */
	protected int currentDepthBound = Integer.MAX_VALUE;
	/** Per-attempt memo for {@link #contextDepth} (contexts share structure). */
	protected Map<PredictionContext, Integer> depthCache;

	// -- precedence-decision construction mode (see classifyPrecedence) --

	/**
	 * When true, the start-state closure in progress belongs to a precedence
	 * loop decision: frame-0 precedence predicates are <em>evaluated</em>
	 * against {@link #precEvalValue} (mirroring the runtime, which collects
	 * and evaluates precedence predicates only while computing a precedence
	 * DFA start state), and precedence-filter suppression marks are placed
	 * (mirroring {@code outermostPrecedenceReturn} detection in
	 * ParserATNSimulator#closure_).
	 */
	protected boolean precStartClosure;
	/** Representative precedence of the class under construction. */
	protected int precEvalValue;

	/**
	 * Inclusive entry-precedence bounds of the precedence class under
	 * construction (see {@link #classifyPrecedence}). A class's table is
	 * only ever consulted for invocations whose entry precedence lies in
	 * this interval, and the entry precedence is exactly the precedence
	 * argument of the invoking call site. The closure uses the bounds to
	 * restrict decision-boundary pops to the compatible call sites: the
	 * runtime pops with the real stack, which is always such a call site,
	 * so the restriction prunes only phantom paths (an over-approximation
	 * artifact), never a runtime-reachable derivation. The one exception -
	 * invoking the left-recursive rule itself as the parse entry, where the
	 * runtime chases every FOLLOW link from the empty stack - is handled by
	 * the generated call site, which defers entry-frame invocations to
	 * adaptivePredict (see the {@code isEntry} parameter of dfaPredict).
	 */
	protected int precClassLo, precClassHi;

	/**
	 * ATN states of the left-recursive rule's body under per-precedence-
	 * class construction (rule-local: base alternatives, operand descent
	 * follow states, and the operator loop; rule references contribute
	 * their follow states, not their bodies). Token consumption at these
	 * states - or in a rule invoked from them (a loop-block return state
	 * in the calling context) - is "loop consumption"; everything else a
	 * boundary-popped config consumes is foreign (see
	 * {@link #FOREIGN_CONSUME_TAINT}).
	 */
	protected java.util.BitSet precLoopStates;

	/**
	 * ATN states of the precedence rule with an outgoing token-matching
	 * transition - a pending token obligation.
	 */
	protected java.util.BitSet precTokenObligatedStates;

	/**
	 * ATN states of the precedence rule that can reach a token match by
	 * epsilon without passing through the loop decision: positions owing
	 * a token of the <i>current</i> alternative (the reverse-epsilon
	 * closure of {@link #precTokenObligatedStates}, blocked at the
	 * decision state). A phantom enclosing frame at such a position owes
	 * something the iterate reading does not - the list element of
	 * {@code expr (',' expr)* '>>' expr} returns into a pending
	 * {@code ','}/{@code '>>'} - so the enter/exit substitution veto
	 * stands. A frame whose only path to a token crosses the decision
	 * owes exactly the next operator - which the iterate reading owes
	 * too - and is benign.
	 */
	protected java.util.BitSet precPendingStates;

	/**
	 * True while building any precedence class in which the enter/exit
	 * substitution may fire. The substitution maps an exit derivation
	 * (an enclosing frame of the same loop consumes the operator) onto
	 * an iterate derivation of the same sentence. The mapping requires
	 * the operator's guard to pass at the inner precedence - which the
	 * conflict itself attests: iterate is viable on the shared lookahead
	 * prefix only when an enabled alternative consumes it. Precedence
	 * gaps between the outer frame and the inner class do not matter:
	 * the iterate lineage can always take the free loop-exit and rejoin
	 * exactly the caller continuation the exit lineage heads for, so
	 * exit never becomes uniquely viable in any class. The substitution
	 * is therefore no longer restricted to class 0; the load-bearing
	 * conditions are the per-conflict vetoes (foreign consumption,
	 * pending obligations, attested real iterate).
	 */
	protected boolean precClassSubstitutable;

	/**
	 * True when every guarded (precedence-bearing) alternative of the
	 * precedence rule is <i>simple</i>: at most one rule invocation and
	 * no internal iteration. The enter/exit substitution argument maps
	 * an enclosing frame's "operator consumed, operand in progress" onto
	 * the inner loop's iterate - valid only when the operand completes
	 * the alternative's recursive structure. A compound alternative like
	 * {@code expr (',' expr)* '>>' expr} breaks it: the iterate reading
	 * of {@code a,c>>x} completes the inner Send as a list element and
	 * then dies (the outer Send's '>>' is missing), while the exit
	 * reading completes the sentence - adaptive predicts exit, so the
	 * substitution must not fire. Detected empirically: accepting
	 * iterate there produced (expr a) , ((expr c) >> (expr x)) instead
	 * of (expr a) , (expr c) >> (expr x).
	 */
	protected boolean precSimpleLoop;
	/** Rule index of the precedence decision under construction; -1 = none. */
	protected int precRuleIndex = -1;

	/** Decision under construction (debug filtering for closure traces). */
	protected int currentDecisionNumber = -1;
	/** During precedence-mode construction: exact conflicts are not trusted. */
	protected boolean trustExactAmbig = true;
	/**
	 * When true, {@link #buildDFA} builds a bounded hybrid table: untrusted
	 * conflicts and the frontier beyond {@link #hybridDepthCap}/
	 * {@link #hybridStateCap} become escape states instead of failing the
	 * decision (see {@link Category#HYBRID}).
	 */
	protected boolean hybridMode;
	/**
	 * Lazily built: state number of a rule call's follow state - the state
	 * grammar-wide FOLLOW links from the callee's stop state point back to -
	 * mapped to that call's RuleTransition. Used to re-derive, on the
	 * tool-side ATN, what ATNDeserializer#markPrecedenceDecisions encodes as
	 * EpsilonTransition#outermostPrecedenceReturn in deserialized ATNs.
	 */
	protected Map<Integer, RuleTransition> callByFollowState;

	/** Analyzer for prefix-factorable decisions (alt-mask dry run). */
	protected final PrefixFactorAnalyzer factorAnalyzer;
	/** Analyzer for optional-postfix (X Y?) decisions (take-rule dry run). */
	protected final OptionalPostfixAnalyzer postfixAnalyzer;
	/** Factor plan of the decision under construction; null when the
	 *  decision has no groups or is a precedence loop decision. */
	protected PrefixFactorAnalyzer.Plan currentFactorPlan;
	/** Optional-postfix shape of the decision under construction; null
	 *  when the decision is not an optional-postfix (X Y?) shape. */
	protected OptionalPostfixAnalyzer.Shape currentPostfixShape;

	public DecisionClassifier(Grammar g) {
		this.g = g;
		this.atn = g.atn;
		this.allTokens = IntervalSet.of(Token.MIN_USER_TOKEN_TYPE, atn.maxTokenType);
		this.factorAnalyzer = new PrefixFactorAnalyzer(g, atn);
		this.postfixAnalyzer = new OptionalPostfixAnalyzer(atn);
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
		this.currentFactorPlan = null;
		this.currentPostfixShape = null;
		if (s.nonGreedy) {
			res.category = Category.NON_GREEDY;
			return res;
		}
		if (isPrecedenceLoopDecision(s)) {
			return classifyPrecedence(s);
		}
		if (g.decisionLOOK != null && s.decision < g.decisionLOOK.size()
			&& AnalysisPipeline.disjoint(g.decisionLOOK.get(s.decision))) {
			res.category = Category.LL1;
			res.k = 1;
			return res;
		}

		// alt-mask dry run: compute the prefix-factor plan up front; the
		// construction attempts count the states it would resolve.
		this.currentFactorPlan = factorAnalyzer.analyze(s);
		// optional-postfix take-rule dry run
		this.currentPostfixShape = postfixAnalyzer.analyze(s);
		if (currentFactorPlan.hasGroups()
			&& "factor".equals(System.getProperty("antlr.dfa.debug"))) {
			for (PrefixFactorAnalyzer.Group grp : currentFactorPlan.groups) {
				System.err.printf("FACTOR d=%d %s%n", s.decision, grp.toString(g));
			}
		}

		// iterative deepening: exact first, then progressively wider approximation
		Result outcome = res;
		boolean allOverflowed = true;
		for (int depthBound : wideningDepths) {
			Result attempt = new Result(s);
			attempt.wideningDepth = depthBound;
			if (!buildDFA(s, attempt, depthBound)) {
				outcome = attempt;
				allOverflowed = false;
				break;
			}
			outcome = attempt;
		}
		if (allOverflowed) outcome.category = Category.OVERFLOW;

		// full construction failed: try a bounded hybrid table (exact
		// contexts; the depth cap keeps it small) whose escape states defer
		// untrusted conflicts and the deep frontier to adaptivePredict
		if (outcome.dfa == null
			&& (outcome.category == Category.OVERFLOW
				|| outcome.category == Category.CONTEXT_SENSITIVE)) {
			Result hybrid = classifyHybrid(s);
			if (hybrid != null && hybrid.dfa != null) {
				if (currentFactorPlan.hasGroups()) hybrid.factorPlan = currentFactorPlan;
				hybrid.hasPostfixShape = currentPostfixShape != null;
				return hybrid;
			}
		}
		if (currentFactorPlan.hasGroups()) outcome.factorPlan = currentFactorPlan;
		outcome.hasPostfixShape = currentPostfixShape != null;
		return outcome;
	}

	/** One hybrid construction attempt; null if it overflowed (can't happen
	 *  in practice: the depth/state caps bound it) or found predicates. */
	protected Result classifyHybrid(DecisionState s) {
		Result hybrid = new Result(s);
		hybrid.wideningDepth = Integer.MAX_VALUE;
		this.hybridMode = true;
		try {
			if (buildDFA(s, hybrid, Integer.MAX_VALUE)) return null;
		}
		finally {
			this.hybridMode = false;
		}
		return hybrid;
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
	// Precedence loop decisions (left-recursive rules)
	// ---------------------------------------------------------------------

	/**
	 * Classify the precedence loop decision of a left-recursive rule and, if
	 * possible, build its per-precedence static prediction tables.
	 *
	 * <p>The runtime predicts this decision with a <em>precedence DFA</em>:
	 * a separate lazily-built DFA start state per value of
	 * {@code parser.getPrecedence()} (see {@code DFA.isPrecedenceDfa} and
	 * {@code ParserATNSimulator.applyPrecedenceFilter}). The precedence only
	 * influences prediction through the {@code precpred(_ctx, n)} guards of
	 * the operator alternatives, all evaluated during start-state
	 * computation, so precedence values with identical guard outcomes are
	 * behaviorally indistinguishable: the guard constants {@code n1<...<nk}
	 * partition all precedence values into at most {@code k+1} classes.
	 * This method replays the runtime's start-state computation once per
	 * class (evaluating frame-0 precedence predicates against a class
	 * representative and applying the precedence filter) and runs the
	 * ordinary exhaustive DFA construction from there.</p>
	 *
	 * <p>Tables are emitted only if <em>every</em> class is statically
	 * decidable (the runtime dispatch must be total). Exact ambiguities are
	 * not trusted inside precedence classes: the min-alt substitution
	 * argument has not been re-established under precedence filtering, so
	 * such classes conservatively demote the whole decision.</p>
	 */
	protected Result classifyPrecedence(DecisionState s) {
		Result res = new Result(s);
		res.category = Category.LR_PRECEDENCE;

		// distinct guard constants of this rule's operator alternatives
		TreeSet<Integer> cutSet = new TreeSet<Integer>();
		for (ATNState st : atn.states) {
			if (st == null || st.ruleIndex != s.ruleIndex) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t instanceof PrecedencePredicateTransition) {
					cutSet.add(((PrecedencePredicateTransition)t).precedence);
				}
			}
		}
		if (cutSet.isEmpty()) return res; // not the expected LR shape; stay adaptive

		int[] cutoffs = new int[cutSet.size()];
		int[] reps = new int[cutSet.size()+1];
		int ci = 0;
		reps[0] = 0;
		for (int cut : cutSet) {
			cutoffs[ci] = cut;
			reps[ci+1] = cut+1; // first precedence above this cutoff
			ci++;
		}

		callByFollowState();

		// The operator loop block: every rule-local position of the
		// transformed left-recursive rule - base alternatives, the operand
		// descent follow states, and the operator loop. Rule references
		// contribute their follow states (operand bodies are foreign rules
		// reached with a loop-block frame in the context instead). The
		// block is entered from the rule start: rooting the BFS at the
		// loop's iterate branch alone missed the base-operand descent, so
		// consumption inside a nested invocation's operand (the follow
		// state of the base operand's rule reference) was misclassified
		// foreign, vetoing the enter/exit substitution on shapes like
		// `1 + 1`.
		this.precLoopStates = new java.util.BitSet();
		Deque<ATNState> loopWork = new ArrayDeque<ATNState>();
		loopWork.add(atn.ruleToStartState[s.ruleIndex]);
		while (!loopWork.isEmpty()) {
			ATNState st = loopWork.remove();
			if (st.ruleIndex != s.ruleIndex
				|| st instanceof RuleStopState
				|| precLoopStates.get(st.stateNumber)) {
				continue;
			}
			precLoopStates.set(st.stateNumber);
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				loopWork.add(t instanceof RuleTransition
					? ((RuleTransition)t).followState : t.target);
			}
		}

		// Token obligations and the pending-token states (see
		// precTokenObligatedStates / precPendingStates).
		this.precTokenObligatedStates = new java.util.BitSet();
		for (ATNState st : atn.states) {
			if (st == null || st.ruleIndex != s.ruleIndex) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				if (!st.transition(i).isEpsilon()) {
					precTokenObligatedStates.set(st.stateNumber);
					break;
				}
			}
		}
		// Pending-token states: positions that CANNOT reach the loop
		// decision by epsilon alone - every continuation owes a token of
		// the current alternative. The complement set (states that skip
		// to the decision freely) owes nothing: an optional trailing
		// token like the '(+)'? of Duckdb's valueExpression base
		// alternative is not an obligation, but Send's '>>' is - the
		// list-star exit of expr (',' expr)* '>>' expr cannot proceed
		// without it. A phantom frame at a pending position has no
		// iterate-side counterpart (the veto stands); a frame that can
		// skip owes exactly the next operator, which iterate owes too.
		List<List<ATNState>> reverseEps = new ArrayList<List<ATNState>>(atn.states.size());
		for (int i = 0; i < atn.states.size(); i++) reverseEps.add(new ArrayList<ATNState>());
		for (ATNState st : atn.states) {
			if (st == null || st.ruleIndex != s.ruleIndex) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (t.isEpsilon() && !(t instanceof RuleTransition)
					&& t.target.ruleIndex == s.ruleIndex) {
					reverseEps.get(t.target.stateNumber).add(st);
				}
			}
		}
		java.util.BitSet epsSkip = new java.util.BitSet();
		Deque<ATNState> skipWork = new ArrayDeque<ATNState>();
		epsSkip.set(s.stateNumber);
		skipWork.add(s);
		while (!skipWork.isEmpty()) {
			ATNState cur = skipWork.remove();
			for (ATNState pred : reverseEps.get(cur.stateNumber)) {
				if (!epsSkip.get(pred.stateNumber)) {
					epsSkip.set(pred.stateNumber);
					skipWork.add(pred);
				}
			}
		}
		this.precPendingStates = new java.util.BitSet();
		for (ATNState st : atn.states) {
			if (st != null && st.ruleIndex == s.ruleIndex
				&& !(st instanceof RuleStopState)
				&& !epsSkip.get(st.stateNumber)) {
				precPendingStates.set(st.stateNumber);
			}
		}

		// Simplicity of the guarded alternatives (see precSimpleLoop):
		// walk each precedence-guarded alternative's continuation.
		this.precSimpleLoop = true;
		for (ATNState st : atn.states) {
			if (st == null || st.ruleIndex != s.ruleIndex) continue;
			for (int i = 0; i < st.getNumberOfTransitions(); i++) {
				Transition t = st.transition(i);
				if (!(t instanceof PrecedencePredicateTransition)) continue;
				int invocations = 0;
				Set<ATNState> visited = new HashSet<ATNState>();
				Deque<ATNState> altWork = new ArrayDeque<ATNState>();
				altWork.add(t.target);
				while (!altWork.isEmpty() && precSimpleLoop) {
					ATNState x = altWork.remove();
					if (x == s || x.ruleIndex != s.ruleIndex || !visited.add(x)) continue;
					if (x instanceof StarBlockStartState || x instanceof PlusBlockStartState) {
						precSimpleLoop = false;
						break;
					}
					for (int j = 0; j < x.getNumberOfTransitions(); j++) {
						Transition t2 = x.transition(j);
						if (t2 instanceof RuleTransition) {
							if (++invocations > 1) precSimpleLoop = false;
							altWork.add(((RuleTransition)t2).followState);
						}
						else {
							altWork.add(t2.target);
						}
					}
				}
				if (!precSimpleLoop) break;
			}
		}

		StaticDFA[] tables = new StaticDFA[reps.length];
		if (System.getProperty("antlr.dfa.debug.loopstates") != null) {
			System.err.printf("LOOP-STATES d=%d rule=%s count=%d:%n",
				s.decision, g.getRule(s.ruleIndex).name, precLoopStates.cardinality());
			for (int i = precLoopStates.nextSetBit(0); i >= 0; i = precLoopStates.nextSetBit(i+1)) {
				System.err.printf("    %d%n", i);
			}
		}
		StringBuilder note = new StringBuilder();
		boolean anyTable = false;
		int maxK = 0;
		int totalStates = 0;
		this.precRuleIndex = s.ruleIndex;
		this.trustExactAmbig = false;
		try {
			for (int c = 0; c < reps.length; c++) {
				this.precEvalValue = reps[c];
				this.precClassLo = reps[c];
				this.precClassHi = c < cutoffs.length ? cutoffs[c] : Integer.MAX_VALUE;
				this.precClassSubstitutable = true;
				Result attempt = null;
				for (int depthBound : wideningDepths) {
					Result a = new Result(s);
					a.wideningDepth = depthBound;
					if (!buildDFA(s, a, depthBound)) { attempt = a; break; }
					attempt = a; // overflow at this bound; try tighter widening
				}
				if (attempt.dfa == null && !attempt.sawPredicate) {
					// full construction failed for this class: bounded
					// hybrid table with adaptive escapes
					Result h = new Result(s);
					h.wideningDepth = Integer.MAX_VALUE;
					this.hybridMode = true;
					try {
						if (!buildDFA(s, h, Integer.MAX_VALUE)) attempt = h;
					}
					finally {
						this.hybridMode = false;
					}
				}
				res.sawPrecPredicate |= attempt.sawPrecPredicate;
				res.usedWidening |= attempt.usedWidening;
				if (note.length() > 0) note.append(' ');
				note.append("p").append(c == 0 ? "<=" + cutoffs[0]
					: c < cutoffs.length ? "=" + (cutoffs[c-1]+1) + ".." + cutoffs[c]
					: ">" + cutoffs[cutoffs.length-1]).append(':');
				if (attempt.dfa == null) {
					res.adaptiveClasses++;
					if (attempt.category == Category.HYBRID) {
						// hybrid construction worked but fell below the
						// coverage floor; the class stays adaptive
						note.append(String.format("adaptive(cover=%.0f%%)", attempt.coverage*100));
					}
					else note.append(attempt.category);
					if (!attempt.contextSensitiveConflicts.isEmpty()) {
						note.append("/ctx").append(distinct(attempt.contextSensitiveConflicts));
					}
					if (!attempt.approxConflicts.isEmpty()) {
						note.append("/approx").append(distinct(attempt.approxConflicts));
					}
					if (!attempt.exactAmbigConflicts.isEmpty()) {
						note.append("/exact").append(distinct(attempt.exactAmbigConflicts));
					}
				}
				else {
					anyTable = true;
					tables[c] = attempt.dfa;
					maxK = Math.max(maxK, attempt.k);
					totalStates += attempt.numDfaStates;
					if (attempt.category == Category.HYBRID) {
						note.append(String.format("hyb(cover=%.0f%%)", attempt.coverage*100));
						res.escapes += attempt.escapes;
					}
					else {
						note.append(attempt.dfa.cyclic ? "LL(*)" : "k=" + attempt.k);
					}
				}
			}
		}
		finally {
			this.precRuleIndex = -1;
			this.trustExactAmbig = true;
			this.precStartClosure = false;
		}

		res.precNote = "classes=" + reps.length + " [" + note + "]";
		if (anyTable) {
			// classes without a table dispatch to adaptivePredict (their
			// dispatch entry is -1); the group is worthwhile as long as any
			// class predicts statically
			res.precDfa = new PrecedenceStaticDFA(s.decision, cutoffs, tables);
			res.k = maxK;
			res.numDfaStates = totalStates;
		}
		return res;
	}

	/** Lazily built {@link #callByFollowState}. */
	protected Map<Integer, RuleTransition> callByFollowState() {
		if (callByFollowState == null) {
			callByFollowState = new HashMap<Integer, RuleTransition>();
			for (ATNState st : atn.states) {
				if (st == null) continue;
				for (int i = 0; i < st.getNumberOfTransitions(); i++) {
					Transition t = st.transition(i);
					if (t instanceof RuleTransition) {
						RuleTransition rt = (RuleTransition)t;
						callByFollowState.put(rt.followState.stateNumber, rt);
					}
				}
			}
		}
		return callByFollowState;
	}

	/**
	 * Evaluate a precedence guard {@code precpred(n)} of rule {@code rule}
	 * against the precedence the rule was entered with in this closure
	 * path, when that value is statically known - and it usually is: the
	 * top return state of the configuration's context identifies the call
	 * site (it is the follow state of the {@link RuleTransition} that
	 * entered the rule), and the transition carries the call's constant
	 * precedence argument. Depth-bounded widening preserves the immediate
	 * return frame, so the evaluation stays available at every widening
	 * level except the fully context-free approximation.
	 *
	 * <p>Returns null when the entry precedence is unknown or ambiguous:
	 * empty/wildcard context (the decision's own frame, or a phantom frame
	 * entered by popping through the decision boundary), a wildcard among
	 * the tops of a merged context, or merged tops with disagreeing guard
	 * outcomes. Callers must then fall back to assumed-true traversal with
	 * {@link #PRECPRED_TAINT} - the evaluation only ever prunes paths whose
	 * guard provably fails.</p>
	 *
	 * <p>Why pruning is behavior-preserving:</p>
	 * <ul>
	 * <li>Against the parse: a lookahead path through a false precedence
	 * guard is unparseable - the generated code checks the guard and fails.
	 * Pruning removes no real parse.</li>
	 * <li>Against {@code adaptivePredict}, which traverses these guards as
	 * pure epsilon (it evaluates precedence predicates only in frame 0 of a
	 * precedence DFA start state) and therefore keeps the pruned paths: the
	 * left-recursion transform's guards only restrict <em>which
	 * derivation</em> produces a token string, never the strings
	 * themselves. A phantom "consume this operator in a deeper frame" path
	 * is duplicated, token for token and with the same alternative at the
	 * decision under construction, by the legal path that exits the deeper
	 * frames (loop exits are unguarded) and consumes the operator at an
	 * outer loop level whose weaker guard admits it. So the pruned closure
	 * computes the same prefix-to-viable-alternatives map, just without
	 * enumerating the redundant derivations that blow up the configuration
	 * space in precedence ladders.</li>
	 * </ul>
	 */
	protected Boolean evalPrecpredFromContext(PredictionContext ctx, int rule, int n) {
		if (ctx == null || ctx.isEmpty()) return null;
		Boolean agreed = null;
		for (int i = 0; i < ctx.size(); i++) {
			int returnState = ctx.getReturnState(i);
			if (returnState == PredictionContext.EMPTY_RETURN_STATE) return null;
			RuleTransition call = callByFollowState().get(returnState);
			if (call == null || call.target.ruleIndex != rule) return null;
			boolean outcome = n >= call.precedence;
			if (agreed == null) agreed = outcome;
			else if (agreed != outcome) return null;
		}
		return agreed;
	}

	/**
	 * Tool-side port of ParserATNSimulator#applyPrecedenceFilter, applied to
	 * the merged start configuration set of a precedence class before it
	 * becomes DFA state 0. Pass 1 (evaluating the operator guards on the
	 * loop-entry alternative) already happened eagerly during closure - see
	 * the PrecedencePredicateTransition case - so only pass 2 remains:
	 * eliminate loop-exit configurations shadowed by a loop-entry
	 * configuration at the same ATN state with an equal context (the
	 * "let the current invocation take the operator" resolution that makes
	 * precedence loops unambiguous), keeping configurations whose
	 * suppression mark identifies them as re-entries through 0-precedence
	 * call sites (a real outer invocation may legitimately take the
	 * operator, cf. {@code e '[' e ']'}).
	 *
	 * <p>Because the suppression mark lives above the taint bits in
	 * {@link ATNConfig#reachesIntoOuterContext} and merging takes the max,
	 * a merge with a suppressed configuration can swallow taint bits; they
	 * are conservatively restored from the pre-merge configurations, keyed
	 * by (state, alt). Suppression marks are cleared from the survivors so
	 * they cannot perturb DFA state identity downstream.</p>
	 */
	protected Set<ATNConfig> applyPrecedenceFilter(Set<ATNConfig> raw) {
		// conservative taint restore: OR of pre-merge taints per (state, alt)
		Map<Long, Integer> taintByStateAlt = new HashMap<Long, Integer>();
		for (ATNConfig c : raw) {
			long key = ((long)c.state.stateNumber << 32) | c.alt;
			Integer prev = taintByStateAlt.get(key);
			int taint = c.reachesIntoOuterContext & (BOUNDARY_TAINT|PRECPRED_TAINT|WIDENED_TAINT);
			taintByStateAlt.put(key, prev == null ? taint : (prev|taint));
		}

		Set<ATNConfig> merged = canonical(raw);

		Map<Integer, PredictionContext> statesFromAlt1 = new HashMap<Integer, PredictionContext>();
		for (ATNConfig c : merged) {
			if (c.alt == 1) statesFromAlt1.put(c.state.stateNumber, c.context);
		}

		Set<ATNConfig> result = new LinkedHashSet<ATNConfig>();
		for (ATNConfig c : merged) {
			if (c.alt != 1 && !c.isPrecedenceFilterSuppressed()) {
				PredictionContext ctx1 = statesFromAlt1.get(c.state.stateNumber);
				if (ctx1 != null && ctx1.equals(c.context)) {
					continue; // eliminated: shadowed by the loop-entry alternative
				}
			}
			ATNConfig kept = new ATNConfig(c);
			kept.setPrecedenceFilterSuppressed(false);
			long key = ((long)c.state.stateNumber << 32) | c.alt;
			Integer taint = taintByStateAlt.get(key);
			if (taint != null) kept.reachesIntoOuterContext |= taint;
			result.add(kept);
		}
		return result;
	}

	// ---------------------------------------------------------------------
	// Static SLL DFA construction
	// ---------------------------------------------------------------------

	/** One construction attempt at the given context-depth bound; true = overflow. */
	protected boolean buildDFA(DecisionState s, Result res, int depthBound) {
		this.currentDepthBound = depthBound;
		this.currentDecisionNumber = s.decision;
		this.depthCache = new java.util.IdentityHashMap<PredictionContext, Integer>();
		Map<Set<ATNConfig>, Integer> stateIds = new HashMap<Set<ATNConfig>, Integer>();
		List<Set<ATNConfig>> states = new ArrayList<Set<ATNConfig>>();
		List<List<Integer>> edges = new ArrayList<List<Integer>>();
		List<List<IntervalSet>> edgeLabels = new ArrayList<List<IntervalSet>>();
		List<Integer> acceptAlts = new ArrayList<Integer>(); // 0 = not an accept state
		List<Long> acceptMasks = new ArrayList<Long>(); // 0 = not a mask-accept state
		List<Integer> fallbackAlts = new ArrayList<Integer>(); // 0 = none
		List<Integer> stateDepth = new ArrayList<Integer>(); // lookahead depth (BFS layer)

		Set<ATNConfig> start = new LinkedHashSet<ATNConfig>();
		Set<Object> startBusy = new HashSet<Object>();
		this.precStartClosure = precRuleIndex >= 0;
		for (int i = 0; i < s.getNumberOfTransitions(); i++) {
			closure(new ATNConfig(s.transition(i).target, i+1, EmptyPredictionContext.Instance),
					start, startBusy, res, 0);
		}
		this.precStartClosure = false;
		if (precRuleIndex >= 0) {
			start = applyPrecedenceFilter(start);
		}
		start = canonical(start);
		if (precRuleIndex >= 0 && "full".equals(System.getProperty("antlr.dfa.debug"))) {
			System.err.printf("PREC-START d=%d rep=%d [%d,%d] configs:%n",
				s.decision, precEvalValue, precClassLo, precClassHi);
			for (ATNConfig c : start) {
				System.err.printf("    alt=%d rule=%s state=%d taint=%d supp=%s ctx=%s%n",
					c.alt, g.getRule(c.state.ruleIndex).name, c.state.stateNumber,
					c.reachesIntoOuterContext, c.isPrecedenceFilterSuppressed(), c.context);
			}
		}
		stateIds.put(start, 0);
		states.add(start);
		edges.add(new ArrayList<Integer>());
		edgeLabels.add(new ArrayList<IntervalSet>());
		acceptAlts.add(0);
		acceptMasks.add(0L);
		fallbackAlts.add(0);
		stateDepth.add(0);

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
				// accept (or, for cardinality 0, dead/error) state
				acceptAlts.set(d, Math.max(0, alts.nextSetBit(0)));
				continue;
			}

			// alt-mask acceptance: a state whose live alternatives fall
			// inside a single prefix-factor group resolves to an
			// alternative mask instead of a unique alt or an escape; the
			// generated parser executes the group's shared prefix and
			// defers the choice to its tail decision
			PrefixFactorAnalyzer.Group factorGroup = currentFactorPlan != null
				? currentFactorPlan.groupCovering(alts) : null;
			if (factorGroup != null) res.factorMaskStates++;
			if (PredictionMode.hasSLLConflictTerminatingPrediction(PredictionMode.SLL, cs)) {
				Collection<BitSet> altSubsets = PredictionMode.getConflictingAltSubsets(cs);
				boolean exact = PredictionMode.allSubsetsConflict(altSubsets)
					&& PredictionMode.allSubsetsEqual(altSubsets);
				boolean widenedTainted = false;
				boolean anyBoundary = false;
				boolean allBoundary = true;
				for (ATNConfig c : cs) {
					if ((c.reachesIntoOuterContext & WIDENED_TAINT) != 0) widenedTainted = true;
					if ((c.reachesIntoOuterContext & BOUNDARY_TAINT) != 0) anyBoundary = true;
					else allBoundary = false;
				}
				BitSet conflicting = PredictionMode.getAlts(altSubsets);

				// Enter/exit ambiguity of a precedence loop, resolved to
				// iterate by substitution: when the exit alternative's
				// scanned tokens were consumed exclusively by (a phantom or
				// real) enclosing invocation of this same operator loop - no
				// FOREIGN_CONSUME taint - every exit derivation of a sentence
				// maps to an iterate derivation of the same sentence. The
				// mapping is precedence-safe: the outer reading consumes the
				// operator through the same loop alternative (same guard,
				// same operand precedence), and the iterate lineage can
				// always take the loop-exit epsilon and rejoin exactly the
				// caller continuation the exit lineage heads for - so
				// whenever exit is viable, iterate is viable too. Exit
				// therefore never becomes uniquely viable, and whenever the
				// adaptive engine terminates - unique iterate, or an
				// ambiguity resolved to the minimum alternative - it answers
				// iterate. Spurious (widening-born, or phantom-only) exit
				// viability only adds the conflict; the resolution matches
				// the runtime either way. This holds in every precedence
				// class, not just class 0: within a class, iterate viability
				// already attests that the operator's guard passes at the
				// current precedence. Iterate viability itself must be
				// attested by a non-widened, non-foreign config so the
				// accept is never based on an analysis artifact alone.
				// Exactness of the conflict is NOT required: the conflicting
				// subsets diverge precisely because exit carries caller
				// continuations (and outer incarnations of this loop) that
				// iterate only rejoins after its free loop-exit - the
				// inexact shapes are the common case in wrapper-dense
				// grammars.
				if (precRuleIndex >= 0 && precClassSubstitutable && precSimpleLoop
					&& conflicting.cardinality() == 2
					&& conflicting.get(1) && conflicting.get(2)) {
					boolean exitForeign = false;
					boolean exitObligated = false;
					boolean iterateReal = false;
					for (ATNConfig c : cs) {
						if (c.alt == 2) {
							if ((c.reachesIntoOuterContext & FOREIGN_CONSUME_TAINT) != 0) {
								exitForeign = true;
							}
							// Every phantom same-rule frame of the exit
							// derivation must be free of token obligations
							// outside the iteration-start region. An
							// operand-start position owes the operand -
							// exactly what the iterate reading owes too
							// (both invoke the same operand next) - UNLESS
							// its invocation returns into pending tokens of
							// a compound alternative: the operand of a
							// simple binary/unary operator returns to the
							// alternative tail (nothing pending, benign),
							// but a list element of expr (',' expr)* '>>'
							// expr returns to a position owing ',' or '>>',
							// which has no iterate-side counterpart - there
							// the runtime can kill iterate with deeper
							// lookahead and legitimately predict exit.
							if (c.state.ruleIndex == precRuleIndex
								&& precPendingStates.get(c.state.stateNumber)
								&& hasPendingPrecReturn(c.context)) {
								exitObligated = true;
							}
							if (hasObligatedReturn(c.context,
									java.util.Collections.newSetFromMap(
										new java.util.IdentityHashMap<PredictionContext, Boolean>()))) {
								exitObligated = true;
							}
						}
						if (c.alt == 1 && (c.reachesIntoOuterContext
								& (WIDENED_TAINT|FOREIGN_CONSUME_TAINT)) == 0) {
							iterateReal = true;
						}
					}
					if (!exitForeign && !exitObligated && iterateReal) {
						res.exactAmbigConflicts.add(conflicting);
						acceptAlts.set(d, 1);
						continue;
					}
					if ("veto".equals(System.getProperty("antlr.dfa.debug"))) {
						System.err.printf("SUBST-VETO d=%d state=%d foreign=%s obligated=%s iterateReal=%s%n",
							s.decision, d, exitForeign, exitObligated, iterateReal);
						for (ATNConfig c : cs) {
							if (c.alt == 2 && c.state.ruleIndex == precRuleIndex) {
								ATNState st = c.state;
								StringBuilder tr = new StringBuilder();
								for (int i = 0; i < st.getNumberOfTransitions(); i++) {
									Transition t = st.transition(i);
									if (i > 0) tr.append(' ');
									tr.append(t.isEpsilon() ? "eps" : t.label() != null ? t.label().toString(g.getVocabulary()) : "?")
										.append("->").append(t.target.stateNumber);
								}
								System.err.printf("    alt=2 %s:%d taint=%d transitions: %s ctx=%s%n",
									g.getRule(st.ruleIndex).name, st.stateNumber,
									c.reachesIntoOuterContext, tr, decodeContext(c.context));
							}
						}
					}
				}

				// Optional-postfix take rule: the take/skip conflict of an
				// X Y? decision is a dangling-else shape. When every token
				// the skip reading consumes is mirrored into the take
				// structure itself - each descended skip configuration's
				// deepest context frame is a return into the decision's
				// block end (the phantom continuation funnels back through
				// this decision's own take path), nothing was consumed in
				// a foreign frame, and take viability is attested by a
				// non-widened, non-foreign config - then skip is never
				// uniquely viable: take's continuation after the optional
				// is identical to skip's (both reach the same block end),
				// and the mirrored re-descents consume the same tokens, so
				// whenever the adaptive engine terminates - unique take,
				// or an ambiguity resolved to the minimum alternative -
				// it answers take. The starvation check (in the analyzer)
				// excludes X Y? Z shapes where a mandatory Z can be served
				// by the skipped input (TRIM, namedParameter); the mirror
				// check excludes the alias family, whose clause
				// continuations (LIMIT, FROM, ...) let skip genuinely win.
				if (currentPostfixShape != null
					&& conflicting.cardinality() == 2
					&& conflicting.get(1) && conflicting.get(2)) {
					boolean skipForeign = false;
					boolean mirrorOk = true;
					boolean realTake = false;
					for (ATNConfig c : cs) {
						if (c.alt == 2) {
							if ((c.reachesIntoOuterContext & FOREIGN_CONSUME_TAINT) != 0) {
								skipForeign = true;
							}
							if (c.state.ruleIndex != s.ruleIndex
								&& !bottomFrameIs(c.context,
									currentPostfixShape.blockEndState,
									java.util.Collections.newSetFromMap(
										new java.util.IdentityHashMap<PredictionContext, Boolean>()))) {
								mirrorOk = false;
							}
						}
						if (c.alt == 1
							&& (c.reachesIntoOuterContext & (WIDENED_TAINT|FOREIGN_CONSUME_TAINT)) == 0) {
							realTake = true;
						}
					}
					if (mirrorOk && !skipForeign && realTake) {
						res.takeRuleFires++;
						res.exactAmbigConflicts.add(conflicting);
						acceptAlts.set(d, 1);
						if ("postfix".equals(System.getProperty("antlr.dfa.debug"))) {
							System.err.printf("TAKE-FIRE d=%d state=%d exact=%s%n", s.decision, d, exact);
							for (ATNConfig c : cs) {
								if (c.alt == 2) {
									System.err.printf("    skip %s:%d taint=%d ctx=%s%n",
										g.getRule(c.state.ruleIndex).name, c.state.stateNumber,
										c.reachesIntoOuterContext, decodeContext(c.context));
								}
							}
						}
						continue;
					}
					else if ("postfix".equals(System.getProperty("antlr.dfa.debug"))) {
						System.err.printf("TAKE-VETO d=%d state=%d exact=%s foreign=%s mirror=%s realTake=%s%n",
							s.decision, d, exact, skipForeign, mirrorOk, realTake);
					}
				}

				// A conflict is trusted as an exact ambiguity only if it is
				// exact, involves no widened (analysis-only over-approximated)
				// configs, and its boundary usage is uniform: either the
				// conflict is internal to the decision's sub-language, or
				// every config popped through the decision boundary so the
				// real outer context substitutes into all alternatives
				// identically. Mixed usage (dangling-else shape) means
				// full-context prediction may kill the boundary-only
				// alternative, so min-alt is not behavior-preserving.
				// Assumed-true precedence guards (PRECPRED_TAINT) do not
				// block trust: the runtime's own SLL closure traverses
				// non-frame-0 precedence predicates as epsilon under exactly
				// the same assumption, so such configs mirror the runtime
				// rather than over-approximate it. Inside precedence classes
				// exact conflicts are otherwise never trusted: the min-alt
				// substitution argument has not been re-established under
				// precedence filtering (the enter/exit rule above is the
				// one class-mode shape where it has).
				boolean untrusted = widenedTainted || (anyBoundary && !allBoundary)
					|| !trustExactAmbig;
				if (untrusted) res.approxConflicts.add(conflicting);
				else if (exact) res.exactAmbigConflicts.add(conflicting);
				else res.contextSensitiveConflicts.add(conflicting);
				if (!untrusted && exact) {
					// trusted exact ambiguity: min-alt resolution matches
					// both runtime SLL and full-context LL
					acceptAlts.set(d, conflicting.nextSetBit(0));
				}
				else if (hybridMode) {
					// the conflict cannot be resolved statically: defer
					// this state to adaptivePredict - UNLESS its live
					// alternatives are covered by one prefix-factor
					// group, in which case it accepts with the group's
					// alternative mask (the parser resolves the choice
					// at the group's tail decision)
					if (factorGroup != null) {
						res.factorEscapesCured++;
						acceptMasks.set(d, altBits(factorGroup.alts));
					}
					else {
						acceptAlts.set(d, StaticDFA.ESCAPE);
					}
					if (System.getProperty("antlr.dfa.debug") != null) {
						System.err.printf("ESCAPE-CONFLICT d=%d state=%d depth=%d exact=%s hard=%s anyB=%s allB=%s trustEA=%s alts=%s%n",
							s.decision, d, stateDepth.get(d), exact, widenedTainted,
							anyBoundary, allBoundary, trustExactAmbig, conflicting);
						if ("full".equals(System.getProperty("antlr.dfa.debug"))) {
							for (ATNConfig c : cs) {
								System.err.printf("    alt=%d rule=%s state=%d taint=%d ctx=%s%n",
									c.alt, g.getRule(c.state.ruleIndex).name,
									c.state.stateNumber, c.reachesIntoOuterContext,
									c.context);
							}
						}
					}
				}
				else if (abortOnUntrustedConflict) {
					// not a trusted exact ambiguity: no widening level can
					// make this decision table-eligible
					res.numDfaStates = states.size();
					res.category = Category.CONTEXT_SENSITIVE;
					return false;
				}
				continue;
			}

			if (hybridMode
				&& (stateDepth.get(d) >= hybridDepthCap
					|| (states.size() >= hybridStateCap && stateDepth.get(d) >= 2))) {
				// Beyond the depth/size budget: don't expand, defer to
				// adaptivePredict (BFS order makes this the deep, cold
				// frontier - the shallow hot states are already built).
				// Depth 0 and 1 are exempt from the state budget: a wide
				// first rank (bounded by the token alphabet) must not starve
				// its own siblings, or single-token predictions - the
				// dynamic majority - escape on sheer decision fanout.
				if (factorGroup != null) {
					res.factorEscapesCured++;
					acceptMasks.set(d, altBits(factorGroup.alts));
				}
				else {
					acceptAlts.set(d, StaticDFA.ESCAPE);
				}
				continue;
			}

			// Error-avoidance fallback for this (expanding) state: the
			// minimum alternative that already "finished the decision entry
			// rule" (popped through the decision boundary). When prediction
			// later dies at this state on a token matching no edge, the
			// walker returns this alternative instead of failing, letting
			// the parser report a more precise error at the mismatch point -
			// mirroring adaptivePredict's
			// getAltThatFinishedDecisionEntryRule recovery.
			int fallback = 0;
			for (ATNConfig c : cs) {
				if ((c.reachesIntoOuterContext & BOUNDARY_TAINT) != 0
					&& (fallback == 0 || c.alt < fallback)) {
					fallback = c.alt;
				}
			}
			fallbackAlts.set(d, fallback);

			for (LabeledSuccessor ls : successors(configs, res)) {
				Integer id = stateIds.get(ls.configs);
				if (id == null) {
					if (states.size() >= maxDfaStates) { overflow = true; break; }
					id = states.size();
					stateIds.put(ls.configs, id);
					states.add(ls.configs);
					edges.add(new ArrayList<Integer>());
					edgeLabels.add(new ArrayList<IntervalSet>());
					acceptAlts.add(0);
					acceptMasks.add(0L);
					fallbackAlts.add(0);
					stateDepth.add(stateDepth.get(d)+1);
					work.add(id);
				}
				edges.get(d).add(id);
				edgeLabels.get(d).add(ls.label);
			}
		}

		res.numDfaStates = states.size();
		if (overflow) {
			if (System.getProperty("antlr.dfa.debug") != null) {
				dumpOverflowStats(s, states, depthBound);
			}
			res.category = Category.OVERFLOW;
			return true;
		}
		if (res.sawPredicate) {
			// user predicates gate viability in ways the table cannot
			// represent; not eligible even for a hybrid table
			res.category = Category.PREDICATED;
			return false;
		}

		boolean cyclic = isCyclic(edges);
		if (!cyclic) res.k = longestPath(edges);

		if ("states".equals(System.getProperty("antlr.dfa.debug"))
			&& String.valueOf(currentDecisionNumber).equals(System.getProperty("antlr.dfa.debug.decision"))) {
			dumpStates(s, states, edges, edgeLabels, acceptAlts, stateDepth);
		}

		int escapes = 0;
		for (int a : acceptAlts) {
			if (a == StaticDFA.ESCAPE) escapes++;
		}
		if (escapes > 0) {
			// hybrid table: emit only if the start state itself resolves
			// something - a start-state escape means nothing is decidable
			res.category = Category.HYBRID;
			res.escapes = escapes;
			res.coverage = startCoverage(edges, edgeLabels, acceptAlts, acceptMasks);
			if (acceptAlts.get(0) != StaticDFA.ESCAPE && res.coverage >= hybridMinCoverage) {
				res.dfa = toStaticDFA(s.decision, edgeLabels, edges, acceptAlts, acceptMasks, fallbackAlts, cyclic, res.k);
			}
			return false;
		}

		if (!res.contextSensitiveConflicts.isEmpty() || !res.approxConflicts.isEmpty()) {
			res.category = Category.CONTEXT_SENSITIVE;
		}
		else if (!res.exactAmbigConflicts.isEmpty()) res.category = Category.EXACT_AMBIG;
		else if (cyclic) res.category = Category.LLSTAR;
		else res.category = Category.LLK;

		if (res.category == Category.LLK || res.category == Category.LLSTAR
			|| res.category == Category.EXACT_AMBIG) {
			res.dfa = toStaticDFA(s.decision, edgeLabels, edges, acceptAlts, acceptMasks, fallbackAlts, cyclic, res.k);
		}
		return false;
	}

	/**
	 * Hot-path coverage proxy of a hybrid table: the fraction of the start
	 * state's outgoing token space whose successor subgraph contains at
	 * least one accept state. An edge leading exclusively to escapes (a
	 * conflict right behind it at every depth) contributes nothing - the
	 * table can never resolve a prediction down that path; an edge whose
	 * subgraph mixes accepts and escapes resolves its short-lookahead
	 * prefixes (typically the dynamic majority) and escapes the rest.
	 */
	protected static double startCoverage(List<List<Integer>> edges,
										  List<List<IntervalSet>> edgeLabels,
										  List<Integer> acceptAlts,
										  List<Long> acceptMasks) {
		int n = edges.size();
		// reverse reachability from accept states
		List<List<Integer>> reverse = new ArrayList<List<Integer>>(n);
		for (int i = 0; i < n; i++) reverse.add(new ArrayList<Integer>());
		for (int sIdx = 0; sIdx < n; sIdx++) {
			for (int target : edges.get(sIdx)) reverse.get(target).add(sIdx);
		}
		boolean[] reachesAccept = new boolean[n];
		Deque<Integer> work = new ArrayDeque<Integer>();
		for (int i = 0; i < n; i++) {
			if (acceptAlts.get(i) > 0 || acceptMasks.get(i) != 0) {
				reachesAccept[i] = true;
				work.add(i);
			}
		}
		while (!work.isEmpty()) {
			for (int prev : reverse.get(work.remove())) {
				if (!reachesAccept[prev]) {
					reachesAccept[prev] = true;
					work.add(prev);
				}
			}
		}
		long total = 0, useful = 0;
		List<Integer> startEdges = edges.get(0);
		List<IntervalSet> startLabels = edgeLabels.get(0);
		for (int e = 0; e < startEdges.size(); e++) {
			int width = startLabels.get(e).size();
			total += width;
			if (reachesAccept[startEdges.get(e)]) useful += width;
		}
		return total == 0 ? 0 : (double)useful/total;
	}

	/** Diagnostics for overflowing decisions (-Dantlr.dfa.debug): where the
	 *  configuration population concentrates, and its taint composition. */
	private void dumpOverflowStats(DecisionState s, List<Set<ATNConfig>> states, int depthBound) {
		Map<String, Integer> ruleHisto = new HashMap<String, Integer>();
		int totalConfigs = 0, emptyCtx = 0, precTainted = 0, widenedTainted = 0, boundary = 0;
		int maxSetSize = 0;
		for (Set<ATNConfig> set : states) {
			maxSetSize = Math.max(maxSetSize, set.size());
			for (ATNConfig c : set) {
				totalConfigs++;
				String rn = g.getRule(c.state.ruleIndex).name;
				ruleHisto.merge(rn, 1, Integer::sum);
				if (c.context == null || c.context.isEmpty()) emptyCtx++;
				if ((c.reachesIntoOuterContext & PRECPRED_TAINT) != 0) precTainted++;
				if ((c.reachesIntoOuterContext & WIDENED_TAINT) != 0) widenedTainted++;
				if ((c.reachesIntoOuterContext & BOUNDARY_TAINT) != 0) boundary++;
			}
		}
		List<Map.Entry<String, Integer>> top = new ArrayList<Map.Entry<String, Integer>>(ruleHisto.entrySet());
		top.sort((a, b) -> b.getValue() - a.getValue());
		StringBuilder sb = new StringBuilder();
		sb.append("OVERFLOW-DEBUG d=").append(s.decision)
		  .append(" depthBound=").append(depthBound)
		  .append(" states=").append(states.size())
		  .append(" configs=").append(totalConfigs)
		  .append(" maxSet=").append(maxSetSize)
		  .append(" emptyCtx=").append(emptyCtx)
		  .append(" precTaint=").append(precTainted)
		  .append(" widened=").append(widenedTainted)
		  .append(" boundary=").append(boundary)
		  .append("\n  top rules: ");
		for (int i = 0; i < Math.min(8, top.size()); i++) {
			sb.append(top.get(i).getKey()).append('=').append(top.get(i).getValue()).append(' ');
		}
		System.err.println(sb);
	}

	/** Alternative-set bitmask of a prefix-factor group (bit 1<<(alt-1)
	 *  per member alternative). */
	protected static long altBits(BitSet alts) {
		long bits = 0;
		for (int a = alts.nextSetBit(0); a >= 0; a = alts.nextSetBit(a+1)) {
			bits |= 1L << (a-1);
		}
		return bits;
	}

	/** Serialize the recorded DFA into the flat table form used by codegen. */
	protected static StaticDFA toStaticDFA(int decision,
										   List<List<IntervalSet>> edgeLabels,
										   List<List<Integer>> edgeTargets,
										   List<Integer> acceptAlts,
										   List<Long> acceptMasks,
										   List<Integer> fallbackAlts,
										   boolean cyclic, int maxK) {
		int n = edgeTargets.size();
		int[] accepts = new int[n];
		long[] masks = new long[n];
		int[] fallbacks = new int[n];
		int[] offsets = new int[n+1];
		List<int[]> triples = new ArrayList<int[]>();
		for (int s = 0; s < n; s++) {
			accepts[s] = acceptAlts.get(s);
			masks[s] = acceptMasks.get(s);
			fallbacks[s] = fallbackAlts.get(s);
			offsets[s] = triples.size()*3;
			List<int[]> stateTriples = new ArrayList<int[]>();
			for (int e = 0; e < edgeTargets.get(s).size(); e++) {
				int target = edgeTargets.get(s).get(e);
				for (Interval iv : edgeLabels.get(s).get(e).getIntervals()) {
					stateTriples.add(new int[]{iv.a, iv.b, target});
				}
			}
			// disjoint by construction; sort by lo for binary search
			stateTriples.sort((a, b) -> Integer.compare(a[0], b[0]));
			triples.addAll(stateTriples);
		}
		offsets[n] = triples.size()*3;
		int[] edges = new int[triples.size()*3];
		for (int i = 0; i < triples.size(); i++) {
			edges[i*3] = triples.get(i)[0];
			edges[i*3+1] = triples.get(i)[1];
			edges[i*3+2] = triples.get(i)[2];
		}
		return new StaticDFA(decision, n, accepts, masks, fallbacks, offsets, edges, cyclic, maxK);
	}

	/** A DFA edge under construction: token class label -> successor config set. */
	protected static final class LabeledSuccessor {
		final IntervalSet label;
		final Set<ATNConfig> configs;
		LabeledSuccessor(IntervalSet label, Set<ATNConfig> configs) {
			this.label = label;
			this.configs = configs;
		}
	}

	/**
	 * Compute the successor configuration sets of a DFA state, one edge per
	 * distinct successor set, labeled with the full token class (never per
	 * token). Token classes are the equivalence classes of the partition
	 * induced by the states' move labels; classes that close to the same
	 * successor set are merged into a single edge with the union label.
	 */
	protected List<LabeledSuccessor> successors(Set<ATNConfig> configs, Result res) {
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

		// partition the token space into equivalence classes over move labels:
		// between two consecutive boundary points every token behaves alike
		TreeSet<Integer> boundarySet = new TreeSet<Integer>();
		for (IntervalSet label : labels) {
			for (Interval iv : label.getIntervals()) {
				boundarySet.add(iv.a);
				boundarySet.add(iv.b+1);
			}
		}
		Integer[] boundaries = boundarySet.toArray(new Integer[0]);

		// accumulate the full label of each class (key = set of covering moves)
		Map<BitSet, IntervalSet> classLabels = new LinkedHashMap<BitSet, IntervalSet>();
		for (int b = 0; b < boundaries.length-1; b++) {
			int lo = boundaries[b];
			int hi = boundaries[b+1]-1;
			BitSet key = new BitSet(labels.size());
			for (int m = 0; m < labels.size(); m++) {
				if (labels.get(m).contains(lo)) key.set(m);
			}
			if (key.isEmpty()) continue;
			IntervalSet classLabel = classLabels.get(key);
			if (classLabel == null) {
				classLabel = new IntervalSet();
				classLabels.put(key, classLabel);
			}
			classLabel.add(lo, hi);
		}

		// close each class; merge classes that reach the same successor set
		Map<Set<ATNConfig>, IntervalSet> bySuccessor = new LinkedHashMap<Set<ATNConfig>, IntervalSet>();
		for (Map.Entry<BitSet, IntervalSet> e : classLabels.entrySet()) {
			BitSet key = e.getKey();
			Set<ATNConfig> succ = new LinkedHashSet<ATNConfig>();
			Set<Object> busy = new HashSet<Object>();
			for (int m = key.nextSetBit(0); m >= 0; m = key.nextSetBit(m+1)) {
				ATNConfig c = moveConfigs.get(m);
				if (moveIsStop.get(m)) {
					succ.add(c);
				}
				else {
					ATNConfig advanced = new ATNConfig(c, moveTargets.get(m));
					if (precRuleIndex >= 0
						&& (c.reachesIntoOuterContext & BOUNDARY_TAINT) != 0) {
						if (isLoopConsumption(c)) {
							// loop-block consumption by a boundary-popped
							// config: the phantom consumer is an enclosing
							// invocation of this loop (kept untainted so
							// the enter/exit substitution can fire)
						}
						else {
							// boundary-popped config consuming outside the
							// operator loop: the phantom consumer is not an
							// enclosing invocation of this loop
							advanced.reachesIntoOuterContext |= FOREIGN_CONSUME_TAINT;
							if ("full".equals(System.getProperty("antlr.dfa.debug"))
								&& (c.reachesIntoOuterContext & FOREIGN_CONSUME_TAINT) == 0) {
								System.err.printf("FOREIGN-MARK rule=%s state=%d alt=%d taint=%d ctx=%s label=%s%n",
									g.getRule(c.state.ruleIndex).name, c.state.stateNumber,
									c.alt, c.reachesIntoOuterContext, c.context,
									labels.get(m).toString(g.getVocabulary()));
							}
						}
					}
					closure(advanced, succ, busy, res, 0);
				}
			}
			succ = canonical(succ);
			IntervalSet merged = bySuccessor.get(succ);
			if (merged == null) {
				bySuccessor.put(succ, e.getValue());
			}
			else {
				merged.addAll(e.getValue());
			}
		}

		List<LabeledSuccessor> result = new ArrayList<LabeledSuccessor>(bySuccessor.size());
		for (Map.Entry<Set<ATNConfig>, IntervalSet> e : bySuccessor.entrySet()) {
			result.add(new LabeledSuccessor(e.getValue(), e.getKey()));
		}
		return result;
	}

	/**
	 * SLL epsilon closure, mirroring ParserATNSimulator's closure: predicates
	 * are traversed (and recorded), rule invocations push a singleton context
	 * (widened on recursion), rule stops pop or - with a wildcard context -
	 * chase the grammar-wide FOLLOW links the tool attaches to rule stop states.
	 *
	 * <p>{@code depth} is the runtime's frame-balance counter: +1 per rule
	 * invocation while non-negative (latched once the closure leaves the
	 * decision's frame), -1 per return. It exists to identify frame-0
	 * precedence predicates ({@code depth == 0}), which the runtime
	 * evaluates while computing a precedence DFA start state and which are
	 * evaluated here under the same conditions ({@link #precStartClosure}).
	 * Boundary FOLLOW chases force the depth negative: under widening a
	 * return can pop through the boundary from a nominally balanced depth,
	 * and a predicate reached that way belongs to a phantom frame, never to
	 * frame 0.</p>
	 */
	protected void closure(ATNConfig config, Set<ATNConfig> configs, Set<Object> busy, Result res, int depth) {
		// The busy key must distinguish pre- from post-boundary lineage
		// (ATNConfig equality ignores taint): under per-precedence-class
		// construction the decision rule's stop state is chased with the
		// class's call-site filter from the decision's own frame
		// (depth >= 0) but unfiltered from phantom enclosing frames
		// (depth < 0). An identical (state, alt, ctx) key arriving first
		// via the filtered pop must not swallow the later unfiltered one -
		// that dedup once dropped the "exit two loops, BETWEEN takes the
		// AND" derivations. The runtime has no such filter, so its
		// first-arrival closure is always complete and its closureBusy
		// key needs no lineage bit.
		if (!busy.add(depth >= 0 ? (Object)config
				: (Object)new java.util.AbstractMap.SimpleEntry<ATNConfig, Boolean>(config, Boolean.TRUE))) {
			return;
		}
		ATNState p = config.state;

		if (p instanceof RuleStopState) {
			PredictionContext ctx = config.context;
			if (ctx != null && !ctx.isEmpty()) {
				for (int i = 0; i < ctx.size(); i++) {
					if (ctx.getReturnState(i) == PredictionContext.EMPTY_RETURN_STATE) {
						closure(new ATNConfig(config, p, EmptyPredictionContext.Instance), configs, busy, res, depth);
					}
					else {
						ATNState returnState = atn.states.get(ctx.getReturnState(i));
						closure(new ATNConfig(config, returnState, ctx.getParent(i)), configs, busy, res, depth-1);
					}
				}
				return;
			}
			// Wildcard context: popping through the decision-entry boundary.
			// Taint the config (its descendants inherit through the copy
			// constructors), keep it (it participates in stop-state conflict
			// detection and matches EOF), and additionally chase FOLLOW
			// links via the epsilon transitions below.
			config.reachesIntoOuterContext |= BOUNDARY_TAINT;
			configs.add(config);
		}

		if (!p.onlyHasEpsilonTransitions()) {
			configs.add(config);
		}

		// p being a rule stop here means the context was empty: the epsilon
		// transitions below are the grammar-wide FOLLOW links
		boolean boundaryChase = p instanceof RuleStopState;

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
				closure(callee, configs, busy, res, depth >= 0 ? depth+1 : depth);
			}
			else if (t instanceof PrecedencePredicateTransition) {
				int n = ((PrecedencePredicateTransition)t).precedence;
				if (precStartClosure && depth == 0) {
					// Frame-0 guard of the precedence decision under
					// construction: evaluate precpred(n) = n >= p against
					// the class representative, exactly like the runtime's
					// precedence DFA start-state computation (pass 1 of
					// applyPrecedenceFilter). Failing configs are
					// eliminated; passing ones continue untainted.
					if (n >= precEvalValue) {
						closure(new ATNConfig(config, t.target), configs, busy, res, depth);
					}
				}
				else {
					// Deeper guard: evaluate against the constant precedence
					// the enclosing rule was entered with, when the context
					// identifies it (see evalPrecpredFromContext, including
					// the argument for why the pruning is behavior-
					// preserving even though adaptivePredict traverses these
					// guards as epsilon). Unknown entry precedence falls
					// back to the runtime's own treatment - assumed true -
					// tainted as an over-approximation.
					Boolean outcome = evalPrecpredFromContext(config.context, p.ruleIndex, n);
					if (outcome != null) {
						if (outcome) {
							closure(new ATNConfig(config, t.target), configs, busy, res, depth);
						}
						// else: guard provably fails; path pruned
					}
					else {
						res.sawPrecPredicate = true;
						ATNConfig c2 = new ATNConfig(config, t.target);
						c2.reachesIntoOuterContext |= PRECPRED_TAINT;
						closure(c2, configs, busy, res, depth);
					}
				}
			}
			else if (t instanceof PredicateTransition) {
				res.sawPredicate = true;
				closure(new ATNConfig(config, t.target), configs, busy, res, depth);
			}
			else if (t.isEpsilon()) {
				if (boundaryChase && precRuleIndex >= 0 && p.ruleIndex == precRuleIndex
					&& depth >= 0) {
					// Popping the decision boundary of the precedence rule
					// under a per-class construction - from the decision's
					// own frame (depth >= 0, first pop): only call sites
					// whose precedence argument lies in the class interval
					// can be the invocation this table serves (see
					// precClassLo). FOLLOW links to incompatible call sites
					// are phantom paths the runtime (with its real stack)
					// never takes. Later same-rule stops (depth < 0) belong
					// to phantom *enclosing* invocations of arbitrary entry
					// precedence and must chase every call site: filtering
					// them by this class's interval once dropped the real
					// "exit two loops, let BETWEEN take the AND" derivations
					// and misparsed FRAME BETWEEN a OR b AND c AND d.
					RuleTransition call = callByFollowState().get(t.target.stateNumber);
					if (call != null && call.target.ruleIndex == precRuleIndex
						&& (call.precedence < precClassLo || call.precedence > precClassHi)) {
						continue;
					}
				}
				ATNConfig c2 = new ATNConfig(config, t.target);
				int newDepth = depth;
				if (boundaryChase) {
					// definitively outside frame 0 (see method doc)
					newDepth = Math.min(depth-1, -1);
					if (precStartClosure && p.ruleIndex == precRuleIndex) {
						// Re-derive outermostPrecedenceReturn: this FOLLOW
						// link returns from the decision's own rule to a
						// call site that invoked it with precedence 0, so a
						// real outer invocation may legitimately take an
						// operator here - the precedence filter must not
						// eliminate this path (ATNDeserializer marks such
						// links for ParserATNSimulator#closure_).
						RuleTransition call = callByFollowState.get(t.target.stateNumber);
						if (call != null && call.precedence == 0
							&& call.target.ruleIndex == precRuleIndex) {
							c2.setPrecedenceFilterSuppressed(true);
						}
					}
				}
				closure(c2, configs, busy, res, newDepth);
			}
		}
	}

	/**
	 * Is a token consumption by this (boundary-popped) config attributable
	 * to the operator loop of the precedence rule under construction -
	 * directly (at a loop-block state) or as an operand descent (a
	 * loop-block return state somewhere in the calling context)?
	 */
	protected boolean isLoopConsumption(ATNConfig c) {
		if (precLoopStates.get(c.state.stateNumber)) return true;
		return hasLoopReturn(c.context,
			java.util.Collections.newSetFromMap(new java.util.IdentityHashMap<PredictionContext, Boolean>()));
	}

	/**
	 * Does the context's top-level frame chain contain a same-rule return
	 * state with an outgoing token transition - i.e., is this re-entered
	 * operand's result awaited by pending tokens of a compound
	 * alternative (like the ','/'>>' after a list element), rather than
	 * an alternative tail (the operand of a simple operator, after which
	 * the alternative is complete)?
	 */
	/**
	 * Is every context frame adjacent to the empty context (the deepest
	 * frame of each context branch) a return into {@code endState}? For
	 * the optional-postfix mirror check: a skip configuration mirrors
	 * the take reading iff it re-entered the decision through this
	 * decision's own take path, i.e. its deepest frame's return state is
	 * the decision block's end state. A branch that reaches the empty
	 * context without such a frame means the configuration consumed
	 * tokens fully outside the decision's discipline (foreign).
	 */
	private boolean bottomFrameIs(PredictionContext ctx, int endState,
								  Set<PredictionContext> visited) {
		if (ctx == null || ctx.isEmpty()) return false;
		if (!visited.add(ctx)) return true;
		for (int i = 0; i < ctx.size(); i++) {
			PredictionContext p = ctx.getParent(i);
			if (p == null || p.isEmpty()) {
				if (ctx.getReturnState(i) != endState) return false;
			}
			else if (!bottomFrameIs(p, endState, visited)) {
				return false;
			}
		}
		return true;
	}

	private boolean hasPendingPrecReturn(PredictionContext ctx) {
		if (ctx == null || ctx.isEmpty()) return false;
		for (int i = 0; i < ctx.size(); i++) {
			int rs = ctx.getReturnState(i);
			if (rs != PredictionContext.EMPTY_RETURN_STATE) {
				ATNState st = atn.states.get(rs);
				if (st.ruleIndex == precRuleIndex
					&& precPendingStates.get(rs)) {
					return true;
				}
			}
		}
		return false;
	}

	/**
	 * Does the context chain contain a same-rule return state with a
	 * token obligation outside the iteration-start region - a phantom
	 * frame of the precedence rule that cannot be discharged by iterating
	 * (see {@link #precPendingStates})?
	 */
	private boolean hasObligatedReturn(PredictionContext ctx, Set<PredictionContext> visited) {
		if (ctx == null || ctx.isEmpty() || !visited.add(ctx)) return false;
		for (int i = 0; i < ctx.size(); i++) {
			int rs = ctx.getReturnState(i);
			if (rs != PredictionContext.EMPTY_RETURN_STATE) {
				ATNState st = atn.states.get(rs);
				if (st.ruleIndex == precRuleIndex
					&& precPendingStates.get(rs)) {
					return true;
				}
				if (hasObligatedReturn(ctx.getParent(i), visited)) return true;
			}
		}
		return false;
	}

	private boolean hasLoopReturn(PredictionContext ctx, Set<PredictionContext> visited) {
		if (ctx == null || ctx.isEmpty() || !visited.add(ctx)) return false;
		for (int i = 0; i < ctx.size(); i++) {
			int rs = ctx.getReturnState(i);
			if (rs != PredictionContext.EMPTY_RETURN_STATE) {
				if (precLoopStates.get(rs)) return true;
				if (hasLoopReturn(ctx.getParent(i), visited)) return true;
			}
		}
		return false;
	}

	/**
	 * Debug dump (-Dantlr.dfa.debug=states -Dantlr.dfa.debug.decision=N):
	 * every constructed DFA state of decision N with depth, accept, edges
	 * (token names), and configs decoded to rule names; context return
	 * states are decoded via callByFollowState to the invoked rule.
	 */
	protected void dumpStates(DecisionState s, List<Set<ATNConfig>> states,
							  List<List<Integer>> edges, List<List<IntervalSet>> edgeLabels,
							  List<Integer> acceptAlts, List<Integer> stateDepth) {
		System.err.printf("=== DFA states d=%d class=[%d,%d] rep=%d ===%n",
			s.decision, precClassLo, precClassHi, precEvalValue);
		for (int i = 0; i < states.size(); i++) {
			StringBuilder edgeStr = new StringBuilder();
			for (int e = 0; e < edges.get(i).size(); e++) {
				if (e > 0) edgeStr.append(' ');
				edgeStr.append(edgeLabels.get(i).get(e).toString(g.getVocabulary()))
					.append("->").append(edges.get(i).get(e));
			}
			System.err.printf("state %d depth=%d accept=%d edges: %s%n",
				i, stateDepth.get(i), acceptAlts.get(i), edgeStr);
			for (ATNConfig c : states.get(i)) {
				System.err.printf("    alt=%d %s:%d taint=%d ctx=%s%n",
					c.alt, g.getRule(c.state.ruleIndex).name, c.state.stateNumber,
					c.reachesIntoOuterContext, decodeContext(c.context));
			}
		}
	}

	/** Decode a context's return states to invoked-rule names for dumps. */
	protected String decodeContext(PredictionContext ctx) {
		if (ctx == null || ctx.isEmpty()) return "$";
		StringBuilder sb = new StringBuilder("[");
		for (int i = 0; i < ctx.size(); i++) {
			if (i > 0) sb.append(", ");
			int rs = ctx.getReturnState(i);
			if (rs == PredictionContext.EMPTY_RETURN_STATE) {
				sb.append("$");
			}
			else {
				RuleTransition call = callByFollowState().get(rs);
				sb.append(call != null ? g.getRule(call.target.ruleIndex).name : "?")
					.append('@').append(rs)
					.append(' ').append(decodeContext(ctx.getParent(i)));
			}
		}
		return sb.append(']').toString();
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
	// Table building (codegen entry point)
	// ---------------------------------------------------------------------

	/**
	 * Build serialized prediction tables for every decision that is
	 * statically decidable with behavior preservation and not already served
	 * by the LL(1) fast path, storing them on the grammar
	 * ({@link Grammar#staticDecisionDFAs} for plain decisions,
	 * {@link Grammar#staticPrecedenceDFAs} for the per-precedence table
	 * groups of left-recursive loop decisions). Everything else (the
	 * "unsafe residue": context-sensitive/overflow/predicated/non-greedy
	 * decisions and precedence decisions with any non-static class) is left
	 * to {@code adaptivePredict}.
	 */
	public static void buildTables(Grammar g) {
		DecisionClassifier classifier = new DecisionClassifier(g);
		classifier.abortOnUntrustedConflict = true;
		Map<Integer, StaticDFA> tables = new LinkedHashMap<Integer, StaticDFA>();
		Map<Integer, PrecedenceStaticDFA> precTables = new LinkedHashMap<Integer, PrecedenceStaticDFA>();
		Map<Integer, PrefixFactorAnalyzer.Plan> factorPlans =
			new LinkedHashMap<Integer, PrefixFactorAnalyzer.Plan>();
		for (DecisionState s : g.atn.decisionToState) {
			// the LL(1) fast path already covers disjoint decisions
			if (g.decisionLOOK != null && s.decision < g.decisionLOOK.size()
				&& AnalysisPipeline.disjoint(g.decisionLOOK.get(s.decision))) {
				continue;
			}
			Result r = classifier.classify(s);
			if (r.dfa != null) {
				tables.put(s.decision, r.dfa);
			}
			else if (r.precDfa != null) {
				precTables.put(s.decision, r.precDfa);
			}
			if (r.factorPlan != null && r.factorPlan.hasGroups()) {
				factorPlans.put(s.decision, r.factorPlan);
			}
		}
		g.staticDecisionDFAs = tables;
		g.staticPrecedenceDFAs = precTables;
		g.staticFactorPlans = factorPlans;
		addSyntheticContinuationStates(g, factorPlans);
	}

	/**
	 * Create, for every group of every factor plan, the two synthetic ATN
	 * states the factored codegen needs (see
	 * {@link PrefixFactorAnalyzer.Group#syntheticInvokeState}): the shared
	 * prefix's rule invocation must push a call-site whose continuation
	 * keeps every member's tail viable, so nested adaptive simulations
	 * inside the shared invocation resolve against the real tail decision
	 * rather than against one member's (arbitrary) alternative
	 * continuation. The states are appended to the grammar's ATN before it
	 * is serialized into the generated parser; they are unreachable from
	 * any rule start.
	 */
	private static void addSyntheticContinuationStates(
		Grammar g, Map<Integer, PrefixFactorAnalyzer.Plan> factorPlans)
	{
		for (PrefixFactorAnalyzer.Plan plan : factorPlans.values()) {
			for (PrefixFactorAnalyzer.Group grp : plan.groups) {
				int ruleIndex = g.atn.decisionToState.get(plan.decision).ruleIndex;
				int sharedRule = grp.prefix.get(grp.prefix.size()-1).id;
				// The tail state fans out (epsilon) to every member's tail
				// start, so it must be a real decision state: non-decision
				// states may only have one transition (ATNDeserializer's
				// verifyATN). It never serves as a prediction call site.
				org.antlr.v4.runtime.atn.BasicBlockStartState tail =
					new org.antlr.v4.runtime.atn.BasicBlockStartState();
				tail.ruleIndex = ruleIndex;
				g.atn.addState(tail);
				org.antlr.v4.runtime.atn.BlockEndState tailEnd =
					new org.antlr.v4.runtime.atn.BlockEndState();
				tailEnd.ruleIndex = ruleIndex;
				g.atn.addState(tailEnd);
				tail.endState = tailEnd;
				tailEnd.startState = tail;
				g.atn.defineDecisionState(tail);
				for (int alt = grp.alts.nextSetBit(0); alt >= 0; alt = grp.alts.nextSetBit(alt+1)) {
					tail.addTransition(new org.antlr.v4.runtime.atn.EpsilonTransition(
						g.atn.states.get(grp.tailStartState.get(alt))));
				}
				org.antlr.v4.runtime.atn.BasicState invoke =
					new org.antlr.v4.runtime.atn.BasicState();
				invoke.ruleIndex = ruleIndex;
				g.atn.addState(invoke);
				invoke.addTransition(new org.antlr.v4.runtime.atn.RuleTransition(
					g.atn.ruleToStartState[sharedRule], sharedRule, tail));
				grp.syntheticTailState = tail.stateNumber;
				grp.syntheticInvokeState = invoke.stateNumber;
			}
		}
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

		int precStatic = 0;
		for (Result r : results) {
			if (r.precDfa != null) precStatic++;
		}

		// Runtime adaptivePredict fallback accounting: the table categories
		// alone are deceptive - a "static" decision may still carry hybrid
		// escape states or adaptive precedence classes that hand individual
		// predictions back to the adaptive engine at runtime.
		int fbStaticOnly = 0, fbEscapeStates = 0, fbHybridDecisions = 0;
		int fbAdaptiveClasses = 0, fbAdaptiveOnly = 0;
		for (Result r : results) {
			if (r.category == Category.LR_PRECEDENCE) {
				if (r.precDfa == null) {
					fbAdaptiveOnly++;
				}
				else {
					fbEscapeStates += r.escapes;
					fbAdaptiveClasses += r.adaptiveClasses;
					if (r.escapes > 0) fbHybridDecisions++;
					else if (r.adaptiveClasses == 0) fbStaticOnly++;
				}
			}
			else if (r.category == Category.LL1 || (r.dfa != null && r.escapes == 0)) {
				fbStaticOnly++;
			}
			else if (r.dfa == null) {
				fbAdaptiveOnly++;
			}
			else {
				fbHybridDecisions++;
				fbEscapeStates += r.escapes;
			}
		}

		StringBuilder buf = new StringBuilder();
		buf.append("=== decision report: grammar ").append(g.name)
		   .append(" (").append(results.size()).append(" decisions) ===\n");
		buf.append("summary:");
		for (Map.Entry<Category, Integer> e : counts.entrySet()) {
			if (e.getValue() > 0) {
				buf.append(' ').append(e.getKey()).append('=').append(e.getValue());
				if (e.getKey() == Category.LR_PRECEDENCE && precStatic > 0) {
					buf.append('(').append(precStatic).append(" static)");
				}
			}
		}
		buf.append('\n');
		buf.append(String.format(
			"fallbacks: static-only=%d/%d; escape-states=%d across %d hybrid decisions"
				+ " + %d adaptive precedence classes; adaptive-only decisions=%d%n",
			fbStaticOnly, results.size(), fbEscapeStates, fbHybridDecisions,
			fbAdaptiveClasses, fbAdaptiveOnly));

		// alt-mask dry run totals
		int factorDecisions = 0, factorGroups = 0, factorCured = 0, factorMaskStates = 0;
		for (Result r : results) {
			if (r.factorPlan != null && r.factorPlan.hasGroups()) {
				factorDecisions++;
				factorGroups += r.factorPlan.groups.size();
				factorCured += r.factorEscapesCured;
				factorMaskStates += r.factorMaskStates;
			}
		}
		if (factorDecisions > 0) {
			buf.append(String.format(
				"factorable: %d decisions (%d groups); mask-accepts=%d states; escapes curable=%d%n",
				factorDecisions, factorGroups, factorMaskStates, factorCured));
		}
		int postfixDecisions = 0, takeFires = 0;
		for (Result r : results) {
			if (r.hasPostfixShape) postfixDecisions++;
			takeFires += r.takeRuleFires;
		}
		if (postfixDecisions > 0) {
			buf.append(String.format("take-rule: %d optional-postfix decisions; fires at %d conflict states%n",
				postfixDecisions, takeFires));
		}

		int maxKAnyAcyclic = 0;
		int maxKLLK = 0;
		Map<Integer, Integer> llkHistogram = new java.util.TreeMap<Integer, Integer>();
		for (Result r : results) {
			if (r.category == Category.LL1) continue;
			if (r.category == Category.LR_PRECEDENCE) {
				if (r.precNote != null) {
					Rule lrRule = g.getRule(r.decisionState.ruleIndex);
					buf.append(String.format("%-22s d=%-4d %-17s", lrRule.name,
						r.decisionState.decision, r.category));
					buf.append(' ').append(r.precNote)
					   .append(r.precDfa != null ? " [static]" : " [adaptive]")
					   .append(" fb=").append(fbString(r)).append('\n');
				}
				continue;
			}
			if (r.k > maxKAnyAcyclic) maxKAnyAcyclic = r.k;
			if (r.category == Category.LLK) {
				maxKLLK = Math.max(maxKLLK, r.k);
				llkHistogram.merge(r.k, 1, Integer::sum);
			}
			Rule rule = g.getRule(r.decisionState.ruleIndex);
			buf.append(String.format("%-22s d=%-4d %-17s", rule.name, r.decisionState.decision, r.category));
			buf.append(" fb=").append(fbString(r));
			if (r.k >= 0) buf.append(" k=").append(r.k);
			buf.append(" dfaStates=").append(r.numDfaStates);
			if (r.category == Category.HYBRID) {
				buf.append(" escapes=").append(r.escapes)
				   .append(String.format(" cover=%.0f%%", r.coverage*100))
				   .append(r.dfa == null ? " [adaptive]" : "");
			}
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
			if (r.factorPlan != null && r.factorPlan.hasGroups()) {
				buf.append(" [factorable: groups=").append(r.factorPlan.groups.size())
				   .append(" maskStates=").append(r.factorMaskStates)
				   .append(" cures=").append(r.factorEscapesCured)
				   .append(']');
			}
			if (r.hasPostfixShape) {
				buf.append(" [postfix: takeFires=").append(r.takeRuleFires).append(']');
			}
			buf.append('\n');
		}
		buf.append("LLK k histogram: ").append(llkHistogram).append('\n');
		buf.append("max k: LLK=").append(maxKLLK)
		   .append(", any acyclic non-LL(1) (incl. conflicted)=").append(maxKAnyAcyclic).append('\n');
		return buf.toString();
	}

	/**
	 * Runtime-fallback descriptor of a decision: "0" when no prediction of
	 * this decision can reach adaptivePredict (pure LL(1) code or a
	 * complete static table); "all" when the decision has no table at all
	 * (fully adaptive); otherwise the number of table states that defer to
	 * the adaptive engine - escape states of a hybrid table ("N"), and for
	 * precedence-dispatched decisions escape states plus classes without
	 * tables ("Ne+Mc").
	 */
	private static String fbString(Result r) {
		if (r.category == Category.LR_PRECEDENCE) {
			if (r.precDfa == null) return "all";
			if (r.escapes == 0 && r.adaptiveClasses == 0) return "0";
			StringBuilder s = new StringBuilder();
			if (r.escapes > 0) s.append(r.escapes).append('e');
			if (r.adaptiveClasses > 0) {
				if (s.length() > 0) s.append('+');
				s.append(r.adaptiveClasses).append('c');
			}
			return s.toString();
		}
		if (r.category == Category.LL1) return "0";
		if (r.dfa == null) return "all";
		return r.escapes > 0 ? String.valueOf(r.escapes) : "0";
	}

	private static Set<BitSet> distinct(List<BitSet> sets) {
		return new LinkedHashSet<BitSet>(sets);
	}
}
