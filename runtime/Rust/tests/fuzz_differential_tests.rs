//! Differential fuzz of table-driven vs adaptive prediction.
//!
//! `gen/fuzzexprparser.rs` is generated from `grammars/FuzzExpr.g4` with
//! `-Xstatic-dfa`; `gen/adaptive/fuzzexprparser.rs` is the same grammar
//! without it. The grammar concentrates everything the static tables must
//! get right: a precedence ladder driven by per-precedence dispatch, a
//! scan-past-expression decision served by a hybrid table with adaptive
//! escapes, and a soft keyword (`as` doubles as an identifier) whose
//! genuinely context-sensitive corners exercise the escape path at
//! runtime.
//!
//! Every input - grammar-directed random expressions and unconstrained
//! token soup, from a fixed-seed generator so failures reproduce - must
//! produce an identical parse tree (or identically fail) under both
//! parsers. `dfa_predict` never consumes input before escaping, so any
//! divergence is a real prediction bug, not a harness artifact.

#[rustfmt::skip]
mod gen {
    #![allow(non_snake_case)]
    #![allow(clippy::all)]
    pub mod fuzzexprlexer;
    pub mod fuzzexprlistener;
    pub mod fuzzexprparser;
    pub mod adaptive {
        pub mod fuzzexprlexer;
        pub mod fuzzexprlistener;
        pub mod fuzzexprparser;
    }
}

use dbt_antlr4::common_token_stream::CommonTokenStream;
use dbt_antlr4::recognizer::Recognizer;
use dbt_antlr4::token_factory::CommonTokenFactory;
use dbt_antlr4::trees::string_tree;
use dbt_antlr4::{Arena, InputStream};

fn parse_static(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::fuzzexprlexer::FuzzExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::fuzzexprparser::FuzzExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

fn parse_adaptive(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::adaptive::fuzzexprlexer::FuzzExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::adaptive::fuzzexprparser::FuzzExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

fn assert_equivalent(input: &str) {
    let s = parse_static(input);
    let a = parse_adaptive(input);
    match (&s, &a) {
        (Ok(st), Ok(at)) => assert_eq!(st, at, "tree divergence on input: {input:?}"),
        (Err(_), Err(_)) => {} // both unrecoverable; exact error compared in Java probes
        _ => panic!("outcome divergence on input {input:?}:\n  static:   {s:?}\n  adaptive: {a:?}"),
    }
}

/// Deterministic xorshift64* PRNG: fixed seeds, reproducible failures.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545F4914F6CDD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Grammar-directed random expression, depth-bounded.
fn gen_expr(rng: &mut Rng, depth: usize, out: &mut String) {
    if depth == 0 || rng.below(4) == 0 {
        // atom
        match rng.below(6) {
            0 => out.push_str("x"),
            1 => out.push_str("1"),
            2 => out.push_str("as"), // soft keyword as identifier
            3 => out.push_str("a.b"),
            4 => {
                out.push_str("f(");
                gen_expr(rng, depth.saturating_sub(1), out);
                out.push(')');
            }
            _ => {
                out.push('(');
                gen_expr(rng, depth.saturating_sub(1), out);
                out.push(')');
            }
        }
        return;
    }
    match rng.below(7) {
        0 => {
            out.push_str("- ");
            gen_expr(rng, depth - 1, out);
        }
        1 => {
            gen_expr(rng, depth - 1, out);
            out.push('[');
            gen_expr(rng, depth - 1, out);
            out.push(']');
        }
        op => {
            gen_expr(rng, depth - 1, out);
            out.push_str([" ^ ", " * ", " / ", " + ", " - "][op - 2]);
            gen_expr(rng, depth - 1, out);
        }
    }
}

/// Random items list: `e (as name)? (',' ...)* ';'?`.
fn gen_input(rng: &mut Rng) -> String {
    let mut out = String::new();
    let items = 1 + rng.below(3);
    for i in 0..items {
        if i > 0 {
            out.push_str(", ");
        }
        let depth = 1 + rng.below(4);
        gen_expr(rng, depth, &mut out);
        match rng.below(3) {
            0 => out.push_str(" as y"),
            1 => out.push_str(" as as"), // alias spelled with the soft keyword
            _ => {}
        }
    }
    if rng.below(2) == 0 {
        out.push(';');
    }
    out
}

#[test]
fn test_fuzz_valid_expressions() {
    let mut rng = Rng(0x5EED_0001);
    for _ in 0..400 {
        let input = gen_input(&mut rng);
        assert_equivalent(&input);
    }
}

#[test]
fn test_fuzz_token_soup() {
    const TOKENS: &[&str] = &[
        "x", "1", "as", "+", "-", "*", "/", "^", "(", ")", "[", "]", ",", ";", ".", "f",
    ];
    let mut rng = Rng(0x5EED_0002);
    for _ in 0..400 {
        let len = rng.below(12);
        let mut input = String::new();
        for i in 0..len {
            if i > 0 {
                input.push(' ');
            }
            input.push_str(TOKENS[rng.below(TOKENS.len())]);
        }
        assert_equivalent(&input);
    }
}

/// Pinned corners: the soft-keyword collisions that force runtime escapes.
#[test]
fn test_escape_corners() {
    for input in [
        "as",                // the keyword alone is an expression
        "as as as",          // expression 'as', aliased to 'as'
        "as.as as as",       // qualified soft keywords with alias
        "a as as, as as a;", // both orders across a list
        "x + as as y",       // ladder + alias
        "f(as) as as",       // call arg + alias
        "a[as] as x",        // index + alias
        "- as as as",        // prefix + alias
    ] {
        assert_equivalent(input);
    }
}

/// The static parser must contain a hybrid table (adaptive escapes) and a
/// precedence dispatch; the adaptive twin must contain neither.
#[test]
fn test_generated_shape() {
    let stat = include_str!("gen/fuzzexprparser.rs");
    assert!(stat.contains("adaptive escapes"), "expected a hybrid table");
    assert!(
        stat.contains("precedence-dispatched"),
        "expected a dispatch"
    );
    let adap = include_str!("gen/adaptive/fuzzexprparser.rs");
    assert!(
        !adap.contains("dfa_predict"),
        "adaptive twin must not use tables"
    );
}
