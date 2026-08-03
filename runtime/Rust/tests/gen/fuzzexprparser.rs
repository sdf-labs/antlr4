// Generated from FuzzExpr.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::prelude::*;
use dbt_antlr4::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use super::fuzzexprlistener::*;
dbt_antlr4::check_version!("2","0");
pub const FuzzExpr_T__0:i32=1; 
pub const FuzzExpr_T__1:i32=2; 
pub const FuzzExpr_T__2:i32=3; 
pub const FuzzExpr_T__3:i32=4; 
pub const FuzzExpr_T__4:i32=5; 
pub const FuzzExpr_T__5:i32=6; 
pub const FuzzExpr_T__6:i32=7; 
pub const FuzzExpr_T__7:i32=8; 
pub const FuzzExpr_T__8:i32=9; 
pub const FuzzExpr_T__9:i32=10; 
pub const FuzzExpr_T__10:i32=11; 
pub const FuzzExpr_T__11:i32=12; 
pub const FuzzExpr_T__12:i32=13; 
pub const FuzzExpr_ID:i32=14; 
pub const FuzzExpr_INT:i32=15; 
pub const FuzzExpr_WS:i32=16;
pub const FuzzExpr_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_item:usize = 1; 
pub const RULE_name:usize = 2; 
pub const RULE_q:usize = 3; 
pub const RULE_e:usize = 4; 
pub const RULE_atom:usize = 5;
pub const ruleNames: [&'static str; 6] = [
    "s", "item", "name", "q", "e", "atom"
];

pub const _LITERAL_NAMES: [Option<&'static str>;14] = [
	None, Some("','"), Some("';'"), Some("'as'"), Some("'.'"), Some("'['"), 
	Some("']'"), Some("'-'"), Some("'^'"), Some("'*'"), Some("'/'"), Some("'+'"), 
	Some("'('"), Some("')'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;17]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, FuzzExprParserExt<'input, 'arena>, FuzzExprParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				FuzzExprParserExt {
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
        L: FuzzExprListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct FuzzExprTreeWalker;
impl FuzzExprTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: FuzzExprListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, FuzzExprParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <FuzzExprParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum FuzzExprParserNodeKind {
    SContext,
    ItemContext,
    NameContext,
    QContext,
    EContext,
    AtomContext,
    Terminal,
    Error,
}
pub type FuzzExprParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, FuzzExprParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => FuzzExprParser }
dbt_antlr4::impl_node_kind! { FuzzExprParserNodeKind {
; SContext(enter_s, exit_s, ), ItemContext(enter_item, exit_item, ), NameContext(enter_name, exit_name, ), QContext(enter_q, exit_q, ), EContext(enter_e, exit_e, ), AtomContext(enter_atom, exit_atom, ), 
    }; listener = dyn FuzzExprListener<'arena, Tok>,
}

pub struct FuzzExprParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> FuzzExprParserExt<'input, 'arena> {
}

dbt_antlr4::impl_parser_recog! { FuzzExprParserExt, FuzzExprParserNodeKind, "FuzzExpr.g4"; sempred (FuzzExprParserNode<'input, 'arena, TF::Tok>, FuzzExprParser) { 4 => e_sempred, } }

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn e_sempred(_ctx: Option<&'arena EContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 4)
		    }
	        1 => {
			recog.precpred(None, 3)
		    }
	        2 => {
			recog.precpred(None, 2)
		    }
	        3 => {
			recog.precpred(None, 6)
		    }
		    _ => true
		}
	}
}
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { FuzzExprParserNodeKind, SContext, SContextExt, RULE_s }

