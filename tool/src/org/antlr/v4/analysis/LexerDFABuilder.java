/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.analysis;

import org.antlr.runtime.Token;
import org.antlr.v4.runtime.CharStream;
import org.antlr.v4.runtime.IntStream;
import org.antlr.v4.runtime.atn.ATN;
import org.antlr.v4.runtime.atn.ATNConfig;
import org.antlr.v4.runtime.atn.ATNConfigSet;
import org.antlr.v4.runtime.atn.ATNSimulator;
import org.antlr.v4.runtime.atn.ATNState;
import org.antlr.v4.runtime.atn.ActionTransition;
import org.antlr.v4.runtime.atn.AtomTransition;
import org.antlr.v4.runtime.atn.LexerATNSimulator;
import org.antlr.v4.runtime.atn.NotSetTransition;
import org.antlr.v4.runtime.atn.PredicateTransition;
import org.antlr.v4.runtime.atn.RangeTransition;
import org.antlr.v4.runtime.atn.RuleTransition;
import org.antlr.v4.runtime.atn.SetTransition;
import org.antlr.v4.runtime.atn.Transition;
import org.antlr.v4.runtime.atn.WildcardTransition;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.dfa.DFAState;
import org.antlr.v4.runtime.atn.PredictionContextCache;
import org.antlr.v4.runtime.atn.LexerAction;
import org.antlr.v4.runtime.atn.LexerActionExecutor;
import org.antlr.v4.runtime.misc.Interval;
import org.antlr.v4.runtime.misc.IntervalSet;
import org.antlr.v4.tool.ErrorType;
import org.antlr.v4.tool.Grammar;
import org.antlr.v4.tool.LexerGrammar;
import org.antlr.v4.tool.ast.GrammarAST;
import org.antlr.v4.tool.Rule;
import org.antlr.v4.tool.ast.ActionAST;
import org.antlr.v4.tool.ast.PredAST;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Deque;
import java.util.IdentityHashMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeSet;

/**
 * Builds, at tool time, the complete lexer DFA that the runtime's
 * {@code LexerATNSimulator} would otherwise construct lazily, for lexers
 * that need no runtime input-dependent machinery (see eligibility below).
 * The result is one {@link StaticLexerDFA} per lexer mode, serialized into
 * the generated recognizer; a small table walker in the target runtime then
 * tokenizes with no ATN simulation, no DFA-state allocation, and a dense
 * per-byte dispatch.
 *
 * <p><b>Faithfulness by construction.</b> The expansion literally drives
 * the reference {@link LexerATNSimulator} (start-state computation and
 * target-state computation) over the entire input alphabet, so accept
 * choice (maximal munch, then rule priority), non-greedy loop handling, and
 * lexer-action executor accumulation are exactly the runtime's. The only
 * difference from the lazy runtime DFA is that edges above
 * {@link StaticLexerDFA#ASCII_MAX} and the EOF edge - which the runtime
 * never caches - are precomputed too (they are input-independent in an
 * eligible lexer).</p>
 *
 * <p><b>Eligibility</b> (checked over the whole lexer ATN; if any check
 * fails, no tables are built and the runtime silently keeps its lazy
 * simulator):</p>
 *
 * <ul>
 * <li>No semantic predicates ({@code {...}?}) anywhere in the lexer: their
 * outcome depends on runtime state, so states/edges involving them are
 * never cached even by the lazy DFA.</li>
 * <li>No position-dependent lexer actions (embedded {@code {...}} code):
 * their execution offset depends on the match position.</li>
 * <li>The DFA stays within {@link #maxStates} states per mode (table-size
 * bound).</li>
 * <li>No recursive lexer rules: a rule invocation cycle makes the
 * exhaustive expansion non-terminating in practice (context-carrying
 * configs keep spawning fresh DFA states, and the reference simulator's
 * recursive closure blows the JVM stack long before the state cap
 * trips).</li>
 * </ul>
 *
 * <p>Modes, {@code mode}/{@code pushMode}/{@code popMode},
 * {@code type}/{@code channel}/{@code skip}/{@code more} commands,
 * non-greedy loops, and explicit {@code EOF} references in rules are all
 * supported.</p>
 */
