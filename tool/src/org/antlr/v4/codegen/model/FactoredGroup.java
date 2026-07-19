/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.codegen.OutputModelFactory;

import java.util.ArrayList;
import java.util.List;

/**
 * One prefix-factor group of a {@link FactoredAltBlock}: the shared
 * prefix (executed once, committing to no group member) and one tail
 * arm per member alternative, keyed by the tail's FIRST tokens.
 */
public class FactoredGroup extends OutputModelObject {
	/** Match-arm key of the group's alternative mask (hex literal). */
	public final String maskKey;
	@ModelElement public final CodeBlockForAlt prefix;
	@ModelElement public final List<FactoredTailArm> tails = new ArrayList<FactoredTailArm>();

	public FactoredGroup(OutputModelFactory factory, long mask, CodeBlockForAlt prefix) {
		super(factory);
		this.maskKey = "0x" + Long.toHexString(mask);
		this.prefix = prefix;
	}
}
