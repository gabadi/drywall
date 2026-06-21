# AGENTS.md

## Tool name mappings

Prompts use generic names; use the installed binaries:

- `ir-dry-checker` → `gherkin-ir-dry-checker`

`gherkin-ir-dry-checker` takes JSON IR from `gherkin-parser`, not the `.feature`:

```
gherkin-parser <feature> <ir.json>
gherkin-ir-dry-checker [--include-exact] <ir.json> <report>
```

`findings: []` = already normalized. Also installed: `gherkin-parser`, `gherkin-mutator`.

## Command index

| Purpose | Command |
|---------|---------|
| Install tools (once per machine) | `mise install` |
| Format | `cargo fmt` |
| Lint | `cargo clippy -- -D warnings` |
| Unit tests | `cargo nextest run --profile unit` |
| Property tests | `cargo nextest run --profile property` |
| Acceptance tests | `cargo nextest run --profile acceptance` |
| Language mutation | `cargo mutants --test-tool nextest -j 8 --no-shuffle -- --profile mutation` |
| Gherkin mutation | `gherkin-mutator --level soft --feature <feature>` |
| Coverage (≥90% lines) | `mise exec -- cargo llvm-cov nextest --profile unit --lcov --output-path lcov.info --fail-under-lines 90` |
| CRAP (threshold ≤6) | `cargo crap --lcov lcov.info --exclude 'acceptance/**' --exclude 'src/main.rs' --threshold 6 --fail-above` |
| Build release binary | `cargo build --release` |
| DRY self-check | `./target/release/drywall ./src` |

## Mutation runs

**Always launch mutation commands with `run_in_background: true`.** Do not block waiting for mutation output — it takes several minutes. Check results when the background notification arrives.

## Tooling notes

- **gherkin-ir-dry-checker output**: use `rtk json <report>` to read the JSON report compactly rather than dumping full JSON into context.
- **gherkin-mutator**: `--feature-file` and `--feature-path` are wrong flag names and silently run 0 mutations; use `--feature`.
- **Build release binary before acceptance**: `cargo build --release` must run before `--profile acceptance` — stale binary causes false failures.
- **acceptance-entrypoint-generator**: requires `--steps <steps-file>` (basename only) for non-scaffold features; omitting it defaults to `scaffold_cli_steps.rs` and silently generates the wrong entrypoint.
- **cargo-mutants `--exclude`**: patterns match absolute paths, not CWD-relative. Use `**/filename.rs` form, not `src/filename.rs`.
- **acceptance/runtime/mod.rs `World`**: add `#[allow(dead_code)]` on the struct (not per-field) — runtime is `include!`-ed across multiple test binaries with differing step coverage.
- **Acceptance fixture paths**: must be outside gitignored paths (e.g. not under `tmp/`) — gitignore-awareness is default-on and silently skips ignored dirs, producing false-empty scans.
- **Ad-hoc QA scratch fixtures**: write to `/tmp` or a non-gitignored path; any fixture under project `tmp/` is silently skipped by drywall — produces false exit 0 mimicking a missing feature.
- **Parallel language test bodies (js_X / ts_X)**: differentiate source body structure, not just identifiers — identical short bodies trigger dogfood self-detection even when function names differ.
- **`Lang::Tsx` grammar**: must use `LANGUAGE_TSX`, not `LANGUAGE_TYPESCRIPT`; `typescript_grammar_rejects_jsx_markup` is the guard — removing or weakening it silently breaks JSX in .tsx.

## drywall output format

drywall output mirrors dry4go: per-function node counts, one function per line. Sort order for ties is unspecified upstream; drywall pins lexicographic tie-break to guarantee deterministic output for mutation testing.