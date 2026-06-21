---
name: run-drywall
description: >
  Build, run, and smoke-test the drywall CLI — detects duplicate functions in
  Rust, JavaScript, TypeScript, and Python. Use when asked to run, build,
  screenshot, or verify the drywall tool, or when running a smoke test,
  checking the binary works, or testing a change before merging.
---

drywall is a CLI tool that detects structurally duplicate functions using
tree-sitter AST analysis. It is driven directly — no server or GUI. The
primary agent path is `smoke.sh` in this skill directory; it builds the binary
and runs representative invocations covering all exit codes and output formats.

## Prerequisites

Rust toolchain (managed by `rust-toolchain.toml` — `rustup` handles it
automatically).

```bash
rustup show   # verify toolchain is installed
```

No OS packages beyond the Rust toolchain are required.

## Build

```bash
cargo build --release
```

Binary lands at `./target/release/drywall`. The build is fast on an already-
compiled tree — under 5 seconds if no sources changed.

## Run (agent path)

Run the smoke script from repo root:

```bash
bash .claude/skills/run-drywall/smoke.sh
```

Exit 0 = all checks pass. Exit 1 = at least one check failed (check output).

The script verifies:
- `--help` exits 0
- Clean directory → exit 0
- Duplicate Rust functions → exit 1, `DUPLICATE` on stdout
- `--format json` → non-empty JSON array on stdout, exit 1
- Duplicate Python functions → exit 1
- Unknown `--lang` value → exit 2
- Clean directory with `--format json` → exit 0

## Run (human path)

```bash
./target/release/drywall --help
./target/release/drywall src/
./target/release/drywall src/ --format json
```

A window does NOT open — drywall is a pure CLI.

## Output contract

| Scenario | stdout | stderr | Exit |
|---|---|---|---|
| No duplicates | (empty) | (empty) | 0 |
| Duplicates found, text format | `DUPLICATE score=…\n  file:L-L\n  file:L-L (N nodes / N nodes)` | (empty) | 1 |
| Duplicates found, JSON format | JSON array of pair objects | (empty) | 1 |
| Argument error (bad `--lang`, etc.) | (empty) | error message | 2 |
| Parse error in a file | (empty) | `error: parse error in <file>` | 2 |

## Test suite

```bash
cargo test
```

240 tests across 13 suites, ~9 seconds. Covers acceptance, property, and
mutation-hardening tests.

## Gotchas

- **Extension required for auto-detection.** Passing a `.rs`-suffixed path is
  mandatory for auto-detection; creating a tmpfile with `mktemp` and no
  extension will cause drywall to skip the file silently (exit 0). Always add
  the extension: `mktemp /tmp/XXXXXX; mv "$f" "${f}.rs"; f="${f}.rs"`.
- **DUPLICATE text goes to stdout, not stderr.** The CLI routes all report
  output through `result.stdout` → `print!`. Only error messages (parse errors,
  walk errors) go to stderr. If you redirect to capture errors, use `2>/dev/null`,
  not `1>/dev/null`.
- **Exit 1 means "duplicates found," not "error."** CI integrations should
  treat exit 1 as a signal, not a failure — unless they intend to fail on
  detected duplicates. Exit 2 is the real error code.
- **Pairs sorted by score descending.** Multiple pairs appear highest-score
  first; ties break by `(left file, left line, right file, right line)`.
- **--lang overrides extension.** Passing `--lang rust` to a `.py` file will
  parse it as Rust (and likely produce no results or a parse error).
