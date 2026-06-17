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
| Coverage (≥90% lines) | `~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo llvm-cov nextest --profile unit --lcov --output-path lcov.info --fail-under-lines 90` |
| CRAP (threshold ≤6) | `cargo crap --lcov lcov.info --exclude 'acceptance/**' --threshold 6 --fail-above` |
| Build release binary | `cargo build --release` |
| DRY self-check | `./target/release/drywall ./src` |

## Tooling notes

- **Coverage/llvm-cov**: use the explicit rustup cargo path (`~/.rustup/toolchains/stable-aarch64-apple-darwin/bin/cargo llvm-cov`) — the system Homebrew Rust does not include `llvm-profdata` (because AGENTS.md "coverage" command fails with `llvm-profdata not found`).
- **Diff for tools**: use bare `git` (not `rtk git`) when generating patch files for tools like `cargo-mutants --in-diff`; RTK reformats `git diff` output and breaks unified-diff parsing.
- **drywall output format**: mirrors dry4go — per-function node counts; sort order ties unspecified upstream; drywall pins lexicographic tie-break for deterministic mutation testing.
- **gherkin-ir-dry-checker output**: pipe the JSON report through a compact summary (e.g. `python3 -c "import json,sys; r=json.load(sys.stdin); print(f'findings: {len(r[\"findings\"])}')"`) rather than dumping full JSON into context.