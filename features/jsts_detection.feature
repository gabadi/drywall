# acceptance-mutation-manifest-begin
# {"version":1,"tested_at":"2026-06-21T10:26:07.778776Z","feature_name":"JavaScript and TypeScript duplicate detection","feature_path":"features/jsts_detection.feature","background_hash":"74234e98afe7498fb5daf1f36ac2d78acc339464f950703b8c019892f982b90b","implementation_hash":"unknown","scenarios":[]}
# acceptance-mutation-manifest-end

Feature: JavaScript and TypeScript duplicate detection

  # TRACKING: #5 (parent #1)
  #
  # CONTRACT:
  #   drywall [OPTIONS] <path>...
  #   request:  path      — string, one or more filesystem paths (file or directory) to scan
  #   request:  --lang    — enum {rust, js, ts}, force a grammar regardless of file extension;
  #                         default auto-detect by extension. js forces JavaScript, ts forces TypeScript.
  #   response exit=0: zero duplicate pairs — when no qualifying JS/TS pair scores >= threshold
  #   response exit=1: one or more duplicate pairs in the requested format — when at least one
  #                    JS/TS pair scores >= threshold
  #   response exit=2: error text on stderr — when a JS/TS target file fails to parse
  #
  #   This slice adds JavaScript and TypeScript as analysis targets on top of the Rust
  #   target specified by features/duplicate_detection.feature (#3). The shared pipeline
  #   (normalization to _ID/_LIT, subtree hashing, Jaccard scoring, qualification gates,
  #   threshold, sorting, exit codes, text/json output) is identical to #3 and is NOT
  #   redefined here. Only what is NEW for JS/TS is specified: the recognized JS/TS
  #   function forms, extension-based grammar auto-detection, the js/ts values of --lang,
  #   and mixed-language single-output. Built-in/gitignore/--exclude exclusion is owned by
  #   features/cli_surface.feature (#4) and is not redefined here.
  #
  #   text/json output per pair is exactly as specified by #3 (mirrors dry4go); JS/TS files
  #   appear in the same DUPLICATE blocks / JSON pair objects as Rust files.
  #
  # CONSTRAINTS:
  #   - Extension to grammar auto-detection (no --lang): .rs -> Rust, .js/.jsx -> JavaScript,
  #     .ts/.tsx -> TypeScript. JSX is parsed by the JavaScript grammar, TSX by TypeScript.
  #   - Recognized JS/TS function forms (each a distinct analysis unit): function declarations,
  #     arrow functions bound to a `const` or `let` binding, class methods, and exported named
  #     functions (the same forms reached through an `export`).
  #   - Normalization maps each grammar's identifier nodes to _ID and literal nodes to _LIT,
  #     so two JS/TS functions with identical structure but different identifier/literal text
  #     normalize identically and score 1.0 — exactly as for Rust.
  #   - --lang forces ONE grammar for every scanned file this invocation, overriding extension
  #     detection: js forces JavaScript, ts forces TypeScript. Accepted values are rust, js, ts.
  #   - In a mixed-language directory with no --lang, each file is parsed with the grammar its
  #     own extension selects, and pairs from all languages are merged into ONE sorted output.
  #   - Qualification gates, --threshold, --min-lines, --min-nodes, --format, sorting, and the
  #     three exclusion layers behave identically to #3/#4 for JS/TS files.
  #
  # SEQUENCING: none
  #
  # NFR:
  #   - Output is deterministic: same inputs and options yield byte-identical output across runs,
  #     including the merged ordering of a mixed-language directory.
  #   - Invocation exits promptly; no hang and no interactive prompt.
  #   - Error (exit 2) is distinguishable from clean-no-duplicates (exit 0): exit 2 writes a
  #     diagnostic to stderr; exit 0 writes no diagnostic.
  #
  # SIDE EFFECTS:
  #   - Adds js and ts as accepted --lang values (previously only rust). Adds .js, .jsx, .ts,
  #     .tsx to the set of extensions scanned by default.
  #
  # SCOPE:
  #   - Does NOT: detect duplication across languages (a Rust function is never paired with a
  #     JS/TS function; comparison is within a grammar's hash space only).
  #   - Does NOT: analyze Python sources or accept --lang py (tracked separately).
  #   - Does NOT: add a jsx or tsx value to --lang; .jsx/.tsx are auto-detected or reached by
  #     forcing js/ts.
  #   - Does NOT: redefine reporting format, exit codes, thresholds, size gates, --format, or
  #     the exclusion layers (owned by #3 and #4).
  #   - ASSUMED: a "function" for JS/TS means any recognized JS/TS function form; the term is
  #              not limited to constructs JS calls "functions" (a class method qualifies).
  #   - ASSUMED: line ranges are 1-based and inclusive, identical to #3.
  #   - ASSUMED: when --lang is supplied, extension auto-detection is bypassed for every file.
  #
  # UX INTENT: none
  # Design artifacts: none

  # jsts-detection-1
  # Two JS/TS files whose functions share structure but differ only in identifiers are
  # reported as a pair, proving the shared normalization pipeline runs under each grammar.
  Scenario Outline: Structural duplicate in a JS or TS source pair is reported
    Given a "<ext>" file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<ext>" file "<right_file>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"
    And the reported score is at least "<min_score>"

    Examples:
      | ext | left_file    | right_file   | structure      | left_ids | right_ids | args  | exit_code | min_score |
      | ts  | src/alpha.ts | src/beta.ts  | accumulate_sum | a,b,sum  | x,y,total | ./src | 1         | 0.82      |
      | js  | src/alpha.js | src/beta.js  | accumulate_sum | a,b,sum  | x,y,total | ./src | 1         | 0.82      |

  # jsts-detection-2
  # Each recognized JS/TS function form is a reportable analysis unit: a structural twin
  # whose function uses the named form is detected as a pair. The form is carried by the
  # "<structure>" parameter so the step phrasing stays identical to jsts-detection-1.
  Scenario Outline: Each JS/TS function form is a reportable analysis unit
    Given a "<ext>" file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<ext>" file "<right_file>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"

    Examples:
      | ext | left_file    | right_file  | structure               | left_ids | right_ids | args  | exit_code |
      | js  | src/alpha.js | src/beta.js | function-declaration    | a,b,sum  | x,y,total | ./src | 1         |
      | ts  | src/alpha.ts | src/beta.ts | arrow-function          | a,b,sum  | x,y,total | ./src | 1         |
      | ts  | src/alpha.ts | src/beta.ts | class-method            | a,b,sum  | x,y,total | ./src | 1         |
      | js  | src/alpha.js | src/beta.js | exported-named-function | a,b,sum  | x,y,total | ./src | 1         |

  # jsts-detection-3
  # A file's grammar is auto-detected from its extension when no --lang is given; the
  # twin is detected, proving the correct grammar parsed it. The extension is the
  # discriminating variable.
  Scenario Outline: Grammar is auto-detected from file extension
    Given a "<ext>" file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<ext>" file "<right_file>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"

    Examples:
      | ext | left_file     | right_file   | structure      | left_ids | right_ids | args  | exit_code |
      | tsx | src/alpha.tsx | src/beta.tsx | accumulate_sum | a,b,sum  | x,y,total | ./src | 1         |
      | jsx | src/alpha.jsx | src/beta.jsx | accumulate_sum | a,b,sum  | x,y,total | ./src | 1         |

  # jsts-detection-4
  # --lang forces one grammar regardless of extension; a non-standard extension is parsed
  # as the forced language and the twin is reported. The forced language and the
  # non-standard extension are the discriminating variables.
  Scenario Outline: --lang forces a grammar on a non-standard extension
    Given a "<ext>" file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<ext>" file "<right_file>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"

    Examples:
      | ext | left_file     | right_file   | structure      | left_ids | right_ids | args            | exit_code |
      | ts  | src/alpha.txt | src/beta.txt | accumulate_sum | a,b,sum  | x,y,total | --lang ts ./src | 1         |
      | js  | src/alpha.inc | src/beta.inc | accumulate_sum | a,b,sum  | x,y,total | --lang js ./src | 1         |

  # jsts-detection-5
  # A directory mixing languages is analyzed file-by-file with each file's own grammar,
  # and pairs from every language appear in a single merged output. The two extensions
  # are the discriminating variables.
  Scenario Outline: Mixed-language directory yields one merged output
    Given a "<left_ext>" file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<left_ext>" file "<left_twin>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    And a "<right_ext>" file "<right_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<right_ext>" file "<right_twin>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<left_twin>"
    And stdout reports a duplicate pair for "<right_file>" and "<right_twin>"

    Examples:
      | left_ext | right_ext | left_file | left_twin | right_file | right_twin | structure      | left_ids | right_ids | args  | exit_code |
      | rs       | ts        | src/a.rs  | src/b.rs  | src/c.ts   | src/d.ts   | accumulate_sum | a,b,sum  | x,y,total | ./src | 1         |

  # jsts-detection-6
  # A JS/TS file that fails to parse is an error (exit 2) with a stderr diagnostic,
  # identical to the Rust parse-error contract in #3.
  Scenario Outline: Unparsable JS/TS source is an error
    Given a "<ext>" file "<file>" containing source that fails to parse
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stderr is not empty

    Examples:
      | ext | file         | args  | exit_code |
      | ts  | src/broken.ts | ./src | 2         |
      | js  | src/broken.js | ./src | 2         |
