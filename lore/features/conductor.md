# Feature: lore conductor

**Status:** Idea → In design (ADR 0001 drafted, tickets LOR-1 … LOR-6)

## What It Does
Runs the build loop for tickets: pick next `todo` → worktree → brief builder → `inspect` → independent review → done / escalate / blocked. JB's flow: give a task, work out requirements with Claude, tickets (with `verify` steps) land in `lore/`, the conductor runs them, JB gets the product.

## Design (see `decisions/conductor-and-gates.md`)
- Gates are data on the ticket: `verify[]`, `manual[]`, `protect[]`, `attempts/max_attempts`.
- Tests are written at planning time by Claude/Codex, never by the builder; `protect` globs make them read-only to the builder. Bugs and new behavior need red-then-green.
- One `transition()` function; `done` needs a green inspect record at the current HEAD, unchanged protected files, an independent review, and approved `manual` items.
- Credit protection: max 2 attempts, hard worker timeouts (kill the process group), run-level call cap, cheap checks before expensive ones, no blind retries.
- Kill switch: `STOP` file. Isolation: worktree per ticket, PR-only.
- Resume from `~/.lore/runs/<session>/<ticket>.json`; `session close` archives instead of deleting.
- Event log `~/.lore/events.jsonl` feeds the localhost dashboard (board, live feed, ticket detail, alerts, STOP/approve).

## Edge Cases
- Unautomatable criteria (UI feel, copy) → `manual` → ticket parked as needs-human, loop continues.
- Worker hangs (agy once idle ~14 min after a dropped connection) → timeout + kill.
- Builder tries to weaken a test → rejected via `protect` hash.
- Two agents writing `ticket.json` → needs lock + atomic write (LOR-1).

## Assumptions
- Worker command templates in `lore/config.yml` are enough to drive agy and Codex headlessly: validate with fake workers first, then one real smoke run.
- agy/Codex allow-rules can stay narrow for unattended builds: validate in a supervised run.

## Open Questions
- Final ticket states (`backlog/todo/inprogress/review/done/blocked`): awaiting JB's confirmation.
- Where do `verify` blocks live on the ticket, and how does the conductor skill write them from requirements?
- Unattended write-access policy for workers (worktree-only?).

## Notes
Not a separate framework on purpose; see `decisions/agent-orchestration-model.md`.
