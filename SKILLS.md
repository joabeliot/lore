---
name: lore
description: Initialize, update, and maintain the lore project memory system. Use this skill whenever the user mentions init lore, set up project memory, generate lore from an existing repo, update CONTEXT.md, log a decision, add a feature file, or bridge ideas from Claude Web into Claude Code. Trigger even if the user doesn't say "lore" explicitly — if they're trying to capture project state, decisions, architecture, or current focus for AI context, this skill applies.
version: 1.0.0
---

# SKILL: lore — Project AI Readiness Layer

## What This Is

`lore` is a folder you commit to your project. It's the cassette an AI agent plugs into
to know the project — instead of reading the entire codebase every time, it reads `lore`
and gets the architecture, the decisions, the guardrails, and the full history of every
conversation that shaped the project.

Think of it as the project's memory. Not for the code — for the builder.

**Use this skill to:**
- Init `lore` on a new project from scratch
- Read an existing repo and generate `lore` from what's already there
- Update `lore` files during or after a dev session
- Bridge ideas from Claude Web into Claude Code context

---

## Folder Structure

```
project/
  README.md
  CLAUDE.md
  lore/
    OG.md
    MISSION.md
    CONTEXT.md
    ADR.md
    GUARDRAILS.md
    architecture.md
    architecture/
      models.md
      apis.md
    features/
    ideas/
    skills/
      custom/
      skills.yml
```

### Folder Structure Explanation

