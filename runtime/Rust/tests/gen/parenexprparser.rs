// Generated from ParenExpr.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::Arena;
use dbt_antlr4::PredictionContextCache;
use dbt_antlr4::parser::{Parser, BaseParser, ParserRecog, ListenerId};
use dbt_antlr4::token::CommonToken;
use dbt_antlr4::token_stream::TokenStream;
use dbt_antlr4::TokenSource;
use dbt_antlr4::parser_atn_simulator::ParserATNSimulator;
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::rule_context::{CustomRuleContext, RuleContext};
use dbt_antlr4::recognizer::{Recognizer,Actions};
use dbt_antlr4::atn_config_set::ATNConfigSet;
use dbt_antlr4::atn_deserializer::ATNDeserializer;
use dbt_antlr4::atn_simulator::BaseATNSimulator;
use dbt_antlr4::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use dbt_antlr4::atn::{ATN, INVALID_ALT};
use dbt_antlr4::error_strategy::{DefaultErrorStrategy, ErrorStrategyDelegate, ErrorStrategy};
use dbt_antlr4::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use dbt_antlr4::tree::*;
use dbt_antlr4::token::{TOKEN_EOF,Token};
use dbt_antlr4::int_stream::EOF;
use dbt_antlr4::vocabulary::{Vocabulary,VocabularyImpl};
use dbt_antlr4::token_factory::TokenFactory;
use super::parenexprlistener::*;
use std::marker::PhantomData;
use std::sync::LazyLock;
use std::rc::Rc;
use std::ops::{DerefMut, Deref};

dbt_antlr4::check_version!("2","0");
pub const ParenExpr_T__0:i32=1; 
pub const ParenExpr_T__1:i32=2; 
pub const ParenExpr_T__2:i32=3; 
pub const ParenExpr_T__3:i32=4; 
pub const ParenExpr_T__4:i32=5; 
pub const ParenExpr_T__5:i32=6; 
pub const ParenExpr_T__6:i32=7; 
pub const ParenExpr_T__7:i32=8; 
pub const ParenExpr_ID:i32=9; 
pub const ParenExpr_INT:i32=10; 
pub const ParenExpr_WS:i32=11;
pub const ParenExpr_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_e:usize = 1; 
pub const RULE_q:usize = 2;
pub const ruleNames: [&'static str; 3] = [
    "s", "e", "q"
];

pub const _LITERAL_NAMES: [Option<&'static str>;9] = [
	None, Some("'('"), Some("','"), Some("')'"), Some("'-'"), Some("'*'"), 
	Some("'/'"), Some("'+'"), Some("'SELECT'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
	None, None, None, None, None, None, None, None, None, Some("ID"), Some("INT"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, ParenExprParserExt<'input, 'arena>, ParenExprParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				ParenExprParserExt {
					_pd: Default::default(),
				}
			),
            err_handler: unsafe { ErrorStrategyDelegate::new(strategy) },
        }
    }

    pub fn new(arena: &'arena Arena, input: Input) -> Self{
    	Self::with_strategy(arena, input, Box::new(DefaultErrorStrategy::new()))
    }

    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) {
        self.err_handler = unsafe { ErrorStrategyDelegate::new(strategy) };
    }

    /// Adds parse listener for this parser
    /// returns `listener_id` that can be used later to get listener back
    ///
    /// ### Example for listener usage:
    /// todo
    pub fn add_parse_listener<L>(
        &mut self,
        listener: Box<L>,
    ) -> ListenerId<L>
    where
        L: ParenExprListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct ParenExprTreeWalker;
impl ParenExprTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParenExprListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, ParenExprParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <ParenExprParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum ParenExprParserNodeKind {
    SContext,
    EContext,
    QContext,
    Terminal,
    Error,
}
pub type ParenExprParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, ParenExprParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => ParenExprParser }
dbt_antlr4::impl_node_kind! { ParenExprParserNodeKind {
    EContext(EContextAll), QContext(QContextAll), ; SContext(enter_s, exit_s, ), 
    }; listener = dyn ParenExprListener<'arena, Tok>,
}

pub struct ParenExprParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> ParenExprParserExt<'input, 'arena> {
}

