// Generated from XMLLexer.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unused_variables)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::Arena;
use dbt_antlr4::atn::ATN;
use dbt_antlr4::char_stream::CharStream;
use dbt_antlr4::int_stream::IntStream;
use dbt_antlr4::lexer::{BaseLexer, LexerRecog, Lexer as _};
use dbt_antlr4::atn_config_set::LexerATNConfigSet;
use dbt_antlr4::atn_deserializer::ATNDeserializer;
use dbt_antlr4::atn_simulator::BaseATNSimulator;
use dbt_antlr4::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;
use dbt_antlr4::TokenSource;
use dbt_antlr4::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use dbt_antlr4::PredictionContextCache;
use dbt_antlr4::recognizer::Actions;
use dbt_antlr4::token_factory::{CommonTokenFactory, TokenFactory};
use dbt_antlr4::rule_context::{BaseRuleContext,EmptyNodeKind,EmptyCustomRuleContext,EmptyRuleNode};
use dbt_antlr4::vocabulary::{Vocabulary,VocabularyImpl};

use std::ops::{DerefMut, Deref};
use std::sync::LazyLock;

dbt_antlr4::check_version!("2","0");
pub const COMMENT:i32=1; 
pub const CDATA:i32=2; 
pub const DTD:i32=3; 
pub const EntityRef:i32=4; 
pub const CharRef:i32=5; 
pub const SEA_WS:i32=6; 
pub const OPEN:i32=7; 
pub const XMLDeclOpen:i32=8; 
pub const TEXT:i32=9; 
pub const CLOSE:i32=10; 
pub const SPECIAL_CLOSE:i32=11; 
pub const SLASH_CLOSE:i32=12; 
pub const SLASH:i32=13; 
pub const EQUALS:i32=14; 
pub const STRING:i32=15; 
pub const Name:i32=16; 
pub const S:i32=17; 
pub const PI:i32=18;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;3] = [
    "DEFAULT_MODE", "INSIDE", "PROC_INSTR"
];

