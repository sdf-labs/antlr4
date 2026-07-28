#![allow(nonstandard_style)]
// Generated from GuardExpr.g4 by ANTLR 4.13.2
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::token::{CommonToken, Token};
use dbt_antlr4::tree::ParseTreeListener;
use super::guardexprparser::*;

pub trait GuardExprListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, GuardExprParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link GuardExprParser#prog}.
    /// @param ctx the parse tree
    fn enter_prog<'input: 'arena>(&mut self, _ctx: &ProgContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link GuardExprParser#prog}.
    /// @param ctx the parse tree
    fn exit_prog<'input: 'arena>(&mut self, _ctx: &ProgContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code assign}
    /// labeled alternative in {@link GuardExprParser#stmt}.
    /// @param ctx the parse tree
    fn enter_assign<'input: 'arena>(&mut self, _ctx: &AssignContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code assign}
    /// labeled alternative in {@link GuardExprParser#stmt}.
    /// @param ctx the parse tree
    fn exit_assign<'input: 'arena>(&mut self, _ctx: &AssignContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code eval}
    /// labeled alternative in {@link GuardExprParser#stmt}.
    /// @param ctx the parse tree
    fn enter_eval<'input: 'arena>(&mut self, _ctx: &EvalContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code eval}
    /// labeled alternative in {@link GuardExprParser#stmt}.
    /// @param ctx the parse tree
    fn exit_eval<'input: 'arena>(&mut self, _ctx: &EvalContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code not}
    /// labeled alternative in {@link GuardExprParser#expr}.
    /// @param ctx the parse tree
    fn enter_not<'input: 'arena>(&mut self, _ctx: &NotContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code not}
    /// labeled alternative in {@link GuardExprParser#expr}.
    /// @param ctx the parse tree
    fn exit_not<'input: 'arena>(&mut self, _ctx: &NotContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code predicated}
    /// labeled alternative in {@link GuardExprParser#expr}.
    /// @param ctx the parse tree
    fn enter_predicated<'input: 'arena>(&mut self, _ctx: &PredicatedContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code predicated}
    /// labeled alternative in {@link GuardExprParser#expr}.
    /// @param ctx the parse tree
    fn exit_predicated<'input: 'arena>(&mut self, _ctx: &PredicatedContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code or}
    /// labeled alternative in {@link GuardExprParser#expr}.
    /// @param ctx the parse tree
    fn enter_or<'input: 'arena>(&mut self, _ctx: &OrContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code or}
    /// labeled alternative in {@link GuardExprParser#expr}.
    /// @param ctx the parse tree
    fn exit_or<'input: 'arena>(&mut self, _ctx: &OrContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code and}
    /// labeled alternative in {@link GuardExprParser#expr}.
    /// @param ctx the parse tree
    fn enter_and<'input: 'arena>(&mut self, _ctx: &AndContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code and}
    /// labeled alternative in {@link GuardExprParser#expr}.
    /// @param ctx the parse tree
    fn exit_and<'input: 'arena>(&mut self, _ctx: &AndContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code cmp}
    /// labeled alternative in {@link GuardExprParser#pred}.
    /// @param ctx the parse tree
    fn enter_cmp<'input: 'arena>(&mut self, _ctx: &CmpContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code cmp}
    /// labeled alternative in {@link GuardExprParser#pred}.
    /// @param ctx the parse tree
    fn exit_cmp<'input: 'arena>(&mut self, _ctx: &CmpContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code inList}
    /// labeled alternative in {@link GuardExprParser#pred}.
    /// @param ctx the parse tree
    fn enter_inList<'input: 'arena>(&mut self, _ctx: &InListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code inList}
    /// labeled alternative in {@link GuardExprParser#pred}.
    /// @param ctx the parse tree
    fn exit_inList<'input: 'arena>(&mut self, _ctx: &InListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link GuardExprParser#value}.
    /// @param ctx the parse tree
    fn enter_value<'input: 'arena>(&mut self, _ctx: &ValueContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link GuardExprParser#value}.
    /// @param ctx the parse tree
    fn exit_value<'input: 'arena>(&mut self, _ctx: &ValueContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
