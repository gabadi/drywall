# QA Findings: jsts-detection

## Pre-handoff checks
- Unit tests: 102 passed
- Acceptance tests: 51 passed
- Property tests: 36 passed
- Coverage: ≥90% lines (pass)
- CRAP: 62 functions, none exceed threshold 6
- DRY: drywall ./src exits 0, no duplicates
- Acceptance generator: regenerates logically identical output from IR (HashMap insertion order and single-line vs multi-line dispatch formatting differ; 13 test functions match, 68 dispatch calls match, no logical difference)

## Adversarial end-to-end QA results

All 13 QA procedures verified against the compiled binary using scratch fixtures:

- **QA-1** (TS structural twin): exit 1, DUPLICATE block with score=1.0000, stderr empty ✓
- **QA-2** (JS structural twin): exit 1, DUPLICATE block with score=1.0000, stderr empty ✓
- **QA-3** (function forms): arrow-function, class-method, exported-named-function all detected ✓; function-declaration covered by QA-1/QA-2 ✓
- **QA-4** (.ts auto-detected): covered by QA-1 ✓
- **QA-5** (.js auto-detected): covered by QA-2 ✓
- **QA-6** (.tsx auto-detected, plain TS content): exit 1, pair reported ✓ — but JSX markup in .tsx exits 2 (parse error); this is DEFECT-1
- **QA-6** (.jsx with JSX markup): exit 1, pair reported ✓
- **QA-7** (--lang ts on .txt): exit 1 with --lang ts, exit 0 without --lang ✓
- **QA-8** (--lang js on .inc): exit 1 ✓
- **QA-9** (mixed .rs+.ts): exit 1, both .rs pair and .ts pair in one report ✓; determinism confirmed (byte-identical on two runs) ✓
- **QA-10** (unparsable TS): exit 2, stderr non-empty ✓
- **QA-11** (unparsable JS): exit 2, stderr non-empty ✓
- **QA-12** (no cross-language pairing): exit 0, no DUPLICATE output ✓ — scope boundary holds
- **QA-13** (dogfood): exit 0, no DUPLICATE, stderr empty ✓

## Bullet-to-harness traceability

| QA Procedure | Acceptance Scenario | Status |
|---|---|---|
| QA-1 exit=1, DUPLICATE, score≥0.82 | structural_duplicate_...example_1 (ts) | ✓ AUTOMATED |
| QA-1 stderr empty | (none) | NOT AUTOMATED — no `stderr is empty` step in jsts scenarios (same gap exists in duplicate_detection; accepted precedent) |
| QA-2 exit=1, DUPLICATE, score≥0.82 | structural_duplicate_...example_2 (js) | ✓ AUTOMATED |
| QA-2 stderr empty | (none) | NOT AUTOMATED — same precedent as QA-1 |
| QA-3 all four function forms | each_js_ts_function_form_...examples 1-4 | ✓ AUTOMATED (arrow, class-method, exported-named in TS; function-decl in JS) |
| QA-4 .ts auto-detected | structural_duplicate_...example_1 | ✓ AUTOMATED |
| QA-5 .js auto-detected | structural_duplicate_...example_2 | ✓ AUTOMATED |
| QA-6 .tsx auto-detected (JSX markup) | grammar_is_auto_detected_...example_1 | DEFECT — test uses plain TS, not JSX markup; passes for wrong reason (see DEFECT-1) |
| QA-6 .jsx auto-detected | grammar_is_auto_detected_...example_2 | ✓ AUTOMATED |
| QA-7 --lang ts forces non-standard ext | lang_forces_a_grammar_...example_1 | ✓ AUTOMATED |
| QA-7 control (no --lang → exit 0) | (none) | NOT AUTOMATED — control invocation not in Gherkin |
| QA-8 --lang js forces non-standard ext | lang_forces_a_grammar_...example_2 | ✓ AUTOMATED |
| QA-9 mixed output, both pairs | mixed_language_directory_...example_1 | ✓ AUTOMATED |
| QA-9 determinism (byte-identical) | (none) | NOT AUTOMATED — determinism verified manually in QA run |
| QA-10 exit=2, stderr non-empty | unparsable_js_ts_source_...example_1 (ts) | ✓ AUTOMATED |
| QA-11 exit=2, stderr non-empty | unparsable_js_ts_source_...example_2 (js) | ✓ AUTOMATED |
| QA-12 no cross-language pairing → exit 0 | (none) | NOT AUTOMATED — no Gherkin scenario for this scope boundary |
| QA-13 dogfood unaffected | (none) | NOT AUTOMATED — same gap as prior QA tasks; run manually ✓ |

