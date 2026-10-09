---
name: lore
description: Initialize, update, and maintain the lore project memory system. Use this skill whenever the user mentions init lore, set up project memory, generate lore from an existing repo, update STATE.md or CONTEXT.md, log a decision, add a feature file, or bridge ideas from Claude Web into Claude Code. Trigger even if the user doesn't say "lore" explicitly — if they're trying to capture project state, decisions, architecture, or current focus for AI context, this skill applies.
version: 3.0.0
author: Joab Eliot
license: MIT
metadata:
  hermes:
    tags: [lore, project-memory, context, cli, tickets]
---

# SKILL: lore — Project AI Readiness Layer

## What This Is

`lore` is a folder you commit to your project. It's the single source of truth — the bible — that any developer, AI agent, or new team member reads to understand the project. Not the code. The *project*: why it exists, how it's designed, what's been decided, what's being built, and what the rules are.

Think of `lore` as the interface between humans and the codebase. Code tells you *what*. `lore` tells you *why*, *how*, and *what's next*.

> **📖 Lore is the bible of this project.** Every step of every session starts and ends here. Never skip reading it. Never let it go stale. When lore and your instinct disagree, update lore first — your instinct is either new information that belongs in lore, or a mistake that lore protects you from. When in doubt, check lore. When you change the project, update lore. This is the first file loaded and the last file written in every session. No exceptions.

**Use this skill to:**
- Init `lore` on a new project from scratch
- Read an existing repo and generate `lore` from what's already there
- Update `lore` files during or after a dev session
- Manage tickets via the `lore` CLI
- Log decisions, features, and test coverage
- Bridge ideas from the ideation layer into execution

---

## Folder Structure

> **Status — profiles are the target structure.** This section describes the shared core plus the `backend` / `frontend` profiles (LOR-8). `lore create project` does **not** scaffold them yet (tracked in LOR-9); until it does, new projects get the **legacy structure** in the box below, and agents create profile folders by hand. Existing projects keep working: if `STATE.md` is absent, treat the `CONTEXT.md` header as the state and the `CONTEXT.md` log as the log. Migration is described under *Migrating a legacy lore*.

### Shared core (every project)

```
project/
  CLAUDE.md                  ← AI session entry point (always loaded)
  lore/
    INDEX.md                 ← Tier 1: TOC + loading guide
    GUARDRAILS.md            ← Tier 1: rules true on every machine
    STATE.md                 ← Tier 1: current Focus / Phase / Open / Next only
    log/YYYY-MM.md           ← Tier 1 (latest entries): dated session entries, one file per month
    sessions/                ← Raw auto-captured session logs. Never auto-loaded
    local/                   ← Untracked (gitignored): this machine's accounts, paths, limits
    domain/
      glossary.md            ← Product terms and what they mean
      rules/<rule>.md        ← Business rules, one per file
    features/<area>/<flow>.md  ← One file per flow (code-linked)
    ideas/                   ← Pre-feature captures (unvalidated)
    decisions/NNNN-<slug>.md ← Numbered decision records
    testing/registry.md      ← What's covered, what's not
    references.md            ← Pointers to docs that live outside lore/
    bullpen/<agent>/identity.md  ← Agent roster (see Bullpen)
    skills/                  ← custom/ + skills.yml
    workspace/ticket.json    ← CLI-managed ticket state (never edit directly)
    OG.md                    ← 🔒 Human-only: raw dev journal
    MISSION.md               ← 🔒 Human-only: project soul
    CHANGELOG.md             ← Hook-generated: git commit history
```

### Backend profile adds

```
    architecture/
      overview.md            ← Services, data flow, infrastructure
      services/<name>.md     ← One per service or app module (code-linked)
      data/models.md         ← Schemas, relationships, constraints
      data/migrations.md     ← Migration order, gotchas, irreversible steps
      jobs.md                ← Queues, scheduled tasks, background workers
      security.md            ← Auth, permissions, where secrets live (never the values)
    apis/<surface>.md        ← Endpoints this project exposes, per surface or version (code-linked)
    contracts/
      consumers/<client>.md  ← Who calls this backend and what they depend on (code-linked)
      upstream/<service>.md  ← Third-party and internal services this backend calls (code-linked)
    ops/
      environments.md        ← Hosts and config keys per environment (names only)
      deploy.md              ← How a deploy happens, order across services
      runbooks/<task>.md
      incidents/YYYY-MM-DD-<slug>.md
      observability.md       ← Dashboards, alerts, logs
```

### Frontend profile adds (mobile and web)

```
    architecture/
      overview.md            ← Layers, state management, dependency injection
      modules/<area>.md      ← One per code area, mirroring the source tree (code-linked)
      navigation.md          ← Routes, gates, deep links
      state.md               ← Where state lives, caching, persistence
      environments.md        ← Hosts, env keys, build flavours (names only)
    design/
      system.md              ← Colours, type, spacing, component rules
      copy.md                ← Voice and wording rules
    contracts/<service>.md   ← Each backend/third-party API the app consumes (code-linked)
    platform/
      ios.md, android.md, web.md  ← Permissions, entitlements, store setup, gotchas
    analytics.md             ← Events, what they mean, where they go
    ops/
      release.md             ← How a build is cut, version rules, release history
      runbooks/<task>.md
```

