# End-to-End QA Suite: Python duplicate detection

Tracks: #6 (parent #1)
Feature file: `features/python_detection.feature`

## Scope

Verifies the Python analysis target entirely through the command-line
interface. QA spawns the compiled binary as a subprocess, passes fixture
directories and flags, and observes only exit code, stdout, and stderr. QA does
not call any project API and does not inspect internal structures (parser,
grammar selection, LangConfig, AST, hash sets, inverted index).

This suite covers only what #6 adds on top of the Rust (#3) and JS/TS (#5)
targets: the recognized Python function forms, `.py` extension grammar
auto-detection, the `py` value of `--lang`, mixed-language single-output
spanning Python, and the Python parse-error case. The shared contract — text/JSON
format, exit codes, thresholds, size gates, sorting,
`--threshold`/`--min-lines`/`--min-nodes`/`--format`, and the three exclusion
layers — is verified by the #3 suite (`features/duplicate_detection.qa.md`) and
the #4 suite (`features/cli_surface.qa.md`) and is not re-verified here. The
`__pycache__` built-in exclusion is owned and verified by #4, not here.

The user-interface affordances exercised are the positional `<path>` arguments
and the `--lang` flag (`py`).

## Preconditions

- The release binary is built (`cargo build --release`) at
  `target/release/drywall`.
- Fixtures are created on disk under a QA-controlled temp directory (e.g.
  `./tmp/qa-python/<case>/`); QA writes `.py`, `.rs`, `.ts`, and
  non-standard-extension files, never touching project internals.
