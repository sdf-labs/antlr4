/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.codegen.OutputModelFactory;

/**
 * One tail arm of a {@link FactoredGroup}: the arm key (token-type
 * constants of the member alternative's tail FIRST set, joined for a
 * Rust match pattern) and the tail body (the member's label-context
 * specialization action, if any, followed by its tail element ops).
 */
public class FactoredTailArm extends OutputModelObject {
	/** Match-arm pattern: token-type constants joined with " | ". */
	public final String armKey;
	@ModelElement public final CodeBlockForAlt body;

	public FactoredTailArm(OutputModelFactory factory, String armKey, CodeBlockForAlt body) {
		super(factory);
		this.armKey = armKey;
		this.body = body;
	}
}