public class LexerDFABuilder {
	/** Highest input code point. */
	public static final int MAX_CHAR = 0x10FFFF;

	/** Hard cap on DFA states per mode; overflow abandons the tables (always sound). */
	public int maxStates = Integer.parseInt(System.getProperty("antlr.lexerdfa.maxStates", "30000"));

	private final ATN atn;

	/** Mode whose expansion tripped {@link #maxStates}, or -1. */
	private int capExceededMode = -1;
	private LexerDFABuilder(ATN atn) {
		this.atn = atn;
	}

	/**
	 * Build static lexer DFA tables for {@code g} and store them in
	 * {@link Grammar#staticLexerDFAs}. When no tables are generated (the
	 * lexer is ineligible or a state cap trips), emit one warning per reason
	 * with grammar locations instead of silently falling back.
	 */
	public static void buildTables(Grammar g) {
		LexerDFABuilder builder = new LexerDFABuilder(g.atn);
		List<Ineligibility> reasons = builder.collectIneligibility(g);
		if (!reasons.isEmpty()) {
			for (Ineligibility reason : reasons) {
				g.tool.errMgr.grammarError(ErrorType.STATIC_LEXER_DFA_INELIGIBLE,
					g.fileName, reason.token, reason.reason);
			}
			return;
		}
		StaticLexerDFA[] tables = builder.build();
		if (tables != null) {
			g.staticLexerDFAs = tables;
		}
		else {
			g.tool.errMgr.toolError(ErrorType.STATIC_LEXER_DFA_STATE_CAP,
				builder.modeName(g, builder.capExceededMode), builder.maxStates);
		}
	}

	/** A single reason this lexer cannot be fully table-driven. */
	private static class Ineligibility implements Comparable<Ineligibility> {
		final int ruleIndex;
		/** Location of the offending rule or action (never null). */
		final Token token;
		final String reason;

		Ineligibility(int ruleIndex, Token token, String reason) {
			this.ruleIndex = ruleIndex;
			this.token = token;
			this.reason = reason;
		}

		/** Order by grammar location (line, then column). */
		@Override public int compareTo(Ineligibility o) {
			int d = token.getLine() - o.token.getLine();
			return d != 0 ? d : token.getCharPositionInLine() - o.token.getCharPositionInLine();
		}
	}

	/**
	 * Every reason this lexer ATN needs runtime evaluation, in grammar order;
	 * empty when the lexer is eligible for static table expansion.
	 */
	private List<Ineligibility> collectIneligibility(Grammar g) {
		List<Ineligibility> reasons = new ArrayList<Ineligibility>();
		// One warning per (rule, kind); a rule with several predicates gets one.
		Set<Integer> predRules = new LinkedHashSet<Integer>();
		Set<Integer> actionRules = new LinkedHashSet<Integer>();
		for (ATNState s : atn.states) {
			if (s == null) continue;
			for (Transition t : s.getTransitions()) {
				if (t instanceof PredicateTransition) {
					predRules.add(s.ruleIndex);
				}
				if (t instanceof ActionTransition
					&& atn.lexerActions[((ActionTransition) t).actionIndex].isPositionDependent()) {
					actionRules.add(s.ruleIndex);
				}
			}
		}
		for (Integer ruleIndex : predRules) {
			Rule r = g.getRule(ruleIndex);
			reasons.add(new Ineligibility(ruleIndex, actionToken(r, true),
				"lexer rule " + r.name + " contains a predicate ({...}?) that is evaluated at runtime"));
		}
		for (Integer ruleIndex : actionRules) {
			Rule r = g.getRule(ruleIndex);
			reasons.add(new Ineligibility(ruleIndex, actionToken(r, false),
				"lexer rule " + r.name + " contains an embedded action ({...}) whose effect may depend on the input position"));
		}
		for (Integer ruleIndex : recursiveRules()) {
			Rule r = g.getRule(ruleIndex);
			reasons.add(new Ineligibility(ruleIndex, ruleNameToken(r),
				"lexer rule " + r.name + " can recursively invoke itself (recursive rules have no finite DFA expansion)"));
		}
		Collections.sort(reasons);
		return reasons;
	}

