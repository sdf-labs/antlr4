// Generated from CSV.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::prelude::*;
use dbt_antlr4::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use super::csvlistener::*;
use super::csvvisitor::*;

dbt_antlr4::check_version!("2","0");
pub const CSV_T__0:i32=1; 
pub const CSV_T__1:i32=2; 
pub const CSV_T__2:i32=3; 
pub const CSV_WS:i32=4; 
pub const CSV_TEXT:i32=5; 
pub const CSV_STRING:i32=6;
pub const CSV_EOF:i32=EOF;
pub const RULE_csvFile:usize = 0; 
pub const RULE_hdr:usize = 1; 
pub const RULE_row:usize = 2; 
pub const RULE_field:usize = 3;
pub const ruleNames: [&'static str; 4] = [
    "csvFile", "hdr", "row", "field"
];

pub const _LITERAL_NAMES: [Option<&'static str>;4] = [
	None, Some("','"), Some("'\\r'"), Some("'\\n'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;7]  = [
	None, None, None, None, Some("WS"), Some("TEXT"), Some("STRING")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, CSVParserExt<'input, 'arena>, CSVParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct CSVParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> CSVParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				CSVParserExt {
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
        L: CSVListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub trait Visitable<'input: 'arena, 'arena, Tok: Token + 'input> {
    fn accept<V>(&'arena self, visitor: &mut V) -> Result<V::Return, ANTLRError>
    where
        'input: 'arena,
        V: CSVVisitor<'input, 'arena, Tok> + ?Sized;
}
pub struct CSVTreeWalker;
impl CSVTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: CSVListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, CSVParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <CSVParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum CSVParserNodeKind {
    CsvFileContext,
    HdrContext,
    RowContext,
    FieldContext,
    Terminal,
    Error,
}
pub type CSVParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, CSVParserNodeKind, Tok>;

dbt_antlr4::impl_deref! { parser => CSVParser }
dbt_antlr4::impl_node_kind! { CSVParserNodeKind {
; CsvFileContext(enter_csvFile, exit_csvFile,  visit_csvFile), HdrContext(enter_hdr, exit_hdr,  visit_hdr), RowContext(enter_row, exit_row,  visit_row), FieldContext(enter_field, exit_field,  visit_field), 
    }; listener = dyn CSVListener<'arena, Tok>, visitor = CSVVisitor,
}

pub struct CSVParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> CSVParserExt<'input, 'arena> {
}

dbt_antlr4::impl_parser_recog! { CSVParserExt, CSVParserNodeKind, "CSV.g4" }

//------------------- csvFile ----------------
pub type CsvFileContextAll<'input, 'arena, Tok = CommonToken<'input>> = CsvFileContext<'input, 'arena, Tok>;

dbt_antlr4::impl_visitable! { CSVVisitor::CsvFileContext(visit_csvFile) }
dbt_antlr4::impl_ctx! { CSVParserNodeKind, CsvFileContext, CsvFileContextExt, RULE_csvFile }

pub trait CsvFileContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn hdr(&self) -> Option<&'arena HdrContextAll<'input, 'arena, Tok>>;
    fn row_all(&self) -> Vec<&'arena RowContextAll<'input, 'arena, Tok>>;
    fn row(&self, i: usize) -> Option<&'arena RowContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> CsvFileContextAttrs<'input, 'arena, Tok> for CsvFileContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn hdr(&self) -> Option<&'arena HdrContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn row_all(&self) -> Vec<&'arena RowContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn row(&self, i: usize) -> Option<&'arena RowContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> CSVParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn csvFile(&mut self,) -> Result<&'arena CsvFileContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, CsvFileContext<TF::Tok>, RULE_csvFile, 0, |_parentctx| CsvFileContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena CsvFileContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(8);
		recog.hdr()?;
		recog.base.set_state(10); 
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		loop {
			{
			{
			recog.base.set_state(9);
			recog.row()?;
			}
			}
			recog.base.set_state(12); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 110) != 0)) {break}
		}
		}
		})
	}
}
//------------------- hdr ----------------
pub type HdrContextAll<'input, 'arena, Tok = CommonToken<'input>> = HdrContext<'input, 'arena, Tok>;

dbt_antlr4::impl_visitable! { CSVVisitor::HdrContext(visit_hdr) }
dbt_antlr4::impl_ctx! { CSVParserNodeKind, HdrContext, HdrContextExt, RULE_hdr }

