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
 * One shared-descent group of a {@link DescentAltBlock}: the group's
 * alternative mask, the common rule's invocation (the speculative
 * prefix), the tail-dispatch arms (token-type constants of each
 * non-default member alternative's tail FIRST set, keyed to that
 * alternative's singleton bit), and the default member's singleton bit
 * (or an adaptive fallback when the dispatch is all-explicit).
 */
public class DescentGroup extends OutputModelObject {
	/** One tail-dispatch arm: token-type constants -> a member's singleton bit. */
	public static class Tail extends OutputModelObject {
		public final String armKey;
		public final String altBit;
		/** Guarded arm: commits only when the real parse stack's postfix
		 *  chase cannot consume the dispatch token (the group's nullable
		 *  member is then dead on this stack). */
		public final boolean guarded;

		public Tail(OutputModelFactory factory, String armKey, int alt, boolean guarded) {
			super(factory);
			this.armKey = armKey;
			this.altBit = "0x" + Long.toHexString(1L << (alt-1));
			this.guarded = guarded;
		}
	}

	/** The decision this group belongs to. */
	public final int decision;
	/** A valid call-site state of the common rule: the block sets the
	 *  parser's invoking state to it before the neutral parse. */
	public final int callSiteState;
	/** Coverage bits of the group's block alternatives (hex literal):
	 *  the arm fires for any multi-alt subset of this mask. */
	public final String coverBits;
	/** Leading shared tokens before the common rule: token-type
	 *  constants in match order (empty for a pure zero-token descent). */
	public final java.util.List<String> prefixTokens = new java.util.ArrayList<String>();
	/** prefixTokens.size(), as a template-ready property. */
	public int prefixLen;
	/** The common rule's generated method name (the shared descent). */
	public final String ruleName;
	@ModelElement public final List<Tail> tails = new ArrayList<Tail>();
	/** The default member's singleton bit (hex literal), or null for an
	 *  all-explicit dispatch (the fallback widens via adaptivePredict). */
	public final String defaultBit;
	/** Head-guard check of a group with conditional members (see
	 *  SharedDescentAnalyzer.Group#headGuard): rendered as a boolean
	 *  expression that is false when the lookahead enters a member's
	 *  pre-R side exit (the neutral span would misalign); null when the
	 *  group has no conditional members. */
	public String headGuard;
	/** FIRST(R) pre-check (see SharedDescentAnalyzer.Group#firstTokens):
	 *  the neutral parse is attempted only when the post-prefix
	 *  lookahead can start R. */
	public String firstCheck;

	public DescentGroup(OutputModelFactory factory, int decision, long cover, String ruleName, String defaultBit,
						int callSiteState) {
		super(factory);
		this.decision = decision;
		this.coverBits = "0x" + Long.toHexString(cover);
		this.ruleName = ruleName;
		this.defaultBit = defaultBit;
		this.callSiteState = callSiteState;
	}
}
