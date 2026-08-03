// Generated from StaticDFA.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::prelude::*;
use dbt_antlr4::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use super::staticdfalistener::*;
dbt_antlr4::check_version!("2","0");
pub const StaticDFA_T__0:i32=1; 
pub const StaticDFA_T__1:i32=2; 
pub const StaticDFA_T__2:i32=3; 
pub const StaticDFA_T__3:i32=4; 
pub const StaticDFA_T__4:i32=5; 
pub const StaticDFA_T__5:i32=6; 
pub const StaticDFA_T__6:i32=7; 
pub const StaticDFA_T__7:i32=8; 
pub const StaticDFA_T__8:i32=9; 
pub const StaticDFA_T__9:i32=10; 
pub const StaticDFA_T__10:i32=11; 
pub const StaticDFA_T__11:i32=12; 
pub const StaticDFA_ID:i32=13; 
pub const StaticDFA_WS:i32=14;
pub const StaticDFA_EOF:i32=EOF;
pub const RULE_stat:usize = 0; 
pub const RULE_expr:usize = 1; 
pub const RULE_name:usize = 2; 
pub const RULE_starLoop:usize = 3; 
pub const RULE_plusLoop:usize = 4; 
pub const RULE_opt:usize = 5; 
pub const RULE_dup:usize = 6;
pub const ruleNames: [&'static str; 7] = [
    "stat", "expr", "name", "starLoop", "plusLoop", "opt", "dup"
];

pub const _LITERAL_NAMES: [Option<&'static str>;13] = [
	None, Some("'create'"), Some("'table'"), Some("'view'"), Some("'drop'"), 
	Some("'('"), Some("')'"), Some("'.'"), Some("'a'"), Some("'b'"), Some("'c'"), 
	Some("'d'"), Some("'e'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;15]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, Some("ID"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, StaticDFAParserExt<'input, 'arena>, StaticDFAParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct StaticDFAParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> StaticDFAParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				StaticDFAParserExt {
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
        L: StaticDFAListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct StaticDFATreeWalker;
impl StaticDFATreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: StaticDFAListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, StaticDFAParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <StaticDFAParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum StaticDFAParserNodeKind {
    StatContext,
    ExprContext,
    NameContext,
    StarLoopContext,
    PlusLoopContext,
    OptContext,
    DupContext,
    Terminal,
    Error,
}
pub type StaticDFAParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, StaticDFAParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => StaticDFAParser }
dbt_antlr4::impl_node_kind! { StaticDFAParserNodeKind {
    StatContext(StatContextAll), ExprContext(ExprContextAll), DupContext(DupContextAll), ; NameContext(enter_name, exit_name, ), StarLoopContext(enter_starLoop, exit_starLoop, ), PlusLoopContext(enter_plusLoop, exit_plusLoop, ), OptContext(enter_opt, exit_opt, ), 
    }; listener = dyn StaticDFAListener<'arena, Tok>,
}

pub struct StaticDFAParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> StaticDFAParserExt<'input, 'arena> {
}

dbt_antlr4::impl_parser_recog! { StaticDFAParserExt, StaticDFAParserNodeKind, "StaticDFA.g4" }

//------------------- stat ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum StatContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	CreateTableContext(CreateTableContext<'input, 'arena, Tok>),
	CreateViewContext(CreateViewContext<'input, 'arena, Tok>),
	DropTableContext(DropTableContext<'input, 'arena, Tok>),
    Error(StatContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { StatContextAll { } { CreateTableContext, CreateViewContext, DropTableContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { StatContextAll { } { CreateTableContext, CreateViewContext, DropTableContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { StaticDFAParserNodeKind::StatContextAll { CreateTableContext, CreateViewContext, DropTableContext, Error, } }
dbt_antlr4::impl_node_inner! { StaticDFAParserNodeKind::StatContext::StatContextAll { CreateTableContext, CreateViewContext, DropTableContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { StaticDFAListener::StaticDFAParserNodeKind::StatContextAll { CreateTableContext(enter_createTable, exit_createTable), CreateViewContext(enter_createView, exit_createView), DropTableContext(enter_dropTable, exit_dropTable), } }

impl<'input, 'arena, Tok> Deref for StatContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn StatContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use StatContextAll::*;
		match self{
			CreateTableContext(inner) => inner,
			CreateViewContext(inner) => inner,
			DropTableContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type StatContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, StatContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;
#[derive(Debug)]
pub struct StatContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr4::impl_ctx! { @labeled StaticDFAParserNodeKind::StatContext, StatContextAll, StatContextExt, RULE_stat }
impl<'input: 'arena, 'arena, Tok: Token + 'input> StatContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena StaticDFAParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut StaticDFAParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, StatContextExt {
				ph: PhantomData
			}
		)
	}
}
pub trait StatContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> StatContextAttrs<'input, 'arena, Tok> for StatContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type CreateTableContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, CreateTableContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;

pub trait CreateTableContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CreateTableContextAttrs<'input, 'arena, Tok> for CreateTableContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == StaticDFA_ID)
    }
}
#[derive(Debug)]
pub struct CreateTableContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: StatContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt StaticDFAParserNodeKind::StatContext, StatContextAll::CreateTableContext, CreateTableContextExt, RULE_stat }

impl<'input, 'arena, Tok> StatContextAttrs<'input, 'arena, Tok> for CreateTableContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CreateTableContextExt<'input, 'arena, Tok> {
	fn new(base: StatContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut StaticDFAParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut StatContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            StatContextAll::CreateTableContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut StatContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type CreateViewContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, CreateViewContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;

pub trait CreateViewContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CreateViewContextAttrs<'input, 'arena, Tok> for CreateViewContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == StaticDFA_ID)
    }
}
#[derive(Debug)]
pub struct CreateViewContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: StatContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt StaticDFAParserNodeKind::StatContext, StatContextAll::CreateViewContext, CreateViewContextExt, RULE_stat }

