# lore Index — lore (inception)

**Always load:** `GUARDRAILS.md`, `CONTEXT.md`

## Tier 2 — Load When Relevant
| File / Dir | Load when |
|---|---|
| `workspace/ticket.json` | Use `lore ticket list` — never edit directly |
| `architecture/overview.md` | Making structural changes (modules, storage, dashboard) |
| `architecture/models.md` | Touching Session / Ticket / Config structs or on-disk formats |
| `architecture/apis.md` | Changing CLI surface or dashboard endpoints |
| `decisions/conductor-and-gates.md` | Conductor, gates, event log, resume, dashboard (ADR 0001, findings F1–F9, test plan) |
| `decisions/agent-orchestration-model.md` | Delegating to agy/Codex/Hermes; worker permissions; why no framework |
| `features/conductor.md` | Working on `lore conductor` |
| `features/takeme.md` | Touching `lore takeme` or the shell wrapper |
| `ideas/doorstallor-pilot-findings.md` | Starting the pilot on doorstallor; what its gate lacks |
| `ideas/overnight-runs.md` | Anything unattended (preconditions live here) |
| `decisions/project-profiles.md` | Working on profiles, STATE/log split, front matter, migration (LOR-8/9/10) |
| `testing/registry.md` | Writing or reviewing tests |
| `bullpen/` | Orchestrating — agents, priority order, invocation quirks |

## Human-Only (Never auto-load)
`OG.md` — raw dev journal
`MISSION.md` — project soul
`CHANGELOG.md` — git history (hook-generated, not installed yet)

## Proposed Additions
Agent-suggested lore expansions pending human review.
- [none yet]
