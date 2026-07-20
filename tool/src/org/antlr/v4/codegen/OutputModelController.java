/*
 * Copyright (c) 2012-2017 The ANTLR Project. All rights reserved.
 * Use of this file is governed by the BSD 3-clause license that
 * can be found in the LICENSE.txt file in the project root.
 */

package org.antlr.v4.codegen;

import org.antlr.runtime.tree.CommonTreeNodeStream;
import org.antlr.v4.analysis.LeftRecursiveRuleAltInfo;
import org.antlr.v4.codegen.model.*;
import org.antlr.v4.codegen.model.decl.CodeBlock;
import org.antlr.v4.misc.Utils;
import org.antlr.v4.parse.ANTLRParser;
import org.antlr.v4.parse.GrammarASTAdaptor;
import org.antlr.v4.semantics.UseDefAnalyzer;
import org.antlr.v4.tool.*;
import org.antlr.v4.tool.ast.ActionAST;
import org.antlr.v4.tool.ast.BlockAST;
import org.antlr.v4.tool.ast.GrammarAST;
import org.antlr.v4.tool.ast.PredAST;
import org.stringtemplate.v4.ST;
import org.stringtemplate.v4.STGroup;

import java.util.ArrayList;
import java.util.List;
import java.util.Stack;

/** This receives events from SourceGenTriggers.g and asks factory to do work.
 *  Then runs extensions in order on resulting SrcOps to get final list.
 **/
public class OutputModelController {
	/** Who does the work? Doesn't have to be CoreOutputModelFactory. */
	public OutputModelFactory delegate;

	/** Post-processing CodeGeneratorExtension objects; done in order given. */
	public List<CodeGeneratorExtension> extensions = new ArrayList<CodeGeneratorExtension>();

	/** While walking code in rules, this is set to the tree walker that
	 *  triggers actions.
	 */
	public SourceGenTriggers walker;

	/** Context set by the SourceGenTriggers.g */
	public int codeBlockLevel = -1;
	public int treeLevel = -1;
	public OutputModelObject root; // normally ParserFile, LexerFile, ...
	public Stack<RuleFunction> currentRule = new Stack<RuleFunction>();
	public Alternative currentOuterMostAlt;
	public CodeBlock currentBlock;
	public CodeBlockForOuterMostAlt currentOuterMostAlternativeBlock;

	public OutputModelController(OutputModelFactory factory) {
		this.delegate = factory;
	}

	public void addExtension(CodeGeneratorExtension ext) { extensions.add(ext); }

	/** Build a file with a parser containing rule functions. Use the
	 *  controller as factory in SourceGenTriggers so it triggers codegen
	 *  extensions too, not just the factory functions in this factory.
	 */
	public OutputModelObject buildParserOutputModel(SourceType sourceType) {
		CodeGenerator gen = delegate.getGenerator();
		ParserFile file = parserFile(gen.getRecognizerFileName(sourceType));
		setRoot(file);
		file.parser = parser(file);
		// ST can't compare strings so need booleans
		file.genLean = sourceType == SourceType.SOURCE_LEAN;
		file.genContexts = sourceType == SourceType.SOURCE_CONTEXTS;
		file.genDFA = sourceType == SourceType.SOURCE_DFA;

		Grammar g = delegate.getGrammar();
		for (Rule r : g.rules.values()) {
			buildRuleFunction(file.parser, r);
		}

		return file;
	}

	public OutputModelObject buildLexerOutputModel(SourceType sourceType) {
		CodeGenerator gen = delegate.getGenerator();
		LexerFile file = lexerFile(gen.getRecognizerFileName(sourceType));
		setRoot(file);
		file.lexer = lexer(file);

		Grammar g = delegate.getGrammar();
		for (Rule r : g.rules.values()) {
			buildLexerRuleActions(file.lexer, r);
		}

		return file;
	}

	public OutputModelObject buildListenerOutputModel(SourceType sourceType) {
		CodeGenerator gen = delegate.getGenerator();
		return new ListenerFile(delegate, gen.getListenerFileName(sourceType));
	}