impl<'input, 'arena, Tok> StatContextAttrs<'input, 'arena, Tok> for CreateViewContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CreateViewContextExt<'input, 'arena, Tok> {
	fn new(base: StatContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut StaticDFAParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut StatContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            StatContextAll::CreateViewContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut StatContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type DropTableContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, DropTableContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;

pub trait DropTableContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DropTableContextAttrs<'input, 'arena, Tok> for DropTableContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == StaticDFA_ID)
    }
}
#[derive(Debug)]
pub struct DropTableContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: StatContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt StaticDFAParserNodeKind::StatContext, StatContextAll::DropTableContext, DropTableContextExt, RULE_stat }

impl<'input, 'arena, Tok> StatContextAttrs<'input, 'arena, Tok> for DropTableContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DropTableContextExt<'input, 'arena, Tok> {
	fn new(base: StatContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut StaticDFAParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut StatContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            StatContextAll::DropTableContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut StatContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> StaticDFAParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn stat(&mut self,) -> Result<&'arena StatContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, StatContext<TF::Tok>, RULE_stat, 0, |_parentctx| StatContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StatContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(23);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(0)?; _sdp } {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				recog.base.with_mut_ctx(|ctx| {
				    CreateTableContextExt::copy_from(ctx);
				    ctx.set_alt_number(1);
				});
				let _local_ctx_fn = |recog: &Self| -> &'arena CreateTableContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				{
				recog.base.set_state(14);
				recog.base.match_token(StaticDFA_T__0,&mut recog.err_handler)?;
				recog.base.set_state(15);
				recog.base.match_token(StaticDFA_T__1,&mut recog.err_handler)?;
				recog.base.set_state(16);
				recog.base.match_token(StaticDFA_ID,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				recog.base.with_mut_ctx(|ctx| {
				    CreateViewContextExt::copy_from(ctx);
				    ctx.set_alt_number(2);
				});
				let _local_ctx_fn = |recog: &Self| -> &'arena CreateViewContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				{
				recog.base.set_state(17);
				recog.base.match_token(StaticDFA_T__0,&mut recog.err_handler)?;
				recog.base.set_state(18);
				recog.base.match_token(StaticDFA_T__2,&mut recog.err_handler)?;
				recog.base.set_state(19);
				recog.base.match_token(StaticDFA_ID,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				recog.base.with_mut_ctx(|ctx| {
				    DropTableContextExt::copy_from(ctx);
				    ctx.set_alt_number(3);
				});
				let _local_ctx_fn = |recog: &Self| -> &'arena DropTableContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				{
				recog.base.set_state(20);
				recog.base.match_token(StaticDFA_T__3,&mut recog.err_handler)?;
				recog.base.set_state(21);
				recog.base.match_token(StaticDFA_T__1,&mut recog.err_handler)?;
				recog.base.set_state(22);
				recog.base.match_token(StaticDFA_ID,&mut recog.err_handler)?;
				}
			}

			_ => {}
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
	CallContext(CallContext<'input, 'arena, Tok>),
	RefContext(RefContext<'input, 'arena, Tok>),
    Error(ExprContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { ExprContextAll { } { CallContext, RefContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { ExprContextAll { } { CallContext, RefContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { StaticDFAParserNodeKind::ExprContextAll { CallContext, RefContext, Error, } }
dbt_antlr4::impl_node_inner! { StaticDFAParserNodeKind::ExprContext::ExprContextAll { CallContext, RefContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { StaticDFAListener::StaticDFAParserNodeKind::ExprContextAll { CallContext(enter_call, exit_call), RefContext(enter_ref, exit_ref), } }

impl<'input, 'arena, Tok> Deref for ExprContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn ExprContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			CallContext(inner) => inner,
			RefContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type ExprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ExprContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ExprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr4::impl_ctx! { @labeled StaticDFAParserNodeKind::ExprContext, ExprContextAll, ExprContextExt, RULE_expr }
impl<'input: 'arena, 'arena, Tok: Token + 'input> ExprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena StaticDFAParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut StaticDFAParserNode<'input, 'arena, Tok>, ANTLRError>
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

pub type CallContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, CallContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;

pub trait CallContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CallContextAttrs<'input, 'arena, Tok> for CallContext<'input, 'arena, Tok>
{
    fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct CallContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt StaticDFAParserNodeKind::ExprContext, ExprContextAll::CallContext, CallContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for CallContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CallContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut StaticDFAParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::CallContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type RefContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RefContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;

pub trait RefContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RefContextAttrs<'input, 'arena, Tok> for RefContext<'input, 'arena, Tok>
{
    fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct RefContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt StaticDFAParserNodeKind::ExprContext, ExprContextAll::RefContext, RefContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for RefContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RefContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut StaticDFAParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::RefContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> StaticDFAParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn expr(&mut self,) -> Result<&'arena ExprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, ExprContext<TF::Tok>, RULE_expr, 2, |_parentctx| ExprContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ExprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(30);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(1)?; _sdp } {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				recog.base.with_mut_ctx(|ctx| {
				    CallContextExt::copy_from(ctx);
				    ctx.set_alt_number(1);
				});
				let _local_ctx_fn = |recog: &Self| -> &'arena CallContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				{
				recog.base.set_state(25);
				recog.name()?;
				recog.base.set_state(26);
				recog.base.match_token(StaticDFA_T__4,&mut recog.err_handler)?;
				recog.base.set_state(27);
				recog.base.match_token(StaticDFA_T__5,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				recog.base.with_mut_ctx(|ctx| {
				    RefContextExt::copy_from(ctx);
				    ctx.set_alt_number(2);
				});
				let _local_ctx_fn = |recog: &Self| -> &'arena RefContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				{
				recog.base.set_state(29);
				recog.name()?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- name ----------------
pub type NameContextAll<'input, 'arena, Tok = CommonToken<'input>> = NameContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { StaticDFAParserNodeKind, NameContext, NameContextExt, RULE_name }

pub trait NameContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> NameContextAttrs<'input, 'arena, Tok> for NameContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == StaticDFA_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == StaticDFA_ID).nth(i)
    }
}

impl<'input, 'arena, Input, TF> StaticDFAParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn name(&mut self,) -> Result<&'arena NameContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		if let Some(_node) = self.base.resume_take(RULE_name)? {
		    return Ok(_node.as_rule_context().unwrap());
		}
        dbt_antlr4::parse_rule!(recog = self, _parentctx, NameContext<TF::Tok>, RULE_name, 4, |_parentctx| NameContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena NameContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(32);
		recog.base.match_token(StaticDFA_ID,&mut recog.err_handler)?;
		recog.base.set_state(37);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		while _la==StaticDFA_T__6 {
			{
			{
			recog.base.set_state(33);
			recog.base.match_token(StaticDFA_T__6,&mut recog.err_handler)?;
			recog.base.set_state(34);
			recog.base.match_token(StaticDFA_ID,&mut recog.err_handler)?;
			}
			}
			recog.base.set_state(39);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
		}
		}
		})
	}
}
//------------------- starLoop ----------------
pub type StarLoopContextAll<'input, 'arena, Tok = CommonToken<'input>> = StarLoopContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { StaticDFAParserNodeKind, StarLoopContext, StarLoopContextExt, RULE_starLoop }

