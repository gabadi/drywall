# mutation-stamp: sha256=903834206bee5691c1277c467a1b98b4c2d0650c962e077f3537f31ba2f157d7
# acceptance-mutation-manifest-begin
# {"version":1,"tested_at":"2026-06-17T07:08:53.383254Z","feature_name":"Rust duplicate detection","feature_path":"features/duplicate_detection.feature","background_hash":"74234e98afe7498fb5daf1f36ac2d78acc339464f950703b8c019892f982b90b","implementation_hash":"unknown","scenarios":[{"index":0,"name":"Structural duplicate across files is reported","scenario_hash":"4d458c8b756e70694198ebeb1b2a35f20626b8a8329d585200d6906da265209e","mutation_count":8,"result":{"Total":8,"Killed":8,"Survived":0,"Errors":0},"tested_at":"2026-06-17T07:08:53.383254Z"},{"index":1,"name":"Unrelated functions produce no pair","scenario_hash":"82a7611bc6e2d0d03457de5a2b4154d229d74442d29c3240068ea432029c2400","mutation_count":6,"result":{"Total":6,"Killed":6,"Survived":0,"Errors":0},"tested_at":"2026-06-17T07:08:53.383254Z"},{"index":2,"name":"Dogfood run on own source exits clean","scenario_hash":"411ac28f3478b46b00b461f266b91230df397ba4d219edb65734604fee50bec2","mutation_count":2,"result":{"Total":2,"Killed":2,"Survived":0,"Errors":0},"tested_at":"2026-06-17T07:08:53.383254Z"},{"index":3,"name":"Function below a size gate is not a candidate","scenario_hash":"6de8f7d671be182da007ffdc49ddbee35c3eaa65aa66eaf1e8b2f53bc1e623c0","mutation_count":12,"result":{"Total":12,"Killed":12,"Survived":0,"Errors":0},"tested_at":"2026-06-17T07:08:53.383254Z"},{"index":4,"name":"Pairs are sorted by score descending","scenario_hash":"89fce39cd0f5e4067477fe285beb06774fed27e0c17da87103823a649c50f658","mutation_count":4,"result":{"Total":4,"Killed":4,"Survived":0,"Errors":0},"tested_at":"2026-06-17T07:08:53.383254Z"},{"index":5,"name":"Threshold flag gates reporting","scenario_hash":"99a82c43951b4153ae6bdb35c5c463a23c12dac69cf5e62d3795d09d21693f28","mutation_count":8,"result":{"Total":8,"Killed":8,"Survived":0,"Errors":0},"tested_at":"2026-06-17T07:08:53.383254Z"},{"index":6,"name":"Size-gate flags override defaults","scenario_hash":"142c8adae3391f1df86aa2518e0d48656cb18481721934c01437998a30654030","mutation_count":14,"result":{"Total":14,"Killed":14,"Survived":0,"Errors":0},"tested_at":"2026-06-17T07:08:53.383254Z"},{"index":7,"name":"JSON output format","scenario_hash":"675d1547bb4c256b6888eeeaeb55c3d69b0c2dfe20464d083a313045cfb95851","mutation_count":6,"result":{"Total":6,"Killed":6,"Survived":0,"Errors":0},"tested_at":"2026-06-17T07:08:53.383254Z"},{"index":8,"name":"Unreadable or unparsable input is an error","scenario_hash":"c1594e878776f6683f83a83c74916c2f18a0a066126a95b3dc8e6531cf3848c4","mutation_count":6,"result":{"Total":6,"Killed":6,"Survived":0,"Errors":0},"tested_at":"2026-06-17T07:08:53.383254Z"}]}
# acceptance-mutation-manifest-end

