#![allow(nonstandard_style)]
// Generated from StaticDFA.g4 by ANTLR 4.13.2
use dbt_antlr4::errors::ANTLRError;
use dbt_antlr4::token::{CommonToken, Token};
use dbt_antlr4::tree::ParseTreeListener;
use super::staticdfaparser::*;

pub trait StaticDFAListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, StaticDFAParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by the {@code createTable}
    /// labeled alternative in {@link StaticDFAParser#stat}.
    /// @param ctx the parse tree
    fn enter_createTable<'input: 'arena>(&mut self, _ctx: &CreateTableContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code createTable}
    /// labeled alternative in {@link StaticDFAParser#stat}.
    /// @param ctx the parse tree
    fn exit_createTable<'input: 'arena>(&mut self, _ctx: &CreateTableContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code createView}
    /// labeled alternative in {@link StaticDFAParser#stat}.
    /// @param ctx the parse tree
    fn enter_createView<'input: 'arena>(&mut self, _ctx: &CreateViewContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code createView}
    /// labeled alternative in {@link StaticDFAParser#stat}.
    /// @param ctx the parse tree
    fn exit_createView<'input: 'arena>(&mut self, _ctx: &CreateViewContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code dropTable}
    /// labeled alternative in {@link StaticDFAParser#stat}.
    /// @param ctx the parse tree
    fn enter_dropTable<'input: 'arena>(&mut self, _ctx: &DropTableContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code dropTable}
    /// labeled alternative in {@link StaticDFAParser#stat}.
    /// @param ctx the parse tree
    fn exit_dropTable<'input: 'arena>(&mut self, _ctx: &DropTableContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code call}
    /// labeled alternative in {@link StaticDFAParser#expr}.
    /// @param ctx the parse tree
    fn enter_call<'input: 'arena>(&mut self, _ctx: &CallContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code call}
    /// labeled alternative in {@link StaticDFAParser#expr}.
    /// @param ctx the parse tree
    fn exit_call<'input: 'arena>(&mut self, _ctx: &CallContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code ref}
    /// labeled alternative in {@link StaticDFAParser#expr}.
    /// @param ctx the parse tree
    fn enter_ref<'input: 'arena>(&mut self, _ctx: &RefContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code ref}
    /// labeled alternative in {@link StaticDFAParser#expr}.
    /// @param ctx the parse tree
    fn exit_ref<'input: 'arena>(&mut self, _ctx: &RefContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link StaticDFAParser#name}.
    /// @param ctx the parse tree
    fn enter_name<'input: 'arena>(&mut self, _ctx: &NameContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link StaticDFAParser#name}.
    /// @param ctx the parse tree
    fn exit_name<'input: 'arena>(&mut self, _ctx: &NameContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link StaticDFAParser#starLoop}.
    /// @param ctx the parse tree
    fn enter_starLoop<'input: 'arena>(&mut self, _ctx: &StarLoopContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link StaticDFAParser#starLoop}.
    /// @param ctx the parse tree
    fn exit_starLoop<'input: 'arena>(&mut self, _ctx: &StarLoopContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link StaticDFAParser#plusLoop}.
    /// @param ctx the parse tree
    fn enter_plusLoop<'input: 'arena>(&mut self, _ctx: &PlusLoopContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link StaticDFAParser#plusLoop}.
    /// @param ctx the parse tree
    fn exit_plusLoop<'input: 'arena>(&mut self, _ctx: &PlusLoopContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link StaticDFAParser#opt}.
    /// @param ctx the parse tree
    fn enter_opt<'input: 'arena>(&mut self, _ctx: &OptContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link StaticDFAParser#opt}.
    /// @param ctx the parse tree
    fn exit_opt<'input: 'arena>(&mut self, _ctx: &OptContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code dupFirst}
    /// labeled alternative in {@link StaticDFAParser#dup}.
    /// @param ctx the parse tree
    fn enter_dupFirst<'input: 'arena>(&mut self, _ctx: &DupFirstContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code dupFirst}
    /// labeled alternative in {@link StaticDFAParser#dup}.
    /// @param ctx the parse tree
    fn exit_dupFirst<'input: 'arena>(&mut self, _ctx: &DupFirstContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by the {@code dupSecond}
    /// labeled alternative in {@link StaticDFAParser#dup}.
    /// @param ctx the parse tree
    fn enter_dupSecond<'input: 'arena>(&mut self, _ctx: &DupSecondContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by the {@code dupSecond}
    /// labeled alternative in {@link StaticDFAParser#dup}.
    /// @param ctx the parse tree
    fn exit_dupSecond<'input: 'arena>(&mut self, _ctx: &DupSecondContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