pub trait StarLoopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> StarLoopContextAttrs<'input, 'arena, Tok> for StarLoopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

impl<'input, 'arena, Input, TF> StaticDFAParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn starLoop(&mut self,) -> Result<&'arena StarLoopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, StarLoopContext<TF::Tok>, RULE_starLoop, 6, |_parentctx| StarLoopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StarLoopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(44);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(3)?; _sdp };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				{
				{
				recog.base.set_state(40);
				recog.base.match_token(StaticDFA_T__7,&mut recog.err_handler)?;
				recog.base.set_state(41);
				recog.base.match_token(StaticDFA_T__8,&mut recog.err_handler)?;
				}
				} 
			}
			recog.base.set_state(46);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(3)?; _sdp };
		}
		recog.base.set_state(47);
		recog.base.match_token(StaticDFA_T__7,&mut recog.err_handler)?;
		recog.base.set_state(48);
		recog.base.match_token(StaticDFA_T__9,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- plusLoop ----------------
pub type PlusLoopContextAll<'input, 'arena, Tok = CommonToken<'input>> = PlusLoopContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { StaticDFAParserNodeKind, PlusLoopContext, PlusLoopContextExt, RULE_plusLoop }

pub trait PlusLoopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> PlusLoopContextAttrs<'input, 'arena, Tok> for PlusLoopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

impl<'input, 'arena, Input, TF> StaticDFAParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn plusLoop(&mut self,) -> Result<&'arena PlusLoopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, PlusLoopContext<TF::Tok>, RULE_plusLoop, 8, |_parentctx| PlusLoopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena PlusLoopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(52); 
		recog.err_handler.sync(&mut recog.base)?;
		_alt = 1;
		loop {
			match _alt {
			    x if x == 1 =>
				{
				{
				recog.base.set_state(50);
				recog.base.match_token(StaticDFA_T__7,&mut recog.err_handler)?;
				recog.base.set_state(51);
				recog.base.match_token(StaticDFA_T__8,&mut recog.err_handler)?;
				}
				}

			_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			recog.base.set_state(54); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(4)?; _sdp };
			if _alt==2 || _alt==INVALID_ALT { break }
		}
		recog.base.set_state(56);
		recog.base.match_token(StaticDFA_T__7,&mut recog.err_handler)?;
		recog.base.set_state(57);
		recog.base.match_token(StaticDFA_T__9,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- opt ----------------
pub type OptContextAll<'input, 'arena, Tok = CommonToken<'input>> = OptContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { StaticDFAParserNodeKind, OptContext, OptContextExt, RULE_opt }

pub trait OptContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> OptContextAttrs<'input, 'arena, Tok> for OptContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

impl<'input, 'arena, Input, TF> StaticDFAParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn opt(&mut self,) -> Result<&'arena OptContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, OptContext<TF::Tok>, RULE_opt, 10, |_parentctx| OptContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena OptContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(61);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(5)?; _sdp } {
			x if x == 1 =>{
				{
				recog.base.set_state(59);
				recog.base.match_token(StaticDFA_T__7,&mut recog.err_handler)?;
				recog.base.set_state(60);
				recog.base.match_token(StaticDFA_T__8,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		recog.base.set_state(63);
		recog.base.match_token(StaticDFA_T__7,&mut recog.err_handler)?;
		recog.base.set_state(64);
		recog.base.match_token(StaticDFA_T__9,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- dup ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum DupContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	DupFirstContext(DupFirstContext<'input, 'arena, Tok>),
	DupSecondContext(DupSecondContext<'input, 'arena, Tok>),
    Error(DupContext<'input, 'arena, Tok>)
}

dbt_antlr4::impl_rule_context! { DupContextAll { } { DupFirstContext, DupSecondContext, Error, } }
dbt_antlr4::impl_parser_rule_context! { DupContextAll { } { DupFirstContext, DupSecondContext, Error, } }
dbt_antlr4::impl_tree_trait_delegates! { StaticDFAParserNodeKind::DupContextAll { DupFirstContext, DupSecondContext, Error, } }
dbt_antlr4::impl_node_inner! { StaticDFAParserNodeKind::DupContext::DupContextAll { DupFirstContext, DupSecondContext, Error, } }
dbt_antlr4::impl_listener_dispatch! { StaticDFAListener::StaticDFAParserNodeKind::DupContextAll { DupFirstContext(enter_dupFirst, exit_dupFirst), DupSecondContext(enter_dupSecond, exit_dupSecond), } }

impl<'input, 'arena, Tok> Deref for DupContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn DupContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use DupContextAll::*;
		match self{
			DupFirstContext(inner) => inner,
			DupSecondContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type DupContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, DupContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;
#[derive(Debug)]
pub struct DupContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr4::impl_ctx! { @labeled StaticDFAParserNodeKind::DupContext, DupContextAll, DupContextExt, RULE_dup }
impl<'input: 'arena, 'arena, Tok: Token + 'input> DupContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena StaticDFAParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut StaticDFAParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, DupContextExt {
				ph: PhantomData
			}
		)
	}
}
pub trait DupContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> DupContextAttrs<'input, 'arena, Tok> for DupContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type DupFirstContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, DupFirstContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;

pub trait DupFirstContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DupFirstContextAttrs<'input, 'arena, Tok> for DupFirstContext<'input, 'arena, Tok>
{
}
#[derive(Debug)]
pub struct DupFirstContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: DupContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt StaticDFAParserNodeKind::DupContext, DupContextAll::DupFirstContext, DupFirstContextExt, RULE_dup }

impl<'input, 'arena, Tok> DupContextAttrs<'input, 'arena, Tok> for DupFirstContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DupFirstContextExt<'input, 'arena, Tok> {
	fn new(base: DupContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut StaticDFAParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut DupContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            DupContextAll::DupFirstContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut DupContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type DupSecondContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, DupSecondContextExt<'input, 'arena, Tok>, StaticDFAParserNodeKind, Tok>;

pub trait DupSecondContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DupSecondContextAttrs<'input, 'arena, Tok> for DupSecondContext<'input, 'arena, Tok>
{
}
#[derive(Debug)]
pub struct DupSecondContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: DupContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr4::impl_ctx! { @alt StaticDFAParserNodeKind::DupContext, DupContextAll::DupSecondContext, DupSecondContextExt, RULE_dup }

impl<'input, 'arena, Tok> DupContextAttrs<'input, 'arena, Tok> for DupSecondContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DupSecondContextExt<'input, 'arena, Tok> {
	fn new(base: DupContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut StaticDFAParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut DupContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            DupContextAll::DupSecondContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr4::cast_unchecked!(src => mut DupContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> StaticDFAParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn dup(&mut self,) -> Result<&'arena DupContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, DupContext<TF::Tok>, RULE_dup, 12, |_parentctx| DupContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena DupContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(70);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(6)?; _sdp } {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				recog.base.with_mut_ctx(|ctx| {
				    DupFirstContextExt::copy_from(ctx);
				    ctx.set_alt_number(1);
				});
				let _local_ctx_fn = |recog: &Self| -> &'arena DupFirstContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				{
				recog.base.set_state(66);
				recog.base.match_token(StaticDFA_T__10,&mut recog.err_handler)?;
				recog.base.set_state(67);
				recog.base.match_token(StaticDFA_T__11,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				recog.base.with_mut_ctx(|ctx| {
				    DupSecondContextExt::copy_from(ctx);
				    ctx.set_alt_number(2);
				});
				let _local_ctx_fn = |recog: &Self| -> &'arena DupSecondContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				{
				recog.base.set_state(68);
				recog.base.match_token(StaticDFA_T__10,&mut recog.err_handler)?;
				recog.base.set_state(69);
				recog.base.match_token(StaticDFA_T__11,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}

// the serialized ATN is followed by 6 static DFA table(s) (-Xstatic-dfa; parser SLL prediction or lexer mode tables):
//   decision 0: LL(k), k=2, 5 states
//   decision 1: LL(*) cyclic, 5 states
//   decision 3: LL(k), k=2, 4 states
//   decision 4: LL(k), k=2, 4 states
//   decision 5: LL(k), k=2, 4 states
//   decision 6: LL(k), k=2, 3 states
static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 14] = [
    "CAIckgEEAA4ABAIOAgQEDgQEBg4GBAgOCAQKDgoEDA4MAgACAAIAAgACAAIAAgACAAIABgAwEAACAgIC",
    "AgICAgICBgI+EAICBAIEAgQKBEgQBBQEGAROEgQCBgIGCgZWEAYUBhgGXBIGAgYCBgIGAggCCAgIahAI",
    "FggYCGwCCAIIAggCCgIKBgp8EAoCCgIKAgoCDAIMAgwCDAYMjgEQDAIMAAAOAAQIDBAUGAAAkgEALgIA",
    "AAAEPAIAAAAIQAIAAAAMWAIAAAAQaAIAAAAUegIAAAAYjAECAAAAHB4KAgAAHiAKBAAAIDAKGgAAIiQK",
    "AgAAJCYKBgAAJjAKGgAAKCoKCAAAKiwKBAAALDAKGgAALhwCAAAALiICAAAALigCAAAAMAICAAAAMjQG",
    "CAQANDYKCgAANjgKDAAAOD4CAAAAOj4GCAQAPDICAAAAPDoCAAAAPgYCAAAAQEoKGgAAQkQKDgAAREgK",
    "GgAARkICAAAASE4CAAAASkYCAAAASkwCAAAATAoCAAAATkoCAAAAUFIKEAAAUlYKEgAAVFACAAAAVlwC",
    "AAAAWFQCAAAAWFoCAAAAWl4CAAAAXFgCAAAAXmAKEAAAYGIKFAAAYg4CAAAAZGYKEAAAZmoKEgAAaGQC",
    "AAAAamwCAAAAbGgCAAAAbG4CAAAAbnACAAAAcHIKEAAAcnQKFAAAdBICAAAAdngKEAAAeHwKEgAAenYC",
    "AAAAenwCAAAAfH4CAAAAfoABChAAAIABggEKFAAAggEWAgAAAIQBhgEKFgAAhgGOAQoYAACIAYoBChYA",
    "AIoBjgEKGAAAjAGEAQIAAACMAYgBAgAAAI4BGgIAAAAOLjxKWGx6jAEODA4AChgAAAYCBAAAAAAAAAwY",
    "GBgYAgICCAgEBAQGBgYIAAAAAAIKHgAABAIAAAQAAAAABhgYGB4aGgIBAQQKCgYODggaGgIAAAAABggS",
    "AAACBAAAAAAABhISEhAQAhISBBQUBgAAAAAICBIAAAIEAAAAAAAGEhISEBACEhIEFBQGAAAAAAoIEgAA",
    "AgQAAAAAAAYSEhIQEAISEgQUFAYAAAAADAYMAAACAAAAAAYMDBYWAhgYBAAAAAAA"
];