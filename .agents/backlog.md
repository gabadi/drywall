# Curator Backlog

Enforcement-gate proposals (mechanical fixes that beat documentation). Append-only; each entry dated.

---

2026-06-21 | swarmforge-pattern | tool-error | architect,coder,cleaner,QA,ux-engineer | pending | merge_and_process.sh unconditionally git-fetches from origin, failing for all local-only swarm branches; add local-branch fallback
2026-06-21 | swarmforge-pattern | convention-gap | architect,hardender,specifier,QA,cleaner | pending | handoff commit abbrev must be exactly 10 chars via git rev-parse --short=10; all roles independently relearn this constraint each session
2026-06-21 | backlog | convention-gap | generate_entrypoint.py | pending | acceptance-entrypoint-generator --steps flag not documented at call sites; default scaffold_cli_steps.rs silently produces wrong entrypoints for non-scaffold features
