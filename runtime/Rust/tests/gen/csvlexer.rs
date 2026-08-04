// Generated from CSV.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unused_variables)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::prelude::*;
use dbt_antlr4::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr4::check_version!("2","0");
pub const T__0:i32=1; 
pub const T__1:i32=2; 
pub const T__2:i32=3; 
pub const WS:i32=4; 
pub const TEXT:i32=5; 
pub const STRING:i32=6;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;6] = [
    "T__0", "T__1", "T__2", "WS", "TEXT", "STRING"
];
pub const _LITERAL_NAMES: [Option<&'static str>;4] = [
	None, Some("','"), Some("'\\r'"), Some("'\\n'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;7]  = [
	None, None, None, None, Some("WS"), Some("TEXT"), Some("STRING")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, CSVLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct CSVLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr4::impl_token_source! { CSVLexer }
dbt_antlr4::impl_deref! { lexer => CSVLexer }

impl<'input, 'arena, Input, TF> CSVLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = CSVLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct CSVLexerActions {
}

impl CSVLexerActions {
}

dbt_antlr4::impl_lexer_recog! { CSVLexerActions, "CSVLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 7] = [
    "CAAMVAwBBAAOAAQCDgIEBA4EBAYOBgQIDggECg4KAgACAAICAgICBAIEAgYIBioQBhYGGAYsAgYCBgII",
    "CAg4EAgWCBgIOgIKAgoCCgIKCgpIEAoUChgKThIKAgoCCgAADAICBgQKBg4IEgoWDAIABgIAQEAKABQU",
    "GhpAQEREWFgCAEREWgACAgAAAAAGAgAAAAAKAgAAAAAOAgAAAAASAgAAAAAWAgAAAAIaAgAAAAYeAgAA",
    "AAoiAgAAAA4oAgAAABI2AgAAABY+AgAAABocClgAABwEAgAAAB4gChoAACAIAgAAACIkChQAACQMAgAA",
    "ACYqDgAAACgmAgAAACosAgAAACwoAgAAACwuAgAAAC4wAgAAADAyDAYAADIQAgAAADQ4EAIAADY0AgAA",
    "ADg6AgAAADo2AgAAADo8AgAAADwUAgAAAD5KCkQAAEBCCkQAAEJICkQAAERIEAQAAEZAAgAAAEZEAgAA",
    "AEhOAgAAAEpGAgAAAEpMAgAAAExQAgAAAE5KAgAAAFBSCkQAAFIYAgAAAAoALDpGSgIAAgA="
];