Feature: Rust duplicate detection

  # TRACKING: #3 (parent #1)
  #
  # CONTRACT:
  #   drywall [OPTIONS] <path>...
  #   request:  path        — string, one or more filesystem paths (file or directory) to scan
  #   request:  --threshold — float in [0.0, 1.0], Jaccard cutoff for reporting a pair; default 0.82
  #   request:  --min-lines — integer, minimum source lines for a function to qualify; default 4
  #   request:  --min-nodes — integer, minimum normalized AST nodes for a function to qualify; default 20
  #   request:  --format    — enum {text, json}, output encoding; default text
  #   request:  --lang      — enum {rust}, force a language regardless of file extension; default auto-detect
  #   request:  --exclude   — glob string, repeatable, paths matching are not scanned
  #   response exit=0: text or json output containing zero duplicate pairs — when no qualifying pair scores >= threshold
  #   response exit=1: one or more duplicate pairs in the requested format — when at least one pair scores >= threshold
  #   response exit=2: error text on stderr — when a path is unreadable or a target file fails to parse
  #
  #   text output per pair (mirrors dry4go):
  #     DUPLICATE score=<score>
  #       <left_file>:<left_start>-<left_end>
  #       <right_file>:<right_start>-<right_end>  (<left_nodes> nodes / <right_nodes> nodes)
  #   where <left_nodes>/<right_nodes> are each function's own normalized AST node count.
  #
  #   json output: one JSON object per pair (or array of objects) with fields
  #     score, left{file,start_line,end_line,nodes}, right{file,start_line,end_line,nodes}.
  #
  # CONSTRAINTS:
  #   - A function qualifies as a candidate only if it has >= --min-lines source lines
  #     AND >= --min-nodes normalized AST nodes. Both gates must pass.
  #   - Recognized Rust function forms: `fn` declarations, `impl` block methods,
  #     closures bound to a `let` binding, and trait-implementation methods.
  #   - Normalization maps identifier nodes to _ID and literal nodes to _LIT; operators,
  #     keywords, and structure are preserved. Two functions with identical structure but
  #     different identifier/literal text normalize identically.
  #   - A pair is reported only when its Jaccard similarity is >= --threshold AND both
  #     member functions pass the gates.
  #   - Output pairs are sorted by score descending. Equal scores are tie-broken
  #     deterministically by (left file path, left start line, right file path, right start line)
  #     ascending, so identical input always yields byte-identical output.
  #   - Jaccard similarity = |A intersect B| / |A union B| over each function's set of
  #     normalized subtree hashes.
  #
  # SEQUENCING: none
  #
  # NFR:
  #   - Output is deterministic: the same inputs and options yield byte-identical output
  #     across runs (required for acceptance and mutation testing).
  #   - Invocation exits promptly; no hang and no interactive prompt.
  #   - Error (exit 2) is distinguishable from clean-no-duplicates (exit 0): exit 2 writes
  #     a diagnostic to stderr; exit 0 writes no diagnostic.
  #
  # SIDE EFFECTS: none
  #
  # SCOPE:
  #   - Does NOT: analyze JavaScript, TypeScript, or Python sources (Rust target only).
  #   - Does NOT: support languages other than rust for --lang in this slice.
  #   - Does NOT: detect duplication across granularities other than function-level.
  #   - Does NOT: deduplicate or cluster beyond pairwise reporting (no transitive grouping).
  #   - ASSUMED: a "duplicate pair" is an unordered pair of distinct functions; a function is
  #              never paired with itself, and each unordered pair is reported at most once.
  #   - ASSUMED: line ranges are 1-based and inclusive of the function's first and last source line.
  #   - ASSUMED: when --format json reports zero pairs, output is an empty JSON array `[]`.
  #
  # UX INTENT: none
  # Design artifacts: none

  # duplicate-detection-1
  # Structurally identical functions differing only in identifiers are reported as a pair.
  Scenario Outline: Structural duplicate across files is reported
    Given a Rust file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a Rust file "<right_file>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"
    And the reported score is at least "<min_score>"

    Examples:
      | left_file       | right_file       | structure      | left_ids | right_ids | args  | exit_code | min_score |
      | src/alpha.rs    | src/beta.rs      | accumulate_sum | a,b,sum  | x,y,total | ./src | 1         | 0.82      |

  # duplicate-detection-2
  # Unrelated functions share no structure, so no pair is reported and the run is clean.
  Scenario Outline: Unrelated functions produce no pair
    Given a Rust file "<left_file>" containing a function with structure "<left_structure>"
    And a Rust file "<right_file>" containing a function with structure "<right_structure>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And no duplicate pair is reported

    Examples:
      | left_file    | right_file  | left_structure | right_structure | args  | exit_code |
      | src/alpha.rs | src/beta.rs | accumulate_sum | parse_config    | ./src | 0         |

  # duplicate-detection-3
  # The dogfood gate: drywall on its own source reports no duplicates and exits 0.
  Scenario Outline: Dogfood run on own source exits clean
    Given the drywall project source directory at "<path>"
    When I run drywall with the arguments "<path>"
    Then the exit code is "<exit_code>"
    And no duplicate pair is reported

    Examples:
      | path  | exit_code |
      | ./src | 0         |

  # duplicate-detection-4
  # A function below a size gate is not a candidate, so no pair is reported even when twins exist.
  Scenario Outline: Function below a size gate is not a candidate
    Given a Rust file "<left_file>" containing a function with "<lines>" source lines and "<nodes>" normalized nodes
    And a Rust file "<right_file>" containing a structurally identical function
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And no duplicate pair is reported

    Examples:
      | left_file    | right_file  | lines | nodes | args  | exit_code |
      | src/alpha.rs | src/beta.rs | 3     | 25    | ./src | 0         |
      | src/alpha.rs | src/beta.rs | 6     | 15    | ./src | 0         |

  # duplicate-detection-5
  # Reported pairs are ordered by score descending with a deterministic tie-break.
  Scenario Outline: Pairs are sorted by score descending
    Given a duplicate pair scoring "<high_score>" and a duplicate pair scoring "<low_score>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And the pair scoring "<high_score>" is reported before the pair scoring "<low_score>"

    Examples:
      | high_score | low_score | args  | exit_code |
      | 0.95       | 0.84      | ./src | 1         |

  # duplicate-detection-6
  # --threshold raises or lowers the reporting cutoff; a pair below the cutoff is suppressed.
  Scenario Outline: Threshold flag gates reporting
    Given a duplicate pair scoring "<pair_score>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And the pair is reported equals "<reported>"

    Examples:
      | pair_score | args                     | exit_code | reported |
      | 0.84       | --threshold 0.90 ./src   | 0         | false    |
      | 0.84       | --threshold 0.80 ./src   | 1         | true     |

  # duplicate-detection-7
  # --min-lines and --min-nodes override the default gates, qualifying or disqualifying a function.
  Scenario Outline: Size-gate flags override defaults
    Given a Rust file "<left_file>" containing a function with "<lines>" source lines and "<nodes>" normalized nodes
    And a Rust file "<right_file>" containing a structurally identical function
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And the pair is reported equals "<reported>"

    Examples:
      | left_file    | right_file  | lines | nodes | args                                  | exit_code | reported |
      | src/alpha.rs | src/beta.rs | 3     | 25    | --min-lines 2 ./src                   | 1         | true     |
      | src/alpha.rs | src/beta.rs | 6     | 15    | --min-nodes 10 ./src                  | 1         | true     |

  # duplicate-detection-8
  # --format json emits machine-readable pairs; an empty result is the empty array.
  Scenario Outline: JSON output format
    Given a Rust source set producing "<pair_count>" duplicate pairs
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout is valid JSON
    And the JSON contains "<pair_count>" pair objects

    Examples:
      | pair_count | args                | exit_code |
      | 0          | --format json ./src | 0         |
      | 1          | --format json ./src | 1         |

  # duplicate-detection-9
  # An unreadable path or unparsable source is an error, distinct from a clean run.
  Scenario Outline: Unreadable or unparsable input is an error
    Given the input condition "<condition>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stderr is not empty

    Examples:
      | condition             | args             | exit_code |
      | path does not exist   | ./does-not-exist | 2         |
      | source fails to parse | ./src            | 2         |
