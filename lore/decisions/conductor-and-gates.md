# ADR 0001 — `lore conductor`, enforced gates, event log, resumable runs

**Status:** Proposed (awaiting JB review)
**Date:** 2026-10-08
**Scope:** `~/projects/personal/lore` (Rust CLI, v0.1.0)

## Goal

JB hands the conductor (Claude Code) a task. They work out requirements together, the
conductor writes tickets with acceptance criteria into `lore/`, then runs the whole
build loop with agy (builder) and Codex (QA/reviewer) as workers. Nothing is
overlooked, no credits are wasted, and the human can watch everything from an admin
panel. Enforcement lives in code, not in prompts.

## Verified findings in the current code

| # | Finding | Where |
|---|---|---|
| F1 | `inspect` only knows Cargo and npm. For a Flutter or Wrangler project it prints "No build system detected, skipping" and **passes**. | `commands.rs` cmd_inspect |
| F2 | `ticket done` has no gate, and `ticket edit --status done` bypasses everything. No transition rules at all (backlog → done is legal). | cmd_ticket_done, cmd_ticket_edit |
| F3 | `inspect` writes no record, and a dirty working tree is only a warning. | cmd_inspect |
| F4 | `ticket.json` is read-modify-write with a plain `fs::write`: not atomic, no lock. Parallel agents can lose updates or corrupt the file. | save_ticket_file |
| F5 | `session close` deletes the session file. Sessions hold only a creation date, no run state, so nothing can resume. | cmd_session_close, models.rs |
| F6 | `session log` attaches to "first inprogress ticket, else last ticket". With parallel tickets, entries are misattributed. | cmd_session_log |
| F7 | The repo has **zero tests**. | whole repo |
| F8 | `run_cmd` pipes stdout/stderr but only reads them after the child exits. A child that writes more than the pipe buffer (~64 KB, e.g. `flutter test`) blocks forever and hits the timeout. It also kills only the direct child, not its process group. *Suspected, will be proven by a test.* | run_cmd |
| F9 | Dashboard binds `0.0.0.0`, reachable from the LAN. | dashboard.rs:176 |

## Decisions

### D1. Gates are data on the ticket
Add to `Ticket` (all `#[serde(default)]`, so existing `ticket.json` files still load):
- `verify: Vec<VerifyStep>` where `VerifyStep { name, cmd, cwd?, timeout_secs, expect }`
  and `expect` is `pass` or `red_then_green`.
- `manual: Vec<String>`, criteria a human must confirm.
- `protect: Vec<String>`, globs of test files the builder may not modify.
- `attempts: u32`, `max_attempts: u32` (default 2).

`inspect` runs the ticket's `verify` steps (plus the existing context/git/build checks,
extended with Flutter/Dart and Wrangler detection) and writes a record to
`lore/workspace/inspect/<ticket>/<n>.json`: HEAD sha, hash of protected files,
per-step exit code/duration/output tail, overall result. Exit code is 0 or non-zero.
No model opinions in `inspect`.

### D2. One transition function
Statuses become: `backlog, todo, inprogress, review, done, blocked`. All status
changes go through `transition(from, to, ctx)`.

- `done` requires: latest inspect record is green, recorded HEAD equals current HEAD,
  protected-file hash unchanged, an independent review verdict recorded, and
  `manual` is empty or approved.
- `ticket edit --status` can no longer set `done` or skip states.
- `--force` exists but is logged loudly as event `forced` and shown on the dashboard.

### D3. Safe state writes
`ticket.json` writes become atomic (temp file + rename) under an advisory file lock
(`fs2`). Read-modify-write happens inside the lock.

### D4. Append-only event log
Every mutating command appends one JSON line to `~/.lore/events.jsonl`
(`O_APPEND`, single `write` per line): `ts, session, ticket, actor, action, result,
detail`. `session log` takes an explicit `--ticket` (F6). Per-ticket `logs` stay.

