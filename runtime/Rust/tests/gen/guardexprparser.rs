// Generated from GuardExpr.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::prelude::*;
use dbt_antlr4::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use super::guardexprlistener::*;
dbt_antlr4::check_version!("2","0");
pub const GuardExpr_T__0:i32=1; 
pub const GuardExpr_T__1:i32=2; 
pub const GuardExpr_T__2:i32=3; 
pub const GuardExpr_T__3:i32=4; 
pub const GuardExpr_T__4:i32=5; 
pub const GuardExpr_T__5:i32=6; 
pub const GuardExpr_T__6:i32=7; 
pub const GuardExpr_T__7:i32=8; 
pub const GuardExpr_T__8:i32=9; 
pub const GuardExpr_T__9:i32=10; 
pub const GuardExpr_T__10:i32=11; 
pub const GuardExpr_ID:i32=12; 
pub const GuardExpr_INT:i32=13; 
pub const GuardExpr_WS:i32=14;
pub const GuardExpr_EOF:i32=EOF;
pub const RULE_prog:usize = 0; 
pub const RULE_stmt:usize = 1; 
pub const RULE_expr:usize = 2; 
pub const RULE_pred:usize = 3; 
pub const RULE_value:usize = 4;
pub const ruleNames: [&'static str; 5] = [
    "prog", "stmt", "expr", "pred", "value"
];

