#![allow(nonstandard_style)]
// Generated from LrDfa.g4 by ANTLR 4.13.2
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::token::{CommonToken, Token};
use dbt_antlr4::tree::ParseTreeListener;
use super::lrdfaparser::*;

pub trait LrDfaListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, LrDfaParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link LrDfaParser#s}.
    /// @param ctx the parse tree
    fn enter_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link LrDfaParser#s}.
    /// @param ctx the parse tree
    fn exit_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code Add}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn enter_Add<'input: 'arena>(&mut self, _ctx: &AddContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code Add}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn exit_Add<'input: 'arena>(&mut self, _ctx: &AddContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code Neg}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn enter_Neg<'input: 'arena>(&mut self, _ctx: &NegContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code Neg}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn exit_Neg<'input: 'arena>(&mut self, _ctx: &NegContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code Mul}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn enter_Mul<'input: 'arena>(&mut self, _ctx: &MulContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code Mul}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn exit_Mul<'input: 'arena>(&mut self, _ctx: &MulContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code Ternary}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn enter_Ternary<'input: 'arena>(&mut self, _ctx: &TernaryContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code Ternary}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn exit_Ternary<'input: 'arena>(&mut self, _ctx: &TernaryContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code Pow}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn enter_Pow<'input: 'arena>(&mut self, _ctx: &PowContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code Pow}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn exit_Pow<'input: 'arena>(&mut self, _ctx: &PowContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code Index}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn enter_Index<'input: 'arena>(&mut self, _ctx: &IndexContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code Index}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn exit_Index<'input: 'arena>(&mut self, _ctx: &IndexContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code Id}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn enter_Id<'input: 'arena>(&mut self, _ctx: &IdContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code Id}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn exit_Id<'input: 'arena>(&mut self, _ctx: &IdContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code Fact}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn enter_Fact<'input: 'arena>(&mut self, _ctx: &FactContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code Fact}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn exit_Fact<'input: 'arena>(&mut self, _ctx: &FactContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code Int}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn enter_Int<'input: 'arena>(&mut self, _ctx: &IntContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code Int}
    /// labeled alternative in {@link LrDfaParser#e}.
    /// @param ctx the parse tree
    fn exit_Int<'input: 'arena>(&mut self, _ctx: &IntContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
