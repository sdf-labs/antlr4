/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.analysis.PrecedenceStaticDFA;
import org.antlr.v4.analysis.StaticDFA;
import org.antlr.v4.codegen.CompactSerializer;
import org.antlr.v4.codegen.OutputModelFactory;
import org.antlr.v4.runtime.misc.IntegerList;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;

/**
 * All statically-precomputed SLL prediction tables of a parser
 * (see {@link StaticDFA} and {@link PrecedenceStaticDFA}), packed into one
 * compact serialized blob - in the same vein as {@code _serializedATN} - and
 * lazily deserialized by the target runtime's {@code StaticDFATables} at
 * first use.
 *
 * <p>The logical int stream below is encoded with
 * {@link CompactSerializer} (zigzag LEB128 varints, base64, 80-column line
 * segments; runtimes concatenate all segments before decoding):</p>
 *
 * <pre>
 * FORMAT_VERSION
 * numTables
 * numDecisionSlots                  (max decision number + 1)
 * for each table:
 *   decision                        (-1: referenced only via a dispatch below)
 *   numStates
 *   numEdgeInts
 *   accepts[numStates]              (predicted alt per state; 0 = non-accept;
 *                                    -1 = escape: defer to adaptivePredict)
 *   fallbacks[numStates]            (error-avoidance alt per state; 0 = none)
 *   edgeOffsets[numStates+1]        (index of each state's first edge int)
 *   edges[numEdgeInts]              ((lo, hi, target) triples, lo-sorted per state)
 *   numMaskStates                   (states accepting with an alternative
 *                                    mask instead of a unique alt: the live
 *                                    set is covered by one prefix-factor
 *                                    group; the parser executes the shared
 *                                    prefix and resolves at the tail decision)
 *   for each mask state:            (sorted by state index)
 *     state
 *     numAlts
 *     alts[numAlts]                 (the group's member alternatives)
 * numPrecedenceDispatches
 * for each dispatch:                (a left-recursive precedence loop decision)
 *   decision
 *   numCutoffs
 *   cutoffs[numCutoffs]             (sorted; class(p) = #cutoffs &lt; p)
 *   tableIndex[numCutoffs+1]        (table of each precedence class;
 *                                    -1 = class dispatches to adaptivePredict)
 * </pre>
 *
 * <p>Identical class tables are deduplicated by content and shared through
 * their table index.</p>
 */
public class SerializedStaticDFAs extends OutputModelObject {
	/** v6: per-table alternative-mask section (prefix-factor groups). */
	public static final int FORMAT_VERSION = 6;

	public final int numTables;
	/** Base64 text segments of the serialized blob, one rendered per line. */
	public final List<String> segments;
	/** One human-readable provenance line per table, for a generated comment. */
	public final List<String> tableComments = new ArrayList<String>();

	public SerializedStaticDFAs(OutputModelFactory factory,
								Map<Integer, StaticDFA> dfas,
								Map<Integer, PrecedenceStaticDFA> precDfas) {
		super(factory);
		IntegerList data = new IntegerList();
		this.numTables = appendIntStream(dfas, precDfas, data, tableComments);
		segments = CompactSerializer.encode(data.toArray());
	}

