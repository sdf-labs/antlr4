// Generated from GuardExpr.g4 by ANTLR 4.13.2

use super::guardexprparser::*;
use dbt_antlr4::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by GuardExprParser.

pub trait GuardExprBaseListener<'arena>:
    ParseTreeListener<'arena, GuardExprParserNodeKind> {

    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_prog(&mut self, _ctx: &ProgContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_prog(&mut self, _ctx: &ProgContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_assign(&mut self, _ctx: &AssignContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_assign(&mut self, _ctx: &AssignContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_eval(&mut self, _ctx: &EvalContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_eval(&mut self, _ctx: &EvalContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_not(&mut self, _ctx: &NotContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_not(&mut self, _ctx: &NotContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_predicated(&mut self, _ctx: &PredicatedContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_predicated(&mut self, _ctx: &PredicatedContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_or(&mut self, _ctx: &OrContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_or(&mut self, _ctx: &OrContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_and(&mut self, _ctx: &AndContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_and(&mut self, _ctx: &AndContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_cmp(&mut self, _ctx: &CmpContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_cmp(&mut self, _ctx: &CmpContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_inlist(&mut self, _ctx: &InListContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_inlist(&mut self, _ctx: &InListContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link GuardExprBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_value(&mut self, _ctx: &ValueContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  GuardExprBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_value(&mut self, _ctx: &ValueContext<'input, 'arena>) {}


}