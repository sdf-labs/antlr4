/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this source code is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen.model;

import org.antlr.v4.codegen.OutputModelFactory;

import java.util.ArrayList;
import java.util.List;

/**
 * An alt block whose decision may accept with a shared-descent group's
 * alternative mask (rule-level factoring). The runtime's
 * dfa_predict_mask walker returns the set of live alternatives as a
 * bitmask. Singleton masks take the ordinary alternative arms; a mask
 * covering a shared-descent group takes its descent arm: speculatively
 * call the group's common rule once (muted, rewound, graft removed),
 * then dispatch on the post-descent lookahead to the member
 * alternative's singleton arm. The prefix enumeration that exhausted
 * the table's construction budget is executed once instead of being
 * simulated per alternative.
 */
public class DescentAltBlock extends AltBlock {
	@ModelElement public final List<FactoredAltBlock.SingletonArm> singletons =
		new ArrayList<FactoredAltBlock.SingletonArm>();
	@ModelElement public final List<DescentGroup> groups = new ArrayList<DescentGroup>();

	public DescentAltBlock(AltBlock base) {
		super(base.factory, base.ast, base.alts);
		this.decision = base.decision;
		this.tableCanDefer = base.tableCanDefer;
		this.label = base.label;
		this.preamble = base.preamble;
		for (int i = 0; i < base.alts.size(); i++) {
			singletons.add(new FactoredAltBlock.SingletonArm(base.factory, i+1, base.alts.get(i)));
		}
	}
}
