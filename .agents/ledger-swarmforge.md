# Swarmforge Ledger

Work queue for swarmforge-scoped findings. Prunable: curator removes `applied` entries older than 3 cycles each run.
Format: `<date> | <session-id> | <role> | <failure-class> | <verdict> | <status> | <one-line summary>`
Status ∈ `pending | applied | stale`

---

2026-06-18 | (architect) | architect | wrong-path | rejected→swarmforge | pending | Before writing handoff to: field, verify role name via ls swarmforge/roles/
2026-06-18 | (hardender) | hardender | convention-gap | rejected→swarmforge | pending | cargo-mutants nextest profile: use --test-tool nextest -- --profile mutation not --profile directly
2026-06-18 | (integrator-crap) | integrator | wrong-path | rejected→swarmforge | pending | Before swarm_handoff.sh, cd to main repo root to ensure commit hashes resolve correctly
2026-06-18 | (integrator-crap) | integrator | wrong-path | rejected→swarmforge | pending | Before force-updating feature branch, check if remote exists and is behind
2026-06-18 | (integrator-merge) | integrator | wrong-path | rejected→swarmforge | pending | Force-update remote feature branch via git push origin <hash>:refs/heads/<branch> --force-with-lease
2026-06-19 | (coder-exclusion) | coder | wrong-path | rejected→swarmforge | pending | merge_and_process.sh unconditional git fetch fails for local-only branches; add fallback
2026-06-19 | (coder-exclusion) | coder | convention-gap | rejected→swarmforge | pending | Before adding coverage tests to reduce CRAP, verify C<=threshold algebraically first
2026-06-19 | (specifier-exclusions) | specifier | convention-gap | rejected→swarmforge | pending | Specifier Phase 2 prune: keep parametrized step shape, do NOT inline single-row values into plain Scenarios
2026-06-19 | (qa-exclusions-route-back) | QA | convention-gap | rejected→swarmforge | pending | merge_and_process.sh should try local branch first before git fetch origin
2026-06-19 | (qa-exclusions-route-back) | QA | convention-gap | rejected→swarmforge | pending | QA: document exact regeneration command including --steps and --feature-path flags
2026-06-19 | (qa-note-defect) | coder | convention-gap | rejected→swarmforge | pending | Acceptance test verification: always run full pipeline (gherkin-parser → entrypoint-generator → nextest) not just committed tests
2026-06-19 | (qa-note-defect) | QA | convention-gap | rejected→swarmforge | pending | Distinguish QA note types: defect-report requires investigation, completion/audit requires acknowledgment only
2026-06-19 | (qa-note-defect) | QA | wrong-path | rejected→swarmforge | pending | QA defect note with no commit/branch: stop-and-report, do not silently close
2026-06-19 | (scan-module) | architect | convention-gap | rejected→swarmforge | pending | merge_and_process.sh always fetches from remote — breaks local-only swarm; add local branch fallback
2026-06-19 | (js-ts-spec) | specifier | convention-gap | rejected→swarmforge | stale | specifier role: git reset --hard origin/default gated by auto-classifier; add allow-rule or startup note
2026-06-19 | (ux-passthrough) | ux-engineer | tool-error | rejected→swarmforge | pending | entire session info --transcript blocked by deny rule on .entire/metadata/**; primary retro path permanently broken
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | wrong-path | rejected→swarmforge | pending | QA note citing case IDs must name the feature/file — case IDs not globally unique across QA suites
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | wrong-path | rejected→swarmforge | pending | Never run find / (whole-FS); bound searches to repo, ~/.local, ~/src, or known build dir
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | convention-gap | rejected→swarmforge | pending | specifier reset rule vs destructive-gate tension: decide canonical behavior for named-base handoffs
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | convention-gap | rejected→swarmforge | pending | Base-adoption fallback: git checkout -B <branch> <commit>, never git checkout <commit> -- <subset>
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | convention-gap | rejected→swarmforge | pending | Pre-handoff gate: git show HEAD:<file> matches message claims; git diff --stat shows full base present
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | convention-gap | rejected→swarmforge | pending | Backward work-flow gap: QA→specifier has no defined delivery channel for spec patches
2026-06-21 | c01492bf-e25d-4c04-90cd-79eba5e938df | coder | convention-gap | rejected→swarmforge | pending | Coder: before investigating handoff commit, first establish branch topology via git log --oneline
2026-06-21 | (cleaner-qa-cleanup) | cleaner | convention-gap | rejected→swarmforge | pending | merge_and_process.sh fails when sender branch not pushed: fallback to git merge --no-ff <sha>
2026-06-21 | (cli-surface-11) | coder | convention-gap | rejected→swarmforge | pending | After merging spec commit, run acceptance pipeline before reading diff to identify actual failures
2026-06-21 | (cli-surface-qa-gap) | coder | tool-error | rejected→swarmforge | pending | merge_and_process.sh git fetch origin fails for local-only branches; fallback to git rev-parse + merge
2026-06-21 | (qa-spec-gaps) | QA | convention-gap | rejected→swarmforge | pending | QA: before sending git_handoff with no new code, confirm whether a commit is actually needed
2026-06-21 | (qa-spec-gaps) | QA | convention-gap | rejected→swarmforge | pending | QA route-back to specifier: lead with what gap is and why it matters (not just the routing rule)
2026-06-21 | (qa-spec-gaps) | QA | convention-gap | rejected→swarmforge | pending | Built-in exclusion scenarios: include parent-scan where excluded dir IS reachable under scanned path
2026-06-21 | (qa-spec-gaps) | QA | convention-gap | rejected→swarmforge | pending | When routing back via git_handoff with spec gaps: commit findings file first so commit carries context
2026-06-21 | (cli-surface-qa-verif) | QA | convention-gap | rejected→swarmforge | stale | commit abbrev: always use git rev-parse --short=10 HEAD rather than git log --oneline
2026-06-21 | (cli-surface-step-dedup) | cleaner | convention-gap | rejected→swarmforge | pending | include!-based shared module: qualify types with crate:: path not bare type names
2026-06-21 | (cli-surface-step-dedup) | cleaner | convention-gap | rejected→swarmforge | pending | include!-based shared module is the correct pattern for APS step utilities; document this
2026-06-21 | (ux-no-intent) | ux-engineer | tool-error | rejected→swarmforge | pending | merge_and_process.sh assumes origin/swarmforge-<role> exists; local worktrees use local-only branches
2026-06-21 | (ux-passthrough-2) | ux-engineer | convention-gap | rejected→swarmforge | pending | Handoff send is the final task step; run agent-retro AFTER handoff is queued, then done_with_current.sh
2026-06-21 | (ux-no-intent-short) | ux-engineer | convention-gap | rejected→swarmforge | pending | cli-surface has no UX Intent section — may indicate gap in specifier template or process upstream
2026-06-21 | e8291b83-d205-4af2-bd47-d04608d89322 | architect | wrong-path | rejected→swarmforge | stale | When receiving git_handoff, run git diff HEAD <commit> --stat FIRST as ground-truth check; empty diff = no-forward [will-not-fix: no perceived value — empty-diff scenario not seen in practice]
2026-06-21 | 1762020f-4f10-4aaa-bfb8-843752680366 | cleaner | convention-gap | rejected→swarmforge | pending | cargo mutants --list (not a full run) is the correct scan/count mode command; clarify engineering article
2026-06-21 | 1762020f-4f10-4aaa-bfb8-843752680366 | cleaner | convention-gap | rejected→swarmforge | stale | Handoff pipeline should enforce rebase on downstream cleaner branch before sending upstream — add/add conflicts otherwise [will-not-fix: upstream changes require cleaner review before propagation — gate never cleared]
2026-06-21 | 1762020f-4f10-4aaa-bfb8-843752680366 | cleaner | convention-gap | rejected→swarmforge | pending | Before CRAP refactor, count branch points (match arms + loops + conditions) to plan extract-helper passes as one refactor
2026-06-21 | b4761e12-ce6a-4176-88b1-82541ca6fd67 | architect | convention-gap | rejected→swarmforge | stale | git_handoff commit: always use git rev-parse --short=10 HEAD (10 chars); default --short=7 is wrong
2026-06-21 | b4761e12-ce6a-4176-88b1-82541ca6fd67 | architect | tool-error | rejected→swarmforge | pending | agent-retro: entire session info --transcript deny rule blocks primary path; fallback via ~/.claude/sessions/*.json is the live path
2026-06-21 | 1b981d1f-54de-444f-a0f2-8e248dcf7fdd | architect | convention-gap | rejected→swarmforge | pending | Architect must read all ADRs in docs/adr/ before evaluating architectural state; do not relitigate encoded decisions
2026-06-21 | c5e8d278-c766-4ef8-ac17-9ebc08f308c7 | hardender | wrong-path | rejected→swarmforge | pending | After launching mutation with run_in_background:true, do NOT poll manually — wait for task notification
2026-06-21 | c5e8d278-c766-4ef8-ac17-9ebc08f308c7 | hardender | wrong-path | rejected→swarmforge | pending | Review runner adapter for shared file paths before first gherkin-mutator run; multi-worker TOCTOU causes false survivors
2026-06-21 | c5e8d278-c766-4ef8-ac17-9ebc08f308c7 | hardender | convention-gap | rejected→swarmforge | pending | Sequence: Gherkin mutation first (monopolizes debug build), language mutation after — avoid build lock contention
2026-06-21 | c5e8d278-c766-4ef8-ac17-9ebc08f308c7 | hardender | convention-gap | rejected→swarmforge | pending | gherkin-mutator has no diff/manifest skip — feature request: respect implementation_hash to skip verified mutations
2026-06-21 | cd1fedb9-ccf8-4ea9-80a9-f688b5733499 | specifier | convention-gap | rejected→swarmforge | pending | Specifier must resolve PRD-traceable decisions from defaults; ask user only genuinely-ambiguous questions, one at a time
2026-06-21 | cd1fedb9-ccf8-4ea9-80a9-f688b5733499 | specifier | convention-gap | rejected→swarmforge | pending | Enumerated requirements (list of items in issue/PRD) are required coverage — do not ask user to choose a subset
2026-06-21 | cd1fedb9-ccf8-4ea9-80a9-f688b5733499 | specifier | convention-gap | rejected→swarmforge | stale | Investigate: specifier git reset --hard origin/<default> routinely denied by sandbox; add guarded form or allow-rule
2026-06-21 | 033a44af-c980-43bf-bfea-16b4db9a46c1 | coder | wrong-path | rejected→swarmforge | pending | Merge specifier commit FIRST before any file reads; missing feature file discovered only at gherkin-parser time
2026-06-21 | 033a44af-c980-43bf-bfea-16b4db9a46c1 | coder | wrong-path | rejected→swarmforge | pending | Smoke-test new grammar crates via unit test in the crate (#[test] in ast.rs), not a standalone /tmp file
2026-06-21 | cea00e56-970a-410d-a6fa-19cfac1fcd8e | coder | convention-gap | rejected→swarmforge | pending | Add jsts_detection to run-acceptance script STEPS_MAP and feature list (current list missing jsts_detection.feature)
2026-06-21 | 8fbf47f7-c991-464e-8232-c9d7b6c7debb | ux-engineer | convention-gap | rejected→swarmforge | stale | Clarify in ux-engineer role: `# UX INTENT: none` comment satisfies "no ## UX Intent section" — pass through to cleaner
2026-06-21 | 8fbf47f7-c991-464e-8232-c9d7b6c7debb | ux-engineer | convention-gap | rejected→swarmforge | stale | Constitution or role: in autonomous sessions, resolve ambiguities silently and act — do not narrate decision branches inline [will-not-fix: term "autonomous session" is undefined — all sessions are autonomous; rule is too vague to change LLM behavior]
2026-06-21 | 2308d3c6-531d-460c-8ab1-ca4d79eae2ad | ux-engineer | tool-error | rejected→swarmforge | pending | entire session info --transcript blocked by sandbox deny rule — primary retro data path non-functional; fallback works
2026-06-21 | 2308d3c6-531d-460c-8ab1-ca4d79eae2ad | ux-engineer | convention-gap | rejected→swarmforge | stale | swarm-persona: if persona content already in last user message from /swarm-persona command, skip Skill tool invocation
2026-06-21 | dc39fd1d-1487-4be4-8e7c-5fc4338f7e20 | curator | convention-gap | rejected→swarmforge | pending | Retro files MUST include Session ID in header; without it curator ledger entries lack traceability
2026-06-21 | 4f568388-4756-468f-b118-65f5ed8d4f6f | integrator | convention-gap | rejected→swarmforge | stale | Remove --timeout 1800 from gh pr checks --watch in integrator role — flag does not exist in installed gh CLI
2026-06-21 | 4f568388-4756-468f-b118-65f5ed8d4f6f | integrator | wrong-path | rejected→swarmforge | stale | Before gh pr merge --delete-branch, check whether any worktree holds feature branch; omit --delete-branch if so
2026-06-21 | 219bdcb5-2ff5-4bec-a76b-4ae66ef729d7 | curator | convention-gap | rejected→swarmforge | pending | Before processing retros, count ls ~/.claude/worklog/retros/*.md | wc -l and confirm count before batch-reading; do not start until all files are in read list
2026-06-21 | 4f8f2fb7-0288-436f-8ecf-bfbb15bf4573 | cleaner | convention-gap | rejected→swarmforge | pending | At 100% coverage CRAP=CC (identity): if CC > threshold, no coverage increase helps — plan refactor to reduce CC directly
2026-06-21 | 7e755b42-3ef8-496a-a00f-62543c985dfe | coder | convention-gap | rejected→swarmforge | pending | git_handoff commit hash duplicate entry (traceability): git rev-parse --short=10 HEAD is the verified form — inferable from existing entries but pattern persists
2026-06-21 | 8190a76c-957c-4c7f-9cb6-9b87ab896e60 | QA | wrong-path | rejected→swarmforge | pending | ready_for_next.sh must be called with full path (bash /path/to/swarmforge/scripts/ready_for_next.sh) in agent worktrees; bare command fails if not on PATH
2026-06-21 | 178101ab-fb94-46a7-9c36-ff63a64c6497 | specifier | convention-gap | rejected→swarmforge | pending | Before reporting any prompt-named tool as missing, check .claude/skills/<name>/, swarmforge/scripts/, PATH, ~/go/bin in order — prompt tools are capability references not necessarily PATH binaries
