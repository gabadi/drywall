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

## Setup

Run `mise install` once to install the Rust toolchain and all project tools (nextest, llvm-cov, crap, mutants). Requires mise to be activated in the shell (`eval "$(mise activate zsh)"` in `.zshrc`); without activation, prefix commands with `mise exec --`.

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