- **Fixture visibility under gitignore-awareness**: because gitignore-awareness
  is default-on (see #4), QA must ensure fixture source files are NOT reported
  ignored by `git check-ignore`. If the repo gitignores `tmp/`, QA places
  fixtures outside any ignored path (e.g. a scratch directory created with
  `mktemp -d`, or a path the harness first confirms is not ignored). A fixture
  silently swallowed by an ambient `.gitignore` would make a test pass for the
  wrong reason (zero files scanned → false "no duplicates").

## Fixture builders (QA-side, on disk only)

- **PY-TWIN(form)**: two `.py` files whose functions use the named Python
  function form (`def-function`, `async-def-function`, `class-method`) and have
  identical control flow and statement structure but different identifier and
  literal text; each function clears the default gates (≥ 4 lines, ≥ 20 nodes)
  and scores ≥ 0.82. For `class-method`, each `def` is nested in a `class` body;
  for `async-def-function`, each function is declared `async def`.
- **NONSTD-EXT-TWIN(py)**: a structural twin pair whose files use a non-standard
  extension (`.txt`) so that only `--lang py` causes them to be analyzed.
- **MIXED-DIR**: one directory containing an `.rs` twin pair, a `.ts` twin pair,
  AND a `.py` twin pair, each pair a structural twin scoring ≥ 0.82, used to
  confirm all three grammars run in one invocation and all three pairs appear in
  one output.
- **UNPARSABLE(py)**: a single `.py` file containing syntactically invalid
  Python source.

## User-visible workflows

### QA-1 — Python structural twin reported (exit 1)
- Setup: PY-TWIN(`def-function`) in `<scratch>/py-twin/src/`.
- Input: `drywall <scratch>/py-twin/src`
- Expected exit code: `1`.
- Expected stdout: one `DUPLICATE` block naming both `.py` files with their line
  ranges; `score=` ≥ `0.82`.
- Expected stderr: empty.
- Rationale: proves the shared normalization/Jaccard pipeline runs under the
  Python grammar with identifier names differing.

### QA-2 — Each Python function form is recognized (exit 1)
Run once per function form: `def-function`, `async-def-function`,
`class-method`.
- Setup: PY-TWIN(`<form>`) in `<scratch>/form/<form>/src/`.
- Input: `drywall <scratch>/form/<form>/src`
- Expected exit code: `1`; one `DUPLICATE` block naming both files.
- Rationale: each form is a reportable analysis unit. `async def` exercises the
  `async`-keyword path and `class-method` exercises a `def` nested in a class
  body; a regression in either would let a whole class of Python duplicates go
  unreported even though all three share one `function_definition` node kind.

### QA-3 — `.py` auto-detected as Python without `--lang` (exit 1)
- Setup: PY-TWIN in `<scratch>/auto-py/src/` (no `--lang`).
- Input: `drywall <scratch>/auto-py/src`
- Expected exit code: `1`; the pair is reported, confirming the `.py` extension
  selected the Python grammar.

### QA-4 — `--lang py` forces Python on a non-standard extension (exit 1)
- Setup: NONSTD-EXT-TWIN(`py`) — a structural twin in `.txt` files under
  `<scratch>/force-py/src/`. Without `--lang` these `.txt` files are not
  analyzed.
- Input: `drywall --lang py <scratch>/force-py/src`
- Expected exit code: `1`; the pair is reported, confirming the forced language
  overrode extension-based detection.
- Control (optional): `drywall <scratch>/force-py/src` (no `--lang`) → exit `0`,
  no `DUPLICATE` line, because `.txt` is not an auto-detected extension.

### QA-5 — Mixed `.rs` + `.ts` + `.py` directory yields one merged output (exit 1)
- Setup: MIXED-DIR under `<scratch>/mixed/src/`: an `.rs` twin pair
  (`a.rs`/`b.rs`), a `.ts` twin pair (`c.ts`/`d.ts`), AND a `.py` twin pair
  (`e.py`/`f.py`), all six files in the same scanned directory.
- Input: `drywall <scratch>/mixed/src`
- Expected exit code: `1`.
- Expected stdout: a single report containing the `.rs` pair, the `.ts` pair,
  AND the `.py` pair `DUPLICATE` blocks; the blocks are ordered by `score=` per
  the shared sorting contract.
- Determinism: running the same input twice yields byte-identical stdout.
- Rationale: each file is parsed with its own extension's grammar and all pairs
  merge into one sorted output — one invocation, one report across three
  languages.

### QA-6 — Unparsable Python source is an error (exit 2)
- Setup: UNPARSABLE(`py`) in `<scratch>/bad-py/src/`.
- Input: `drywall <scratch>/bad-py/src`
- Expected exit code: `2`.
- Expected stderr: non-empty diagnostic.
- Rationale: distinguishes a Python parse failure (exit 2) from a clean
  no-duplicate run (exit 0), matching the Rust parse-error contract.

### QA-7 — No cross-language pairing with Python (exit 0)
- Setup: one `.rs` file and one `.py` file in `<scratch>/cross/src/` whose
  functions are structural twins of EACH OTHER (same structure, different
  identifiers) but there is NO same-language twin for either.
- Input: `drywall <scratch>/cross/src`
- Expected exit code: `0`; stdout has no `DUPLICATE` line.
- Rationale: a Python function is never paired with a Rust (or JS/TS) function;
  comparison is within a grammar's hash space only. Guards the scope boundary —
  if cross-language pairing leaked in, this would wrongly report a pair (exit 1).

### QA-8 — Dogfood unaffected by Python support (exit 0)
- Input: `drywall ./src` run from the drywall project root.
- Expected exit code: `0`; no `DUPLICATE` line; stderr empty.
- Rationale: drywall's own source is Rust; adding the Python target must not
  change the dogfood result. The dogfood gate must still pass.

## Observable states summary

| State | How QA observes it |
|---|---|
| Python duplicates found | exit code `1` AND ≥ 1 `DUPLICATE` block naming the `.py` files |
| No duplicates | exit code `0` AND no `DUPLICATE` block |
| Python parse error | exit code `2` AND non-empty stderr diagnostic |
| Auto-detected grammar | a `.py` twin is reported with no `--lang` |
| Forced grammar | a twin in a non-standard extension is reported only with `--lang py` |
| Merged mixed-language output | one run reports the `.rs`, `.ts`, and `.py` pairs |
| No cross-language pairing | rs-vs-py structural twins with no same-language partner → exit `0` |
| Deterministic output | identical input run twice yields byte-identical stdout |

## Out of scope for this suite

- Rust and JS/TS analysis (verified by the #3 and #5 suites); this suite uses
  `.rs`/`.ts` fixtures only as mixed-directory and cross-language companions.
- The full-word `rust` versus short `py` spelling of `--lang` (a known
  inconsistency tracked for a future unification; not a behavior this slice
  changes).
- Reporting format, exit-code-2 argument errors, thresholds, size gates,
  `--format`, single-pattern `--exclude` (verified by the #3 suite).
- Built-in / gitignore / repeatable `--exclude` exclusion, including the
  `__pycache__` built-in exclusion (verified by the #4 suite).
- Cross-language duplicate detection as a feature (deliberately excluded; QA-7
  asserts its absence, it does not test it as supported behavior).
- Internal verification of grammar selection, LangConfig, normalization, or
  hashing (covered by the project's unit tests, not by this CLI-only QA suite).
