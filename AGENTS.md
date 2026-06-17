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
| Format | `cargo fmt` |
| Lint | `cargo clippy -- -D warnings` |
| Unit tests | `cargo nextest run --profile unit` |
| Property tests | `cargo nextest run --profile property` |
| Acceptance tests | `cargo nextest run --profile acceptance` |
| Coverage (≥90% lines) | `cargo llvm-cov nextest --profile unit --lcov --output-path lcov.info --fail-under-lines 90` |
| CRAP (threshold ≤6) | `cargo crap --lcov lcov.info --exclude 'acceptance/**' --threshold 6 --fail-above` |
| Build release binary | `cargo build --release` |
| DRY self-check | `./target/release/drywall ./src` |

## Rust engineering workflow

Always run `cargo clippy -- -D warnings` before submitting manual trait implementations or significant refactors. Clippy catches unnecessary impls and common patterns automatically.

Always run `cargo build --release` before running acceptance tests. The acceptance step dispatcher executes the compiled binary at `target/release/drywall` — the tests will silently use a stale binary or fail to find it if the build step is skipped.

## Rust coverage and CRAP

`cargo-llvm-cov` is NOT installed in this environment (no rustup; Homebrew Rust). CRAP must run without coverage data; scores in acceptance step files will be inflated — CC≤6 in `acceptance/**` files is acceptable without coverage.

When `cargo-llvm-cov` IS available, set env vars explicitly (Homebrew path has no `rustup`):
```
LLVM_COV=$(brew --prefix)/opt/llvm/bin/llvm-cov \
LLVM_PROFDATA=$(brew --prefix)/opt/llvm/bin/llvm-profdata \
cargo llvm-cov --lcov --output-path lcov.info
```

CRAP MUST always exclude acceptance infrastructure: `cargo crap --lcov lcov.info --exclude 'acceptance/**'`
