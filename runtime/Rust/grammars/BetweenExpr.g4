// Differential-test grammar isolating the BETWEEN ... AND ... collision
// with the boolean AND/OR loop, in the exact shape the dbt SQL grammars
// carry it (see tests/between_differential_tests.rs):
//
//   booleanExpression : valueExpression predicate? | NOT b | b AND b | b OR b
//   predicate         : NOT? BETWEEN valueExpression AND valueExpression | ...
//
// BETWEEN is a *suffix* predicate on the pure-arithmetic value ladder, so
// its 'AND' separator collides with the boolean loop's 'AND' operator:
// after `a BETWEEN b AND c`, the boolean loop sees AND and must decide
// iterate (a new boolean operand) vs exit (the AND belongs to an enclosing
// construct). Statically provable shapes of this conflict resolve in the
// class tables of the b-loop decision; the rest escape to adaptivePredict.
//
// The grammar also carries 'NOT'? BETWEEN, whose lookahead collides with
// the prefix 'NOT' b alternative, and '(' b ')' in atoms, which puts
// boolean frames inside BETWEEN bounds (the phantom-FOLLOW mixing ground).
//
// The lambda production (ID '->' b) is what makes the collision genuinely
// context-sensitive - it puts a booleanExpression directly in a value
// position, so a BETWEEN bound can end with a lambda whose body absorbs
// 'AND' tokens: in `x BETWEEN y -> a AND b AND c` the first AND belongs
// to the lambda body and the second is the separator. Which AND is the
// separator depends on the whole rest of the input, so the b-loop and the
// predicate decision become hybrid tables that escape to adaptivePredict
// on exactly these inputs (verified: both engines parse the corners below
// identically, resolving maximal-munch like the runtime). DELETE the
// lambda production and every decision in this grammar is statically
// k=1 - function calls, IN lists, subscripts and LIKE..ESCAPE suffixes
// do NOT trigger the escape; only a value-position recursion back into
// the boolean rule does. This grammar is the minimal witness of the
// production (Trino) escape pattern, where lambdas play that role.
grammar BetweenExpr;

s : b EOF ;

b : v pred?              # predicated
  | 'NOT' b              # not
  | b 'AND' b            # and
  | b 'OR' b             # or
  ;

pred : 'NOT'? 'BETWEEN' v 'AND' v    # between
     | ('='|'<'|'>') v               # cmp
     ;

v : v ('*'|'/') v
  | v ('+'|'-') v
  | atom
  ;

atom : ID | INT | '(' b ')' | ID '->' b ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\n]+ -> skip ;
