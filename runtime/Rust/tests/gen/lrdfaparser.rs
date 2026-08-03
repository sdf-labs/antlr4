// Generated from LrDfa.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::prelude::*;
use dbt_antlr4::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use super::lrdfalistener::*;
dbt_antlr4::check_version!("2","0");
pub const LrDfa_T__0:i32=1; 
pub const LrDfa_T__1:i32=2; 
pub const LrDfa_T__2:i32=3; 
pub const LrDfa_T__3:i32=4; 
pub const LrDfa_T__4:i32=5; 
pub const LrDfa_T__5:i32=6; 
pub const LrDfa_T__6:i32=7; 
pub const LrDfa_T__7:i32=8; 
pub const LrDfa_T__8:i32=9; 
pub const LrDfa_T__9:i32=10; 
pub const LrDfa_ID:i32=11; 
pub const LrDfa_INT:i32=12; 
pub const LrDfa_WS:i32=13;
pub const LrDfa_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_e:usize = 1;
pub const ruleNames: [&'static str; 2] = [
    "s", "e"
];

pub const _LITERAL_NAMES: [Option<&'static str>;11] = [
	None, Some("'['"), Some("']'"), Some("'!'"), Some("'-'"), Some("'^'"), 
	Some("'*'"), Some("'/'"), Some("'+'"), Some("'?'"), Some("':'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;14]  = [
	None, None, None, None, None, None, None, None, None, None, None, Some("ID"), 
	Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, LrDfaParserExt<'input, 'arena>, LrDfaParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct LrDfaParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> LrDfaParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				LrDfaParserExt {
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
        L: LrDfaListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct LrDfaTreeWalker;
impl LrDfaTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: LrDfaListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, LrDfaParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <LrDfaParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum LrDfaParserNodeKind {
    SContext,
    EContext,
    Terminal,
    Error,
}
pub type LrDfaParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, LrDfaParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => LrDfaParser }
dbt_antlr4::impl_node_kind! { LrDfaParserNodeKind {
    EContext(EContextAll), ; SContext(enter_s, exit_s, ), 
    }; listener = dyn LrDfaListener<'arena, Tok>,
}

pub struct LrDfaParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> LrDfaParserExt<'input, 'arena> {
}

dbt_antlr4::impl_parser_recog! { LrDfaParserExt, LrDfaParserNodeKind, "LrDfa.g4"; sempred (LrDfaParserNode<'input, 'arena, TF::Tok>, LrDfaParser) { 1 => e_sempred, } }

impl<'input, 'arena, Input, TF> LrDfaParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn e_sempred(_ctx: Option<&'arena EContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 6)
		    }
	        1 => {
			recog.precpred(None, 5)
		    }
	        2 => {
			recog.precpred(None, 4)
		    }
	        3 => {
			recog.precpred(None, 3)
		    }
	        4 => {
			recog.precpred(None, 9)
		    }
	        5 => {
			recog.precpred(None, 8)
		    }
		    _ => true
		}
	}
}
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { LrDfaParserNodeKind, SContext, SContextExt, RULE_s }

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
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LrDfa_EOF)
    }
}

