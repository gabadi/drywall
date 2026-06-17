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

## Rust coverage and CRAP

On Homebrew Rust (no rustup), `cargo llvm-cov` requires explicit env vars:
```
LLVM_COV=$(brew --prefix)/opt/llvm/bin/llvm-cov \
LLVM_PROFDATA=$(brew --prefix)/opt/llvm/bin/llvm-profdata \
cargo llvm-cov nextest --profile unit --lcov --output-path lcov.info
```
If `cargo-llvm-cov` is not installed, CRAP runs without coverage data — CC≤6 in `acceptance/**` is acceptable in that case.