pub trait HdrContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn row(&self) -> Option<&'arena RowContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> HdrContextAttrs<'input, 'arena, Tok> for HdrContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn row(&self) -> Option<&'arena RowContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> CSVParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn hdr(&mut self,) -> Result<&'arena HdrContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, HdrContext<TF::Tok>, RULE_hdr, 2, |_parentctx| HdrContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena HdrContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(14);
		recog.row()?;
		}
		})
	}
}
//------------------- row ----------------
pub type RowContextAll<'input, 'arena, Tok = CommonToken<'input>> = RowContext<'input, 'arena, Tok>;

dbt_antlr4::impl_visitable! { CSVVisitor::RowContext(visit_row) }
dbt_antlr4::impl_ctx! { CSVParserNodeKind, RowContext, RowContextExt, RULE_row }

pub trait RowContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn field_all(&self) -> Vec<&'arena FieldContextAll<'input, 'arena, Tok>>;
    fn field(&self, i: usize) -> Option<&'arena FieldContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RowContextAttrs<'input, 'arena, Tok> for RowContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn field_all(&self) -> Vec<&'arena FieldContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn field(&self, i: usize) -> Option<&'arena FieldContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> CSVParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn row(&mut self,) -> Result<&'arena RowContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, RowContext<TF::Tok>, RULE_row, 4, |_parentctx| RowContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena RowContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(16);
		recog.field()?;
		recog.base.set_state(21);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		while _la==CSV_T__0 {
			{
			{
			recog.base.set_state(17);
			recog.base.match_token(CSV_T__0,&mut recog.err_handler)?;
			recog.base.set_state(18);
			recog.field()?;
			}
			}
			recog.base.set_state(23);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
		}
		recog.base.set_state(25);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==CSV_T__1 {
			{
			recog.base.set_state(24);
			recog.base.match_token(CSV_T__1,&mut recog.err_handler)?;
			}
		}

		recog.base.set_state(27);
		recog.base.match_token(CSV_T__2,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- field ----------------
pub type FieldContextAll<'input, 'arena, Tok = CommonToken<'input>> = FieldContext<'input, 'arena, Tok>;

dbt_antlr4::impl_visitable! { CSVVisitor::FieldContext(visit_field) }
dbt_antlr4::impl_ctx! { CSVParserNodeKind, FieldContext, FieldContextExt, RULE_field }

pub trait FieldContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token TEXT
    /// Returns `None` if there is no child corresponding to token TEXT
    fn TEXT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> FieldContextAttrs<'input, 'arena, Tok> for FieldContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token TEXT
    /// Returns `None` if there is no child corresponding to token TEXT
    fn TEXT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CSV_TEXT)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == CSV_STRING)
    }
}

impl<'input, 'arena, Input, TF> CSVParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn field(&mut self,) -> Result<&'arena FieldContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr4::parse_rule!(recog = self, _parentctx, FieldContext<TF::Tok>, RULE_field, 6, |_parentctx| FieldContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena FieldContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(32);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    CSV_TEXT  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(29);
		        recog.base.match_token(CSV_TEXT,&mut recog.err_handler)?;
		        }}
		    CSV_STRING  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(30);
		        recog.base.match_token(CSV_STRING,&mut recog.err_handler)?;
		        }}
		    CSV_T__0 |CSV_T__1 |CSV_T__2  => {
		        /*------- Outer Most Alt 3 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
		        {
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 6] = [
    "CAIMRgQADgAEAg4CBAQOBAQGDgYCAAIACAAWEAAWABgAGAICAgICBAIEAgQKBCgQBBQEGAQuEgQCBAYE",
    "NBAEAgQCBAIGAgYCBgYGQhAGAgYAAAgABAgMAABGABACAAAABBwCAAAACCACAAAADEACAAAAEBQGBAIA",
    "EhYGCAQAFBICAAAAFhgCAAAAGBQCAAAAGBoCAAAAGgICAAAAHB4GCAQAHgYCAAAAICoGDAYAIiQKAgAA",
    "JCgGDAYAJiICAAAAKC4CAAAAKiYCAAAAKiwCAAAALDICAAAALioCAAAAMDQKBAAAMjACAAAAMjQCAAAA",
    "NDYCAAAANjgKBgAAOAoCAAAAOkIKCgAAPEIKDAAAPkICAAAAQDoCAAAAQDwCAAAAQD4CAAAAQg4CAAAA",
    "CBgqMkA="
];