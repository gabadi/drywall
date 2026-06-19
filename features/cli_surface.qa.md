# End-to-End QA Suite: CLI surface exclusions (built-in, gitignore, repeatable exclude)

Tracks: #4 (parent #1)
Feature file: `features/cli_surface.feature`

## Scope

Verifies the three-layer exclusion contract entirely through the command-line
interface: built-in directory exclusions, gitignore-awareness, and the
repeatable `--exclude` flag. QA spawns the compiled binary as a subprocess,
passes fixture directories and flags, and observes only exit code, stdout, and
stderr. QA does not call any project API and does not inspect internal
structures (directory walker, glob matcher, git invocation, parser, hash sets).

This suite covers only the exclusion behaviour that #4 adds on top of #3:
the always-on built-in excluded directories, default-on gitignore-awareness, and
the repeatability of `--exclude`. Reporting format, thresholds, size gates,
`--format`, `--lang`, and single-pattern `--exclude` are verified by the #3 QA
suite (`features/duplicate_detection.qa.md`) and are not re-verified here.

The user-interface affordances exercised are the positional `<path>` arguments
and the repeatable `--exclude` flag. Gitignore-awareness has no flag; QA
exercises it by controlling the on-disk git state of the fixture tree.

## Preconditions

- The release binary is built (`cargo build --release`) at
  `target/release/drywall`.
- Fixtures are created on disk under a QA-controlled temp directory (e.g.
  `./tmp/qa-cli/<case>/`); QA writes `.rs` files, never touches project
  internals.
- **Fixture visibility under gitignore-awareness**: because gitignore-awareness
  is default-on and delegates to the `git` CLI, QA must ensure fixture source
  files are NOT reported ignored by `git check-ignore`, except in the case that
  deliberately tests gitignore exclusion (QA-8). If the repo gitignores `tmp/`
  (common), QA must place fixtures outside any ignored path — e.g. an isolated
  scratch directory created with `mktemp -d` outside the repo, or a path the QA
  harness first confirms is not ignored via `git check-ignore`. A fixture
  silently swallowed by an ambient `.gitignore` would make a test pass for the
  wrong reason (zero files scanned → false "no duplicates").

## Fixture builders (QA-side, on disk only)

- **STRUCTURAL-TWIN**: two `.rs` files whose functions have identical control
  flow and statement structure but different identifier and literal text; each
  function clears the default gates (≥ 4 lines, ≥ 20 nodes) and scores ≥ 0.82.
- **HIDDEN-TWIN(dir)**: a STRUCTURAL-TWIN where the left file is at the scanned
  root (e.g. `src/alpha.rs`) and the right file is placed inside a built-in
  excluded directory named `dir` (e.g. `node_modules/beta.rs`). Without the
  exclusion the pair would score ≥ 0.82 and be reported.
- **GIT-FIXTURE**: a fixture tree that is its own git work tree (`git init`),
  with a `.gitignore` whose contents QA controls, so `git check-ignore` resolves
  deterministically for the fixture paths.
- **NO-GIT-FIXTURE**: a STRUCTURAL-TWIN in a directory that is NOT a git work
  tree (no `.git`, no ancestor repo), used to exercise the git-absent fallback.

## User-visible workflows

### QA-1 — Each built-in excluded directory hides its twin (exit 0)
Run once per built-in excluded directory name:
`.git`, `target`, `node_modules`, `__pycache__`, `vendor`, `dist`, `.next`.

- Setup: HIDDEN-TWIN(`<dir>`) under `<scratch>/auto/<dir>/`, with the left file
  at `<root>/src/alpha.rs` and the right twin at `<root>/<dir>/beta.rs`.
- Input: `drywall <root>/src` (scan only the non-excluded root) — and
  additionally `drywall <root>` (scan the parent that contains the excluded dir).
- Expected exit code: `0` for both invocations.
- Expected stdout: no `DUPLICATE` line.
- Expected stderr: empty.
- Rationale: the twin inside the built-in excluded directory is never scanned.
  The seven directory names are each a distinct case.

### QA-2 — Passing the parent does not re-include a built-in excluded subdir (exit 0)
- Setup: HIDDEN-TWIN(`node_modules`) under `<scratch>/parent/proj/`, twin at
  `proj/node_modules/beta.rs`, left at `proj/alpha.rs`.
- Input: `drywall <scratch>/parent/proj` (the parent `proj` is passed
  explicitly; `node_modules` is reachable only because its parent was passed).
- Expected exit code: `0`; stdout has no `DUPLICATE` line.
- Rationale: naming an ancestor does not opt a built-in excluded subdirectory
  back into scanning.

### QA-3 — A built-in excluded directory passed directly is still excluded (exit 0)
- Setup: STRUCTURAL-TWIN where BOTH files live inside the built-in excluded dir,
  e.g. `<scratch>/direct/node_modules/alpha.rs` and `.../node_modules/beta.rs`.
- Input: `drywall <scratch>/direct/node_modules` (the excluded directory is
  itself the path argument).
- Expected exit code: `0`; stdout has no `DUPLICATE` line.
- Rationale: built-in exclusion is by directory name at any segment and is not
  overridden by naming the directory on the command line.

### QA-4 — A twin outside built-in excluded directories is still reported (exit 1)
- Setup: STRUCTURAL-TWIN with both files under `<scratch>/visible/src/`, in a
  path confirmed not gitignored.
