---
name: lore
description: Initialize, update, and maintain the lore project memory system. Use this skill whenever the user mentions init lore, set up project memory, generate lore from an existing repo, update CONTEXT.md, log a decision, add a feature file, or bridge ideas from Claude Web into Claude Code. Trigger even if the user doesn't say "lore" explicitly — if they're trying to capture project state, decisions, architecture, or current focus for AI context, this skill applies.
version: 2.1.0
---

# SKILL: lore — Project AI Readiness Layer

## What This Is

`lore` is a folder you commit to your project. It's the single source of truth — the bible — that any developer, AI agent, or new team member reads to understand the project. Not the code. The *project*: why it exists, how it's designed, what's been decided, what's being built, and what the rules are.

Think of `lore` as the interface between humans and the codebase. Code tells you *what*. `lore` tells you *why*, *how*, and *what's next*.

**Use this skill to:**
- Init `lore` on a new project from scratch
- Read an existing repo and generate `lore` from what's already there
- Update `lore` files during or after a dev session
- Manage the kanban board
- Log decisions, features, and test coverage
- Bridge ideas from Claude Web into Claude Code context

---

## Folder Structure

```
project/
  CLAUDE.md                  ← AI session entry point (always loaded)
  lore/
    INDEX.md                 ← Tier 1: TOC + loading guide (always loaded)
    GUARDRAILS.md            ← Tier 1: project rules (always loaded)
    CONTEXT.md               ← Tier 1: current state + session log (always loaded)
    OG.md                    ← 🔒 Human-only: raw dev journal
    MISSION.md               ← 🔒 Human-only: project soul
    CHANGELOG.md             ← Hook-generated: git commit history
    kanban/
      backlog.md             ← Captured, not yet scheduled
      todo.md                ← Scheduled, not started
      inprogress.md          ← Active work
      done.md                ← Completed
    architecture/
      overview.md            ← Service map, data flow, infra topology
      models.md              ← Data models and schemas
      apis.md                ← API contracts and external services
    features/                ← One .md per feature
    ideas/                   ← Pre-feature captures (unvalidated)
    testing/
      registry.md            ← What's covered, what's not
    decisions/               ← Architecture Decision Records (one per decision)
    skills/
      custom/                ← Project-specific Claude skills
      skills.yml             ← Skill registry
```

---

## Tiered Loading

Not everything loads every session. This keeps token cost low and context relevant.

| Tier | Files | When |
|---|---|---|
| **1 — Always** | `INDEX.md`, `GUARDRAILS.md`, `CONTEXT.md` | Every session via CLAUDE.md hook |
| **2 — On-Demand** | `kanban/`, `architecture/`, `features/`, `testing/`, `decisions/` | Load only what the task requires |
| **Never Auto** | `OG.md`, `MISSION.md`, `CHANGELOG.md` | Human or agent pulls explicitly |

**Rule:** Start every session reading Tier 1 only. Load Tier 2 files when the task requires them — name which files you loaded in your session log entry.

---

## Agent Session Workflow

A concrete sequence for any agent operating in a project with `lore`. Follow this every session, no exceptions.

### Starting a session
1. Read `lore/INDEX.md` → `lore/GUARDRAILS.md` → `lore/CONTEXT.md` (Tier 1)
2. Note the **Focus**, **Phase**, **Open**, and **Next** fields from CONTEXT.md — this is your briefing
3. If picking up a task: read `lore/kanban/todo.md` and `lore/kanban/inprogress.md`
4. Load Tier 2 files only as the task requires — announce which ones you load

### During a session
- Load Tier 2 files as needed, name what you loaded
- Move kanban tasks as their state changes — don't wait until the end
- Log significant decisions to `decisions/` as you make them, not in bulk at session end
- If you discover a gap in lore (missing feature doc, stale architecture), fix it as you go

