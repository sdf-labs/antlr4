/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.analysis.StaticDFA;
import org.antlr.v4.codegen.CompactSerializer;
import org.antlr.v4.codegen.OutputModelFactory;
import org.antlr.v4.runtime.misc.IntegerList;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

/**
 * All statically-precomputed SLL prediction tables of a parser
 * (see {@link StaticDFA}), packed into one compact serialized blob -
 * in the same vein as {@code _serializedATN} - and lazily deserialized by
 * the target runtime's {@code StaticDFATables} at first use.
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
 *   decision
 *   numStates
 *   numEdgeInts
 *   accepts[numStates]              (predicted alt per state; 0 = non-accept)
 *   fallbacks[numStates]            (error-avoidance alt per state; 0 = none)
 *   edgeOffsets[numStates+1]        (index of each state's first edge int)
 *   edges[numEdgeInts]              ((lo, hi, target) triples, lo-sorted per state)
 * </pre>
 */
public class SerializedStaticDFAs extends OutputModelObject {
	public static final int FORMAT_VERSION = 3;

	public final int numTables;
	/** Base64 text segments of the serialized blob, one rendered per line. */
	public final List<String> segments;
	/** One human-readable provenance line per table, for a generated comment. */
	public final List<String> tableComments = new ArrayList<String>();

	public SerializedStaticDFAs(OutputModelFactory factory, Map<Integer, StaticDFA> dfas) {
		super(factory);
		this.numTables = dfas.size();
		IntegerList data = new IntegerList();
		appendIntStream(dfas, data, tableComments);
		segments = CompactSerializer.encode(data.toArray());
	}

	/**
	 * Append the logical int stream for the tables to {@code data} and one
	 * provenance line per table to {@code comments}. Shared with
	 * {@link SerializedBase64ATN}, which appends the stream after the ATN
	 * ints so both are decoded from a single blob.
	 */
	public static void appendIntStream(Map<Integer, StaticDFA> dfas,
									   IntegerList data,
									   List<String> comments) {
		int numDecisionSlots = 0;
		for (StaticDFA dfa : dfas.values()) {
			numDecisionSlots = Math.max(numDecisionSlots, dfa.decision+1);
		}

		data.add(FORMAT_VERSION);
		data.add(dfas.size());
		data.add(numDecisionSlots);
		for (StaticDFA dfa : dfas.values()) {
			data.add(dfa.decision);
			data.add(dfa.numStates);
			data.add(dfa.edges.length);
			for (int v : dfa.accepts) data.add(v);
			for (int v : dfa.fallbacks) data.add(v);
			for (int v : dfa.edgeOffsets) data.add(v);
			for (int v : dfa.edges) data.add(v);
			comments.add("decision "+dfa.decision+": "
				+(dfa.cyclic ? "LL(*) cyclic" : "LL(k), k="+dfa.maxK)
				+", "+dfa.numStates+" states");
		}
	}
}
