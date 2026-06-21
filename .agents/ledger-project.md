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
