// Generated from BetweenExpr.g4 by ANTLR 4.13.2

use super::betweenexprparser::*;
use dbt_antlr4::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by BetweenExprParser.

pub trait BetweenExprBaseListener<'arena>:
    ParseTreeListener<'arena, BetweenExprParserNodeKind> {

    /**
     * Enter a parse tree produced by \{@link BetweenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_s(&mut self, _ctx: &SContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  BetweenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, _ctx: &SContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link BetweenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_not(&mut self, _ctx: &NotContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  BetweenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_not(&mut self, _ctx: &NotContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link BetweenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_predicated(&mut self, _ctx: &PredicatedContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  BetweenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_predicated(&mut self, _ctx: &PredicatedContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link BetweenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_or(&mut self, _ctx: &OrContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  BetweenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_or(&mut self, _ctx: &OrContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link BetweenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_and(&mut self, _ctx: &AndContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  BetweenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_and(&mut self, _ctx: &AndContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link BetweenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_between(&mut self, _ctx: &BetweenContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  BetweenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_between(&mut self, _ctx: &BetweenContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link BetweenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_cmp(&mut self, _ctx: &CmpContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  BetweenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cmp(&mut self, _ctx: &CmpContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link BetweenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_v(&mut self, _ctx: &VContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  BetweenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_v(&mut self, _ctx: &VContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link BetweenExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_atom(&mut self, _ctx: &AtomContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  BetweenExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom(&mut self, _ctx: &AtomContext<'input, 'arena>) {}


}