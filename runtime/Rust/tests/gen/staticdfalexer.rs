// Generated from StaticDFA.g4 by ANTLR 4.13.2
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
pub const T__3:i32=4; 
pub const T__4:i32=5; 
pub const T__5:i32=6; 
pub const T__6:i32=7; 
pub const T__7:i32=8; 
pub const T__8:i32=9; 
pub const T__9:i32=10; 
pub const T__10:i32=11; 
pub const T__11:i32=12; 
pub const ID:i32=13; 
pub const WS:i32=14;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;14] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "T__9", "T__10", "T__11", "ID", "WS"
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

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, StaticDFALexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct StaticDFALexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr4::impl_token_source! { StaticDFALexer }
dbt_antlr4::impl_deref! { lexer => StaticDFALexer }

impl<'input, 'arena, Input, TF> StaticDFALexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = StaticDFALexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct StaticDFALexerActions {
}

impl StaticDFALexerActions {
}

dbt_antlr4::impl_lexer_recog! { StaticDFALexerActions, "StaticDFALexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 13] = [
    "CAAcoAEMAQQADgAEAg4CBAQOBAQGDgYECA4IBAoOCgQMDgwEDg4OBBAOEAQSDhIEFA4UBBYOFgQYDhgE",
    "Gg4aAgACAAIAAgACAAIAAgACAgICAgICAgICAgICBAIEAgQCBAIEAgYCBgIGAgYCBgIIAggCCgIKAgwC",
    "DAIOAg4CEAIQAhICEgIUAhQCFgIWAhgIGIwBEBgWGBgYjgECGggalgEQGhYaGBqYAQIaAhoAABwCAgYE",
    "CgYOCBIKFgwaDh4QIhImFCoWLhgyGjYcAgAEAgDCAfQBBgASFBoaQECiAQACAgAAAAAGAgAAAAAKAgAA",
    "AAAOAgAAAAASAgAAAAAWAgAAAAAaAgAAAAAeAgAAAAAiAgAAAAAmAgAAAAAqAgAAAAAuAgAAAAAyAgAA",
    "AAA2AgAAAAI6AgAAAAZIAgAAAApUAgAAAA5eAgAAABJoAgAAABZsAgAAABpwAgAAAB50AgAAACJ4AgAA",
    "ACZ8AgAAACqAAQIAAAAuhAECAAAAMooBAgAAADaUAQIAAAA6PArGAQAAPD4K5AEAAD5ACsoBAABAQgrC",
    "AQAAQkQK6AEAAERGCsoBAABGBAIAAABISgroAQAASkwKwgEAAExOCsQBAABOUArYAQAAUFIKygEAAFII",
    "AgAAAFRWCuwBAABWWArSAQAAWFoKygEAAFpcCu4BAABcDAIAAABeYArIAQAAYGIK5AEAAGJkCt4BAABk",
    "ZgrgAQAAZhACAAAAaGoKUAAAahQCAAAAbG4KUgAAbhgCAAAAcHIKXAAAchwCAAAAdHYKwgEAAHYgAgAA",
    "AHh6CsQBAAB6JAIAAAB8fgrGAQAAfigCAAAAgAGCAQrIAQAAggEsAgAAAIQBhgEKygEAAIYBMAIAAACI",
    "AYwBDgAAAIoBiAECAAAAjAGOAQIAAACOAYoBAgAAAI4BkAECAAAAkAE0AgAAAJIBlgEOAgAAlAGSAQIA",
    "AACWAZgBAgAAAJgBlAECAAAAmAGaAQIAAACaAZwBAgAAAJwBngEMGgAAngE4AgAAAAYAjgGYAQIMAAA="
];