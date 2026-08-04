//! Glob-import target for generated code.
//!
//! `use dbt_antlr4::prelude::*;` brings in every name referenced by the
//! boilerplate ANTLR emits (including the expansions of this crate's
//! `impl_*` macros), replacing the long per-file import lists the code
//! generator used to emit. Names a particular grammar does not use are
//! simply ignored, so generated files no longer need
//! `#![allow(unused_imports)]` to compensate for over-importing.

pub use std::marker::PhantomData;
pub use std::ops::{Deref, DerefMut};
pub use std::rc::Rc;
pub use std::sync::LazyLock;

pub use crate::arena::Arena;
pub use crate::atn::{ATN, INVALID_ALT};
pub use crate::atn_config_set::{ATNConfigSet, LexerATNConfigSet};
pub use crate::atn_deserializer::ATNDeserializer;
pub use crate::atn_simulator::{
    BaseATNSimulator, LexerATNSimulatorManager, ParserATNSimulatorManager,
};
pub use crate::char_stream::CharStream;
pub use crate::error_strategy::{DefaultErrorStrategy, ErrorStrategy, ErrorStrategyDelegate};
pub use crate::errors::ANTLRError;
pub use crate::int_stream::{IntStream, EOF};
pub use crate::lexer::{BaseLexer, Lexer, LexerRecog};
pub use crate::lexer_atn_simulator::{ILexerATNSimulator, LexerATNSimulator};
pub use crate::parser::{BaseParser, ListenerId, Parser, ParserRecog};
pub use crate::parser_atn_simulator::ParserATNSimulator;
pub use crate::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
pub use crate::recognizer::{Actions, Recognizer};
pub use crate::static_lexer_dfa::StaticLexerTables;
pub use crate::rule_context::{
    BaseRuleContext, CustomRuleContext, EmptyCustomRuleContext, EmptyNodeKind, EmptyRuleNode,
    RuleContext,
};
pub use crate::token::{CommonToken, OwningToken, Token, TOKEN_EOF};
pub use crate::token_factory::{CommonTokenFactory, TokenFactory};
pub use crate::token_stream::TokenStream;
pub use crate::tree::*;
pub use crate::vocabulary::{Vocabulary, VocabularyImpl};
pub use crate::{PredictionContextCache, TokenSource};
