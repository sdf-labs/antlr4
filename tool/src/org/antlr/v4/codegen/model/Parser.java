/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.codegen.OutputModelFactory;
import org.antlr.v4.tool.Grammar;

import java.util.ArrayList;
import java.util.List;

public class Parser extends Recognizer {
	public ParserFile file;

	@ModelElement public List<RuleFunction> funcs = new ArrayList<RuleFunction>();

	/**
	 * All statically-precomputed SLL prediction tables (-Xstatic-dfa),
	 * packed into one compact serialized blob deserialized by the target
	 * runtime (in the same vein as the serialized ATN); null unless tables
	 * exist and the target supports static DFA prediction. Targets using
	 * the compact base64 ATN encoding (Rust) carry the tables inside the
	 * ATN blob instead (see {@link SerializedBase64ATN}) and leave this
	 * null. Targets whose Parser template lacks the staticDFAs formal
	 * argument are unaffected (the model walker skips unknown fields).
	 */
	@ModelElement public SerializedStaticDFAs staticDFAs;

	public Parser(OutputModelFactory factory, ParserFile file) {
		super(factory);
		this.file = file; // who contains us?
		Grammar g = factory.getGrammar();
		boolean haveTables =
			(g.staticDecisionDFAs!=null && !g.staticDecisionDFAs.isEmpty())
			|| (g.staticPrecedenceDFAs!=null && !g.staticPrecedenceDFAs.isEmpty());
		if ( haveTables
			 && factory.getGenerator().getTarget().supportsStaticDFA()
			 && !factory.getGenerator().getTarget().isATNSerializedAsBase64VarInts() ) {
			staticDFAs = new SerializedStaticDFAs(factory, g.staticDecisionDFAs, g.staticPrecedenceDFAs);
		}
	}
}
