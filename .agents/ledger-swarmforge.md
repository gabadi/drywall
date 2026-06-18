# Swarmforge Ledger

Work queue for swarmforge-scoped findings. Prunable: curator removes `applied` entries older than 3 cycles each run.
Format: `<date> | <session-id> | <role> | <failure-class> | <verdict> | <status> | <one-line summary>`
Status ∈ `pending | applied | stale`

---

2026-06-16 | 0c3619ed-b8cc-4848-b38a-fae030d2bd08 | architect | convention-gap | rejected→swarmforge | applied | DRY chicken-and-egg check-then-install rule targets swarmforge constitution, not project
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | wrong-path | rejected→swarmforge | applied | cargo-mutants flag fix (-j not --max-workers) targets swarmforge hardender role prompt
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | convention-gap | rejected→swarmforge | applied | Read generate_entrypoint.py before runner adapter — targets hardender role prompt
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | wrong-path | rejected→swarmforge | applied | Scenario Outline → plain Scenario fix for generator — targets hardender role prompt
2026-06-16 | 3df20fbb-25fd-4cf3-9140-02cfe10620a6 | hardender | wrong-path | rejected→swarmforge | applied | cargo-llvm-cov startup check targets swarmforge engineering rules
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | convention-gap | rejected→swarmforge | applied | gh pr create --head rule targets swarmforge integrator role prompt
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | tool-error | rejected→swarmforge | applied | SSH pre-check before git push targets swarmforge integrator role prompt
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | convention-gap | rejected→swarmforge | applied | Use git rev-parse --short=10 for handoff commit abbrev targets swarmforge handoffs article
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | wrong-path | rejected→swarmforge | applied | Absolute paths for tmp files targets swarmforge integrator/engineering
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | tool-error | rejected→swarmforge | applied | gh pr merge auto-mode classifier block — investigate swarmforge settings
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | wrong-path | rejected→swarmforge | applied | swarm_handoff.sh must run from assigned worktree targets swarmforge integrator prompt
2026-06-16 | d4a40556-f924-407b-94b1-161d35602f8a | integrator | wrong-path | rejected→swarmforge | applied | Handoff delivery delay when invoked outside worktree — investigate handoffd.bb
2026-06-16 | 3047ad02-3cc9-4a73-a17c-db61183e2637 | QA | wrong-path | rejected→swarmforge | applied | ready_for_next.sh must run from assigned worktree — targets swarmforge constitution
2026-06-16 | 3047ad02-3cc9-4a73-a17c-db61183e2637 | QA | convention-gap | rejected→swarmforge | applied | CRAP --exclude 'acceptance/**' rule targets swarmforge engineering rules (duplicate of hardender#3)
2026-06-16 | 34d2722c-85c8-4754-afe6-736114e676c2 | cleaner | tool-error | rejected→swarmforge | applied | Check-before-install for startup tools targets swarmforge engineering constitution
2026-06-16 | 34d2722c-85c8-4754-afe6-736114e676c2 | cleaner | wrong-path | rejected→swarmforge | applied | cargo build --release before acceptance tests targets swarmforge local-engineering rules
2026-06-16 | 34d2722c-85c8-4754-afe6-736114e676c2 | cleaner | convention-gap | rejected→swarmforge | applied | Run cargo clippy before writing manual trait impls — targets swarmforge engineering rules
2026-06-16 | a238fa12-6e38-4f43-bf66-efc530cc68ef | coder | convention-gap | rejected→swarmforge | applied | Standalone script over heredoc for multi-language generators — targets swarmforge engineering
2026-06-16 | a238fa12-6e38-4f43-bf66-efc530cc68ef | coder | wrong-path | rejected→swarmforge | applied | Verify test runner conventions before choosing output dir — targets swarmforge engineering
2026-06-16 | a238fa12-6e38-4f43-bf66-efc530cc68ef | coder | convention-gap | rejected→swarmforge | applied | Verify generated code compiles without warnings — targets swarmforge engineering
2026-06-16 | ec72e255-9882-4c2e-8ef6-ee08578aca9d | ux-engineer | convention-gap | rejected→swarmforge | applied | Clarify UX INTENT comment vs markdown heading — targets swarmforge ux-engineer role prompt
2026-06-16 | ec72e255-9882-4c2e-8ef6-ee08578aca9d | ux-engineer | tool-error | rejected→swarmforge | applied | Double swarm-persona invocation — investigate harness skill replay mechanism
2026-06-17 | fb292f78-e50e-4fd4-9f73-f5a114cbcd14 | architect | convention-gap | rejected→swarmforge | applied | Post-refactor full verification sequence — targets constitution/local-engineering.prompt
2026-06-17 | fb292f78-e50e-4fd4-9f73-f5a114cbcd14 | architect | convention-gap | rejected→swarmforge | applied | Export function before writing property test — targets architect role prompt
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | convention-gap | rejected→swarmforge | applied | GENERATED file must be regenerated not hand-merged — targets hardender role prompt
2026-06-17 | e1630652-fc8c-43c3-8e9e-e1d89202c14d | hardender | wrong-path | rejected→swarmforge | applied | cargo-mutants --profile mutation usage — targets hardender role prompt
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | wrong-path | rejected→swarmforge | applied | Check mergeable after gh pr create — targets integrator role prompt
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | wrong-path | rejected→swarmforge | applied | Check git log for already-merged features before PR — targets integrator role prompt
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | timeout | rejected→swarmforge | applied | CI polling loops need wall-clock timeout — targets integrator role prompt
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | tool-error | rejected→swarmforge | applied | SSH pre-check before git push — targets integrator role prompt (duplicate of 2026-06-16 integrator#2)
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | timeout | rejected→swarmforge | applied | Background until-loops bypass harness timeout block — investigate swarmforge scripts
2026-06-17 | 8aafd328-8e37-4d8e-a66c-5fd669c3b6cb | integrator | convention-gap | rejected→swarmforge | applied | git_handoff not note after feature lands — targets integrator role prompt
2026-06-17 | ab98a5aa-c35b-4929-9bbe-ba608e1308ae | QA | convention-gap | rejected→swarmforge | applied | Remove scaffold-era ASSUMED annotations when deferred behavior ships — targets QA role prompt
2026-06-17 | ab98a5aa-c35b-4929-9bbe-ba608e1308ae | QA | convention-gap | rejected→swarmforge | applied | Run cargo fmt after regenerating acceptance tests — targets QA role prompt
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | convention-gap | rejected→swarmforge | applied | Fallback when merge_and_process not in PATH — targets cleaner role prompt or handoffs article
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | convention-gap | rejected→swarmforge | applied | Module split: move functions and tests together in one pass — targets engineering article
2026-06-17 | a675d102-d21a-46f9-9e6a-e4e00227828d | cleaner | tool-error | rejected→swarmforge | applied | GPG/1Password commit signing failure — investigate global git config
2026-06-17 | 1a7938af-e074-4405-a483-d928715a4d47 | coder | convention-gap | rejected→swarmforge | applied | Preview git diff before merge to avoid blind --theirs — targets coder role prompt
2026-06-17 | 1a7938af-e074-4405-a483-d928715a4d47 | coder | tool-error | rejected→swarmforge | applied | advisor() overload fallback not documented — targets constitution or role prompt
2026-06-17 | 7ac4552c-bbbc-42b6-942e-1ff1441be247 | specifier | convention-gap | rejected→swarmforge | applied | Post-handoff close-out sequence missing from specifier prompt — targets swarmforge/roles/specifier.prompt
