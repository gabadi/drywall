# Project Ledger

Permanent recurrence-detection index. Only items with `project` scope that carry algorithmic value: `promoted→*`, `rejected→inferable`, `rejected→duplicate`, `rejected→first-occurrence`, `rejected→phenomenon`.
Ephemerals and `rejected→swarmforge` items are excluded — see `ledger-swarmforge.md`.
Format: `<date> | <session-id> | <role> | <failure-class> | <verdict> | <one-line summary>`

---

2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | wrong-path | promoted→AGENTS.md | cargo llvm-cov requires LLVM_COV/LLVM_PROFDATA env vars on Homebrew Rust (no rustup)
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | convention-gap | promoted→AGENTS.md | CRAP for Rust must exclude acceptance/**: cargo crap --lcov lcov.info --exclude 'acceptance/**'
2026-06-16 | 3047ad02-3cc9-4a73-a17c-db61183e2637 | QA | tool-error | rejected→inferable | GPG signing via 1Password fails intermittently — inferable from environment config, not project code
2026-06-16 | 34d2722c-85c8-4754-afe6-736114e676c2 | cleaner | convention-gap | promoted→AGENTS.md | cargo-llvm-cov not installed; CRAP runs without coverage; CC≤6 in acceptance step files acceptable
2026-06-17 | fb292f78-e50e-4fd4-9f73-f5a114cbcd14 | architect | tool-error | rejected→first-occurrence | agent-retro tool_result_sizes returns type names only; first occurrence of skill investigation
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | tool-error | promoted→AGENTS.md | Use bare git (not rtk git) for diff generation; RTK reformats diff and breaks --in-diff
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | convention-gap | promoted→backlog | Equivalent mutant j=i+1→i*1 in find_duplicate_pairs: document in code or suppress; route to coder
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | wrong-path | rejected→duplicate | llvm-profdata PATH workaround (rustup explicit path) — related items already in ledger/AGENTS.md
2026-06-17 | ab98a5aa-c35b-4929-9bbe-ba608e1308ae | QA | wrong-path | rejected→inferable | llvm-profdata symlink (rustup→Homebrew) — machine-specific env config; inferable from environment
2026-06-17 | ab98a5aa-c35b-4929-9bbe-ba608e1308ae | QA | wrong-path | promoted→backlog | generate_entrypoint.py hardcodes scaffold_cli feature path; clobbers metadata for multiple features
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | wrong-path | promoted→AGENTS.md | Coverage command: use explicit rustup toolchain path (Homebrew Rust lacks llvm-profdata)
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | wrong-path | promoted→AGENTS.md | Coverage command explicit rustup path (merged with action 2)
2026-06-17 | 7ac4552c-bbbc-42b6-942e-1ff1441be247 | specifier | convention-gap | promoted→AGENTS.md | drywall output format mirrors dry4go: per-function node counts, lexicographic tie-break
2026-06-17 | 7ac4552c-bbbc-42b6-942e-1ff1441be247 | specifier | convention-gap | promoted→AGENTS.md | gherkin-ir-dry-checker: pipe JSON report through compact summary to avoid dumping full JSON
2026-06-17 | eb8eba5a-c8b4-4c90-b8c5-5a3cd1d659a0 | ux-engineer | tool-error | rejected→first-occurrence | agent-retro primary path blocked by deny rule on .entire/metadata/**; first occurrence
2026-06-18 | (ca7ffb05) | hardender | convention-gap | promoted→AGENTS.md | gherkin-mutator defaults to features/a-feature.feature; must invoke via symlink pattern
2026-06-18 | (hardender) | hardender | convention-gap | promoted→AGENTS.md | cargo-mutants --exclude patterns match absolute paths; use **/filename.rs not src/filename.rs
2026-06-19 | (scan-module) | architect | wrong-path | promoted→AGENTS.md | cargo build --release must precede --profile acceptance to avoid stale-binary false failures
2026-06-19 | (specifier) | specifier | convention-gap | promoted→AGENTS.md | acceptance/QA fixtures must be outside gitignored paths; gitignore-awareness on by default silently skips
2026-06-21 | c01492bf-e25d-4c04-90cd-79eba5e938df | coder | convention-gap | promoted→AGENTS.md | acceptance-entrypoint-generator requires --steps <steps-file> for non-scaffold features
2026-06-21 | c01492bf-e25d-4c04-90cd-79eba5e938df | coder | convention-gap | promoted→AGENTS.md | World in acceptance/runtime/mod.rs needs #[allow(dead_code)] on struct for multi-binary coverage
2026-06-19 | (scan-module) | architect | tool-error | rejected→machine-specific | file-based commit workaround for 1Password pipe failure; machine-specific env config
2026-06-19 | (coder) | coder | convention-gap | rejected→inferable | diversify test function bodies to avoid Jaccard gate — inferable from AGENTS.md DRY self-check note
2026-06-21 | 033a44af-c980-43bf-bfea-16b4db9a46c1 | coder | convention-gap | promoted→AGENTS.md | parallel js_X/ts_X test bodies must differ in source structure — identical short bodies trigger dogfood self-detection (recurrence)
2026-06-21 | 17ae50d5-de10-451c-8bc9-29c74fc0e4ef | QA | wrong-path | promoted→AGENTS.md | ad-hoc QA scratch fixtures must use /tmp not project tmp/ — gitignore silently skips project gitignored paths
2026-06-21 | cea00e56-970a410d-a6fa-19cfac1fcd8e | coder | convention-gap | promoted→AGENTS.md | Lang::Tsx must use LANGUAGE_TSX not LANGUAGE_TYPESCRIPT; guard is typescript_grammar_rejects_jsx_markup
2026-06-21 | c5e8d278-c766-4ef8-ac17-9ebc08f308c7 | hardender | convention-gap | promoted→backlog | binary(~acceptance) nextest filter matches mutation test files; change to exact match
2026-06-21 | cd1fedb9-ccf8-4ea9-80a9-f688b5733499 | specifier | convention-gap | rejected→inferable | gherkin-ir-dry-checker residual placeholder-variant findings expected — already in auto-memory
2026-06-21 | 44b0fcc3-6aa9-4a5f-841b-aa836a59c617 | cleaner | tool-error | rejected→inferable | GPG signing failure via 1Password in worktree — already in auto-memory feedback_git_commit_gpg
2026-06-21 | 033a44af-c980-43bf-bfea-16b4db9a46c1 | coder | convention-gap | promoted→memory | scan filter extension requires reviewing property tests whose names encode old language list
2026-06-21 | 7e755b42-3ef8-496a-a00f-62543c985dfe | coder | convention-gap | promoted→AGENTS.md | entrypoint-generator writes absolute include! paths when output dir is outside project root; fix: write to tests/ or sed paths
2026-06-21 | 7e755b42-3ef8-496a-a00f-62543c985dfe | coder | convention-gap | rejected→inferable | cargo add tree-sitter-<lang> to let cargo resolve ABI-compatible version — standard Rust cargo practice
2026-06-21 | d3a4e42a-eff7-478e-9364-84f8b20845d0 | architect | convention-gap | rejected→duplicate | scan-filter property test 4-point update list — already promoted to memory in prior curator run
2026-06-21 | 8190a76c-957c-4c7f-9cb6-9b87ab896e60 | QA | convention-gap | rejected→inferable | AGENTS.md tests_regen/ not a committed artifact — gitignore already handles; stated as low priority in retro
2026-06-21 | 8190a76c-957c-4c7f-9cb6-9b87ab896e60 | QA | tool-error | rejected→first-occurrence | agent-retro conversation_arc content fields empty in retro extract — extract.py may not capture content field; first occurrence
2026-06-21 | 178101ab-fb94-46a7-9c36-ff63a64c6497 | specifier | tool-error | rejected→first-occurrence | agent-retro extract.py prepends Warning: line before JSON when subagent cost data missing; consumers must strip to first {
2026-06-21 | 178101ab-fb94-46a7-9c36-ff63a64c6497 | specifier | convention-gap | rejected→first-occurrence | agent-retro subagent dispatch costs not attributed by default; --subagents-dir flag needed for accurate budget table
