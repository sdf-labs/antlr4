/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.analysis;

import org.antlr.v4.runtime.Token;
import org.antlr.v4.runtime.atn.ATN;
import org.antlr.v4.runtime.atn.ATNState;
import org.antlr.v4.runtime.atn.ActionTransition;
import org.antlr.v4.runtime.atn.BlockStartState;
import org.antlr.v4.runtime.atn.DecisionState;
import org.antlr.v4.runtime.atn.LL1Analyzer;
import org.antlr.v4.runtime.atn.PrecedencePredicateTransition;
import org.antlr.v4.runtime.atn.PredicateTransition;
import org.antlr.v4.runtime.atn.RuleTransition;
import org.antlr.v4.runtime.atn.Transition;
import org.antlr.v4.runtime.misc.IntervalSet;
import org.antlr.v4.tool.Grammar;

import java.util.ArrayList;
import java.util.BitSet;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

/**
 * Detects <em>prefix-factorable</em> decisions: block decisions whose
 * alternatives partition into groups sharing an identical leading element
 * sequence (token matches and rule invocations) that ends at a rule
 * invocation, with pairwise LL(1)-disjoint tails. For such a group the
 * static prediction walker may accept with an <em>alternative mask</em>
 * (the still-live alternatives) instead of a unique alternative or an
 * adaptive escape: the generated parser executes the shared prefix once
 * and defers the choice to a trivial tail decision ("implied
 * left-factoring").
 *
 * <p>The canonical shape is the parenthesized family of an expression
 * rule: {@code '(' expression (',' expression)+ ')'} (row constructor)
 * vs {@code '(' expression ')'}. Both share {@code '(' expression};
 * after the shared invocation returns, the tails begin with ',' and ')'
 * respectively - disjoint, so the tail is decidable with k=1. The
 * unbounded expression descent in between is what defeats unique-alt
 * static prediction; the mask protocol accepts the decision at the
 * shallow depth where the live set narrows to the group.</p>
 *
 * <p>Soundness: the walker only ever removes alternatives whose SLL
 * configurations all died on the scanned prefix, and SLL viability
 * over-approximates full-context viability - so the returned mask is
 * always a superset of the runtime's live alternatives, and every group
 * member genuinely begins with the shared prefix (element-level
 * equality proven here).</p>
 *
 * <p>v1 restrictions: shared prefixes end at a rule invocation and
 * contain no predicates/actions; set transitions and sub-blocks end the
 * prefix; tails must be non-nullable with pairwise-disjoint local FIRST
 * sets; decisions with more than 64 alternatives are excluded (the mask
 * is a u64).</p>
 */
public class PrefixFactorAnalyzer {

	/** A prefix element: a token match or a rule invocation. */
	public static final class Element {
		public static final int TOKEN = 1;
		public static final int RULE = 2;
		public final int kind;
		/** Token type for {@link #TOKEN}, rule index for {@link #RULE}. */
		public final int id;

		public Element(int kind, int id) { this.kind = kind; this.id = id; }

		@Override
		public boolean equals(Object o) {
			return o instanceof Element && ((Element)o).kind == kind && ((Element)o).id == id;
		}

		@Override
		public int hashCode() { return kind * 1000003 + id; }

		public String toString(Grammar g) {
			if (kind == TOKEN) {
				String name = g.getTokenDisplayName(id);
				return name != null ? name : String.valueOf(id);
			}
			return "<" + g.getRule(id).name + ">";
		}
	}

	/** One prefix group: alternatives sharing {@link #prefix}, with
	 *  LL(1)-disjoint tails starting at {@link #tailStartState}. */
	public static final class Group {
		public final BitSet alts = new BitSet();
		public final List<Element> prefix;
		/** Per-alternative ATN state where the tail begins (right after
		 *  the shared rule invocation's follow state). */
		public final Map<Integer, Integer> tailStartState = new HashMap<Integer, Integer>();
		/** Per-alternative local tail FIRST (block-end delimited). */
		public final Map<Integer, IntervalSet> tailFirst = new HashMap<Integer, IntervalSet>();
		/**
		 * Synthetic call-site state of the group's shared rule
		 * invocation, created in the ATN by
		 * {@link DecisionClassifier#buildTables}: a BasicState with a
		 * single RuleTransition to the shared rule whose follow state is
		 * {@link #syntheticTailState}. The generated factored arm sets
		 * the call-site state to this state, so nested adaptive
		 * simulations inside the shared invocation pop into a position
		 * where every member's tail is genuinely viable.
		 */
		public int syntheticInvokeState = -1;
		/**
		 * Synthetic tail-decision state: a BasicState with an epsilon
		 * transition to every member's {@link #tailStartState}.
		 */
		public int syntheticTailState = -1;

