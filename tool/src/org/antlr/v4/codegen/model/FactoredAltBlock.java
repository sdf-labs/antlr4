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
 * An alt block whose decision is driven by a statically-precomputed SLL
 * prediction table via the mask protocol ("implied left-factoring"):
 * the runtime's dfaPredictMask walker returns the set of live
 * alternatives as a bitmask. Singleton masks take the ordinary
 * alternative arms; a mask covering a prefix-factor group takes its
 * factored arm, which executes the group's shared prefix and then
 * resolves the choice with the group's LL(1) tail decision.
 */
public class FactoredAltBlock extends AltBlock {
	/** One singleton-mask arm: the ordinary full-alternative body, keyed
	 *  by its single-bit alternative mask. */
	public static class SingletonArm extends OutputModelObject {
		public final String armKey;
		@ModelElement public final CodeBlockForAlt body;

		public SingletonArm(OutputModelFactory factory, int alt, CodeBlockForAlt body) {
			super(factory);
			this.armKey = "0x" + Long.toHexString(1L << (alt-1));
			this.body = body;
		}
	}

	@ModelElement public final List<SingletonArm> singletons = new ArrayList<SingletonArm>();
	@ModelElement public final List<FactoredGroup> groups = new ArrayList<FactoredGroup>();

	public FactoredAltBlock(AltBlock base) {
		super(base.factory, base.ast, base.alts);
		this.decision = base.decision;
		this.tableCanDefer = base.tableCanDefer;
		this.label = base.label;
		this.preamble = base.preamble;
		for (int i = 0; i < base.alts.size(); i++) {
			singletons.add(new SingletonArm(base.factory, i+1, base.alts.get(i)));
		}
	}
}