	/**
	 * Append the logical int stream for the tables to {@code data} and one
	 * provenance line per table/dispatch to {@code comments}; returns the
	 * number of tables written. Shared with {@link SerializedBase64ATN},
	 * which appends the stream after the ATN ints so both are decoded from
	 * a single blob.
	 */
	public static int appendIntStream(Map<Integer, StaticDFA> dfas,
									  Map<Integer, PrecedenceStaticDFA> precDfas,
									  IntegerList data,
									  List<String> comments) {
		List<StaticDFA> tables = new ArrayList<StaticDFA>();
		List<Integer> tableDecisions = new ArrayList<Integer>();
		int numDecisionSlots = 0;

		if (dfas != null) {
			for (StaticDFA dfa : dfas.values()) {
				tables.add(dfa);
				tableDecisions.add(dfa.decision);
				numDecisionSlots = Math.max(numDecisionSlots, dfa.decision+1);
			}
		}

		// dispatches reference class tables by index, deduplicated by content
		List<int[]> dispatches = new ArrayList<int[]>();
		if (precDfas != null) {
			for (PrecedenceStaticDFA group : precDfas.values()) {
				numDecisionSlots = Math.max(numDecisionSlots, group.decision+1);
				int[] entry = new int[2 + group.cutoffs.length + group.tables.length];
				entry[0] = group.decision;
				entry[1] = group.cutoffs.length;
				System.arraycopy(group.cutoffs, 0, entry, 2, group.cutoffs.length);
				StringBuilder comment = new StringBuilder("decision "+group.decision
					+": precedence-dispatched over cutoffs "+Arrays.toString(group.cutoffs)
					+", tables");
				for (int c = 0; c < group.tables.length; c++) {
					StaticDFA t = group.tables[c];
					int idx;
					if (t == null) {
						idx = -1; // class dispatches to adaptivePredict
					}
					else {
						idx = indexOf(tables, t);
						if (idx < 0) {
							idx = tables.size();
							tables.add(t);
							tableDecisions.add(-1);
						}
					}
					entry[2 + group.cutoffs.length + c] = idx;
					comment.append(c == 0 ? " [" : " ").append(idx);
				}
				comment.append(']');
				dispatches.add(entry);
				comments.add(comment.toString());
			}
		}

		data.add(FORMAT_VERSION);
		data.add(tables.size());
		data.add(numDecisionSlots);
		for (int i = 0; i < tables.size(); i++) {
			StaticDFA dfa = tables.get(i);
			data.add(tableDecisions.get(i));
			data.add(dfa.numStates);
			data.add(dfa.edges.length);
			for (int v : dfa.accepts) data.add(v);
			for (int v : dfa.fallbacks) data.add(v);
			for (int v : dfa.edgeOffsets) data.add(v);
			for (int v : dfa.edges) data.add(v);
			// alternative-mask section (v6)
			int numMaskStates = 0;
			for (long m : dfa.acceptMasks) {
				if (m != 0) numMaskStates++;
			}
			data.add(numMaskStates);
			if (numMaskStates > 0) {
				for (int s = 0; s < dfa.numStates; s++) {
					long m = dfa.acceptMasks[s];
					if (m == 0) continue;
					data.add(s);
					data.add(Long.bitCount(m));
					for (int a = 0; a < 64; a++) {
						if ((m & (1L << a)) != 0) data.add(a+1);
					}
				}
			}
			int escapes = 0;
			for (int v : dfa.accepts) {
				if (v == StaticDFA.ESCAPE) escapes++;
			}
			comments.add((tableDecisions.get(i) >= 0
					? "decision "+dfa.decision : "table "+i+" (decision "+dfa.decision+")")
				+": "+(dfa.cyclic ? "LL(*) cyclic" : "LL(k), k="+dfa.maxK)
				+", "+dfa.numStates+" states"
				+(escapes > 0 ? ", "+escapes+" adaptive escapes" : "")
				+(numMaskStates > 0 ? ", "+numMaskStates+" mask accepts" : ""));
		}
		data.add(dispatches.size());
		for (int[] entry : dispatches) {
			for (int v : entry) data.add(v);
		}
		return tables.size();
	}

	/** Index of a table with identical content, or -1. */
	private static int indexOf(List<StaticDFA> tables, StaticDFA t) {
		for (int i = 0; i < tables.size(); i++) {
			StaticDFA o = tables.get(i);
			if (o == t || (Arrays.equals(o.accepts, t.accepts)
				&& Arrays.equals(o.acceptMasks, t.acceptMasks)
				&& Arrays.equals(o.fallbacks, t.fallbacks)
				&& Arrays.equals(o.edgeOffsets, t.edgeOffsets)
				&& Arrays.equals(o.edges, t.edges))) {
				return i;
			}
		}
		return -1;
	}
}
