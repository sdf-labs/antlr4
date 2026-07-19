// Differential-test grammar for alt-mask prediction ("implied
// left-factoring", see PrefixFactorAnalyzer and FactoredAltBlock):
// the primary block of a left-recursive expression rule with a
// parenthesized family
//
//   '(' expr (',' expr)+ ')'   (row constructor)
//   '(' expr ')'               (plain parens)
//   '(' query ')'              (subquery)
//
// The row-constructor and paren alternatives share the unbounded prefix
// '(' expr, so the primary decision cannot be statically resolved to a
// unique alternative: distinguishing them needs the token after the
// full expression (',' vs ')'). The subquery alternative keeps pace
// whenever the content can start a query (identifiers - a parenthesized
// table name is a valid query). With -Xstatic-dfa, the tool proves the
// group ('(' expr shared, tails ',' vs ')' LL(1)-disjoint) and the
// table accepts with the alt mask {rowConstructor, parens} once the
// subquery reading dies; the generated factored arm executes '(' expr
// once and resolves the choice with a k=1 tail switch. Deep nesting
// ('((((((x)))))') keeps the subquery reading alive arbitrarily long;
// those paths still escape to adaptivePredict, and both engines must
// agree on every corner (tests/paren_differential_tests.rs).
grammar ParenExpr;

s : e EOF ;

e : INT                                    # num
  | ID                                     # col
  | '(' e (',' e)+ ')'                     # rowConstructor
  | '(' e ')'                              # parens
  | '(' q ')'                              # subquery
  | '-' e                                  # unary
  | e ('*'|'/') e                          # mul
  | e ('+'|'-') e                          # add
  ;

q : 'SELECT' ID                            # select
  | ID                                     # table
  | '(' q ')'                              # nested
  ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\n]+ -> skip ;
