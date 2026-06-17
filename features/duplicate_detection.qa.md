# End-to-End QA Suite: Rust duplicate detection

Tracks: #3 (parent #1)
Feature file: `features/duplicate_detection.feature`

## Scope

Verifies the duplicate-detection contract entirely through the command-line
interface. QA spawns the compiled binary as a subprocess, passes fixture
directories and flags, and observes only exit code, stdout, and stderr. QA does
not call any project API and does not inspect internal structures (parser, AST,
hash sets, inverted index).

The user-interface affordances exercised are the positional `<path>` arguments
and the flags `--threshold`, `--min-lines`, `--min-nodes`, `--format`, `--lang`,
and `--exclude`.

## Preconditions

- The release binary is built (`cargo build --release`) at
  `target/release/drywall`.
- Fixtures are created on disk under a QA-controlled temp directory (e.g.
  `./tmp/qa-dd/<case>/`); QA writes `.rs` files, never touches project internals.

## Fixture builders (QA-side, on disk only)

- **STRUCTURAL-TWIN**: two `.rs` files whose functions have identical control
  flow and statement structure but different identifier and literal text; each
  function is large enough to clear the default gates (≥ 4 lines, ≥ 20 nodes).
- **UNRELATED**: two `.rs` files with functions of different structure (e.g. an
  accumulating loop vs. a config-parsing match), each clearing the gates.
- **TINY-LINES**: a `.rs` file whose function is 3 source lines (below the
  4-line default) but otherwise a structural twin of a second file's function.
- **TINY-NODES**: a `.rs` file whose function has ~15 normalized nodes (below the
  20-node default) but otherwise a structural twin of a second file's function.
- **TWO-PAIRS**: a fixture set producing two distinct duplicate pairs with clearly
  different scores (e.g. ~0.95 and ~0.84) so ordering is observable.
- **UNPARSABLE**: a `.rs` file containing syntactically invalid Rust.

## User-visible workflows

### QA-1 — Structural twin reported as a duplicate pair (exit 1)
- Setup: STRUCTURAL-TWIN in `./tmp/qa-dd/twin/`.
- Input: `drywall ./tmp/qa-dd/twin`
- Expected exit code: `1`
- Expected stdout: one `DUPLICATE` block naming both fixture files with their
  line ranges; the `score=` value is ≥ `0.82`.
- Expected stderr: empty.

### QA-2 — Unrelated functions produce no pair (exit 0)
- Setup: UNRELATED in `./tmp/qa-dd/unrelated/`.
- Input: `drywall ./tmp/qa-dd/unrelated`
- Expected exit code: `0`
- Expected stdout: no `DUPLICATE` line (empty, or a "no duplicates" line if the
  implementation prints one — QA asserts the absence of any pair).
- Expected stderr: empty.

### QA-3 — Dogfood: drywall on its own source exits clean (exit 0)
- Input: `drywall ./src` run from the project root.
- Expected exit code: `0`
- Expected stdout: no `DUPLICATE` line.
- Expected stderr: empty.
- Rationale: the dogfood gate — drywall's own source must contain no pair that
  scores at or above the default threshold under the default gates.

### QA-4 — Function below the line gate is not a candidate (exit 0)
- Setup: TINY-LINES in `./tmp/qa-dd/tiny-lines/`.
- Input: `drywall ./tmp/qa-dd/tiny-lines`
- Expected exit code: `0`
- Expected stdout: no `DUPLICATE` line.
- Rationale: a function with fewer than 4 source lines is not a candidate, so no
  pair forms even though a structural twin exists.

### QA-5 — Function below the node gate is not a candidate (exit 0)
- Setup: TINY-NODES in `./tmp/qa-dd/tiny-nodes/`.
- Input: `drywall ./tmp/qa-dd/tiny-nodes`
- Expected exit code: `0`
- Expected stdout: no `DUPLICATE` line.

### QA-6 — Pairs sorted by score descending (deterministic)
- Setup: TWO-PAIRS in `./tmp/qa-dd/two-pairs/`.
- Input: `drywall ./tmp/qa-dd/two-pairs`
- Expected exit code: `1`
- Expected stdout: two `DUPLICATE` blocks; the block with the higher `score=`
  appears before the block with the lower `score=`.
- Determinism: running the same input twice yields byte-identical stdout.