A project gets **only** its profile's folders. Do not scaffold a folder just to leave it empty — create it when there is something true to put in it. A web app that is also a thin backend picks the profile that matches most of its code and adds individual files from the other by hand (log it under `Proposed Additions`).

### Legacy structure (what `lore create project` scaffolds today)

```
    INDEX.md  GUARDRAILS.md  CONTEXT.md  OG.md  MISSION.md  CHANGELOG.md
    workspace/ticket.json
    architecture/{overview,models,apis}.md
    features/  ideas/  decisions/  testing/registry.md  bullpen/  skills/
```

---

## Tiered Loading

Not everything loads every session. This keeps token cost low and context relevant.

| Tier | Files | When |
|---|---|---|
| **1 — Always** | `INDEX.md`, `GUARDRAILS.md`, `STATE.md`, latest `log/` entries | Every session via the session-start hook |
| **2 — On-Demand** | `architecture/`, `apis/`, `contracts/`, `domain/`, `features/`, `design/`, `platform/`, `ops/`, `testing/`, `decisions/`, `bullpen/` | Load only what the task requires |
| **CLI-managed** | `workspace/ticket.json` | Never read directly — use `lore ticket list` / `lore session status` |
| **Never Auto** | `OG.md`, `MISSION.md`, `CHANGELOG.md`, `sessions/`, `local/` | Human or agent pulls explicitly |

**Rule:** Start every session reading Tier 1 only. Load Tier 2 files when the task requires them — name which files you loaded in your log entry.

**Code-linked loading:** files with `paths:` front matter (see *Code-Linked Front Matter*) are loaded by matching the folder you are editing against their `paths`, instead of guessing from filenames.

**Legacy projects:** with no `STATE.md`, Tier 1 is `INDEX.md`, `GUARDRAILS.md` and the **whole header block** of `CONTEXT.md` (Focus/Phase/Open/Next) plus its latest entries. Never load the tail of `CONTEXT.md` alone: the header is at the top and gets cut off as the log grows.

---

## Agent Session Workflow

A concrete sequence for any agent operating in a project with `lore`. Follow this every session, no exceptions.

### Starting a session
1. Read `lore/INDEX.md` → `lore/GUARDRAILS.md` → `lore/STATE.md` → latest `lore/log/` entries (Tier 1). Legacy project: `CONTEXT.md` instead of the last two.
2. Note the **Focus**, **Phase**, **Open**, and **Next** fields from STATE.md — this is your briefing
2a. Read `lore/local/` if it exists — this machine's accounts, paths and limits. It is never committed.
3. If picking up a task: run `lore session status` + `lore ticket list --status todo`
4. Load Tier 2 files only as the task requires — announce which ones you load

### During a session
- Load Tier 2 files as needed, name what you loaded
- Update ticket state as it changes — `lore ticket start <ID>`, `lore ticket done <ID>` — don't wait until the end
- Log significant decisions to `decisions/` as you make them, not in bulk at session end
- If you discover a gap in lore (missing feature doc, stale architecture), fix it as you go

### Ending a session
Do all of the following before closing:
1. **Rewrite `STATE.md`** — Focus, Phase, Open, Next must reflect current state (legacy: the `CONTEXT.md` header)
2. **Append a log entry** to this month's `log/YYYY-MM.md` — compact, 3-5 lines (see the `log/` contract; legacy: append to the `CONTEXT.md` log)
3. **Close tickets** — `lore ticket done <ID>` for each completed ticket; `lore ticket add` for new ones discovered
4. **Update feature files** for anything that started, changed, or completed. If you touched code under a lore file's `paths`, re-verify that file and bump its `verified_at`
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

**Always load:** `GUARDRAILS.md`, `STATE.md`, latest `log/` entries

## Tier 2 — Load When Relevant
| File / Dir | Load when |
|---|---|
| `architecture/` | Making structural or data model changes |
| `domain/` | Touching business rules or product terms |
| `references.md` | You need a spec that lives outside lore/ |
| `features/[name].md` | Working on that specific feature |
| `testing/registry.md` | Writing or reviewing tests |
| `decisions/` | Making or revisiting a significant decision |

## Human-Only (Never auto-load)
`OG.md` — raw dev journal  
`MISSION.md` — project soul  
`CHANGELOG.md` — git history (hook-generated)  
`sessions/` — raw session logs; `local/` — this machine only