	public OutputModelObject buildBaseListenerOutputModel(SourceType sourceType) {
		CodeGenerator gen = delegate.getGenerator();
		return new BaseListenerFile(delegate, gen.getBaseListenerFileName(sourceType));
	}

	public OutputModelObject buildVisitorOutputModel(SourceType sourceType) {
		CodeGenerator gen = delegate.getGenerator();
		return new VisitorFile(delegate, gen.getVisitorFileName(sourceType));
	}

	public OutputModelObject buildBaseVisitorOutputModel(SourceType sourceType) {
		CodeGenerator gen = delegate.getGenerator();
		return new BaseVisitorFile(delegate, gen.getBaseVisitorFileName(sourceType));
	}

	public ParserFile parserFile(String fileName) {
		ParserFile f = delegate.parserFile(fileName);
		for (CodeGeneratorExtension ext : extensions) f = ext.parserFile(f);
		return f;
	}

	public Parser parser(ParserFile file) {
		Parser p = delegate.parser(file);
		for (CodeGeneratorExtension ext : extensions) p = ext.parser(p);
		return p;
	}

	public LexerFile lexerFile(String fileName) {
		return new LexerFile(delegate, fileName);
	}

	public Lexer lexer(LexerFile file) {
		return new Lexer(delegate, file);
	}

	/** Create RuleFunction per rule and update sempreds,actions of parser
	 *  output object with stuff found in r.
	 */
	public void buildRuleFunction(Parser parser, Rule r) {
		RuleFunction function = rule(r);
		parser.funcs.add(function);
		pushCurrentRule(function);
		function.fillNamedActions(delegate, r);

		if ( r instanceof LeftRecursiveRule ) {
			buildLeftRecursiveRuleFunction((LeftRecursiveRule)r,
										   (LeftRecursiveRuleFunction)function);
		}
		else {
			buildNormalRuleFunction(r, function);
		}

		Grammar g = getGrammar();
		for (ActionAST a : r.actions) {
			if ( a instanceof PredAST ) {
				PredAST p = (PredAST) a;
				RuleSempredFunction rsf = parser.sempredFuncs.get(r);
				if (rsf == null) {
					rsf = new RuleSempredFunction(delegate, r, function.ctxType);
					parser.sempredFuncs.put(r, rsf);
				}
				boolean isCtxDependent = UseDefAnalyzer.actionIsContextDependent(p);
				rsf.actions.put(g.sempreds.get(p), new Action(delegate, p, isCtxDependent));
			}
		}

		popCurrentRule();
	}

