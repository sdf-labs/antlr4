// Generated from ParenExpr.g4 by ANTLR 4.13.2

use super::parenexprparser::*;
use dbt_antlr4::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by ParenExprParser.

pub trait ParenExprBaseListener<'arena>:
    ParseTreeListener<'arena, ParenExprParserNodeKind> {

    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_s(&mut self, _ctx: &SContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, _ctx: &SContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_add(&mut self, _ctx: &AddContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_add(&mut self, _ctx: &AddContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_col(&mut self, _ctx: &ColContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_col(&mut self, _ctx: &ColContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_parens(&mut self, _ctx: &ParensContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_parens(&mut self, _ctx: &ParensContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_subquery(&mut self, _ctx: &SubqueryContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_subquery(&mut self, _ctx: &SubqueryContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_rowconstructor(&mut self, _ctx: &RowConstructorContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_rowconstructor(&mut self, _ctx: &RowConstructorContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_mul(&mut self, _ctx: &MulContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mul(&mut self, _ctx: &MulContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_num(&mut self, _ctx: &NumContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_num(&mut self, _ctx: &NumContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_unary(&mut self, _ctx: &UnaryContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unary(&mut self, _ctx: &UnaryContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_select(&mut self, _ctx: &SelectContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_select(&mut self, _ctx: &SelectContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_table(&mut self, _ctx: &TableContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_table(&mut self, _ctx: &TableContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link ParenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_nested(&mut self, _ctx: &NestedContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  ParenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_nested(&mut self, _ctx: &NestedContext<'input, 'arena>) {}


}