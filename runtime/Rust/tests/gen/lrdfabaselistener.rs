// Generated from LrDfa.g4 by ANTLR 4.13.2

use super::lrdfaparser::*;
use dbt_antlr4::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by LrDfaParser.

pub trait LrDfaBaseListener<'arena>:
    ParseTreeListener<'arena, LrDfaParserNodeKind> {

    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_s(&mut self, _ctx: &SContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_s(&mut self, _ctx: &SContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_add(&mut self, _ctx: &AddContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_add(&mut self, _ctx: &AddContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_neg(&mut self, _ctx: &NegContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_neg(&mut self, _ctx: &NegContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_mul(&mut self, _ctx: &MulContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mul(&mut self, _ctx: &MulContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_ternary(&mut self, _ctx: &TernaryContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ternary(&mut self, _ctx: &TernaryContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_pow(&mut self, _ctx: &PowContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_pow(&mut self, _ctx: &PowContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_index(&mut self, _ctx: &IndexContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_index(&mut self, _ctx: &IndexContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_id(&mut self, _ctx: &IdContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_id(&mut self, _ctx: &IdContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_fact(&mut self, _ctx: &FactContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_fact(&mut self, _ctx: &FactContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link LrDfaBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_int(&mut self, _ctx: &IntContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  LrDfaBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_int(&mut self, _ctx: &IntContext<'input, 'arena>) {}


}