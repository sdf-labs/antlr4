//! Tests for table-driven static SLL prediction (`-Xstatic-dfa`).
//!
//! The parser in `gen/staticdfaparser.rs` is generated with `-Xstatic-dfa`;
//! every non-LL(1) decision in `grammars/StaticDFA.g4` is statically
//! decidable, so the generated code drives them all through
//! `BaseParser::dfa_predict` tables (LL(2) alt blocks, a cyclic LL(*)
//! dotted-name decision, non-LL(1) star/plus/optional decisions, and an
//! exact ambiguity resolved min-alt) with no `adaptive_predict` call sites.

#[rustfmt::skip]
mod gen {
    #![allow(non_snake_case)]
    #![allow(clippy::all)]
    pub mod staticdfalexer;
    pub mod staticdfalistener;
    pub mod staticdfaparser;
}

use dbt_antlr4::common_token_stream::CommonTokenStream;
use dbt_antlr4::recognizer::Recognizer;
use dbt_antlr4::token_factory::CommonTokenFactory;
use dbt_antlr4::trees::string_tree;
use dbt_antlr4::{Arena, InputStream};

use crate::gen::staticdfalexer::StaticDFALexer;
use crate::gen::staticdfaparser::StaticDFAParser;

fn parse(input: &'static str, rule: &str) -> Result<String, dbt_antlr4::errors::ANTLRError> {
    Arena::with(|arena| {
        let lexer = StaticDFALexer::<_, CommonTokenFactory>::new(arena, InputStream::new(input));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = StaticDFAParser::new(arena, tokens);
        match rule {
            "stat" => parser
                .stat()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "expr" => parser
                .expr()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "starLoop" => parser
                .starLoop()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "plusLoop" => parser
                .plusLoop()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "opt" => parser
                .opt()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "dup" => parser
                .dup()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            _ => unreachable!(),
        }
    })
}

/// LL(2)/LL(3) keyword-pair alt block.
#[test]
fn test_llk_alt_block() {
    assert_eq!(
        parse("create table foo", "stat").unwrap(),
        "(stat create table foo)"
    );
    assert_eq!(
        parse("create view foo", "stat").unwrap(),
        "(stat create view foo)"
    );
    assert_eq!(
        parse("drop table foo", "stat").unwrap(),
        "(stat drop table foo)"
    );
}

/// LL(*): the cyclic DFA scans an arbitrarily long dotted name; the token
/// after it ('(' vs EOF) picks the alternative.
#[test]
fn test_llstar_dotted_name() {
    assert_eq!(
        parse("aa.bb.cc.dd()", "expr").unwrap(),
        "(expr (name aa . bb . cc . dd) ( ))"
    );
    assert_eq!(
        parse("aa.bb.cc.dd", "expr").unwrap(),
        "(expr (name aa . bb . cc . dd))"
    );
    // single component, both ways
    assert_eq!(parse("aa()", "expr").unwrap(), "(expr (name aa) ( ))");
    assert_eq!(parse("aa", "expr").unwrap(), "(expr (name aa))");
}

/// Non-LL(1) star loop: enter/exit needs LA(2) ('a b' repeats vs final 'a c').
#[test]
fn test_dfa_star_loop() {
    assert_eq!(parse("a c", "starLoop").unwrap(), "(starLoop a c)");
    assert_eq!(parse("a b a c", "starLoop").unwrap(), "(starLoop a b a c)");
    assert_eq!(
        parse("a b a b a c", "starLoop").unwrap(),
        "(starLoop a b a b a c)"
    );
}

/// Non-LL(1) plus loop: continue/exit needs LA(2).
#[test]
fn test_dfa_plus_loop() {
    assert_eq!(parse("a b a c", "plusLoop").unwrap(), "(plusLoop a b a c)");
    assert_eq!(
        parse("a b a b a c", "plusLoop").unwrap(),
        "(plusLoop a b a b a c)"
    );
}

/// Non-LL(1) optional: enter/bypass needs LA(2).
#[test]
fn test_dfa_optional() {
    assert_eq!(parse("a c", "opt").unwrap(), "(opt a c)");
    assert_eq!(parse("a b a c", "opt").unwrap(), "(opt a b a c)");
}

/// Exact ambiguity: static min-alt resolution must pick alternative 1
/// (#dupFirst), exactly like runtime SLL/LL prediction.
#[test]
fn test_exact_ambiguity_min_alt() {
    use crate::gen::staticdfaparser::DupContextAll;
    Arena::with(|arena| {
        let lexer = StaticDFALexer::<_, CommonTokenFactory>::new(arena, InputStream::new("d e"));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = StaticDFAParser::new(arena, tokens);
        let result = parser.dup().unwrap();
        assert!(
            matches!(result, DupContextAll::DupFirstContext(_)),
            "min-alt resolution must pick #dupFirst"
        );
    });
}

/// Prediction failure surfaces as an error (recovered or not), never a panic
/// or infinite loop - including mid-scan failures of the cyclic LL(*) DFA.
#[test]
fn test_no_viable_alt_handling() {
    // 'create foo' - fails inside the LL(2) DFA at depth 2
    let r = parse("create foo", "stat");
    match r {
        Ok(tree) => assert!(tree.contains("create")), // recovered parse
        Err(_) => {}                                  // unrecoverable is fine too
    }
    // dotted name that ends at a dot: cyclic DFA hits EOF mid-scan
    let r = parse("aa.bb.", "expr");
    match r {
        Ok(tree) => assert!(tree.contains("aa")),
        Err(_) => {}
    }
}

/// The generated parser must contain zero bare adaptive_predict call
/// sites: every prediction goes through dfa_predict, and the only
/// adaptive_predict occurrences are the hybrid-escape fallbacks inlined at
/// each table-driven site (dead code for this grammar's escape-free
/// tables).
#[test]
fn test_no_adaptive_predict_in_generated_code() {
    let src = include_str!("gen/staticdfaparser.rs");
    assert_eq!(
        src.matches("adaptive_predict(").count(),
        src.matches("if _sdp == INVALID_ALT").count(),
        "StaticDFA parser should be fully table-driven"
    );
    assert!(
        src.contains("dfa_predict"),
        "expected static DFA call sites"
    );
}
