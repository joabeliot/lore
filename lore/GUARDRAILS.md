# Guardrails — lore

lore is the gate every other project's agents run through. It gets held to a higher
standard than the projects it guards.

## Always
- Read existing code before writing anything. The whole CLI is ~2.7k lines; read the part you touch.
- Production-ready by default unless JB says MVP/POC.
- Tests first for gate logic (transition rules, inspect, done). A gate with no test is a suggestion.
- Tests are hermetic: point `HOME` at a temp dir. Never read or write the real `~/.lore`.
- Keep on-disk formats backward compatible: new `Ticket`/`Session` fields are `#[serde(default)]`; an old `ticket.json` must still load.
- All writes to `ticket.json` are atomic (temp file + rename) under a lock once D3 lands; never add a new plain `fs::write` to shared state.
- `inspect` is deterministic: exit 0 or non-zero, no model opinions inside it.
- Every mutating command appends one event to the event log once D4 lands.
- Confirm before any destructive operation (delete, archive, force, overwrite).
- Commit `lore/` alongside code changes. They move together.

## Never
- Push, merge, release or run `lore update` without JB's explicit OK.
- Touch `OG.md` or `MISSION.md` (human-only).
- Add a way to mark a ticket `done` that skips the gate (`--force` is allowed only if logged loudly).
- Bind the dashboard to anything but `127.0.0.1`.
- Let the builder edit tests listed in a ticket's `protect` globs.
- Edit `workspace/ticket.json` by hand; use the CLI.
- Invent subdirectories inside `lore/` outside the canonical structure.

## Conventions
- Rust, edition 2021. Errors via `anyhow`. CLI via `clap` derive.
- Keep `main.rs` as argument wiring only; behavior lives in `commands.rs` / `dashboard.rs` / `models.rs` (split further as the conductor lands).
- Event/log timestamps are UTC RFC 3339.
- Session identity is the UUID; prefixes are accepted wherever a session is taken.

## Data-loss hazards in the CLI today (LOR-7): do not trigger these on real projects
- `lore delete project` deletes the whole `lore/` folder with no prompt (docs claim it keeps project files).
- Re-running `lore create project` on an existing project silently empties its ticket list.
- Parallel `lore ticket add` loses tickets (20 parallel adds gave 19). Run ticket writes one at a time.
- Commit or back up `lore/` before running any of these, and never run them on this repo's own `lore/`.

## Known issues being fixed (see decisions/conductor-and-gates.md, F1–F9)
inspect passes vacuously on non-Cargo/npm projects; `done` ungated; non-atomic ticket writes; `run_cmd` output-pipe hang; dashboard binds 0.0.0.0; zero tests.
