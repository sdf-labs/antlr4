#![allow(nonstandard_style)]
// Generated from ParenExpr.g4 by ANTLR 4.13.2
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::token::{CommonToken, Token};
use dbt_antlr4::tree::ParseTreeListener;
use super::parenexprparser::*;

pub trait ParenExprListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, ParenExprParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link ParenExprParser#s}.
    /// @param ctx the parse tree
    fn enter_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ParenExprParser#s}.
    /// @param ctx the parse tree
    fn exit_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code add}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn enter_add<'input: 'arena>(&mut self, _ctx: &AddContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code add}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn exit_add<'input: 'arena>(&mut self, _ctx: &AddContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code col}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn enter_col<'input: 'arena>(&mut self, _ctx: &ColContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code col}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn exit_col<'input: 'arena>(&mut self, _ctx: &ColContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code parens}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn enter_parens<'input: 'arena>(&mut self, _ctx: &ParensContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code parens}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn exit_parens<'input: 'arena>(&mut self, _ctx: &ParensContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code subquery}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn enter_subquery<'input: 'arena>(&mut self, _ctx: &SubqueryContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code subquery}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn exit_subquery<'input: 'arena>(&mut self, _ctx: &SubqueryContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code rowConstructor}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn enter_rowConstructor<'input: 'arena>(&mut self, _ctx: &RowConstructorContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code rowConstructor}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn exit_rowConstructor<'input: 'arena>(&mut self, _ctx: &RowConstructorContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code mul}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn enter_mul<'input: 'arena>(&mut self, _ctx: &MulContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code mul}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn exit_mul<'input: 'arena>(&mut self, _ctx: &MulContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code num}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn enter_num<'input: 'arena>(&mut self, _ctx: &NumContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code num}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn exit_num<'input: 'arena>(&mut self, _ctx: &NumContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code unary}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn enter_unary<'input: 'arena>(&mut self, _ctx: &UnaryContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code unary}
    /// labeled alternative in {@link ParenExprParser#e}.
    /// @param ctx the parse tree
    fn exit_unary<'input: 'arena>(&mut self, _ctx: &UnaryContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code select}
    /// labeled alternative in {@link ParenExprParser#q}.
    /// @param ctx the parse tree
    fn enter_select<'input: 'arena>(&mut self, _ctx: &SelectContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code select}
    /// labeled alternative in {@link ParenExprParser#q}.
    /// @param ctx the parse tree
    fn exit_select<'input: 'arena>(&mut self, _ctx: &SelectContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code table}
    /// labeled alternative in {@link ParenExprParser#q}.
    /// @param ctx the parse tree
    fn enter_table<'input: 'arena>(&mut self, _ctx: &TableContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code table}
    /// labeled alternative in {@link ParenExprParser#q}.
    /// @param ctx the parse tree
    fn exit_table<'input: 'arena>(&mut self, _ctx: &TableContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code nested}
    /// labeled alternative in {@link ParenExprParser#q}.
    /// @param ctx the parse tree
    fn enter_nested<'input: 'arena>(&mut self, _ctx: &NestedContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code nested}
    /// labeled alternative in {@link ParenExprParser#q}.
    /// @param ctx the parse tree
    fn exit_nested<'input: 'arena>(&mut self, _ctx: &NestedContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