	public void buildLeftRecursiveRuleFunction(LeftRecursiveRule r, LeftRecursiveRuleFunction function) {
		buildNormalRuleFunction(r, function);

		// now inject code to start alts
		CodeGenerator gen = delegate.getGenerator();
		STGroup codegenTemplates = gen.getTemplates();

		// pick out alt(s) for primaries
		CodeBlockForOuterMostAlt outerAlt = (CodeBlockForOuterMostAlt)function.code.get(0);
		List<CodeBlockForAlt> primaryAltsCode = new ArrayList<CodeBlockForAlt>();
		SrcOp primaryStuff = outerAlt.ops.get(0);
		if ( primaryStuff instanceof Choice ) {
			Choice primaryAltBlock = (Choice) primaryStuff;
			primaryAltsCode.addAll(primaryAltBlock.alts);
		}
		else { // just a single alt I guess; no block
			primaryAltsCode.add((CodeBlockForAlt)primaryStuff);
		}

		// pick out alt(s) for op alts
		StarBlock opAltStarBlock = (StarBlock)outerAlt.ops.get(1);
		CodeBlockForAlt altForOpAltBlock = opAltStarBlock.alts.get(0);
		List<CodeBlockForAlt> opAltsCode = new ArrayList<CodeBlockForAlt>();
		SrcOp opStuff = altForOpAltBlock.ops.get(0);
		if ( opStuff instanceof AltBlock ) {
			AltBlock opAltBlock = (AltBlock)opStuff;
			opAltsCode.addAll(opAltBlock.alts);
		}
		else { // just a single alt I guess; no block
			opAltsCode.add((CodeBlockForAlt)opStuff);
		}

		// Insert code in front of each primary alt to create specialized ctx if there was a label
		for (int i = 0; i < primaryAltsCode.size(); i++) {
			LeftRecursiveRuleAltInfo altInfo = r.recPrimaryAlts.get(i);
			if ( altInfo.altLabel==null ) continue;
			ST altActionST = codegenTemplates.getInstanceOf("recRuleReplaceContext");
			altActionST.add("ctxName", Utils.capitalize(altInfo.altLabel));
			Action altAction =
				new Action(delegate, function.altLabelCtxs.get(altInfo.altLabel), altActionST);
			CodeBlockForAlt alt = primaryAltsCode.get(i);
			alt.insertOp(0, altAction);
		}

		// Insert code to set ctx.stop after primary block and before op * loop
		ST setStopTokenAST = codegenTemplates.getInstanceOf("recRuleSetStopToken");
		Action setStopTokenAction = new Action(delegate, function.ruleCtx, setStopTokenAST);
		outerAlt.insertOp(1, setStopTokenAction);

		// Insert code to set _prevctx at start of * loop
		ST setPrevCtx = codegenTemplates.getInstanceOf("recRuleSetPrevCtx");
		Action setPrevCtxAction = new Action(delegate, function.ruleCtx, setPrevCtx);
		opAltStarBlock.addIterationOp(setPrevCtxAction);

		// Insert code in front of each op alt to create specialized ctx if there was an alt label
		for (int i = 0; i < opAltsCode.size(); i++) {
			ST altActionST;
			LeftRecursiveRuleAltInfo altInfo = r.recOpAlts.getElement(i);
			String templateName;
			if ( altInfo.altLabel!=null ) {
				templateName = "recRuleLabeledAltStartAction";
				altActionST = codegenTemplates.getInstanceOf(templateName);
				altActionST.add("currentAltLabel", altInfo.altLabel);
			}
			else {
				templateName = "recRuleAltStartAction";
				altActionST = codegenTemplates.getInstanceOf(templateName);
				altActionST.add("ctxName", Utils.capitalize(r.name));
			}
			altActionST.add("ruleName", r.name);
			// add label of any lr ref we deleted
			altActionST.add("label", altInfo.leftRecursiveRuleRefLabel);
			if (altActionST.impl.formalArguments.containsKey("isListLabel")) {
				altActionST.add("isListLabel", altInfo.isListLabel);
			}
			else if (altInfo.isListLabel) {
				delegate.getGenerator().tool.errMgr.toolError(ErrorType.CODE_TEMPLATE_ARG_ISSUE, templateName, "isListLabel");
			}
			Action altAction =
				new Action(delegate, function.altLabelCtxs.get(altInfo.altLabel), altActionST);
			CodeBlockForAlt alt = opAltsCode.get(i);
			alt.insertOp(0, altAction);
		}

		// Alt-mask factoring ("implied left-factoring"): when the primary
		// block's decision has a validated prefix-factor plan, replace the
		// choice with a FactoredAltBlock whose group arms execute the
		// shared prefix and resolve the choice with an LL(1) tail switch.
		// Only for targets with mask codegen; others keep the DFAAltBlock
		// (mask-accept states in its table defer to adaptivePredict).
		if (primaryStuff instanceof AltBlock
			&& delegate.getGenerator().getTarget().supportsFactoredAltMask()) {
			org.antlr.v4.tool.Grammar gg = getGrammar();
			org.antlr.v4.analysis.PrefixFactorAnalyzer.Plan factorPlan =
				gg.staticFactorPlans != null
					? gg.staticFactorPlans.get(((AltBlock)primaryStuff).decision) : null;
			if (factorPlan != null) {
				org.antlr.v4.codegen.model.FactoredAltBlock factored =
					buildFactoredAltBlock(r, (AltBlock)primaryStuff, factorPlan);
				if (factored != null) outerAlt.ops.set(0, factored);
				if ("factor".equals(System.getProperty("antlr.dfa.debug"))) {
					System.err.printf("FACTOR-CODEGEN rule=%s decision=%d groups=%d built=%s%n",
						r.name, ((AltBlock)primaryStuff).decision, factorPlan.groups.size(),
						factored != null);
				}
			}
		}
	}

