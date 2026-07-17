// Generated from BetweenExpr.g4 by ANTLR 4.13.2
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
use super::betweenexprlistener::*;
use std::marker::PhantomData;
use std::sync::LazyLock;
use std::rc::Rc;
use std::ops::{DerefMut, Deref};

dbt_antlr4::check_version!("2","0");
pub const BetweenExpr_T__0:i32=1; 
pub const BetweenExpr_T__1:i32=2; 
pub const BetweenExpr_T__2:i32=3; 
pub const BetweenExpr_T__3:i32=4; 
pub const BetweenExpr_T__4:i32=5; 
pub const BetweenExpr_T__5:i32=6; 
pub const BetweenExpr_T__6:i32=7; 
pub const BetweenExpr_T__7:i32=8; 
pub const BetweenExpr_T__8:i32=9; 
pub const BetweenExpr_T__9:i32=10; 
pub const BetweenExpr_T__10:i32=11; 
pub const BetweenExpr_T__11:i32=12; 
pub const BetweenExpr_T__12:i32=13; 
pub const BetweenExpr_T__13:i32=14; 
pub const BetweenExpr_ID:i32=15; 
pub const BetweenExpr_INT:i32=16; 
pub const BetweenExpr_WS:i32=17;
pub const BetweenExpr_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_b:usize = 1; 
pub const RULE_pred:usize = 2; 
pub const RULE_v:usize = 3; 
pub const RULE_atom:usize = 4;
pub const ruleNames: [&'static str; 5] = [
    "s", "b", "pred", "v", "atom"
];

pub const _LITERAL_NAMES: [Option<&'static str>;15] = [
	None, Some("'NOT'"), Some("'AND'"), Some("'OR'"), Some("'BETWEEN'"), Some("'='"), 
	Some("'<'"), Some("'>'"), Some("'*'"), Some("'/'"), Some("'+'"), Some("'-'"), 
	Some("'('"), Some("')'"), Some("'->'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;18]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, BetweenExprParserExt<'input, 'arena>, BetweenExprParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct BetweenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> BetweenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				BetweenExprParserExt {
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
        L: BetweenExprListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct BetweenExprTreeWalker;
impl BetweenExprTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: BetweenExprListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, BetweenExprParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <BetweenExprParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum BetweenExprParserNodeKind {
    SContext,
    BContext,
    PredContext,
    VContext,
    AtomContext,
    Terminal,
    Error,
}
pub type BetweenExprParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, BetweenExprParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => BetweenExprParser }
dbt_antlr4::impl_node_kind! { BetweenExprParserNodeKind {
    BContext(BContextAll), PredContext(PredContextAll), ; SContext(enter_s, exit_s, ), VContext(enter_v, exit_v, ), AtomContext(enter_atom, exit_atom, ), 
    }; listener = dyn BetweenExprListener<'arena, Tok>,
}

pub struct BetweenExprParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> BetweenExprParserExt<'input, 'arena> {
}

impl<'input, 'arena, Input, TF> ParserRecog<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for BetweenExprParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena {
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }        
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for BetweenExprParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn get_grammar_file_name(&self) -> & str{ "BetweenExpr.g4" }
   	fn get_rule_names(&self) -> &[& str] { &ruleNames }
   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&'arena BetweenExprParserNode<'input, 'arena, TF::Tok>>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input, 'arena, Input, TF>
	) -> bool {
		match rule_index {
		    1 => BetweenExprParser::<'input, 'arena, Input, TF>::b_sempred(_localctx.and_then(|x| x.as_rule_context()), pred_index, recog),
		    3 => BetweenExprParser::<'input, 'arena, Input, TF>::v_sempred(_localctx.and_then(|x| x.as_rule_context()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, 'arena, Input, TF> BetweenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn b_sempred(_ctx: Option<&'arena BContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
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

	fn v_sempred(_ctx: Option<&'arena VContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        2 => {
			recog.precpred(None, 3)
		    }
	        3 => {
			recog.precpred(None, 2)
		    }
		    _ => true
		}
	}
}
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

pub type SContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, SContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct SContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for SContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::SContext }
	fn get_rule_index(&self) -> usize { RULE_s }
    fn make_node(
        arena: &'arena Arena,
        ctx: SContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a SContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => SContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut SContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut SContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena BetweenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut BetweenExprParserNode<'input, 'arena, Tok>, ANTLRError>
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
    fn b(&self) -> Option<&'arena BContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SContextAttrs<'input, 'arena, Tok> for SContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn b(&self) -> Option<&'arena BContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == BetweenExpr_EOF)
    }
}

