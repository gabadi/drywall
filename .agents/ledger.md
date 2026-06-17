# Curator Ledger

Append-only audit of all processed retro items.
Format: `<date> | <session-id> | <role> | <failure-class> | <verdict> | <one-line summary>`

---

2026-06-16 | 0c3619ed-b8cc-4848-b38a-fae030d2bd08 | architect | convention-gap | ephemeral | Advisor-before-work pattern worked; no change needed
2026-06-16 | 0c3619ed-b8cc-4848-b38a-fae030d2bd08 | architect | convention-gap | rejected→swarmforge | DRY chicken-and-egg check-then-install rule targets swarmforge constitution, not project
2026-06-16 | 0c3619ed-b8cc-4848-b38a-fae030d2bd08 | architect | convention-gap | ephemeral | Release binary precondition noted but not fixed (out of architect lane)
2026-06-16 | 0c3619ed-b8cc-4848-b38a-fae030d2bd08 | architect | convention-gap | ephemeral | Property-test separation pattern (proptest + #[ignore]) validated; ephemeral acknowledge
2026-06-16 | 9fc20581-f3eb-4b65-b3af-bff7c11af751 | specifier | convention-gap | ephemeral | Awake presence signal handled correctly; no change needed
2026-06-16 | 814c0d7a-1f9c-4910-a15a-33bee187e22f | specifier | convention-gap | ephemeral | Awake presence signal handled correctly; no change needed
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | wrong-path | rejected→swarmforge | cargo-mutants flag fix (-j not --max-workers) targets swarmforge hardender role prompt
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | wrong-path | promoted→AGENTS.md | cargo llvm-cov requires LLVM_COV/LLVM_PROFDATA env vars on Homebrew Rust (no rustup)
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | convention-gap | promoted→AGENTS.md | CRAP for Rust must exclude acceptance/**: cargo crap --lcov lcov.info --exclude 'acceptance/**'
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | convention-gap | rejected→swarmforge | Read generate_entrypoint.py before runner adapter — targets hardender role prompt
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | wrong-path | rejected→swarmforge | Scenario Outline → plain Scenario fix for generator — targets hardender role prompt
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | convention-gap | ephemeral | Advisor call before mutation-driven Gherkin edits; pattern confirmed working
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | wrong-path | rejected→swarmforge | cargo-llvm-cov startup check targets swarmforge engineering rules
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | convention-gap | ephemeral | Plain-Scenarios conversion for equivalent mutants at scaffold stage; ephemeral acknowledge
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | convention-gap | rejected→swarmforge | gh pr create --head rule targets swarmforge integrator role prompt
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | tool-error | rejected→swarmforge | SSH pre-check before git push targets swarmforge integrator role prompt
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | convention-gap | rejected→swarmforge | Use git rev-parse --short=10 for handoff commit abbrev targets swarmforge handoffs article
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | wrong-path | rejected→swarmforge | Absolute paths for tmp files targets swarmforge integrator/engineering
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | tool-error | rejected→swarmforge | gh pr merge auto-mode classifier block — investigate swarmforge settings
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | wrong-path | rejected→swarmforge | swarm_handoff.sh must run from assigned worktree targets swarmforge integrator prompt
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | wrong-path | rejected→swarmforge | Handoff delivery delay when invoked outside worktree — investigate handoffd.bb
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | convention-gap | ephemeral | HTTPS fallback for SSH worked cleanly; no data loss
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | convention-gap | ephemeral | Handoff format validation caught bad commit abbrev; harness working correctly
2026-06-16 | 3047ad02-3cc9-4a73-a17c-db61183e2637 | QA | wrong-path | rejected→swarmforge | ready_for_next.sh must run from assigned worktree — targets swarmforge constitution
2026-06-16 | 3047ad02-3cc9-4a73-a17c-db61183e2637 | QA | convention-gap | rejected→swarmforge | CRAP --exclude 'acceptance/**' rule targets swarmforge engineering rules (duplicate of hardender#3)
2026-06-16 | 3047ad02-3cc9-4a73-a17c-db61183e2637 | QA | convention-gap | ephemeral | Advisor call before refactoring acceptance infra was correct; pattern confirmed
2026-06-16 | 3047ad02-3cc9-4a73-a17c-db61183e2637 | QA | tool-error | rejected→inferable | GPG signing via 1Password fails intermittently — inferable from environment config, not project code
2026-06-16 | 3047ad02-3cc9-4a73-a17c-db61183e2637 | QA | convention-gap | ephemeral | CRAP must exclude acceptance/** for this project (duplicate of hardender#3, already promoted)
2026-06-16 | 3047ad02-3cc9-4a73-a17c-db61183e2637 | QA | convention-gap | ephemeral | QA procedure bullet-mapping is correct primary QA activity; acknowledged
2026-06-16 | 34d2722c-85c8-4754-afe6-736114e676c2 | cleaner | tool-error | rejected→swarmforge | Check-before-install for startup tools targets swarmforge engineering constitution
2026-06-16 | 34d2722c-85c8-4754-afe6-736114e676c2 | cleaner | wrong-path | rejected→swarmforge | cargo build --release before acceptance tests targets swarmforge local-engineering rules
2026-06-16 | 34d2722c-85c8-4754-afe6-736114e676c2 | cleaner | convention-gap | rejected→swarmforge | Run cargo clippy before writing manual trait impls — targets swarmforge engineering rules
2026-06-16 | 34d2722c-85c8-4754-afe6-736114e676c2 | cleaner | convention-gap | ephemeral | CRAP inflation from missing coverage data — boundary-file CRAP expected; ephemeral
2026-06-16 | 34d2722c-85c8-4754-afe6-736114e676c2 | cleaner | convention-gap | promoted→AGENTS.md | cargo-llvm-cov not installed; CRAP runs without coverage; CC≤6 in acceptance step files acceptable
2026-06-16 | a238fa12-6e38-4f43-bf66-efc530cc68ef | coder | convention-gap | rejected→swarmforge | Standalone script over heredoc for multi-language generators — targets swarmforge engineering
2026-06-16 | a238fa12-6e38-4f43-bf66-efc530cc68ef | coder | wrong-path | rejected→swarmforge | Verify test runner conventions before choosing output dir — targets swarmforge engineering
2026-06-16 | a238fa12-6e38-4f43-bf66-efc530cc68ef | coder | convention-gap | rejected→swarmforge | Verify generated code compiles without warnings — targets swarmforge engineering
2026-06-16 | a238fa12-6e38-4f43-bf66-efc530cc68ef | coder | convention-gap | ephemeral | Advisor before implementation surfaces hidden constraints; pattern confirmed
2026-06-16 | a238fa12-6e38-4f43-bf66-efc530cc68ef | coder | convention-gap | ephemeral | APS pipeline (gherkin-parser → generator → cargo test) works for Rust; include!() approach sound
2026-06-16 | ec72e255-9882-4c2e-8ef6-ee08578aca9d | ux-engineer | convention-gap | rejected→swarmforge | Clarify UX INTENT comment vs markdown heading — targets swarmforge ux-engineer role prompt
2026-06-16 | ec72e255-9882-4c2e-8ef6-ee08578aca9d | ux-engineer | convention-gap | ephemeral | Pass-through path (UX INTENT: none → forward to cleaner) executed correctly
2026-06-16 | ec72e255-9882-4c2e-8ef6-ee08578aca9d | ux-engineer | tool-error | rejected→swarmforge | Double swarm-persona invocation — investigate harness skill replay mechanism
2026-06-17 | fb292f78-e50e-4fd4-9f73-f5a114cbcd14 | architect | convention-gap | rejected→swarmforge | Post-refactor full verification sequence — targets constitution/local-engineering.prompt
2026-06-17 | fb292f78-e50e-4fd4-9f73-f5a114cbcd14 | architect | convention-gap | rejected→swarmforge | Export function before writing property test — targets architect role prompt
2026-06-17 | fb292f78-e50e-4fd4-9f73-f5a114cbcd14 | architect | convention-gap | ephemeral | Advisor call before handoff pattern confirmed; acknowledged
2026-06-17 | fb292f78-e50e-4fd4-9f73-f5a114cbcd14 | architect | tool-error | rejected→first-occurrence | agent-retro tool_result_sizes returns type names only; first occurrence of skill investigation
2026-06-17 | fb292f78-e50e-4fd4-9f73-f5a114cbcd14 | architect | convention-gap | ephemeral | GPG workaround stable; already in session memory; inferable from environment
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | tool-error | promoted→AGENTS.md | Use bare git (not rtk git) for diff generation; RTK reformats diff and breaks --in-diff
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | convention-gap | promoted→backlog | Equivalent mutant j=i+1→i*1 in find_duplicate_pairs: document in code or suppress; route to coder
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | wrong-path | rejected→duplicate | llvm-profdata PATH workaround (rustup explicit path) — related items already in ledger/AGENTS.md
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | convention-gap | rejected→swarmforge | GENERATED file must be regenerated not hand-merged — targets hardender role prompt
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | wrong-path | rejected→swarmforge | cargo-mutants --profile mutation usage — targets hardender role prompt
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | convention-gap | ephemeral | Advisor call before mutation work confirmed correct; acknowledged
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | convention-gap | ephemeral | Equivalent mutant identification and documentation was correct; acknowledged
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | wrong-path | rejected→swarmforge | Check mergeable after gh pr create — targets integrator role prompt
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | wrong-path | rejected→swarmforge | Check git log for already-merged features before PR — targets integrator role prompt
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | timeout | rejected→swarmforge | CI polling loops need wall-clock timeout — targets integrator role prompt
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | tool-error | rejected→swarmforge | SSH pre-check before git push — targets integrator role prompt (duplicate of 2026-06-16 integrator#2)
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | timeout | rejected→swarmforge | Background until-loops bypass harness timeout block — investigate swarmforge scripts
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | convention-gap | ephemeral | HTTPS fallback for SSH worked cleanly; acknowledged
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | convention-gap | ephemeral | Conflict/already-merged diagnosis was correct; acknowledged
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | convention-gap | rejected→swarmforge | git_handoff not note after feature lands — targets integrator role prompt
2026-06-17 | ab98a5aa-c35b-4929-9bbe-ba608e1308ae | QA | wrong-path | rejected→inferable | llvm-profdata symlink (rustup→Homebrew) — machine-specific env config; inferable from environment
2026-06-17 | ab98a5aa-c35b-4929-9bbe-ba608e1308ae | QA | convention-gap | rejected→swarmforge | Remove scaffold-era ASSUMED annotations when deferred behavior ships — targets QA role prompt
2026-06-17 | ab98a5aa-c35b-4929-9bbe-ba608e1308ae | QA | wrong-path | promoted→backlog | generate_entrypoint.py hardcodes scaffold_cli feature path; clobbers metadata for multiple features
2026-06-17 | ab98a5aa-c35b-4929-9bbe-ba608e1308ae | QA | convention-gap | ephemeral | merge_and_process intent-notation correctly inferred; acknowledged
2026-06-17 | ab98a5aa-c35b-4929-9bbe-ba608e1308ae | QA | convention-gap | rejected→swarmforge | Run cargo fmt after regenerating acceptance tests — targets QA role prompt
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | convention-gap | rejected→swarmforge | Fallback when merge_and_process not in PATH — targets cleaner role prompt or handoffs article
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | wrong-path | promoted→AGENTS.md | Coverage command: use explicit rustup toolchain path (Homebrew Rust lacks llvm-profdata)
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | convention-gap | rejected→swarmforge | Module split: move functions and tests together in one pass — targets engineering article
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | wrong-path | promoted→AGENTS.md | Coverage command explicit rustup path (merged with action 2)
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | convention-gap | ephemeral | DRY self-check correctly caught test duplication; acknowledged
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | convention-gap | ephemeral | CRAP threshold exclusion for main.rs correct and in AGENTS.md; acknowledged
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | tool-error | rejected→swarmforge | GPG/1Password commit signing failure — investigate global git config
2026-06-17 | 1a7938af-e074-4405-a483-d928715a4d47 | coder | convention-gap | ephemeral | Fork delegation for large implementation worked cleanly; acknowledged
2026-06-17 | 1a7938af-e074-4405-a483-d928715a4d47 | coder | convention-gap | rejected→swarmforge | Preview git diff before merge to avoid blind --theirs — targets coder role prompt
2026-06-17 | 1a7938af-e074-4405-a483-d928715a4d47 | coder | convention-gap | ephemeral | Dogfood gate (scenario 3) is self-enforcing; no extra guard needed
2026-06-17 | 1a7938af-e074-4405-a483-d928715a4d47 | coder | tool-error | rejected→swarmforge | advisor() overload fallback not documented — targets constitution or role prompt
2026-06-17 | 7ac4552c-bbbc-42b6-942e-1ff1441be247 | specifier | convention-gap | rejected→swarmforge | Post-handoff close-out sequence missing from specifier prompt — targets swarmforge/roles/specifier.prompt
2026-06-17 | 7ac4552c-bbbc-42b6-942e-1ff1441be247 | specifier | convention-gap | ephemeral | Batch ambiguities into one AskUserQuestion before writing spec; pattern confirmed
2026-06-17 | 7ac4552c-bbbc-42b6-942e-1ff1441be247 | specifier | convention-gap | promoted→AGENTS.md | drywall output format mirrors dry4go: per-function node counts, lexicographic tie-break
2026-06-17 | 7ac4552c-bbbc-42b6-942e-1ff1441be247 | specifier | convention-gap | promoted→AGENTS.md | gherkin-ir-dry-checker: pipe JSON report through compact summary to avoid dumping full JSON
2026-06-17 | 10ade55d-2868-44cd-9c63-2668a7e07468 | specifier | convention-gap | ephemeral | Idle-cycle awake ack handled correctly; acknowledged
2026-06-17 | eb8eba5a-c8b4-4c90-b8c5-5a3cd1d659a0 | ux-engineer | convention-gap | ephemeral | No-UX-Intent fast-path correctly applied; acknowledged
2026-06-17 | eb8eba5a-c8b4-4c90-b8c5-5a3cd1d659a0 | ux-engineer | tool-error | rejected→first-occurrence | agent-retro primary path blocked by deny rule on .entire/metadata/**; first occurrence
2026-06-17 | eb8eba5a-c8b4-4c90-b8c5-5a3cd1d659a0 | ux-engineer | convention-gap | ephemeral | UX INTENT comment vs section heading distinction works correctly; acknowledged
