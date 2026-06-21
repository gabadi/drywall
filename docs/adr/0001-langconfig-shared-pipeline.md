# 1. One shared pipeline driven by per-grammar LangConfig

Date: 2026-06-21
Status: Accepted
Tracking: #5 (parent #1)

## Context

drywall must add JavaScript and TypeScript as analysis targets alongside the
existing Rust target. The normalization → subtree-hash → Jaccard pipeline that
detects duplicates is already implemented for Rust and is, in principle,
language-agnostic: it operates on tree-sitter's generic `Node` type and on
node-kind name strings, not on anything Rust-specific.

Two things genuinely differ between languages:

1. **Which node-kind names are identifiers and which are literals.** Rust uses
   `identifier`, `field_identifier`, `type_identifier`, `lifetime`, and a set of
   `*_literal` kinds. JS/TS grammars use different names (e.g. `identifier`,
   `property_identifier`, `string`, `number`, `true`/`false`, `template_string`).
2. **Which node-kind shapes are recognized as a reportable function form.** Rust
   recognizes `fn`, `impl` methods, `let`-bound closures, trait methods. JS/TS
   recognizes function declarations, `const`/`let` arrow functions, class
   methods, and exported named functions.

Everything else — walking subtrees, emitting `_ID`/`_LIT`, hashing, building the
inverted index, computing Jaccard, sorting, formatting — is identical regardless
of source language.

The competing approach considered earlier in the project (recorded in the PRD)
was OXC for JS/TS plus `syn` for Rust — richer, language-specific semantic
models with separate code paths per language.

## Decision

Keep a single shared pipeline. Parse every language with a tree-sitter
**grammar**, and confine all per-language knowledge to a **LangConfig** value
that declares (a) the identifier node-kind names and (b) the literal node-kind
names for that grammar, plus the per-language function-form recognition used by
the extractor. Adding a language is a new grammar dependency plus one LangConfig
entry and its function-form cases — not a new pipeline.

JSX is parsed by the JavaScript grammar; TSX by the TypeScript grammar. No
separate JSX/TSX grammar or config is introduced.

## Consequences

- Behavior across languages is uniform by construction: the same threshold,
  gates, sorting, exit codes, and output format apply to JS/TS exactly as to
  Rust, because they are the same code. The JS/TS feature spec can therefore
  reference #3's contract for all shared behavior and specify only what is new.
- Misclassifying a node kind (e.g. forgetting `property_identifier` is an
  identifier in JS) silently degrades match quality rather than erroring — it
  is the most likely defect class for this slice, so the spec and QA must
  include a same-structure / different-names twin per language to catch it.
- Cross-language matching is *not* enabled by this decision and remains out of
  scope: hashes from different grammars are not compared as a deliberate slice
  boundary, even though the pipeline is shared.
- The richer semantic model OXC/`syn` would provide is forgone. Subtree hashing
  needs only the identifier/literal distinction, which tree-sitter supplies for
  all three languages, so nothing in scope requires it.
