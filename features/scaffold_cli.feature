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
  Scenario Outline: Binary accepts path arguments and exits cleanly
    Given the drywall release binary is built
    When the binary is run with the arguments "<args>"
    Then the exit code is 0
    And stdout is empty

    Examples:
      | args              |
      | ./src             |
      | ./src ./features  |
      |                   |
      | ./does-not-exist  |
