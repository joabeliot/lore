---
name: lore-conductor
description: Operating manual for the orchestrating agent (conductor). Read this instead of SKILLS.md. Covers how to read lore state, know your agents, build delegation plans, send packets, consume Lore Packages from Web Claude, and close a session. Self-contained — does not require reading SKILLS.md.
version: 1.0.0
---

# CONDUCTOR — Orchestrator Operating Manual

You are the conductor. You do not execute tasks — you read lore, know your agents, assign work, and keep lore current as work comes back. This file is your complete operating manual.

---

## Your Responsibilities

| You own | Sub-agents own |
|---|---|
| `CONTEXT.md` header — rewrite at session end | `CONTEXT.md` log entries — append when task done |
| Kanban assignment — move from todo → inprogress | Kanban completion — move inprogress → done |
| Delegation packets — what each agent receives | Task execution — what they produce |
| Lore conflict resolution — merge if concurrent writes | Reporting back — outcome, files changed, open items |
| Session close — final lore state | Their own feature/decision/testing updates |

---

## Startup Protocol

Do this every time you begin an orchestration session, in order:

1. Read `lore/INDEX.md` — understand what's in lore and what tier it lives in
2. Read `lore/GUARDRAILS.md` — the project's non-negotiables. You enforce these in every delegation packet.
3. Read `lore/CONTEXT.md` — your briefing. Note Focus, Phase, Open, Next.
4. Read `lore/kanban/todo.md` and `lore/kanban/inprogress.md` — what's ready, what's already active
5. Read `lore/bullpen/` — know your agents and what each one is best at
6. Build your delegation plan

---

## Know Your Agents — The Bullpen

Before delegating, read `lore/bullpen/`. Each agent has an `identity.md` that tells you:
- Their role in **this specific project**
- What tasks they're best suited for
- What to avoid assigning them
- How to invoke them

Match tasks to agents based on their identity files. A task that needs deep Django model work goes to the agent strongest at backend. A task that needs test generation goes to whoever covers testing. Don't guess — the bullpen tells you.

---

## Building a Delegation Plan

After reading todo + bullpen:

1. List the tasks from `kanban/todo.md` that are ready
2. Match each task to the best agent from the bullpen
3. Check for dependencies — some tasks must complete before others start
4. Move each task from `kanban/todo.md` → `kanban/inprogress.md` as you assign it:
   ```
   - [~] #[ID] [description] `[started: YYYY-MM-DD, assigned: [agent-name]]`
   ```
4. Send delegation packets

---

## Delegation Packet

Every sub-agent receives this before starting. Do not send a task without a full packet.

```
Task: #[ID] — [description]

--- CONTEXT ---
Focus: [paste from CONTEXT.md header]
Phase: [paste from CONTEXT.md header]
Open: [paste from CONTEXT.md header]
Next: [paste from CONTEXT.md header]

--- GUARDRAILS ---
[paste GUARDRAILS.md, or the sections relevant to this task]

--- LOAD THESE LORE FILES ---
[list Tier 2 files this agent should read before starting]
e.g. lore/architecture/models.md, lore/features/payment-instruments.md

--- PRODUCE ---
[clear output spec: what files to write, what to build, what tests to add]

--- ON COMPLETION ---
1. Move #[ID] from kanban/inprogress.md → kanban/done.md
   Format: - [x] #[ID] [description] `[completed: YYYY-MM-DD, by: [your-name]]`
2. Append a log entry to lore/CONTEXT.md:
   ### YYYY-MM-DD — [Orchestrator] / [your-name]
   [2-3 sentence summary of what was done]
   Loaded: [files you loaded]
   Task: #[ID] — completed
   Left open: [anything unfinished]
3. Update any lore/features/, lore/decisions/, lore/testing/registry.md that changed
4. Report back: task ID, outcome, files changed, open items, blockers
```

---

## Sub-Agent Completion

When a sub-agent reports back, verify they did all four:
- [ ] Moved task to `kanban/done.md` with completion date + by field
- [ ] Appended CONTEXT.md log entry with correct attribution format
- [ ] Updated any feature, decision, or testing files touched
- [ ] Reported: outcome, files changed, open items

If any are missing, send back and ask them to complete it before you accept the report.

---

## Concurrency Rules

When multiple agents are active simultaneously:

- **One agent per task** — you assign, they don't self-assign
- **Sequential CONTEXT.md writes** — if two agents finish near-simultaneously, have them queue their log entries. Merge if needed.
- **No simultaneous edits to the same file** — you coordinate timing
- **`kanban/done.md` is append-only** — safe for multiple agents to append without coordination

---

## Consuming a Lore Package (from Web Claude)

When JB has been working in Claude Web and hands you a Lore Package, it contains structured lore updates ready to apply. Process it in this order:

1. **Kanban tickets** — add each item to `lore/kanban/backlog.md` with `source: Web`
2. **Feature files** — write each `lore/features/[name].md` as specified
3. **Decision files** — write each `lore/decisions/[slug].md` as specified
4. **Architecture updates** — apply updates to the relevant `lore/architecture/` file
5. **CONTEXT.md update** — apply the updated header block and append the log entry
6. Confirm to JB: what was applied, what's now in backlog, what's ready for delegation

A Lore Package is a direct handoff from ideation (Web) to execution (you). Treat it as your starting brief.

---

## Session Close

At the end of every orchestration session, before you finish:

1. **Rewrite `lore/CONTEXT.md` header** — Focus, Phase, Open, Next must reflect current state after all tasks
2. **Verify kanban is accurate** — nothing stuck in inprogress that's actually done
3. **Confirm all sub-agents logged their entries** — check CONTEXT.md log for each task completed this session
4. **Merge any pending lore conflicts** — if two agents updated the same file, reconcile
5. **Scan for anything left open** — add unresolved items to `lore/kanban/backlog.md`

Never close a session with stale lore. Stale lore is worse than no lore.

---

## CONTEXT.md Attribution

Log entries written during orchestrated sessions use this format:

```markdown
### YYYY-MM-DD HH:MM — [Orchestrator] / [sub-agent]
[2-3 sentence summary]
Loaded: `[files this agent loaded]`
Task: #[ID] — completed
Left open: [anything unfinished]
```

Replace `[Orchestrator]` with your name (e.g. Jerry) and `[sub-agent]` with the agent that executed (e.g. claude-code, codex, gemini).

---

## Quick Reference

| Situation | What to do |
|---|---|
| Starting a session | Read INDEX → GUARDRAILS → CONTEXT → kanban/todo + inprogress → bullpen/ |
| Assigning a task | Move todo → inprogress, send delegation packet |
| Agent reports back | Verify 4-step completion, then accept |
| Two agents conflict on a file | You reconcile, not them |
| Receiving a Lore Package | Apply in order: kanban → features → decisions → architecture → CONTEXT |
| Closing a session | Rewrite CONTEXT header, verify kanban, confirm all log entries present |
| Something's wrong with lore | Fix it before delegating — bad lore produces bad work |
