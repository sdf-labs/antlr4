/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.analysis.StaticLexerDFA;
import org.antlr.v4.runtime.misc.IntegerList;

import java.util.List;

/**
 * All statically-precomputed lexer DFA tables of a lexer grammar (see
 * {@link StaticLexerDFA} and {@code LexerDFABuilder}), packed into the same
 * compact int stream used by {@link SerializedStaticDFAs} and appended
 * after the ATN ints in the base64 blob (see {@link SerializedBase64ATN}),
 * so the target runtime decodes both in one streaming pass. The runtime's
 * table walker tokenizes straight off these tables - no ATN simulation, no
 * lazily-built DFA.
 *
 * <p>The logical int stream (encoded with {@code CompactSerializer}):</p>
 *
 * <pre>
 * FORMAT_VERSION
 * numModes
 * for each mode:
 *   numStates
 *   startState
 *   acceptType[numStates]       (token type per accept state; -1 = non-accept)
 *   acceptActions[numStates]    (offset into actionLists; -1 = no actions)
 *   eof[numStates]              (EOF target; -1 = no edge)
 *   ascii[numStates*128]        (dense target rows for symbols 0..127; -1 = no edge)
 *   hiOffsets[numStates+1]      (index of each state's first triple in hiEdges)
 *   numHiEdgeInts
 *   hiEdges[numHiEdgeInts]      ((lo, hi, target) triples, lo-sorted per state, lo &gt; 127)
 *   numActionListInts
 *   actionLists[...]            (length-prefixed lists of lexer action indices)
 * </pre>
 */
public final class SerializedStaticLexerDFAs {
	public static final int FORMAT_VERSION = 3; // v3: -1 (not 0) marks non-accept states

	private SerializedStaticLexerDFAs() { }

	/**
	 * Append the logical int stream for {@code dfas} to {@code data} and one
	 * provenance line per mode to {@code comments}.
	 */
	public static void appendIntStream(StaticLexerDFA[] dfas, IntegerList data, List<String> comments) {
		data.add(FORMAT_VERSION);
		data.add(dfas.length);
		for (StaticLexerDFA dfa : dfas) {
			data.add(dfa.numStates);
			data.add(dfa.startState);
			for (int v : dfa.acceptType) data.add(v);
			for (int v : dfa.acceptActions) data.add(v);
			for (int v : dfa.eof) data.add(v);
			for (int v : dfa.ascii) data.add(v);
			for (int v : dfa.hiOffsets) data.add(v);
			data.add(dfa.hiEdges.length);
			for (int v : dfa.hiEdges) data.add(v);
			data.add(dfa.actionLists.length);
			for (int v : dfa.actionLists) data.add(v);
			if (comments != null) {
				int accepts = 0;
				for (int v : dfa.acceptType) if (v >= 0) accepts++;
				comments.add("mode "+dfa.mode+": "+dfa.numStates+" states, "
					+accepts+" accepts, "+(dfa.hiEdges.length/3)+" hi edges");
			}
		}
	}
}
