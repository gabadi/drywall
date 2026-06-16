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

## Rust coverage and CRAP

`cargo-llvm-cov` is NOT installed in this environment (no rustup; Homebrew Rust). CRAP must run without coverage data; scores in acceptance step files will be inflated — CC≤6 in `acceptance/**` files is acceptable without coverage.

When `cargo-llvm-cov` IS available, set env vars explicitly (Homebrew path has no `rustup`):
```
LLVM_COV=$(brew --prefix)/opt/llvm/bin/llvm-cov \
LLVM_PROFDATA=$(brew --prefix)/opt/llvm/bin/llvm-profdata \
cargo llvm-cov --lcov --output-path lcov.info
```

CRAP MUST always exclude acceptance infrastructure: `cargo crap --lcov lcov.info --exclude 'acceptance/**'`