	/** Location of {@code r}'s name in the grammar source. */
	private static Token ruleNameToken(Rule r) {
		return ((GrammarAST) r.ast.getChild(0)).getToken();
	}

	/**
	 * Location of the first predicate ({@code predicates=true}) or embedded
	 * action in {@code r}, falling back to the rule name token.
	 */
	private static Token actionToken(Rule r, boolean predicates) {
		for (ActionAST a : r.actions) {
			if ((a instanceof PredAST) == predicates && a.getToken() != null) {
				return a.getToken();
			}
		}
		return ruleNameToken(r);
	}

	/** Indexes of all lexer rules that can (transitively) invoke themselves. */
	private Set<Integer> recursiveRules() {
		int numRules = atn.ruleToStartState.length;
		List<List<Integer>> calls = new ArrayList<List<Integer>>(numRules);
		for (int i = 0; i < numRules; i++) calls.add(new ArrayList<Integer>());
		for (ATNState s : atn.states) {
			if (s == null) continue;
			for (Transition t : s.getTransitions()) {
				if (t instanceof RuleTransition) {
					calls.get(s.ruleIndex).add(((RuleTransition)t).target.ruleIndex);
				}
			}
		}
		// DFS from each rule; a cycle back to the start rule = recursion.
		Set<Integer> recursive = new LinkedHashSet<Integer>();
		for (int start = 0; start < numRules; start++) {
			if (calls.get(start).isEmpty()) continue;
			boolean[] seen = new boolean[numRules];
			List<Integer> stack = new ArrayList<Integer>(calls.get(start));
			while (!stack.isEmpty()) {
				int r = stack.remove(stack.size() - 1);
				if (r == start) { recursive.add(start); break; }
				if (seen[r]) continue;
				seen[r] = true;
				stack.addAll(calls.get(r));
			}
		}
		return recursive;
	}

	/** Display name of lexer mode {@code mode} for diagnostics. */
	private static String modeName(Grammar g, int mode) {
		if (g instanceof LexerGrammar) {
			List<String> names = new ArrayList<String>(((LexerGrammar) g).modes.keySet());
			if (mode >= 0 && mode < names.size()) return names.get(mode);
		}
		return String.valueOf(mode);
	}

	private StaticLexerDFA[] build() {
		int numModes = atn.modeToStartState.size();
		DFA[] decisionToDFA = new DFA[numModes];
		for (int m = 0; m < numModes; m++) {
			decisionToDFA[m] = new DFA(atn.modeToStartState.get(m), m);
		}
		Sim sim = new Sim(atn, decisionToDFA);
		StaticLexerDFA[] tables = new StaticLexerDFA[numModes];
		for (int m = 0; m < numModes; m++) {
			sim.setMode(m);
			StaticLexerDFA table = buildMode(sim, m);
			if (table == null) return null; // state cap hit: no tables at all
			tables[m] = table;
		}
		return tables;
	}

