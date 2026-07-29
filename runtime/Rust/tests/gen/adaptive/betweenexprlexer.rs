// Generated from BetweenExpr.g4 by ANTLR 4.13.2
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
pub const T__12:i32=13; 
pub const T__13:i32=14; 
pub const ID:i32=15; 
pub const INT:i32=16; 
pub const WS:i32=17;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;17] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "T__9", "T__10", "T__11", "T__12", "T__13", "ID", "INT", "WS"
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

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, BetweenExprLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct BetweenExprLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr4::impl_token_source! { BetweenExprLexer }
dbt_antlr4::impl_deref! { lexer => BetweenExprLexer }

impl<'input, 'arena, Input, TF> BetweenExprLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = BetweenExprLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct BetweenExprLexerActions {
}

impl BetweenExprLexerActions {
}

dbt_antlr4::impl_lexer_recog! { BetweenExprLexerActions, "BetweenExprLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 16] = [
    "CAAiuAEMAQQADgAEAg4CBAQOBAQGDgYECA4IBAoOCgQMDgwEDg4OBBAOEAQSDhIEFA4UBBYOFgQYDhgE",
    "Gg4aBBwOHAQeDh4EIA4gAgACAAIAAgACAgICAgICAgIEAgQCBAIGAgYCBgIGAgYCBgIGAgYCCAIIAgoC",
    "CgIMAgwCDgIOAhACEAISAhICFAIUAhYCFgIYAhgCGgIaAhoCHAgcmgEQHBYcGBycAQIeCB6kARAeFh4Y",
    "HqYBAiAIIK4BECAWIBggsAECIAIgAAAiAgIGBAoGDggSChYMGg4eECISJhQqFi4YMho2HDoePiBCIgIA",
    "BgIAwgH0AQIAYHIEABIUQEC8AQACAgAAAAAGAgAAAAAKAgAAAAAOAgAAAAASAgAAAAAWAgAAAAAaAgAA",
    "AAAeAgAAAAAiAgAAAAAmAgAAAAAqAgAAAAAuAgAAAAAyAgAAAAA2AgAAAAA6AgAAAAA+AgAAAABCAgAA",
    "AAJGAgAAAAZOAgAAAApWAgAAAA5cAgAAABJsAgAAABZwAgAAABp0AgAAAB54AgAAACJ8AgAAACaAAQIA",
    "AAAqhAECAAAALogBAgAAADKMAQIAAAA2kAECAAAAOpgBAgAAAD6iAQIAAABCrAECAAAARkgKnAEAAEhK",
    "Cp4BAABKTAqoAQAATAQCAAAATlAKggEAAFBSCpwBAABSVAqIAQAAVAgCAAAAVlgKngEAAFhaCqQBAABa",
    "DAIAAABcXgqEAQAAXmAKigEAAGBiCqgBAABiZAquAQAAZGYKigEAAGZoCooBAABoagqcAQAAahACAAAA",
    "bG4KegAAbhQCAAAAcHIKeAAAchgCAAAAdHYKfAAAdhwCAAAAeHoKVAAAeiACAAAAfH4KXgAAfiQCAAAA",
    "gAGCAQpWAACCASgCAAAAhAGGAQpaAACGASwCAAAAiAGKAQpQAACKATACAAAAjAGOAQpSAACOATQCAAAA",
    "kAGSAQpaAACSAZQBCnwAAJQBOAIAAACWAZoBDgAAAJgBlgECAAAAmgGcAQIAAACcAZgBAgAAAJwBngEC",
    "AAAAngE8AgAAAKABpAEOAgAAogGgAQIAAACkAaYBAgAAAKYBogECAAAApgGoAQIAAACoAUACAAAAqgGu",
    "AQ4EAACsAaoBAgAAAK4BsAECAAAAsAGsAQIAAACwAbIBAgAAALIBtAECAAAAtAG2AQwgAAC2AUQCAAAA",
    "CACcAaYBsAECDAAA"
];