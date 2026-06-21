# acceptance-mutation-manifest-begin
# {"version":1,"tested_at":"2026-06-21T02:19:19Z","feature_name":"CLI surface exclusions auto built-in gitignore and repeatable exclude","feature_path":"features/cli_surface.feature","background_hash":"74234e98afe7498fb5daf1f36ac2d78acc339464f950703b8c019892f982b90b","implementation_hash":"sha256:95b9d1898498d586238cdb1d77842a4bac217c3c54135149afc16034b297517a","scenarios":[{"index":4,"name":"Repeated exclude flags union their patterns","scenario_hash":"e2d445cee3503c0f39ef5cd61e5d7a005ee1d6ac58b9b342a3a06b3289df19c7","mutation_count":10,"result":{"Total":10,"Killed":10,"Survived":0,"Errors":0},"tested_at":"2026-06-21T01:52:31Z"},{"index":8,"name":"A non-ignored twin in a git work tree is still reported","scenario_hash":"79018e2342bd6e6925d1f9dcda1328ad80759d64e30106dae0827ddd79d8ba4b","mutation_count":5,"result":{"Total":5,"Killed":5,"Survived":0,"Errors":0},"tested_at":"2026-06-21T01:51:33Z"},{"index":3,"name":"A twin outside built-in excluded directories is still reported","scenario_hash":"4aa6283251ff8303cb1ce7a20e346191d2b67f3cc515a4c7b126aedbe357efbd","mutation_count":4,"result":{"Total":4,"Killed":4,"Survived":0,"Errors":0},"tested_at":"2026-06-21T01:50:26Z"},{"index":9,"name":"Built-in exclusion output is deterministic across runs","scenario_hash":"4db36a0f7b9938293dd6a2ed88a76e0d11ed859774ca04bb0c03c786e8060935","mutation_count":4,"result":{"Total":4,"Killed":4,"Survived":0,"Errors":0},"tested_at":"2026-06-21T01:44:44Z"},{"index":1,"name":"Explicitly passing the parent does not re-include a built-in excluded subdirectory","scenario_hash":"e752ee6d372c44c7e4deeae84504d9f94e26f4c58971d69458ab7996f240e457","mutation_count":8,"result":{"Total":8,"Killed":8,"Survived":0,"Errors":0},"tested_at":"2026-06-21T01:42:15Z"},{"index":2,"name":"A built-in excluded directory passed directly is still excluded","scenario_hash":"627ec8a2a6093d6089efd8a037ce52ca034a010190bf253fcdd0474ad6bb3c56","mutation_count":4,"result":{"Total":4,"Killed":4,"Survived":0,"Errors":0},"tested_at":"2026-06-21T01:42:15Z"},{"index":6,"name":"Dogfood run stays clean with gitignore-awareness on","scenario_hash":"f35ab67c6382e50254cb9a17acfae92fe2a0616b40b579c894bcd9eb09705035","mutation_count":2,"result":{"Total":2,"Killed":2,"Survived":0,"Errors":0},"tested_at":"2026-06-21T01:42:15Z"},{"index":7,"name":"Gitignore-awareness silently no-ops when git is unavailable","scenario_hash":"ace01c48ab351669ea7e555ea49dfcea3a3325db43fe4fe06ad2e44f92aa362c","mutation_count":4,"result":{"Total":4,"Killed":4,"Survived":0,"Errors":0},"tested_at":"2026-06-21T01:42:15Z"}]}
# acceptance-mutation-manifest-end

