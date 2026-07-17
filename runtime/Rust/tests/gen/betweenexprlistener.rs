#![allow(nonstandard_style)]
// Generated from BetweenExpr.g4 by ANTLR 4.13.2
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::token::{CommonToken, Token};
use dbt_antlr4::tree::ParseTreeListener;
use super::betweenexprparser::*;

pub trait BetweenExprListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, BetweenExprParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link BetweenExprParser#s}.
    /// @param ctx the parse tree
    fn enter_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link BetweenExprParser#s}.
    /// @param ctx the parse tree
    fn exit_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code not}
    /// labeled alternative in {@link BetweenExprParser#b}.
    /// @param ctx the parse tree
    fn enter_not<'input: 'arena>(&mut self, _ctx: &NotContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code not}
    /// labeled alternative in {@link BetweenExprParser#b}.
    /// @param ctx the parse tree
    fn exit_not<'input: 'arena>(&mut self, _ctx: &NotContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code predicated}
    /// labeled alternative in {@link BetweenExprParser#b}.
    /// @param ctx the parse tree
    fn enter_predicated<'input: 'arena>(&mut self, _ctx: &PredicatedContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code predicated}
    /// labeled alternative in {@link BetweenExprParser#b}.
    /// @param ctx the parse tree
    fn exit_predicated<'input: 'arena>(&mut self, _ctx: &PredicatedContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code or}
    /// labeled alternative in {@link BetweenExprParser#b}.
    /// @param ctx the parse tree
    fn enter_or<'input: 'arena>(&mut self, _ctx: &OrContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code or}
    /// labeled alternative in {@link BetweenExprParser#b}.
    /// @param ctx the parse tree
    fn exit_or<'input: 'arena>(&mut self, _ctx: &OrContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code and}
    /// labeled alternative in {@link BetweenExprParser#b}.
    /// @param ctx the parse tree
    fn enter_and<'input: 'arena>(&mut self, _ctx: &AndContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code and}
    /// labeled alternative in {@link BetweenExprParser#b}.
    /// @param ctx the parse tree
    fn exit_and<'input: 'arena>(&mut self, _ctx: &AndContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code between}
    /// labeled alternative in {@link BetweenExprParser#pred}.
    /// @param ctx the parse tree
    fn enter_between<'input: 'arena>(&mut self, _ctx: &BetweenContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code between}
    /// labeled alternative in {@link BetweenExprParser#pred}.
    /// @param ctx the parse tree
    fn exit_between<'input: 'arena>(&mut self, _ctx: &BetweenContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code cmp}
    /// labeled alternative in {@link BetweenExprParser#pred}.
    /// @param ctx the parse tree
    fn enter_cmp<'input: 'arena>(&mut self, _ctx: &CmpContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code cmp}
    /// labeled alternative in {@link BetweenExprParser#pred}.
    /// @param ctx the parse tree
    fn exit_cmp<'input: 'arena>(&mut self, _ctx: &CmpContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link BetweenExprParser#v}.
    /// @param ctx the parse tree
    fn enter_v<'input: 'arena>(&mut self, _ctx: &VContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link BetweenExprParser#v}.
    /// @param ctx the parse tree
    fn exit_v<'input: 'arena>(&mut self, _ctx: &VContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link BetweenExprParser#atom}.
    /// @param ctx the parse tree
    fn enter_atom<'input: 'arena>(&mut self, _ctx: &AtomContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link BetweenExprParser#atom}.
    /// @param ctx the parse tree
    fn exit_atom<'input: 'arena>(&mut self, _ctx: &AtomContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