- Input: `drywall <scratch>/visible/src`
- Expected exit code: `1`; stdout reports one `DUPLICATE` block naming both
  files.
- Rationale: guards against over-exclusion — exclusion must not suppress
  ordinary source directories.

### QA-5 — Two `--exclude` patterns both take effect, left file excluded (exit 0)
- Setup: STRUCTURAL-TWIN at `<scratch>/repeat-a/src/alpha.rs` and `.../src/beta.rs`.
- Input:
  `drywall --exclude "**/alpha.rs" --exclude "**/beta.rs" <scratch>/repeat-a/src`
- Expected exit code: `0`; stdout has no `DUPLICATE` line.
- Rationale: the flag is repeatable and the patterns union — excluding either
  member of the only pair removes the pair.

### QA-6 — Two `--exclude` patterns, only one matches, twin removed (exit 0)
- Setup: STRUCTURAL-TWIN with the left file at `<scratch>/repeat-b/src/alpha.rs`
  and the right twin at `<scratch>/repeat-b/gen/beta.rs`.
- Input:
  `drywall --exclude "**/gen/**" --exclude "**/legacy/**" <scratch>/repeat-b/src <scratch>/repeat-b/gen`
- Expected exit code: `0`; stdout has no `DUPLICATE` line.
- Rationale: a file is excluded if it matches ANY supplied pattern; the
  `**/legacy/**` pattern matches nothing here, but `**/gen/**` removes the twin.

### QA-7 — Determinism of built-in exclusion output
- Setup: reuse the QA-1 `node_modules` fixture.
- Input: run `drywall <root>` twice.
- Expected: byte-identical stdout and identical exit code across both runs.
- Rationale: built-in exclusion is environment-independent and deterministic.

### QA-8 — A gitignored twin is not scanned when git is available (exit 0)
- Setup: GIT-FIXTURE under `<scratch>/git-ignored/`: `git init`; `.gitignore`
  containing `generated/`; left file `src/alpha.rs` (not ignored), right twin
  `generated/beta.rs` (ignored by the `.gitignore`). QA confirms
  `git check-ignore generated/beta.rs` reports it ignored before running drywall.
- Input: `drywall <scratch>/git-ignored/src` and also `drywall <scratch>/git-ignored`.
- Expected exit code: `0`; stdout has no `DUPLICATE` line.
- Expected stderr: empty.
- Rationale: with git available in a work tree, a gitignored path is excluded
  from scanning, so its twin forms no pair.

### QA-9 — A non-ignored twin in a git work tree is still reported (exit 1)
- Setup: GIT-FIXTURE under `<scratch>/git-visible/`: `git init`; a `.gitignore`
  that does NOT match the twin files; STRUCTURAL-TWIN at `src/alpha.rs` and
  `src/beta.rs`. QA confirms neither file is reported by `git check-ignore`.
- Input: `drywall <scratch>/git-visible/src`
- Expected exit code: `1`; stdout reports the pair.
- Rationale: gitignore-awareness must only subtract ignored paths; non-ignored
  source in a repo is still analyzed. Guards against over-exclusion via git.

### QA-10 — Gitignore-awareness silently no-ops when not in a repo (exit 1)
- Setup: NO-GIT-FIXTURE — a STRUCTURAL-TWIN in a scratch directory with NO `.git`
  and no ancestor git work tree (verify `git rev-parse` fails there).
- Input: `drywall <scratch>/no-git/src`
- Expected exit code: `1`; stdout reports the pair; stderr empty.
- Rationale: outside a work tree, gitignore resolution no-ops without error; only
  built-in and user exclusion apply, so the visible twin is still reported.

### QA-11 — Dogfood stays clean with gitignore-awareness on (exit 0)
- Input: `drywall ./src` run from the drywall project root (a git work tree with
  a `.gitignore`).
- Expected exit code: `0`; stdout has no `DUPLICATE` line; stderr empty.
- Rationale: the regression gate — adding gitignore-awareness must not change the
  dogfood result. `./src` is tracked (not gitignored), so it remains fully
  scanned and continues to contain no above-threshold pair.

## Observable states summary

| State | How QA observes it |
|---|---|
| Twin excluded (built-in / gitignore / `--exclude`) | exit code `0` AND no `DUPLICATE` block |
| Twin visible (not excluded) | exit code `1` AND ≥ 1 `DUPLICATE` block |
| Git-absent fallback | in a non-repo, a visible twin is still reported; stderr empty |
| Built-in determinism | identical input run twice yields byte-identical stdout |

## Out of scope for this suite

- Reporting format, exit code 2, thresholds, size gates, `--format`, `--lang`,
  and single-pattern `--exclude` (verified by `features/duplicate_detection.qa.md`).
- A flag to disable or override built-in exclusion or gitignore-awareness (none
  provided in this slice).
- Exact gitignore precedence and negation semantics (delegated to the git CLI;
  drywall adopts git's answer verbatim, so QA asserts only that git's verdict is
  honored, not how git computes it).
- JavaScript, TypeScript, and Python analysis (Rust target only).
- Internal verification of the directory walker, glob matcher, or git invocation
  (covered by the project's unit tests, not by this CLI-only QA suite).
