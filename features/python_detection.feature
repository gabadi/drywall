# acceptance-mutation-manifest-begin
# {"version":1,"tested_at":"2026-06-21T17:59:58.023116Z","feature_name":"Python duplicate detection","feature_path":"features/python_detection.feature","background_hash":"74234e98afe7498fb5daf1f36ac2d78acc339464f950703b8c019892f982b90b","implementation_hash":"unknown","scenarios":[]}
# acceptance-mutation-manifest-end

Feature: Python duplicate detection

  # TRACKING: #6 (parent #1)
  #
  # CONTRACT:
  #   drywall [OPTIONS] <path>...
  #   request:  path      — string, one or more filesystem paths (file or directory) to scan
  #   request:  --lang    — enum {rust, js, ts, py}, force a grammar regardless of file
  #                         extension; default auto-detect by extension. py forces Python.
  #   response exit=0: zero duplicate pairs — when no qualifying Python pair scores >= threshold
  #   response exit=1: one or more duplicate pairs in the requested format — when at least one
  #                    Python pair scores >= threshold
  #   response exit=2: error text on stderr — when a Python target file fails to parse
  #
  #   This slice adds Python as an analysis target on top of the Rust target
  #   (features/duplicate_detection.feature, #3) and the JS/TS target
  #   (features/jsts_detection.feature, #5). The shared pipeline (normalization to
  #   _ID/_LIT, subtree hashing, Jaccard scoring, qualification gates, threshold,
  #   sorting, exit codes, text/json output) is identical to #3 and is NOT redefined
  #   here. Only what is NEW for Python is specified: the recognized Python function
  #   forms, .py extension grammar auto-detection, the py value of --lang, and
  #   mixed-language single-output spanning Python. Built-in/gitignore/--exclude
  #   exclusion is owned by features/cli_surface.feature (#4) and is not redefined here.
  #
  #   text/json output per pair is exactly as specified by #3 (mirrors dry4go); Python
  #   files appear in the same DUPLICATE blocks / JSON pair objects as Rust and JS/TS files.
  #
  # CONSTRAINTS:
  #   - Extension to grammar auto-detection (no --lang): .py -> Python. (The .rs, .js/.jsx,
  #     and .ts/.tsx mappings are owned by #3/#5 and are unchanged.)
  #   - Recognized Python function forms (each a reportable analysis unit): a plain `def`
  #     function, an `async def` function, and a class method (a `def` nested in a class
  #     body). All three are the same grammar node (`function_definition`); the extractor
  #     recognizes one node kind, while `async def` carries an `async` keyword and a method
  #     is a function_definition nested in a class body. They are observably distinct forms.
  #   - Normalization maps the Python grammar's identifier nodes to _ID and literal nodes to
  #     _LIT, so two Python functions with identical structure but different identifier/literal
  #     text normalize identically and score 1.0 — exactly as for Rust and JS/TS.
  #   - --lang forces ONE grammar for every scanned file this invocation, overriding extension
  #     detection: py forces Python. Accepted values are rust, js, ts, py. (The full-word
  #     `rust` versus short `js`/`ts`/`py` spelling is a known inconsistency tracked for a
  #     future unification; it does not affect this slice.)
  #   - In a mixed-language directory with no --lang, each file is parsed with the grammar its
  #     own extension selects, and pairs from all languages are merged into ONE sorted output.
  #   - Qualification gates, --threshold, --min-lines, --min-nodes, --format, sorting, and the
  #     three exclusion layers behave identically to #3/#4 for Python files.
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
  #   - Adds py as an accepted --lang value (previously rust, js, ts). Adds .py to the set of
  #     extensions scanned by default. Python's conventional build dir __pycache__ is added to
  #     the built-in auto-excluded set; that exclusion is owned and verified by #4, not here.
  #
  # SCOPE:
  #   - Does NOT: detect duplication across languages (a Python function is never paired with a
  #     Rust or JS/TS function; comparison is within a grammar's hash space only).
  #   - Does NOT: redefine reporting format, exit codes, thresholds, size gates, --format, or
  #     the exclusion layers (owned by #3 and #4).
  #   - Does NOT: treat `def`, `async def`, and class method as three separate extractor node
  #     kinds; they are one function_definition form observed in three shapes.
  #   - ASSUMED: a "function" for Python means any recognized Python function form; a class
  #              method qualifies, not only module-level `def`.
  #   - ASSUMED: line ranges are 1-based and inclusive, identical to #3.
  #   - ASSUMED: when --lang is supplied, extension auto-detection is bypassed for every file.
  #
  # UX INTENT: none
  # Design artifacts: none

  # python-detection-1
  # Two Python files whose functions share structure but differ only in identifiers are
  # reported as a pair, proving the shared normalization pipeline runs under the Python grammar.
  Scenario Outline: Structural duplicate in a Python source pair is reported
    Given a "<ext>" file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<ext>" file "<right_file>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"
    And the reported score is at least "<min_score>"

    Examples:
      | ext | left_file    | right_file  | structure      | left_ids | right_ids | args  | exit_code | min_score |
      | py  | src/alpha.py | src/beta.py | accumulate_sum | a,b,sum  | x,y,total | ./src | 1         | 0.82      |

  # python-detection-2
  # Each recognized Python function form is a reportable analysis unit: a structural twin
  # whose function uses the named form is detected as a pair. The form is carried by the
  # "<structure>" parameter so the step phrasing stays identical to python-detection-1.
  Scenario Outline: Each Python function form is a reportable analysis unit
    Given a "<ext>" file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<ext>" file "<right_file>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"

    Examples:
      | ext | left_file    | right_file  | structure          | left_ids | right_ids | args  | exit_code |
      | py  | src/alpha.py | src/beta.py | def-function       | a,b,sum  | x,y,total | ./src | 1         |
      | py  | src/alpha.py | src/beta.py | async-def-function | a,b,sum  | x,y,total | ./src | 1         |
      | py  | src/alpha.py | src/beta.py | class-method       | a,b,sum  | x,y,total | ./src | 1         |

  # python-detection-3
  # A file's grammar is auto-detected from its .py extension when no --lang is given; the
  # twin is detected, proving the Python grammar parsed it. The extension is the
  # discriminating variable.
  Scenario Outline: Python grammar is auto-detected from file extension
    Given a "<ext>" file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<ext>" file "<right_file>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"

    Examples:
      | ext | left_file    | right_file  | structure      | left_ids | right_ids | args  | exit_code |
      | py  | src/alpha.py | src/beta.py | accumulate_sum | a,b,sum  | x,y,total | ./src | 1         |

  # python-detection-4
  # --lang py forces the Python grammar regardless of extension; a non-standard extension is
  # parsed as Python and the twin is reported. The non-standard extension is the
  # discriminating variable.
  Scenario Outline: --lang py forces the Python grammar on a non-standard extension
    Given a "<ext>" file "<left_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<ext>" file "<right_file>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"

    Examples:
      | ext | left_file     | right_file   | structure      | left_ids | right_ids | args            | exit_code |
      | py  | src/alpha.txt | src/beta.txt | accumulate_sum | a,b,sum  | x,y,total | --lang py ./src | 1         |

  # python-detection-5
  # A directory mixing languages is analyzed file-by-file with each file's own grammar,
  # and pairs from every language appear in a single merged output. The three extensions
  # are the discriminating variables.
  Scenario Outline: Mixed-language directory including Python yields one merged output
    Given a "<a_ext>" file "<a_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<a_ext>" file "<a_twin>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    And a "<b_ext>" file "<b_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<b_ext>" file "<b_twin>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    And a "<c_ext>" file "<c_file>" containing a function with structure "<structure>" and identifiers "<left_ids>"
    And a "<c_ext>" file "<c_twin>" containing a function with structure "<structure>" and identifiers "<right_ids>"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<a_file>" and "<a_twin>"
    And stdout reports a duplicate pair for "<b_file>" and "<b_twin>"
    And stdout reports a duplicate pair for "<c_file>" and "<c_twin>"

    Examples:
      | a_ext | b_ext | c_ext | a_file   | a_twin   | b_file   | b_twin   | c_file   | c_twin   | structure      | left_ids | right_ids | args  | exit_code |
      | rs    | ts    | py    | src/a.rs | src/b.rs | src/c.ts | src/d.ts | src/e.py | src/f.py | accumulate_sum | a,b,sum  | x,y,total | ./src | 1         |

  # python-detection-6
  # A Python file that fails to parse is an error (exit 2) with a stderr diagnostic,
  # identical to the Rust parse-error contract in #3.
  Scenario Outline: Unparsable Python source is an error
    Given a "<ext>" file "<file>" containing source that fails to parse
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stderr is not empty

    Examples:
      | ext | file          | args  | exit_code |
      | py  | src/broken.py | ./src | 2         |
