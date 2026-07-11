//! Tests for per-precedence static prediction tables (`-Xstatic-dfa` on a
//! left-recursive grammar).
//!
//! The parser in `gen/lrdfaparser.rs` is generated with `-Xstatic-dfa` from
//! `grammars/LrDfa.g4`, a left-recursive expression grammar exercising the
//! shapes that make precedence prediction hard: multiple binary precedence
//! levels, right-associative operators (`^` and the ternary), a prefix
//! operator, a postfix operator, and array indexing (`e '[' e ']'`, whose
//! nested 0-precedence re-entry exercises the precedence-filter suppression
//! path). Every precedence class of the operator-loop decision is
//! statically decidable, so the generated loop runs entirely on
//! `dfa_predict` with a per-precedence table dispatch - the static analogue
//! of the adaptive runtime's per-precedence DFA start states, making the
//! loop behave like a hand-rolled precedence-climbing (Pratt) parser.

#[rustfmt::skip]
mod gen {
    #![allow(non_snake_case)]
    #![allow(clippy::all)]
    pub mod lrdfalexer;
    pub mod lrdfalistener;
    pub mod lrdfaparser;
}

use dbt_antlr4::common_token_stream::CommonTokenStream;
use dbt_antlr4::recognizer::Recognizer;
use dbt_antlr4::token_factory::CommonTokenFactory;
use dbt_antlr4::trees::string_tree;
use dbt_antlr4::{Arena, InputStream};

use crate::gen::lrdfalexer::LrDfaLexer;
use crate::gen::lrdfaparser::LrDfaParser;

fn parse(input: &'static str) -> Result<String, dbt_antlr4::errors::ANTLRError> {
    Arena::with(|arena| {
        let lexer = LrDfaLexer::<_, CommonTokenFactory>::new(arena, InputStream::new(input));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = LrDfaParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
    })
}

/// Left-associative operators at distinct precedence levels.
#[test]
fn test_precedence_and_left_associativity() {
    assert_eq!(
        parse("1+2*3").unwrap(),
        "(s (e (e 1) + (e (e 2) * (e 3))) <EOF>)"
    );
    assert_eq!(
        parse("1*2+3").unwrap(),
        "(s (e (e (e 1) * (e 2)) + (e 3)) <EOF>)"
    );
    assert_eq!(
        parse("1+2+3").unwrap(),
        "(s (e (e (e 1) + (e 2)) + (e 3)) <EOF>)"
    );
    assert_eq!(
        parse("1-2-3").unwrap(),
        "(s (e (e (e 1) - (e 2)) - (e 3)) <EOF>)"
    );
}

/// `^` binds right: `2^3^4` = `2^(3^4)`.
#[test]
fn test_right_associativity() {
    assert_eq!(
        parse("2^3^4").unwrap(),
        "(s (e (e 2) ^ (e (e 3) ^ (e 4))) <EOF>)"
    );
    assert_eq!(
        parse("2^3*4").unwrap(),
        "(s (e (e (e 2) ^ (e 3)) * (e 4)) <EOF>)"
    );
}

/// Prefix minus binds tighter than binary operators but looser than `^`.
#[test]
fn test_prefix_operator() {
    assert_eq!(parse("-1+2").unwrap(), "(s (e (e - (e 1)) + (e 2)) <EOF>)");
    assert_eq!(parse("2^-3").unwrap(), "(s (e (e 2) ^ (e - (e 3))) <EOF>)");
}

/// Postfix factorial binds tightest of the operators.
#[test]
fn test_postfix_operator() {
    assert_eq!(parse("1+2!").unwrap(), "(s (e (e 1) + (e (e 2) !)) <EOF>)");
    assert_eq!(parse("-a!").unwrap(), "(s (e - (e (e a) !)) <EOF>)");
}

/// Array indexing: the nested `e` re-enters the rule with precedence 0
/// through a call site the precedence filter must not eliminate.
#[test]
fn test_index_suppression_shape() {
    assert_eq!(parse("a[1]").unwrap(), "(s (e (e a) [ (e 1) ]) <EOF>)");
    assert_eq!(
        parse("a[1][2]").unwrap(),
        "(s (e (e (e a) [ (e 1) ]) [ (e 2) ]) <EOF>)"
    );
    assert_eq!(
        parse("a[b?c:d]").unwrap(),
        "(s (e (e a) [ (e (e b) ? (e c) : (e d)) ]) <EOF>)"
    );
    assert_eq!(
        parse("a[1+2]*3").unwrap(),
        "(s (e (e (e a) [ (e (e 1) + (e 2)) ]) * (e 3)) <EOF>)"
    );
}

/// The ternary is right-associative and lowest-precedence.
#[test]
fn test_ternary() {
    assert_eq!(
        parse("a?b:c?d:e").unwrap(),
        "(s (e (e a) ? (e b) : (e (e c) ? (e d) : (e e))) <EOF>)"
    );
    assert_eq!(
        parse("1+a?b:c+2").unwrap(),
        "(s (e (e (e 1) + (e a)) ? (e b) : (e (e c) + (e 2))) <EOF>)"
    );
}

/// Everything at once. Prefix minus binds tighter than `^` here because it
/// is a higher alternative in the grammar, so `-2^x` is `(-2)^x`.
#[test]
fn test_mixed() {
    assert_eq!(
        parse("1*-2^a[3]!+b?c[1]:-d").unwrap(),
        "(s (e (e (e (e 1) * (e (e - (e 2)) ^ (e (e (e a) [ (e 3) ]) !))) + (e b)) \
         ? (e (e c) [ (e 1) ]) : (e - (e d))) <EOF>)"
    );
}

/// Malformed input is recovered by the default error strategy exactly like
/// under adaptive prediction (verified differentially against an
/// adaptive-prediction build of the same grammar): the dispatch's k=0 exit
/// class and the fallback machinery must not mask genuine syntax errors.
#[test]
fn test_errors_still_reported() {
    assert_eq!(parse("1+").unwrap(), "(s (e (e 1) + e) <EOF>)");
    assert_eq!(
        parse("a[").unwrap(),
        "(s (e (e a) [ e <missing ']'>) <EOF>)"
    );
    assert_eq!(parse("?a").unwrap(), "(s (e ? a) <EOF>)");
    assert_eq!(parse("").unwrap(), "(s e <EOF>)");
    assert_eq!(parse("1]2").unwrap(), "(s (e 1) ] 2)");
}

/// The operator loop of the left-recursive rule must be table-driven: no
/// adaptive_predict call sites anywhere in the generated parser.
#[test]
fn test_no_adaptive_predict_in_generated_code() {
    let src = include_str!("gen/lrdfaparser.rs");
    assert!(
        !src.contains("adaptive_predict"),
        "grammars/LrDfa.g4 must generate zero adaptive_predict call sites"
    );
    assert!(
        src.contains("precedence-dispatched"),
        "the operator loop decision must be served by a precedence dispatch"
    );
}
