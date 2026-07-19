/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.analysis;

import org.antlr.v4.runtime.atn.ATN;
import org.antlr.v4.runtime.atn.ATNState;
import org.antlr.v4.runtime.atn.BlockStartState;
import org.antlr.v4.runtime.atn.DecisionState;
import org.antlr.v4.runtime.atn.RuleTransition;
import org.antlr.v4.runtime.atn.Transition;

import java.util.ArrayDeque;
import java.util.Deque;
import java.util.HashSet;
import java.util.Set;

/**
 * Detects <em>optional-postfix</em> decisions: two-alternative block
 * decisions of the shape {@code X Y?} - alternative 1 ("take") enters a
 * postfix (a rule invocation, possibly nested inside a compound block),
 * alternative 2 ("skip") is an epsilon path straight to the block end.
 * The {@code pred=predicate?} decision of the SQL dialects'
 * booleanExpression is the canonical example; {@code (AS? alias)?} is
 * the canonical compound shape.
 *
 * <p>Such a decision's take/skip conflict is a dangling-else shape:
 * whenever every token the skip reading consumes is mirrored into the
 * take structure itself (the phantom continuation funnels back through
 * this decision's own take path), skip is never uniquely viable and the
 * conflict statically resolves to take. See DecisionClassifier's
 * optional-postfix take rule.</p>
 */
public class OptionalPostfixAnalyzer {

	/** The detected optional-postfix shape of one decision. */
	public static final class Shape {
		public final int decision;
		/** State number of the decision block's end state: the position
		 *  the take reading reaches when the postfix completes. The
		 *  mirror check requires every descended skip configuration's
		 *  deepest context frame to be a return into this state (i.e.
		 *  the skip re-entered through this decision's own take path). */
		public final int blockEndState;
		/** Rule index of the postfix rule when alternative 1 begins with
		 *  a plain rule invocation; -1 for compound postfix blocks. */
		public final int postfixRule;
		public final boolean compound;

		public Shape(int decision, int blockEndState, int postfixRule, boolean compound) {
			this.decision = decision;
			this.blockEndState = blockEndState;
			this.postfixRule = postfixRule;
			this.compound = compound;
		}
	}

	private final ATN atn;

	public OptionalPostfixAnalyzer(ATN atn) { this.atn = atn; }

	/** The optional-postfix shape of the decision, or null. */
	public Shape analyze(DecisionState ds) {
		if (!(ds instanceof BlockStartState)) return null;
		if (ds.getNumberOfTransitions() != 2) return null;
		BlockStartState block = (BlockStartState)ds;

		// Alternative 2 must skip: an epsilon-only path to the block end.
		if (!epsilonPathTo(ds.transition(1).target, block.endState, new HashSet<Integer>())) {
			return null;
		}

		// Alternative 1 must head into a postfix. Walk epsilon links to
		// the first non-epsilon element.
		ATNState first = firstElement(ds.transition(0).target);
		if (first == null) return null;
		boolean compound = true;
		int postfixRule = -1;
		for (int i = 0; i < first.getNumberOfTransitions(); i++) {
			Transition t = first.transition(i);
			if (t instanceof RuleTransition) {
				postfixRule = ((RuleTransition)t).ruleIndex;
				compound = false;
				break;
			}
		}
		if (postfixRule < 0 && !(first instanceof BlockStartState)) return null;

		// Starvation check: if a mandatory element follows the optional
		// and its FIRST set intersects the optional's, the skipped input
		// can serve that element instead - skip genuinely wins there
		// (TRIM(x), IN foo), so the take rule must not apply. Safe when
		// the rest of the alternative can complete token-free (the
		// optional is effectively last, as in pred=predicate?).
		ATNState ruleStop = null;
		for (ATNState st : atn.states) {
			if (st != null && st.ruleIndex == ds.ruleIndex
				&& st instanceof org.antlr.v4.runtime.atn.RuleStopState) {
				ruleStop = st;
				break;
			}
		}
		org.antlr.v4.runtime.atn.LL1Analyzer ll1 = new org.antlr.v4.runtime.atn.LL1Analyzer(atn);
		org.antlr.v4.runtime.misc.IntervalSet contFirst =
			ll1.LOOK(block.endState, ruleStop, null);
		if (!contFirst.contains(org.antlr.v4.runtime.Token.EPSILON)) {
			org.antlr.v4.runtime.misc.IntervalSet takeFirst =
				ll1.LOOK(ds.transition(0).target, block.endState, null);
			takeFirst.remove(org.antlr.v4.runtime.Token.EPSILON);
			if (contFirst.and(takeFirst).size() > 0) return null;
		}

		return new Shape(ds.decision, block.endState.stateNumber, postfixRule, compound);
	}

	private boolean epsilonPathTo(ATNState from, ATNState to, Set<Integer> visited) {
		if (from == to) return true;
		if (!visited.add(from.stateNumber)) return false;
		for (int i = 0; i < from.getNumberOfTransitions(); i++) {
			Transition t = from.transition(i);
			if (t.isEpsilon() && epsilonPathTo(t.target, to, visited)) return true;
		}
		return false;
	}

	/** First state with a non-epsilon outgoing transition reachable by
	 *  epsilon links; null when none is found (empty alternative).
	 *  NB: RuleTransition.isEpsilon() is true - it must be treated as an
	 *  element, not an epsilon link to follow. */
	private ATNState firstElement(ATNState start) {
		Deque<ATNState> work = new ArrayDeque<ATNState>();
		Set<Integer> visited = new HashSet<Integer>();
		work.add(start);
		while (!work.isEmpty()) {
			ATNState s = work.remove();
			if (!visited.add(s.stateNumber)) continue;
			for (int i = 0; i < s.getNumberOfTransitions(); i++) {
				Transition t = s.transition(i);
				if (t instanceof RuleTransition) return s;
				if (!t.isEpsilon()) return s;
				work.add(t.target);
			}
		}
		return null;
	}
}
