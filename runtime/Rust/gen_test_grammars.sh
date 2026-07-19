#!/usr/bin/env bash
set -euo pipefail

TOOL_VERSION="2.0.0"
ANTLR_PATH="../../../tool/target/dbt-antlr4-${TOOL_VERSION}-complete.jar"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

declare -a GRAMMARS=(
    "VisitorBasic"
    "VisitorCalc"
    "CSV"
    "ReferenceToATN"
    "XMLLexer"
    "SimpleLR"
    "Labels"
    "StaticDFA"
    "LrDfa"
    "FuzzExpr"
    "BetweenExpr"
    "ParenExpr"
#    "FHIRPath"
)

declare -a ADDITIONAL_ARGS=(
    "-visitor"
    "-visitor"
    "-visitor"
    ""
    ""
    ""
    ""
    "-Xstatic-dfa"
    "-Xstatic-dfa"
    "-Xstatic-dfa"
    "-Xstatic-dfa"
    "-Xstatic-dfa"
)

for i in "${!GRAMMARS[@]}"; do
    grammar="${GRAMMARS[$i]}"
    arg="${ADDITIONAL_ARGS[$i]}"
    file_name="${grammar}.g4"

    cmd=(java -cp "$ANTLR_PATH" org.antlr.v4.Tool -Dlanguage=Rust -o ../tests/gen "$file_name")
    if [[ -n "$arg" ]]; then
        cmd+=("$arg")
    fi

    echo "Generating: $grammar"
    (cd "$SCRIPT_DIR/grammars" && "${cmd[@]}")
done

# The differential-fuzz twin: FuzzExpr generated WITHOUT -Xstatic-dfa into a
# sibling module, so tests can compare table-driven and adaptive prediction
# on the same grammar (tests/fuzz_differential_tests.rs).
echo "Generating: FuzzExpr (adaptive twin)"
(cd "$SCRIPT_DIR/grammars" && java -cp "$ANTLR_PATH" org.antlr.v4.Tool -Dlanguage=Rust \
    -o ../tests/gen/adaptive FuzzExpr.g4)

# Same differential twin for the BETWEEN-collision grammar
# (tests/between_differential_tests.rs).
echo "Generating: BetweenExpr (adaptive twin)"
(cd "$SCRIPT_DIR/grammars" && java -cp "$ANTLR_PATH" org.antlr.v4.Tool -Dlanguage=Rust \
    -o ../tests/gen/adaptive BetweenExpr.g4)

# Same differential twin for the paren-family alt-mask grammar
# (tests/paren_differential_tests.rs).
echo "Generating: ParenExpr (adaptive twin)"
(cd "$SCRIPT_DIR/grammars" && java -cp "$ANTLR_PATH" org.antlr.v4.Tool -Dlanguage=Rust \
    -o ../tests/gen/adaptive ParenExpr.g4)
