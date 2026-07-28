/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.runtime.atn;

import java.util.Base64;

/**
 * The deserialized statically-precomputed SLL prediction tables of one
 * generated parser (produced by the ANTLR tool's {@code -Xstatic-dfa}
 * option). All of a parser's tables are packed into one compact serialized
 * blob - in the same vein as {@code _serializedATN} - deserialized once at
 * class initialization; predictions then run through
 * {@link org.antlr.v4.runtime.Parser#dfaPredict} as a plain table walk with
 * no ATN simulation and no per-prediction allocation.
 *
 * <p>Wire format: each logical value is zigzag-mapped
 * ({@code (v << 1) ^ (v >> 31)}) and stored as an unsigned LEB128 varint;
 * the byte stream is base64-encoded (standard alphabet) and split into
 * string segments. Logical layout:</p>
 *
 * <pre>
 * FORMAT_VERSION
 * numTables
 * numDecisionSlots                  (max decision number + 1)
 * for each table:
 *   decision                        (-1: referenced only via a dispatch below)
 *   numStates
 *   numEdgeInts
 *   accepts[numStates]              (predicted alt per state; 0 = non-accept;
 *                                    -1 = escape: defer to adaptivePredict)
 *   fallbacks[numStates]            (error-avoidance alt per state; 0 = none)
 *   edgeOffsets[numStates+1]        (index of each state's first edge int)
 *   edges[numEdgeInts]              ((lo, hi, target) triples, lo-sorted per state)
 * numPrecedenceDispatches
 * for each dispatch:                (a left-recursive precedence loop decision)
 *   decision
 *   numCutoffs
 *   cutoffs[numCutoffs]             (sorted; class(p) = #cutoffs &lt; p)
 *   tableIndex[numCutoffs+1]        (table of each precedence class;
 *                                    -1 = class dispatches to adaptivePredict)
 * </pre>
 *
 * <p>State 0 of each table is its start state; {@code accepts[s] > 0} marks
 * a terminal accept state predicting that alternative (numbering identical
 * to {@code adaptivePredict}'s return value). A lookahead token matching no
 * edge means no viable alternative. Cyclic tables (LL(*) decisions) are
 * walked by the same loop; termination is guaranteed because every step
 * consumes one token of lookahead and the input is finite.</p>
 *
 * <p>Precedence dispatches serve the operator loops of left-recursive
 * rules: the loop's viable-operator set depends on the current precedence
 * (the {@code _p} argument of the rewritten rule), so the tool precomputes
 * one table per precedence equivalence class and the walker selects by
 * {@code getPrecedence()} - the static analogue of the adaptive runtime's
 * per-precedence DFA start states.</p>
 */
public class StaticDFATables {
	/** Must match the tool's SerializedStaticDFAs.FORMAT_VERSION. */
	public static final int FORMAT_VERSION = 7;

	/** Concatenated per-table data: accepts, fallbacks, edgeOffsets, edges. */
	protected final int[] data;
	/** Per-table (acceptsAt, fallbacksAt, edgeOffsetsAt, edgesAt, endAt) indexes into {@link #data}. */
	protected final int[] metas;
	/** Per-table guarded-take resolution alt per state (0 = not guarded). */
	protected final int[][] guardedAlts;
	/** Per-table sorted invoking states tripping the decision's stack guard. */
	protected final int[][] guardDanger;
	/** Per-table sorted invoking states the guard walk may pop past. */
	protected final int[][] guardPass;
	/**
	 * decision number -> table index ({@code >= 0}), -1 (no table), or
	 * {@code -(dispatch offset) - 2}: an offset into {@link #dispatchData},
	 * where a dispatch entry reads {@code numCutoffs, cutoffs...,
	 * tableIndex...} ({@code numCutoffs+1} table indices).
	 */
	protected final int[] decisionToTable;
	/** Flattened precedence dispatch entries. */
	protected final int[] dispatchData;

	protected StaticDFATables(int[] data, int[] metas, int[] decisionToTable, int[] dispatchData,
							  int[][] guardedAlts, int[][] guardDanger, int[][] guardPass) {
		this.data = data;
		this.metas = metas;
		this.decisionToTable = decisionToTable;
		this.dispatchData = dispatchData;
		this.guardedAlts = guardedAlts;
		this.guardDanger = guardDanger;
		this.guardPass = guardPass;
	}

