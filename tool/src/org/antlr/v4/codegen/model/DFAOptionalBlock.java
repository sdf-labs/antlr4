/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.codegen.OutputModelFactory;
import org.antlr.v4.tool.ast.GrammarAST;

import java.util.List;

/**
 * An optional block whose decision is driven by a statically-precomputed
 * SLL prediction table via the runtime's dfaPredict walker instead of
 * adaptivePredict. Structurally identical to {@link OptionalBlock}.
 */
public class DFAOptionalBlock extends OptionalBlock {
	public DFAOptionalBlock(OutputModelFactory factory,
							GrammarAST questionAST,
							List<CodeBlockForAlt> alts)
	{
		super(factory, questionAST, alts);
	}
}
