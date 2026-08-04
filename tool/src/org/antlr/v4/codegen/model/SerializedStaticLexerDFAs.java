/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.analysis.StaticLexerDFA;
import org.antlr.v4.runtime.atn.LexerAction;
import org.antlr.v4.runtime.atn.LexerChannelAction;
import org.antlr.v4.runtime.atn.LexerCustomAction;
import org.antlr.v4.runtime.atn.LexerModeAction;
import org.antlr.v4.runtime.atn.LexerPushModeAction;
import org.antlr.v4.runtime.atn.LexerTypeAction;
import org.antlr.v4.runtime.misc.IntegerList;
import java.util.List;

/**
 * All statically-precomputed lexer DFA tables of a lexer grammar (see
 * {@link StaticLexerDFA} and {@code LexerDFABuilder}), packed into the same
 * compact int stream used by {@link SerializedStaticDFAs}. When the lexer is
 * fully static (the whole-grammar gate in {@code LexerDFABuilder}), the
 * generated code embeds ONLY this stream - no ATN at all - via
 * {@link #appendStandaloneIntStream} (which appends the lexer's
 * position-independent lexer actions, the only ATN data the table walker
 * still needs); otherwise the tables follow the ATN ints in the base64
 * blob via {@link #appendIntStream} (see {@link SerializedBase64ATN}).
 * Either way the target runtime decodes everything in one streaming pass.
 * The runtime's table walker tokenizes straight off these tables - no ATN
 * simulation, no lazily-built DFA.
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
	/** Standalone (ATN-less) stream: the v3 mode tables followed by the lexer actions. */
	public static final int TABLES_FORMAT_VERSION = 4;

	private SerializedStaticLexerDFAs() { }

	/**
	 * Append the logical int stream for {@code dfas} to {@code data} and one
	 * provenance line per mode to {@code comments}.
	 */
	public static void appendIntStream(StaticLexerDFA[] dfas, IntegerList data, List<String> comments) {
		data.add(FORMAT_VERSION);
		appendModeTables(dfas, data, comments);
	}

	/**
	 * Append the standalone (ATN-less) stream: {@link #TABLES_FORMAT_VERSION},
	 * the same mode-table section as {@link #appendIntStream}, then the lexer
	 * actions the tables' accept-action indices reference - serialized with
	 * the same (type, data1, data2) encoding as the ATN's lexer-action
	 * section (see {@code ATNSerializer}), so the runtime decodes them with
	 * the same code. Used when the lexer is fully static and its generated
	 * code embeds no ATN at all.
	 */
	public static void appendStandaloneIntStream(StaticLexerDFA[] dfas, LexerAction[] lexerActions, IntegerList data, List<String> comments) {
		data.add(TABLES_FORMAT_VERSION);
		appendModeTables(dfas, data, comments);
		data.add(lexerActions.length);
		for (LexerAction action : lexerActions) {
			data.add(action.getActionType().ordinal());
			switch (action.getActionType()) {
			case CHANNEL:
				data.add(((LexerChannelAction)action).getChannel());
				data.add(0);
				break;
			case CUSTOM:
				data.add(((LexerCustomAction)action).getRuleIndex());
				data.add(((LexerCustomAction)action).getActionIndex());
				break;
			case MODE:
				data.add(((LexerModeAction)action).getMode());
				data.add(0);
				break;
			case PUSH_MODE:
				data.add(((LexerPushModeAction)action).getMode());
				data.add(0);
				break;
			case TYPE:
				data.add(((LexerTypeAction)action).getType());
				data.add(0);
				break;
			default: // MORE, POP_MODE, SKIP: no arguments
				data.add(0);
				data.add(0);
				break;
			}
		}
	}

	private static void appendModeTables(StaticLexerDFA[] dfas, IntegerList data, List<String> comments) {
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
