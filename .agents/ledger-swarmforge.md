# Swarmforge Ledger

Work queue for swarmforge-scoped findings. Prunable: curator removes `applied` entries older than 3 cycles each run.
Format: `<date> | <session-id> | <role> | <failure-class> | <verdict> | <status> | <one-line summary>`
Status ∈ `pending | applied | stale`

---

2026-06-18 | (architect) | architect | convention-gap | rejected→swarmforge | pending | Before writing handoff commit, derive with git rev-parse --short=10 HEAD not git log --oneline
2026-06-18 | (architect) | architect | wrong-path | rejected→swarmforge | pending | Before writing handoff to: field, verify role name via ls swarmforge/roles/
2026-06-18 | (hardender) | hardender | convention-gap | rejected→swarmforge | pending | cargo-mutants nextest profile: use --test-tool nextest -- --profile mutation not --profile directly
2026-06-18 | (hardender) | hardender | convention-gap | rejected→swarmforge | pending | handoff role names case-sensitive; use git rev-parse --short=10 for commit abbrev
2026-06-18 | (integrator-crap) | integrator | wrong-path | rejected→swarmforge | pending | Before swarm_handoff.sh, cd to main repo root to ensure commit hashes resolve correctly
2026-06-18 | (integrator-crap) | integrator | wrong-path | rejected→swarmforge | pending | Before force-updating feature branch, check if remote exists and is behind
2026-06-18 | (integrator-merge) | integrator | wrong-path | rejected→swarmforge | pending | Force-update remote feature branch via git push origin <hash>:refs/heads/<branch> --force-with-lease
2026-06-18 | (integrator-merge) | integrator | wrong-path | rejected→swarmforge | pending | Before retrying gh pr merge after error, run gh pr view --json state to check if already merged
2026-06-19 | (coder-exclusion) | coder | wrong-path | rejected→swarmforge | pending | merge_and_process.sh unconditional git fetch fails for local-only branches; add fallback
2026-06-19 | (coder-exclusion) | coder | convention-gap | rejected→swarmforge | pending | Before adding coverage tests to reduce CRAP, verify C<=threshold algebraically first
2026-06-19 | (cleaner-clean) | cleaner | convention-gap | rejected→swarmforge | pending | Always run git rev-parse --short=10 HEAD immediately before writing git_handoff draft
2026-06-19 | (specifier-exclusions) | specifier | convention-gap | rejected→swarmforge | pending | Specifier Phase 2 prune: keep parametrized step shape, do NOT inline single-row values into plain Scenarios
2026-06-19 | (specifier-exclusions) | specifier | convention-gap | rejected→swarmforge | pending | git_handoff commit: abbrev via git rev-parse --short=10 HEAD (10 chars) not git log default
2026-06-19 | (qa-exclusions-route-back) | QA | convention-gap | rejected→swarmforge | pending | merge_and_process.sh should try local branch first before git fetch origin
2026-06-19 | (qa-exclusions-route-back) | QA | convention-gap | rejected→swarmforge | pending | QA: document exact regeneration command including --steps and --feature-path flags
2026-06-19 | (qa-note-defect) | coder | convention-gap | rejected→swarmforge | pending | Acceptance test verification: always run full pipeline (gherkin-parser → entrypoint-generator → nextest) not just committed tests
2026-06-19 | (qa-note-defect) | QA | convention-gap | rejected→swarmforge | pending | Distinguish QA note types: defect-report requires investigation, completion/audit requires acknowledgment only
2026-06-19 | (qa-note-defect) | QA | wrong-path | rejected→swarmforge | pending | QA defect note with no commit/branch: stop-and-report, do not silently close
2026-06-19 | (scan-module) | architect | convention-gap | rejected→swarmforge | pending | merge_and_process.sh always fetches from remote — breaks local-only swarm; add local branch fallback
2026-06-19 | (js-ts-spec) | specifier | convention-gap | rejected→swarmforge | pending | specifier role: git reset --hard origin/default gated by auto-classifier; add allow-rule or startup note
2026-06-19 | (ux-passthrough) | ux-engineer | tool-error | rejected→swarmforge | pending | entire session info --transcript blocked by deny rule on .entire/metadata/**; primary retro path permanently broken
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | wrong-path | rejected→swarmforge | pending | QA note citing case IDs must name the feature/file — case IDs not globally unique across QA suites
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | wrong-path | rejected→swarmforge | pending | Never run find / (whole-FS); bound searches to repo, ~/.local, ~/src, or known build dir
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | convention-gap | rejected→swarmforge | pending | specifier reset rule vs destructive-gate tension: decide canonical behavior for named-base handoffs
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | convention-gap | rejected→swarmforge | pending | Base-adoption fallback: git checkout -B <branch> <commit>, never git checkout <commit> -- <subset>
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | convention-gap | rejected→swarmforge | pending | Pre-handoff gate: git show HEAD:<file> matches message claims; git diff --stat shows full base present
2026-06-20 | afdf5692-db7c-4979-a811-7c5efd6ff189 | specifier | convention-gap | rejected→swarmforge | pending | Backward work-flow gap: QA→specifier has no defined delivery channel for spec patches
2026-06-21 | c01492bf-e25d-4c04-90cd-79eba5e938df | coder | convention-gap | rejected→swarmforge | pending | Coder: before investigating handoff commit, first establish branch topology via git log --oneline
2026-06-21 | (cleaner-qa-cleanup) | cleaner | convention-gap | rejected→swarmforge | pending | merge_and_process.sh fails when sender branch not pushed: fallback to git merge --no-ff <sha>
2026-06-21 | ca7ffb05-6213-4c90-bd00-6c83407f5c79 | hardender | wrong-path | rejected→swarmforge | pending | swarm_handoff.sh: requires git rev-parse --short=10; role names case-sensitive (check swarmforge.conf)
2026-06-21 | (cli-surface-11) | coder | convention-gap | rejected→swarmforge | pending | After merging spec commit, run acceptance pipeline before reading diff to identify actual failures
2026-06-21 | (cli-surface-qa-gap) | coder | tool-error | rejected→swarmforge | pending | merge_and_process.sh git fetch origin fails for local-only branches; fallback to git rev-parse + merge
2026-06-21 | (qa-spec-gaps) | QA | convention-gap | rejected→swarmforge | pending | QA: before sending git_handoff with no new code, confirm whether a commit is actually needed
2026-06-21 | (qa-spec-gaps) | QA | convention-gap | rejected→swarmforge | pending | QA route-back to specifier: lead with what gap is and why it matters (not just the routing rule)
2026-06-21 | (qa-spec-gaps) | QA | convention-gap | rejected→swarmforge | pending | Built-in exclusion scenarios: include parent-scan where excluded dir IS reachable under scanned path
2026-06-21 | (qa-spec-gaps) | QA | convention-gap | rejected→swarmforge | pending | When routing back via git_handoff with spec gaps: commit findings file first so commit carries context
2026-06-21 | (cli-surface-qa-verif) | QA | convention-gap | rejected→swarmforge | pending | commit abbrev: always use git rev-parse --short=10 HEAD rather than git log --oneline
2026-06-21 | (cli-surface-step-dedup) | cleaner | convention-gap | rejected→swarmforge | pending | include!-based shared module: qualify types with crate:: path not bare type names
2026-06-21 | (cli-surface-step-dedup) | cleaner | convention-gap | rejected→swarmforge | pending | include!-based shared module is the correct pattern for APS step utilities; document this
2026-06-21 | (ux-no-intent) | ux-engineer | tool-error | rejected→swarmforge | pending | merge_and_process.sh assumes origin/swarmforge-<role> exists; local worktrees use local-only branches
2026-06-21 | (ux-passthrough-2) | ux-engineer | convention-gap | rejected→swarmforge | pending | Handoff send is the final task step; run agent-retro AFTER handoff is queued, then done_with_current.sh
2026-06-21 | (ux-no-intent-short) | ux-engineer | convention-gap | rejected→swarmforge | pending | cli-surface has no UX Intent section — may indicate gap in specifier template or process upstream
