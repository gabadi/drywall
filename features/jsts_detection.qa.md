# End-to-End QA Suite: JavaScript and TypeScript duplicate detection

Tracks: #5 (parent #1)
Feature file: `features/jsts_detection.feature`

## Scope

Verifies the JavaScript and TypeScript analysis targets entirely through the
command-line interface. QA spawns the compiled binary as a subprocess, passes
fixture directories and flags, and observes only exit code, stdout, and stderr.
QA does not call any project API and does not inspect internal structures
(parser, grammar selection, LangConfig, AST, hash sets, inverted index).

This suite covers only what #5 adds on top of the Rust target: the recognized
JS/TS function forms, extension-based grammar auto-detection, the `js`/`ts`
values of `--lang`, mixed-language single-output, and the JS/TS parse-error
case. The shared contract — text/JSON format, exit codes, thresholds, size
gates, sorting, `--threshold`/`--min-lines`/`--min-nodes`/`--format`, and the
three exclusion layers — is verified by the #3 suite
(`features/duplicate_detection.qa.md`) and the #4 suite
(`features/cli_surface.qa.md`) and is not re-verified here.

The user-interface affordances exercised are the positional `<path>` arguments
and the `--lang` flag (`js`, `ts`).

## Preconditions

- The release binary is built (`cargo build --release`) at
  `target/release/drywall`.
- Fixtures are created on disk under a QA-controlled temp directory (e.g.
  `./tmp/qa-jsts/<case>/`); QA writes `.js`, `.jsx`, `.ts`, `.tsx`, `.rs`, and
  non-standard-extension files, never touching project internals.