	/**
	 * Deserialize the base64/varint segments emitted by the tool. Throws on
	 * malformed input or a format-version mismatch: the blob is generated
	 * together with the parser that embeds it, so any failure is a build
	 * inconsistency, not a runtime condition.
	 */
	public static StaticDFATables deserialize(String[] segments) {
		StringBuilder joined = new StringBuilder();
		for (String segment : segments) joined.append(segment);
		byte[] bytes = Base64.getDecoder().decode(joined.toString());

		IntReader ints = new IntReader(bytes);
		int version = ints.next();
		if (version != FORMAT_VERSION) {
			throw new UnsupportedOperationException(
				"static DFA table format version mismatch ("+version+" != "+FORMAT_VERSION+"): " +
				"parser was generated with a different ANTLR tool version, please regenerate");
		}
		int numTables = ints.next();
		int numSlots = ints.next();

		int[] decisionToTable = new int[numSlots];
		java.util.Arrays.fill(decisionToTable, -1);
		int[] metas = new int[numTables*5];
		int[][] guardedAlts = new int[numTables][];
		int[][] guardDanger = new int[numTables][];
		int[][] guardPass = new int[numTables][];

		// first pass over structure requires data sizes; buffer grows as we read
		IntBuffer data = new IntBuffer();
		for (int table = 0; table < numTables; table++) {
			int decision = ints.next();
			int numStates = ints.next();
			int numEdgeInts = ints.next();
			if (decision >= 0) decisionToTable[decision] = table;

			metas[table*5] = data.size;                       // acceptsAt
			for (int i = 0; i < numStates; i++) data.add(ints.next());
			metas[table*5+1] = data.size;                     // fallbacksAt
			for (int i = 0; i < numStates; i++) data.add(ints.next());
			metas[table*5+2] = data.size;                     // edgeOffsetsAt
			for (int i = 0; i < numStates+1; i++) data.add(ints.next());
			metas[table*5+3] = data.size;                     // edgesAt
			for (int i = 0; i < numEdgeInts; i++) data.add(ints.next());
			// v6: alternative-mask section. The Java target has no
			// factored codegen, so a mask-accept state simply defers to
			// adaptivePredict - exactly like an escape state - and the
			// mask's member alternatives are not retained here (the Rust
			// runtime, which does implement the mask protocol, decodes
			// them from the same section).
			int numMaskStates = ints.next();
			for (int i = 0; i < numMaskStates; i++) {
				int state = ints.next();
				int numAlts = ints.next();
				for (int a = 0; a < numAlts; a++) ints.next();
				data.set(metas[table*5]+state, MASK_DEFER);
			}
			// v7: guarded-take section (optional-postfix guards)
			int numGuardedStates = ints.next();
			if (numGuardedStates > 0) {
				int[] alts = new int[numStates];
				for (int i = 0; i < numGuardedStates; i++) {
					int state = ints.next();
					alts[state] = ints.next();
					data.set(metas[table*5]+state, GUARDED);
				}
				guardedAlts[table] = alts;
			}
			int numDanger = ints.next();
			if (numDanger > 0) {
				int[] danger = new int[numDanger];
				for (int i = 0; i < numDanger; i++) danger[i] = ints.next();
				guardDanger[table] = danger;
			}
			int numPass = ints.next();
			if (numPass > 0) {
				int[] pass = new int[numPass];
				for (int i = 0; i < numPass; i++) pass[i] = ints.next();
				guardPass[table] = pass;
			}
			metas[table*5+4] = data.size;                     // endAt
		}
		int numDispatches = ints.next();
		IntBuffer dispatchData = new IntBuffer();
		for (int dispatch = 0; dispatch < numDispatches; dispatch++) {
			int decision = ints.next();
			decisionToTable[decision] = -dispatchData.size - 2;
			int numCutoffs = ints.next();
			dispatchData.add(numCutoffs);
			for (int i = 0; i < 2*numCutoffs + 1; i++) dispatchData.add(ints.next());
		}
		if (!ints.atEnd()) {
			throw new IllegalStateException("trailing bytes in static DFA blob");
		}
		return new StaticDFATables(data.toArray(), metas, decisionToTable, dispatchData.toArray(),
			guardedAlts, guardDanger, guardPass);
	}

	/** {@code accepts} sentinel: a hybrid table's escape state - the walker
	 *  defers the whole prediction to {@code adaptivePredict}. */
	public static final int ESCAPE = -1;
	/** {@code accepts} sentinel: a mask-accept state (the live alternatives
	 *  are covered by one prefix-factor group). Java has no factored
	 *  codegen, so the walker defers to {@code adaptivePredict}; the Rust
	 *  runtime resolves the mask through its factored alternative path. */
	public static final int MASK_DEFER = -2;

	/** {@code accepts} sentinel: a guarded-take state of an
	 *  optional-postfix decision: the walker evaluates the decision's
	 *  stack guard ({@link #guardDefers}) and resolves to
	 *  {@link #guardedAlt} when it passes, deferring to
	 *  {@code adaptivePredict} when the stack trips a danger invoking
	 *  state. */
	public static final int GUARDED = -3;