### QA-7 — `--threshold` raises the reporting cutoff
- Setup: STRUCTURAL-TWIN scoring ~0.84 in `./tmp/qa-dd/thresh/`.
- Input A: `drywall --threshold 0.90 ./tmp/qa-dd/thresh`
  - Expected exit code: `0`; stdout has no `DUPLICATE` line.
- Input B: `drywall --threshold 0.80 ./tmp/qa-dd/thresh`
  - Expected exit code: `1`; stdout has one `DUPLICATE` block.

### QA-8 — `--min-lines` override qualifies a short function
- Setup: TINY-LINES (3-line twin) in `./tmp/qa-dd/min-lines/`.
- Input: `drywall --min-lines 2 ./tmp/qa-dd/min-lines`
- Expected exit code: `1`; stdout reports the pair that the default gate
  suppressed.

### QA-9 — `--min-nodes` override qualifies a small function
- Setup: TINY-NODES (~15-node twin) in `./tmp/qa-dd/min-nodes/`.
- Input: `drywall --min-nodes 10 ./tmp/qa-dd/min-nodes`
- Expected exit code: `1`; stdout reports the pair that the default gate
  suppressed.

### QA-10 — `--format json` with duplicates emits parseable JSON (exit 1)
- Setup: STRUCTURAL-TWIN in `./tmp/qa-dd/json/`.
- Input: `drywall --format json ./tmp/qa-dd/json`
- Expected exit code: `1`
- Expected stdout: valid JSON containing exactly one pair object with `score`,
  and `left`/`right` objects each carrying `file`, `start_line`, `end_line`,
  and `nodes`.
- Expected stderr: empty.

### QA-11 — `--format json` with no duplicates emits empty array (exit 0)
- Setup: UNRELATED in `./tmp/qa-dd/json-empty/`.
- Input: `drywall --format json ./tmp/qa-dd/json-empty`
- Expected exit code: `0`
- Expected stdout: valid JSON equal to `[]`.

### QA-12 — `--exclude` suppresses a matching path
- Setup: STRUCTURAL-TWIN where the right file lives under
  `./tmp/qa-dd/excl/vendor/`.
- Input: `drywall --exclude "**/vendor/**" ./tmp/qa-dd/excl`
- Expected exit code: `0`; stdout has no `DUPLICATE` line, because one member of
  the only candidate pair was excluded from scanning.

### QA-13 — `--lang rust` forces analysis of a non-standard extension
- Setup: STRUCTURAL-TWIN whose files use a non-`.rs` extension (e.g. `.rsx`) in
  `./tmp/qa-dd/lang/`.
- Input: `drywall --lang rust ./tmp/qa-dd/lang`
- Expected exit code: `1`; stdout reports the pair, confirming the forced
  language overrode extension-based detection.

### QA-14 — Nonexistent path is an error (exit 2)
- Input: `drywall ./tmp/qa-dd/does-not-exist`
- Expected exit code: `2`
- Expected stderr: non-empty diagnostic naming the unreadable path.
- Expected stdout: no `DUPLICATE` line.

### QA-15 — Unparsable source is an error (exit 2)
- Setup: UNPARSABLE in `./tmp/qa-dd/unparsable/`.
- Input: `drywall ./tmp/qa-dd/unparsable`
- Expected exit code: `2`
- Expected stderr: non-empty diagnostic.
- Rationale: distinguishes a parse failure (exit 2) from a clean no-duplicate
  run (exit 0).

## Observable states summary

| State | How QA observes it |
|---|---|
| Duplicates found | exit code `1` AND ≥ 1 `DUPLICATE` block (or ≥ 1 JSON pair object) |
| No duplicates | exit code `0` AND no `DUPLICATE` block (JSON: `[]`) |
| Error | exit code `2` AND non-empty stderr diagnostic |
| Deterministic output | identical input run twice yields byte-identical stdout |
| Sorted output | higher `score=` block precedes lower `score=` block |

## Out of scope for this suite

- JavaScript, TypeScript, and Python analysis (Rust target only this slice).
- `--lang` values other than `rust`.
- Cross-pair clustering or transitive grouping.
- Internal verification of normalization, hashing, or Jaccard math (covered by
  the project's unit tests, not by this CLI-only QA suite).