	/** Exhaustively expand one mode's DFA; null on state-cap overflow. */
	private StaticLexerDFA buildMode(Sim sim, int mode) {
		DFAState s0 = sim.startState(DUMMY_INPUT, atn.modeToStartState.get(mode));

		IdentityHashMap<DFAState, Integer> ids = new IdentityHashMap<DFAState, Integer>();
		List<DFAState> states = new ArrayList<DFAState>();
		Deque<DFAState> work = new ArrayDeque<DFAState>();
		ids.put(s0, 0);
		states.add(s0);
		work.add(s0);

		List<int[]> asciiRows = new ArrayList<int[]>();
		List<int[]> hiRows = new ArrayList<int[]>();   // flattened (lo,hi,target) per state
		List<Integer> eofTargets = new ArrayList<Integer>();

		while (!work.isEmpty()) {
			DFAState s = work.poll();
			int si = ids.get(s);
			if (states.size() > maxStates) { capExceededMode = mode; return null; }

			int[] ascii = new int[StaticLexerDFA.ASCII_MAX + 1];
			Arrays.fill(ascii, -1);
			List<Integer> hi = new ArrayList<Integer>();

			// EOF edge (never cached by the runtime, but input-independent)
			DFAState eofTarget = sim.target(DUMMY_INPUT, s, Token.EOF);
			eofTargets.add(eofTarget == ATNSimulator.ERROR ? -1 : entryOf(eofTarget, ids, states, work));

			// Partition [0, MAX_CHAR] at every label boundary of this
			// state's transitions; the target is constant within each
			// resulting interval because no transition's membership changes
			// inside it.
			TreeSet<Integer> bounds = new TreeSet<Integer>();
			bounds.add(0);
			bounds.add(MAX_CHAR + 1);
			for (ATNConfig c : s.configs) {
				for (Transition t : c.state.getTransitions()) {
					IntervalSet m = matchSet(t);
					if (m == null) continue;
					for (Interval iv : m.getIntervals()) {
						bounds.add(Math.max(iv.a, 0));
						if (iv.b < MAX_CHAR) bounds.add(iv.b + 1);
					}
				}
			}
			Integer prev = null;
			for (Integer b : bounds) {
				if (prev != null && prev < b) {
					int lo = prev, hiEnd = b - 1;
					DFAState target = sim.target(DUMMY_INPUT, s, lo);
					if (target != ATNSimulator.ERROR) {
						int ti = entryOf(target, ids, states, work);
						int denseEnd = Math.min(hiEnd, StaticLexerDFA.ASCII_MAX);
						for (int c = lo; c <= denseEnd; c++) ascii[c] = ti;
						if (hiEnd > StaticLexerDFA.ASCII_MAX) {
							int hiLo = Math.max(lo, StaticLexerDFA.ASCII_MAX + 1);
							// merge with previous interval when contiguous
							int n = hi.size();
							if (n >= 3 && hi.get(n - 1) == ti && hi.get(n - 2) == hiLo - 1) {
								hi.set(n - 2, hiEnd);
							}
							else {
								hi.add(hiLo);
								hi.add(hiEnd);
								hi.add(ti);
							}
						}
					}
				}
				prev = b;
			}

			asciiRows.add(ascii);
			int[] hiArr = new int[hi.size()];
			for (int i = 0; i < hi.size(); i++) hiArr[i] = hi.get(i);
			hiRows.add(hiArr);
		}

		int numStates = states.size();
		int[] ascii = new int[numStates * (StaticLexerDFA.ASCII_MAX + 1)];
		int[] eof = new int[numStates];
		int[] hiOffsets = new int[numStates + 1];
		int hiTotal = 0;
		for (int[] r : hiRows) hiTotal += r.length;
		int[] hiEdges = new int[hiTotal];
		int[] acceptType = new int[numStates];
		int[] acceptActions = new int[numStates];
		List<Integer> actionLists = new ArrayList<Integer>();
		Map<List<Integer>, Integer> dedupLists = new java.util.HashMap<List<Integer>, Integer>();

		for (int i = 0; i < numStates; i++) {
			System.arraycopy(asciiRows.get(i), 0, ascii, i * (StaticLexerDFA.ASCII_MAX + 1), StaticLexerDFA.ASCII_MAX + 1);
			eof[i] = eofTargets.get(i);
		}
		// assemble hi edges with per-state offsets (in triple units, matching
		// the runtime's Vec<(lo, hi, target)> indexing; hiEdges itself stays
		// a flattened int array)

		int at = 0;
		for (int i = 0; i < numStates; i++) {
			hiOffsets[i] = at / 3;
			for (int v : hiRows.get(i)) hiEdges[at++] = v;
		}
		hiOffsets[numStates] = at / 3;

		for (int i = 0; i < numStates; i++) {
			DFAState s = states.get(i);
			if (!s.isAcceptState) {
				acceptType[i] = -1; // -1 = not an accept state; 0 is a valid accept token type
				acceptActions[i] = -1;
				continue;
			}
			acceptType[i] = s.prediction;
			LexerActionExecutor exec = s.lexerActionExecutor;
			if (exec == null || exec.getLexerActions().length == 0) {
				acceptActions[i] = -1;
				continue;
			}
			List<Integer> list = new ArrayList<Integer>();
			for (LexerAction a : exec.getLexerActions()) {
				list.add(actionIndex(a));
			}
			Integer off = dedupLists.get(list);
			if (off == null) {
				off = actionLists.size();
				actionLists.add(list.size());
				actionLists.addAll(list);
				dedupLists.put(list, off);
			}
			acceptActions[i] = off;
		}

		int[] actionListsArr = new int[actionLists.size()];
		for (int i = 0; i < actionLists.size(); i++) actionListsArr[i] = actionLists.get(i);

		return new StaticLexerDFA(mode, numStates, ids.get(s0), ascii, eof,
			hiOffsets, hiEdges, acceptType, acceptActions, actionListsArr);
	}