	/**
	 * Build the {@link org.antlr.v4.codegen.model.FactoredAltBlock} for a
	 * left-recursive rule's primary block from its factor plan, or null
	 * when model-level validation fails (the ordinary DFAAltBlock then
	 * stays; mask-accept states in its table defer to adaptivePredict).
	 * Runs after the label-context specialization actions have been
	 * inserted in front of the primary alternatives, so each factored
	 * tail arm can reuse its member's action op.
	 */
	protected org.antlr.v4.codegen.model.FactoredAltBlock buildFactoredAltBlock(
		LeftRecursiveRule r,
		AltBlock primaryBlock,
		org.antlr.v4.analysis.PrefixFactorAnalyzer.Plan plan)
	{
		org.antlr.v4.codegen.model.FactoredAltBlock factored =
			new org.antlr.v4.codegen.model.FactoredAltBlock(primaryBlock);
		for (org.antlr.v4.analysis.PrefixFactorAnalyzer.Group grp : plan.groups) {
			int[] members = new int[Long.bitCount(altBitsOf(grp))];
			long mask = altBitsOf(grp);
			int prefixLen = grp.prefix.size();
			// per-member structure: context action (labeled alts), prefix, tail
			java.util.List<CodeBlockForAlt> tailBodies = new java.util.ArrayList<CodeBlockForAlt>();
			CodeBlockForAlt prefixBody = null;
			boolean valid = true;
			int m = 0;
			for (int alt = grp.alts.nextSetBit(0); alt >= 0 && valid; alt = grp.alts.nextSetBit(alt+1)) {
				if (alt < 1 || alt > primaryBlock.alts.size()
					|| alt > r.recPrimaryAlts.size()) { valid = false; break; }
				CodeBlockForAlt altBody = primaryBlock.alts.get(alt-1);
				boolean hasCtxAction = r.recPrimaryAlts.get(alt-1).altLabel != null;
				int skip = hasCtxAction ? 1 : 0;
				// The ops list carries inert null placeholders between
				// real element ops; the element sequence is the non-null ops.
				java.util.List<SrcOp> elementOps = new java.util.ArrayList<SrcOp>();
				if (altBody.ops != null) {
					for (int i = skip; i < altBody.ops.size(); i++) {
						SrcOp op = altBody.ops.get(i);
						if (op != null) elementOps.add(op);
					}
				}
				if (elementOps.size() < prefixLen + 1) {
					if ("factor".equals(System.getProperty("antlr.dfa.debug"))) {
						System.err.printf("FACTOR-FAIL alt=%d elementOps=%d need>=%d%n", alt,
							elementOps.size(), prefixLen + 1);
					}
					valid = false; break;
				}
				java.util.List<SrcOp> prefixOps = elementOps.subList(0, prefixLen);
				for (SrcOp op : prefixOps) {
					if (!isPlainPrefixElement(op)) {
						if ("factor".equals(System.getProperty("antlr.dfa.debug"))) {
							System.err.printf("FACTOR-FAIL alt=%d op=%s not plain%n", alt,
								op.getClass().getSimpleName());
						}
						valid = false; break;
					}
				}
				if (!valid) break;
				if (prefixBody == null) {
					prefixBody = new CodeBlockForAlt(delegate);
					java.util.List<SrcOp> sharedOps = new java.util.ArrayList<SrcOp>(prefixOps);
					// The shared rule invocation must push the synthetic
					// call-site state (whose continuation spans every
					// member's tail), not any one member's - else nested
					// adaptive simulations prune against that member's
					// continuation (the (-1)-vs-(COL, -1) bug class).
					org.antlr.v4.codegen.model.InvokeRule sharedInvoke =
						new org.antlr.v4.codegen.model.InvokeRule(
							(org.antlr.v4.codegen.ParserFactory)delegate,
							((org.antlr.v4.codegen.model.InvokeRule)prefixOps.get(prefixLen-1)).ast,
							null);
					sharedInvoke.stateNumber = grp.syntheticInvokeState;
					sharedOps.set(prefixLen-1, sharedInvoke);
					prefixBody.addOps(sharedOps);
				}
				else if (!samePrefix(prefixBody.ops, prefixOps)) {
					if ("factor".equals(System.getProperty("antlr.dfa.debug"))) {
						System.err.printf("FACTOR-FAIL alt=%d prefix mismatch%n", alt);
					}
					valid = false; break;
				}
				CodeBlockForAlt tailBody = new CodeBlockForAlt(delegate);
				if (hasCtxAction) tailBody.addOp(altBody.ops.get(0));
				tailBody.addOps(new java.util.ArrayList<SrcOp>(elementOps.subList(prefixLen, elementOps.size())));
				tailBodies.add(tailBody);
				members[m++] = alt;
			}
			if (!valid) return null;
			org.antlr.v4.codegen.model.FactoredGroup group =
				new org.antlr.v4.codegen.model.FactoredGroup(delegate, mask, prefixBody);
			for (int i = 0; i < members.length; i++) {
				org.antlr.v4.runtime.misc.IntervalSet first = grp.tailFirst.get(members[i]);
				if (first == null || first.isNil()) return null;
				group.tails.add(new org.antlr.v4.codegen.model.FactoredTailArm(
					delegate, tailArmKey(first), tailBodies.get(i)));
			}
			factored.groups.add(group);
		}
		return factored;
	}

