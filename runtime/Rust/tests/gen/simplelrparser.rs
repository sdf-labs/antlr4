// Generated from SimpleLR.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::prelude::*;
use dbt_antlr4::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use super::simplelrlistener::*;
dbt_antlr4::check_version!("2","0");
pub const SimpleLR_ID:i32=1; 
pub const SimpleLR_WS:i32=2;
pub const SimpleLR_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_a:usize = 1;
pub const ruleNames: [&'static str; 2] = [
    "s", "a"
];

pub const _LITERAL_NAMES: [Option<&'static str>;0] = [
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;3]  = [
	None, Some("ID"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, SimpleLRParserExt<'input, 'arena>, SimpleLRParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct SimpleLRParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> SimpleLRParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				SimpleLRParserExt {
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
        L: SimpleLRListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct SimpleLRTreeWalker;
impl SimpleLRTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: SimpleLRListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, SimpleLRParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <SimpleLRParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum SimpleLRParserNodeKind {
    SContext,
    AContext,
    Terminal,
    Error,
}
pub type SimpleLRParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, SimpleLRParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => SimpleLRParser }
dbt_antlr4::impl_node_kind! { SimpleLRParserNodeKind {
; SContext(enter_s, exit_s, ), AContext(enter_a, exit_a, ), 
    }; listener = dyn SimpleLRListener<'arena, Tok>,
}

pub struct SimpleLRParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> SimpleLRParserExt<'input, 'arena> {
}

dbt_antlr4::impl_parser_recog! { SimpleLRParserExt, SimpleLRParserNodeKind, "SimpleLR.g4"; sempred (SimpleLRParserNode<'input, 'arena, TF::Tok>, SimpleLRParser) { 1 => a_sempred, } }

impl<'input, 'arena, Input, TF> SimpleLRParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn a_sempred(_ctx: Option<&'arena AContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 2)
		    }
		    _ => true
		}
	}
}
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { SimpleLRParserNodeKind, SContext, SContextExt, RULE_s }

pub trait SContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SContextAttrs<'input, 'arena, Tok> for SContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> SimpleLRParser<'input, 'arena, Input, TF>
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
		recog.a_rec(0)?;
		}
        let tmp = recog.input.lt(-1);
        recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });println!("test");
		})
	}
}
//------------------- a ----------------
pub type AContextAll<'input, 'arena, Tok = CommonToken<'input>> = AContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { SimpleLRParserNodeKind, AContext, AContextExt, RULE_a }

pub trait AContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AContextAttrs<'input, 'arena, Tok> for AContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == SimpleLR_ID)
    }
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> SimpleLRParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  a(&mut self,) -> Result<&'arena AContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.a_rec(0)
	}

	fn a_rec(&mut self, _p: i32) -> Result<&'arena AContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(rec recog = self, _parentctx, _parentState, AContext<TF::Tok>, RULE_a, 2, |_parentctx| AContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena AContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		{
		recog.base.set_state(7);
		recog.base.match_token(SimpleLR_ID,&mut recog.err_handler)?;
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(13);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(0,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				{
				/*recRuleAltStartAction*/
				let tmp = AContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
				let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_a)?;

				recog.base.set_state(9);
				if !({recog.precpred(None, 2)}) {
					Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
				}
				recog.base.set_state(10);
				recog.base.match_token(SimpleLR_ID,&mut recog.err_handler)?;
				}
				} 
			}
			recog.base.set_state(15);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(0,&mut recog.base)?;
		}
		}
		})
	}
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 3] = [
    "CAIEIgQADgAEAg4CAgACAAICAgICAgICAgIKAhgQAhQCGAIeEgICAgACBAQABAAAHgAIAgAAAAQMAgAA",
    "AAgKBgQCAAoCAgAAAAwODAIBAA4QCgIAABAaAgAAABIUFAQAABQYCgIAABYSAgAAABgeAgAAABoWAgAA",
    "ABocAgAAABwGAgAAAB4aAgAAAAIa"
];