	/**
 	 * Table id of {@code s} (assigning and enqueueing a fresh one), with
 	 * {@link StaticLexerDFA#ACCEPT_BIT_INT} set when {@code s} is an accept
 	 * state - so the runtime walker's per-char loop reads accept info from
 	 * the same table entry as the target state.
 	 */
	private static int entryOf(DFAState s, IdentityHashMap<DFAState, Integer> ids,
							List<DFAState> states, Deque<DFAState> work) {
		int id = idOf(s, ids, states, work);
		return s.isAcceptState ? id | StaticLexerDFA.ACCEPT_BIT_INT : id;
	}

	private static int idOf(DFAState s, IdentityHashMap<DFAState, Integer> ids,
							List<DFAState> states, Deque<DFAState> work) {
		Integer id = ids.get(s);
		if (id == null) {
			id = states.size();
			ids.put(s, id);
			states.add(s);
			work.add(s);
		}
		return id;
	}

	/** The input symbols a transition can consume, or null for non-consuming transitions. */
	private static IntervalSet matchSet(Transition t) {
		if (t instanceof AtomTransition) {
			int label = ((AtomTransition)t).label;
			return label < 0 ? null : IntervalSet.of(label);
		}
		if (t instanceof RangeTransition) {
			RangeTransition r = (RangeTransition)t;
			return IntervalSet.of(r.from, r.to);
		}
		if (t instanceof SetTransition) {
			return ((SetTransition)t).set;
		}
		if (t instanceof NotSetTransition) {
			IntervalSet set = ((NotSetTransition)t).set;
			return set.complement(IntervalSet.of(0, MAX_CHAR));
		}
		if (t instanceof WildcardTransition) {
			return IntervalSet.of(0, MAX_CHAR);
		}
		return null; // epsilon, rule, action, predicate: consume nothing
	}

	private int actionIndex(LexerAction a) {
		for (int i = 0; i < atn.lexerActions.length; i++) {
			if (atn.lexerActions[i] == a) return i;
		}
		throw new IllegalStateException("lexer action not in ATN action list");
	}

	/**
	 * Exposes the reference simulator's protected construction entry points.
	 * {@code recog} is null: eligible lexers never evaluate predicates, and
	 * lexer actions are accumulated, not executed, during construction.
	 */
	private static final class Sim extends LexerATNSimulator {
		Sim(ATN atn, DFA[] decisionToDFA) {
			super(null, atn, decisionToDFA, new PredictionContextCache());
			this.startIndex = 0;
		}

		DFAState startState(CharStream input, ATNState p) {
			ATNConfigSet s0 = computeStartState(input, p);
			s0.hasSemanticContext = false;
			return addDFAState(s0);
		}

		void setMode(int m) { this.mode = m; }

		DFAState target(CharStream input, DFAState s, int t) {
			return computeTargetState(input, s, t);
		}
	}

	/**
	 * Stand-in stream: only {@link #index()} is consulted during
	 * construction (for position-dependent action offset fixing, which is a
	 * no-op on eligible lexers).
	 */
	private static final CharStream DUMMY_INPUT = new CharStream() {
		@Override public String getText(Interval interval) { return ""; }
		@Override public void consume() { }
		@Override public int LA(int i) { return IntStream.EOF; }
		@Override public int mark() { return -1; }
		@Override public void release(int marker) { }
		@Override public int index() { return 0; }
		@Override public void seek(int index) { }
		@Override public int size() { return 0; }
		@Override public String getSourceName() { return "<static-lexer-dfa>"; }
	};
}