	/** Group alternative bitmask (bit 1<<(alt-1) per member). */
	private static long altBitsOf(org.antlr.v4.analysis.PrefixFactorAnalyzer.Group grp) {
		long bits = 0;
		for (int a = grp.alts.nextSetBit(0); a >= 0; a = grp.alts.nextSetBit(a+1)) {
			bits |= 1L << (a-1);
		}
		return bits;
	}

	/** Prefix elements must be plain, unlabeled token matches or rule
	 *  invocations (v1). */
	private static boolean isPlainPrefixElement(SrcOp op) {
		if (op instanceof org.antlr.v4.codegen.model.MatchToken) {
			return ((org.antlr.v4.codegen.model.MatchToken)op).labels.isEmpty();
		}
		if (op instanceof org.antlr.v4.codegen.model.InvokeRule) {
			return ((org.antlr.v4.codegen.model.InvokeRule)op).labels.isEmpty();
		}
		return false;
	}

	private static boolean samePrefixElement(SrcOp a, SrcOp b) {
		if (a instanceof org.antlr.v4.codegen.model.MatchToken
			&& b instanceof org.antlr.v4.codegen.model.MatchToken) {
			return ((org.antlr.v4.codegen.model.MatchToken)a).ttype
				== ((org.antlr.v4.codegen.model.MatchToken)b).ttype;
		}
		if (a instanceof org.antlr.v4.codegen.model.InvokeRule
			&& b instanceof org.antlr.v4.codegen.model.InvokeRule) {
			return ((org.antlr.v4.codegen.model.InvokeRule)a).name
				.equals(((org.antlr.v4.codegen.model.InvokeRule)b).name);
		}
		return false;
	}

	private static boolean samePrefix(java.util.List<SrcOp> a, java.util.List<SrcOp> b) {
		if (a.size() != b.size()) return false;
		for (int i = 0; i < a.size(); i++) {
			if (!samePrefixElement(a.get(i), b.get(i))) return false;
		}
		return true;
	}

	/** Match-arm pattern of a tail FIRST set: the grammar's token-type
	 *  constants joined with " | ". */
	private String tailArmKey(org.antlr.v4.runtime.misc.IntervalSet first) {
		StringBuilder sb = new StringBuilder();
		org.antlr.v4.codegen.Target target = delegate.getGenerator().getTarget();
		for (int i = 0; i < first.size(); i++) {
			if (i > 0) sb.append(" | ");
			int ttype = first.get(i);
			sb.append(getGrammar().name).append('_')
				.append(target.escapeIfNeeded(target.getTokenTypeAsTargetLabel(getGrammar(), ttype)));
		}
		return sb.toString();
	}