### Ending a session
Do all of the following before closing:
1. **Rewrite `CONTEXT.md` header** — Focus, Phase, Open, Next must reflect current state
2. **Append a log entry** — compact, 3-5 lines (see CONTEXT.md contract for format)
3. **Move kanban tasks** — anything completed goes to `done.md`; newly proposed tasks go to `backlog.md`
4. **Update feature files** for anything that started, changed, or completed
5. **Update `testing/registry.md`** if tests were added or removed
6. **Write decision files** for any significant architectural choices made this session

Never skip the session-end update. A lore that isn't updated after every session is a lore that lies.

---

## File Contracts

Each file has a defined audience, purpose, and update rhythm.

---

### `INDEX.md`
**Audience:** Claude Code — first stop every session.
**Purpose:** Lightweight TOC. Tells agents what exists, what tier it lives in, and any agent-proposed additions pending review.
**Rule:** Keep it under 60 lines. Dense, not descriptive.

**Template:**
```markdown
# lore Index

**Always load:** `GUARDRAILS.md`, `CONTEXT.md`

## Tier 2 — Load When Relevant
| File / Dir | Load when |
|---|---|
| `kanban/` | Planning work or picking up a task |
| `architecture/` | Making structural or data model changes |
| `features/[name].md` | Working on that specific feature |
| `testing/registry.md` | Writing or reviewing tests |
| `decisions/` | Making or revisiting a significant decision |

## Human-Only (Never auto-load)
`OG.md` — raw dev journal  
`MISSION.md` — project soul  
`CHANGELOG.md` — git history (hook-generated)

## Proposed Additions
Agent-suggested lore expansions pending human review. If approved, they become canonical.
- [none yet]
```

---

### `GUARDRAILS.md`
**Audience:** Claude Code + developers — always loaded.
**Purpose:** Project-wide rules. What to always do, never do, and how conventions work here.
**Rule:** The most read file after CONTEXT.md. Keep it honest and current. Split by domain if needed.

**Template:**
```markdown
# Guardrails

## Always
- [Pattern to always follow]

## Never
- [Pattern to never use in this project]

## Conventions
- [Naming, structure, or style decisions specific to this project]

## Backend
- [Backend-specific rules]

## Frontend
- [Frontend-specific rules]
```

---

### `CONTEXT.md`
**Audience:** Claude Code — always loaded.
**Purpose:** Dense current state + chronological session log. This is how any agent picks up where the last one left off without re-reading the whole codebase.
**Rule:** The header block is rewritten each session. Log entries are appended, never deleted. Keep entries compact — 3-5 lines max.

**Template:**
```markdown
# Context

**Focus:** [what's actively being built — one line]
**Phase:** [Alpha / Beta / Prod / R&D]
**Open:** [open thread], [open thread]
**Next:** [next task], [next task]

---

## Log

### YYYY-MM-DD — [Dev Name]
[2-3 sentence summary of what was asked, what was done, and what the state is now]
Loaded: `architecture/models.md`, `features/auth.md`
Left open: [anything unfinished or deferred]

---
```

**Multi-agent log format** (Hermes orchestrated sessions):
```markdown
### YYYY-MM-DD — [Dev Name] / [sub-agent]
[2-3 sentence summary]
Loaded: `architecture/models.md`
Task: #[ID] — completed / in progress
Left open: [anything unfinished]
```

Format: `[Human] / [Sub-agent]` makes it always clear who orchestrated and who executed.

**How to write a good log entry:**
- Summarize intent + outcome in 2-3 sentences. Not a transcript.
- List files loaded this session so the next agent knows what context was available.
- Flag anything left open so the next session starts where this one left off.

---

### `OG.md` 🔒
**Audience:** You — the developer. Claude reads it, never writes it.
**Rule:** Never AI-generated. Never structured. This is your unfiltered voice.
**Purpose:** Raw dev journal. Doubts, instincts, hunches, things you want to remember but aren't ready to formalize. Claude reads this to understand your intent and vibe when making judgment calls.

**Prompt to start:** *"What's going on in my head about this project right now?"*

