/*
 * Copyright (c) 2012-2022 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.test.runtime.rust;

import org.antlr.v4.test.runtime.RuntimeRunner;
import org.antlr.v4.test.runtime.RuntimeTestDescriptor;
import org.antlr.v4.test.runtime.RuntimeTests;
import org.junit.jupiter.api.parallel.Execution;
import org.junit.jupiter.api.parallel.ExecutionMode;

/**
 * Runs the full runtime-test descriptor corpus against the Rust target with
 * {@code -Xstatic-dfa} enabled (see {@link RustStaticDFARunner}). Since all
 * expected outputs were authored against {@code adaptive_predict}, this
 * suite is a broad differential test of the table-driven static prediction
 * path: parse trees, matched text, and error messages must be identical.
 *
 * <p>Descriptors that introspect the adaptive prediction engine itself are
 * excluded: ambiguity/full-context diagnostics ({@code showDiagnosticErrors})
 * and ATN traces ({@code traceATN}) are reported from inside
 * {@code adaptive_predict}, which statically-decided decisions bypass by
 * design (a trusted exact ambiguity, for example, is resolved to its minimum
 * alternative by the table without a runtime ambiguity report).</p>
 *
 * <p>Executed {@link ExecutionMode#SAME_THREAD} so this suite never overlaps
 * {@link RustRuntimeTests} in the forked JVM (see that class for why), and
 * its runner uses a separate build-cache directory
 * ({@link RustStaticDFARunner#getRuntimeConfigurationName()}) so the two
 * suites cannot clobber each other's compiled runtime even across
 * concurrent Maven JVMs.</p>
 */
@Execution(ExecutionMode.SAME_THREAD)
public class RustStaticDFARuntimeTests extends RuntimeTests {
	@Override
	protected RuntimeRunner createRuntimeRunner() {
		return new RustStaticDFARunner();
	}

	@Override
	protected boolean skipDescriptor(RuntimeTestDescriptor descriptor) {
		return descriptor.showDiagnosticErrors || descriptor.traceATN;
	}
}
