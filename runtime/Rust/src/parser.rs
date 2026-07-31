//! Base parser implementation
use std::borrow::{Borrow, Cow};
use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::arena::Arena;
use crate::atn::ATN;
use crate::atn_simulator::{IATNSimulator, ParserATNSimulatorManager};
use crate::error_listener::{
    ConsoleErrorListener, ErrorListener, ErrorListenerDelegate, ProxyErrorListener,
};
use crate::error_strategy::ErrorStrategy;
use crate::errors::ANTLRError;
use crate::interval_set::IntervalSet;
use crate::parser_atn_simulator::ParserATNSimulator;
use crate::recognizer::{Actions, Recognizer};
use crate::rule_context::{states_stack, RuleContext as _};
use crate::token::{OwningToken, Token, TOKEN_EOF};
use crate::token_factory::TokenFactory;
use crate::token_stream::TokenStream;
use crate::tree::{NodeKindType, ParseTreeListener, ParseTreeWalker, Tree as _, TreeNode};
use crate::utils::cell_update;
use crate::vocabulary::Vocabulary;

#[cfg(feature = "recursion-limit")]
const DEFAULT_RECURSION_LIMIT: u32 = 2000;

/// parser functionality required for `ParserATNSimulator` to work
#[allow(missing_docs)]
pub trait Parser<'input, 'arena, TF>: Recognizer<'input, 'arena, TF::Tok>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn get_arena(&self) -> &'arena Arena;

    fn get_interpreter(&self) -> &'arena ParserATNSimulator<'arena>;

    fn get_token_factory(&self) -> &TF;

    fn get_current_context(&self) -> &'arena TreeNode<'input, 'arena, Self::Node, TF::Tok>;

    fn consume(
        &mut self,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<(), ANTLRError>
    where
        Self: Sized;

    fn precpred(
        &self,
        localctx: Option<&TreeNode<'input, 'arena, Self::Node, TF::Tok>>,
        precedence: i32,
    ) -> bool;

    fn get_input_stream_mut(&mut self) -> &mut dyn TokenStream<'input, 'arena, TF>;
    fn get_input_stream(&self) -> &dyn TokenStream<'input, 'arena, TF>;
    fn get_current_token(&self) -> &'arena TF::Tok;
    fn get_expected_tokens<'a>(&'a self) -> Cow<'a, IntervalSet>;

    fn add_error_listener(
        &mut self,
        listener: Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'input>,
    ) where
        Self: Sized;

    fn remove_error_listeners(&mut self);

    fn notify_error_listeners(
        &self,
        msg: String,
        offending_token: Option<isize>,
        err: Option<&ANTLRError>,
    );
    fn get_error_lister_dispatch<'a>(
        &'a self,
    ) -> Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'a>
    where
        Self: Sized;

    fn is_expected_token(&self, symbol: i32) -> bool;
    fn get_precedence(&self) -> i32;

    fn get_state(&self) -> i32;
    fn set_state(&mut self, v: i32);
    fn get_rule_invocation_stack(&self) -> Vec<String>;

    #[cfg(feature = "recursion-limit")]
    fn get_recursion_limit(&self) -> u32;
    #[cfg(feature = "recursion-limit")]
    fn set_recursion_limit(&mut self, v: u32);
}

type ResumableNode<'input, 'arena, Node, TF> = (
    &'arena TreeNode<'input, 'arena, Node, <TF as TokenFactory<'input, 'arena>>::Tok>,
    usize,
    isize,
);

/// Abstract base parser implementation
///
/// Only meant to be instantiated by generated parsers
pub struct BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    // Grammar-specific implementation of parser functionality required for
    // `ParserATNSimulator` to work
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    // Token factory used by the input stream
    TF: TokenFactory<'input, 'arena> + 'arena,
    // Token stream (lexer)
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    interp: Option<ParserATNSimulator<'arena>>,
    atn_manager: &'static ParserATNSimulatorManager,
    global_cache_threshold: usize,

    /// Rule context parser is currently processing
    ctx: *mut (),

    /// Track the {@link ParserRuleContext} objects during the parse and hook
    /// them up using the {@link ParserRuleContext#children} list so that it
    /// forms a parse tree. The {@link ParserRuleContext} returned from the start
    /// rule represents the root of the parse tree.
    ///
    /// <p>Note that if we are not building parse trees, rule contexts only point
    /// upwards. When a rule exits, it returns the context bute that gets garbage
    /// collected if nobody holds a reference. It points upwards but nobody
    /// points at it. </p>
    ///
    /// <p>When we build parse trees, we are adding all of these contexts to
    /// {@link ParserRuleContext#children} list. Contexts are then not candidates
    /// for garbage collection.</p>
    ///
    /// Returns {@code true} if a complete parse tree will be constructed while
    /// parsing, otherwise {@code false}
    pub build_parse_trees: bool,

    /// true if parser reached EOF
    pub matched_eof: bool,

    state: i32,
    /// Token stream that is currently used by this parser
    pub input: Input,
    precedence_stack: Vec<i32>,
    #[cfg(feature = "recursion-limit")]
    pub recursion_limit: u32,
    #[cfg(feature = "recursion-limit")]
    current_recursion_depth: u32,

    parse_listeners: Vec<Box<Node::Listener>>,
    _syntax_errors: Cell<i32>,
    error_listeners: Vec<ErrorListenerDelegate<'input, 'arena, Self, TF::Tok>>,
    /// Resume-mode state of a shared-descent factoring: the parsed
    /// prefix context, its rule, and the stream position just past it
    /// (the tail); see `start_resume`.
    resume_node: Option<ResumableNode<'input, 'arena, Node, TF>>,
    #[doc(hidden)]
    pub resume_take_target: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
    /// Mute all listeners (parse and error), counted for nesting; see
    /// `begin_neutral_parse`.
    muted: u32,

    pub arena: &'arena Arena,
    ext: Ext,
    pd: PhantomData<(
        &'input (),
        &'arena TF,
        //        &'arena TreeNode<'input, 'arena, Node, TF::Tok>,
    )>,
}

