//! Differential corners for the guarded-take escape of the
//! optional-postfix take rule (grammar `grammars/GuardExpr.g4`, generated
//! twice: `gen/guardexprparser.rs` with `-Xstatic-dfa`,
//! `gen/adaptive/guardexprparser.rs` without).
//!
//! The grammar mirrors the dbt SQL shape (Bigquery's
//! `UPDATE SET targets+=expression EQ values+=expression`): the
//! assignment's first `expr` is followed by `'='`, which is also the
//! comparison operator of the optional postfix predicate, so the
//! predicate decision's take/skip conflict is genuinely
//! context-sensitive - inside an assignment target skip must win
//! (`set a = b ;` assigns b to a), everywhere else take wins
//! (`a = b ;` is a comparison). The static table resolves the conflict
//! to take guarded by a stack test that defers to adaptivePredict when
//! the epsilon-pop chase from the decision's block end can reach the
//! assignment's `'='` position. Both engines must agree on every corner
//! (tree or error).

#[rustfmt::skip]
mod gen {
    #![allow(non_snake_case)]
    #![allow(clippy::all)]
    pub mod guardexprlexer;
    pub mod guardexprlistener;
    pub mod guardexprparser;
    pub mod adaptive {
        pub mod guardexprlexer;
        pub mod guardexprlistener;
        pub mod guardexprparser;
    }
}

use dbt_antlr4::common_token_stream::CommonTokenStream;
use dbt_antlr4::recognizer::Recognizer;
use dbt_antlr4::token_factory::CommonTokenFactory;
use dbt_antlr4::trees::string_tree;
use dbt_antlr4::{Arena, InputStream};

fn parse_static(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::guardexprlexer::GuardExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::guardexprparser::GuardExprParser::new(arena, tokens);
        parser
            .prog()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

fn parse_adaptive(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::adaptive::guardexprlexer::GuardExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::adaptive::guardexprparser::GuardExprParser::new(arena, tokens);
        parser
            .prog()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

/// Corners of the assignment/comparison dangling-else. Both engines must
/// agree on every one (tree or error).
const CORNERS: &[&str] = &[
    // take wins outside assignments: comparisons
    "a = b ;",
    "a < b ;",
    "a > 1 ;",
    "a = b = c ;",
    "a IN ( b ) ;",
    "a = b AND c = d ;",
    // skip wins inside assignment targets
    "set a = b ;",
    "set a = 1 ;",
    "set ab = cd ;",
    // the assignment value is itself a comparison: the value-side
    // decision is sealed off from the assignment's '=' and must take
    "set a = b = c ;",
    "set a = b < c ;",
    "set a = b AND c ;",
    "set a = b AND c = d ;",
    // multiple statements: guard state per statement
    "set a = b ; c = d ;",
    "a = b ; set c = d ; e = f ;",
    "set a = b ; set c = d ;",
    // parentheses seal the expression: comparisons inside, even in an
    // assignment value (the chase must stop at the paren frame)
    "set x = ( a = b ) ;",
    "set x = ( a ) = b ;",
    "( a = b ) ;",
    "( a ) = b ;",
    "set ( a ) = b ;",
    // IN: a second predicate-start token
    "a IN ( b ) ;",
    "set a = b IN ( c ) ;",
    "a IN ( b = c ) ;",
    // boolean operators: right operands, chase through the loop frames
    "a AND b = c ;",
    "set a = b AND c ;",
    "a AND set ;",
    "NOT a = b ;",
    "set a = NOT b = c ;",
    "a OR b AND c = d OR e ;",
    "set x = a OR b = c AND d ;",
    // nested assignments are not a thing: the second 'set' starts a new
    // statement only after ';'
    "set a = b ; set c = d = e ;",
    // errors: both engines must fail identically
    "set a = ;",
    "set = b ;",
    "a = ;",
    "set a b ;",
    "a IN ( ;",
    "set a = b",
    "( a = b ;",
    "a AND ;",
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
