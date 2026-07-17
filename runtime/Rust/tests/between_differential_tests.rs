//! Differential corners for the BETWEEN ... AND ... collision with the
//! boolean AND/OR loop (grammar `grammars/BetweenExpr.g4`, generated twice:
//! `gen/betweenexprparser.rs` with `-Xstatic-dfa`,
//! `gen/adaptive/betweenexprparser.rs` without).
//!
//! The grammar mirrors the dbt SQL shape: BETWEEN is a suffix predicate on
//! the pure-arithmetic value ladder, so its `AND` separator is not the
//! boolean loop's `AND` - the two collide only through lookahead. The
//! lambda production (`ID '->' b`) recurses from a value position back
//! into the boolean rule, which is what makes the collision genuinely
//! context-sensitive: `x BETWEEN y -> a AND b AND c` parses the first AND
//! into the lambda body and takes the second as the separator, and which
//! AND is which depends on the whole rest of the input. On these shapes
//! the hybrid tables escape to adaptivePredict; both engines must agree
//! on every corner. (Without the lambda production every decision in this
//! grammar is statically k=1: the escape is triggered specifically by
//! value-position recursion into the boolean rule - not by function
//! calls, IN lists, subscripts, or LIKE..ESCAPE suffixes.)

#[rustfmt::skip]
mod gen {
    #![allow(non_snake_case)]
    #![allow(clippy::all)]
    pub mod betweenexprlexer;
    pub mod betweenexprlistener;
    pub mod betweenexprparser;
    pub mod adaptive {
        pub mod betweenexprlexer;
        pub mod betweenexprlistener;
        pub mod betweenexprparser;
    }
}

use dbt_antlr4::common_token_stream::CommonTokenStream;
use dbt_antlr4::recognizer::Recognizer;
use dbt_antlr4::token_factory::CommonTokenFactory;
use dbt_antlr4::trees::string_tree;
use dbt_antlr4::{Arena, InputStream};

