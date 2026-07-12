// Differential-fuzz grammar for -Xstatic-dfa (see tests/fuzz_differential_tests.rs).
// Concentrates the decision shapes that exercise hybrid tables and
// per-precedence dispatch:
//   - a full precedence ladder (left/right-associative binary operators,
//     prefix, postfix, indexing, calls, parens);
//   - selectItem-shaped decisions ('e AS ID | e') that must scan past an
//     unbounded expression - the classic hybrid-table candidate;
//   - a soft keyword ('as' is also a valid identifier via `name`), so the
//     alias decision has genuinely context-sensitive corners that resolve
//     through the escape path at runtime.
grammar FuzzExpr;

s : item (',' item)* ';'? EOF ;

item
  : e 'as' name
  | e
  ;

name : ID | 'as' ;

q : name ('.' name)* ;

e : e '[' e ']'
  | '-' e
  | <assoc=right> e '^' e
  | e ('*'|'/') e
  | e ('+'|'-') e
  | atom
  ;

atom
  : q
  | INT
  | '(' e ')'
  | ID '(' (e (',' e)*)? ')'
  ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