impl<'input, 'arena, Input, TF> LrDfaParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn s(&mut self,) -> Result<&'arena SContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, SContext<TF::Tok>, RULE_s, 0, |_parentctx| SContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(4);
		recog.e_rec(0)?;
		recog.base.set_state(5);
		recog.base.match_token(LrDfa_EOF,&mut recog.err_handler)?;
		}
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
	NegContext(NegContext<'input, 'arena, Tok>),
	MulContext(MulContext<'input, 'arena, Tok>),
	TernaryContext(TernaryContext<'input, 'arena, Tok>),
	PowContext(PowContext<'input, 'arena, Tok>),
	IndexContext(IndexContext<'input, 'arena, Tok>),
	IdContext(IdContext<'input, 'arena, Tok>),
	FactContext(FactContext<'input, 'arena, Tok>),
	IntContext(IntContext<'input, 'arena, Tok>),
    Error(EContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { EContextAll { } { AddContext, NegContext, MulContext, TernaryContext, PowContext, IndexContext, IdContext, FactContext, IntContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { EContextAll { } { AddContext, NegContext, MulContext, TernaryContext, PowContext, IndexContext, IdContext, FactContext, IntContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { LrDfaParserNodeKind::EContextAll { AddContext, NegContext, MulContext, TernaryContext, PowContext, IndexContext, IdContext, FactContext, IntContext, Error, } }
dbt_antlr4::impl_node_inner! { LrDfaParserNodeKind::EContext::EContextAll { AddContext, NegContext, MulContext, TernaryContext, PowContext, IndexContext, IdContext, FactContext, IntContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { LrDfaListener::LrDfaParserNodeKind::EContextAll { AddContext(enter_Add, exit_Add), NegContext(enter_Neg, exit_Neg), MulContext(enter_Mul, exit_Mul), TernaryContext(enter_Ternary, exit_Ternary), PowContext(enter_Pow, exit_Pow), IndexContext(enter_Index, exit_Index), IdContext(enter_Id, exit_Id), FactContext(enter_Fact, exit_Fact), IntContext(enter_Int, exit_Int), } }

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
			NegContext(inner) => inner,
			MulContext(inner) => inner,
			TernaryContext(inner) => inner,
			PowContext(inner) => inner,
			IndexContext(inner) => inner,
			IdContext(inner) => inner,
			FactContext(inner) => inner,
			IntContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type EContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, EContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;
#[derive(Debug)]
pub struct EContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr4::impl_ctx! { @labeled LrDfaParserNodeKind::EContext, EContextAll, EContextExt, RULE_e }
impl<'input: 'arena, 'arena, Tok: Token + 'input> EContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena LrDfaParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut LrDfaParserNode<'input, 'arena, Tok>, ANTLRError>
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

pub type AddContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AddContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;

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

dbt_antlr4::impl_ctx! { @alt LrDfaParserNodeKind::EContext, EContextAll::AddContext, AddContextExt, RULE_e }

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

	fn copy_from(src: &mut LrDfaParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::AddContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type NegContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NegContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;

pub trait NegContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NegContextAttrs<'input, 'arena, Tok> for NegContext<'input, 'arena, Tok>
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct NegContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt LrDfaParserNodeKind::EContext, EContextAll::NegContext, NegContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for NegContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NegContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut LrDfaParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::NegContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type MulContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, MulContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;

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

dbt_antlr4::impl_ctx! { @alt LrDfaParserNodeKind::EContext, EContextAll::MulContext, MulContextExt, RULE_e }

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

	fn copy_from(src: &mut LrDfaParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::MulContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type TernaryContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, TernaryContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;

pub trait TernaryContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
	fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TernaryContextAttrs<'input, 'arena, Tok> for TernaryContext<'input, 'arena, Tok>
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct TernaryContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt LrDfaParserNodeKind::EContext, EContextAll::TernaryContext, TernaryContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for TernaryContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TernaryContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut LrDfaParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::TernaryContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type PowContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PowContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;

pub trait PowContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
	fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PowContextAttrs<'input, 'arena, Tok> for PowContext<'input, 'arena, Tok>
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct PowContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt LrDfaParserNodeKind::EContext, EContextAll::PowContext, PowContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for PowContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PowContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut LrDfaParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::PowContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type IndexContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, IndexContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;

pub trait IndexContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
	fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> IndexContextAttrs<'input, 'arena, Tok> for IndexContext<'input, 'arena, Tok>
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct IndexContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt LrDfaParserNodeKind::EContext, EContextAll::IndexContext, IndexContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for IndexContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> IndexContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut LrDfaParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::IndexContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type IdContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, IdContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;

pub trait IdContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> IdContextAttrs<'input, 'arena, Tok> for IdContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LrDfa_ID)
    }
}
#[derive(Debug)]
pub struct IdContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt LrDfaParserNodeKind::EContext, EContextAll::IdContext, IdContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for IdContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> IdContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut LrDfaParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::IdContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type FactContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, FactContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;

pub trait FactContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> FactContextAttrs<'input, 'arena, Tok> for FactContext<'input, 'arena, Tok>
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct FactContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt LrDfaParserNodeKind::EContext, EContextAll::FactContext, FactContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for FactContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> FactContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut LrDfaParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::FactContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type IntContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, IntContextExt<'input, 'arena, Tok>, LrDfaParserNodeKind, Tok>;

pub trait IntContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token INT
	/// Returns `None` if there is no child corresponding to token INT
	fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> IntContextAttrs<'input, 'arena, Tok> for IntContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LrDfa_INT)
    }
}
#[derive(Debug)]
pub struct IntContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt LrDfaParserNodeKind::EContext, EContextAll::IntContext, IntContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for IntContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> IntContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut LrDfaParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::IntContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> LrDfaParser<'input, 'arena, Input, TF>
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
        dbt_antlr4::parse_rule!(rec recog = self, _parentctx, _parentState, EContext<TF::Tok>, RULE_e, 2, |_parentctx| EContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena EContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
		let mut _la: i32 = -1;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(12);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LrDfa_T__3  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { NegContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena NegContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

		        recog.base.set_state(8);
		        recog.base.match_token(LrDfa_T__3,&mut recog.err_handler)?;
		        recog.base.set_state(9);
		        recog.e_rec(7)?;
		        }}
		    LrDfa_ID  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { IdContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena IdContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        recog.base.set_state(10);
		        recog.base.match_token(LrDfa_ID,&mut recog.err_handler)?;
		        }}
		    LrDfa_INT  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { IntContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena IntContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        recog.base.set_state(11);
		        recog.base.match_token(LrDfa_INT,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(38);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(2)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(2,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(36);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(1)?; _sdp } {
					1 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						PowContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena PowContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(14);
						if !({recog.precpred(None, 6)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
						}
						recog.base.set_state(15);
						recog.base.match_token(LrDfa_T__4,&mut recog.err_handler)?;
						recog.base.set_state(16);
						recog.e_rec(6)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						MulContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena MulContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(17);
						if !({recog.precpred(None, 5)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
						}
						recog.base.set_state(18);
						_la = recog.base.input.la(1);
						if { !(_la==LrDfa_T__5 || _la==LrDfa_T__6) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(19);
						recog.e_rec(6)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						AddContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena AddContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(20);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(21);
						_la = recog.base.input.la(1);
						if { !(_la==LrDfa_T__3 || _la==LrDfa_T__7) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(22);
						recog.e_rec(5)?;
						}
					}
				,
					4 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						TernaryContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena TernaryContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(23);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(24);
						recog.base.match_token(LrDfa_T__8,&mut recog.err_handler)?;
						recog.base.set_state(25);
						recog.e_rec(0)?;
						recog.base.set_state(26);
						recog.base.match_token(LrDfa_T__9,&mut recog.err_handler)?;
						recog.base.set_state(27);
						recog.e_rec(3)?;
						}
					}
				,
					5 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						IndexContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena IndexContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(29);
						if !({recog.precpred(None, 9)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
						}
						recog.base.set_state(30);
						recog.base.match_token(LrDfa_T__0,&mut recog.err_handler)?;
						recog.base.set_state(31);
						recog.e_rec(0)?;
						recog.base.set_state(32);
						recog.base.match_token(LrDfa_T__1,&mut recog.err_handler)?;
						}
					}
				,
					6 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						FactContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena FactContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(34);
						if !({recog.precpred(None, 8)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
						}
						recog.base.set_state(35);
						recog.base.match_token(LrDfa_T__2,&mut recog.err_handler)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(40);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(2)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(2,&mut recog.base)? } else { _sdp } };
		}
		}
		})
	}
}