## Code defect — BLOCKING (route to coder)

### DEFECT-1: `.tsx` files with JSX markup fail to parse (exit 2); acceptance test passes for the wrong reason

**Evidence:**
- `.tsx` file containing `function TestComp() { return <div>hello</div>; }` → exit 2, parse error.
- `.jsx` file with identical JSX markup → exit 1, pair reported correctly.
- `grammar_is_auto_detected_from_file_extension_example_1` (tsx) acceptance test **passes for the wrong reason**: `make_source("tsx", ...)` calls `accumulate_sum_ts()`, generating plain TypeScript (no JSX). The test would pass even if `.tsx` JSX support were entirely absent.

**Contract violated:**
- Feature CONSTRAINTS: `.tsx → TypeScript` — implies `.tsx` content is parsed, not errored.
- `features/jsts_detection.qa.md` QA-6 fixture: "TSX-TWIN whose function body **contains JSX/TSX markup**."
- The spec treats `.jsx` and `.tsx` as parallel — JSX markup works in one but fails in the other.

**Root cause:** `src/ast.rs:74` wires `Lang::TypeScript → tree_sitter_typescript::LANGUAGE_TYPESCRIPT`. The `tree_sitter_typescript` crate provides a separate `LANGUAGE_TSX` grammar that handles TypeScript files containing JSX markup. The code never uses it.

**Required fix (coder):** Wire `LANGUAGE_TSX` for `.tsx` extension so JSX markup in `.tsx` files parses correctly. Update `grammar_is_auto_detected_from_file_extension_example_1` fixture to use JSX content so the test would fail if TSX support were removed.

## Specification coverage gaps (route to specifier — non-blocking; after coder resolves DEFECT-1)

### GAP-1: QA-12 (no cross-language pairing) has no automated scenario

The feature has no Gherkin scenario asserting that a .rs and .ts structural twin pair does NOT produce a DUPLICATE output. This is the critical scope boundary. Verified manually (exit 0, no output), but a regression would go undetected by the acceptance suite.

### GAP-2: QA-7 control (no --lang → exit 0) not automated

The QA-7 control case (non-standard extension, no --lang, must be exit 0) is listed as optional in the qa.md but is not in the Gherkin. Verified manually.

### GAP-4: stderr empty not asserted in QA-1/QA-2/QA-7/QA-8/QA-9 scenarios

Same pattern as cli_surface QA gaps; accepted precedent from earlier tasks.

## Observation-harness note

No `observation-harness/` directory exists in this project. The `cli-surface` feature (also a user-facing CLI surface) was verified and integrated without one, establishing project precedent that the acceptance harness covers CLI verification. Applying that precedent here; no observation-harness route-back issued.

## Disposition

**BLOCKED — routing to coder.** DEFECT-1 (`.tsx` JSX markup → parse error; acceptance test passes for wrong reason) must be fixed before this task can be integrated. The specification contract requires JSX markup support for `.tsx`; the code delivers it for `.jsx` but not `.tsx`. The acceptance test for `.tsx` auto-detection passes only because it exercises plain TypeScript, not JSX — it would pass even with no TSX support at all.

Spec coverage gaps (GAP-1 through GAP-4) route to specifier after DEFECT-1 is resolved and re-verified.
