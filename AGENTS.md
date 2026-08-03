# AGENTS.md

## What this repo is

This is **sdf-labs/antlr4** (`origin`), a **fork of antlr/antlr4**
(`upstream`) maintained for
[dbt-fusion](https://github.com/dbt-labs/dbt-core). The fork's reason for
existing is the **Rust target**: the Rust runtime crate (`runtime/Rust`,
published as `dbt-antlr4`) and its tool codegen. Other runtime targets
(Java, C++, Go, etc.) are inherited and kept here for reference only —
nearly all fork work happens in `runtime/Rust` and the tool's Rust codegen.

Two version numbers, kept in lockstep since 2.0.0 but set in different places:
- Tool/Java/Maven version: `pom.xml` `2.0.3` (the shaded tool jar is named
  `dbt-antlr4-<version>-complete.jar` via `shadedArtifactId` in `tool/pom.xml`;
  the Maven coordinate remains `org.antlr:antlr4`)
- Rust runtime crate version: `runtime/Rust/Cargo.toml` (`dbt-antlr4`, `2.0.3`)

### Upstream feature compatibility

- Version 1.x.y maintains full compliance with upstream Antlr features
- Since version 2, we no longer maintain strict adherence to upstream Antlr.
  Most notably, 2.0.0 introduced `-Xstatic-dfa` option (Rust runtime only),
  which enables additional static analysis at codegen time to generate
  optimized code paths that avoids the overhead a full runtime
  `adaptivePredict`

## Build (Maven, builds the tool that generates parsers)

ANTLR is written in itself, so the build has a self-dependency. Use `install`, not `compile`:

```bash
export MAVEN_OPTS="-Xmx1G"     # required on Linux
mvn install -DskipTests        # produces tool/target/dbt-antlr4-<version>-complete.jar
```

Maven modules (`pom.xml`): `runtime/Java`, `tool`, `antlr4-maven-plugin`, `tool-testsuite`,
`runtime-testsuite`. JDK 21, Maven 3.8+.

## Rust target: the important workflow

The Rust runtime is a Cargo crate, **not** part of the Maven build. To work on it:

```bash
cd runtime/Rust
cargo build
cargo test --all-features       # runs tests against the checked-in generated parsers in tests/gen/
```

Cargo features (off by default): `recursion-limit`, `arena-allocation-limit` — these gate
fork-specific safety limits added in recent commits. The `stacker` dep is optional.

### Regenerating the Rust test parsers (critical, easy to get wrong)

`runtime/Rust/tests/gen/` contains **checked-in generated parsers**. If you change the Rust
codegen template you must regenerate them, and this requires the freshly-built tool jar:

```bash
# 1. build the tool jar first (from repo root):
mvn install -DskipTests
# 2. regenerate (script hardcodes TOOL_VERSION — update it if pom.xml version changed):
cd runtime/Rust && ./gen_test_grammars.sh
```

`gen_test_grammars.sh` has `TOOL_VERSION="2.0.3"` hardcoded and reads
`tool/target/dbt-antlr4-${TOOL_VERSION}-complete.jar`. Keep it in sync with `pom.xml`.

### Single source of truth for the Rust codegen template

`runtime/Rust/templates/Rust.stg` is a **symlink** to
`tool/resources/org/antlr/v4/tool/templates/codegen/Rust/Rust.stg`. Edit only the real file
under `tool/resources/...`; do not replace the symlink with a copy.

Rust tool target Java class: `tool/src/org/antlr/v4/codegen/target/RustTarget.java`.

## Runtime test suite (cross-target, JVM-driven)

`runtime-testsuite` tests every target by launching processes to compile/run generated code.
Tests are defined as text descriptor files (not Java), see
`doc/antlr-project-testing.md`. Run a single target from the `runtime-testsuite` dir:

```bash
cd runtime-testsuite
export MAVEN_OPTS="-Xmx1G"
mvn -Dtest='rust.**' test       # or java.**, cpp.**, go.** ...
```

Rust runner lives at `runtime-testsuite/test/org/antlr/v4/test/runtime/rust/` and shells out
to `cargo`. Note: Rust is wired into `runtime-testsuite` but is **not** in the CI matrix in
`.github/workflows/hosted.yml` — CI does not test Rust, so verify Rust changes locally.

Two Rust suites exist: `rust.RustRuntimeTests` (adaptive prediction, all descriptors) and
`rust.RustStaticDFARuntimeTests` (same corpus generated with `-Xstatic-dfa`; a differential
test of table-driven static prediction — skips `showDiagnosticErrors`/`traceATN` descriptors,
which introspect the adaptive engine that static tables bypass).

Tool-only unit tests: `cd tool-testsuite && mvn test`.

## Conventions

- Commits require DCO sign-off: `git commit -s` (lowercase `-s`). See `CONTRIBUTING.md`.
- `.editorconfig` defines the Java style; `runtime/Rust/rustfmt.toml` (edition 2021) the Rust style.
- Contributor conduct rules: `ANTLR-HOUSE-RULES.md`.