Feature: CLI surface exclusions auto built-in gitignore and repeatable exclude

  # TRACKING: #4 (parent #1)
  #
  # CONTRACT:
  #   drywall [OPTIONS] <path>...
  #   request:  path      — string, one or more filesystem paths (file or directory) to scan
  #   request:  --exclude — glob string, repeatable; a path matching ANY supplied pattern is not scanned
  #   response exit=0: zero duplicate pairs — when every qualifying pair was suppressed or none exists
  #   response exit=1: one or more duplicate pairs — when at least one pair survives exclusion and scores >= threshold
  #
  #   This slice owns the exclusion surface that #3 does not specify: the always-on
  #   built-in directory exclusions, gitignore-awareness, and the repeatability of
  #   --exclude. Reporting format, exit codes, thresholds, size gates, --format, and
  #   --lang are specified by features/duplicate_detection.feature (#3) and are not
  #   redefined here.
  #
  #   A file is scanned only if it survives ALL THREE exclusion layers, in this order
  #   of evaluation (all are subtractive; none re-includes a path removed by another):
  #     1. Built-in directory exclusions (always on, no flag, no opt-out).
  #     2. Gitignore exclusions (default on, no opt-out) — paths git would ignore.
  #     3. User --exclude glob patterns (repeatable; union).
  #
  # CONSTRAINTS:
  #   - Built-in excluded directory names, always skipped without any flag and with no
  #     opt-out, at any depth under a scanned path:
  #     .git, target, node_modules, __pycache__, vendor, dist, .next.
  #   - Built-in exclusion applies even when the excluded directory is reachable only
  #     because an ancestor path was passed explicitly. Passing a parent does NOT opt its
  #     excluded subdirectories back into scanning.
  #   - Built-in exclusion matches on the directory NAME at any path segment, not on a
  #     leading-anchored path; e.g. a `target` directory nested below the scanned root
  #     is excluded the same as one directly under it.
  #   - Passing a built-in-excluded directory as a direct, explicit path argument is still
  #     excluded; the exclusion is not overridden by naming it on the command line.
  #   - Gitignore-awareness is DEFAULT ON with no opt-out flag. A path that git would treat
  #     as ignored is not scanned. The built-in seven always apply regardless of gitignore.
  #   - Gitignore resolution delegates to the `git` CLI (e.g. `git check-ignore`) when a
  #     `git` executable is available AND the scanned path is inside a git work tree.
  #     drywall does not reimplement gitignore matching in-process for this slice.
  #   - When no `git` executable is available, OR the scanned path is not inside a git work
  #     tree, gitignore-awareness silently no-ops: only the built-in seven and user
  #     --exclude patterns apply. This is not an error (still exit 0/1 per the duplicate
  #     result), and no diagnostic is written for the absence of git or a repo.
  #   - --exclude is repeatable. Supplying the flag more than once unions the patterns:
  #     a file is excluded if it matches ANY supplied pattern.
  #   - The three exclusion layers are independent and purely subtractive: none re-includes
  #     a path removed by another. A user --exclude pattern never re-includes a built-in or
  #     gitignored path; a non-matching gitignore never re-includes a built-in path.
  #
  # SEQUENCING: none
  #
  # NFR:
  #   - Built-in exclusion is deterministic and environment-independent: identical inputs
  #     yield byte-identical output regardless of machine, repo, or git config.
  #   - Gitignore exclusion is deterministic for a fixed repo state and git configuration;
  #     it may vary with the host's git config (e.g. core.excludesFile) — this is an
  #     accepted consequence of delegating to git, not a defect.
  #   - Invocation exits promptly; no hang and no interactive prompt. Shelling out to git
  #     must not hang the run.
  #
  # SIDE EFFECTS: none
  #
  # SCOPE:
  #   - Does NOT: redefine reporting format, exit codes, thresholds, size gates,
  #     --format, or --lang (owned by #3).
  #   - Does NOT: provide a flag to disable or override built-in exclusion or
  #     gitignore-awareness in this slice (both are default-on, no opt-out).
  #   - Does NOT: reimplement gitignore semantics in-process; it relies on the git CLI.
  #   - Does NOT: error or warn when git is absent or the path is not a repo; it falls
  #     back to built-in + user exclusion silently.
  #   - Does NOT: analyze JavaScript, TypeScript, or Python sources (Rust target only).
  #   - ASSUMED: built-in excluded names are matched case-sensitively against directory
  #              segment names, consistent with the literal names in the PRD.
  #   - ASSUMED: the built-in excluded set is a fixed list compiled into the binary,
  #              independent of any ignore file, so fixtures with no .gitignore are still
  #              excluded by name.
  #   - ASSUMED: gitignore precedence and matching are exactly whatever the git CLI reports
  #              (repo .gitignore at every level, .git/info/exclude, global excludesFile,
  #              negations) — drywall adopts git's answer verbatim.
  #   - ASSUMED: an --exclude glob is matched against each candidate file's path relative
  #              to the scanned root, consistent with #3 scenario 9.
  #   - ASSUMED: the dogfood gate `drywall ./src` must continue to exit 0 with
  #              gitignore-awareness on; acceptance fixtures must be placed where the git
  #              CLI does NOT report them ignored, so they remain visible to the scanner.
  #
  # UX INTENT: none
  # Design artifacts: none

  # cli-surface-1
  # A built-in excluded directory under the scanned path is never scanned, so a twin
  # hidden inside it forms no pair. The scanned path ("." — the parent) DOES contain the
  # excluded directory, so exclusion is what suppresses the pair, not unreachability.
  Scenario Outline: Built-in excluded directory under the scanned path is never scanned
    Given a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And no duplicate pair is reported
    And stderr is empty

    Examples:
      | left_file    | right_file           | args | exit_code |
      | src/alpha.rs | .git/beta.rs         | .    | 0         |
      | src/alpha.rs | target/beta.rs       | .    | 0         |
      | src/alpha.rs | node_modules/beta.rs | .    | 0         |
      | src/alpha.rs | __pycache__/beta.rs  | .    | 0         |
      | src/alpha.rs | vendor/beta.rs       | .    | 0         |
      | src/alpha.rs | dist/beta.rs         | .    | 0         |
      | src/alpha.rs | .next/beta.rs        | .    | 0         |

  # cli-surface-2
  # Built-in exclusion wins even when the excluded dir is reached only via an explicitly passed parent.
  Scenario Outline: Explicitly passing the parent does not re-include a built-in excluded subdirectory
    Given a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And no duplicate pair is reported

    Examples:
      | left_file     | right_file                | args | exit_code |
      | proj/alpha.rs | proj/node_modules/beta.rs | proj | 0         |
      | proj/alpha.rs | proj/target/beta.rs       | proj | 0         |

  # cli-surface-3
  # Naming a built-in excluded directory directly as a path argument is still excluded.
  Scenario Outline: A built-in excluded directory passed directly is still excluded
    Given a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And no duplicate pair is reported

    Examples:
      | left_file             | right_file           | args           | exit_code |
      | node_modules/alpha.rs | node_modules/beta.rs | ./node_modules | 0         |

  # cli-surface-4
  # A non-excluded twin outside any built-in excluded dir is still reported (guards over-exclusion).
  Scenario Outline: A twin outside built-in excluded directories is still reported
    Given a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"

    Examples:
      | left_file    | right_file  | args  | exit_code |
      | src/alpha.rs | src/beta.rs | ./src | 1         |

  # cli-surface-5
  # --exclude is repeatable: a file matching ANY supplied pattern is suppressed.
  # A witness file (never excluded) ensures each exclusion pattern is independently detectable:
  # if either exclude breaks, the escaped file pairs with the witness and exit becomes 1.
  Scenario Outline: Repeated exclude flags union their patterns
    Given a Rust file "<witness_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "p,q,res"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And no duplicate pair is reported

    Examples:
      | witness_file | left_file    | right_file  | args                                                 | exit_code |
      | src/gamma.rs | src/alpha.rs | src/beta.rs | --exclude **/alpha.rs --exclude **/beta.rs ./src     | 0         |
      | src/gamma.rs | src/alpha.rs | gen/beta.rs | --exclude **/alpha.rs --exclude **/gen/** ./src ./gen | 0         |

  # cli-surface-6
  # When git is available in a work tree, a gitignored twin is not scanned, so no pair forms.
  Scenario Outline: A gitignored path is not scanned when git is available
    Given a git work tree with a git executable available
    And a gitignore entry "<ignore_pattern>"
    And a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And no duplicate pair is reported
    And stderr is empty

    Examples:
      | ignore_pattern | left_file    | right_file       | args  | exit_code |
      | generated/     | src/alpha.rs | generated/beta.rs | ./src | 0         |

  # cli-surface-7
  # With gitignore-awareness on, the dogfood run on drywall's own source still exits clean.
  Scenario Outline: Dogfood run stays clean with gitignore-awareness on
    Given the drywall project source directory at "<args>"
    And a git work tree with a git executable available
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And no duplicate pair is reported

    Examples:
      | args  | exit_code |
      | ./src | 0         |

  # cli-surface-8
  # QA-10: with no git executable on PATH, gitignore-awareness no-ops: only built-in and
  # user exclusion apply, and a non-ignored twin is still reported. No error is raised.
  # The not-a-work-tree variant of this no-op is a distinct mode, covered by cli-surface-11.
  Scenario Outline: Gitignore-awareness silently no-ops when no git executable is available
    Given no git executable is available
    And a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"
    And stderr is empty

    Examples:
      | left_file    | right_file  | args  | exit_code |
      | src/alpha.rs | src/beta.rs | ./src | 1         |

  # cli-surface-9
  # QA-9: gitignore-awareness only subtracts ignored paths; a non-ignored twin in a
  # git work tree is still scanned and reported (guards against over-exclusion via git).
  Scenario Outline: A non-ignored twin in a git work tree is still reported
    Given a git work tree with a git executable available
    And a gitignore entry "<ignore_pattern>"
    And a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"

    Examples:
      | ignore_pattern | left_file    | right_file  | args  | exit_code |
      | generated/     | src/alpha.rs | src/beta.rs | ./src | 1         |

  # cli-surface-11
  # QA-12: gitignore-awareness also no-ops when git IS available but the scanned path is
  # NOT inside a git work tree (no .git, no ancestor repo). This is a DISTINCT failure mode
  # from cli-surface-8 (no git executable at all); both fall back to built-in + user
  # exclusion only, so a non-ignored twin is still reported, with no error.
  Scenario Outline: Gitignore-awareness silently no-ops outside a git work tree
    Given a directory that is not inside a git work tree
    And a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    When I run drywall with the arguments "<args>"
    Then the exit code is "<exit_code>"
    And stdout reports a duplicate pair for "<left_file>" and "<right_file>"
    And stderr is empty

    Examples:
      | left_file    | right_file  | args  | exit_code |
      | src/alpha.rs | src/beta.rs | ./src | 1         |

  # cli-surface-10
  # QA-7: built-in exclusion is environment-independent and deterministic; running the
  # same input twice yields byte-identical stdout and the same exit code.
  Scenario Outline: Built-in exclusion output is deterministic across runs
    Given a Rust file "<left_file>" containing a function with structure "accumulate_sum" and identifiers "a,b,sum"
    And a Rust file "<right_file>" containing a function with structure "accumulate_sum" and identifiers "x,y,total"
    When I run drywall twice with the arguments "<args>"
    Then both runs share the exit code "<exit_code>"
    And both runs produce byte-identical stdout

    Examples:
      | left_file    | right_file           | args | exit_code |
      | src/alpha.rs | node_modules/beta.rs | .    | 0         |
