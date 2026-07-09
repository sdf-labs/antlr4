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
 *   decision
 *   numStates
 *   numEdgeInts
 *   accepts[numStates]              (predicted alt per state; 0 = non-accept)
 *   edgeOffsets[numStates+1]        (index of each state's first edge int)
 *   edges[numEdgeInts]              ((lo, hi, target) triples, lo-sorted per state)
 * </pre>
 *
 * <p>State 0 of each table is its start state; {@code accepts[s] > 0} marks
 * a terminal accept state predicting that alternative (numbering identical
 * to {@code adaptivePredict}'s return value). A lookahead token matching no
 * edge means no viable alternative. Cyclic tables (LL(*) decisions) are
 * walked by the same loop; termination is guaranteed because every step
 * consumes one token of lookahead and the input is finite.</p>
 */
public class StaticDFATables {
	/** Must match the tool's SerializedStaticDFAs.FORMAT_VERSION. */
	public static final int FORMAT_VERSION = 2;

	/** Concatenated per-table data: accepts, edgeOffsets, edges. */
	protected final int[] data;
	/** Per-table (acceptsAt, edgeOffsetsAt, edgesAt, endAt) indexes into {@link #data}. */
	protected final int[] metas;
	/** decision number -> table index, or -1. */
	protected final int[] decisionToTable;

	protected StaticDFATables(int[] data, int[] metas, int[] decisionToTable) {
		this.data = data;
		this.metas = metas;
		this.decisionToTable = decisionToTable;
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
		int[] metas = new int[numTables*4];

		// first pass over structure requires data sizes; buffer grows as we read
		IntBuffer data = new IntBuffer();
		for (int table = 0; table < numTables; table++) {
			int decision = ints.next();
			int numStates = ints.next();
			int numEdgeInts = ints.next();
			decisionToTable[decision] = table;

			metas[table*4] = data.size;                       // acceptsAt
			for (int i = 0; i < numStates; i++) data.add(ints.next());
			metas[table*4+1] = data.size;                     // edgeOffsetsAt
			for (int i = 0; i < numStates+1; i++) data.add(ints.next());
			metas[table*4+2] = data.size;                     // edgesAt
			for (int i = 0; i < numEdgeInts; i++) data.add(ints.next());
			metas[table*4+3] = data.size;                     // endAt
		}
		if (!ints.atEnd()) {
			throw new IllegalStateException("trailing bytes in static DFA blob");
		}
		return new StaticDFATables(data.toArray(), metas, decisionToTable);
	}

	/** Predicted alternative if {@code state} of {@code decision}'s table accepts; else 0. */
	public int accept(int decision, int state) {
		int acceptsAt = metas[decisionToTable[decision]*4];
		return data[acceptsAt+state];
	}

	/** Successor of {@code state} on token {@code t}, or -1 (binary search). */
	public int edge(int decision, int state, int t) {
		int table = decisionToTable[decision];
		int edgeOffsetsAt = metas[table*4+1];
		int edgesAt = metas[table*4+2];
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

		int[] toArray() {
			int[] result = new int[size];
			System.arraycopy(data, 0, result, 0, size);
			return result;
		}
	}
}
