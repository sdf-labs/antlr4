//! Differential corners for alt-mask prediction ("implied
//! left-factoring") on the paren family (grammar `grammars/ParenExpr.g4`,
//! generated twice: `gen/parenexprparser.rs` with `-Xstatic-dfa`,
//! `gen/adaptive/parenexprparser.rs` without).
//!
//! The primary block's rowConstructor (`'(' e (',' e)+ ')'`) and parens
//! (`'(' e ')'`) alternatives share the unbounded prefix `'(' e`; the
//! subquery alternative (`'(' q ')'`) keeps pace on query-shaped content
//! (identifiers included: a parenthesized table name is a valid query).
//! The static table accepts with the alt mask {rowConstructor, parens}
//! once the subquery reading dies, and the generated factored arm
//! executes `'(' e` once and resolves the choice with a k=1 tail switch;
//! deep nesting keeps the subquery reading alive and escapes to
//! adaptivePredict. Both engines must agree on every corner (tree or
//! error).

#[rustfmt::skip]
mod gen {
    #![allow(non_snake_case)]
    #![allow(clippy::all)]
    pub mod parenexprlexer;
    pub mod parenexprlistener;
    pub mod parenexprparser;
    pub mod adaptive {
        pub mod parenexprlexer;
        pub mod parenexprlistener;
        pub mod parenexprparser;
    }
}

use dbt_antlr4::common_token_stream::CommonTokenStream;
use dbt_antlr4::recognizer::Recognizer;
use dbt_antlr4::token_factory::CommonTokenFactory;
use dbt_antlr4::trees::string_tree;
use dbt_antlr4::{Arena, InputStream};

fn parse_static(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::parenexprlexer::ParenExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::parenexprparser::ParenExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

fn parse_adaptive(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::adaptive::parenexprlexer::ParenExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::adaptive::parenexprparser::ParenExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

/// Corners of the paren family. Both engines must agree on every one.
const CORNERS: &[&str] = &[
    // plain parens (alt parens)
    "(1)",
    "(x)",
    "(-1)",
    "(1 + 2)",
    "((1))",
    // row constructors
    "(1, 2)",
    "(x, y, z)",
    "(1, 2, 3, 4)",
    "((1, 2), 3)",
    // subqueries
    "(SELECT t)",
    "(t)",
    "((SELECT t))",
    // the tail decision inside operators
    "1 + (2 * 3)",
    "(1 + 2) * (3 + 4)",
    "-(1, 2)",
    "1 * (x, (y, z))",
    // nested-paren escapes (subquery reading stays alive arbitrarily
    // long; these defer to adaptive and must still agree)
    "((((((x))))))",
    "((((1, 2))))",
    // error corners
    "(1,)",
    "(1 2)",
    "(1, 2",
    "(SELECT)",
    "(1,)",
];

fn check(input: &str) {
    let s = parse_static(input);
    let a = parse_adaptive(input);
    assert_eq!(s, a, "static vs adaptive mismatch on {input:?}");
}

#[test]
fn paren_family_corners_agree() {
    for corner in CORNERS {
        check(corner);
    }
}