	public void buildNormalRuleFunction(Rule r, RuleFunction function) {
		CodeGenerator gen = delegate.getGenerator();
		// TRIGGER factory functions for rule alts, elements
		GrammarASTAdaptor adaptor = new GrammarASTAdaptor(r.ast.token.getInputStream());
		GrammarAST blk = (GrammarAST)r.ast.getFirstChildWithType(ANTLRParser.BLOCK);
		CommonTreeNodeStream nodes = new CommonTreeNodeStream(adaptor,blk);
		walker = new SourceGenTriggers(nodes, this);
		try {
			// walk AST of rule alts/elements
			function.code = DefaultOutputModelFactory.list(walker.block(null, null));
			function.hasLookaheadBlock = walker.hasLookaheadBlock;
		}
		catch (org.antlr.runtime.RecognitionException e){
			e.printStackTrace(System.err);
		}

		function.ctxType = gen.getTarget().getRuleFunctionContextStructName(function);

		function.postamble = rulePostamble(function, r);
	}

	public void buildLexerRuleActions(Lexer lexer, final Rule r) {
		if (r.actions.isEmpty()) {
			return;
		}

		CodeGenerator gen = delegate.getGenerator();
		Grammar g = delegate.getGrammar();
		String ctxType = gen.getTarget().getRuleFunctionContextStructName(r);
		RuleActionFunction raf = lexer.actionFuncs.get(r);
		if ( raf==null ) {
			raf = new RuleActionFunction(delegate, r, ctxType);
		}

		for (ActionAST a : r.actions) {
			if ( a instanceof PredAST ) {
				PredAST p = (PredAST)a;
				RuleSempredFunction rsf = lexer.sempredFuncs.get(r);
				if ( rsf==null ) {
					rsf = new RuleSempredFunction(delegate, r, ctxType);
					lexer.sempredFuncs.put(r, rsf);
				}
				rsf.actions.put(g.sempreds.get(p), new Action(delegate, p));
			}
			else if ( a.getType()== ANTLRParser.ACTION ) {
				raf.actions.put(g.lexerActions.get(a), new Action(delegate, a));
			}
		}

		if (!raf.actions.isEmpty() && !lexer.actionFuncs.containsKey(r)) {
			// only add to lexer if the function actually contains actions
			lexer.actionFuncs.put(r, raf);
		}
	}

	public RuleFunction rule(Rule r) {
		RuleFunction rf = delegate.rule(r);
		for (CodeGeneratorExtension ext : extensions) rf = ext.rule(rf);
		return rf;
	}

	public List<SrcOp> rulePostamble(RuleFunction function, Rule r) {
		List<SrcOp> ops = delegate.rulePostamble(function, r);
		for (CodeGeneratorExtension ext : extensions) ops = ext.rulePostamble(ops);
		return ops;
	}

	public Grammar getGrammar() { return delegate.getGrammar(); }

	public CodeGenerator getGenerator() { return delegate.getGenerator(); }

	public CodeBlockForAlt alternative(Alternative alt, boolean outerMost) {
		CodeBlockForAlt blk = delegate.alternative(alt, outerMost);
		if ( outerMost ) {
			currentOuterMostAlternativeBlock = (CodeBlockForOuterMostAlt)blk;
		}
		for (CodeGeneratorExtension ext : extensions) blk = ext.alternative(blk, outerMost);
		return blk;
	}

	public CodeBlockForAlt finishAlternative(CodeBlockForAlt blk, List<SrcOp> ops,
											 boolean outerMost)
	{
		blk = delegate.finishAlternative(blk, ops);
		for (CodeGeneratorExtension ext : extensions) blk = ext.finishAlternative(blk, outerMost);
		return blk;
	}

	public List<SrcOp> ruleRef(GrammarAST ID, GrammarAST label, GrammarAST args) {
		List<SrcOp> ops = delegate.ruleRef(ID, label, args);
		for (CodeGeneratorExtension ext : extensions) {
			ops = ext.ruleRef(ops);
		}
		return ops;
	}