pub const ruleNames: [&'static str;24] = [
    "COMMENT", "CDATA", "DTD", "EntityRef", "CharRef", "SEA_WS", "OPEN", 
    "XMLDeclOpen", "SPECIAL_OPEN", "TEXT", "CLOSE", "SPECIAL_CLOSE", "SLASH_CLOSE", 
    "SLASH", "EQUALS", "STRING", "Name", "S", "HEXDIGIT", "DIGIT", "NameChar", 
    "NameStartChar", "PI", "IGNORE"
];
pub const _LITERAL_NAMES: [Option<&'static str>;15] = [
	None, None, None, None, None, None, None, Some("'<'"), None, None, Some("'>'"), 
	None, Some("'/>'"), Some("'/'"), Some("'='")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;19]  = [
	None, Some("COMMENT"), Some("CDATA"), Some("DTD"), Some("EntityRef"), Some("CharRef"), 
	Some("SEA_WS"), Some("OPEN"), Some("XMLDeclOpen"), Some("TEXT"), Some("CLOSE"), 
	Some("SPECIAL_CLOSE"), Some("SLASH_CLOSE"), Some("SLASH"), Some("EQUALS"), 
	Some("STRING"), Some("Name"), Some("S"), Some("PI")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, XMLLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct XMLLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr4::impl_token_source! { XMLLexer }
dbt_antlr4::impl_deref! { lexer => XMLLexer }

impl<'input, 'arena, Input, TF> XMLLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = XMLLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct XMLLexerActions {
}

impl XMLLexerActions {
	fn CLOSE_action<'input, 'arena, Input, TF>(action_index: i32, recog: &mut BaseLexerType<'input, 'arena, Input, TF>)
	where
	    TF: TokenFactory<'input, 'arena> + 'arena,
	    Input: CharStream<'input>,
	{
		match action_index {
	        0 => {
			recog.pop_mode();
	        },
			_ => {}
		}
	}
	fn COMMENT_sempred<'arena, 'input, Input, TF>(pred_index:i32, recog: &mut BaseLexerType<'input, 'arena, Input, TF>) -> bool
	where
	    TF: TokenFactory<'input, 'arena> + 'arena,
	    Input: CharStream<'input>,
	 {
		match pred_index {
	        0 => {
			true
		    }
		    _ => true
		}
	}
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseLexerType<'input, 'arena, Input, TF>, TF::Tok>
    for XMLLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn action(_localctx: Option<&EmptyRuleNode<'input, 'arena, TF::Tok>>, rule_index: i32, action_index: i32, recog:&mut BaseLexerType<'input, 'arena, Input, TF>) {
        match rule_index {
            10 => XMLLexerActions::CLOSE_action(action_index, recog), 
            _ => {}
        }
    }
    fn sempred(_localctx: Option<&EmptyRuleNode<'input, 'arena, TF::Tok>>, rule_index: i32, pred_index: i32, recog:&mut BaseLexerType<'input, 'arena, Input, TF>) -> bool {
        match rule_index {
            0 => XMLLexerActions::COMMENT_sempred(pred_index, recog), 
            _ => true
        }
    }
}
impl<'input, 'arena, Input, TF> LexerRecog<'input, 'arena, TF, BaseLexerType<'input, 'arena, Input, TF>>
    for XMLLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn get_rule_names(&self) -> &'static [&'static str] { &ruleNames }
    fn get_literal_names(&self) -> &[Option<&str>] { &_LITERAL_NAMES }
    fn get_symbolic_names(&self) -> &[Option<&str>] { &_SYMBOLIC_NAMES }
    fn get_grammar_file_name(&self) -> &'static str { "XMLLexer.g4" }
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 43] = [
    "CAAkzAMMAQwBDAEEAA4ABAIOAgQEDgQEBg4GBAgOCAQKDgoEDA4MBA4ODgQQDhAEEg4SBBQOFAQWDhYE",
    "GA4YBBoOGgQcDhwEHg4eBCAOIAQiDiIEJA4kBCYOJgQoDigEKg4qBCwOLAQuDi4CAAIAAgACAAIAAgAK",
    "AHQQABQAGAB6EgACAAIAAgACAAIAAgACAgICAgICAgICAgICAgICAgICAgICCgKgARACFAIYAqYBEgIC",
    "AgICAgICAgIEAgQCBAIECgS6ARAEFAQYBMABEgQCBAIEAgQCBAIGAgYCBgIGAggCCAIIAggICNwBEAgW",
    "CBgI3gECCAIIAggCCAIIAggCCAgI8gEQCBYIGAj0AQIIAggGCP4BEAgCCgIKBgqGAhAKAgoGCowCEAoC",
    "DAIMAgwCDAIOAg4CDgIOAg4CDgIOAg4CDgIOAhACEAIQAhACEAIQAhACEAISCBK+AhASFhIYEsACAhQC",
    "FAIUAhYCFgIWAhYCFgIYAhgCGAIYAhgCGgIaAhwCHAIeAh4KHuwCEB4UHhge8gISHgIeAh4CHgoe/AIQ",
    "HhQeGB6CAxIeAh4GHogDEB4CIAIgCiCQAxAgFCAYIJYDEiACIgIiAiICIgIkAiQCJgImAigCKAIoAigG",
    "KLIDECgCKgYquAMQKgIsAiwCLAIsAiwCLgIuAi4CLgZ2ogG8AQAwBgIKBA4GEggWChoMHg4iECYAKhIu",
    "FDIWNhg6Gj4cQh5GIEoiTgBSAFYAWgBeJGIABgACBBIEABISQEAEAExMeHgEAEREeHgEAE5OeHgGABIU",
    "GhpAQAYAYHKCAYwBwgHMAQIAYHIGAO4C7gKADN4N/oABgIEBEAB0dIIBtAHCAfQB4IEBnoYBgLAB3r8B",
    "gsAB/t8GgOQHnvcH4PcH+v8H3gMABgIAAAAACgIAAAAADgIAAAAAEgIAAAAAFgIAAAAAGgIAAAAAHgIA",
    "AAAAIgIAAAAAJgIAAAAAKgIAAAACLgIAAAACMgIAAAACNgIAAAACOgIAAAACPgIAAAACQgIAAAACRgIA",
    "AAACSgIAAAAEXgIAAAAEYgIAAAAGZgIAAAAKiAECAAAADrABAgAAABLKAQIAAAAW/AECAAAAGooCAgAA",
    "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",
    "tgMMDAAACgIABgAACgQAAhQACAAA"
];