impl<'input, 'arena, Ext, Node, Input, TF> Deref
    for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    type Target = Ext;

    fn deref(&self) -> &Self::Target {
        &self.ext
    }
}

impl<'input, 'arena, Ext, Node, Input, TF> DerefMut
    for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ext
    }
}

pub trait ParserRecog<'input, 'arena, R, Tok>: Actions<'input, 'arena, R, Tok>
where
    'input: 'arena,
    R: Recognizer<'input, 'arena, Tok>,
    Tok: Token + 'input,
{
    fn get_atn_simulator_man(&self) -> &'static ParserATNSimulatorManager;
}

impl<'input, 'arena, Ext, Node, Input, TF> Recognizer<'input, 'arena, TF::Tok>
    for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    type Node = Node;

    fn sempred(
        &mut self,
        localctx: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
        rule_index: i32,
        action_index: i32,
    ) -> bool {
        Ext::sempred(localctx, rule_index, action_index, self)
    }

    fn get_rule_names(&self) -> &[&str] {
        self.ext.get_rule_names()
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        self.ext.get_vocabulary()
    }

    fn get_grammar_file_name(&self) -> &str {
        self.ext.get_grammar_file_name()
    }

    fn get_atn(&self) -> &ATN {
        self.interp.as_ref().unwrap().atn()
    }
}

impl<'input, 'arena, Ext, Node, Input, TF> Parser<'input, 'arena, TF>
    for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    fn get_arena(&self) -> &'arena Arena {
        self.arena
    }

    #[inline(always)]
    fn get_interpreter(&self) -> &'arena ParserATNSimulator<'arena> {
        unsafe {
            std::mem::transmute::<&ParserATNSimulator<'arena>, &'arena ParserATNSimulator<'arena>>(
                self.interp.as_ref().unwrap(),
            )
        }
    }

    fn get_token_factory(&self) -> &TF {
        self.input.get_token_source().get_token_factory()
    }

    #[inline(always)]
    fn get_current_context(&self) -> &'arena TreeNode<'input, 'arena, Node, TF::Tok> {
        self.ctx().unwrap()
    }

    fn consume(
        &mut self,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<(), ANTLRError> {
        let o = self.get_current_token();
        if o.borrow().get_token_type() != TOKEN_EOF {
            self.input.consume();
        }
        if self.build_parse_trees || !self.parse_listeners.is_empty() {
            if err_handler.in_error_recovery_mode(self) {
                // todo report ructc inference issue
                let node = self.create_error_node(o)?;
                self.add_child_to_ctx(node);
                if !self.is_muted() {
                    for listener in &mut self.parse_listeners {
                        listener.visit_error_node(
                            node.as_error_node()
                                .expect("node was created as error node"),
                        )?
                    }
                }
            } else {
                let node = self.create_token_node(o)?;
                self.add_child_to_ctx(node);
                if !self.is_muted() {
                    for listener in &mut self.parse_listeners {
                        listener.visit_terminal(
                            node.as_terminal_node()
                                .expect("node was created as terminal node"),
                        )?
                    }
                }
            }
        }
        Ok(())
    }

    fn precpred(
        &self,
        _localctx: Option<&TreeNode<'input, 'arena, Node, TF::Tok>>,
        precedence: i32,
    ) -> bool {
        //        localctx.map(|it|println!("check at{}",it.to_string_tree(self)));
        //        println!("{}",self.get_precedence());
        precedence >= self.get_precedence()
    }

    fn get_input_stream_mut(&mut self) -> &mut dyn TokenStream<'input, 'arena, TF> {
        &mut self.input //.as_mut()
    }

    fn get_input_stream(&self) -> &dyn TokenStream<'input, 'arena, TF> {
        &self.input
    }

    #[inline]
    fn get_current_token(&self) -> &'arena TF::Tok {
        self.input.get(self.input.index())
    }

    fn get_expected_tokens<'a>(&'a self) -> Cow<'a, IntervalSet> {
        let states_stack = states_stack(self.ctx().unwrap());
        self.interp
            .as_ref()
            .unwrap()
            .atn()
            .get_expected_tokens::<TF::Tok>(self.state, states_stack)
    }

    fn add_error_listener(
        &mut self,
        listener: Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'input>,
    ) {
        self.error_listeners
            .push(ErrorListenerDelegate::new(listener))
    }

    fn remove_error_listeners(&mut self) {
        self.error_listeners.clear();
    }

    fn notify_error_listeners(
        &self,
        msg: String,
        offending_token: Option<isize>,
        err: Option<&ANTLRError>,
    ) {
        cell_update(&self._syntax_errors, |it| it + 1);

        if self.is_muted() {
            return;
        }

        let offending_token: Option<&_> = match offending_token {
            None => Some(self.get_current_token().borrow()),
            Some(x) => Some(self.input.get(x).borrow()),
        };
        let line = offending_token.map(|x| x.get_line()).unwrap_or(0);
        let column = offending_token
            .map(|x| x.get_char_position_in_line())
            .unwrap_or(-1);

        for listener in self.error_listeners.iter() {
            listener.syntax_error(
                self,
                offending_token.map(|x| x as _),
                line,
                column,
                &msg,
                err,
            )
        }
    }

    fn get_error_lister_dispatch<'a>(
        &'a self,
    ) -> Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'a> {
        Box::new(ProxyErrorListener {
            delegates: self.error_listeners.borrow(),
        })
    }

    fn is_expected_token(&self, _symbol: i32) -> bool {
        unimplemented!()
    }

    fn get_precedence(&self) -> i32 {
        *self.precedence_stack.last().unwrap_or(&-1)
    }

    #[inline(always)]
    fn get_state(&self) -> i32 {
        self.state
    }

    #[inline(always)]
    fn set_state(&mut self, v: i32) {
        self.state = v;
    }

    fn get_rule_invocation_stack(&self) -> Vec<String> {
        let mut vec = Vec::new();
        let rule_names = self.get_rule_names();
        let mut ctx = self.get_current_context();
        loop {
            let rule_index = ctx.get_rule_index();
            vec.push(rule_names.get(rule_index).unwrap_or(&"n/a").to_string());
            ctx = if let Some(parent) = ctx.get_parent() {
                parent
            } else {
                break;
            }
        }
        vec
    }

    #[cfg(feature = "recursion-limit")]
    fn get_recursion_limit(&self) -> u32 {
        self.recursion_limit
    }

    #[cfg(feature = "recursion-limit")]
    fn set_recursion_limit(&mut self, v: u32) {
        self.recursion_limit = v;
    }

    //    fn get_rule_invocation_stack(&self, c: _) -> Vec<String> {
    //        unimplemented!()
    //    }
}

