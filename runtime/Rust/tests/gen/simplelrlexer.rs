// Generated from SimpleLR.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unused_variables)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::prelude::*;
use dbt_antlr4::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr4::check_version!("2","0");
pub const ID:i32=1; 
pub const WS:i32=2;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;2] = [
    "ID", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;0] = [
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;3]  = [
	None, Some("ID"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, SimpleLRLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct SimpleLRLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr4::impl_token_source! { SimpleLRLexer }
dbt_antlr4::impl_deref! { lexer => SimpleLRLexer }

impl<'input, 'arena, Input, TF> SimpleLRLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = SimpleLRLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct SimpleLRLexerActions {
}

impl SimpleLRLexerActions {
}

dbt_antlr4::impl_lexer_recog! { SimpleLRLexerActions, "SimpleLRLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 3] = [
    "CAAEHAwBBAAOAAQCDgICAAgADhAAFgAYABACAgICAgICAgAABAICBgQCAAIEABQUQEAcAAICAAAAAAYC",
    "AAAAAgwCAAAABhQCAAAACg4EwgH0AQAMCgIAAAAOEAIAAAAQDAIAAAAQEgIAAAASBAIAAAAUFg4AAAAW",
    "GAIAAAAYGgwCAAAaCAIAAAAEABACDAAA"
];