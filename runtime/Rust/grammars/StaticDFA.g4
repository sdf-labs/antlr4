/*
 * Test grammar for -Xstatic-dfa: every non-LL(1) decision below is
 * statically decidable, so the generated parser must contain no
 * adaptive_predict call sites (verified by tests/static_dfa_tests.rs).
 */
grammar StaticDFA;

// LL(2)/LL(3) alt block: keyword-pair alternatives
stat : 'create' 'table' ID     #createTable
     | 'create' 'view' ID      #createView
     | 'drop' 'table' ID       #dropTable
     ;

// LL(*): cyclic DFA scans the dotted name; '(' vs anything-else decides
expr : name '(' ')'            #call
     | name                    #ref
     ;
name : ID ('.' ID)* ;

// non-LL(1) loop enter/exit decisions (k=2)
starLoop : ('a' 'b')* 'a' 'c' ;
plusLoop : ('a' 'b')+ 'a' 'c' ;

// non-LL(1) optional (k=2)
opt : ('a' 'b')? 'a' 'c' ;

// exact ambiguity: statically resolved to the minimum alternative,
// identical to runtime SLL/LL behavior
dup : 'd' 'e'                  #dupFirst
    | 'd' 'e'                  #dupSecond
    ;

ID : [a-z]+ ;
WS : [ \t\r\n]+ -> skip ;