---

### `MISSION.md` 🔒
**Audience:** You — the developer. Claude reads it, never writes it.
**Rule:** Never AI-generated. This is the soul of the project.
**Purpose:** The *why*. Not operational detail — why it should exist, who it's for, what success looks like. Claude reads this when making decisions that require understanding what the project is trying to *be*, not just what it's currently doing.

**Must answer:** What is this? Who is it for? What problem does it solve? Why should it exist? What does success look like?

**Prompt to write it:** *"If I had to explain this to someone who'd never heard of it, and I wanted them to understand not just what it does but why it matters — what would I say?"*

---

### `CHANGELOG.md`
**Audience:** Humans + Claude Code — never auto-loaded, pull on demand.
**Purpose:** Auto-generated commit history. The evolution of the product in git form.
**Rule:** Written by the `post-commit` git hook, not by agents or humans. Never manually edited.

**Format (auto-generated):**
```markdown
# Changelog

## YYYY-MM-DD HH:MM — [short hash] — [commit message]
[commit body if present]

---
```

---

### `kanban/`
**Audience:** Claude Code + humans — load when planning or picking up work.
**Purpose:** The agent work queue. Human drops tickets in backlog. Agents pick up from todo, move through in-progress, land in done.
**Rule:** IDs are permanent. Once assigned, an ID never changes even when the ticket moves. Agents must update kanban when they start or finish a task.

**`kanban/backlog.md`** — captured, not yet scheduled:
```markdown
# Backlog

- [ ] #001 [Task description] `[source: JB, YYYY-MM-DD]`
- [ ] #002 [Task description] `[source: Agent, YYYY-MM-DD]`
```

**`kanban/todo.md`** — scheduled, not started:
```markdown
# Todo

- [ ] #001 [Task description] `[scheduled: YYYY-MM-DD]`
```

**`kanban/inprogress.md`** — active:
```markdown
# In Progress

# Solo session
- [~] #001 [Task description] `[started: YYYY-MM-DD, assigned: claude-code]`

# Hermes orchestrated — multiple agents
- [~] #002 [Task A] `[started: YYYY-MM-DD, assigned: claude-code]`
- [~] #003 [Task B] `[started: YYYY-MM-DD, assigned: codex]`
```

**`kanban/done.md`** — completed:
```markdown
# Done

- [x] #001 [Task description] `[completed: YYYY-MM-DD, by: claude-code]`
```

**Agent kanban rules:**
1. When picking up a task: move from `todo.md` to `inprogress.md`, add started date + `assigned: [your-name]`
2. When finishing a task: move from `inprogress.md` to `done.md`, add completed date + `by: [your-name]`
3. When proposing a new task: add to `backlog.md` with `source: Agent`
4. Never delete entries — always move them
5. In orchestrated sessions: Hermes assigns tasks, sub-agents never self-assign from todo

---

### `architecture/overview.md`
**Audience:** Claude Code + humans — load when making structural changes.
**Purpose:** How the system is designed. Service map, data flow, infra topology, external dependencies.
**Rule:** Updated by agent when architecture changes. Covers the *shape* of the system, not every field.

---

### `architecture/models.md`
**Audience:** Claude Code + humans.
**Purpose:** Data models and schemas — field names, types, relationships, constraints, quirks.
**Include:** Soft deletes, multi-tenancy patterns, custom managers, naming conventions, anything non-obvious.

---

### `architecture/apis.md`
**Audience:** Claude Code + humans.
**Purpose:** API contracts — internal endpoints and external services.
**Include:** Base URLs, auth method, key endpoints, rate limits, versioning, known gotchas.

---

### `features/[feature-name].md`
**Audience:** Claude Code + humans — load when working on that feature.
**Purpose:** One file per committed or in-progress feature.

**Template:**
```markdown
# Feature: [Name]

**Status:** Idea / In Progress / Done / Paused

## What It Does
[What problem it solves and how]

## Edge Cases
- [Known edge case]

## Open Questions
- [Unresolved question]

## Notes
[Anything else relevant]
```

