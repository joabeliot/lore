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
- `kanban/` — work queue (backlog → todo → inprogress → done)
- `architecture/` — system design, models, APIs
- `features/` — one file per feature
- `testing/registry.md` — test coverage map
- `decisions/` — architecture decision records

## Current Focus
Epic 3: Payment instrument architecture redesign

## Session Rule
This project uses `lore` for AI memory. At the end of every session:
1. Rewrite the `CONTEXT.md` header with current state
2. Append a compact log entry
3. Update kanban and any changed feature/decision files