	/**
	 * Is {@code decision} precedence-dispatched (per-precedence-class
	 * tables of a left-recursive loop)? Such tables are built against the
	 * decision rule's compatible call sites and must not be consulted when
	 * the left-recursive rule itself is the parse entry (no caller frame):
	 * the adaptive engine then explores every FOLLOW link from the empty
	 * stack, a behavior the class tables deliberately prune.
	 */
	public boolean isPrecedenceDispatched(int decision) {
		return decisionToTable[decision] <= -2;
	}

	/**
	 * The table serving {@code decision}: {@code precedence} (the parser's
	 * current precedence, i.e. the top of its precedence stack) selects the
	 * precedence class of dispatched decisions - the operator loops of
	 * left-recursive rules - and is ignored for plain ones. Returns -1 when
	 * the selected precedence class has no static table and the prediction
	 * must run through {@code adaptivePredict}.
	 */
	public int tableFor(int decision, int precedence) {
		int table = decisionToTable[decision];
		if (table <= -2) {
			// precedence dispatch: class(p) = #cutoffs < p
			int at = -table - 2;
			int numCutoffs = dispatchData[at];
			int cls = 0;
			while (cls < numCutoffs && dispatchData[at+1+cls] < precedence) cls++;
			table = dispatchData[at+1+numCutoffs+cls];
		}
		return table;
	}

	/** Predicted alternative if {@code state} of {@code table} accepts;
	 *  0 = not an accept state; {@link #ESCAPE} = escape state. */
	public int accept(int table, int state) {
		int acceptsAt = metas[table*5];
		return data[acceptsAt+state];
	}

	/**
	 * Error-avoidance fallback alternative of {@code state}, or 0: the
	 * minimum alternative that already finished the decision entry rule,
	 * returned when no edge matches so the parser reports a more precise
	 * error at the actual mismatch point (mirroring adaptivePredict's
	 * getAltThatFinishedDecisionEntryRule recovery).
	 */
	public int fallback(int table, int state) {
		int fallbacksAt = metas[table*5+1];
		return data[fallbacksAt+state];
	}

	/** Successor of {@code state} on token {@code t}, or -1 (binary search). */
	public int edge(int table, int state, int t) {
		int edgeOffsetsAt = metas[table*5+2];
		int edgesAt = metas[table*5+3];
		int lo = data[edgeOffsetsAt+state]/3;
		int hi = data[edgeOffsetsAt+state+1]/3 - 1;
		while (lo <= hi) {
			int mid = (lo+hi) >>> 1;
			int at = edgesAt + mid*3;
			if (t < data[at]) hi = mid-1;
			else if (t > data[at+1]) lo = mid+1;
			else return data[at+2];
		}
		return -1;
	}

	/** Resolution alternative of a {@link #GUARDED} state. */
	public int guardedAlt(int table, int state) {
		return guardedAlts[table][state];
	}

	/**
	 * The stack guard of a guarded-take state: walk the parse stack's
	 * invoking states - the epsilon-pop chase from the decision's block
	 * end - and answer whether the prediction must defer to
	 * {@code adaptivePredict}: an invoking state whose follow region
	 * contains a guard root ({@code danger}) trips it; an invoking state
	 * whose follow cannot reach its own rule's stop ends the chase (the
	 * configurations it could still realize are confined to the mirror
	 * discipline); anything else keeps walking.
	 */
	public boolean guardDefers(int table, org.antlr.v4.runtime.RuleContext ctx) {
		int[] danger = guardDanger[table];
		int[] pass = guardPass[table];
		for (org.antlr.v4.runtime.RuleContext c = ctx;
			 c instanceof org.antlr.v4.runtime.ParserRuleContext;
			 c = c.parent)
		{
			int inv = ((org.antlr.v4.runtime.ParserRuleContext)c).invokingState;
			if (inv < 0) break;
			if (java.util.Arrays.binarySearch(danger, inv) >= 0) return true;
			if (java.util.Arrays.binarySearch(pass, inv) < 0) break;
		}
		return false;
	}

	private static final class IntReader {
		final byte[] bytes;
		int at;

		IntReader(byte[] bytes) { this.bytes = bytes; }

		/** Read one logical value: un-zigzagged LEB128 varint. */
		int next() {
			int value = 0;
			int shift = 0;
			while (true) {
				byte b = bytes[at++];
				value |= (b & 0x7F) << shift;
				if ((b & 0x80) == 0) break;
				shift += 7;
			}
			return (value >>> 1) ^ -(value & 1); // un-zigzag
		}

		boolean atEnd() { return at == bytes.length; }
	}

	private static final class IntBuffer {
		int[] data = new int[4096];
		int size;

		void add(int v) {
			if (size == data.length) {
				int[] bigger = new int[data.length*2];
				System.arraycopy(data, 0, bigger, 0, size);
				data = bigger;
			}
			data[size++] = v;
		}

		void set(int i, int v) { data[i] = v; }

		int[] toArray() {
			int[] result = new int[size];
			System.arraycopy(data, 0, result, 0, size);
			return result;
		}
	}
}