impl<'input, 'arena, Input, TF> BetweenExprParser<'input, 'arena, Input, TF>
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
			/*InvokeRule b*/
			recog.base.set_state(10);
			recog.b_rec(0)?;
			recog.base.set_state(11);
			recog.base.match_token(BetweenExpr_EOF,&mut recog.err_handler)?;
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
//------------------- b ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum BContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	NotContext(NotContext<'input, 'arena, Tok>),
	PredicatedContext(PredicatedContext<'input, 'arena, Tok>),
	OrContext(OrContext<'input, 'arena, Tok>),
	AndContext(AndContext<'input, 'arena, Tok>),
    Error(BContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { BContextAll { } { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { BContextAll { } { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { BetweenExprParserNodeKind::BContextAll { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr4::impl_node_inner! { BetweenExprParserNodeKind::BContext::BContextAll { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { BetweenExprListener::BetweenExprParserNodeKind::BContextAll { NotContext(enter_not, exit_not), PredicatedContext(enter_predicated, exit_predicated), OrContext(enter_or, exit_or), AndContext(enter_and, exit_and), } }

impl<'input, 'arena, Tok> Deref for BContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn BContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use BContextAll::*;
		match self{
			NotContext(inner) => inner,
			PredicatedContext(inner) => inner,
			OrContext(inner) => inner,
			AndContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type BContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, BContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct BContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for BContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::BContext }
	fn get_rule_index(&self) -> usize { RULE_b }
    fn make_node(
        arena: &'arena Arena,
        ctx: BContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(BContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a BContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => BContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut BContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut BContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> BContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena BetweenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut BetweenExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, BContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait BContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> BContextAttrs<'input, 'arena, Tok> for BContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type NotContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NotContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;

pub trait NotContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn b(&self) -> Option<&'arena BContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NotContextAttrs<'input, 'arena, Tok> for NotContext<'input, 'arena, Tok>
{
    fn b(&self) -> Option<&'arena BContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct NotContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: BContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for NotContextExt<'input, 'arena, Tok>
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::BContext }
	fn get_rule_index(&self) -> usize { RULE_b }
    fn make_node(
        arena: &'arena Arena,
        ctx: NotContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(BContextAll::NotContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a NotContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => BContextAll<'input, 'arena, Tok>) {
                BContextAll::NotContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut NotContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut BContextAll<'input, 'arena, Tok>) {
                BContextAll::NotContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> BContextAttrs<'input, 'arena, Tok> for NotContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NotContextExt<'input, 'arena, Tok> {
	fn new(base: BContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut BetweenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut BContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            BContextAll::NotContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut BContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type PredicatedContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PredicatedContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;

pub trait PredicatedContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn v(&self) -> Option<&'arena VContextAll<'input, 'arena, Tok>>;
	fn pred(&self) -> Option<&'arena PredContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PredicatedContextAttrs<'input, 'arena, Tok> for PredicatedContext<'input, 'arena, Tok>
{
    fn v(&self) -> Option<&'arena VContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn pred(&self) -> Option<&'arena PredContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct PredicatedContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: BContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for PredicatedContextExt<'input, 'arena, Tok>
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::BContext }
	fn get_rule_index(&self) -> usize { RULE_b }
    fn make_node(
        arena: &'arena Arena,
        ctx: PredicatedContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(BContextAll::PredicatedContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a PredicatedContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => BContextAll<'input, 'arena, Tok>) {
                BContextAll::PredicatedContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut PredicatedContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut BContextAll<'input, 'arena, Tok>) {
                BContextAll::PredicatedContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> BContextAttrs<'input, 'arena, Tok> for PredicatedContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PredicatedContextExt<'input, 'arena, Tok> {
	fn new(base: BContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut BetweenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut BContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            BContextAll::PredicatedContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut BContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type OrContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, OrContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;

pub trait OrContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn b_all(&self) -> Vec<&'arena BContextAll<'input, 'arena, Tok>>;
	fn b(&self, i: usize) -> Option<&'arena BContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> OrContextAttrs<'input, 'arena, Tok> for OrContext<'input, 'arena, Tok>
{
    fn b_all(&self) -> Vec<&'arena BContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn b(&self, i: usize) -> Option<&'arena BContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct OrContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: BContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for OrContextExt<'input, 'arena, Tok>
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::BContext }
	fn get_rule_index(&self) -> usize { RULE_b }
    fn make_node(
        arena: &'arena Arena,
        ctx: OrContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(BContextAll::OrContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a OrContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => BContextAll<'input, 'arena, Tok>) {
                BContextAll::OrContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut OrContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut BContextAll<'input, 'arena, Tok>) {
                BContextAll::OrContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> BContextAttrs<'input, 'arena, Tok> for OrContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> OrContextExt<'input, 'arena, Tok> {
	fn new(base: BContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut BetweenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut BContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            BContextAll::OrContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut BContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type AndContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AndContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;

pub trait AndContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn b_all(&self) -> Vec<&'arena BContextAll<'input, 'arena, Tok>>;
	fn b(&self, i: usize) -> Option<&'arena BContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AndContextAttrs<'input, 'arena, Tok> for AndContext<'input, 'arena, Tok>
{
    fn b_all(&self) -> Vec<&'arena BContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn b(&self, i: usize) -> Option<&'arena BContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct AndContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: BContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for AndContextExt<'input, 'arena, Tok>
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::BContext }
	fn get_rule_index(&self) -> usize { RULE_b }
    fn make_node(
        arena: &'arena Arena,
        ctx: AndContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(BContextAll::AndContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a AndContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => BContextAll<'input, 'arena, Tok>) {
                BContextAll::AndContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut AndContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut BContextAll<'input, 'arena, Tok>) {
                BContextAll::AndContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> BContextAttrs<'input, 'arena, Tok> for AndContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AndContextExt<'input, 'arena, Tok> {
	fn new(base: BContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut BetweenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut BContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            BContextAll::AndContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut BContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> BetweenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  b(&mut self,) -> Result<&'arena BContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.b_rec(0)
	}

	fn b_rec(&mut self, _p: i32) -> Result<&'arena BContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
		let _parentctx = recog.base.take_ctx();
		let _parentState = recog.base.get_state();
		recog.base.enter_recursion_rule(BContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 2, RULE_b, _p)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena BContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
		let result: Result<(), ANTLRError> = (|| {
	        let mut _alt: i32;
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(20);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    BetweenExpr_T__11 |BetweenExpr_ID |BetweenExpr_INT  => {
			        {
			        recog.base.with_mut_ctx(|ctx| { PredicatedContextExt::copy_from(ctx); });
			        let _local_ctx_fn = |recog: &Self| -> &'arena PredicatedContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

			        /*InvokeRule v*/
			        recog.base.set_state(14);
			        recog.v_rec(0)?;
			        recog.base.set_state(16);
			        recog.err_handler.sync(&mut recog.base)?;
			        match { let _sdp = recog.base.dfa_predict(0)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(0,&mut recog.base)? } else { _sdp } } {
			        	x if x == 1 =>{
			        		{
			        		/*InvokeRule pred*/
			        		recog.base.set_state(15);
			        		recog.pred()?;
			        		}
			        	}

			        	_ => {}
			        }
			        }}
			    BetweenExpr_T__0  => {
			        {
			        recog.base.with_mut_ctx(|ctx| { NotContextExt::copy_from(ctx); });
			        let _local_ctx_fn = |recog: &Self| -> &'arena NotContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        recog.base.set_state(18);
			        recog.base.match_token(BetweenExpr_T__0,&mut recog.err_handler)?;
			        /*InvokeRule b*/
			        recog.base.set_state(19);
			        recog.b_rec(3)?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			let tmp = recog.input.lt(-1);
			recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
			recog.base.set_state(30);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(3)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(3,&mut recog.base)? } else { _sdp } };
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					{
					recog.base.set_state(28);
					recog.err_handler.sync(&mut recog.base)?;
					match { let _sdp = recog.base.dfa_predict(2)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(2,&mut recog.base)? } else { _sdp } } {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let tmp = BContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
							AndContextExt::copy_from(tmp);
							let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_b)?;
							let _local_ctx_fn = |recog: &Self| -> &'arena AndContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

							recog.base.set_state(22);
							if !({recog.precpred(None, 2)}) {
								Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(23);
							recog.base.match_token(BetweenExpr_T__1,&mut recog.err_handler)?;
							/*InvokeRule b*/
							recog.base.set_state(24);
							recog.b_rec(3)?;
							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let tmp = BContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
							OrContextExt::copy_from(tmp);
							let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_b)?;
							let _local_ctx_fn = |recog: &Self| -> &'arena OrContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

							recog.base.set_state(25);
							if !({recog.precpred(None, 1)}) {
								Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(26);
							recog.base.match_token(BetweenExpr_T__2,&mut recog.err_handler)?;
							/*InvokeRule b*/
							recog.base.set_state(27);
							recog.b_rec(2)?;
							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(32);
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
//------------------- pred ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum PredContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	CmpContext(CmpContext<'input, 'arena, Tok>),
	BetweenContext(BetweenContext<'input, 'arena, Tok>),
    Error(PredContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { PredContextAll { } { CmpContext, BetweenContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { PredContextAll { } { CmpContext, BetweenContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { BetweenExprParserNodeKind::PredContextAll { CmpContext, BetweenContext, Error, } }
dbt_antlr4::impl_node_inner! { BetweenExprParserNodeKind::PredContext::PredContextAll { CmpContext, BetweenContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { BetweenExprListener::BetweenExprParserNodeKind::PredContextAll { CmpContext(enter_cmp, exit_cmp), BetweenContext(enter_between, exit_between), } }

impl<'input, 'arena, Tok> Deref for PredContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn PredContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use PredContextAll::*;
		match self{
			CmpContext(inner) => inner,
			BetweenContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type PredContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PredContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct PredContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for PredContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::PredContext }
	fn get_rule_index(&self) -> usize { RULE_pred }
    fn make_node(
        arena: &'arena Arena,
        ctx: PredContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(PredContextAll::Error(ctx))
    }
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a PredContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => PredContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut PredContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut PredContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PredContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena BetweenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut BetweenExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, PredContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait PredContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> PredContextAttrs<'input, 'arena, Tok> for PredContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type CmpContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, CmpContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;

pub trait CmpContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn v(&self) -> Option<&'arena VContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CmpContextAttrs<'input, 'arena, Tok> for CmpContext<'input, 'arena, Tok>
{
    fn v(&self) -> Option<&'arena VContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct CmpContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: PredContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for CmpContextExt<'input, 'arena, Tok>
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::PredContext }
	fn get_rule_index(&self) -> usize { RULE_pred }
    fn make_node(
        arena: &'arena Arena,
        ctx: CmpContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(PredContextAll::CmpContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a CmpContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => PredContextAll<'input, 'arena, Tok>) {
                PredContextAll::CmpContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut CmpContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut PredContextAll<'input, 'arena, Tok>) {
                PredContextAll::CmpContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> PredContextAttrs<'input, 'arena, Tok> for CmpContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CmpContextExt<'input, 'arena, Tok> {
	fn new(base: PredContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut BetweenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut PredContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            PredContextAll::CmpContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut PredContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type BetweenContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, BetweenContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;

pub trait BetweenContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn v_all(&self) -> Vec<&'arena VContextAll<'input, 'arena, Tok>>;
	fn v(&self, i: usize) -> Option<&'arena VContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> BetweenContextAttrs<'input, 'arena, Tok> for BetweenContext<'input, 'arena, Tok>
{
    fn v_all(&self) -> Vec<&'arena VContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn v(&self, i: usize) -> Option<&'arena VContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct BetweenContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: PredContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CustomRuleContext<'input, 'arena, Tok> for BetweenContextExt<'input, 'arena, Tok>
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::PredContext }
	fn get_rule_index(&self) -> usize { RULE_pred }
    fn make_node(
        arena: &'arena Arena,
        ctx: BetweenContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_labeled_node(PredContextAll::BetweenContext(ctx))
    }
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a BetweenContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => PredContextAll<'input, 'arena, Tok>) {
                PredContextAll::BetweenContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut BetweenContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            match dbt_antlr4::cast_unchecked!(node => mut PredContextAll<'input, 'arena, Tok>) {
                PredContextAll::BetweenContext(ctx) => Some(ctx),
                _ => None
            }
        } else {
            None
        }
    }
}

impl<'input, 'arena, Tok> PredContextAttrs<'input, 'arena, Tok> for BetweenContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> BetweenContextExt<'input, 'arena, Tok> {
	fn new(base: PredContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut BetweenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut PredContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            PredContextAll::BetweenContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
        *dbt_antlr4::cast_unchecked!(src => mut PredContextAll<'input, 'arena, Tok>) = ctx;
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> BetweenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn pred(&mut self,) -> Result<&'arena PredContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(PredContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 4, RULE_pred)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena PredContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(43);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    BetweenExpr_T__0 |BetweenExpr_T__3  => {
			        /*------- Outer Most Alt 1 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            BetweenContextExt::copy_from(ctx);
			            ctx.set_alt_number(1);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena BetweenContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(34);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if _la==BetweenExpr_T__0 {
			        	{
			        	recog.base.set_state(33);
			        	recog.base.match_token(BetweenExpr_T__0,&mut recog.err_handler)?;
			        	}
			        }

			        recog.base.set_state(36);
			        recog.base.match_token(BetweenExpr_T__3,&mut recog.err_handler)?;
			        /*InvokeRule v*/
			        recog.base.set_state(37);
			        recog.v_rec(0)?;
			        recog.base.set_state(38);
			        recog.base.match_token(BetweenExpr_T__1,&mut recog.err_handler)?;
			        /*InvokeRule v*/
			        recog.base.set_state(39);
			        recog.v_rec(0)?;
			        }}
			    BetweenExpr_T__4 |BetweenExpr_T__5 |BetweenExpr_T__6  => {
			        /*------- Outer Most Alt 2 -------*/
			        recog.base.with_mut_ctx(|ctx| {
			            CmpContextExt::copy_from(ctx);
			            ctx.set_alt_number(2);
			        });
			        let _local_ctx_fn = |recog: &Self| -> &'arena CmpContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
			        {
			        recog.base.set_state(41);
			        _la = recog.base.input.la(1);
			        if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 224) != 0)) } {
			        	recog.err_handler.recover_inline(&mut recog.base)?;
			        }
			        else {
			        	if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
			        	recog.err_handler.report_match(&mut recog.base);
			        	recog.base.consume(&mut recog.err_handler)?;
			        }
			        /*InvokeRule v*/
			        recog.base.set_state(42);
			        recog.v_rec(0)?;
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
//------------------- v ----------------
pub type VContextAll<'input, 'arena, Tok = CommonToken<'input>> = VContext<'input, 'arena, Tok>;

pub type VContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, VContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct VContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for VContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::VContext }
	fn get_rule_index(&self) -> usize { RULE_v }
    fn make_node(
        arena: &'arena Arena,
        ctx: VContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a VContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => VContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut VContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut VContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> VContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena BetweenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut BetweenExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, VContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait VContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>>;
    fn v_all(&self) -> Vec<&'arena VContextAll<'input, 'arena, Tok>>;
    fn v(&self, i: usize) -> Option<&'arena VContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> VContextAttrs<'input, 'arena, Tok> for VContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn v_all(&self) -> Vec<&'arena VContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn v(&self, i: usize) -> Option<&'arena VContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> BetweenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  v(&mut self,) -> Result<&'arena VContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.v_rec(0)
	}

	fn v_rec(&mut self, _p: i32) -> Result<&'arena VContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
		let _parentctx = recog.base.take_ctx();
		let _parentState = recog.base.get_state();
		recog.base.enter_recursion_rule(VContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 6, RULE_v, _p)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena VContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 6;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
	        let mut _alt: i32;
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			{
			/*InvokeRule atom*/
			recog.base.set_state(46);
			recog.atom()?;
			}
			let tmp = recog.input.lt(-1);
			recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
			recog.base.set_state(56);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(7)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(7,&mut recog.base)? } else { _sdp } };
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					{
					recog.base.set_state(54);
					recog.err_handler.sync(&mut recog.base)?;
					match { let _sdp = recog.base.dfa_predict(6)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(6,&mut recog.base)? } else { _sdp } } {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let tmp = VContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
							let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_v)?;

							recog.base.set_state(48);
							if !({recog.precpred(None, 3)}) {
								Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(49);
							_la = recog.base.input.la(1);
							if { !(_la==BetweenExpr_T__7 || _la==BetweenExpr_T__8) } {
								recog.err_handler.recover_inline(&mut recog.base)?;
							}
							else {
								if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler)?;
							}
							/*InvokeRule v*/
							recog.base.set_state(50);
							recog.v_rec(4)?;
							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let tmp = VContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
							let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_v)?;

							recog.base.set_state(51);
							if !({recog.precpred(None, 2)}) {
								Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(52);
							_la = recog.base.input.la(1);
							if { !(_la==BetweenExpr_T__9 || _la==BetweenExpr_T__10) } {
								recog.err_handler.recover_inline(&mut recog.base)?;
							}
							else {
								if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler)?;
							}
							/*InvokeRule v*/
							recog.base.set_state(53);
							recog.v_rec(3)?;
							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(58);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = { let _sdp = recog.base.dfa_predict(7)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(7,&mut recog.base)? } else { _sdp } };
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
//------------------- atom ----------------
pub type AtomContextAll<'input, 'arena, Tok = CommonToken<'input>> = AtomContext<'input, 'arena, Tok>;

pub type AtomContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AtomContextExt<'input, 'arena, Tok>, BetweenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct AtomContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for AtomContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = BetweenExprParserNodeKind;
    fn node_tag() -> BetweenExprParserNodeKind { BetweenExprParserNodeKind::AtomContext }
	fn get_rule_index(&self) -> usize { RULE_atom }
    fn make_node(
        arena: &'arena Arena,
        ctx: AtomContext<'input, 'arena, Tok>,
    ) -> *mut BetweenExprParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a AtomContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => AtomContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut BetweenExprParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut AtomContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr4::cast_unchecked!(node.ctx_ptr() => mut AtomContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AtomContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena BetweenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut BetweenExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, AtomContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait AtomContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn b(&self) -> Option<&'arena BContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AtomContextAttrs<'input, 'arena, Tok> for AtomContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == BetweenExpr_ID)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == BetweenExpr_INT)
    }
    fn b(&self) -> Option<&'arena BContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> BetweenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn atom(&mut self,) -> Result<&'arena AtomContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(AtomContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 8, RULE_atom)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena AtomContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(68);
			recog.err_handler.sync(&mut recog.base)?;
			match { let _sdp = recog.base.dfa_predict(8)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(8,&mut recog.base)? } else { _sdp } } {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(59);
					recog.base.match_token(BetweenExpr_ID,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(60);
					recog.base.match_token(BetweenExpr_INT,&mut recog.err_handler)?;
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					recog.base.set_state(61);
					recog.base.match_token(BetweenExpr_T__11,&mut recog.err_handler)?;
					/*InvokeRule b*/
					recog.base.set_state(62);
					recog.b_rec(0)?;
					recog.base.set_state(63);
					recog.base.match_token(BetweenExpr_T__12,&mut recog.err_handler)?;
					}
				}
			,
				4 =>{
					/*------- Outer Most Alt 4 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
					{
					recog.base.set_state(65);
					recog.base.match_token(BetweenExpr_ID,&mut recog.err_handler)?;
					recog.base.set_state(66);
					recog.base.match_token(BetweenExpr_T__13,&mut recog.err_handler)?;
					/*InvokeRule b*/
					recog.base.set_state(67);
					recog.b_rec(0)?;
					}
				}

				_ => {}
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

// the serialized ATN is followed by 9 static SLL prediction tables (-Xstatic-dfa):
//   decision 3: precedence-dispatched over cutoffs [1, 2], tables [4 5 6]
//   decision 7: precedence-dispatched over cutoffs [2, 3], tables [7 8 6]
//   decision 0: LL(k), k=1, 9 states, 3 adaptive escapes
//   decision 2: LL(k), k=1, 3 states
//   decision 6: LL(k), k=1, 3 states
//   decision 8: LL(k), k=2, 13 states
//   table 4 (decision 3): LL(k), k=1, 9 states, 2 adaptive escapes
//   table 5 (decision 3): LL(k), k=1, 9 states, 1 adaptive escapes
//   table 6 (decision 3): LL(k), k=0, 1 states
//   table 7 (decision 7): LL(k), k=1, 9 states
//   table 8 (decision 7): LL(k), k=1, 10 states, 1 adaptive escapes
static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 19] = [
    "CAIijgEEAA4ABAIOAgQEDgQEBg4GBAgOCAIAAgACAAICAgICAgYCIhACAgICAgYCKhACAgICAgICAgIC",
    "AgICCgI6EAIUAhgCQBICAgQGBEYQBAIEAgQCBAIEAgQCBAIEBgRYEAQCBgIGAgYCBgIGAgYCBgIGAgYK",
    "Bm4QBhQGGAZ0EgYCCAIIAggCCAIIAggCCAIIAggGCIoBEAgCCAAEBAwKAAQIDBAABgIACg4CABASAgAU",
    "FpgBABQCAAAABCgCAAAACFYCAAAADFoCAAAAEIgBAgAAABQWBgQCABYYCgAAAhgCAgAAABocDAIBABwg",
    "BgwGAB4iBggEACAeAgAAACAiAgAAACIqAgAAACQmCgIAACYqBgQCBigaAgAAACgkAgAAACo8AgAAACwu",
    "FAQAAC4wCgQAADA6BgQCBjI0FAIAADQ2CgYAADY6BgQCBDgsAgAAADgyAgAAADpAAgAAADw4AgAAADw+",
    "AgAAAD4GAgAAAEA8AgAAAEJGCgIAAERCAgAAAERGAgAAAEZIAgAAAEhKCggAAEpMBgwGAExOCgQAAE5Q",
    "BgwGAFBYAgAAAFJUDgAAAFRYBgwGAFZEAgAAAFZSAgAAAFgKAgAAAFpcDAYBAFxeBhAIAF5wAgAAAGBi",
    "FAYAAGJkDgIAAGRuBgwGCGZoFAQAAGhqDgQAAGpuBgwGBmxgAgAAAGxmAgAAAG50AgAAAHBsAgAAAHBy",
    "AgAAAHIOAgAAAHRwAgAAAHaKAQoeAAB4igEKIAAAenwKGAAAfH4GBAIAfoABChoAAIABigECAAAAggGE",
    "AQoeAACEAYYBChwAAIYBigEGBAIAiAF2AgAAAIgBeAIAAACIAXoCAAAAiAGCAQIAAACKARICAAAAEiAo",
    "ODxEVmxwiAEKEhIAEjAABAEEBAEBBAQEAAAAAAAAAAAAMDAwMDAwMDAwAQECAgIEBAQGBgYICAgKCg4M",
    "EBYOGhoQBAYMAAIEAAAAAAwMDAQEAgYGBAwGDAACBAAAAAAMDAwQEgIUFgQQGkgABgAEAgICAgICAgII",
    "AAACAAAAAAAAAAAAAAASEkhISEhISEhISEhIGBgCHh4EICAGAQEIAgIKBAQMBgYOCAgQCg4SEBYUGhoW",
    "HBwYARIwAAQEAQEEBAQEBAAAAAAAAAAAADAwMDAwMDAwMAEBAgICBAQEBgYGCAgICgoODBAWDhoaEAES",
    "MAAEBAEEBAQEBAQAAAAAAAAAAAAwMDAwMDAwMDABAQICAgQEBAYGBggICAoKDgwQFg4aGhABAgAEAAAA",
    "ARIwAAQEBAQEBAIEBAAAAAAAAAAAADAwMDAwMDAwMAEBAgICBAQEBgYGCAgICgoODBAWDhoaEAEUNgAE",
    "BAQEBAQBBAQEAAAAAAAAAAAAADY2NjY2NjY2NjYBAQICAgQEBAYGBggICAoKDgwQEg4UFhAaGhIEBgQC",
    "BAgKDA4EBAYOEAw="
];