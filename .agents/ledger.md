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
