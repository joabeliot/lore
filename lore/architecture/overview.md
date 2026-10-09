# Architecture Overview — lore

Single Rust binary (`lore`, v0.1.0, edition 2021), installed to `~/.local/bin/lore`.
Source: `src/main.rs` (clap wiring), `src/commands.rs` (all command logic),
`src/models.rs` (structs + session/ticket file helpers), `src/dashboard.rs` +
`src/dashboard.html` (embedded `tiny_http` server). Deps: clap, serde (+json/yaml),
uuid, chrono, regex, anyhow, ureq, tiny_http.

## Storage (all plain files, no database)
- Global: `~/.lore/sessions/<uuid>.yml`, one per registered project.
- Per project: `<project>/lore/config.yml` (session uuid, paths) and
  `<project>/lore/workspace/ticket.json` (tickets + auto-increment counter).
- Skills in `skills/` are installed into agent skill dirs by `install.sh`.

## Data flow
`resolve_session` (explicit prefix, else walk up from cwd to `lore/config.yml`)
→ load session yml → load config → load `ticket.json` → mutate → write back whole file.
Every command is read-modify-write; there is no locking (see decisions F4).

## Dashboard
`lore dashboard --port N` starts an HTTP server serving `dashboard.html` and JSON
endpoints that read the same session/ticket files on each request.

## Planned (ADR 0001, `decisions/conductor-and-gates.md`)
Event log, gated `done`, `verify` steps run by `inspect`, resumable runs,
`lore conductor`, richer dashboard.
