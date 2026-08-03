/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.analysis;

/**
 * A statically-precomputed lexer DFA for a single lexer mode, produced by
 * {@link LexerDFABuilder} for lexers without esoteric features (no semantic
 * predicates, no position-dependent lexer actions). This is the
 * compile-time equivalent of the DFA the runtime's
 * {@code LexerATNSimulator} would otherwise build lazily: it is constructed
 * by driving that very simulator exhaustively over the whole input
 * alphabet, so maximal-munch, rule-priority, and non-greedy semantics are
 * identical by construction.
 *
 * <p>Layout: state {@link #startState} is the start state. For each state,
 * {@link #ascii} holds a dense row of 128 target-state entries
 * ({@code -1} = no edge) for input symbols 0..127, {@link #eof} the
 * (single) EOF target, and {@link #hiEdges}/{@link #hiOffsets} a sorted,
 * disjoint list of {@code (lo, hi, target)} triples covering symbols above
 * 127 (usually 0-1 entries). {@link #acceptType}{@code [s] >= 0} marks an
 * accept state producing that token type (0 is a real accept type: rules
 * with a {@code more} or {@code type(...)} command get no token type
 * assigned); {@link #acceptActions}/
 * {@link #actionLists} give the lexer-action list to execute at accept
 * (e.g. {@code skip}, {@code mode(X)}), all position-independent.</p>
 */
public class StaticLexerDFA {
	/** Highest input symbol with a dense per-state table entry. */
	public static final int ASCII_MAX = 127;

	/**
	 * Bit OR-ed into every serialized edge target (ascii/eof/hiEdges) when
	 * the target is an accept state: the runtime walker's per-char loop
	 * then reads target state and accept info from one table entry. Limits
	 * table size to 32767 states per mode (see LexerDFABuilder#maxStates).
	 */
	public static final int ACCEPT_BIT_INT = 0x8000;
	public final int mode;
	public final int numStates;
	public final int startState;
	/** numStates*128 dense target rows (target | ACCEPT_BIT_INT); -1 = no edge. */
	public final int[] ascii;
	/** EOF target per state (target | ACCEPT_BIT_INT); -1 = no edge. */
	public final int[] eof;
	/** Index of each state's first triple in {@link #hiEdges}; length numStates+1. */
	public final int[] hiOffsets;
	/** Flattened (lo, hi, target|ACCEPT_BIT_INT) triples, sorted by lo within each state, lo &gt; 127. */
	public final int[] hiEdges;
	/** Token type per accept state; -1 = not an accept state (0 is a real
 	 * accept type: rules with a {@code more}/{@code type(...)} command get no
 	 * token type assigned). */
	public final int[] acceptType;
	/**
	 * Offset of each accept state's action list in {@link #actionLists};
	 * -1 = no actions. A list is stored as {@code len, actionIndex...} with
	 * action indices into the ATN's lexer action list.
	 */
	public final int[] acceptActions;
	/** Flattened length-prefixed lexer action index lists. */
	public final int[] actionLists;

	public StaticLexerDFA(int mode, int numStates, int startState,
						  int[] ascii, int[] eof,
						  int[] hiOffsets, int[] hiEdges,
						  int[] acceptType, int[] acceptActions, int[] actionLists) {
		this.mode = mode;
		this.numStates = numStates;
		this.startState = startState;
		this.ascii = ascii;
		this.eof = eof;
		this.hiOffsets = hiOffsets;
		this.hiEdges = hiEdges;
		this.acceptType = acceptType;
		this.acceptActions = acceptActions;
		this.actionLists = actionLists;
	}
}