		public Group(List<Element> prefix) { this.prefix = prefix; }

		public String toString(Grammar g) {
			StringBuilder sb = new StringBuilder();
			sb.append(alts).append(" prefix=");
			for (Element e : prefix) sb.append(e.toString(g)).append(' ');
			sb.append("tails=");
			for (Map.Entry<Integer, IntervalSet> e : tailFirst.entrySet()) {
				sb.append(e.getKey()).append(':')
					.append(e.getValue().toString(g.getVocabulary())).append(' ');
			}
			return sb.toString().trim();
		}
	}

	/** The factor plan of one decision: its validated groups. */
	public static final class Plan {
		public final int decision;
		public final List<Group> groups = new ArrayList<Group>();

		public Plan(int decision) { this.decision = decision; }

		public boolean hasGroups() { return !groups.isEmpty(); }

		/** The single group covering the given live-alternative set, or
		 *  null when the live set spans groups or outside alternatives.
		 *  A live set of cardinality &lt; 2 needs no mask (unique-alt
		 *  acceptance handles it). */
		public Group groupCovering(BitSet liveAlts) {
			if (liveAlts.cardinality() < 2) return null;
			Group found = null;
			for (Group g : groups) {
				BitSet b = (BitSet)liveAlts.clone();
				b.and(g.alts);
				if (b.equals(liveAlts)) {
					if (found != null) return null; // covered by two groups: reject
					found = g;
				}
			}
			return found;
		}
	}

	/** Per-alternative extraction result: the leading element sequence,
	 *  the state following each element, and where actions or
	 *  predicates were seen along the way. */
	private static final class AltPath {
		final List<Element> elements = new ArrayList<Element>();
		/** stateAfter[i] = state number after elements[i] (for a RULE
		 *  element, the rule transition's follow state). */
		final List<Integer> stateAfter = new ArrayList<Integer>();
		/** Element depth of the first action seen (elements.size() at
		 *  that point); actions at depth &gt;= a group's shared prefix
		 *  length are inside tails and benign. Integer.MAX_VALUE = none. */
		int firstActionDepth = Integer.MAX_VALUE;
		/** Any predicate seen anywhere on the path (tails included:
		 *  the tail-FIRST proof cannot see through predicates). */
		boolean sawPredicate;
	}

	private final Grammar g;
	private final ATN atn;
	private final LL1Analyzer ll1;

	public PrefixFactorAnalyzer(Grammar g, ATN atn) {
		this.g = g;
		this.atn = atn;
		this.ll1 = new LL1Analyzer(atn);
	}

	/** Analyze a decision; returns a plan (possibly without groups).
	 *  Never returns null. */
	public Plan analyze(DecisionState ds) {
		Plan plan = new Plan(ds.decision);
		if (!(ds instanceof BlockStartState)) return plan;
		int nAlts = ds.getNumberOfTransitions();
		if (nAlts < 2 || nAlts > 64) return plan;

		BlockStartState block = (BlockStartState)ds;
		List<AltPath> paths = new ArrayList<AltPath>();
		boolean dbg = "factor".equals(System.getProperty("antlr.dfa.debug"));
		for (int i = 0; i < nAlts; i++) {
			AltPath p = extract(ds.transition(i).target);
			paths.add(p);
			if (dbg) {
				StringBuilder sb = new StringBuilder();
				sb.append("  EXTRACT d=").append(ds.decision).append(" alt=").append(i+1).append(":");
				for (Element e : p.elements) sb.append(' ').append(e.toString(g));
				sb.append(" (actionDepth=").append(p.firstActionDepth)
					.append(" sawPred=").append(p.sawPredicate).append(')');
				System.err.println(sb);
			}
		}

		// Candidate groups: maximal clusters of alternatives sharing a
		// prefix of length >= 2 that ends at a rule invocation. Computed
		// by descending shared-prefix length; an alternative joins at
		// most one group (the one at its deepest shared node).
		BitSet grouped = new BitSet();
		int maxLen = 0;
		for (AltPath p : paths) maxLen = Math.max(maxLen, p.elements.size());
		for (int len = maxLen; len >= 2; len--) {
			Map<List<Element>, List<Integer>> byKey = new HashMap<List<Element>, List<Integer>>();
			for (int alt = 1; alt <= nAlts; alt++) {
				if (grouped.get(alt)) continue;
				AltPath p = paths.get(alt-1);
				if (p.elements.size() < len) continue;
				if (p.elements.get(len-1).kind != Element.RULE) continue;
				List<Element> key = new ArrayList<Element>(p.elements.subList(0, len));
				byKey.computeIfAbsent(key, k -> new ArrayList<Integer>()).add(alt);
			}
			for (List<Integer> members : byKey.values()) {
				if (members.size() < 2) continue;
				Group grp = buildGroup(paths, members, len, block);
				if (grp != null) {
					plan.groups.add(grp);
					for (int alt : members) grouped.set(alt);
				}
			}
		}
		return plan;
	}

