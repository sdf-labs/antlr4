/*
 * Copyright (c) 2012-2022 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.test.runtime.rust;

import org.antlr.v4.test.runtime.RuntimeRunner;
import org.antlr.v4.test.runtime.RuntimeTests;
import org.junit.jupiter.api.parallel.Execution;
import org.junit.jupiter.api.parallel.ExecutionMode;

/**
 * Runs the descriptor corpus against the Rust target with adaptive
 * prediction. Executed {@link ExecutionMode#SAME_THREAD} so this suite never
 * overlaps {@link RustStaticDFARuntimeTests} in the forked JVM: with
 * concurrent classes, surefire 2.22.0 mis-attributes the two suites'
 * dynamic tests to whichever class reports first (706/1-style splits), and
 * any failure is reported against the wrong suite. Tests within the suite
 * still run concurrently via the {@code @TestFactory}'s own annotation.
 */
@Execution(ExecutionMode.SAME_THREAD)
public class RustRuntimeTests extends RuntimeTests {
	@Override
	protected RuntimeRunner createRuntimeRunner() {
		return new RustRunner();
	}
}