#[allow(missing_docs)] // todo docs
impl<'input, 'arena, Ext, Node, Input, TF> BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    pub fn new_base_parser(arena: &'arena Arena, input: Input, ext: Ext) -> Self {
        let atn_manager = ext.get_atn_simulator_man();
        let interp = ParserATNSimulator::new(atn_manager.get_simulator(arena));
        Self {
            atn_manager,
            interp: Some(interp),
            global_cache_threshold: 0,
            ctx: std::ptr::null_mut(),
            resume_node: None,
            resume_take_target: None,
            muted: 0,
            build_parse_trees: true,
            matched_eof: false,
            state: -1,
            input,
            precedence_stack: vec![0],
            #[cfg(feature = "recursion-limit")]
            recursion_limit: DEFAULT_RECURSION_LIMIT,
            #[cfg(feature = "recursion-limit")]
            current_recursion_depth: 0,
            parse_listeners: vec![],
            _syntax_errors: Cell::new(0),
            error_listeners: vec![ErrorListenerDelegate::new(
                Box::new(ConsoleErrorListener {})
                    as Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'input>,
            )],
            arena,
            ext,
            pd: PhantomData,
        }
    }

    pub fn set_global_cache_threshold(&mut self, threshold: usize) {
        self.global_cache_threshold = threshold;
    }

    /// Predict the alternative of a decision from its statically-precomputed
    /// SLL prediction DFA (generated by the ANTLR tool's `-Xstatic-dfa`
    /// option and decoded into [`crate::atn::ATN::static_dfas`] together
    /// with the ATN) instead of `adaptive_predict`. Walks the table over
    /// `LA(1..n)` without consuming input; no ATN simulation, no
    /// per-prediction allocation.
    ///
    /// Returns the predicted alternative (1-based, decision-transition
    /// numbering identical to `adaptive_predict`), or
    /// [`crate::atn::INVALID_ALT`] (0) when the table defers the prediction
    /// to the adaptive engine - a hybrid table's escape state, or a
    /// precedence class without a table. Generated call sites rerun such
    /// predictions through `adaptive_predict` (sound: the walker never
    /// consumes input). A lookahead token matching no table edge produces a
    /// `NoViableAlt` error anchored at the decision start token, matching
    /// `adaptive_predict`'s convention.
    /// Diagnostic escape tracing for `dfa_predict` (`ANTLR_DFA_TRACE=1`):
    /// logs every prediction the static tables defer to the adaptive
    /// engine, the tool for finding which decisions a given input keeps
    /// escaping on. Debug builds only - in release it is a compile-time
    /// `false` so every trace branch is eliminated from the hot walk.
    #[cfg(debug_assertions)]
    fn dfa_trace() -> bool {
        static TRACE: std::sync::LazyLock<bool> =
            std::sync::LazyLock::new(|| std::env::var_os("ANTLR_DFA_TRACE").is_some());
        *TRACE
    }

    /// Release builds: tracing is compiled out entirely (see the
    /// debug-build variant above).
    #[cfg(not(debug_assertions))]
    #[inline(always)]
    fn dfa_trace() -> bool {
        false
    }

    pub fn dfa_predict(&mut self, decision: i32) -> Result<i32, ANTLRError> {
        let mask = self.dfa_walk(decision)?;
        if mask == 0 || !mask.is_power_of_two() {
            // defer to the adaptive engine: escape state, no table, or a
            // mask-accept state this call site has no factored
            // alternative path for
            Ok(crate::atn::INVALID_ALT)
        } else {
            Ok(mask.trailing_zeros() as i32 + 1)
        }
    }

    /// Mask-returning sibling of [`Self::dfa_predict`]: walks the same
    /// static table but returns the set of live alternatives as a bitmask
    /// (bit `1<<(alt-1)` per alternative) instead of a unique
    /// alternative. A mask-accept state returns its mask: the live
    /// alternatives are covered by one prefix-factor group, so the
    /// generated parser executes the group's shared prefix and resolves
    /// the choice with its tail decision. `0` defers to the adaptive
    /// engine (same conditions as `dfa_predict` returning
    /// [`crate::atn::INVALID_ALT`]); a unique accept returns its
    /// single-bit mask.
    pub fn dfa_predict_mask(&mut self, decision: i32) -> Result<u64, ANTLRError> {
        self.dfa_walk(decision)
    }

    /// The real-stack postfix chase of the guarded descent dispatch (see
    /// [`crate::follow_sets`]): can `token` follow the decision rule on
    /// the current parse stack? Walking the context frames upward, the
    /// token follows iff it starts the caller-body continuation of some
    /// frame and every frame below completes token-free. A shared-descent
    /// group's tail dispatch commits to an explicit member on the dispatch
    /// token only when this returns false (the group's nullable member,
    /// e.g. a bare column reference, is then dead on this stack).
    pub fn follow_contains(&self, token: i32) -> bool {
        let sets = self.atn_manager.atn().follow_sets();
        let mut c = self.ctx();
        while let Some(node) = c {
            let inv = node.get_invoking_state();
            if inv < 0 {
                break;
            }
            if std::env::var_os("DBG_FOLLOW").is_some() {
                eprintln!("FOLLOW token={} inv={} contains={} nullable={}",
                    token, inv, sets.contains(inv as usize, token), sets.nullable(inv as usize));
            }
            if sets.contains(inv as usize, token) {
                return true;
            }
            if !sets.nullable(inv as usize) {
                return false;
            }
            c = node.get_parent();
        }
        token == crate::token::TOKEN_EOF
    }

    /// Set the stage for a neutral prefix parse:
    /// - Mark the input stream so it can be rewound later
    /// - Skip over prefix tokens (note: do NOT parse them through
    ///   [`Self::match_token`] at this point -- they are guaranteed to match by
    ///   the DFA predictor, and they will be properly parsed on the real
    ///   descent down)
    /// - Save the current context so it can be restored later (the neutral
    ///   parse grafts its subtree under it, and an error unwinding out of it
    ///   leaves the context pointer stale)
    /// - Mute all listeners (the neutral prefix parse of a descent
    ///   block must not fire spurious reports on inputs it cannot
    ///   complete).
    ///
    /// Pair with [`Self::end_neutral_parse`].
    #[doc(hidden)]
    #[allow(clippy::type_complexity)]
    pub fn begin_neutral_parse(
        &mut self,
        prefix_len: isize,
    ) -> (
        isize,
        isize,
        i32,
        Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
    ) {
        let err_cnt = self.syntax_error_count();
        let pos0 = self.input.index();
        let mark = self.input.mark();
        let ctx = self.ctx();
        // the DFA walk buffered only the lookahead it consumed; the
        // neutral parse resumes prefix_len tokens ahead, which can sit
        // one past the buffered range - fetch it before seeking (the
        // stream's seek panics on an unbuffered index)
        self.input.la(prefix_len + 1);
        self.input.seek(pos0 + prefix_len);
        self.muted += 1;

        (pos0, mark, err_cnt, ctx)
    }

    /// Undoes one level of [`Self::begin_neutral_parse`]: un-mutes,
    /// rewinds the stream to the decision start, restores the saved
    /// context, and un-grafts the neutral parse's subtree (the saved
    /// context's last child).
    #[doc(hidden)]
    #[allow(clippy::type_complexity)]
    pub fn end_neutral_parse(
        &mut self,
        (pos0, mark, _err_cnt, ctx): &(
            isize,
            isize,
            i32,
            Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
        ),
    ) {
        assert!(self.muted > 0);
        self.muted -= 1;
        self.input.seek(*pos0);
        self.input.release(*mark);
        self.set_current_ctx(*ctx);
        self.discard_graft();
    }

    /// Remove a grafted context from its parent's children (an errorful
    /// neutral parse's result is thrown away before deferring to the
    /// adaptive engine).
    #[doc(hidden)]
    pub fn discard_graft(&mut self) {
        if self.build_parse_trees {
            // The common rule context was parsed and attached to the current
            // ctx as the last child, pop it off:
            unsafe {
                if let Some(ctx) = self.ctx_mut() {
                    ctx.remove_last_child();
                }
            }
        }
    }

    /// The number of syntax errors reported so far; a neutral prefix
    /// parse that moves this count was errorful.
    #[doc(hidden)]
    pub fn syntax_error_count(&self) -> i32 {
        self._syntax_errors.get()
    }

    /// Begin resume mode after a shared-descent group's common rule was
    /// parsed once: un-graft the parsed node from the current context,
    /// remember it with its rule and the tail stream position, and
    /// rewind the stream to `prefix_pos` (the position saved before the
    /// common rule was called). The re-descent through the ordinary
    /// alternatives then predicts on the prefix tokens (consuming
    /// nothing, the descent being epsilon by construction) until
    /// [`Self::resume_take`] short-circuits the stored rule.
    #[doc(hidden)]
    pub fn start_resume(
        &mut self,
        node: &'arena TreeNode<'input, 'arena, Node, TF::Tok>,
        rule: usize,
    ) {
        let tail_pos = self.input.index();

        self.resume_node = Some((node, rule, tail_pos));
    }

    /// Is resume mode active (a nested descent block must defer to the
    /// adaptive engine rather than start another resume)?
    #[doc(hidden)]
    pub fn resume_active(&self) -> bool {
        self.resume_node.is_some()
    }

    /// Take the stored prefix context when `rule` is called in resume
    /// mode: graft it under the current context, restore the stream to
    /// the tail position, and end resume mode. Returns None when
    /// inactive or the rule does not match.
    #[doc(hidden)]
    pub fn resume_take(
        &mut self,
        rule: usize,
    ) -> Result<Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>, ANTLRError> {
        if self.resume_node.map(|(_, r, _)| r) != Some(rule) {
            return Ok(None);
        }
        let (node, _, tail_pos) = self.resume_node.take().unwrap();

        // Graft the stored node under the current context (the neutral parse
        // created it with a generic invoking state and a wrong parent)
        if self.build_parse_trees {
            self.add_child_to_ctx(node);
        }
        let state = self.get_state();
        let parent = self.take_ctx();
        self.set_current_ctx(Some(node));
        self.with_mut_ctx(|ctx| {
            ctx.set_parent(parent);
            ctx.set_invoking_state(state);
        });
        // Fire parser events that was muted during the neutral parse
        if !self.is_muted() {
            self.parse_listeners = std::mem::take(&mut self.parse_listeners)
                .into_iter()
                .map(|listener| ParseTreeWalker::walk(listener, node))
                .collect::<Result<Vec<_>, ANTLRError>>()?;
        }

        self.set_current_ctx(parent);

        self.input.seek(tail_pos);
        Ok(Some(node))
    }

    /// Abort resume mode on error unwind: restore the stream to the
    /// tail position and clear the state.
    #[doc(hidden)]
    pub fn abort_resume(&mut self) {
        if let Some((_, _, tail_pos)) = self.resume_node.take() {
            self.input.seek(tail_pos);
        }
    }

    /// The shared static-table walk: the prediction of `decision` as an
    /// alternative mask, or `0` when the table defers to the adaptive
    /// engine.
    fn dfa_walk(&mut self, decision: i32) -> Result<u64, ANTLRError> {
        // the precedence selects the table of dispatched decisions (the
        // operator loops of left-recursive rules); plain decisions ignore it
        let precedence = self.get_precedence();
        let static_dfas = &self.atn_manager.atn().static_dfas;
        if static_dfas.is_precedence_dispatched(decision) && self.parent_ctx().is_none() {
            // The left-recursive rule is itself the parse entry (no caller
            // frame): per-precedence-class tables are built against the
            // rule's compatible call sites, but an entry invocation pops
            // into the grammar-wide FOLLOW space instead - only the
            // adaptive engine models that.
            return Ok(0);
        }
        let Some(dfa) = static_dfas.table(decision, precedence) else {
            // this precedence class has no static table
            if Self::dfa_trace() {
                eprintln!("DFA-TRACE no-table d={} prec={}", decision, precedence);
            }
            return Ok(0);
        };
        let mut s = 0usize;
        let mut i = 1isize;
        loop {
            // One `accepts` load serves both the escape and accept checks
            // (previously loaded twice, via is_escape() then accept()).
            let alt = dfa.accepts[s];
            if alt == crate::static_dfa::ESCAPE {
                // hybrid-table escape: the caller reruns the prediction
                // through the adaptive engine (no input was consumed, so
                // the rescan starts clean)
                if Self::dfa_trace() {
                    eprintln!(
                        "DFA-TRACE escape d={} prec={} state={} depth={} la1={}",
                        decision,
                        precedence,
                        s,
                        i - 1,
                        self.input.la(1)
                    );
                }
                return Ok(0);
            }
            if alt == crate::static_dfa::GUARDED {
                // guarded take of an optional-postfix decision: resolve
                // to take unless the real parse stack's epsilon-pop
                // chase from the decision's block end can reach a guard
                // root (an invoking state in the table's danger set);
                // the walk stops at the first invoking state whose
                // follow region cannot reach its own rule's stop.
                let mut defers = false;
                let mut c = self.ctx();
                while let Some(node) = c {
                    let inv = node.get_invoking_state();
                    if inv < 0 {
                        break;
                    }
                    match dfa.guard_step(inv) {
                        Some(true) => {
                            defers = true;
                            break;
                        }
                        Some(false) => break,
                        None => c = node.get_parent(),
                    }
                }
                if !defers {
                    if Self::dfa_trace() {
                        eprintln!(
                            "DFA-TRACE guard-take d={} prec={} state={} depth={} la1={}",
                            decision,
                            precedence,
                            s,
                            i - 1,
                            self.input.la(1)
                        );
                    }
                    return Ok(1u64 << (dfa.guarded_alt(s).unwrap() - 1));
                }
                if Self::dfa_trace() {
                    eprintln!(
                        "DFA-TRACE guard-defer d={} prec={} state={} depth={} la1={}",
                        decision,
                        precedence,
                        s,
                        i - 1,
                        self.input.la(1)
                    );
                }
                return Ok(0);
            }
            if let Some(mask) = dfa.accept_mask(s) {
                // mask-accept: the live alternatives are covered by one
                // prefix-factor group
                if Self::dfa_trace() {
                    eprintln!(
                        "DFA-TRACE mask d={} prec={} state={} depth={} la1={} mask={:#x}",
                        decision,
                        precedence,
                        s,
                        i - 1,
                        self.input.la(1),
                        mask
                    );
                }
                return Ok(mask);
            }
            if alt > 0 {
                if alt > 64 {
                    // the mask encoding cannot represent alternatives
                    // above 64: the tool never emits tables for such
                    // decisions, but a stale table defers rather than
                    // overflowing the shift
                    return Ok(0);
                }
                return Ok(1u64 << (alt - 1));
            }
            let t = self.input.la(i);
            match dfa.edge(s, t) {
                Some(next) => {
                    s = next;
                    i += 1;
                }
                None => {
                    // No viable transition. If some alternative already
                    // finished the decision entry rule, predict it and let
                    // the parser report a more precise error at the actual
                    // mismatch point - mirroring adaptive_predict's
                    // finished-decision-entry-rule recovery.
                    if let Some(alt) = dfa.fallback(s) {
                        if alt > 64 {
                            // see the accept path above: stale-table
                            // defense, never emitted by the tool
                            return Ok(0);
                        }
                        return Ok(1u64 << (alt - 1));
                    }
                    let start = self.input.lt(1).map(|t| OwningToken::from(t as &dyn Token));
                    let offending = self.input.lt(i).map(|t| OwningToken::from(t as &dyn Token));
                    return Err(match (start, offending) {
                        (Some(start), Some(offending)) => {
                            ANTLRError::no_alt_full(self, start, offending)
                        }
                        _ => ANTLRError::no_alt(self),
                    });
                }
            }
        }
    }

    pub fn get_interpreter_mut(&mut self) -> &mut ParserATNSimulator<'arena> {
        self.interp.as_mut().unwrap()
    }

    /// If current context is same as the given one by pointer comparison
    #[inline]
    pub fn ctx_is(&self, other: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>) -> bool {
        if self.ctx.is_null() && other.is_none() {
            true
        } else if self.ctx.is_null() || other.is_none() {
            false
        } else {
            std::ptr::eq(self.ctx, other.unwrap() as *const _ as *const ())
        }
    }

    /// Gets a reference to current context.
    #[inline]
    pub fn ctx(&self) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        if self.ctx.is_null() {
            None
        } else {
            unsafe { Some(&*(self.ctx as *const TreeNode<'input, 'arena, Node, TF::Tok>)) }
        }
    }

    /// Gets a mutable reference to current context.
    ///
    /// # Safety
    /// Follows the same safety rules as dereferencing *mut to &mut
    #[inline]
    pub unsafe fn ctx_mut(
        &mut self,
    ) -> Option<&'arena mut TreeNode<'input, 'arena, Node, TF::Tok>> {
        if self.ctx.is_null() {
            None
        } else {
            Some(&mut *(self.ctx as *mut TreeNode<'input, 'arena, Node, TF::Tok>))
        }
    }

    #[inline]
    fn is_muted(&self) -> bool {
        self.muted > 0
    }

    #[inline]
    fn parent_ctx(&self) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        self.ctx().and_then(|it| it.get_parent())
    }

    #[inline]
    fn set_current_ctx(&mut self, ctx: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>) {
        if let Some(ctx) = ctx {
            self.ctx = ctx as *const TreeNode<'input, 'arena, Node, TF::Tok>
                as *mut TreeNode<'input, 'arena, Node, TF::Tok> as *mut ();
        } else {
            self.ctx = std::ptr::null_mut();
        }
    }

    pub fn take_ctx(&mut self) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        if self.ctx.is_null() {
            None
        } else {
            let ret = unsafe { &*(self.ctx as *const TreeNode<'input, 'arena, Node, TF::Tok>) };
            self.ctx = std::ptr::null_mut();
            Some(ret)
        }
    }

    #[inline]
    fn add_child_to_ctx(&mut self, child: &'arena TreeNode<'input, 'arena, Node, TF::Tok>) {
        if !self.ctx.is_null() {
            unsafe {
                (*(self.ctx as *mut TreeNode<'input, 'arena, Node, TF::Tok>)).add_child(child);
            }
        }
    }

    #[inline]
    pub fn with_mut_ctx<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut TreeNode<'input, 'arena, Node, TF::Tok>) -> R,
    {
        assert!(!self.ctx.is_null());
        unsafe { f(&mut *(self.ctx as *mut TreeNode<'input, 'arena, Node, TF::Tok>)) }
    }

    #[inline]
    pub fn match_token(
        &mut self,
        ttype: i32,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<&'arena TF::Tok, ANTLRError> {
        let mut token = self.get_current_token();
        if token.get_token_type() == ttype {
            if ttype == TOKEN_EOF {
                self.matched_eof = true;
            }

            err_handler.report_match(self);
            self.consume(err_handler)?;
        } else {
            token = err_handler.recover_inline(self)?;
            if self.build_parse_trees && token.get_token_index() == -1 {
                self.add_child_to_ctx(self.create_error_node(token)?);
            }
        }
        Ok(token)
    }

    #[inline]
    pub fn match_wildcard(
        &mut self,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<&'arena TF::Tok, ANTLRError> {
        let mut token = self.get_current_token();
        if token.get_token_type() > 0 {
            err_handler.report_match(self);
            self.consume(err_handler)?;
        } else {
            token = err_handler.recover_inline(self)?;
            if self.build_parse_trees && token.get_token_index() == -1 {
                self.add_child_to_ctx(self.create_error_node(token)?);
            }
        }
        Ok(token)
    }

    /// Adds parse listener for this parser
    /// returns `listener_id` that can be used later to get listener back
    ///
    /// ### Example for listener usage:
    /// todo
    pub fn add_dyn_parse_listener(&mut self, listener: Box<Node::Listener>) {
        self.parse_listeners.push(listener);
    }

    /// Removes parse listener with corresponding `listener_id`, casts it back to user type and returns it to the caller.
    /// `listener_id` is returned when listener is added via `add_parse_listener`
    pub fn remove_parse_listener<L>(&mut self, listener_id: ListenerId<L>) -> Box<L>
    where
        L: ParseTreeListener<'arena, Node, TF::Tok>,
    {
        let index = self
            .parse_listeners
            .iter()
            .position(|it| ListenerId::new(it).actual_id == listener_id.actual_id)
            .expect("listener not found");
        unsafe { listener_id.into_listener(self.parse_listeners.remove(index)) }
    }

    /// Removes all added parse listeners without returning them
    pub fn remove_parse_listeners(&mut self) {
        self.parse_listeners.clear()
    }

    pub fn trigger_enter_rule_event(&mut self) -> Result<(), ANTLRError> {
        if !self.is_muted() {
            let ctx = self.ctx().unwrap();
            for listener in self.parse_listeners.iter_mut() {
                listener.enter_every_rule(ctx)?;
                ctx.enter_rule(listener)?;
            }
        }
        Ok(())
    }

    pub fn trigger_exit_rule_event(&mut self) -> Result<(), ANTLRError> {
        if !self.is_muted() {
            let ctx = self.ctx().unwrap();
            for listener in self.parse_listeners.iter_mut().rev() {
                ctx.exit_rule(listener)?;
                listener.exit_every_rule(ctx)?;
            }
        }
        Ok(())
    }

    //    fn get_atn_with_bypass_alts(&self) { unimplemented!() }
    //
    //    fn compile_parse_tree_pattern(&self, pattern, patternRuleIndex: Lexer, lexer: Lexer) { unimplemented!() }

    #[inline]
    pub fn enter_rule(
        &mut self,
        localctx: &'arena mut TreeNode<'input, 'arena, Node, TF::Tok>,
        state: i32,
        _rule_index: usize,
    ) -> Result<(), ANTLRError> {
        let child = localctx;
        if self.build_parse_trees {
            self.set_current_ctx(child.get_parent());
            self.add_child_to_ctx(child);
        }

        self.set_state(state);
        self.set_current_ctx(Some(child));
        let start = self.input.lt(1);
        self.with_mut_ctx(|ctx| {
            ctx.set_start(start);
        });

        if !self.parse_listeners.is_empty() {
            self.trigger_enter_rule_event()?;
        }

        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth += 1;
            if self.current_recursion_depth > self.recursion_limit {
                return Err(ANTLRError::recursion_limit_exceeded(self.recursion_limit));
            }
        }
        Ok(())
    }

    #[inline]
    pub fn exit_rule(
        &mut self,
    ) -> Result<&'arena TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        assert!(self.ctx().is_some());

        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth -= 1;
        }
        if self.matched_eof {
            // if we have matched EOF, it cannot consume past EOF so we use LT(1) here
            let stop = self.input.lt(1);
            self.with_mut_ctx(|ctx| {
                ctx.set_stop(stop);
            });
        } else {
            // stop node is what we just matched
            let stop = self.input.lt(-1);
            self.with_mut_ctx(|ctx| {
                ctx.set_stop(stop);
            });
        }
        if !self.parse_listeners.is_empty() {
            self.trigger_exit_rule_event()?;
        }

        self.set_state(self.ctx().unwrap().get_invoking_state());
        let child = self.ctx().unwrap();
        self.set_current_ctx(child.get_parent());

        Ok(child)
    }

    pub fn enter_recursion_rule(
        &mut self,
        localctx: &'arena mut TreeNode<'input, 'arena, Node, TF::Tok>,
        state: i32,
        _rule_index: usize,
        precedence: i32,
    ) -> Result<(), ANTLRError> {
        self.set_state(state);
        self.precedence_stack.push(precedence);
        self.set_current_ctx(Some(localctx));
        let start = self.input.lt(1);
        self.with_mut_ctx(|ctx| {
            ctx.set_start(start);
        });
        if !self.parse_listeners.is_empty() {
            self.trigger_enter_rule_event()?;
        }
        //println!("{}",self.input.lt(1).map(Token::to_owned).unwrap());

        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth += 1;
            if self.current_recursion_depth > self.recursion_limit {
                return Err(ANTLRError::recursion_limit_exceeded(self.recursion_limit));
            }
        }
        Ok(())
    }

    pub fn push_new_recursion_context(
        &mut self,
        localctx: &'arena mut TreeNode<'input, 'arena, Node, TF::Tok>,
        state: i32,
        _rule_index: usize,
    ) -> Result<&'arena TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        let stop = self.input.lt(-1);
        self.with_mut_ctx(|ctx| {
            ctx.set_parent(Some(localctx));
            ctx.set_invoking_state(state);
            ctx.set_stop(stop);
        });

        let prev = self.take_ctx().unwrap();
        self.set_current_ctx(Some(localctx));
        let start = prev.get_start_token();
        self.with_mut_ctx(|ctx| {
            ctx.set_start(start);
        });
        if self.build_parse_trees {
            self.add_child_to_ctx(prev);
        }
        if !self.parse_listeners.is_empty() {
            self.trigger_enter_rule_event()?;
        }
        Ok(prev)
    }

    pub fn unroll_recursion_context(
        &mut self,
        parent_ctx: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
    ) -> Result<&'arena TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        assert!(self.ctx().is_some());

        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth -= 1;
        }
        self.precedence_stack.pop();
        let stop = self.input.lt(-1);
        self.with_mut_ctx(|ctx| {
            ctx.set_stop(stop);
        });
        let retctx = self.ctx;

        // unroll so _ctx is as it was before call to recursive method
        if !self.parse_listeners.is_empty() {
            while !self.ctx_is(parent_ctx) {
                self.trigger_exit_rule_event()?;
                self.set_current_ctx(self.parent_ctx());
            }
        } else {
            self.set_current_ctx(parent_ctx);
        }

        // hook into tree
        unsafe {
            (*(retctx as *mut TreeNode<'input, 'arena, Node, TF::Tok>)).set_parent(parent_ctx);
        }

        //        println!("{:?}",self.ctx.as_ref().map(|it|it.to_string_tree(self)));
        if self.build_parse_trees && parent_ctx.is_some() {
            self.add_child_to_ctx(unsafe {
                &*(retctx as *const TreeNode<'input, 'arena, Node, TF::Tok>)
            });
        }
        Ok(unsafe { &*(retctx as *const TreeNode<'input, 'arena, Node, TF::Tok>) })
    }

    #[allow(clippy::mut_from_ref)] // &mut is from the arena allocation
    fn create_token_node(
        &self,
        token: &'arena TF::Tok,
    ) -> Result<&'arena mut TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        let ptr = TreeNode::create_token_node(self.arena, token);
        if ptr.is_null() {
            #[cfg(feature = "arena-allocation-limit")]
            {
                Err(ANTLRError::arena_allocation_limit_exceeded(
                    self.arena.allocation_limit_bytes(),
                    self.arena.total_allocated_bytes(),
                ))
            }
            #[cfg(not(feature = "arena-allocation-limit"))]
            {
                std::alloc::handle_alloc_error(std::alloc::Layout::new::<
                    TreeNode<'input, 'arena, Node, TF::Tok>,
                >())
            }
        } else {
            Ok(unsafe { &mut *ptr })
        }
    }

    #[allow(clippy::mut_from_ref)] // &mut is from the arena allocation
    fn create_error_node(
        &self,
        token: &'arena TF::Tok,
    ) -> Result<&'arena mut TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        let ptr = TreeNode::create_error_node(self.arena, token);
        if ptr.is_null() {
            Err(ANTLRError::dfa_cache_limit_exceeded(0, 0, 0))
        } else {
            Ok(unsafe { &mut *ptr })
        }
    }

    /// Text representation of generated DFA for debugging purposes
    pub fn dump_dfa(&self) {
        let mut seen_one = false;
        for i in 0..self.get_atn().decision_to_state.len() {
            let dfa = self
                .get_interpreter()
                .decision_to_dfa(i)
                .expect("dfa should exist for each decision");
            if !dfa.is_empty() {
                if seen_one {
                    println!()
                }
                println!("Decision {}:", dfa.decision);
                print!("{}", dfa.to_string(self.get_vocabulary()));
                seen_one = true;
            }
        }
    }
}

impl<'input, 'arena, Ext, Node, Input, TF> Drop for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    fn drop(&mut self) {
        if self.global_cache_threshold == 0 {
            return;
        }

        let Some(interp) = self.interp.take() else {
            return;
        };
        let cache_bytes = interp.total_allocated_bytes();
        drop(interp);

        if cache_bytes > self.global_cache_threshold {
            self.atn_manager.reset_all();
        }
    }
}

/// Allows to safely cast listener back to user type
#[derive(Debug)]
pub struct ListenerId<T: ?Sized> {
    pub(crate) actual_id: usize,
    phantom: PhantomData<fn() -> T>,
}

impl<T: ?Sized> ListenerId<T> {
    #[allow(clippy::borrowed_box)]
    pub fn new(listener: &Box<T>) -> ListenerId<T> {
        ListenerId {
            actual_id: listener.as_ref() as *const T as *const () as usize,
            phantom: Default::default(),
        }
    }
}

impl<T> ListenerId<T> {
    unsafe fn into_listener<U: ?Sized>(self, boxed: Box<U>) -> Box<T> {
        Box::from_raw(Box::into_raw(boxed) as *mut T)
    }
}