	/** Validate a candidate group and populate it; null when invalid. */
	private Group buildGroup(List<AltPath> paths, List<Integer> members, int len,
							 BlockStartState block) {
		AltPath first = paths.get(members.get(0)-1);
		// No actions inside the shared region; no predicates anywhere
		// (the tail-FIRST proof cannot see through them).
		for (int alt : members) {
			AltPath p = paths.get(alt-1);
			if (p.firstActionDepth < len || p.sawPredicate) return null;
		}
		Group grp = new Group(new ArrayList<Element>(first.elements.subList(0, len)));
		for (int alt : members) {
			AltPath p = paths.get(alt-1);
			grp.alts.set(alt);
			grp.tailStartState.put(alt, p.stateAfter.get(len-1));
		}
		// Tails: local FIRST delimited by the decision's block end; must
		// be non-nullable, non-empty, and pairwise disjoint.
		IntervalSet union = new IntervalSet();
		for (int alt : members) {
			ATNState tailStart = atn.states.get(grp.tailStartState.get(alt));
			IntervalSet f = ll1.LOOK(tailStart, block.endState, null);
			if (f.isNil() || f.contains(Token.EPSILON)) return null;
			grp.tailFirst.put(alt, f);
			if (union.and(f).size() > 0) return null;
			union.addAll(f);
		}
		return grp;
	}

	/** Extract the leading element sequence of one alternative: walk
	 *  from its first state, recording token matches and rule
	 *  invocations, following single-epsilon links, and stopping at any
	 *  fanout (sub-block), set transition, loop back-edge, or mixed
	 *  state. */
	private AltPath extract(ATNState start) {
		AltPath path = new AltPath();
		Set<Integer> visited = new HashSet<Integer>();
		ATNState cur = start;
		while (cur != null && visited.add(cur.stateNumber)) {
			int n = cur.getNumberOfTransitions();
			if (n == 0) break;
			if (n == 1) {
				Transition t = cur.transition(0);
				if (t instanceof ActionTransition) {
					path.firstActionDepth = Math.min(path.firstActionDepth, path.elements.size());
					cur = t.target;
					continue;
				}
				if (t instanceof PredicateTransition
					|| t instanceof PrecedencePredicateTransition) {
					path.sawPredicate = true;
					cur = t.target;
					continue;
				}
				// NB: RuleTransition.isEpsilon() is true - it must be
				// handled before the generic epsilon follow.
				if (t instanceof RuleTransition) {
					RuleTransition rt = (RuleTransition)t;
					path.elements.add(new Element(Element.RULE, rt.ruleIndex));
					path.stateAfter.add(rt.followState.stateNumber);
					cur = rt.followState;
					continue;
				}
				if (t.isEpsilon()) { cur = t.target; continue; }
				if (t instanceof org.antlr.v4.runtime.atn.AtomTransition) {
					path.elements.add(new Element(Element.TOKEN,
						((org.antlr.v4.runtime.atn.AtomTransition)t).label));
					path.stateAfter.add(t.target.stateNumber);
					cur = t.target;
					continue;
				}
				break; // set/wildcard transition: prefix ends
			}
			// Fanout: v1 stops the prefix here. Still note predicates
			// (they end prefix extraction but poison the tail proof if
			// they fall inside a would-be tail).
			for (int i = 0; i < n; i++) {
				Transition t = cur.transition(i);
				if (t instanceof ActionTransition) {
					path.firstActionDepth = Math.min(path.firstActionDepth, path.elements.size());
				}
				if (t instanceof PredicateTransition
					|| t instanceof PrecedencePredicateTransition) {
					path.sawPredicate = true;
				}
			}
			break;
		}
		return path;
	}
}
