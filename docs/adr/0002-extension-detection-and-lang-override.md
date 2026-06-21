# 2. Extension-based grammar auto-detection with a `--lang` override

Date: 2026-06-21
Status: Accepted
Tracking: #5 (parent #1)

## Context

With more than one grammar available, drywall must decide which grammar parses
each file. Until now the only target was Rust, so the scanner hard-coded `.rs`
and `--lang` accepted only `rust` (any other value exited 2). The JS/TS slice
introduces four new extensions (`.js`, `.jsx`, `.ts`, `.tsx`) and the need to
analyze a directory that mixes languages in one invocation.

Open questions this slice forces:

- How is a file's grammar chosen by default?
- What does `--lang` mean when a directory contains files of several languages?
- What values must `--lang` accept now?

## Decision

**Default — auto-detect by extension, per file:**

| Extension      | Grammar     |
|----------------|-------------|
| `.rs`          | Rust        |
| `.js`, `.jsx`  | JavaScript  |
| `.ts`, `.tsx`  | TypeScript  |

Each file is parsed with the grammar its own extension selects. In a
**mixed-language directory**, files of different languages are each parsed with
their own grammar and the resulting duplicate pairs are merged into one sorted
output stream — a single invocation produces a single report spanning all
languages present.

**Override — `--lang` forces one grammar for every scanned file**, regardless of
extension, for this invocation. Accepted values are extended from `{rust}` to
`{rust, js, ts}`. `js` forces the JavaScript grammar; `ts` forces TypeScript.
There is no separate `jsx`/`tsx` override value — those extensions are an
auto-detection concern only; forcing JS or TS already parses JSX/TSX content via
the corresponding grammar. An unsupported `--lang` value remains an argument
error (exit 2), unchanged from the Rust slice.

## Consequences

- `--lang ts` (and `js`) lets a user analyze files with non-standard extensions,
  satisfying issue #5's explicit acceptance criterion, with the same override
  semantics already established for `rust` — only the accepted set grows.
- When `--lang` is supplied, extension-based detection is bypassed entirely:
  every file is read through the forced grammar even if its extension would map
  elsewhere. This is the intended escape hatch and is the behavior QA verifies
  by forcing a grammar on a non-matching extension.
- A mixed `.rs` + `.ts` directory needs no special flag: auto-detection per file
  plus one merged sorted output is the default. The single-output requirement is
  a direct consequence of decision ADR-0001's shared pipeline — pairs from all
  grammars flow into the same sort.
- JSX/TSX get no first-class CLI surface; `.jsx`/`.tsx` are recognized only
  through extension auto-detection (or by forcing `js`/`ts`). If a future need
  arises to force JSX distinctly from JS this decision would have to be revisited,
  but nothing in scope requires it.
- Python (`.py`, `--lang py`) is deliberately excluded here and tracked
  separately; adding it later is purely additive to both the extension table and
  the accepted `--lang` set.
