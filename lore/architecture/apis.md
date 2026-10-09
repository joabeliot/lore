# Interfaces — lore

Stub. Source of truth: `src/main.rs` (CLI) and `handle_request` in `src/dashboard.rs`.

## CLI surface (v0.1.0)
version · create project · recall · list projects · delete project ·
ticket add/list/show/schedule/start/done/edit · session close/status/attach/log ·
inspect · edit project · update · takeme · dashboard

## Dashboard endpoints (JSON, GET)
`/api/overview` · `/api/projects` · `/api/stats` · `/api/activity` · `/api/tickets` ·
`/api/projects/<shorthand>` (+ `/tickets`, `/sessions`) · `/api/project_names`

## Gotchas
- Dashboard currently binds `0.0.0.0` (ADR F9, to be `127.0.0.1`).
- `inspect` exits the process with code 1 on failure (`std::process::exit`).
