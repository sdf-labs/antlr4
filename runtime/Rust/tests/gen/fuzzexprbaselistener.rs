// Generated from FuzzExpr.g4 by ANTLR 4.13.2

use super::fuzzexprparser::*;
use dbt_antlr4::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by FuzzExprParser.

pub trait FuzzExprBaseListener<'arena>:
    ParseTreeListener<'arena, FuzzExprParserNodeKind> {

    /**
     * Enter a parse tree produced by \{@link FuzzExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_s(&mut self, _ctx: &SContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  FuzzExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, _ctx: &SContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link FuzzExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_item(&mut self, _ctx: &ItemContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  FuzzExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_item(&mut self, _ctx: &ItemContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link FuzzExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_name(&mut self, _ctx: &NameContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  FuzzExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_name(&mut self, _ctx: &NameContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link FuzzExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_q(&mut self, _ctx: &QContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  FuzzExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_q(&mut self, _ctx: &QContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link FuzzExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_e(&mut self, _ctx: &EContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  FuzzExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_e(&mut self, _ctx: &EContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link FuzzExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_atom(&mut self, _ctx: &AtomContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  FuzzExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom(&mut self, _ctx: &AtomContext<'input, 'arena>) {}


}