fn parse_static(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::betweenexprlexer::BetweenExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::betweenexprparser::BetweenExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

fn parse_adaptive(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::adaptive::betweenexprlexer::BetweenExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::adaptive::betweenexprparser::BetweenExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

/// Corners of the BETWEEN/boolean-AND collision. Both engines must agree on
/// every one (tree or error).
const CORNERS: &[&str] = &[
    // plain BETWEEN
    "a BETWEEN 1 AND 2",
    "a BETWEEN 1 + 1 AND 2 * 3",
    // BETWEEN then boolean AND: the upper bound must end at the first AND
    "a BETWEEN 1 AND 2 AND b",
    "a BETWEEN 1 AND 2 AND b AND c",
    // boolean AND whose right operand is a BETWEEN
    "a AND b BETWEEN 1 AND 2",
    "a = 1 AND b BETWEEN 2 AND 3",
    // BETWEEN on both sides of boolean operators
    "a BETWEEN 1 AND 2 AND b BETWEEN 3 AND 4",
    "a BETWEEN 1 AND 2 OR b BETWEEN 3 AND 4 AND c",
    // long chains: the separator/operator choice repeats
    "x BETWEEN a AND b AND c AND d AND e",
    "a AND b BETWEEN 1 AND 2 AND c AND d BETWEEN 3 AND 4 AND e",
    "a BETWEEN 1 + 2 AND 3 - 4 AND b BETWEEN 5 * 6 AND 7 / 8",
    // NOT: prefix boolean NOT vs NOT BETWEEN
    "NOT a BETWEEN 1 AND 2",
    "a NOT BETWEEN 1 AND 2",
    "a NOT BETWEEN 1 AND 2 AND b NOT BETWEEN 3 AND 4",
    "NOT a AND b NOT BETWEEN 1 AND 2",
    // comparison predicate + BETWEEN
    "a = b AND c BETWEEN 1 AND 2",
    "a < b OR c > d BETWEEN 1 AND 2",
    // parentheses: boolean frames inside value bounds (phantom-FOLLOW
    // mixing ground) and parenthesized predicates
    "(a BETWEEN 1 AND 2) AND b",
    "a BETWEEN (x AND y) AND z",
    "a BETWEEN (x) AND (y)",
    "a BETWEEN 1 AND (2) AND b",
    "(a AND b) BETWEEN 1 AND 2",
    "a BETWEEN (x AND y) AND (p OR q) AND r",
    // lambdas: boolean bodies in value positions - the escape trigger.
    // Which AND is the BETWEEN separator depends on the entire tail of
    // the input (genuinely context-sensitive).
    "x BETWEEN y -> a AND b AND c",
    "x BETWEEN y -> a AND y -> b AND c",
    "x BETWEEN a AND y -> b AND c",
    "f -> a BETWEEN 1 AND 2",
    "f -> a AND b",
    "f -> a AND b BETWEEN 1 AND 2",
    "x BETWEEN f -> a AND b",
    "x BETWEEN f -> a AND b AND c",
    "(x BETWEEN f -> a AND b) AND c",
    "f -> g -> a AND b BETWEEN 1 AND 2 AND c",
    // errors: both engines must fail identically
    "a BETWEEN 1 AND",
    "a BETWEEN",
    "a AND",
    "NOT BETWEEN 1 AND 2",
    // a second BETWEEN after a completed predicate: not a value operator
    "a BETWEEN 1 AND 2 BETWEEN 3 AND 4",
    "a < b BETWEEN 1 AND 2",
    // garbage
    "a BETWEEN AND 2",
    "1 AND",
];

#[test]
fn test_corners_differential() {
    for input in CORNERS {
        let s = parse_static(input);
        let a = parse_adaptive(input);
        assert_eq!(
            a.is_ok(),
            s.is_ok(),
            "parse outcome diverges on {input:?}\nadaptive: {a:?}\nstatic:   {s:?}"
        );
        assert_eq!(
            a, s,
            "parse tree diverges on {input:?}\nadaptive: {a:?}\nstatic:   {s:?}"
        );
    }
}

/// The meaning of the collision: the boolean `AND` after a BETWEEN always
/// binds the whole predicate, never the upper bound (the upper bound is a
/// value, and values cannot absorb `AND`). These expected trees lock the
/// semantics so a future table change cannot silently re-associate.
#[test]
fn test_semantics() {
    // AND(BETWEEN(a, 1, 2), b)
    assert_eq!(
        parse_static("a BETWEEN 1 AND 2 AND b").unwrap(),
        "(s (b (b (v (atom a)) (pred BETWEEN (v (atom 1)) AND (v (atom 2)))) AND (b (v (atom b)))) <EOF>)"
    );
    // AND(a, BETWEEN(b, 1, 2))
    assert_eq!(
        parse_static("a AND b BETWEEN 1 AND 2").unwrap(),
        "(s (b (b (v (atom a))) AND (b (v (atom b)) (pred BETWEEN (v (atom 1)) AND (v (atom 2))))) <EOF>)"
    );
    // NOT BETWEEN is a predicate suffix, not a prefix NOT
    assert_eq!(
        parse_static("a NOT BETWEEN 1 AND 2").unwrap(),
        "(s (b (v (atom a)) (pred NOT BETWEEN (v (atom 1)) AND (v (atom 2)))) <EOF>)"
    );
    // OR(a, AND(BETWEEN(b,1,2), c)): AND binds tighter than OR
    assert_eq!(
        parse_static("a OR b BETWEEN 1 AND 2 AND c").unwrap(),
        "(s (b (b (v (atom a))) OR (b (b (v (atom b)) (pred BETWEEN (v (atom 1)) AND (v (atom 2)))) AND (b (v (atom c))))) <EOF>)"
    );
    // parenthesized boolean inside a BETWEEN bound
    assert_eq!(
        parse_static("a BETWEEN (x AND y) AND z").unwrap(),
        "(s (b (v (atom a)) (pred BETWEEN (v (atom ( (b (b (v (atom x))) AND (b (v (atom y)))) ))) AND (v (atom z)))) <EOF>)"
    );
    // lambda in a BETWEEN bound: maximal munch - the body absorbs the
    // first AND, the second is the separator (both engines agree)
    assert_eq!(
        parse_static("x BETWEEN y -> a AND b AND c").unwrap(),
        "(s (b (v (atom x)) (pred BETWEEN (v (atom y -> (b (b (v (atom a))) AND (b (v (atom b)))))) AND (v (atom c)))) <EOF>)"
    );
    // ...but when absorbing would strand the separator, the body stays
    // minimal and the single AND separates
    assert_eq!(
        parse_static("x BETWEEN f -> a AND b").unwrap(),
        "(s (b (v (atom x)) (pred BETWEEN (v (atom f -> (b (v (atom a))))) AND (v (atom b)))) <EOF>)"
    );
    // a lambda body can itself contain a BETWEEN
    assert_eq!(
        parse_static("f -> a BETWEEN 1 AND 2").unwrap(),
        "(s (b (v (atom f -> (b (v (atom a)) (pred BETWEEN (v (atom 1)) AND (v (atom 2))))))) <EOF>)"
    );
}