// the serialized ATN is followed by 8 static DFA table(s) (-Xstatic-dfa; parser SLL prediction or lexer mode tables):
//   decision 2: precedence-dispatched over cutoffs [3, 4, 5, 6, 8, 9], tables [1 2 3 4 5 6 7]
//   decision 1: LL(k), k=1, 7 states
//   table 1 (decision 2): LL(k), k=1, 8 states
//   table 2 (decision 2): LL(k), k=1, 5 states
//   table 3 (decision 2): LL(k), k=1, 9 states
//   table 4 (decision 2): LL(k), k=1, 5 states
//   table 5 (decision 2): LL(k), k=1, 8 states
//   table 6 (decision 2): LL(k), k=1, 3 states
//   table 7 (decision 2): LL(k), k=0, 1 states
static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 13] = [
    "CAIaVAQADgAEAg4CAgACAAIAAgICAgICAgICAgYCGhACAgICAgICAgICAgICAgICAgICAgICAgICAgIC",
    "AgICAgICAgICAgICAgICAgIKAkoQAhQCGAJQEgICAgACBAQABAAEAgAMDgQACAgQEF4ACAIAAAAEGAIA",
    "AAAICgYEAgAKDAoAAAIMAgIAAAAOEAwCAQAQEgoIAAASGgYEAg4UGgoWAAAWGgoYAAAYDgIAAAAYFAIA",
    "AAAYFgIAAAAaTAIAAAAcHhQMAAAeIAoKAAAgSgYEAgwiJBQKAAAkJg4AAAAmSgYEAgwoKhQIAAAqLA4C",
    "AAAsSgYEAgouMBQGAAAwMgoSAAAyNAYEAgA0NgoUAAA2OAYEAgY4SgIAAAA6PBQSAAA8PgoCAAA+QAYE",
    "AgBAQgoEAABCSgIAAABERhQQAABGSgoGAABIHAIAAABIIgIAAABIKAIAAABILgIAAABIOgIAAABIRAIA",
    "AABKUAIAAABMSAIAAABMTgIAAABOBgIAAABQTAIAAAAGGEhMDhAGAg4qAAoMBgIECAAAAAAAAAAAKioq",
    "KioqKgICAgYGBAgIBgoKCAwOChAQBhISDAAAAAABECoABAIEAgICBAQAAAAAAAAAACoqKioqKioqAQEC",
    "AgIEBAQGBgYICBAKEhIMFBQOAAAAAAEKGAAEAgICBAAAAAAAGBgYGBgBAQICAgQGBgYIEAgAAAAAARI2",
    "AAQCBAIEAgQEBAAAAAAAAAAAADY2NjY2NjY2NgEBAgICBAQEBgYGCAgICgoODBAQChISDhQUEAAAAAAB",
    "ChgABAICAgQAAAAAABgYGBgYAQECAgIEBgYGCgoIAAAAAAEQKgAEAgQCBAQEBAAAAAAAAAAAKioqKioq",
    "KioBAQICAgQEBAYGBggIEAoSEgwUFA4AAAAAAQYMAAQCBAAAAAwMDAEBAgICBAAAAAABAgAEAAAAAAAA",
    "AAIEDAYICgwQEgIEBggKDA4="
];