| File / Folder | Audience | Purpose |
|---|---|---|
| [`README.md`](#readmemd) | Humans | Project overview, setup instructions, links to CLAUDE.md and lore |
| [`CLAUDE.md`](#claudemd) | Claude Code | AI entry point — read every session; contains stack, rules, lore index, current focus |
| [`lore/OG.md`](#ogmd) | Developer only 🔒 | Raw, unstructured dev journal; never AI-generated; Claude reads for vibe and intent |
| [`lore/MISSION.md`](#missionmd) | Developer only 🔒 | The project's soul — what it is, who it's for, why it should exist; never AI-generated |
| [`lore/CONTEXT.md`](#contextmd) | Claude Code | Session log — every AI conversation recorded: who asked, what was proposed, what was done |
| [`lore/ADR.md`](#adrmd) | Claude Code + humans | Architecture Decision Records — what was decided, why, and what was rejected |
| [`lore/GUARDRAILS.md`](#guardrailsmd) | Claude Code + developers | Project-specific dos and don'ts — always/never/conventions per domain |
| [`lore/architecture.md`](#architecturemd) | Claude Code + humans | System design, infra topology, service map, data flow |
| [`lore/architecture/models.md`](#architecturemodelsmd) | Claude Code + humans | Data models, schemas, relationships, quirks |
| [`lore/architecture/apis.md`](#architectureapismd) | Claude Code + humans | API contracts, endpoints, auth, rate limits, external services |
| [`lore/features/[name].md`](#featuresfeature-namemd) | Claude Code + humans | One file per committed or in-progress feature |
| [`lore/ideas/[name].md`](#ideasidea-namemd) | Developer + Claude Code | Unvalidated captures — low friction, no strict format |
| `lore/skills/custom/` | Claude Code | Project-specific skills written for this repo |
| `lore/skills/skills.yml` | Claude Code | Registry of all forked/adapted skills in use — like requirements.txt |

> 🔒 = Human-only. Claude never writes to these files.

---

## File Contracts

Each file has a defined audience, purpose, and update rhythm.

---

### `README.md`
**Audience:** Humans — developers, contributors, stakeholders.  
**Rule:** No AI-specific content. Keep it clean.  
**Must include:** Project purpose, setup instructions, link to `CLAUDE.md` and `lore/`.

---

### `CLAUDE.md`
**Audience:** Claude Code — read automatically at the start of every session.  
**Rule:** Keep it dense. This is the first thing Claude reads, every time.  
**Must include:** Project name and purpose, stack, key rules, lore index, current focus, and the Session Rule.

**Template:**
```markdown
# [Project Name]

## What This Is
[2-3 sentences: what the project does, the stack, and who it's for]
See `lore/MISSION.md` for the full project mission.

## Rules
- [Key constraint 1]
- [Key constraint 2]
- See `lore/GUARDRAILS.md` for full guardrails

## Stack
- Backend: [e.g. Django / PostgreSQL]
- Frontend: [e.g. Next.js / Flutter]
- Infra: [e.g. Docker / AWS / Cloudflare]

## lore Index
- `MISSION.md` — full project mission and purpose (read when context matters)
- `CONTEXT.md` — session log of all AI conversations in this project
- `ADR.md` — architecture decision records
- `GUARDRAILS.md` — full guardrails
- `architecture.md` — system design and infra map
- `architecture/models.md` — data models and schemas
- `architecture/apis.md` — API contracts and external services
- `features/` — active and completed features
- `ideas/` — unvalidated ideas

## Current Focus
[What is actively being worked on right now]

## Session Rule
This project uses `lore` for AI memory. At the end of every session:
- Log the session to `lore/CONTEXT.md` using the session entry format
```

---

### `OG.md` 🔒
**Audience:** You — the developer.  
**Rule:** Never AI-generated. Never structured. Claude never writes to this file.  
**Purpose:** Your unfiltered thoughts on the project. Doubts, instincts, intent. Claude reads
this for context and vibe, not instructions.

**Prompt to get started:** *"What's going on in my head about this project right now?"*

---

### `MISSION.md` 🔒
**Audience:** You — the developer. Claude reads it, never writes it.  
**Rule:** Never AI-generated. This is the human's document.  
**Purpose:** The soul of the project. Not operational detail — the *why*. Claude reads this
when it needs to make decisions that require understanding what the project is trying to be,
not just what it's currently doing.

**Must include:** What is this? Who is it for? What problem does it solve? Why should it exist?
What does success look like?

**Prompt to write it:** *"If I had to explain this project to someone who had never heard of it,
and I wanted them to understand not just what it does but why it matters — what would I say?"*

---

### `CONTEXT.md`
**Audience:** Claude Code — this is the AI's memory of every conversation in the project.  
**Rule:** Appended by Claude at the end of every session. Never delete entries. Never rewrite history.  
**Purpose:** A chronological log of every AI session — who asked, what was proposed, what was
actually done. This is how any AI agent (or a new developer) traces back the full history of
how the project was built, decision by decision, session by session.

When a new AI session starts, it reads CONTEXT.md to understand what's happened before —
no need to re-read the entire codebase.

**Template:**
```markdown
# Context

Chronological log of all AI-assisted sessions in this project.

---

### YYYY-MM-DD HH:MM — [Dev Name]

**Asked:** [What the developer requested or the problem they described]

**Proposed:** [What the AI suggested or planned]

**Acted On:**
- [What was actually implemented or changed]
- [Files touched, commands run, decisions made]

**Outcome:** [Result — what worked, what didn't, what's left open]

---
```

**How to populate each session entry:**
- **Date/Time:** Use the current date and time at session end
- **Dev Name:** Pull from `git config user.name`
- **Asked:** Summarize what the developer asked for — capture the intent, not a transcript
- **Proposed:** What the AI suggested before acting — the plan, the approach
- **Acted On:** What was actually done — files created, code written, configs changed
- **Outcome:** What's the state now? Did it work? Anything left unfinished?

---

### `ADR.md`
**Audience:** Claude Code + humans.  
**Purpose:** Architecture Decision Records. Prevents Claude from re-suggesting things already considered.  
**Rule:** Every meaningful decision gets an entry — what was chosen, why, and what lost.

**Entry format:**
```markdown
## [Decision Title] — [YYYY-MM-DD]
**Decided:** [What was chosen]
**Why:** [The reasoning]
**Rejected:** [What else was considered and why it lost]
```

---

### `GUARDRAILS.md`
**Audience:** Claude Code + developers.  
**Purpose:** Project-wide rules — what to always do, never do, and how conventions work.  
**Format:** Single file. Split by domain with headers if the project needs it.

**Template:**
```markdown
# Guardrails

## Always
- [Pattern to always follow]

## Never
- [Pattern to never use in this project]

## Conventions
- [Project-specific naming, structure, or style decisions]

## Backend
- [Backend-specific rules if needed]

## Frontend
- [Frontend-specific rules if needed]
```

---

### `architecture.md`
**Audience:** Claude Code + humans.  
**Purpose:** System design, infra topology, how services connect.  
**Include:** Service map, data flow, deployment setup, external dependencies, anything
non-obvious about how the system is structured.

---

### `architecture/models.md`
**Audience:** Claude Code + humans.  
**Purpose:** Data models and schemas.  
**Include:** Field names, types, relationships, constraints, quirks (e.g. soft deletes,
multi-tenancy patterns, custom managers, naming conventions).

---

### `architecture/apis.md`
**Audience:** Claude Code + humans.  
**Purpose:** API contracts — internal endpoints and external services.  
**Include:** Base URLs, auth method, key endpoints, known gotchas, rate limits, versioning.

---

### `features/[feature-name].md`
**Audience:** Claude Code + humans.  
**Purpose:** One file per committed or in-progress feature.

**Template:**
```markdown
# Feature: [Name]

## Status
[ ] Idea  [ ] In Progress  [ ] Done  [ ] Paused

## What It Does
[Plain description — what problem it solves and how]

## Edge Cases
- [Known edge case]

## Open Questions
- [Unresolved]

## Notes
[Anything else relevant]
```

---

### `ideas/[idea-name].md`
**Audience:** You — and eventually Claude Code.  
**Purpose:** Pre-feature. Unvalidated. Low-friction capture.  
**Rule:** No strict format. Write enough to remember the idea and the instinct behind it.
Promote to `features/` when it's committed.

---

### `skills/custom/`
Project-specific Claude skills. Same SKILL.md format.  
Use for patterns unique to this repo: how views are written, how errors are handled,
how migrations work, how tests are structured.

---

### `skills/skills.yml`
Registry of all forked or adapted skills in use — like `requirements.txt` but for skills.  
Each entry should list the skill name, source, and any notes about what was customized.

**Format:**
```yaml
skills:
  - name: lore
    source: https://github.com/anthropics/skills/lore
    notes: using as-is

  - name: my-custom-skill
    source: custom
    notes: written for this project
```

---

## Session Update Rule

At the end of every session, Claude appends a session entry to `CONTEXT.md`. This is
non-negotiable — it's what makes `lore` a living memory instead of a one-time setup.

**What Claude logs after every session:**

| Field | Source | Description |
|---|---|---|
| Date/Time | System clock | When the session ended |
| Dev Name | `git config user.name` | Who was driving |
| Asked | Conversation | What the developer requested — intent, not transcript |
| Proposed | Conversation | What the AI suggested before acting |
| Acted On | Conversation + file changes | What was actually done — files, code, decisions |
| Outcome | End state | What worked, what didn't, what's left open |

**What Claude never touches:**

| File | Why |
|---|---|
| `OG.md` | Human-only. Always. |
| `MISSION.md` | Human-only. Always. |

---

## Init: New Project

When asked to init `lore` on a new project:

1. Create the full folder structure
2. Stub every file with its template
3. Fill `CLAUDE.md` with what's known: project name, stack, purpose — **including the Session Rule**
4. Leave `OG.md` blank with the prompt: *"What's on your mind about this project?"*
5. Leave `MISSION.md` blank with the prompt: *"What is this project and why should it exist?"*
6. Set `CONTEXT.md` header to: `# Context` with the template ready for the first session entry
7. Ask the developer to confirm: stack, key rules, and current focus before finalizing `CLAUDE.md`

**What NOT to invent for a new project:**
- Do not populate `architecture/models.md` with field names or schemas — stub only
- Do not populate `architecture/apis.md` with endpoint tables — stub only
- Do not create files inside `features/` or `ideas/` — leave the directories empty
- Do not add `ADR.md` entries unless a decision was explicitly stated

The reason: inventing project content contaminates `lore` with hallucinated facts that look real.
It's better to leave a field blank than to fill it with a confident guess.

---

## Init: Existing Repo

When pointed at a repo that has no `lore`:

**Step 1 — Check for CLAUDE.md**
- If `CLAUDE.md` exists: inject the `lore` block into it (Session Rule + lore Index). Do not overwrite the rest.
- If `CLAUDE.md` does not exist: create it from the template.

**Step 2 — Read the repo**  
Scan `README.md`, `CLAUDE.md` (if any), package files (`package.json`, `requirements.txt`,
`pubspec.yaml`, `Dockerfile`, etc.), and folder structure.

**Step 3 — Generate `lore/`** using canonical paths only:
- `lore/architecture.md` from inferred system design
- `lore/architecture/models.md` from model and schema files found
- `lore/architecture/apis.md` from route files, serializers, or API configs found
- `lore/CONTEXT.md` with header ready for first session entry
- `lore/OG.md` left blank with the human prompt
- `lore/MISSION.md` left blank with the human prompt

**Step 4 — Flag gaps**  
Consolidate everything that couldn't be inferred into a numbered list. Don't silently skip.

**Critical: never invent custom subdirectories.** The `lore/` folder structure is fixed — do not
create subdirectories like `lore/apps/`, `lore/config/`, or `lore/infra/` to mirror the repo's
own structure. All inferred content goes into the canonical files. A non-standard `lore/` layout
breaks compatibility with every other Claude instance that reads it.

---

## Web-to-Code Bridge Workflow

The gap between Claude Web (ideation) and Claude Code (execution) is context.
This workflow keeps them in sync.

```
1. Think through ideas, architecture, or features in Claude Web
2. Ask Claude Web to generate or update a lore file from the discussion
3. Paste the output into the correct lore file in your repo
4. Claude Code picks it up next session via CLAUDE.md
5. After building, Claude Code logs the session to CONTEXT.md
6. Commit lore alongside code changes
```

**Rule:** `OG.md` and `MISSION.md` are always written by the human. Every other file can be
AI-generated or AI-updated — but should be human-reviewed before committing.

---

## Keeping `lore` Healthy

- `CONTEXT.md` is appended by Claude every session — don't let it get skipped
- Add an `ADR.md` entry whenever something significant is decided or rejected
- Promote ideas from `ideas/` to `features/` when they become committed work
- Keep `CLAUDE.md` current — stale focus is worse than no focus

---

## Evolving This Skill

This skill lives at `~/.claude/skills/lore/SKILL.md` globally and optionally at
`lore/skills/custom/lore.md` per project.

Treat it like code — version it, improve it, fork it per project if the project needs
a different flavor. When the system evolves (new file types, new workflows, new patterns),
update this file to reflect it.

The goal: any project with `lore/` is immediately legible to any Claude instance,
any developer, and any future agent — with zero onboarding friction.