impl<'input, 'arena, Input, TF> ParserRecog<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for ParenExprParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena {
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }        
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for ParenExprParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn get_grammar_file_name(&self) -> & str{ "ParenExpr.g4" }
   	fn get_rule_names(&self) -> &[& str] { &ruleNames }
   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&'arena ParenExprParserNode<'input, 'arena, TF::Tok>>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input, 'arena, Input, TF>
	) -> bool {
		match rule_index {
		    1 => ParenExprParser::<'input, 'arena, Input, TF>::e_sempred(_localctx.and_then(|x| x.as_rule_context()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn e_sempred(_ctx: Option<&'arena EContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 2)
		    }
	        1 => {
			recog.precpred(None, 1)
		    }
		    _ => true
		}
	}
}
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

pub type SContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, SContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct SContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for SContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::SContext }
	fn get_rule_index(&self) -> usize { RULE_s }
    fn make_node(
        arena: &'arena Arena,
        ctx: SContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a SContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => SContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut SContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut SContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ParenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ParenExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, SContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait SContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SContextAttrs<'input, 'arena, Tok> for SContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_EOF)
    }
}

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn s(&mut self,) -> Result<&'arena SContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(SContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 0, RULE_s)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena SContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule e*/
			recog.base.set_state(6);
			recog.e_rec(0)?;
			recog.base.set_state(7);
			recog.base.match_token(ParenExpr_EOF,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- e ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum EContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	AddContext(AddContext<'input, 'arena, Tok>),
	ColContext(ColContext<'input, 'arena, Tok>),
	ParensContext(ParensContext<'input, 'arena, Tok>),
	SubqueryContext(SubqueryContext<'input, 'arena, Tok>),
	RowConstructorContext(RowConstructorContext<'input, 'arena, Tok>),
	MulContext(MulContext<'input, 'arena, Tok>),
	NumContext(NumContext<'input, 'arena, Tok>),
	UnaryContext(UnaryContext<'input, 'arena, Tok>),
    Error(EContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { EContextAll { } { AddContext, ColContext, ParensContext, SubqueryContext, RowConstructorContext, MulContext, NumContext, UnaryContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { EContextAll { } { AddContext, ColContext, ParensContext, SubqueryContext, RowConstructorContext, MulContext, NumContext, UnaryContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { ParenExprParserNodeKind::EContextAll { AddContext, ColContext, ParensContext, SubqueryContext, RowConstructorContext, MulContext, NumContext, UnaryContext, Error, } }
dbt_antlr4::impl_node_inner! { ParenExprParserNodeKind::EContext::EContextAll { AddContext, ColContext, ParensContext, SubqueryContext, RowConstructorContext, MulContext, NumContext, UnaryContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { ParenExprListener::ParenExprParserNodeKind::EContextAll { AddContext(enter_add, exit_add), ColContext(enter_col, exit_col), ParensContext(enter_parens, exit_parens), SubqueryContext(enter_subquery, exit_subquery), RowConstructorContext(enter_rowConstructor, exit_rowConstructor), MulContext(enter_mul, exit_mul), NumContext(enter_num, exit_num), UnaryContext(enter_unary, exit_unary), } }

impl<'input, 'arena, Tok> Deref for EContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn EContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use EContextAll::*;
		match self{
			AddContext(inner) => inner,
			ColContext(inner) => inner,
			ParensContext(inner) => inner,
			SubqueryContext(inner) => inner,
			RowConstructorContext(inner) => inner,
			MulContext(inner) => inner,
			NumContext(inner) => inner,
			UnaryContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type EContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, EContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct EContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for EContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::EContext }
	fn get_rule_index(&self) -> usize { RULE_e }
    fn make_node(
        arena: &'arena Arena,
        ctx: EContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(EContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a EContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => EContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut EContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut EContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> EContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ParenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ParenExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, EContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait EContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> EContextAttrs<'input, 'arena, Tok> for EContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type AddContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AddContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait AddContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
	fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AddContextAttrs<'input, 'arena, Tok> for AddContext<'input, 'arena, Tok>
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct AddContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for AddContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::EContext }
	fn get_rule_index(&self) -> usize { RULE_e }
    fn make_node(
        arena: &'arena Arena,
        ctx: AddContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(EContextAll::AddContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a AddContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => EContextAll<'input, 'arena, Tok>) {
                EContextAll::AddContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut AddContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut EContextAll<'input, 'arena, Tok>) {
                EContextAll::AddContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for AddContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AddContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::AddContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type ColContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ColContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait ColContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ColContextAttrs<'input, 'arena, Tok> for ColContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_ID)
    }
}
#[derive(Debug)]
pub struct ColContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for ColContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::EContext }
	fn get_rule_index(&self) -> usize { RULE_e }
    fn make_node(
        arena: &'arena Arena,
        ctx: ColContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(EContextAll::ColContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ColContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => EContextAll<'input, 'arena, Tok>) {
                EContextAll::ColContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ColContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut EContextAll<'input, 'arena, Tok>) {
                EContextAll::ColContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for ColContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ColContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::ColContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type ParensContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ParensContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait ParensContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ParensContextAttrs<'input, 'arena, Tok> for ParensContext<'input, 'arena, Tok>
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct ParensContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for ParensContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::EContext }
	fn get_rule_index(&self) -> usize { RULE_e }
    fn make_node(
        arena: &'arena Arena,
        ctx: ParensContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(EContextAll::ParensContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ParensContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => EContextAll<'input, 'arena, Tok>) {
                EContextAll::ParensContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ParensContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut EContextAll<'input, 'arena, Tok>) {
                EContextAll::ParensContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for ParensContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ParensContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::ParensContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type SubqueryContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, SubqueryContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait SubqueryContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SubqueryContextAttrs<'input, 'arena, Tok> for SubqueryContext<'input, 'arena, Tok>
{
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct SubqueryContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for SubqueryContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::EContext }
	fn get_rule_index(&self) -> usize { RULE_e }
    fn make_node(
        arena: &'arena Arena,
        ctx: SubqueryContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(EContextAll::SubqueryContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a SubqueryContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => EContextAll<'input, 'arena, Tok>) {
                EContextAll::SubqueryContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut SubqueryContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut EContextAll<'input, 'arena, Tok>) {
                EContextAll::SubqueryContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for SubqueryContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SubqueryContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::SubqueryContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type RowConstructorContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RowConstructorContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait RowConstructorContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
	fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RowConstructorContextAttrs<'input, 'arena, Tok> for RowConstructorContext<'input, 'arena, Tok>
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct RowConstructorContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for RowConstructorContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::EContext }
	fn get_rule_index(&self) -> usize { RULE_e }
    fn make_node(
        arena: &'arena Arena,
        ctx: RowConstructorContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(EContextAll::RowConstructorContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RowConstructorContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => EContextAll<'input, 'arena, Tok>) {
                EContextAll::RowConstructorContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RowConstructorContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut EContextAll<'input, 'arena, Tok>) {
                EContextAll::RowConstructorContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for RowConstructorContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RowConstructorContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::RowConstructorContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type MulContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, MulContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait MulContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
	fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> MulContextAttrs<'input, 'arena, Tok> for MulContext<'input, 'arena, Tok>
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct MulContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for MulContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::EContext }
	fn get_rule_index(&self) -> usize { RULE_e }
    fn make_node(
        arena: &'arena Arena,
        ctx: MulContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(EContextAll::MulContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a MulContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => EContextAll<'input, 'arena, Tok>) {
                EContextAll::MulContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut MulContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut EContextAll<'input, 'arena, Tok>) {
                EContextAll::MulContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for MulContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> MulContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::MulContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type NumContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NumContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait NumContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token INT
	/// Returns `None` if there is no child corresponding to token INT
	fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NumContextAttrs<'input, 'arena, Tok> for NumContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_INT)
    }
}
#[derive(Debug)]
pub struct NumContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for NumContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::EContext }
	fn get_rule_index(&self) -> usize { RULE_e }
    fn make_node(
        arena: &'arena Arena,
        ctx: NumContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(EContextAll::NumContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a NumContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => EContextAll<'input, 'arena, Tok>) {
                EContextAll::NumContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut NumContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut EContextAll<'input, 'arena, Tok>) {
                EContextAll::NumContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for NumContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NumContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::NumContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type UnaryContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, UnaryContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait UnaryContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> UnaryContextAttrs<'input, 'arena, Tok> for UnaryContext<'input, 'arena, Tok>
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct UnaryContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for UnaryContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::EContext }
	fn get_rule_index(&self) -> usize { RULE_e }
    fn make_node(
        arena: &'arena Arena,
        ctx: UnaryContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(EContextAll::UnaryContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a UnaryContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => EContextAll<'input, 'arena, Tok>) {
                EContextAll::UnaryContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut UnaryContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut EContextAll<'input, 'arena, Tok>) {
                EContextAll::UnaryContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for UnaryContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> UnaryContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::UnaryContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  e(&mut self,) -> Result<&'arena EContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.e_rec(0)
	}

	fn e_rec(&mut self, _p: i32) -> Result<&'arena EContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		if let Some(_node) = self.base.resume_take(RULE_e)? {
		    return Ok(_node.as_rule_context().unwrap());
		}
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
		let _parentctx = recog.base.take_ctx();
		let _parentState = recog.base.get_state();
		recog.base.enter_recursion_rule(EContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 2, RULE_e, _p)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena EContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
	        let mut _alt: i32;
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(32);
			recog.err_handler.sync(&mut recog.base)?;
			match { let _m = recog.base.dfa_predict_mask(1)?;
			  match _m {
			    0 => 1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(1,&mut recog.base)? - 1) as u32),
			    x if x != 0 && (x & !0x1c) == 0 && (x & (x-1)) != 0 && !recog.base.resume_active() => {
			        recog.base.set_state(13);
			        let _neutral_state = recog.base.begin_neutral_parse(1);
			        match recog.e() {
			            Err(e) if !e.is_recoverable() => {
			                    recog.base.end_neutral_parse(&_neutral_state);
			                    return Err(e)
			                },
			            Err(_) => {
			                recog.base.end_neutral_parse(&_neutral_state);
			                1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(1,&mut recog.base)? - 1) as u32)
			            },
			            Ok(_node) => {
			                if recog.base.syntax_error_count() != _neutral_state.2 {
			                    recog.base.end_neutral_parse(&_neutral_state);
			                    // errorful neutral parse: throw its result away and
			                    // defer (muted, so no spurious reports escaped)
			                    1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(1,&mut recog.base)? - 1) as u32)
			                }
			                else {
			                    // the tail token decides on the post-prefix stream.
			                    // An explicit arm (or the static default) rewinds into
			                    // resume mode; anything else rewinds and defers to the
			                    // adaptive engine from the decision start - no resume,
			                    // the chosen body re-parses everything
			                    let _tailbit = match recog.input.la(1) {
			                        ParenExpr_T__1 => Some(0x4),
			                        ParenExpr_T__2 => Some(0x8),
			                        _ => None,
			                    };
			                    match _tailbit {
			                        Some(bit) => {
			                            recog.base.start_resume(dbt_antlr4::tree::NodeInner::as_node(_node), RULE_e);
			                            recog.base.end_neutral_parse(&_neutral_state);
			                            bit
			                        }
			                        None => {
			                            recog.base.end_neutral_parse(&_neutral_state);
			                            1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(1,&mut recog.base)? - 1) as u32)
			                        }
			                    }
			                }
			            },
			        }
			    },
			    x if x != 0 && (x & !0x1c) == 0 && (x & (x-1)) != 0 => 1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(1,&mut recog.base)? - 1) as u32),
			    x if (x & (x-1)) != 0 => 1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(1,&mut recog.base)? - 1) as u32),
			    m => m,
			  } } {
			x if x == 0x1 =>{
				{
				recog.base.with_mut_ctx(|ctx| { NumContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena NumContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

				recog.base.set_state(10);
				recog.base.match_token(ParenExpr_INT,&mut recog.err_handler)?;
				}
			},
			x if x == 0x2 =>{
				{
				recog.base.with_mut_ctx(|ctx| { ColContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena ColContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(11);
				recog.base.match_token(ParenExpr_ID,&mut recog.err_handler)?;
				}
			},
			x if x == 0x4 =>{
				{
				recog.base.with_mut_ctx(|ctx| { RowConstructorContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena RowConstructorContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(12);
				recog.base.match_token(ParenExpr_T__0,&mut recog.err_handler)?;
				/*InvokeRule e*/
				recog.base.set_state(13);
				recog.e_rec(0)?;
				recog.base.set_state(16); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				loop {
					{
					{
					recog.base.set_state(14);
					recog.base.match_token(ParenExpr_T__1,&mut recog.err_handler)?;
					/*InvokeRule e*/
					recog.base.set_state(15);
					recog.e_rec(0)?;
					}
					}
					recog.base.set_state(18); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if !(_la==ParenExpr_T__1) {break}
				}
				recog.base.set_state(20);
				recog.base.match_token(ParenExpr_T__2,&mut recog.err_handler)?;
				}
			},
			x if x == 0x8 =>{
				{
				recog.base.with_mut_ctx(|ctx| { ParensContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena ParensContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(22);
				recog.base.match_token(ParenExpr_T__0,&mut recog.err_handler)?;
				/*InvokeRule e*/
				recog.base.set_state(23);
				recog.e_rec(0)?;
				recog.base.set_state(24);
				recog.base.match_token(ParenExpr_T__2,&mut recog.err_handler)?;
				}
			},
			x if x == 0x10 =>{
				{
				recog.base.with_mut_ctx(|ctx| { SubqueryContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena SubqueryContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(26);
				recog.base.match_token(ParenExpr_T__0,&mut recog.err_handler)?;
				/*InvokeRule q*/
				recog.base.set_state(27);
				recog.q()?;
				recog.base.set_state(28);
				recog.base.match_token(ParenExpr_T__2,&mut recog.err_handler)?;
				}
			},
			x if x == 0x20 =>{
				{
				recog.base.with_mut_ctx(|ctx| { UnaryContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena UnaryContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(30);
				recog.base.match_token(ParenExpr_T__3,&mut recog.err_handler)?;
				/*InvokeRule e*/
				recog.base.set_state(31);
				recog.e_rec(3)?;
				}
			}
				_ => {}
			}
			let tmp = recog.input.lt(-1);
			recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
			recog.base.set_state(42);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(3)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(3,&mut recog.base)? } else { _sdp } };
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					{
					recog.base.set_state(40);
					recog.err_handler.sync(&mut recog.base)?;
					match { let _sdp = recog.base.dfa_predict(2)?; _sdp } {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
							MulContextExt::copy_from(tmp);
							let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
							let _local_ctx_fn = |recog: &Self| -> &'arena MulContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

							recog.base.set_state(34);
							if !({recog.precpred(None, 2)}) {
								Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(35);
							_la = recog.base.input.la(1);
							if { !(_la==ParenExpr_T__4 || _la==ParenExpr_T__5) } {
								recog.err_handler.recover_inline(&mut recog.base)?;
							}
							else {
								if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler)?;
							}
							/*InvokeRule e*/
							recog.base.set_state(36);
							recog.e_rec(3)?;
							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
							AddContextExt::copy_from(tmp);
							let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
							let _local_ctx_fn = |recog: &Self| -> &'arena AddContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

							recog.base.set_state(37);
							if !({recog.precpred(None, 1)}) {
								Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(38);
							_la = recog.base.input.la(1);
							if { !(_la==ParenExpr_T__3 || _la==ParenExpr_T__6) } {
								recog.err_handler.recover_inline(&mut recog.base)?;
							}
							else {
								if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler)?;
							}
							/*InvokeRule e*/
							recog.base.set_state(39);
							recog.e_rec(2)?;
							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(44);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = { let _sdp = recog.base.dfa_predict(3)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(3,&mut recog.base)? } else { _sdp } };
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e) if !e.is_recoverable() => return Err(e),
		Err(ref re)=>{
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx).map(|ctx| { ctx.as_rule_context().unwrap() } )
        })
	}
}
//------------------- q ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum QContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	SelectContext(SelectContext<'input, 'arena, Tok>),
	NestedContext(NestedContext<'input, 'arena, Tok>),
	TableContext(TableContext<'input, 'arena, Tok>),
    Error(QContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { QContextAll { } { SelectContext, NestedContext, TableContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { QContextAll { } { SelectContext, NestedContext, TableContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { ParenExprParserNodeKind::QContextAll { SelectContext, NestedContext, TableContext, Error, } }
dbt_antlr4::impl_node_inner! { ParenExprParserNodeKind::QContext::QContextAll { SelectContext, NestedContext, TableContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { ParenExprListener::ParenExprParserNodeKind::QContextAll { SelectContext(enter_select, exit_select), NestedContext(enter_nested, exit_nested), TableContext(enter_table, exit_table), } }

impl<'input, 'arena, Tok> Deref for QContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn QContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use QContextAll::*;
		match self{
			SelectContext(inner) => inner,
			NestedContext(inner) => inner,
			TableContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type QContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, QContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct QContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for QContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::QContext }
	fn get_rule_index(&self) -> usize { RULE_q }
    fn make_node(
        arena: &'arena Arena,
        ctx: QContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(QContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a QContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => QContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut QContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut QContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> QContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ParenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ParenExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, QContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait QContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> QContextAttrs<'input, 'arena, Tok> for QContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type SelectContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, SelectContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait SelectContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SelectContextAttrs<'input, 'arena, Tok> for SelectContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_ID)
    }
}
#[derive(Debug)]
pub struct SelectContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: QContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for SelectContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::QContext }
	fn get_rule_index(&self) -> usize { RULE_q }
    fn make_node(
        arena: &'arena Arena,
        ctx: SelectContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(QContextAll::SelectContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a SelectContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => QContextAll<'input, 'arena, Tok>) {
                QContextAll::SelectContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut SelectContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut QContextAll<'input, 'arena, Tok>) {
                QContextAll::SelectContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> QContextAttrs<'input, 'arena, Tok> for SelectContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SelectContextExt<'input, 'arena, Tok> {
	fn new(base: QContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut QContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            QContextAll::SelectContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut QContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type NestedContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NestedContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait NestedContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NestedContextAttrs<'input, 'arena, Tok> for NestedContext<'input, 'arena, Tok>
{
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct NestedContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: QContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for NestedContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::QContext }
	fn get_rule_index(&self) -> usize { RULE_q }
    fn make_node(
        arena: &'arena Arena,
        ctx: NestedContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(QContextAll::NestedContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a NestedContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => QContextAll<'input, 'arena, Tok>) {
                QContextAll::NestedContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut NestedContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut QContextAll<'input, 'arena, Tok>) {
                QContextAll::NestedContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> QContextAttrs<'input, 'arena, Tok> for NestedContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NestedContextExt<'input, 'arena, Tok> {
	fn new(base: QContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut QContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            QContextAll::NestedContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut QContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type TableContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, TableContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait TableContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TableContextAttrs<'input, 'arena, Tok> for TableContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_ID)
    }
}
#[derive(Debug)]
pub struct TableContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: QContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for TableContextExt<'input, 'arena, Tok>
{
	type NodeKind = ParenExprParserNodeKind;
    fn node_tag() -> ParenExprParserNodeKind { ParenExprParserNodeKind::QContext }
	fn get_rule_index(&self) -> usize { RULE_q }
    fn make_node(
        arena: &'arena Arena,
        ctx: TableContext<'input, 'arena, Tok>,
    ) -> *mut ParenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(QContextAll::TableContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a TableContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => QContextAll<'input, 'arena, Tok>) {
                QContextAll::TableContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ParenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut TableContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut QContextAll<'input, 'arena, Tok>) {
                QContextAll::TableContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> QContextAttrs<'input, 'arena, Tok> for TableContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TableContextExt<'input, 'arena, Tok> {
	fn new(base: QContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut QContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            QContextAll::TableContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        // replace + forget, NOT assignment: the new value's children Vec
        // was moved out of the old one, so dropping the old value would
        // deallocate the buffer the new value references (the arena can
        // then hand it to another node's Vec - shared-descent resume
        // paths hit this: the decision ctx's Vec buffer is the most
        // recent arena allocation when copy_from runs).
        let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut QContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn q(&mut self,) -> Result<&'arena QContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(QContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 4, RULE_q)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena QContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(52);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ParenExpr_T__7  => {
			        /*------- Outer Most Alt 1 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            SelectContextExt::copy_from(ctx);
			            ctx.set_alt_number(1);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena SelectContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(45);
			        recog.base.match_token(ParenExpr_T__7,&mut recog.err_handler)?;
			        recog.base.set_state(46);
			        recog.base.match_token(ParenExpr_ID,&mut recog.err_handler)?;
			        }}
			    ParenExpr_ID  => {
			        /*------- Outer Most Alt 2 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            TableContextExt::copy_from(ctx);
			            ctx.set_alt_number(2);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena TableContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(47);
			        recog.base.match_token(ParenExpr_ID,&mut recog.err_handler)?;
			        }}
			    ParenExpr_T__0  => {
			        /*------- Outer Most Alt 3 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            NestedContextExt::copy_from(ctx);
			            ctx.set_alt_number(3);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena NestedContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(48);
			        recog.base.match_token(ParenExpr_T__0,&mut recog.err_handler)?;
			        /*InvokeRule q*/
			        recog.base.set_state(49);
			        recog.q()?;
			        recog.base.set_state(50);
			        recog.base.match_token(ParenExpr_T__2,&mut recog.err_handler)?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}

// the serialized ATN is followed by 5 static SLL prediction tables (-Xstatic-dfa):
//   decision 3: precedence-dispatched over cutoffs [1, 2], tables [2 3 4]
//   decision 1: LL(k), k=5, 34 states, 16 mask accepts
//   decision 2: LL(k), k=1, 3 states
//   table 2 (decision 3): LL(k), k=1, 5 states
//   table 3 (decision 3): LL(k), k=1, 6 states
//   table 4 (decision 3): LL(k), k=0, 1 states
static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 17] = [
    "CAIWdAQADgAEAg4CBAQOBAIAAgACAAICAgICAgICAgICAgICCAIiEAIWAhgCJAICAgICAgICAgICAgIC",
    "AgICAgICAgICAgYCQhACAgICAgICAgICAgICCgJSEAIUAhgCWBICAgQCBAIEAgQCBAIEAgQGBGoQBAIE",
    "BgJwEAICAgACBAYABAgABAIACgwEAAgIDg6AAQAMAgAAAARAAgAAAAhoAgAAAAwOBgQCAA4QCgAAAhAC",
    "AgAAABIUDAIBABRCChQAABZCChIAABgaCgIAABogBgQCABweCgQAAB4iBgQCACAcAgAAACIkAgAAACQg",
    "AgAAACQmAgAAACYoAgAAACgqCgYAACpCAgAAACwuCgIAAC4wBgQCADAyCgYAADJCAgAAADQ2CgIAADY4",
    "BggEADg6CgYAADpCAgAAADw+CggAAD5CBgQCBkASAgAAAEAWAgAAAEAYAgAAAEAsAgAAAEA0AgAAAEA8",
    "AgAAAEJUAgAAAERGFAQAAEZIDgAAAEhSBgQCBkpMFAIAAExODgIAAE5SBgQCBFBEAgAAAFBKAgAAAFJY",
    "AgAAAFRQAgAAAFRWAgAAAFYGAgAAAFhUAgAAAFpcChAAAFxqChIAAF5qChIAAGBiCgIAAGJkBggEAGRm",
    "CgYAAGZqAgAAAGhaAgAAAGheAgAAAGhgAgAAAGoKAgAAAG4gAgAAAG4wAgAAAHJuBgQCAAwkQFBUaG4O",
    "CggCRPYBAAAMBAIAAAoAAAAAAAAAAAAGAAAIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAAAAAAAGDY2NjZUZmZ4igGKAZwBogG0AcYB5AHkAeQB5AH2AfYB9gH2AfYB9gH2AfYB",
    "9gH2AfYB9gH2AfYB9gECAgIICAQSEgYUFAgCAgoICAwQEA4SEhAUFBICAhQICBYQEBgSEhoUFBwCAh4I",
    "CCASFBIEBCIGBiQIDiYEBCIGBigIDiYCAioICCASFBwSEiwEBC4GBjAIDjIEBC4GBjQIDjICAjYICCAQ",
    "EDgSEjoUFDwCAj4ICEASFEIgFAYGCAogBAYIJAQICioEBggsBgYICi4EBggwBgYICjIEBgg0BAYINgQG",
    "CDgEBgg6BAYIPAQGCD4EBghABAYIQgQGCAAAAAQGEgAEAgAAAAASEhIICAIKDAQODgIAAAAAAQoYAAQE",
    "BAIEAAAAAAAYGBgYGAEBAgQEBAYGBggOCAAAAAABDCQABAQEBAIEAAAAAAAAJCQkJCQkAQECBAQEBgYG",
    "CAgICgwKDg4IAAAAAAECAAQAAAAAAAAAAgYEAgQEBgg="
];