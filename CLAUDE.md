# lore

## What This Is
Rust CLI for project memory, sessions and tickets for AI-assisted development. Being extended with enforced gates, an event log, resumable runs, `lore conductor`, and a localhost dashboard (see `lore/decisions/conductor-and-gates.md`).

## Structure
```
src/main.rs        clap wiring only
src/commands.rs    command logic (tickets, sessions, inspect)
src/models.rs      structs + session/ticket file helpers
src/dashboard.rs   tiny_http server; src/dashboard.html UI
skills/            agent skills shipped with lore
lore/              this project's own memory (lore for lore)
```

## Rules
- Tests first for gate logic; tests are hermetic (temp `HOME`, never the real `~/.lore`)
- Keep `ticket.json`/session formats backward compatible (`#[serde(default)]`)
- Never push, merge, release or run `lore update` without JB's OK
- See `lore/GUARDRAILS.md` for the full list

## Stack
Rust 2021 · clap · serde (json/yaml) · tiny_http · anyhow · chrono · uuid

## Build / test
```bash
cargo build
cargo test
cargo clippy -- -D warnings
```

## lore
- `lore/` is the bible. Load `INDEX.md`, `GUARDRAILS.md`, `CONTEXT.md` every session.
- `lore ticket list` for work; never edit `lore/workspace/ticket.json` by hand.
- Session: `514ee393` (shorthand LOR)

## Session Rule
At the end of every session:
1. Rewrite the `CONTEXT.md` header with current state
2. Append a compact log entry (3-5 lines)
3. `lore ticket done` only after `lore inspect` passes
4. Never write to `lore/OG.md` or `lore/MISSION.md`