---

### `ideas/[idea-name].md`
**Audience:** You + Claude Code.
**Purpose:** Pre-feature, unvalidated. Low friction capture.
**Rule:** No strict format. Write enough to remember the idea and the instinct behind it. Promote to `features/` when committed.

---

### `testing/registry.md`
**Audience:** Claude Code — load when writing or reviewing tests.
**Purpose:** Living map of test coverage. Agent updates this when tests are added or removed.

**Template:**
```markdown
# Test Registry

## Covered
| Area | Test file | Type | Notes |
|---|---|---|---|
| Auth / login | `tests/test_auth.py` | Unit | Covers happy path + wrong password |

## Not Covered
- Payment webhook failure cases
- Concurrent session handling

## Known Gaps
- [Gap that's accepted and won't be covered]
```

---

### `decisions/[decision-slug].md`
**Audience:** Claude Code + humans — load when making or revisiting a significant decision.
**Purpose:** Architecture Decision Records. Prevents re-litigating what's already been decided.
**Rule:** One file per decision. Filename is a short kebab-case slug of the decision title.

**Template:**
```markdown
# [Decision Title]

**Date:** YYYY-MM-DD
**Status:** Decided / Superseded / Under Review

## Decided
[What was chosen]

## Why
[The reasoning — constraints, tradeoffs, context]

## Rejected
[What else was considered and why it lost]

## Consequences
[What this means going forward — what gets easier, what gets harder]
```

---

### `skills/custom/`
Project-specific Claude skills. Same SKILL.md format.
Use for patterns unique to this repo: how views are written, how errors are handled, how migrations work, how tests are structured.

---

### `skills/skills.yml`
Registry of all skills in use — like `requirements.txt` for Claude skills.

**Format:**
```yaml
skills:
  - name: lore
    version: 2.1.0
    source: https://github.com/joabeliot/lore
    notes: using as-is

  - name: my-custom-skill
    source: custom
    notes: written for this project
```

---

## Session Update Rule

At the end of every session, Claude must:

1. **Update `CONTEXT.md` header** — rewrite the Focus, Phase, Open, Next lines to reflect current state
2. **Append a log entry** to `CONTEXT.md` — compact, 3-5 lines, what was done and what's open
3. **Update kanban** — move any tasks that changed state
4. **Update feature files** if a feature was started, completed, or changed
5. **Log decisions** to `decisions/` if a significant architectural choice was made
6. **Update `testing/registry.md`** if tests were added or removed

**What Claude never touches:**

| File | Why |
|---|---|
| `OG.md` | Human-only. Always. |
| `MISSION.md` | Human-only. Always. |
| `CHANGELOG.md` | Hook-generated. Always. |

---

## Multi-Agent Protocol (Hermes)

When Hermes orchestrates multiple sub-agents (Claude Code, Codex, Gemini CLI), `lore` becomes the shared state layer between all of them. This protocol keeps every agent synchronized and prevents conflicts.

### Session Types

| Type | Who | Protocol |
|---|---|---|
| **Solo** | JB + one agent | Standard Agent Session Workflow |
| **Orchestrated** | Hermes + sub-agents | This protocol |

### Hermes Startup Protocol

When Hermes begins an orchestration session:
1. Read Tier 1: `INDEX.md` → `GUARDRAILS.md` → `CONTEXT.md`
2. Read `kanban/todo.md` and `kanban/inprogress.md` — understand what's ready and what's already active
3. Build the delegation plan: which tasks to assign, to which agents, in what order
4. Assign tasks via delegation packets (see below)
5. Monitor sub-agents; merge lore when they report back

### Delegation Packet

What Hermes sends to each sub-agent when delegating a task. Every sub-agent receives this before starting:

```
Task: #[ID] [description]

Context (paste CONTEXT.md header):
  Focus: ...
  Phase: ...
  Open: ...
  Next: ...

Guardrails: [paste GUARDRAILS.md or the sections relevant to this task]

Load these lore files: [list Tier 2 files relevant to this task]

Produce: [clear output spec — what files to write, what endpoints to build, what tests to write]

On completion you must:
  1. Move #[ID] from kanban/inprogress.md to kanban/done.md (add completed date + by: [your-name])
  2. Append a log entry to lore/CONTEXT.md using the multi-agent format (JB / [your-name])
  3. Update any feature files, decisions, or test registry that changed
  4. Report back to Hermes: task ID, outcome, files changed, what's left open
```

### Sub-Agent Completion Protocol

When a sub-agent finishes, it must do all of the following before reporting back to Hermes:

1. Move task from `kanban/inprogress.md` → `kanban/done.md`
2. Append a log entry to `CONTEXT.md` using the multi-agent format
3. Update any `features/`, `decisions/`, or `testing/registry.md` that changed
4. Report back to Hermes:
   - Task ID and status (completed / partial / blocked)
   - Files changed
   - Anything left open or deferred
   - Any blockers that need Hermes's attention

### Concurrency Rules

These rules prevent lore conflicts when multiple agents are active:

- **One agent per task** — Hermes assigns; sub-agents never self-assign
- **Sequential lore writes** — if two agents finish near-simultaneously, they queue writes; Hermes merges if needed
- **No simultaneous file edits** — two agents must never write to the same file at the same time; Hermes coordinates timing
- **`kanban/done.md` is append-only** — safe for multiple agents to append sequentially without conflict
- **`CONTEXT.md` header is Hermes's** — sub-agents append log entries; only Hermes rewrites the header block at session end

### Hermes Orchestration Loop

```
1. Read lore Tier 1 + kanban/todo.md + kanban/inprogress.md
2. Build task assignments based on todo + current context
3. Send delegation packets to sub-agents (can run in parallel)
4. Receive completion reports from sub-agents
5. Merge any lore conflicts
6. Rewrite CONTEXT.md header with current state
7. Repeat or close session
```

### What Hermes Owns vs Sub-Agents

| Responsibility | Hermes | Sub-Agent |
|---|---|---|
| `CONTEXT.md` header | Rewrites at session end | Appends log entries only |
| `kanban/` task assignment | Assigns from todo | Never self-assigns |
| `kanban/` task movement | Monitors overall state | Moves their own tasks |
| `features/`, `decisions/`, `testing/` | — | Updates files relevant to their task |
| Lore conflict resolution | Merges conflicts | Reports conflicts upward |

---

## Hook Automation

`lore` uses git hooks to automate what agents shouldn't have to manually track.

### `post-commit` → `lore/CHANGELOG.md`

The `post-commit` hook appends every commit to `CHANGELOG.md` automatically. Install it once per project with:

```bash
# From the lore repo root
./install.sh --hooks /path/to/your/project
```

Or manually copy `hooks/post-commit.sh` to your project's `.git/hooks/post-commit` and make it executable:

```bash
cp /path/to/lore/hooks/post-commit.sh your-project/.git/hooks/post-commit
chmod +x your-project/.git/hooks/post-commit
```

### What hooks do vs what agents do

| Responsibility | Hook | Agent |
|---|---|---|
| `CHANGELOG.md` | Auto-appends on every commit | Never touches |
| `CONTEXT.md` | — | Updates header + appends log |
| `kanban/` | — | Moves tasks between files |
| `architecture/` | — | Updates when structure changes |
| `testing/registry.md` | — | Updates when tests change |
| `decisions/` | — | Creates new file per decision |

---

## Agent Creative Additions

Agents are allowed — and encouraged — to propose creative additions to `lore` when they'd serve the project. A creative addition might be:
- A new subfolder not in the canonical structure (e.g., `lore/incidents/` for postmortems)
- A new field or section in an existing file
- A project-specific convention worth formalizing

