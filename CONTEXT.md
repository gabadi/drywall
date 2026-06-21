# drywall — Domain Glossary

The ubiquitous language for drywall, a polyglot AST-subtree DRY analyzer. This
file is a glossary only: no implementation details, no specs. Terms are listed
alphabetically.

---

## Analysis unit

A single source-language construct that drywall treats as one comparable whole
function for duplicate detection. Each analysis unit yields exactly one set of
normalized subtree hashes and one reported line range. What counts as an
analysis unit is **language-specific** and enumerated per language by that
language's [function forms](#function-form).

## Auto-detection

Selecting which [grammar](#grammar) parses a file from the file's extension,
with no `--lang` flag supplied. `.rs` → Rust; `.js` / `.jsx` → JavaScript;
`.ts` / `.tsx` → TypeScript; `.py` → Python. Auto-detection is the default;
`--lang` overrides it. See [language override](#language-override).

## Candidate

A [function](#function) that has passed both [qualification gates](#qualification-gate)
(`--min-lines` and `--min-nodes`) and is therefore eligible to be one half of a
reported [duplicate pair](#duplicate-pair). A function below either gate is not
a candidate and never appears in output.

## Duplicate pair

An unordered pair of two distinct [candidates](#candidate) whose
[Jaccard similarity](#jaccard-similarity) is at least `--threshold`. A function
is never paired with itself; each unordered pair is reported at most once. The
unit of drywall's output.

## Function

Used loosely as a synonym for [analysis unit](#analysis-unit). A "function" in
drywall is whatever the active language's [function forms](#function-form)
recognize — it is not restricted to constructs the source language calls
"functions" (e.g. a Rust `impl` method or a JS class method is a function here).

## Function form

A specific syntactic shape, named by [grammar](#grammar) node type, that the
[extractor](#extractor) recognizes as an [analysis unit](#analysis-unit). The
recognized forms per language:

- **Rust**: `fn` declarations, `impl`-block methods, closures bound to a `let`
  binding, trait-implementation methods.
- **JavaScript / TypeScript**: function declarations, arrow functions bound to a
  `const` or `let` binding, class methods, and exported named functions (the
  same underlying forms reached through an `export`). JSX and TSX files
  (`.jsx`, `.tsx`) are included and use the JS/TS grammars respectively.
- **Python**: `def` functions, `async def` functions, and class methods. All
  three are the same grammar node (`function_definition`); `async def` carries an
  `async` keyword and a method is a `function_definition` nested in a class body,
  so they are observably distinct forms even though the extractor recognizes one
  node kind.

## Grammar

A tree-sitter parser for one source language: `tree-sitter-rust`,
`tree-sitter-javascript`, `tree-sitter-typescript`, `tree-sitter-python`.
drywall parses every
language through tree-sitter's generic `Node` type, so one shared
[normalization](#normalization) and [Jaccard](#jaccard-similarity) pipeline
serves all grammars. JSX is parsed by the JavaScript grammar; TSX by the
TypeScript grammar.

## Jaccard similarity

`|A ∩ B| / |A ∪ B|` computed over two functions' sets of normalized subtree
hashes. The score a [duplicate pair](#duplicate-pair) is reported with. Range
`[0.0, 1.0]`; two functions identical after [normalization](#normalization)
score `1.0`. Language-agnostic — it operates on hashes, not source.

## LangConfig

The per-[grammar](#grammar) configuration that adapts the shared pipeline to a
language. It declares which tree-sitter node type name strings are
[identifiers](#identifier-normalization) and which are
[literals](#literal-normalization), so [normalization](#normalization) emits
`_ID` and `_LIT` correctly for that grammar. Adding a language is a new grammar
dependency plus one LangConfig entry; the rest of the pipeline is unchanged.

## Language override

Forcing a specific [grammar](#grammar) via `--lang`, regardless of file
extension. Accepted values for the current slices: `rust`, `js`, `ts`, `py`.
Used to analyze files with non-standard extensions or to disambiguate a
mixed-language directory. Overrides [auto-detection](#auto-detection). An
unsupported value is an argument error (exit 2). (The full-word `rust` versus the
short `js` / `ts` / `py` spelling is a known inconsistency, tracked for a future
unification; it does not affect this slice.)

## Mixed-language directory

A scanned directory containing source files of more than one supported language
(e.g. `.rs`, `.ts`, and `.py` together). Without `--lang`, each file is parsed with the
[grammar](#grammar) its own extension selects, and the
[duplicate pairs](#duplicate-pair) from all languages are merged into one
[sorted](#sorted-output) output stream — a single invocation, a single report.

## Normalization

Rewriting a parsed subtree into a canonical string where
[identifier](#identifier-normalization) nodes become `_ID` and
[literal](#literal-normalization) nodes become `_LIT`, while operators,
keywords, and structure are preserved verbatim. Two functions with identical
structure but different identifier/literal text normalize identically. The step
that makes detection insensitive to variable names and magic values.

### Identifier normalization

The normalization rule that maps all identifier-category nodes to the single
`_ID` token, so renaming a variable cannot break a match. Which node types count
as identifiers is declared per language by [LangConfig](#langconfig).

### Literal normalization

The normalization rule that maps all literal-category nodes (string, numeric,
boolean, and language equivalents) to the single `_LIT` token, so changing a
constant cannot break a match. Which node types count as literals is declared
per language by [LangConfig](#langconfig).

## Qualification gate

A minimum-size threshold a [function](#function) must clear to become a
[candidate](#candidate): `--min-lines` source lines (default 4) AND `--min-nodes`
normalized AST nodes (default 20). Both must pass. Suppresses noise from trivial
functions that match each other universally. Language-agnostic.

## Sorted output

drywall's output ordering: [duplicate pairs](#duplicate-pair) by
[Jaccard](#jaccard-similarity) score descending, ties broken deterministically
by `(left file, left start line, right file, right start line)` ascending, so
identical input yields byte-identical output. Holds across languages in a
[mixed-language directory](#mixed-language-directory).

---

## Terms intentionally NOT in drywall's language (this slice)

- **Cross-language duplicate** — a Rust function matching a Python or TypeScript
  function. Out of scope; each file is compared only within its own grammar's
  hash space.
- **Block-level duplicate** — sub-function duplication. Out of scope; only
  whole-function pairs are detected.
