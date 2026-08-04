/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.codegen.CompactSerializer;
import org.antlr.v4.codegen.OutputModelFactory;
import org.antlr.v4.runtime.atn.ATN;
import org.antlr.v4.runtime.misc.IntegerList;
import org.antlr.v4.tool.Grammar;

import java.util.ArrayList;
import java.util.List;

/**
 * A serialized ATN encoded with {@link CompactSerializer} (zigzag LEB128
 * varints, base64, 80-column lines) for targets whose runtime deserializes
 * the int stream itself (currently Rust; see {@code serialized_ints} and
 * {@code ATNDeserializer::deserialize_compact} in the Rust runtime).
 * Roughly 2-3x smaller in generated-source bytes than the plain decimal int
 * list, and far fewer tokens for the target compiler to parse.
 *
 * <p>When the grammar has statically-precomputed SLL prediction tables
 * ({@code -Xstatic-dfa}), their serialized stream (see
 * {@link SerializedStaticDFAs}) is appended after the ATN ints so the
 * runtime decodes both from the single blob in one streaming pass; the
 * tables land in the ATN's {@code static_dfas} field. For a lexer the
 * tool proved fully static ({@code staticLexerDFAs}), the ATN is dropped
 * instead: the blob holds only the standalone tables stream (see
 * {@link SerializedStaticLexerDFAs#appendStandaloneIntStream}) and the
 * template emits ATN-less runtime scaffolding ({@link #omitATN}).</p>
 */
public class SerializedBase64ATN extends SerializedATN {
	public final List<String> segments;
	/** One provenance line per appended static DFA table; empty if none. */
	public final List<String> tableComments = new ArrayList<String>();
	public final int numTables;
	/**
	 * True when the blob holds only static lexer tables, no ATN: the lexer is
	 * fully table-driven ({@code -Xstatic-dfa}), so its generated code needs
	 * no ATN at all. Templates branch on this to emit the ATN-less runtime
	 * scaffolding ({@code StaticLexerTables} instead of an {@code ATN}).
	 */
	public final boolean omitATN;
	public SerializedBase64ATN(OutputModelFactory factory, ATN atn) {
		super(factory, atn);
		IntegerList data = new IntegerList();

		Grammar g = factory.getGrammar();
		boolean haveTables =
			(g.staticDecisionDFAs!=null && !g.staticDecisionDFAs.isEmpty())
			|| (g.staticPrecedenceDFAs!=null && !g.staticPrecedenceDFAs.isEmpty());
		int n = 0;
		boolean omit = false;
		if ( haveTables && g.atn==atn
			 && factory.getGenerator().getTarget().supportsStaticDFA() ) {
			for (int v : serialized) data.add(v);
			n = SerializedStaticDFAs.appendIntStream(

				g.staticDecisionDFAs, g.staticPrecedenceDFAs, data, tableComments);
		}
		else if ( g.isLexer() && g.staticLexerDFAs!=null && g.atn==atn
			 && factory.getGenerator().getTarget().supportsStaticDFA() ) {
			// Fully-static lexer: the ATN is unused at runtime (the table
			// walker plus the serialized position-independent lexer actions
			// cover everything), so the blob holds only the tables.
			SerializedStaticLexerDFAs.appendStandaloneIntStream(g.staticLexerDFAs, g.atn.lexerActions, data, tableComments);
			n = g.staticLexerDFAs.length; // mode tables, for the provenance comment
			omit = true;
		}
		else {
			for (int v : serialized) data.add(v);
		}
		numTables = n;
		omitATN = omit;

		segments = CompactSerializer.encode(data.toArray());
	}
}
