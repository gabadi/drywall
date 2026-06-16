# End-to-End QA Suite: Scaffold CLI invocation

Tracks: #2 (parent #1)
Feature file: `features/scaffold_cli.feature`

## Scope

Verifies the scaffold's user-visible contract through the command-line interface
only. QA spawns the compiled binary as a subprocess and observes exit code,
stdout, and stderr. QA does not call any project API and does not inspect
internal data structures (parser, grammar handles, AST).

This scaffold has **no duplicate-detection behavior**. The only observable
contract is: the binary runs, accepts path arguments, and exits cleanly with no
output.

## Preconditions

- The release binary is built (`cargo build --release`) and exists at
  `target/release/drywall`.
- A working directory containing at least `./src` and `./features`.

## User-visible workflows

### QA-1 — Single existing path exits cleanly
- Input: `drywall ./src`
- Expected exit code: `0`
- Expected stdout: empty
- Expected stderr: empty (no warnings, no analysis output)

### QA-2 — Multiple existing paths exit cleanly
- Input: `drywall ./src ./features`
- Expected exit code: `0`
- Expected stdout: empty
- Expected stderr: empty

### QA-3 — No path arguments exits cleanly
- Input: `drywall`
- Expected exit code: `0`
- Expected stdout: empty
- Expected stderr: empty
- Observable state: the process returns promptly; it does not hang and does not
  open an interactive prompt.

### QA-4 — Nonexistent path does not panic
- Input: `drywall ./does-not-exist`
- Expected exit code: `0`
- Expected stdout: empty
- Expected stderr: contains no Rust panic text (no `panicked at`, no backtrace).
- Rationale: path validation is later behavior; the scaffold must tolerate any
  path argument without crashing.

### QA-5 — Minimal Rust source parses without error (observed via clean run)
- Setup: a directory `./tmp/qa-minimal/` containing one minimal valid Rust file,
  e.g. `lib.rs` with `pub fn answer() -> i32 { 42 }`.
- Input: `drywall ./tmp/qa-minimal`
- Expected exit code: `0`
- Expected stdout: empty
- Expected stderr: empty (no parse-error text)
- Rationale: confirms the tree-sitter-rust grammar is loaded and a minimal Rust
  source parses without surfacing an error to the user. Observed only through
  the clean-exit / no-error contract at the CLI; the AST itself is not inspected.

## Observable states summary

| State | How QA observes it |
|---|---|
| Clean success | exit code `0` AND empty stdout |
| No crash | stderr contains no `panicked at` / backtrace text |
| No premature behavior | stdout is empty (no `DUPLICATE`, no JSON) |
| No hang | process returns without manual interruption |

## Out of scope for this suite

- Duplicate-detection output (text or JSON).
- Exit codes `1` (duplicates found) and `2` (error) — not implemented in scaffold.
- Any flag behavior (`--threshold`, `--format`, etc.) — not part of this scaffold.
- Path validation / error reporting on bad input.