- **Fixture visibility under gitignore-awareness**: because gitignore-awareness
  is default-on (see #4), QA must ensure fixture source files are NOT reported
  ignored by `git check-ignore`. If the repo gitignores `tmp/`, QA places
  fixtures outside any ignored path (e.g. a scratch directory created with
  `mktemp -d`, or a path the harness first confirms is not ignored). A fixture
  silently swallowed by an ambient `.gitignore` would make a test pass for the
  wrong reason (zero files scanned → false "no duplicates").

## Fixture builders (QA-side, on disk only)

- **JS-TWIN(form)**: two `.js` files whose functions use the named JS function
  form (`function-declaration`, `arrow-function`, `class-method`,
  `exported-named-function`) and have identical control flow and statement
  structure but different identifier and literal text; each function clears the
  default gates (≥ 4 lines, ≥ 20 nodes) and scores ≥ 0.82.
- **TS-TWIN(form)**: the TypeScript counterpart of JS-TWIN, written in `.ts`
  files. May carry type annotations; the structural twin still scores ≥ 0.82
  after normalization.
- **JSX-TWIN / TSX-TWIN**: a structural twin pair in `.jsx` / `.tsx` files whose
  function body contains JSX/TSX markup, used to confirm those extensions are
  recognized and parsed by the JS / TS grammar respectively.
- **NONSTD-EXT-TWIN(lang)**: a structural twin pair whose files use a
  non-standard extension (`.txt` for `ts`, `.inc` for `js`) so that only
  `--lang <lang>` causes them to be analyzed.
- **MIXED-DIR**: one directory containing an `.rs` twin pair AND a `.ts` twin
  pair, each pair a structural twin scoring ≥ 0.82, used to confirm both
  grammars run in one invocation and both pairs appear in one output.
- **UNPARSABLE(ext)**: a single `.ts` / `.js` file containing syntactically
  invalid source.

## User-visible workflows

### QA-1 — TypeScript structural twin reported (exit 1)
- Setup: TS-TWIN(`arrow-function`) in `<scratch>/ts-twin/src/`.
- Input: `drywall <scratch>/ts-twin/src`
- Expected exit code: `1`.
- Expected stdout: one `DUPLICATE` block naming both `.ts` files with their line
  ranges; `score=` ≥ `0.82`.
- Expected stderr: empty.
- Rationale: proves the shared normalization/Jaccard pipeline runs under the
  TypeScript grammar with identifier names differing.

### QA-2 — JavaScript structural twin reported (exit 1)
- Setup: JS-TWIN(`function-declaration`) in `<scratch>/js-twin/src/`.
- Input: `drywall <scratch>/js-twin/src`
- Expected exit code: `1`; one `DUPLICATE` block naming both `.js` files;
  `score=` ≥ `0.82`; stderr empty.

### QA-3 — Each JS/TS function form is recognized (exit 1)
Run once per function form: `function-declaration`, `arrow-function`,
`class-method`, `exported-named-function`.
- Setup: JS-TWIN(`<form>`) or TS-TWIN(`<form>`) in
  `<scratch>/form/<form>/src/`.
- Input: `drywall <scratch>/form/<form>/src`
- Expected exit code: `1`; one `DUPLICATE` block naming both files.
- Rationale: each form is a distinct analysis unit; a twin written in that form
  must be detected. Missing-form coverage would let a whole class of duplicates
  go unreported.

### QA-4 — `.ts` auto-detected as TypeScript without `--lang` (exit 1)
- Setup: TS-TWIN in `<scratch>/auto-ts/src/` (no `--lang`).
- Input: `drywall <scratch>/auto-ts/src`
- Expected exit code: `1`; the pair is reported, confirming the `.ts` extension
  selected the TypeScript grammar.

### QA-5 — `.js` auto-detected as JavaScript without `--lang` (exit 1)
- Setup: JS-TWIN in `<scratch>/auto-js/src/` (no `--lang`).
- Input: `drywall <scratch>/auto-js/src`
- Expected exit code: `1`; the pair is reported.

### QA-6 — `.tsx` and `.jsx` are auto-detected (exit 1)
Run once for `.tsx` (TSX-TWIN, TypeScript grammar) and once for `.jsx`
(JSX-TWIN, JavaScript grammar).
- Setup: TSX-TWIN / JSX-TWIN in `<scratch>/auto-x/<ext>/src/`.
- Input: `drywall <scratch>/auto-x/<ext>/src`
- Expected exit code: `1`; the pair is reported, confirming JSX/TSX content is
  parsed by the corresponding grammar.

### QA-7 — `--lang ts` forces TypeScript on a non-standard extension (exit 1)
- Setup: NONSTD-EXT-TWIN(`ts`) — a structural twin in `.txt` files under
  `<scratch>/force-ts/src/`. Without `--lang` these `.txt` files are not
  analyzed.
- Input: `drywall --lang ts <scratch>/force-ts/src`
- Expected exit code: `1`; the pair is reported, confirming the forced language
  overrode extension-based detection.
- Control (optional): `drywall <scratch>/force-ts/src` (no `--lang`) → exit `0`,
  no `DUPLICATE` line, because `.txt` is not an auto-detected extension.

### QA-8 — `--lang js` forces JavaScript on a non-standard extension (exit 1)
- Setup: NONSTD-EXT-TWIN(`js`) — a structural twin in `.inc` files under
  `<scratch>/force-js/src/`.
- Input: `drywall --lang js <scratch>/force-js/src`
- Expected exit code: `1`; the pair is reported.

### QA-9 — Mixed `.rs` + `.ts` directory yields one merged output (exit 1)
- Setup: MIXED-DIR under `<scratch>/mixed/src/`: an `.rs` twin pair
  (`a.rs`/`b.rs`) AND a `.ts` twin pair (`c.ts`/`d.ts`), all four files in the
  same scanned directory.
- Input: `drywall <scratch>/mixed/src`
- Expected exit code: `1`.
- Expected stdout: a single report containing BOTH the `.rs` pair `DUPLICATE`
  block and the `.ts` pair `DUPLICATE` block; the blocks are ordered by `score=`
  per the shared sorting contract.
- Determinism: running the same input twice yields byte-identical stdout.
- Rationale: each file is parsed with its own extension's grammar and all pairs
  merge into one sorted output — one invocation, one report across languages.

### QA-10 — Unparsable TypeScript source is an error (exit 2)
- Setup: UNPARSABLE(`ts`) in `<scratch>/bad-ts/src/`.
- Input: `drywall <scratch>/bad-ts/src`
- Expected exit code: `2`.
- Expected stderr: non-empty diagnostic.
- Rationale: distinguishes a JS/TS parse failure (exit 2) from a clean
  no-duplicate run (exit 0), matching the Rust parse-error contract.

### QA-11 — Unparsable JavaScript source is an error (exit 2)
- Setup: UNPARSABLE(`js`) in `<scratch>/bad-js/src/`.
- Input: `drywall <scratch>/bad-js/src`
- Expected exit code: `2`; stderr non-empty.

### QA-12 — No cross-language pairing (exit 0)
- Setup: one `.rs` file and one `.ts` file in `<scratch>/cross/src/` whose
  functions are structural twins of EACH OTHER (same structure, different
  identifiers) but there is NO same-language twin for either.
- Input: `drywall <scratch>/cross/src`
- Expected exit code: `0`; stdout has no `DUPLICATE` line.
- Rationale: a Rust function is never paired with a TypeScript function;
  comparison is within a grammar's hash space only. Guards the scope boundary —
  if cross-language pairing leaked in, this would wrongly report a pair (exit 1).

### QA-13 — Dogfood unaffected by JS/TS support (exit 0)
- Input: `drywall ./src` run from the drywall project root.
- Expected exit code: `0`; no `DUPLICATE` line; stderr empty.
- Rationale: drywall's own source is Rust; adding JS/TS targets must not change
  the dogfood result. The dogfood gate must still pass.

## Observable states summary

| State | How QA observes it |
|---|---|
| JS/TS duplicates found | exit code `1` AND ≥ 1 `DUPLICATE` block naming the JS/TS files |
| No duplicates | exit code `0` AND no `DUPLICATE` block |
| JS/TS parse error | exit code `2` AND non-empty stderr diagnostic |
| Auto-detected grammar | a twin in a given extension is reported with no `--lang` |
| Forced grammar | a twin in a non-standard extension is reported only with `--lang` |
| Merged mixed-language output | one run reports both the `.rs` pair and the `.ts` pair |
| No cross-language pairing | rs-vs-ts structural twins with no same-language partner → exit `0` |
| Deterministic output | identical input run twice yields byte-identical stdout |

## Out of scope for this suite

- Python analysis and `--lang py` (tracked separately; not this slice).
- A `jsx` or `tsx` value of `--lang` (not provided; `.jsx`/`.tsx` are
  auto-detected or reached by forcing `js`/`ts`).
- Reporting format, exit-code-2 argument errors, thresholds, size gates,
  `--format`, single-pattern `--exclude` (verified by the #3 suite).
- Built-in / gitignore / repeatable `--exclude` exclusion (verified by the #4
  suite).
- Cross-language duplicate detection as a feature (deliberately excluded; QA-12
  asserts its absence, it does not test it as supported behavior).
- Internal verification of grammar selection, LangConfig, normalization, or
  hashing (covered by the project's unit tests, not by this CLI-only QA suite).
