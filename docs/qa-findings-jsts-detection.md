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
- **QA-6** (.tsx auto-detected, plain TS content): exit 1, pair reported ✓; JSX markup in .tsx files not supported (see GAP-1)
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
| QA-6 .tsx auto-detected | grammar_is_auto_detected_...example_1 | ✓ AUTOMATED (plain TS content in .tsx — see GAP-1) |
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

## Specification coverage gaps (route to specifier)

### GAP-1: QA-6 TSX-TWIN with JSX markup fails to parse (specification vs implementation mismatch)

The QA-6 procedure specifies a "TSX-TWIN" whose "function body contains JSX/TSX markup." In practice:

- `.tsx` files with actual JSX markup (`return <div>hello</div>`) exit with code 2 (parse error).
- The implementation uses `LANGUAGE_TYPESCRIPT` (tree-sitter TypeScript grammar, which does not support JSX syntax). The tree-sitter TypeScript crate provides a separate `LANGUAGE_TSX` for TSX files.
- The acceptance test for jsts-detection-3 (tsx) uses plain TypeScript content in `.tsx` files and passes, because the TypeScript grammar parses TypeScript without JSX markup.
- JSX in `.jsx` files parses correctly (JavaScript grammar supports JSX).

**What this means:** The contract says "TSX by TypeScript grammar" and the grammar is wired, but JSX markup in `.tsx` is silently a parse error (exit 2) rather than being analyzed. The qa.md TSX-TWIN fixture definition as written cannot be exercised as specified.

This is a specification coverage weakness. The code itself is consistent (it uses LANGUAGE_TYPESCRIPT for .tsx), but the spec/QA-6 fixture description implies markup support that doesn't exist. Routes to specifier for a coverage decision: either (a) narrow the QA-6 TSX-TWIN definition to TypeScript-only content (no JSX markup), or (b) add TSX grammar support via `LANGUAGE_TSX`.

### GAP-2: QA-12 (no cross-language pairing) has no automated scenario

The feature has no Gherkin scenario asserting that a .rs and .ts structural twin pair does NOT produce a DUPLICATE output. This is the critical scope boundary. Verified manually (exit 0, no output), but a regression would go undetected by the acceptance suite.

### GAP-3: QA-7 control (no --lang → exit 0) not automated

The QA-7 control case (non-standard extension, no --lang, must be exit 0) is listed as optional in the qa.md but is not in the Gherkin. Verified manually.

### GAP-4: stderr empty not asserted in QA-1/QA-2/QA-7/QA-8/QA-9 scenarios

Same pattern as cli_surface QA gaps; accepted precedent from earlier tasks.

## Disposition

All code is correct, all tests pass, all adversarial scenarios pass. Gaps above are specification coverage weaknesses in the accepted feature file — not code defects. Routing to specifier for coverage decisions on GAP-1 and GAP-2.
