# mutation-stamp: sha256=ced0cd7295f56b51e8cdd001af082d39cb730d110167c33eacfc744c3504add8
# acceptance-mutation-manifest-begin
# {"version":1,"tested_at":"2026-06-16T06:30:32.749709Z","feature_name":"Scaffold CLI invocation","feature_path":"features/scaffold_cli.feature","background_hash":"74234e98afe7498fb5daf1f36ac2d78acc339464f950703b8c019892f982b90b","implementation_hash":"unknown","scenarios":[]}
# acceptance-mutation-manifest-end

Feature: Scaffold CLI invocation

  # TRACKING: #2 (parent #1)
  #
  # CONTRACT:
  #   drywall [path ...]
  #   request:  path — string, zero or more filesystem path arguments
  #   response exit=0: no stdout, no stderr — for any invocation with zero or more path arguments
  #   (Duplicate-detection output and exit codes 1/2 are NOT part of this scaffold.)
  #
  # CONSTRAINTS:
  #   - The binary must not panic for any path argument, whether the path exists or not.
  #   - No duplicate-detection behavior: the binary produces no analysis output.
  #   - A minimal Rust source file must parse through the loaded tree-sitter-rust grammar without error.
  #
  # SEQUENCING: none
  #
  # NFR:
  #   - Invocation exits promptly; no hang, no interactive prompt.
  #   - Clean exit is distinguishable by exit code 0 and empty stdout.
  #
  # SIDE EFFECTS:
  #   - .gitignore must cover Rust build artifacts (target/).
  #
  # SCOPE:
  #   - Does NOT: detect duplicates or emit any analysis output.
  #   - Does NOT: validate, read, or report errors on the contents of path arguments.
  #   - Does NOT: implement exit codes 1 (duplicates found) or 2 (error); only 0 in this scaffold.
  #   - ASSUMED: passing a nonexistent path is tolerated (exit 0, no output) at the scaffold stage,
  #              since path validation is later behavior. Flag if path validation is expected now.
  #
  # UX INTENT: none
  # Design artifacts: none

  # scaffold-cli-1
  Scenario: Binary exits cleanly with no arguments
    Given the drywall release binary is built
    When the binary is run with the arguments ""
    Then the exit code is 0
    And stdout is empty
    And stderr is empty

  # scaffold-cli-1
  Scenario: Binary exits cleanly with a single path argument
    Given the drywall release binary is built
    When the binary is run with the arguments "./src"
    Then the exit code is 0
    And stdout is empty
    And stderr is empty

  # scaffold-cli-1
  Scenario: Binary exits cleanly with multiple path arguments
    Given the drywall release binary is built
    When the binary is run with the arguments "./src ./features"
    Then the exit code is 0
    And stdout is empty
    And stderr is empty

  # scaffold-cli-1
  # ASSUMED: nonexistent paths tolerated at scaffold stage; path validation is later behavior.
  Scenario: Binary exits cleanly with a nonexistent path argument
    Given the drywall release binary is built
    When the binary is run with the arguments "./does-not-exist"
    Then the exit code is 0
    And stdout is empty
    And stderr contains no panic text

  # scaffold-cli-5
  Scenario: Minimal Rust source parses without error
    Given the drywall release binary is built
    And a minimal Rust source directory exists at "./tmp/qa-minimal"
    When the binary is run with the arguments "./tmp/qa-minimal"
    Then the exit code is 0
    And stdout is empty
    And stderr is empty
