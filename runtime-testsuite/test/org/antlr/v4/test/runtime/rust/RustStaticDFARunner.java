/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */
package org.antlr.v4.test.runtime.rust;

import org.antlr.v4.test.runtime.RunOptions;

import java.util.List;

/**
 * A {@link RustRunner} that generates the recognizers with
 * {@code -Xstatic-dfa}, so every statically-decidable decision is driven by
 * a precomputed prediction table instead of {@code adaptive_predict}. Used
 * by {@link RustStaticDFARuntimeTests} to run the descriptor corpus as a
 * large differential test: expected outputs were authored against adaptive
 * prediction, so any deviation of the table-driven path fails the test.
 */
public class RustStaticDFARunner extends RustRunner {
	/**
	 * Keep this configuration's build cache separate from the adaptive
	 * suite's: both otherwise share {@code <tmp>/ANTLR-runtime-testsuite-cache/Rust},
	 * where {@code cargo clean} during one suite's runtime initialization can
	 * delete the rlib out from under the other suite's compiles when they
	 * overlap (same JVM with concurrent classes, or concurrent Maven JVMs).
	 */
	@Override
	protected String getRuntimeConfigurationName() {
		return "Rust-static-dfa";
	}

	@Override
	protected List<String> getTargetToolOptions(RunOptions ro) {
		List<String> options = super.getTargetToolOptions(ro);
		options.add("-Xstatic-dfa");
		return options;
	}
}
