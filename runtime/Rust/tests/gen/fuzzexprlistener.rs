#![allow(nonstandard_style)]
// Generated from FuzzExpr.g4 by ANTLR 4.13.2
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::token::{CommonToken, Token};
use dbt_antlr4::tree::ParseTreeListener;
use super::fuzzexprparser::*;

pub trait FuzzExprListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, FuzzExprParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link FuzzExprParser#s}.
    /// @param ctx the parse tree
    fn enter_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link FuzzExprParser#s}.
    /// @param ctx the parse tree
    fn exit_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link FuzzExprParser#item}.
    /// @param ctx the parse tree
    fn enter_item<'input: 'arena>(&mut self, _ctx: &ItemContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link FuzzExprParser#item}.
    /// @param ctx the parse tree
    fn exit_item<'input: 'arena>(&mut self, _ctx: &ItemContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link FuzzExprParser#name}.
    /// @param ctx the parse tree
    fn enter_name<'input: 'arena>(&mut self, _ctx: &NameContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link FuzzExprParser#name}.
    /// @param ctx the parse tree
    fn exit_name<'input: 'arena>(&mut self, _ctx: &NameContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link FuzzExprParser#q}.
    /// @param ctx the parse tree
    fn enter_q<'input: 'arena>(&mut self, _ctx: &QContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link FuzzExprParser#q}.
    /// @param ctx the parse tree
    fn exit_q<'input: 'arena>(&mut self, _ctx: &QContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link FuzzExprParser#e}.
    /// @param ctx the parse tree
    fn enter_e<'input: 'arena>(&mut self, _ctx: &EContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link FuzzExprParser#e}.
    /// @param ctx the parse tree
    fn exit_e<'input: 'arena>(&mut self, _ctx: &EContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link FuzzExprParser#atom}.
    /// @param ctx the parse tree
    fn enter_atom<'input: 'arena>(&mut self, _ctx: &AtomContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link FuzzExprParser#atom}.
    /// @param ctx the parse tree
    fn exit_atom<'input: 'arena>(&mut self, _ctx: &AtomContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
