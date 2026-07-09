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
 * A plus loop whose continue/exit decision is driven by a statically-
 * precomputed SLL prediction table via the runtime's dfaPredict walker
 * instead of adaptivePredict. Structurally identical to {@link PlusBlock}
 * (subclassing also keeps SourceGenTriggers' hasLookaheadBlock/_alt
 * declaration logic working).
 */
public class DFAPlusBlock extends PlusBlock {
	public DFAPlusBlock(OutputModelFactory factory,
						GrammarAST plusRoot,
						List<CodeBlockForAlt> alts)
	{
		super(factory, plusRoot, alts);
	}
}