pub const _LITERAL_NAMES: [Option<&'static str>;12] = [
	None, Some("'set'"), Some("'='"), Some("';'"), Some("'NOT'"), Some("'AND'"), 
	Some("'OR'"), Some("'<'"), Some("'>'"), Some("'IN'"), Some("'('"), Some("')'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;15]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, GuardExprParserExt<'input, 'arena>, GuardExprParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				GuardExprParserExt {
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
        L: GuardExprListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct GuardExprTreeWalker;
impl GuardExprTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: GuardExprListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, GuardExprParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <GuardExprParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum GuardExprParserNodeKind {
    ProgContext,
    StmtContext,
    ExprContext,
    PredContext,
    ValueContext,
    Terminal,
    Error,
}
pub type GuardExprParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, GuardExprParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => GuardExprParser }
dbt_antlr4::impl_node_kind! { GuardExprParserNodeKind {
    StmtContext(StmtContextAll), ExprContext(ExprContextAll), PredContext(PredContextAll), ; ProgContext(enter_prog, exit_prog, ), ValueContext(enter_value, exit_value, ), 
    }; listener = dyn GuardExprListener<'arena, Tok>,
}

pub struct GuardExprParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> GuardExprParserExt<'input, 'arena> {
}

dbt_antlr4::impl_parser_recog! { GuardExprParserExt, GuardExprParserNodeKind, "GuardExpr.g4"; sempred (GuardExprParserNode<'input, 'arena, TF::Tok>, GuardExprParser) { 2 => expr_sempred, } }

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn expr_sempred(_ctx: Option<&'arena ExprContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
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
//------------------- prog ----------------
pub type ProgContextAll<'input, 'arena, Tok = CommonToken<'input>> = ProgContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { GuardExprParserNodeKind, ProgContext, ProgContextExt, RULE_prog }

pub trait ProgContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn stmt_all(&self) -> Vec<&'arena StmtContextAll<'input, 'arena, Tok>>;
    fn stmt(&self, i: usize) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ProgContextAttrs<'input, 'arena, Tok> for ProgContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == GuardExpr_EOF)
    }
    fn stmt_all(&self) -> Vec<&'arena StmtContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn stmt(&self, i: usize) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn prog(&mut self,) -> Result<&'arena ProgContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, ProgContext<TF::Tok>, RULE_prog, 0, |_parentctx| ProgContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ProgContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(11); 
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		loop {
			{
			{
			recog.base.set_state(10);
			recog.stmt()?;
			}
			}
			recog.base.set_state(13); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 13330) != 0)) {break}
		}
		recog.base.set_state(15);
		recog.base.match_token(GuardExpr_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- stmt ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum StmtContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	EvalContext(EvalContext<'input, 'arena, Tok>),
	AssignContext(AssignContext<'input, 'arena, Tok>),
    Error(StmtContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { StmtContextAll { } { EvalContext, AssignContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { StmtContextAll { } { EvalContext, AssignContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { GuardExprParserNodeKind::StmtContextAll { EvalContext, AssignContext, Error, } }
dbt_antlr4::impl_node_inner! { GuardExprParserNodeKind::StmtContext::StmtContextAll { EvalContext, AssignContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { GuardExprListener::GuardExprParserNodeKind::StmtContextAll { EvalContext(enter_eval, exit_eval), AssignContext(enter_assign, exit_assign), } }

impl<'input, 'arena, Tok> Deref for StmtContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn StmtContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use StmtContextAll::*;
		match self{
			EvalContext(inner) => inner,
			AssignContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type StmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, StmtContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct StmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr4::impl_ctx! { @labeled GuardExprParserNodeKind::StmtContext, StmtContextAll, StmtContextExt, RULE_stmt }
impl<'input: 'arena, 'arena, Tok: Token + 'input> StmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena GuardExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut GuardExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, StmtContextExt {
				ph: PhantomData
			}
		)
	}
}
pub trait StmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> StmtContextAttrs<'input, 'arena, Tok> for StmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type EvalContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, EvalContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait EvalContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> EvalContextAttrs<'input, 'arena, Tok> for EvalContext<'input, 'arena, Tok>
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct EvalContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: StmtContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt GuardExprParserNodeKind::StmtContext, StmtContextAll::EvalContext, EvalContextExt, RULE_stmt }

impl<'input, 'arena, Tok> StmtContextAttrs<'input, 'arena, Tok> for EvalContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> EvalContextExt<'input, 'arena, Tok> {
	fn new(base: StmtContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut StmtContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            StmtContextAll::EvalContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut StmtContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type AssignContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AssignContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait AssignContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
	fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AssignContextAttrs<'input, 'arena, Tok> for AssignContext<'input, 'arena, Tok>
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct AssignContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: StmtContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt GuardExprParserNodeKind::StmtContext, StmtContextAll::AssignContext, AssignContextExt, RULE_stmt }

impl<'input, 'arena, Tok> StmtContextAttrs<'input, 'arena, Tok> for AssignContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AssignContextExt<'input, 'arena, Tok> {
	fn new(base: StmtContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut StmtContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            StmtContextAll::AssignContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut StmtContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn stmt(&mut self,) -> Result<&'arena StmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, StmtContext<TF::Tok>, RULE_stmt, 2, |_parentctx| StmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(26);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    GuardExpr_T__0  => {
		        /*------- Outer Most Alt 1 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            AssignContextExt::copy_from(ctx);
		            ctx.set_alt_number(1);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena AssignContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(17);
		        recog.base.match_token(GuardExpr_T__0,&mut recog.err_handler)?;
		        recog.base.set_state(18);
		        recog.expr_rec(0)?;
		        recog.base.set_state(19);
		        recog.base.match_token(GuardExpr_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(20);
		        recog.expr_rec(0)?;
		        recog.base.set_state(21);
		        recog.base.match_token(GuardExpr_T__2,&mut recog.err_handler)?;
		        }}
		    GuardExpr_T__3 |GuardExpr_T__9 |GuardExpr_ID |GuardExpr_INT  => {
		        /*------- Outer Most Alt 2 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            EvalContextExt::copy_from(ctx);
		            ctx.set_alt_number(2);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena EvalContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(23);
		        recog.expr_rec(0)?;
		        recog.base.set_state(24);
		        recog.base.match_token(GuardExpr_T__2,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- expr ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum ExprContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	NotContext(NotContext<'input, 'arena, Tok>),
	PredicatedContext(PredicatedContext<'input, 'arena, Tok>),
	OrContext(OrContext<'input, 'arena, Tok>),
	AndContext(AndContext<'input, 'arena, Tok>),
    Error(ExprContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { ExprContextAll { } { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { ExprContextAll { } { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { GuardExprParserNodeKind::ExprContextAll { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr4::impl_node_inner! { GuardExprParserNodeKind::ExprContext::ExprContextAll { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { GuardExprListener::GuardExprParserNodeKind::ExprContextAll { NotContext(enter_not, exit_not), PredicatedContext(enter_predicated, exit_predicated), OrContext(enter_or, exit_or), AndContext(enter_and, exit_and), } }

impl<'input, 'arena, Tok> Deref for ExprContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn ExprContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			NotContext(inner) => inner,
			PredicatedContext(inner) => inner,
			OrContext(inner) => inner,
			AndContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type ExprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ExprContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ExprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr4::impl_ctx! { @labeled GuardExprParserNodeKind::ExprContext, ExprContextAll, ExprContextExt, RULE_expr }
impl<'input: 'arena, 'arena, Tok: Token + 'input> ExprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena GuardExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut GuardExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ExprContextExt {
				ph: PhantomData
			}
		)
	}
}
pub trait ExprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> ExprContextAttrs<'input, 'arena, Tok> for ExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type NotContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NotContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait NotContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NotContextAttrs<'input, 'arena, Tok> for NotContext<'input, 'arena, Tok>
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct NotContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt GuardExprParserNodeKind::ExprContext, ExprContextAll::NotContext, NotContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for NotContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NotContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::NotContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type PredicatedContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PredicatedContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait PredicatedContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn value(&self) -> Option<&'arena ValueContextAll<'input, 'arena, Tok>>;
	fn pred(&self) -> Option<&'arena PredContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PredicatedContextAttrs<'input, 'arena, Tok> for PredicatedContext<'input, 'arena, Tok>
{
    fn value(&self) -> Option<&'arena ValueContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn pred(&self) -> Option<&'arena PredContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct PredicatedContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt GuardExprParserNodeKind::ExprContext, ExprContextAll::PredicatedContext, PredicatedContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for PredicatedContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PredicatedContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::PredicatedContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type OrContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, OrContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait OrContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
	fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> OrContextAttrs<'input, 'arena, Tok> for OrContext<'input, 'arena, Tok>
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct OrContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt GuardExprParserNodeKind::ExprContext, ExprContextAll::OrContext, OrContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for OrContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> OrContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::OrContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type AndContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AndContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait AndContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
	fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AndContextAttrs<'input, 'arena, Tok> for AndContext<'input, 'arena, Tok>
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct AndContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt GuardExprParserNodeKind::ExprContext, ExprContextAll::AndContext, AndContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for AndContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AndContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::AndContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  expr(&mut self,) -> Result<&'arena ExprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: i32) -> Result<&'arena ExprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(rec recog = self, _parentctx, _parentState, ExprContext<TF::Tok>, RULE_expr, 4, |_parentctx| ExprContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena ExprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 4;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(35);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    GuardExpr_T__9 |GuardExpr_ID |GuardExpr_INT  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { PredicatedContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena PredicatedContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

		        recog.base.set_state(29);
		        recog.value()?;
		        recog.base.set_state(31);
		        recog.err_handler.sync(&mut recog.base)?;
		        match { let _sdp = recog.base.dfa_predict(2)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(2,&mut recog.base)? } else { _sdp } } {
		        	x if x == 1 =>{
		        		{
		        		recog.base.set_state(30);
		        		recog.pred()?;
		        		}
		        	}

		        	_ => {}
		        }
		        }}
		    GuardExpr_T__3  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { NotContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena NotContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        recog.base.set_state(33);
		        recog.base.match_token(GuardExpr_T__3,&mut recog.err_handler)?;
		        recog.base.set_state(34);
		        recog.expr_rec(3)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(45);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(5)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(5,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(43);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(4)?; _sdp } {
					1 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						AndContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_expr)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena AndContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(37);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(38);
						recog.base.match_token(GuardExpr_T__4,&mut recog.err_handler)?;
						recog.base.set_state(39);
						recog.expr_rec(3)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						OrContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_expr)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena OrContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(40);
						if !({recog.precpred(None, 1)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
						}
						recog.base.set_state(41);
						recog.base.match_token(GuardExpr_T__5,&mut recog.err_handler)?;
						recog.base.set_state(42);
						recog.expr_rec(2)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(47);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(5)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(5,&mut recog.base)? } else { _sdp } };
		}
		}
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
	InListContext(InListContext<'input, 'arena, Tok>),
    Error(PredContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { PredContextAll { } { CmpContext, InListContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { PredContextAll { } { CmpContext, InListContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { GuardExprParserNodeKind::PredContextAll { CmpContext, InListContext, Error, } }
dbt_antlr4::impl_node_inner! { GuardExprParserNodeKind::PredContext::PredContextAll { CmpContext, InListContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { GuardExprListener::GuardExprParserNodeKind::PredContextAll { CmpContext(enter_cmp, exit_cmp), InListContext(enter_inList, exit_inList), } }

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
			InListContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type PredContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PredContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct PredContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr4::impl_ctx! { @labeled GuardExprParserNodeKind::PredContext, PredContextAll, PredContextExt, RULE_pred }
impl<'input: 'arena, 'arena, Tok: Token + 'input> PredContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena GuardExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut GuardExprParserNode<'input, 'arena, Tok>, ANTLRError>
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

pub type CmpContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, CmpContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait CmpContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn value(&self) -> Option<&'arena ValueContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CmpContextAttrs<'input, 'arena, Tok> for CmpContext<'input, 'arena, Tok>
{
    fn value(&self) -> Option<&'arena ValueContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct CmpContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: PredContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt GuardExprParserNodeKind::PredContext, PredContextAll::CmpContext, CmpContextExt, RULE_pred }

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

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut PredContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            PredContextAll::CmpContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut PredContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type InListContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, InListContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait InListContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> InListContextAttrs<'input, 'arena, Tok> for InListContext<'input, 'arena, Tok>
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct InListContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: PredContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt GuardExprParserNodeKind::PredContext, PredContextAll::InListContext, InListContextExt, RULE_pred }

impl<'input, 'arena, Tok> PredContextAttrs<'input, 'arena, Tok> for InListContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> InListContextExt<'input, 'arena, Tok> {
	fn new(base: PredContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut PredContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            PredContextAll::InListContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut PredContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn pred(&mut self,) -> Result<&'arena PredContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, PredContext<TF::Tok>, RULE_pred, 6, |_parentctx| PredContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena PredContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(55);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    GuardExpr_T__1 |GuardExpr_T__6 |GuardExpr_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            CmpContextExt::copy_from(ctx);
		            ctx.set_alt_number(1);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena CmpContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(48);
		        _la = recog.base.input.la(1);
		        if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 388) != 0)) } {
		        	recog.err_handler.recover_inline(&mut recog.base)?;
		        }
		        else {
		        	if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        	recog.err_handler.report_match(&mut recog.base);
		        	recog.base.consume(&mut recog.err_handler)?;
		        }
		        recog.base.set_state(49);
		        recog.value()?;
		        }}
		    GuardExpr_T__8  => {
		        /*------- Outer Most Alt 2 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            InListContextExt::copy_from(ctx);
		            ctx.set_alt_number(2);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena InListContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(50);
		        recog.base.match_token(GuardExpr_T__8,&mut recog.err_handler)?;
		        recog.base.set_state(51);
		        recog.base.match_token(GuardExpr_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(52);
		        recog.expr_rec(0)?;
		        recog.base.set_state(53);
		        recog.base.match_token(GuardExpr_T__10,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- value ----------------
pub type ValueContextAll<'input, 'arena, Tok = CommonToken<'input>> = ValueContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { GuardExprParserNodeKind, ValueContext, ValueContextExt, RULE_value }

pub trait ValueContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
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
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ValueContextAttrs<'input, 'arena, Tok> for ValueContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == GuardExpr_ID)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == GuardExpr_INT)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn value(&mut self,) -> Result<&'arena ValueContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, ValueContext<TF::Tok>, RULE_value, 8, |_parentctx| ValueContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ValueContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(63);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    GuardExpr_ID  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(57);
		        recog.base.match_token(GuardExpr_ID,&mut recog.err_handler)?;
		        }}
		    GuardExpr_INT  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(58);
		        recog.base.match_token(GuardExpr_INT,&mut recog.err_handler)?;
		        }}
		    GuardExpr_T__9  => {
		        /*------- Outer Most Alt 3 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
		        {
		        recog.base.set_state(59);
		        recog.base.match_token(GuardExpr_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(60);
		        recog.expr_rec(0)?;
		        recog.base.set_state(61);
		        recog.base.match_token(GuardExpr_T__10,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}

// the serialized ATN is followed by 5 static DFA table(s) (-Xstatic-dfa; parser SLL prediction or lexer mode tables):
//   decision 5: precedence-dispatched over cutoffs [1, 2], tables [2 3 4]
//   decision 2: LL(*) cyclic, 35 states, 6 adaptive escapes, 8 guarded takes
//   decision 4: LL(k), k=1, 3 states
//   table 2 (decision 5): LL(k), k=1, 6 states
//   table 3 (decision 5): LL(k), k=1, 7 states
//   table 4 (decision 5): LL(k), k=0, 1 states
static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 17] = [
    "CAIchAEEAA4ABAIOAgQEDgQEBg4GBAgOCAIACAAYEAAWABgAGgIAAgACAgICAgICAgICAgICAgICAgIG",
    "AjYQAgIEAgQCBAYEQBAEAgQCBAYESBAEAgQCBAIEAgQCBAIECgRYEAQUBBgEXhIEAgYCBgIGAgYCBgIG",
    "AgYGBnAQBgIIAggCCAIIAggCCAYIgAEQCAIIAAIICgAECAwQAAIEAAQEDhCKAQAWAgAAAAQ0AgAAAAhG",
    "AgAAAAxuAgAAABB+AgAAABQYBgQCABYUAgAAABgaAgAAABoWAgAAABocAgAAABweAgAAAB4gCgAAAiAC",
    "AgAAACIkCgIAACQmBggEACYoCgQAACgqBggEACosCgYAACw2AgAAAC4wBggEADAyCgYAADI2AgAAADQi",
    "AgAAADQuAgAAADYGAgAAADg6DAQBADo+BhAIADxABgwGAD48AgAAAD5AAgAAAEBIAgAAAEJECggAAERI",
    "BggEBkY4AgAAAEZCAgAAAEhaAgAAAEpMFAQAAExOCgoAAE5YBggEBlBSFAIAAFJUCgwAAFRYBggEBFZK",
    "AgAAAFZQAgAAAFheAgAAAFpWAgAAAFpcAgAAAFwKAgAAAF5aAgAAAGBiDgAAAGJwBhAIAGRmChIAAGZo",
    "ChQAAGhqBggEAGpsChYAAGxwAgAAAG5gAgAAAG5kAgAAAHAOAgAAAHKAAQoYAAB0gAEKGgAAdngKFAAA",
    "eHoGCAQAenwKFgAAfIABAgAAAH5yAgAAAH50AgAAAH52AgAAAIABEgIAAAAQGjQ+RlZabn4OCgwERuoB",
    "AAQABAQCAgQEAAAAAAACAAUABAQCAQEBAQUBAQUCBQUFBQUEAAQAAAAAAAAEAgQEBAACAAQAAAAAAAAA",
    "AAAAAAAAAAAAAAAqKjw8PDw8PDxOeIoBnAHGAcYB2AHYAeoB6gHqAeoB6gHqAeoB6gHqAeoB6gHqAeoB",
    "6gHqAeoB6gHqAQEBAgQEBAYGBgoMCA4QChISDBYWDggIEBQUEhgaFAgIFhQUGBgaGgEBHAQEHgYGIAoM",
    "Ig4QJBISJhYWKAgIKhQUGBgaGggILBQUGBgaLgEBAgQEMAYGBgoMMg4QNBISNhYWOAgIOhQUPBgaPggI",
    "QBQUQhgaRAAQIAIyAjgCPAI+AkACQgJEAgIkDDo8RE5UYggGDAACBAAAAAAMDAwKCgIMDAQAAAAAAQwe",
    "AAQEBAIEBAAAAAAAAB4eHh4eHgEBAgQEBAYGBgoMCBYWCgAAAAABDiQABAQEAgQEBAAAAAAAAAAkJCQk",
    "JCQkAQECBAQEBgYGCgoIDAwKFhYMAAAAAAECAAQAAAAAAAAAAgoEAgQEBgg="
];