## Proposed Additions
Agent-suggested lore expansions pending human review. If approved, they become canonical.
- [none yet]
```

---

### `GUARDRAILS.md`
**Audience:** Claude Code + developers — always loaded.
**Purpose:** Project-wide rules. What to always do, never do, and how conventions work here.
**Rule:** The most read file after STATE.md. Keep it honest and current. Split by domain if needed.
**Machine-specific facts do not belong here.** Accounts, local paths, resource limits and "switch to GitHub account X" go in `local/` (gitignored). A guardrail must be true on every clone; one that is only true on your laptop is wrong for everyone else.

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

### `STATE.md`
**Audience:** Claude Code — always loaded.
**Purpose:** Current state only. How any agent picks up where the last one left off without re-reading the codebase.
**Rule:** Rewritten every session, never appended to. Under 20 lines. History goes to `log/`, not here. Because it holds no log, it can never be cut off by a tail-based loader.

**Template:**
```markdown
# State

**Focus:** [what's actively being built — one line]
**Phase:** [Alpha / Beta / Prod / R&D]
**Open:** [open thread], [open thread]
**Next:** [next task], [next task]
```

*Legacy:* projects without `STATE.md` keep this header at the top of `CONTEXT.md`. See *Migrating a legacy lore*.

---

### `log/YYYY-MM.md`
**Audience:** Claude Code + humans — latest entries loaded every session; older months on demand.
**Purpose:** Chronological session log, one file per calendar month so no single file grows without bound.
**Rule:** Append-only. Never delete or rewrite old entries. Create the month's file on the first entry. Keep entries compact — 3-5 lines.

**Template:**
```markdown
# Log — YYYY-MM

### YYYY-MM-DD — [Dev Name]
[2-3 sentence summary of what was asked, what was done, and what the state is now]
Loaded: `architecture/data/models.md`, `features/auth/login.md`
Left open: [unresolved threads]
Carry forward: [what the ideation layer should be re-briefed on at the start of the next session]
```

**Multi-agent log format** (conductor sessions):
```markdown
### YYYY-MM-DD — [Conductor] / [sub-agent]
[2-3 sentence summary]
Loaded: `architecture/data/models.md`
Task: #[ID] — completed / in progress
Left open: [anything unfinished]
```

Format: `[Conductor] / [Sub-agent]` — replace with actual names (e.g. `Jerry / claude-code`, `Hermes / codex`). Makes it always clear who conducted and who executed.

**How to write a good log entry:**
- Summarize intent + outcome in 2-3 sentences. Not a transcript.
- List files loaded this session so the next agent knows what context was available.
- Flag anything left open so the next session starts where this one left off.

---

### `sessions/`
**Audience:** Humans, on demand. **Never auto-loaded.**
**Purpose:** Raw, auto-captured session logs (written by a hook, not by agents). Evidence, not memory.
**Rule:** Nothing here is a source of truth. Anything worth keeping gets distilled into `log/`, `STATE.md`, `features/` or `decisions/`.

---

### `local/`
**Audience:** This machine's agents and its developer. **Gitignored — never committed.**
**Purpose:** Facts true only here: GitHub/cloud accounts to use, absolute paths, simulator or device names, resource limits, personal tooling quirks.
**Rule:** Read at session start if present; never copy its content into tracked files. `lore create project` adds `lore/local/` to `.gitignore`; if it is missing, add it before writing anything here.

---

### `references.md`
**Audience:** Claude Code + humans.
**Purpose:** Pointers to documents that live outside `lore/` (specs, handoff docs, TODO files, tickets, wikis) so they are discoverable instead of shadow documentation.
**Rule:** Link, don't copy. One line each: path or URL, what it is, whether it is authoritative or superseded. When a referenced doc is superseded by a lore file, say so.

**Template:**
```markdown
# References

| Doc | What it is | Status |
|---|---|---|
| `docs/payments-spec.md` | Original payments spec | Superseded by `features/payments/checkout.md` |
```

---

### `domain/glossary.md`
**Audience:** Claude Code + humans.
**Purpose:** Product terms and what they mean *in this product*, so agents don't guess at overloaded words.
**Template:** `# Glossary` then `**Term** — definition. Where it appears: [code/area]. Not to be confused with: [x].`

---

### `domain/rules/<rule>.md`
**Audience:** Claude Code + humans — load when touching the behaviour the rule governs.
**Purpose:** One business rule per file: what must be true, regardless of how the code expresses it.
**Template:** `# Rule: [name]` · `**Statement:**` · `**Why:**` · `**Where enforced:**` (code/area, or "not enforced yet") · `**Exceptions:**`.

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

### `lore` CLI — Project + Ticket Management

A global Rust binary at `~/.local/bin/lore`. Zero dependencies. Auto-detects the active project by walking up directories until it finds `lore/config.yml`. Supports UUID prefix matching on all commands.

**Install:**
```bash
curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash
```

---

#### Project Commands

```sh
# Create a new project session
lore create project \
  --name "my-app" \
  --description "What this project does" \
  --wrk-dir "/path/to/project" \
  --shorthand MYA

# Planned (LOR-9, not built yet): pick a profile at init
lore create project --profile backend|frontend ...

# Create from a lore package zip (lore/ folder inside)
lore create project --unzip /path/to/archive.zip --name "..." --shorthand MYA

# List all project sessions
lore list projects

# Print project context (readable or --json)
lore recall <uuid-prefix>
lore recall 4cdf --json

# Register an existing project's lore/ folder as a session
lore session attach --wrk-dir <path>

# Edit project settings
lore edit project <uuid-prefix> [--name] [--description] [--shorthand] [--wrk-dir]

# Delete a session (keeps project files)
lore delete project <uuid-prefix>

# Self-update the CLI binary
lore update

# Print version
lore --version
lore version
```

Creates: `~/.lore/sessions/<uuid>.yml` + `lore/config.yml` + `lore/workspace/ticket.json`

---

#### Ticket Commands

```sh
# Add a ticket (auto-increments: MYA-1, MYA-2...)
lore ticket add --name "Build auth flow" --priority P1 --tags "backend,auth"
lore ticket add --name "Implement auth" --context "features/auth.md,architecture/models.md" --priority P1

# List tickets
lore ticket list
lore ticket list --status todo
lore ticket list --priority P1

# Show a ticket (by prefix or ID)
lore ticket show 4cdf MYA-1

# Edit a ticket
lore ticket edit MYA-1 --name "new name" --priority P0 --tags "backend,urgent"
lore ticket edit MYA-1 --context "features/updated.md,architecture/overview.md"

# Move through lifecycle
lore ticket schedule MYA-1             # backlog → todo
lore ticket start MYA-1 --agent agy   # todo → inprogress (with assignment tracking)
lore ticket done MYA-1                 # inprogress → done
```

Tickets are stored in `lore/workspace/ticket.json`. IDs are permanent — shorthand prefix + auto-incrementing number.

---

#### Session Commands

```sh
lore session status    # Shows ticket counts per state
lore session log <session> "message"   # Log a message to a ticket
lore session close     # Closes the active session
```

---

#### Pre-PR Gate

```sh
lore inspect <session> MYA-1    # Verifies context files exist, project builds, tests pass
```

Run `lore inspect` before marking any ticket done. This is the quality gate between building and merging.

---

#### Agent Usage Rules

- **Use `lore ticket <command>` exclusively** — never edit `workspace/ticket.json` directly
- **Use `lore ticket start/done` to move tickets** — preserves metadata and assignment tracking
- **Use `lore inspect <session> <ticket-id>`** as a pre-PR gate before marking done
- **Use `lore recall` at session start** to load project context if no STATE.md (legacy: CONTEXT.md) is available
- **Use `lore session status`** to get a quick state snapshot before planning work
- **Always confirm the active project first** (`lore list projects` or `lore recall <prefix>`) before adding tickets — the CLI auto-detects from cwd but verify it found the right project

---

## Code-Linked Front Matter

Every file under `features/`, `architecture/modules/`, `architecture/services/`, `apis/` and `contracts/` **starts with front matter** tying it to the code it describes:

```yaml
---
paths: [lib/presentation/home/stewardship/**]   # code this file describes (globs, repo-relative)
tests: [test/presentation/worth_it_*]           # tests that cover it (globs)
verified_at: <commit hash>                      # last commit at which this file was checked against the code
---
```

| Field | Rule |
|---|---|
| `paths` | Required. Globs relative to the repo root. A file with no `paths` is not code-linked and should not be in a code-linked folder. |
| `tests` | Optional but expected. Empty means "no tests", say so rather than omitting. |
| `verified_at` | Required. Full or short commit hash at which you last confirmed the file matches the code. Set it only after actually re-reading the code. Never bump it to "make it current". |

What it enables:
- **Loading:** an agent editing `lib/presentation/home/stewardship/x.dart` loads the lore files whose `paths` match, rather than guessing from filenames.
- **Staleness:** a file is suspect if code under its `paths` changed since `verified_at` (`git log <verified_at>..HEAD -- <paths>`). A code area matched by no file's `paths` is uncovered lore.
- **`lore doctor`** (not built yet, separate ticket) will automate both checks. Until then, agents do them by hand at session end.

---

## Architecture and Profile File Contracts

Applies to the folders in each profile. Each entry: **audience** · **purpose** · **template/skeleton**. All carry a rule: *stub honestly, never invent* (see *Init: New Project*). Files marked **[code-linked]** need the front matter above.

### Architecture

#### `architecture/overview.md` (both profiles)
**Audience:** Claude Code + humans — load when making structural changes.
**Purpose:** The shape of the system. Backend: services, data flow, infra topology, external dependencies. Frontend: layers, state management, dependency injection.
**Rule:** Updated when structure changes. Covers the *shape*, not every field.

#### Backend

| File | Audience · Purpose | Skeleton |
|---|---|---|
| `architecture/services/<name>.md` **[code-linked]** | Agents editing that service/app module · what it owns and how it behaves | `# Service: [name]` · Responsibility · Entry points · Depends on · Gotchas |
| `architecture/data/models.md` | Agents touching schema · fields, relationships, constraints, quirks (soft deletes, multi-tenancy, custom managers, naming) | `# Models` · per model: fields, relations, constraints, quirks |
| `architecture/data/migrations.md` | Anyone running or writing migrations · order, gotchas, **irreversible steps** | `# Migrations` · Order · Gotchas · Irreversible (flag loudly) |
| `architecture/jobs.md` | Agents touching async work · queues, scheduled tasks, workers | `# Jobs` · per job: trigger, schedule, retries, idempotent? |
| `architecture/security.md` | Anyone touching auth/permissions · auth model, permission rules, **where secrets live (names/locations only, never values)** | `# Security` · Auth · Permissions · Secrets (where, not what) |
| `apis/<surface>.md` **[code-linked]** | Agents and client devs · endpoints this project exposes per surface/version | `# API: [surface]` · Base URL · Auth · Endpoints · Errors · Versioning · Rate limits |
| `contracts/consumers/<client>.md` **[code-linked]** | Agents changing an API · who calls this backend and what they rely on | `# Consumer: [client]` · Endpoints used · Assumptions · Breaking-change contact |
| `contracts/upstream/<service>.md` **[code-linked]** | Agents touching integrations · third-party/internal services this backend calls | `# Upstream: [service]` · Base URL · Auth · Endpoints used · Limits · Failure behaviour |
| `ops/environments.md` | Deployers · hosts and config keys per environment (**names only, never values**) | `# Environments` · per env: hosts, config keys |
| `ops/deploy.md` | Deployers · how a deploy happens, order across services | `# Deploy` · Steps · Order · Rollback |
| `ops/runbooks/<task>.md` | On-call/anyone · step-by-step for a recurring task | `# Runbook: [task]` · When · Steps · Verify · If it fails |
| `ops/incidents/YYYY-MM-DD-<slug>.md` | Anyone · what broke, why, what changed | `# Incident: [slug]` · Impact · Timeline · Root cause · Fix · Follow-ups |
| `ops/observability.md` | On-call · where to look | `# Observability` · Dashboards · Alerts · Logs |

#### Frontend (mobile and web)

| File | Audience · Purpose | Skeleton |
|---|---|---|
| `architecture/modules/<area>.md` **[code-linked]** | Agents editing that area · one per code area, mirroring the source tree | `# Module: [area]` · Responsibility · Key classes · State · Depends on · Gotchas |
| `architecture/navigation.md` | Agents adding screens · routes, gates (auth/onboarding), deep links | `# Navigation` · Route table · Gates · Deep links |
| `architecture/state.md` | Agents touching data flow · where state lives, caching, persistence | `# State` · Per store: owner, lifetime, persistence |
| `architecture/environments.md` | Builders · hosts, env keys, build flavours (**names only**) | `# Environments` · Flavours · Keys · Hosts |
| `design/system.md` | Agents writing UI · colours, type, spacing, component rules | `# Design system` · Tokens · Components · Do/Don't |
| `design/copy.md` | Agents writing UI text · voice and wording rules | `# Copy` · Voice · Terms to use/avoid · Examples |
| `contracts/<service>.md` **[code-linked]** | Agents and backend devs · each API the app consumes | `# Contract: [service]` · **Status** (requested / agreed / deployed / verified) · Endpoints · Auth · Known gaps |
| `platform/ios.md`, `android.md`, `web.md` | Agents touching platform config · permissions, entitlements, store setup, gotchas | `# iOS` · Permissions · Entitlements · Store setup · Gotchas |
| `analytics.md` | Agents adding events · events, what they mean, where they go | `# Analytics` · per event: name, trigger, properties, destination |
| `ops/release.md` | Releasers · how a build is cut, version/build-number rules, release history | `# Release` · Steps · Versioning · History |
| `ops/runbooks/<task>.md` | Anyone · step-by-step for a recurring task | same as backend runbook |

---

### `features/<area>/<flow>.md`
**Audience:** Claude Code + humans — load when working on that flow.
**Purpose:** One file per committed or in-progress flow, grouped by product area. **Code-linked** (front matter required).

**Template:**
```markdown
---
paths: [lib/presentation/auth/**]
tests: [test/presentation/auth_*]
verified_at: <commit hash>
---
# Feature: [Name]

**Status:** Idea / In Progress / Done / Paused

## What It Does
[What problem it solves and how]

## Edge Cases
- [Known edge case]

## Assumptions
- [assumption] — validate by: [how or when this gets confirmed or invalidated]

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

### `decisions/NNNN-<slug>.md`
**Audience:** Claude Code + humans — load when making or revisiting a significant decision.
**Purpose:** Architecture Decision Records. Prevents re-litigating what's already been decided.
**Rule:** One file per decision. Filename is a zero-padded sequence number plus a short kebab-case slug (`0001-use-postgres.md`). Numbers are never reused; a superseded decision stays, marked `Superseded by NNNN`. Legacy projects with un-numbered slugs keep them; number new ones from the highest existing.

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

### `bullpen/` — Agent Roster

**Audience:** Conductor — load when building a delegation plan.
**Purpose:** The bullpen is the conductor's roster of available agents for this project. It tells the conductor who to use for what, and it's the mechanism by which agents receive their project-specific context when a task is delegated to them.
**Rule:** The conductor decides which agents get a folder and what files go in each one. `identity.md` is the only required file. Everything else is project-specific.

---

#### How the Bullpen Works

**Setting it up (conductor's job):**
1. When initializing a project, the conductor creates a folder in `bullpen/` for each agent that will work on the project
2. At minimum, each folder gets an `identity.md` defining the agent's role in this project
3. Add any other files the agent will need: skills, tools, custom instructions, prompt templates
4. The conductor updates bullpen files when agent roles change or new capabilities are added

**Using it during delegation (conductor's job):**
- Before assigning a task, read all files in the target agent's bullpen folder
- Include the full contents of those files in the delegation packet under `--- YOUR IDENTITY IN THIS PROJECT ---`
- This is how the agent knows who they are in this codebase — not from their own memory, from what you inject

**Receiving it (agent's job):**
- When you receive a delegation packet, your bullpen files are included at the top
- Read them before anything else — they define your role, your strengths, and your constraints for this project
- Your identity in one project may differ from another — always use the bullpen files you were given, not assumptions

---

#### `identity.md` — Required for Every Agent

The baseline file every agent folder must have. Scoped to this project specifically.

**Template:**
```markdown
# [Agent Name]

**Priority:** [#N — routing order when multiple agents are available; omit if solo]
**Role:** [what this agent does in this specific project]
**Strengths:** [what it excels at — be specific to this codebase and stack]
**Delegate when:** [types of tasks that should come to this agent]
**Avoid:** [what not to assign — where this agent underperforms]
**Invocation:** [how the conductor calls or invokes this agent]
```

---

#### Other Files (Project-Specific)

Beyond `identity.md`, add whatever the agent needs to operate well in this project.

| File | Use it when |
|---|---|
| `skills.md` | The agent has specific capabilities relevant to this stack |
| `tools.md` | The agent has access to specific tools in this project |
| `instructions.md` | The agent needs project-specific operating instructions |
| `prompts/` | Reusable prompt templates for common task types |

---

#### Example Bullpen Structure

```
lore/bullpen/
  conductor/
    identity.md          ← Conductor's role + how sub-agents should report back
  claude-code/
    identity.md          ← Role in this project
    skills.md            ← Stack conventions specific to this repo
  codex/
    identity.md
    instructions.md      ← How to format generated code for this codebase
  agy/
    identity.md
    tools.md             ← CLI tools and APIs available to agy here
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
    version: 3.0.0
    source: https://github.com/joabeliot/lore
    notes: using as-is

  - name: my-custom-skill
    source: custom
    notes: written for this project
```

---

## Session Update Rule

At the end of every session, Claude must:

1. **Rewrite `STATE.md`** — Focus, Phase, Open, Next reflect current state (legacy: the `CONTEXT.md` header)
2. **Append a log entry** to `log/YYYY-MM.md` — compact, 3-5 lines, what was done and what's open (legacy: the `CONTEXT.md` log)
3. **Update tickets** — `lore ticket done <ID>` for completed; `lore ticket add` for new ones
4. **Update feature files** if a feature was started, completed, or changed; re-verify any code-linked file whose `paths` you touched and bump its `verified_at`
5. **Log decisions** to `decisions/` if a significant architectural choice was made
6. **Update `testing/registry.md`** if tests were added or removed
7. **Commit** — both code and `lore/` changes committed together. They move as one.

**What Claude never touches:**

| File | Why |
|---|---|
| `OG.md` | Human-only. Always. |
| `MISSION.md` | Human-only. Always. |
| `CHANGELOG.md` | Hook-generated. Always. |
| `sessions/` | Hook-captured raw logs. Never edit, never auto-load. |

---

## Multi-Agent Protocol

> **If you are the conductor (orchestrator):** load the `larn` skill — it is your complete operating manual and supersedes this summary.
> **If you are a sub-agent receiving delegated work:** read this section to understand your role in the conductor model.

When a conductor (e.g. Hermes/Jerry) coordinates multiple sub-agents, `lore` becomes the shared state layer. This protocol keeps every agent synchronized and prevents conflicts.

### Session Types

| Type | Who | Protocol |
|---|---|---|
| **Solo** | Developer + one agent | Standard Agent Session Workflow above |
| **Conductor session** | Conductor + sub-agents | larn skill protocol |

### Conductor Startup (sub-agent's understanding)

When the conductor begins:
1. Reads Tier 1: `INDEX.md` → `GUARDRAILS.md` → `STATE.md` → latest `log/` entries (legacy: `CONTEXT.md`)
2. Runs `lore session status` + `lore ticket list --status todo`
3. Builds delegation plan — which tasks, which agents, what order
4. Assigns tasks via delegation packets
5. Monitors sub-agents; merges lore when they report back

### Delegation Packet

What the conductor sends to each sub-agent when delegating a task:

```
Task: #[ID] [description]

Context (paste STATE.md; legacy: CONTEXT.md header):
  Focus: ...
  Phase: ...
  Open: ...
  Next: ...

Guardrails: [paste GUARDRAILS.md or relevant sections]

Load these lore files: [list Tier 2 files relevant to this task]

Produce: [clear output spec — what files to write, what to build, what tests to write]

On completion you must:
  1. Run `lore ticket done [ID]` to close the ticket
  2. Append a log entry to lore/log/YYYY-MM.md using the multi-agent format
  3. Update any feature files, decisions, or test registry that changed
  4. Report back: task ID, outcome, files changed, what's left open
```

### Sub-Agent Completion Protocol

When a sub-agent finishes, it must do all of the following before reporting back:

1. Run `lore ticket done <ID>` to close the ticket
2. Append a log entry to this month's `log/` file using the multi-agent format
3. Update any `features/`, `decisions/`, or `testing/registry.md` that changed
4. Report back to the conductor:
   - Task ID and status (completed / partial / blocked)
   - Files changed
   - Anything left open or deferred
   - Any blockers that need the conductor's attention

### Concurrency Rules

These rules prevent lore conflicts when multiple agents are active:

- **One agent per task** — conductor assigns; sub-agents never self-assign
- **Sequential lore writes** — if two agents finish near-simultaneously, they queue writes; conductor merges if needed
- **No simultaneous file edits** — two agents must never write to the same file at the same time
- **`lore ticket done` is safe to run concurrently** — the CLI handles atomic writes to ticket.json
- **`STATE.md` is the conductor's** — sub-agents append log entries; only the conductor rewrites `STATE.md` at session end

### Conductor Loop

```
1. Read lore Tier 1 (INDEX, GUARDRAILS, STATE, latest log) + run `lore session status` + `lore ticket list --status todo`
2. Build task assignments based on todo tickets + current context
3. Send delegation packets to sub-agents (can run in parallel)
4. Receive completion reports from sub-agents
5. Merge any lore conflicts
6. Rewrite STATE.md with current state
7. Repeat or close session
```

### Ownership Table

| Responsibility | Conductor | Sub-Agent |
|---|---|---|
| `STATE.md` | Rewrites at session end | Appends log entries only |
| Ticket assignment | Assigns via `lore ticket start <ID> --agent [name]` | Never self-assigns |
| Ticket completion | Monitors overall state | Runs `lore ticket done <ID>` |
| `features/`, `decisions/`, `testing/` | — | Updates files relevant to their task |
| Lore conflict resolution | Merges conflicts | Reports conflicts upward |

---

## Hook Automation

`lore` uses git hooks to automate what agents shouldn't have to manually track.

### `post-commit` → `lore/CHANGELOG.md`

The `post-commit` hook appends every commit to `CHANGELOG.md` automatically. Install it once per project:

```bash
./install.sh --hooks /path/to/your/project
```

### Session-start hook (agent-side, per developer)

A `SessionStart` hook in the agent's own settings (e.g. `~/.claude/settings.json`) injects Tier 1 when a session begins. It must load, in order: `INDEX.md`, `GUARDRAILS.md`, `STATE.md`, then the most recent `log/` entries. For a legacy project with no `STATE.md`, load the full `CONTEXT.md` header block, never just the tail of the file. It must not load `sessions/`, `OG.md` or `MISSION.md`. Updating the hook is tracked in LOR-10; the hook is not shipped in this repo.

### What hooks do vs what agents do

| Responsibility | Hook | Agent |
|---|---|---|
| `CHANGELOG.md` | Auto-appends on every commit | Never touches |
| `STATE.md` + `log/` | — | Rewrites state + appends log |
| `workspace/ticket.json` | — | Updates via `lore ticket` CLI commands |
| `architecture/` | — | Updates when structure changes |
| `testing/registry.md` | — | Updates when tests change |
| `decisions/` | — | Creates new file per decision |

---

## Agent Creative Additions

Agents can propose new lore structure for a project — a new folder, a new file type, a new convention. If it's useful and the developer approves it, it gets added to the canonical spec.

**Rule:**
1. If you think a creative addition would help the project, **build it and use it**
2. Add an entry to `lore/INDEX.md` under `Proposed Additions` describing what you added and why
3. The human reviews and decides whether it becomes canonical
4. Never silently add to the canonical folder structure — only to `Proposed Additions`

---

## Init: New Project

When asked to init `lore` on a new project:

1. Run `lore create project --name "..." --description "..." --wrk-dir "." --shorthand ABC`
   Planned (LOR-9): add `--profile backend|frontend`. Until then, pick the profile with the developer in step 8 and create only that profile's folders by hand.
2. Create the shared core plus **only the chosen profile's** folders (see *Folder Structure*). Add `lore/local/` to `.gitignore`.
3. Stub every file with its template (code-linked files get front matter with empty `paths` and `verified_at: unverified`)
4. Fill `CLAUDE.md` with what's known: project name, stack, purpose, Session Rule, and lore Index
5. Leave `OG.md` blank with the prompt: *"What's on your mind about this project?"*
6. Leave `MISSION.md` blank with the prompt: *"What is this project and why should it exist?"*
7. Set `STATE.md` with placeholder values; create the current month's `log/YYYY-MM.md` with just its heading
8. Ask the developer to confirm: profile (backend / frontend), stack, key rules, and current focus before finalizing `CLAUDE.md`

**What NOT to invent:**
- Do not populate `architecture/data/models.md` with field names — stub only
- Do not populate `apis/`, `contracts/` or `ops/environments.md` with endpoints, hosts or keys — stub only
- Do not invent `domain/rules/`, `design/` tokens or `analytics.md` events — stub only
- Do not create files inside `features/`, `ideas/`, or `decisions/` — leave dirs empty
- Do not add `testing/registry.md` coverage rows — stub only
- Do not put machine-specific facts (accounts, paths) in `GUARDRAILS.md` — they go in `local/`
- Do not write a `verified_at` hash you did not verify

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
- Choose the profile from what the repo is (server/API code → backend; Flutter/React Native/web client → frontend) and say which you chose
- `lore/architecture/overview.md` from inferred system design
- Backend: `architecture/data/models.md` from model/schema files, `apis/<surface>.md` from route/serializer files. Frontend: `architecture/modules/<area>.md` per code area, `architecture/navigation.md` from the router
- Code-linked files get `paths` from the files you read and `verified_at` set to the current `HEAD` hash
- `lore/STATE.md` ready for first session, and the current month's `log/` file
- `lore/references.md` listing existing docs found in the repo (specs, TODOs, handoffs) with their status — link, don't copy
- `lore/GUARDRAILS.md` with reasonable defaults from what you found
- `lore/OG.md` and `lore/MISSION.md` left blank with human prompts

**Step 4 — Register the session**
Run `lore create project --name "..." --wrk-dir "." --shorthand ABC` to create the CLI session.

**Step 5 — Flag gaps**
Consolidate everything that couldn't be inferred into a numbered list. Never silently skip.

**Critical: never invent subdirectories** outside the canonical structure (shared core + the chosen profile). All inferred content goes into canonical files. A non-standard `lore/` layout breaks compatibility with every agent that reads it.

---

## Ideation-to-Code Bridge Workflow

The ideation layer (limn skill) is where ideas get shaped into Lore Packages. The execution layer (this skill + larn skill) is where they get built. The handoff between them is the Lore Package artifact.

```
1. Design in the ideation layer (limn skill for guided sessions)
2. Say "generate lore package" — the ideation agent outputs a structured handoff artifact
3. Hand the Lore Package to the conductor (multi-agent) or Claude Code directly (solo)
4. Agent reads lore, applies the package, picks up tickets via `lore ticket list`, and executes
5. After building, Claude Code updates STATE.md and the log and runs `lore ticket done <ID>` for completed work
6. Commit lore alongside code changes
```

**Rule:** `OG.md` and `MISSION.md` are always written by the human. Every other file can be AI-generated or AI-updated — but should be human-reviewed before committing.

---

## Migrating a legacy lore

For a project still on `CONTEXT.md` and `architecture/{models,apis}.md`. Content is preserved; nothing is deleted until the developer confirms. (A `lore migrate` command is planned in LOR-9; until then, do it by hand.)

1. **Split `CONTEXT.md`.** Header block (Focus/Phase/Open/Next) → `STATE.md`. Each `### YYYY-MM-DD` log entry → `log/YYYY-MM.md` by its date, in original order, text unchanged.
2. **Move architecture files.** Backend: `architecture/models.md` → `architecture/data/models.md`; `architecture/apis.md` → `apis/<surface>.md` (split per surface if it mixes several; external services go to `contracts/upstream/`). Frontend: `architecture/models.md` content goes to the relevant `architecture/modules/<area>.md` or `architecture/state.md`.
3. **Add front matter** to code-linked files. Set `paths`; set `verified_at` only for files you re-checked against the code, otherwise `unverified`.
4. **Move machine-specific guardrails** (accounts, local paths) to `local/` and add `lore/local/` to `.gitignore`.
5. **Update `INDEX.md`**, and add `references.md` for any docs living outside lore.
6. **Confirm with the developer**, then remove `CONTEXT.md` and the old paths. Commit the migration on its own.

---

## Keeping `lore` Healthy

- `STATE.md` is rewritten every session — stale focus is worse than no focus
- Log entries are appended every session — never skip it
- Code-linked files carry a `verified_at`; if code under a file's `paths` changed since, the file is suspect — re-verify or fix it
- Tickets reflect current reality — `lore ticket done` the moment work is complete
- Feature files get updated when features change — not just when they're created
- `testing/registry.md` grows with the test suite
- `decisions/` prevents re-litigating what's already settled
- Commit `lore/` alongside code — they should move together

---

## Evolving This Skill

This skill lives in `skills/lore/SKILL.md` in the lore repo, installed to each agent's skill directory.

When an agent proposes a creative addition that the developer approves, it gets added to this canonical SKILL.md and versioned. Projects then update by running the installer again.

The goal: any project with `lore/` is immediately legible to any agent, any developer, and any future team member — with zero onboarding friction.
