// Generated from VisitorCalc.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unused_variables)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr4::prelude::*;
use dbt_antlr4::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr4::check_version!("2","0");
pub const INT:i32=1; 
pub const MUL:i32=2; 
pub const DIV:i32=3; 
pub const ADD:i32=4; 
pub const SUB:i32=5; 
pub const WS:i32=6;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;6] = [
    "INT", "MUL", "DIV", "ADD", "SUB", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;6] = [
	None, None, Some("'*'"), Some("'/'"), Some("'+'"), Some("'-'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;7]  = [
	None, Some("INT"), Some("MUL"), Some("DIV"), Some("ADD"), Some("SUB"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, VisitorCalcLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct VisitorCalcLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr4::impl_token_source! { VisitorCalcLexer }
dbt_antlr4::impl_deref! { lexer => VisitorCalcLexer }

impl<'input, 'arena, Input, TF> VisitorCalcLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = VisitorCalcLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct VisitorCalcLexerActions {
}

impl VisitorCalcLexerActions {
}

dbt_antlr4::impl_lexer_recog! { VisitorCalcLexerActions, "VisitorCalcLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize_compact(&_serializedATN));
static _serializedATN: [&'static str; 6] = [
    "CAAMQgwBBAAOAAQCDgIEBA4EBAYOBgQIDggECg4KAgAIAB4QABYAGAAgAgICAgIEAgQCBgIGAggCCAIK",
    "CAo4EAoWChgKOgIKAgoAAAwCAgYECgYOCBIKFgwCAAQCAGByBAASEkBARAACAgAAAAAGAgAAAAAKAgAA",
    "AAAOAgAAAAASAgAAAAAWAgAAAAIcAgAAAAYkAgAAAAooAgAAAA4sAgAAABIwAgAAABY2AgAAABoeDgAA",
    "ABwaAgAAAB4gAgAAACAcAgAAACAiAgAAACIEAgAAACQmClQAACYIAgAAACgqCl4AACoMAgAAACwuClYA",
    "AC4QAgAAADAyCloAADIUAgAAADQ4DgIAADY0AgAAADg6AgAAADo2AgAAADo8AgAAADw+AgAAAD5ADAoA",
    "AEAYAgAAAAYAIDoCAAIA"
];
