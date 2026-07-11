grammar LrDfa;

s : e EOF ;

e : e '[' e ']'          # Index
  | e '!'                # Fact
  | '-' e                # Neg
  | <assoc=right> e '^' e # Pow
  | e ('*'|'/') e        # Mul
  | e ('+'|'-') e        # Add
  | <assoc=right> e '?' e ':' e # Ternary
  | ID                   # Id
  | INT                  # Int
  ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
