# my-project

## What This Is
A Django + Flutter app that does X for Y users. Backend on AWS, mobile on iOS + Android.
See `lore/MISSION.md` for the full project mission.

## Rules
- Never push directly to main — always go through staging
- All API changes require a migration and a decision entry
- See `lore/GUARDRAILS.md` for full guardrails

## Stack
- Backend: Django / PostgreSQL / Celery
- Frontend: Flutter (iOS + Android)
- Infra: Docker / AWS EC2 / Cloudflare

## lore Index
- `INDEX.md` — TOC and loading guide (read this first)
- `GUARDRAILS.md` — project rules
- `CONTEXT.md` — current state and session log
- `workspace/ticket.json` — ticket state (use `lore ticket` CLI to manage)
- `architecture/` — system design, models, APIs
- `features/` — one file per feature
- `testing/registry.md` — test coverage map
- `decisions/` — architecture decision records
- `bullpen/` — agent roster (conductor sessions)
- `lore` CLI — global Rust binary: `lore ticket add/list/start/done`, `lore session status`, `lore recall`

## Current Focus
Epic 3: Payment instrument architecture redesign

## Session Rule
This project uses `lore` for AI memory. The session-end checklist is **mandatory** — stale lore is worse than no lore.

### Session Close Checklist
At end of every conductor / build session:
1. **Rewrite `CONTEXT.md` header** — Focus, Phase, Open, Next must reflect current state
2. **Verify ticket consistency** — run `lore ticket list`; run `lore ticket done <ID>` for anything completed. NO stale tickets.
3. **Confirm all sub-agent log entries** — check CONTEXT.md log for each completed task
4. **Merge any lore conflicts** — if two agents touched the same file, reconcile
5. **Scan for open items** — run `lore ticket add "[description]"` for unresolved items
6. **Git commit** — both code AND lore changes committed together
