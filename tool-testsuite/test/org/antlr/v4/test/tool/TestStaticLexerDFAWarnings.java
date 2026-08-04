/*
 * Copyright (c) 2012-present The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.test.tool;

import org.antlr.v4.test.runtime.ErrorQueue;
import org.antlr.v4.tool.ErrorType;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.parallel.Execution;
import org.junit.jupiter.api.parallel.ExecutionMode;

import java.io.File;
import java.nio.file.Paths;

import static org.antlr.v4.test.runtime.FileUtils.deleteDirectory;
import static org.antlr.v4.test.runtime.Generator.antlrOnString;
import static org.antlr.v4.test.runtime.RuntimeTestUtils.TempDirectory;
import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * When {@code -Xstatic-dfa} cannot fully table-drive a lexer, the tool must
 * say why (one warning per reason, with grammar locations) instead of
 * silently falling back to the lazy simulator.
 */
// SAME_THREAD: testStateCap mutates the global antlr.lexerdfa.maxStates property
// and must not race the other tests (the suite enables parallel execution).
@Execution(ExecutionMode.SAME_THREAD)
public class TestStaticLexerDFAWarnings {
	private static String runTool(String grammarStr, String... extraOptions) {
		String[] lines = grammarStr.split("\n");
		String fileName = ToolTestUtils.getFilenameFromFirstLineOfGrammar(lines[0]);

		String tempDirName = "AntlrTestStaticLexerDFA-" + Thread.currentThread().getName() + "-" + System.currentTimeMillis();
		String tempTestDir = Paths.get(TempDirectory, tempDirName).toString();

		try {
			ErrorQueue equeue = antlrOnString(tempTestDir, null, fileName, grammarStr, false, extraOptions);
			String actual = equeue.toString(true);
			return actual.replace(tempTestDir + File.separator, "");
		}
		finally {
			try {
				deleteDirectory(new File(tempTestDir));
			} catch (Exception ignored) {
			}
		}
	}

	@Test
	public void testPredicate() {
		String grammar =
			"lexer grammar L;\n" +
			"A: 'a' {1 == 1}? 'z';\n" +
			"B: 'b';\n";
		String expected =
			"warning(" + ErrorType.STATIC_LEXER_DFA_INELIGIBLE.code + "): L.g4:2:7: -Xstatic-dfa: lexer rule A contains a predicate ({...}?) that is evaluated at runtime; generated lexer will use the lazy simulator\n";
		assertEquals(expected, runTool(grammar, "-Xstatic-dfa"));
	}

	@Test
	public void testEmbeddedAction() {
		String grammar =
			"lexer grammar L;\n" +
			"A: 'a' {System.out.println(\"hi\");} 'z';\n" +
			"B: 'b';\n";
		String expected =
			"warning(" + ErrorType.STATIC_LEXER_DFA_INELIGIBLE.code + "): L.g4:2:7: -Xstatic-dfa: lexer rule A contains an embedded action ({...}) whose effect may depend on the input position; generated lexer will use the lazy simulator\n";
		assertEquals(expected, runTool(grammar, "-Xstatic-dfa"));
	}

	@Test
	public void testRecursiveRules() {
		String grammar =
			"lexer grammar L;\n" +
			"A: 'a' B 'z';\n" +
			"B: 'b' A? | 'c';\n" +
			"C: '/*' (C | .)*? '*/' -> skip;\n";
		String expected =
			"warning(" + ErrorType.STATIC_LEXER_DFA_INELIGIBLE.code + "): L.g4:2:0: -Xstatic-dfa: lexer rule A can recursively invoke itself (recursive rules have no finite DFA expansion); generated lexer will use the lazy simulator\n" +
			"warning(" + ErrorType.STATIC_LEXER_DFA_INELIGIBLE.code + "): L.g4:3:0: -Xstatic-dfa: lexer rule B can recursively invoke itself (recursive rules have no finite DFA expansion); generated lexer will use the lazy simulator\n" +
			"warning(" + ErrorType.STATIC_LEXER_DFA_INELIGIBLE.code + "): L.g4:4:0: -Xstatic-dfa: lexer rule C can recursively invoke itself (recursive rules have no finite DFA expansion); generated lexer will use the lazy simulator\n";
		assertEquals(expected, runTool(grammar, "-Xstatic-dfa"));
	}

	@Test
	public void testMultipleReasonsSortedByLocation() {
		String grammar =
			"lexer grammar L;\n" +
			"C: '/*' (C | .)*? '*/' -> skip;\n" +
			"A: 'a' {1 == 1}? 'z';\n" +
			"B: 'b';\n";
		String expected =
			"warning(" + ErrorType.STATIC_LEXER_DFA_INELIGIBLE.code + "): L.g4:2:0: -Xstatic-dfa: lexer rule C can recursively invoke itself (recursive rules have no finite DFA expansion); generated lexer will use the lazy simulator\n" +
			"warning(" + ErrorType.STATIC_LEXER_DFA_INELIGIBLE.code + "): L.g4:3:7: -Xstatic-dfa: lexer rule A contains a predicate ({...}?) that is evaluated at runtime; generated lexer will use the lazy simulator\n";
		assertEquals(expected, runTool(grammar, "-Xstatic-dfa"));
	}

	@Test
	public void testStateCap() {
		String grammar =
			"lexer grammar L;\n" +
			"A: 'a'+;\n" +
			"B: 'b' -> skip;\n";
		String previous = System.setProperty("antlr.lexerdfa.maxStates", "2");
		try {
			String expected =
				"warning(" + ErrorType.STATIC_LEXER_DFA_STATE_CAP.code + "):  -Xstatic-dfa: static lexer DFA expansion of mode DEFAULT_MODE exceeded the cap of 2 states; generated lexer will use the lazy simulator (raise the cap with -Dantlr.lexerdfa.maxStates)\n";
			assertEquals(expected, runTool(grammar, "-Xstatic-dfa"));
		}
		finally {
			if (previous != null) System.setProperty("antlr.lexerdfa.maxStates", previous);
			else System.clearProperty("antlr.lexerdfa.maxStates");
		}
	}

	@Test
	public void testEligibleLexerIsQuiet() {
		String grammar =
			"lexer grammar L;\n" +
			"A: 'a'+;\n" +
			"B: 'b' -> skip;\n" +
			"mode M;\n" +
			"C: 'c';\n";
		assertEquals("", runTool(grammar, "-Xstatic-dfa"));
	}

	@Test
	public void testNoWarningsWithoutStaticDfaOption() {
		String grammar =
			"lexer grammar L;\n" +
			"A: 'a' {1 == 1}? 'z';\n" +
			"B: '/*' (B | .)*? '*/' -> skip;\n";
		assertEquals("", runTool(grammar));
	}
}
