# Idea: unattended / overnight runs

Captured 2026-10-09. Not for now; supervised runs come first.

## Shape
Hermes cron (or launchd) starts a launcher script that runs `lore conductor run` headlessly per approved ticket; Hermes `send` pings JB with a morning summary (PRs opened, failures, escalations, cost). Claude runs via `claude -p`; it only exists while a session is open, so unattended work needs a launcher; that's a script, not a framework.

## Preconditions (all must hold)
- Real test gate for the target project (see `ideas/doorstallor-pilot-findings.md`).
- Separate staging data/credentials for agents; no prod credentials in the sandbox.
- Worker-stuck rule (timeouts, process-group kill) and run-level spend/iteration caps; STOP file honored.
- Unattended permission policy: workers write only inside their worktree; no blanket `--dangerously-skip-permissions`.
- PR-only; never push to main. Prod deploys and payments keep a human gate.
- Only pre-approved, low-risk tickets (lint fixes, dead-code removal) at first.
- Credit check: Hermes/OpenRouter runway was thin (~$1.72 per Hermes's own report).

## Open
Does Hermes have budget caps and a kill switch? Unverified.
