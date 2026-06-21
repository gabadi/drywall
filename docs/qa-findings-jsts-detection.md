# QA Findings: jsts-detection

## Pre-handoff checks (re-verification after coder/hardender TSX fix)
- Unit tests: 104 passed (2 new: export_const_arrow_function_is_extracted, detect_lang_tsx_returns_tsx)
- Acceptance tests: 51 passed
- Property tests: 43 passed (7 new: tsx_extraction_deterministic, js/ts self-similarity, detect_lang coverage, should_scan_file JS/TS)
- Coverage: ≥90% lines (pass)
- CRAP: 62 functions, none exceed threshold 6
- DRY: drywall ./src exits 0, no duplicates
- Acceptance generator: regenerates logically identical output from IR (include path form and single-line vs multi-line dispatch formatting differ; no logical difference)

## Adversarial end-to-end QA results

All 13 QA procedures verified against the compiled binary using scratch fixtures:

- **QA-1** (TS structural twin): exit 1, DUPLICATE block with score=1.0000, stderr empty ✓
- **QA-2** (JS structural twin): exit 1, DUPLICATE block with score=1.0000, stderr empty ✓
- **QA-3** (function forms): arrow-function, class-method, exported-named-function all detected ✓; function-declaration covered by QA-1/QA-2 ✓
- **QA-4** (.ts auto-detected): covered by QA-1 ✓
- **QA-5** (.js auto-detected): covered by QA-2 ✓
- **QA-6** (.tsx auto-detected with JSX markup): exit 1, DUPLICATE score=1.0000 ✓ — LANGUAGE_TSX wired; DEFECT-1 resolved by e335dcf
- **QA-6** (.jsx auto-detected with JSX markup): exit 1, pair reported ✓
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
| QA-6 .tsx auto-detected (JSX markup) | grammar_is_auto_detected_...example_1 | ✓ AUTOMATED — fixture now uses JSX content; wired to LANGUAGE_TSX |
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

## Resolved defect

### DEFECT-1: RESOLVED — `.tsx` files with JSX markup now parse correctly

Fixed in e335dcf: `Lang::Tsx` variant added, wired to `tree_sitter_typescript::LANGUAGE_TSX`. `.tsx` extension maps to `Lang::Tsx` in `EXT_LANG_MAP`. Acceptance fixture for `grammar_is_auto_detected_from_file_extension_example_1` updated to use actual JSX content (would fail without TSX support). Adversarial re-verification: `accumulate_sum`-style JSX functions in `.tsx` files → DUPLICATE score=1.0000, exit 1.

## Specification coverage gaps (non-blocking)

### GAP-2: QA-12 (no cross-language pairing) has no automated scenario

The feature has no Gherkin scenario asserting that a .rs and .ts structural twin pair does NOT produce a DUPLICATE output. Verified manually (exit 0, no output), but a regression would go undetected by the acceptance suite.

### GAP-3: QA-7 control (no --lang → exit 0) not automated

The QA-7 control case (non-standard extension, no --lang, must be exit 0) is listed as optional in the qa.md but is not in the Gherkin. Verified manually.

### GAP-4: stderr empty not asserted in QA-1/QA-2/QA-7/QA-8/QA-9 scenarios

Same pattern as cli_surface QA gaps; accepted precedent from earlier tasks.

## Observation-harness note

No `observation-harness/` directory exists in this project. The `cli-surface` feature was verified and integrated without one, establishing project precedent that the acceptance harness covers CLI verification. Applying that precedent here.

## Disposition

All 104 unit + 51 acceptance + 43 property tests pass. All 13 adversarial QA scenarios pass including TSX with JSX markup (DEFECT-1 resolved). Remaining gaps (GAP-2, GAP-3, GAP-4) are specification coverage weaknesses — verified manually, documented as NOT AUTOMATED with reasons. Ready for integrator.
