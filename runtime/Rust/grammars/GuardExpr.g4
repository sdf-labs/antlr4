// Differential-test grammar isolating the guarded-take escape of the
// optional-postfix take rule, in the exact shape the dbt SQL grammars
// carry it (Bigquery's `UPDATE SET targets+=expression EQ
// values+=expression` vs `expression: valueExpression predicate?`):
//
//   expr : value pred?            # the X Y? postfix decision
//   stmt : 'set' expr '=' expr    # a caller where skip genuinely wins
//        | expr                   # callers where take genuinely wins
//
// After `set a`, the pred? decision sees '=': take reads `a = b` as a
// comparison (then the assignment's own '=' is missing - dead end), skip
// closes the target expression and lets the assignment's '=' through.
// Which reading survives depends on the CALL STACK (inside an assignment
// target vs everywhere else), so the decision's take/skip conflict is
// genuinely context-sensitive: the static table resolves it to take
// guarded by a stack test that defers to adaptivePredict exactly when an
// invoking state of the assignment's `expr` call is present (the
// epsilon-pop chase from the decision's block end reaches the
// assignment's '=' position). Everywhere else take resolves statically.
//
// The chase must STOP at constructs that seal the expression off from
// the assignment: parentheses ('(' expr ')'), the IN list, and the right
// operand of a boolean operator whose parent chain never enters an
// assignment - inside those, `a = b` is a comparison even when the whole
// thing sits in an assignment value (`set x = (a = b)` is an assignment
// of a comparison, not a nested assignment).
grammar GuardExpr;

prog : stmt+ EOF ;

stmt : 'set' expr '=' expr ';'     # assign
     | expr ';'                    # eval
     ;

expr : value pred?                 # predicated
     | 'NOT' expr                  # not
     | expr 'AND' expr             # and
     | expr 'OR' expr              # or
     ;

pred : ('='|'<'|'>') value         # cmp
     | 'IN' '(' expr ')'           # inList
     ;

value : ID | INT | '(' expr ')' ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\n]+ -> skip ;
