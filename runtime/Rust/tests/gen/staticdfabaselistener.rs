// Generated from StaticDFA.g4 by ANTLR 4.13.2

use super::staticdfaparser::*;
use dbt_antlr4::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by StaticDFAParser.

pub trait StaticDFABaseListener<'arena>:
    ParseTreeListener<'arena, StaticDFAParserNodeKind> {

    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_createtable(&mut self, _ctx: &CreateTableContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_createtable(&mut self, _ctx: &CreateTableContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_createview(&mut self, _ctx: &CreateViewContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_createview(&mut self, _ctx: &CreateViewContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_droptable(&mut self, _ctx: &DropTableContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_droptable(&mut self, _ctx: &DropTableContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_call(&mut self, _ctx: &CallContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_call(&mut self, _ctx: &CallContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_ref(&mut self, _ctx: &RefContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ref(&mut self, _ctx: &RefContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_name(&mut self, _ctx: &NameContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_name(&mut self, _ctx: &NameContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_starloop(&mut self, _ctx: &StarLoopContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_starloop(&mut self, _ctx: &StarLoopContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_plusloop(&mut self, _ctx: &PlusLoopContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_plusloop(&mut self, _ctx: &PlusLoopContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_opt(&mut self, _ctx: &OptContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_opt(&mut self, _ctx: &OptContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_dupfirst(&mut self, _ctx: &DupFirstContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dupfirst(&mut self, _ctx: &DupFirstContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link StaticDFABaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_dupsecond(&mut self, _ctx: &DupSecondContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  StaticDFABaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dupsecond(&mut self, _ctx: &DupSecondContext<'input, 'arena>) {}


}