	public List<SrcOp> tokenRef(GrammarAST ID, GrammarAST label, GrammarAST args)
	{
		List<SrcOp> ops = delegate.tokenRef(ID, label, args);
		for (CodeGeneratorExtension ext : extensions) {
			ops = ext.tokenRef(ops);
		}
		return ops;
	}

	public List<SrcOp> stringRef(GrammarAST ID, GrammarAST label) {
		List<SrcOp> ops = delegate.stringRef(ID, label);
		for (CodeGeneratorExtension ext : extensions) {
			ops = ext.stringRef(ops);
		}
		return ops;
	}

	/** (A|B|C) possibly with ebnfRoot and label */
	public List<SrcOp> set(GrammarAST setAST, GrammarAST labelAST, boolean invert) {
		List<SrcOp> ops = delegate.set(setAST, labelAST, invert);
		for (CodeGeneratorExtension ext : extensions) {
			ops = ext.set(ops);
		}
		return ops;
	}

	public CodeBlockForAlt epsilon(Alternative alt, boolean outerMost) {
		CodeBlockForAlt blk = delegate.epsilon(alt, outerMost);
		for (CodeGeneratorExtension ext : extensions) blk = ext.epsilon(blk);
		return blk;
	}

	public List<SrcOp> wildcard(GrammarAST ast, GrammarAST labelAST) {
		List<SrcOp> ops = delegate.wildcard(ast, labelAST);
		for (CodeGeneratorExtension ext : extensions) {
			ops = ext.wildcard(ops);
		}
		return ops;
	}

	public List<SrcOp> action(ActionAST ast) {
		List<SrcOp> ops = delegate.action(ast);
		for (CodeGeneratorExtension ext : extensions) ops = ext.action(ops);
		return ops;
	}

	public List<SrcOp> sempred(ActionAST ast) {
		List<SrcOp> ops = delegate.sempred(ast);
		for (CodeGeneratorExtension ext : extensions) ops = ext.sempred(ops);
		return ops;
	}

	public Choice getChoiceBlock(BlockAST blkAST, List<CodeBlockForAlt> alts, GrammarAST label) {
		Choice c = delegate.getChoiceBlock(blkAST, alts, label);
		for (CodeGeneratorExtension ext : extensions) c = ext.getChoiceBlock(c);
		return c;
	}

	public Choice getEBNFBlock(GrammarAST ebnfRoot, List<CodeBlockForAlt> alts) {
		Choice c = delegate.getEBNFBlock(ebnfRoot, alts);
		for (CodeGeneratorExtension ext : extensions) c = ext.getEBNFBlock(c);
		return c;
	}

	public boolean needsImplicitLabel(GrammarAST ID, LabeledOp op) {
		boolean needs = delegate.needsImplicitLabel(ID, op);
		for (CodeGeneratorExtension ext : extensions) needs |= ext.needsImplicitLabel(ID, op);
		return needs;
	}

	public OutputModelObject getRoot() { return root; }

	public void setRoot(OutputModelObject root) { this.root = root; }

	public RuleFunction getCurrentRuleFunction() {
		if ( !currentRule.isEmpty() )	return currentRule.peek();
		return null;
	}

	public void pushCurrentRule(RuleFunction r) { currentRule.push(r); }

	public RuleFunction popCurrentRule() {
		if ( !currentRule.isEmpty() ) return currentRule.pop();
		return null;
	}

	public Alternative getCurrentOuterMostAlt() { return currentOuterMostAlt; }

	public void setCurrentOuterMostAlt(Alternative currentOuterMostAlt) { this.currentOuterMostAlt = currentOuterMostAlt; }

	public void setCurrentBlock(CodeBlock blk) {
		currentBlock = blk;
	}

	public CodeBlock getCurrentBlock() {
		return currentBlock;
	}

	public void setCurrentOuterMostAlternativeBlock(CodeBlockForOuterMostAlt currentOuterMostAlternativeBlock) {
		this.currentOuterMostAlternativeBlock = currentOuterMostAlternativeBlock;
	}

	public CodeBlockForOuterMostAlt getCurrentOuterMostAlternativeBlock() {
		return currentOuterMostAlternativeBlock;
	}

	public int getCodeBlockLevel() { return codeBlockLevel; }
}