pub trait SContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn item_all(&self) -> Vec<&'arena ItemContextAll<'input, 'arena, Tok>>;
    fn item(&self, i: usize) -> Option<&'arena ItemContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SContextAttrs<'input, 'arena, Tok> for SContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn item_all(&self) -> Vec<&'arena ItemContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn item(&self, i: usize) -> Option<&'arena ItemContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == FuzzExpr_EOF)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn s(&mut self,) -> Result<&'arena SContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, SContext<TF::Tok>, RULE_s, 0, |_parentctx| SContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(12);
		recog.item()?;
		recog.base.set_state(17);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		while _la==FuzzExpr_T__0 {
			{
			{
			recog.base.set_state(13);
			recog.base.match_token(FuzzExpr_T__0,&mut recog.err_handler)?;
			recog.base.set_state(14);
			recog.item()?;
			}
			}
			recog.base.set_state(19);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
		}
		recog.base.set_state(21);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==FuzzExpr_T__1 {
			{
			recog.base.set_state(20);
			recog.base.match_token(FuzzExpr_T__1,&mut recog.err_handler)?;
			}
		}

		recog.base.set_state(23);
		recog.base.match_token(FuzzExpr_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- item ----------------
pub type ItemContextAll<'input, 'arena, Tok = CommonToken<'input>> = ItemContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { FuzzExprParserNodeKind, ItemContext, ItemContextExt, RULE_item }

pub trait ItemContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ItemContextAttrs<'input, 'arena, Tok> for ItemContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn item(&mut self,) -> Result<&'arena ItemContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, ItemContext<TF::Tok>, RULE_item, 2, |_parentctx| ItemContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ItemContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(30);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _m = recog.base.dfa_predict_mask(2)?;
		  match _m {
		    x if x != 0 && (x & !0x3) == 0 && (x & (x-1)) != 0 && !recog.base.resume_active() && [FuzzExpr_T__2 | FuzzExpr_T__6 | FuzzExpr_T__11 | FuzzExpr_ID | FuzzExpr_INT].contains(&recog.input.la(1)) => {
		        recog.base.set_state(25);
		        let _neutral_state = recog.base.begin_neutral_parse(0);
		        match recog.e() {
		            Err(e) if !e.is_recoverable() => {
		                    recog.base.end_neutral_parse(&_neutral_state);
		                    return Err(e)
		                },
		            Err(_) => {
		                recog.base.end_neutral_parse(&_neutral_state);
		                1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		            },
		            Ok(_node) => {
		                if recog.base.syntax_error_count() != _neutral_state.2 {
		                    recog.base.end_neutral_parse(&_neutral_state);
		                    1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		                }
		                else {
		                    let _tailbit = { let _t = recog.input.la(1); match _t {
		                        FuzzExpr_T__2 if !false || !recog.base.follow_contains(_t) => Some(0x1),
		                        _ => Some(0x2),
		                    } };
		                    match _tailbit {
		                        Some(bit) => {
		                            recog.base.start_resume(_node.as_node(), RULE_e);
		                            recog.base.end_neutral_parse(&_neutral_state);
		                            bit
		                        }
		                        None => {
		                            recog.base.end_neutral_parse(&_neutral_state);
		                            1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		                        }
		                    }
		                }
		            },
		        }
		    },
		    x if x != 0 && (x & !0x3) == 0 && (x & (x-1)) != 0 => 1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32),
		    x if x != 0 && (x & !0x3) == 0 && (x & (x-1)) != 0 && !recog.base.resume_active() && [FuzzExpr_T__2 | FuzzExpr_T__11 | FuzzExpr_ID | FuzzExpr_INT].contains(&recog.input.la(1)) => {
		        recog.base.set_state(45);
		        let _neutral_state = recog.base.begin_neutral_parse(0);
		        match recog.atom() {
		            Err(e) if !e.is_recoverable() => {
		                    recog.base.end_neutral_parse(&_neutral_state);
		                    return Err(e)
		                },
		            Err(_) => {
		                recog.base.end_neutral_parse(&_neutral_state);
		                1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		            },
		            Ok(_node) => {
		                if recog.base.syntax_error_count() != _neutral_state.2 {
		                    recog.base.end_neutral_parse(&_neutral_state);
		                    1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		                }
		                else {
		                    let _tailbit = { let _t = recog.input.la(1); match _t {
		                        FuzzExpr_T__2 if !true || !recog.base.follow_contains(_t) => Some(0x1),
		                        _ => None,
		                    } };
		                    match _tailbit {
		                        Some(bit) => {
		                            recog.base.start_resume(_node.as_node(), RULE_atom);
		                            recog.base.end_neutral_parse(&_neutral_state);
		                            bit
		                        }
		                        None => {
		                            recog.base.end_neutral_parse(&_neutral_state);
		                            1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		                        }
		                    }
		                }
		            },
		        }
		    },
		    x if x != 0 && (x & !0x3) == 0 && (x & (x-1)) != 0 => 1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32),
		    x if x != 0 && (x & !0x3) == 0 && (x & (x-1)) != 0 && !recog.base.resume_active() && [FuzzExpr_T__2 | FuzzExpr_ID].contains(&recog.input.la(1)) => {
		        recog.base.set_state(67);
		        let _neutral_state = recog.base.begin_neutral_parse(0);
		        match recog.q() {
		            Err(e) if !e.is_recoverable() => {
		                    recog.base.end_neutral_parse(&_neutral_state);
		                    return Err(e)
		                },
		            Err(_) => {
		                recog.base.end_neutral_parse(&_neutral_state);
		                1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		            },
		            Ok(_node) => {
		                if recog.base.syntax_error_count() != _neutral_state.2 {
		                    recog.base.end_neutral_parse(&_neutral_state);
		                    1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		                }
		                else {
		                    let _tailbit = { let _t = recog.input.la(1); match _t {
		                        FuzzExpr_T__2 if !true || !recog.base.follow_contains(_t) => Some(0x1),
		                        _ => None,
		                    } };
		                    match _tailbit {
		                        Some(bit) => {
		                            recog.base.start_resume(_node.as_node(), RULE_q);
		                            recog.base.end_neutral_parse(&_neutral_state);
		                            bit
		                        }
		                        None => {
		                            recog.base.end_neutral_parse(&_neutral_state);
		                            1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		                        }
		                    }
		                }
		            },
		        }
		    },
		    x if x != 0 && (x & !0x3) == 0 && (x & (x-1)) != 0 => 1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32),
		    x if x != 0 && (x & !0x3) == 0 && (x & (x-1)) != 0 && !recog.base.resume_active() && [FuzzExpr_T__2 | FuzzExpr_ID].contains(&recog.input.la(1)) => {
		        recog.base.set_state(34);
		        let _neutral_state = recog.base.begin_neutral_parse(0);
		        match recog.name() {
		            Err(e) if !e.is_recoverable() => {
		                    recog.base.end_neutral_parse(&_neutral_state);
		                    return Err(e)
		                },
		            Err(_) => {
		                recog.base.end_neutral_parse(&_neutral_state);
		                1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		            },
		            Ok(_node) => {
		                if recog.base.syntax_error_count() != _neutral_state.2 {
		                    recog.base.end_neutral_parse(&_neutral_state);
		                    1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		                }
		                else {
		                    let _tailbit = { let _t = recog.input.la(1); match _t {
		                        FuzzExpr_T__2 if !true || !recog.base.follow_contains(_t) => Some(0x1),
		                        _ => None,
		                    } };
		                    match _tailbit {
		                        Some(bit) => {
		                            recog.base.start_resume(_node.as_node(), RULE_name);
		                            recog.base.end_neutral_parse(&_neutral_state);
		                            bit
		                        }
		                        None => {
		                            recog.base.end_neutral_parse(&_neutral_state);
		                            1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32)
		                        }
		                    }
		                }
		            },
		        }
		    },
		    x if x != 0 && (x & !0x3) == 0 && (x & (x-1)) != 0 => 1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32),
		    x if x == 0 || (x & (x-1)) != 0 => 1u64.wrapping_shl((recog.get_interpreter().adaptive_predict(2,&mut recog.base)? - 1) as u32),
		    m => m,
		  } } {
		x if x == 0x1 =>{
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(25);
			recog.e_rec(0)?;
			recog.base.set_state(26);
			recog.base.match_token(FuzzExpr_T__2,&mut recog.err_handler)?;
			recog.base.set_state(27);
			recog.name()?;
			}
		},
		x if x == 0x2 =>{
			/*------- Outer Most Alt 2 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			{
			recog.base.set_state(29);
			recog.e_rec(0)?;
			}
		}
			_ => {}
		}
		})
	}
}
//------------------- name ----------------
pub type NameContextAll<'input, 'arena, Tok = CommonToken<'input>> = NameContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { FuzzExprParserNodeKind, NameContext, NameContextExt, RULE_name }

pub trait NameContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> NameContextAttrs<'input, 'arena, Tok> for NameContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == FuzzExpr_ID)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
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
		_la = recog.base.input.la(1);
		if { !(_la==FuzzExpr_T__2 || _la==FuzzExpr_ID) } {
			recog.err_handler.recover_inline(&mut recog.base)?;
		}
		else {
			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
			recog.err_handler.report_match(&mut recog.base);
			recog.base.consume(&mut recog.err_handler)?;
		}
		}
		})
	}
}
//------------------- q ----------------
pub type QContextAll<'input, 'arena, Tok = CommonToken<'input>> = QContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { FuzzExprParserNodeKind, QContext, QContextExt, RULE_q }

pub trait QContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn name_all(&self) -> Vec<&'arena NameContextAll<'input, 'arena, Tok>>;
    fn name(&self, i: usize) -> Option<&'arena NameContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> QContextAttrs<'input, 'arena, Tok> for QContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn name_all(&self) -> Vec<&'arena NameContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn name(&self, i: usize) -> Option<&'arena NameContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn q(&mut self,) -> Result<&'arena QContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		if let Some(_node) = self.base.resume_take(RULE_q)? {
		    return Ok(_node.as_rule_context().unwrap());
		}
        dbt_antlr4::parse_rule!(recog = self, _parentctx, QContext<TF::Tok>, RULE_q, 6, |_parentctx| QContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena QContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(34);
		recog.name()?;
		recog.base.set_state(39);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(3)?; _sdp };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				{
				{
				recog.base.set_state(35);
				recog.base.match_token(FuzzExpr_T__3,&mut recog.err_handler)?;
				recog.base.set_state(36);
				recog.name()?;
				}
				} 
			}
			recog.base.set_state(41);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(3)?; _sdp };
		}
		}
		})
	}
}
//------------------- e ----------------
pub type EContextAll<'input, 'arena, Tok = CommonToken<'input>> = EContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { FuzzExprParserNodeKind, EContext, EContextExt, RULE_e }

pub trait EContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> EContextAttrs<'input, 'arena, Tok> for EContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
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
        dbt_antlr4::parse_rule!(rec recog = self, _parentctx, _parentState, EContext<TF::Tok>, RULE_e, 8, |_parentctx| EContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena EContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 8;
		let mut _la: i32 = -1;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(46);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    FuzzExpr_T__6  => {
		        {
		        recog.base.set_state(43);
		        recog.base.match_token(FuzzExpr_T__6,&mut recog.err_handler)?;
		        recog.base.set_state(44);
		        recog.e_rec(5)?;
		        }}
		    FuzzExpr_T__2 |FuzzExpr_T__11 |FuzzExpr_ID |FuzzExpr_INT  => {
		        {
		        recog.base.set_state(45);
		        recog.atom()?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(64);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(6)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(6,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(62);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(5)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(48);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(49);
						recog.base.match_token(FuzzExpr_T__7,&mut recog.err_handler)?;
						recog.base.set_state(50);
						recog.e_rec(4)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(51);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(52);
						_la = recog.base.input.la(1);
						if { !(_la==FuzzExpr_T__8 || _la==FuzzExpr_T__9) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(53);
						recog.e_rec(4)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(54);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(55);
						_la = recog.base.input.la(1);
						if { !(_la==FuzzExpr_T__6 || _la==FuzzExpr_T__10) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(56);
						recog.e_rec(3)?;
						}
					}
				,
					4 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(57);
						if !({recog.precpred(None, 6)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
						}
						recog.base.set_state(58);
						recog.base.match_token(FuzzExpr_T__4,&mut recog.err_handler)?;
						recog.base.set_state(59);
						recog.e_rec(0)?;
						recog.base.set_state(60);
						recog.base.match_token(FuzzExpr_T__5,&mut recog.err_handler)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(66);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(6)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(6,&mut recog.base)? } else { _sdp } };
		}
		}
		})
	}
}
//------------------- atom ----------------
pub type AtomContextAll<'input, 'arena, Tok = CommonToken<'input>> = AtomContext<'input, 'arena, Tok>;

dbt_antlr4::impl_ctx! { FuzzExprParserNodeKind, AtomContext, AtomContextExt, RULE_atom }

pub trait AtomContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AtomContextAttrs<'input, 'arena, Tok> for AtomContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == FuzzExpr_INT)
    }
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == FuzzExpr_ID)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn atom(&mut self,) -> Result<&'arena AtomContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		if let Some(_node) = self.base.resume_take(RULE_atom)? {
		    return Ok(_node.as_rule_context().unwrap());
		}
        dbt_antlr4::parse_rule!(recog = self, _parentctx, AtomContext<TF::Tok>, RULE_atom, 10, |_parentctx| AtomContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena AtomContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(86);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(9)?; _sdp } {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(67);
				recog.q()?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(68);
				recog.base.match_token(FuzzExpr_INT,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(69);
				recog.base.match_token(FuzzExpr_T__11,&mut recog.err_handler)?;
				recog.base.set_state(70);
				recog.e_rec(0)?;
				recog.base.set_state(71);
				recog.base.match_token(FuzzExpr_T__12,&mut recog.err_handler)?;
				}
			}
		,
			4 =>{
				/*------- Outer Most Alt 4 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
				{
				recog.base.set_state(73);
				recog.base.match_token(FuzzExpr_ID,&mut recog.err_handler)?;
				recog.base.set_state(74);
				recog.base.match_token(FuzzExpr_T__11,&mut recog.err_handler)?;
				recog.base.set_state(83);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if (((_la) & !0x3f) == 0 && ((1usize << _la) & 53384) != 0) {
					{
					recog.base.set_state(75);
					recog.e_rec(0)?;
					recog.base.set_state(80);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==FuzzExpr_T__0 {
						{
						{
						recog.base.set_state(76);
						recog.base.match_token(FuzzExpr_T__0,&mut recog.err_handler)?;
						recog.base.set_state(77);
						recog.e_rec(0)?;
						}
						}
						recog.base.set_state(82);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				recog.base.set_state(85);
				recog.base.match_token(FuzzExpr_T__12,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}

// the serialized ATN is followed by 9 static SLL prediction tables (-Xstatic-dfa):
//   decision 6: precedence-dispatched over cutoffs [2, 3, 4, 6], tables [4 5 6 7 8]
//   decision 2: LL(*) cyclic, 32 states, 15 mask accepts
//   decision 3: LL(k), k=1, 10 states
//   decision 5: LL(k), k=1, 5 states
//   decision 9: LL(k), k=2, 15 states
//   table 4 (decision 6): LL(k), k=1, 9 states
//   table 5 (decision 6): LL(k), k=1, 10 states
//   table 6 (decision 6): LL(k), k=1, 10 states
//   table 7 (decision 6): LL(k), k=1, 9 states
//   table 8 (decision 6): LL(k), k=0, 1 states
static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 30] = [
    "CAIgsgEEAA4ABAIOAgQEDgQEBg4GBAgOCAQKDgoCAAIAAgAKACAQABQAGAAmEgACAAYALBAAAgACAAIC",
    "AgICAgICAgIGAj4QAgIEAgQCBgIGAgYKBkwQBhQGGAZSEgYCCAIIAggCCAYIXhAIAggCCAIIAggCCAII",
    "AggCCAIIAggCCAIIAggCCAoIfhAIFAgYCIQBEggCCgIKAgoCCgIKAgoCCgIKAgoCCgIKCgqeARAKFAoY",
    "CqQBEgoGCqgBEAoCCgYKrgEQCgIKAAIQDAAECAwQFAAGBAAGBhwcAgASFAQADg4WFsABABgCAAAABDwC",
    "AAAACEACAAAADEQCAAAAEFwCAAAAFKwBAgAAABgiBgQCABocCgIAABwgBgQCAB4aAgAAACAmAgAAACIe",
    "AgAAACIkAgAAACQqAgAAACYiAgAAACgsCgQAACooAgAAACosAgAAACwuAgAAAC4wCgAAAjACAgAAADI0",
    "BhAIADQ2CgYAADY4BggEADg+AgAAADo+BhAIADwyAgAAADw6AgAAAD4GAgAAAEBCDgAAAEIKAgAAAERO",
    "BggEAEZICggAAEhMBggEAEpGAgAAAExSAgAAAE5KAgAAAE5QAgAAAFAOAgAAAFJOAgAAAFRWDAgBAFZY",
    "Cg4AAFheBhAIClpeBhQKAFxUAgAAAFxaAgAAAF6AAQIAAABgYhQIAABiZAoQAABkfgYQCAhmaBQGAABo",
    "ag4CAABqfgYQCAhsbhQEAABucA4EAABwfgYQCAZydBQMAAB0dgoKAAB2eAYQCAB4egoMAAB6fgIAAAB8",
    "YAIAAAB8ZgIAAAB8bAIAAAB8cgIAAAB+hAECAAAAgAF8AgAAAIABggECAAAAggESAgAAAIQBgAECAAAA",
    "hgGuAQYMBgCIAa4BCh4AAIoBjAEKGAAAjAGOAQYQCACOAZABChoAAJABrgECAAAAkgGUAQocAACUAaYB",
    "ChgAAJYBoAEGEAgAmAGaAQoCAACaAZ4BBhAIAJwBmAECAAAAngGkAQIAAACgAZwBAgAAAKABogECAAAA",
    "ogGoAQIAAACkAaABAgAAAKYBlgECAAAApgGoAQIAAACoAaoBAgAAAKoBrgEKGgAArAGGAQIAAACsAYgB",
    "AgAAAKwBigECAAAArAGSAQIAAACuARYCAAAAFCIqPE5cfIABoAGmAawBDhIUBEDOAwAAAAAAAAQEBAIA",
    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAQEAAAAAAAAAAAAAAAAAAQAAAQEAAAAAAAAAAAAHkhISHic",
    "AZwBnAGcAZwBqAHGAeQB5AH8AfwB/AGaAqwC4gLiAuICngPOA84DzgPOA84DzgPOA84DzgMGBgIODgQY",
    "GAYcHAgeHgoBAQwCAg4EBBAGBhIICBQKChYOFhgBAQwCAg4EBBAGBhIICBQKChYOFhgYGBoBAQwCAg4E",
    "BBAGBhIKChYOFhgGBgIcHAIGBhwODh4YGCAcHCIeHiQGBiYODigYGCocHCweHi4ICDAKCjIMDAoOFjQI",
    "CDAKCjIMDAoOFjQYGDYKCjIMDAoOFjQBAQwCAg4EBBAGBhIICDgKCjoODhgQFDwWFhgBAQwCAg4EBBAG",
    "BhIICDgKCjoODhgQFDwWFhgYGD4BAQwCAg4EBBAGBhIKCjoODhgQFDwWFhgeBAQCBAYEAgQaBAIEHgQC",
    "BCAEAgQoBAIEKgQCBDAEAgQyBAIENAQCBDYEAgQ4BAIEOgQCBDwEAgQ+BAIEAAAABhQ2AAQEBAQCBAQE",
    "BAQAAAAAAAAAAAAANjY2NjY2NjY2NgEBAgICBAQEBgYGCAgICgoKDAwMDg4WEBoaEgAAAAAKCh4ACAYC",
    "BAAAAAAAAB4eHh4eCgoCDg4EEBAGEhQIFhYEAAAAABIeVAACBgAEAgICAgICAgIIAgAAAAIAAAAAAAAA",
    "AAAAAAAYGBhUVFRUVFRUVFRUVFQGBgIYGAQcHAYeHggBAQoCAgwEBA4GBhAICBIKChQMDBYOFhgYGBoa",
    "GhwAAAAAARIwAAQEBAQCBAIEBAAAAAAAAAAAADAwMDAwMDAwMAEBAgICBAQEBgYGCAoKCgwMDA4WDhoa",
    "EAAAAAABFDwABAQEBAIEBAIEBAAAAAAAAAAAAAA8PDw8PDw8PDw8AQECAgIEBAQGBgYICgoKDAwMDg4O",
    "EBQQFhYOGhoSAAAAAAEUPAAEBAQEAgQEAgQEAAAAAAAAAAAAADw8PDw8PDw8PDwBAQICAgQEBAYGBggK",
    "CgoMDAwODg4QEBASFg4aGhIAAAAAARIwAAQEBAQCBAQEBAAAAAAAAAAAADAwMDAwMDAwMAEBAgICBAQE",
    "BgYGCAoKCgwMDA4WDhoaEAAAAAABAgAEAAAAAAAAAAIMCAQGCAwICgwOEA=="
];