### D5. Resumable runs
Conductor state lives in `~/.lore/runs/<session>/<ticket>.json`: state, attempt,
worker name, worker's own session id, worktree path, pid, started/updated.
`session close` **archives** to `~/.lore/archive/` instead of deleting.
`lore conductor resume <session>` continues from the run files.

### D6. `lore conductor`
Commands: `next | run | status | stop | resume | approve <ticket>`.
Loop per ticket: pick the highest-priority `todo` → create git worktree → build brief
from ticket + context files → call builder with timeout → run `inspect` → call a
reviewer that is **not** the author → apply attempt/budget caps → `done`, escalate or
`blocked`. Workers are configured in `lore/config.yml` (command templates), not
hard-coded. Timeouts kill the whole process group. A `STOP` file (project or
`~/.lore/STOP`) halts before the next step. Tickets with open `manual` items go to
`blocked` (needs-human) and the loop moves on.

### D7. Dashboard
Add read endpoints `/api/events` and `/api/runs`; views: board, live feed, ticket
detail, alerts. Controls limited to STOP and approve-manual. Bind to `127.0.0.1` (F9).

## Rejected
- Separate `conductor` binary: duplicates the ticket and session model.
- Hermes kanban as the board: second source of truth beside `lore ticket`.
- Model-based checks inside `inspect`: gates must be deterministic.

## Build order (each slice reviewed before the next)
0. Tests harness plus fixes F9, F4, F8 (small, high value, no schema change)
1. Event log (D4) and archive-on-close (D5 part)
2. Ticket schema, `transition()`, gated `done` (D1, D2)
3. `inspect` runs `verify`, records results, Flutter/Wrangler detection
4. `lore conductor` (D6) with fake workers first, real agy/Codex second
5. Dashboard (D7)

## Test plan for lore itself

Principle: lore is the gate for everything else, so lore must be tested harder than
what it guards. All tests are hermetic: `HOME` points at a temp dir, so
`~/.lore` is never touched.

**Layer 1: unit (`cargo test`, in-module)**
- Status parsing and display round-trip; old `ticket.json` (no new fields) still loads.
- `transition()` table: every legal edge passes, every illegal edge errors,
  `done` refused without a green record, refused when HEAD moved, refused when
  protected hash changed, refused without independent review.
- Reviewer ≠ author rule.
- Session prefix lookup: exact, unique prefix, ambiguous, none.
- `parse_csv`, glob matching for `protect`.
- Event serialization: one valid JSON line per event.

**Layer 2: integration (`tests/*.rs`, spawn the real binary with temp `HOME`)**
- Full flow: create project → add ticket with `verify` → start → inspect red →
  `done` refused → fix → inspect green → `done` accepted.
- `ticket edit --status done` refused.
- Concurrency: N processes add tickets at once; final file valid JSON, N tickets,
  no duplicate ids (guards F4).
- Atomicity: kill a writer mid-write; `ticket.json` is never truncated.
- `run_cmd`: child emitting 5 MB of output completes (guards F8); child that
  hangs is killed along with its grandchildren at the timeout.
- Inspect on a fake Flutter/Wrangler fixture detects the project type and runs it.
- Event log: every mutating command appends exactly one line; read-only commands none.
- `session close` archives; `conductor resume` after a simulated crash continues
  from the right state.
- Dashboard: server binds only to 127.0.0.1; endpoints return valid JSON.

**Layer 3: conductor with fake workers**
Scripted fake builder/reviewer binaries: succeed, fail once then succeed, fail
always (expect escalate then `blocked` after `max_attempts`), hang (expect timeout
kill), edit a protected test file (expect rejection), self-review (expect rejection),
STOP file (expect halt before next step), `manual` criteria (expect parked, loop
continues). No real model is called in CI.

**Layer 4: one real-worker smoke test, run by hand**
Tiny ticket on a scratch repo through real agy plus Codex, once per release.

CI: `cargo test` plus `cargo clippy -- -D warnings` on every push (a GitHub
Actions file is added in slice 0, committed only with JB's approval).