**Rule:**
1. If you think a creative addition would help the project, **build it and use it**
2. Add an entry to `lore/INDEX.md` under `Proposed Additions` describing what you added and why
3. The human reviews and decides whether it becomes canonical in the lore framework repo
4. Never silently add to the canonical folder structure — only to `Proposed Additions`

---

## Init: New Project

When asked to init `lore` on a new project:

1. Create the full folder structure
2. Stub every file with its template
3. Fill `CLAUDE.md` with what's known: project name, stack, purpose, Session Rule, and lore Index
4. Leave `OG.md` blank with the prompt: *"What's on your mind about this project?"*
5. Leave `MISSION.md` blank with the prompt: *"What is this project and why should it exist?"*
6. Set `CONTEXT.md` header with placeholder values and an empty log section
7. Initialize `kanban/backlog.md` with an empty list, others as stubs
8. Ask the developer to confirm: stack, key rules, and current focus before finalizing `CLAUDE.md`

**What NOT to invent:**
- Do not populate `architecture/models.md` with field names — stub only
- Do not populate `architecture/apis.md` with endpoints — stub only
- Do not create files inside `features/`, `ideas/`, or `decisions/` — leave dirs empty
- Do not add `testing/registry.md` coverage rows — stub only

Inventing content contaminates `lore` with hallucinated facts that look real. A blank stub is better than a confident wrong guess.

---

## Init: Existing Repo

When pointed at a repo that has no `lore`:

**Step 1 — Check for CLAUDE.md**
- If `CLAUDE.md` exists: inject the lore block (Session Rule + lore Index) without overwriting the rest
- If it doesn't exist: create it from the template

**Step 2 — Read the repo**
Scan `README.md`, package files (`requirements.txt`, `package.json`, `pubspec.yaml`, `Dockerfile`), and folder structure to infer stack and architecture.

**Step 3 — Generate `lore/`** using canonical paths only:
- `lore/architecture/overview.md` from inferred system design
- `lore/architecture/models.md` from model/schema files found
- `lore/architecture/apis.md` from route/serializer files found
- `lore/CONTEXT.md` with header ready for first session entry
- `lore/GUARDRAILS.md` with reasonable defaults from what you found
- `lore/OG.md` and `lore/MISSION.md` left blank with human prompts
- `lore/kanban/` stubbed with empty files

**Step 4 — Flag gaps**
Consolidate everything that couldn't be inferred into a numbered list. Never silently skip.

**Critical: never invent subdirectories** outside the canonical structure. All inferred content goes into canonical files. A non-standard `lore/` layout breaks compatibility with every agent that reads it.

---

## Web-to-Code Bridge Workflow

```
1. Think through ideas, architecture, or features in Claude Web
2. Ask Claude Web to generate or update a lore file from the discussion
3. Paste the output into the correct lore file in your repo
4. Claude Code picks it up next session via CLAUDE.md
5. After building, Claude Code updates CONTEXT.md and kanban
6. Commit lore alongside code changes
```

**Rule:** `OG.md` and `MISSION.md` are always written by the human. Every other file can be AI-generated or AI-updated — but should be human-reviewed before committing.

---

## Keeping `lore` Healthy

- `CONTEXT.md` header is rewritten every session — stale focus is worse than no focus
- Log entries are appended every session — never skip it
- Kanban always reflects reality — if something's done, move it
- Feature files get updated when features change — not just when they're created
- `testing/registry.md` grows with the test suite
- `decisions/` prevents re-litigating what's already settled
- Commit `lore/` alongside code — they should move together

---

## Evolving This Skill

This skill lives at `~/.claude/skills/lore/SKILL.md` globally, installed from the lore repo.

When an agent proposes a creative addition that JB approves, it gets added to this canonical SKILL.md and versioned. Projects then update by running `./install.sh` again.

The goal: any project with `lore/` is immediately legible to any Claude instance, any developer, and any future agent — with zero onboarding friction.
