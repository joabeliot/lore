---
name: lore
description: Web Claude workflow for lore. Use this skill when working in Claude Web on a project that uses lore. Covers how to run ideation sessions, generate Lore Packages, and hand off structured output to the conductor or solo agent for execution.
version: 1.3.0
---

# lore — Web Claude Workflow

## Web is the Structured Spark

Web Claude is the starting point. It's where thinking becomes structure — where a conversation about architecture, a product decision, or a feature idea gets shaped into something an agent can act on.

You don't build in Claude Web. You think, design, and decide. When you're ready, you say **"generate lore package"** — and Web Claude outputs a structured artifact that the conductor or solo agent picks up and executes from. That's the spark that kick-starts the whole process.

This works at any scale:
- **Solo** — Claude Code reads the Lore Package directly and runs it
- **Multi-agent** — the conductor consumes it, assigns tasks to sub-agents, they execute

The web session is always the entry point. The Lore Package is always the handoff.

```
Claude Web (you + Web Claude)
    ↓  think, design, decide
    ↓  "generate lore package"
    ↓
Lore Package — the structured spark
    ↓  paste into repo lore/
    ↓
Conductor or Solo Agent reads it
    ↓  applies to lore, picks up tasks
    ↓
Execution (solo or delegated)
    ↓  agents build, update lore, report back
    ↓
lore reflects reality
    ↓  next web session picks up from here
```

---

## Starting a Web Session

Paste this at the start of any Claude Web session (or set it as Project Instructions on claude.ai):

---

```
You are working with a project that uses lore — a structured project memory system committed to the repo.

Your role: IDEATION LAYER. You think, design, plan, and architect. You do not execute — a conductor or solo agent picks up your output and runs it.

How this session works:
1. I'll share context about the project (lore files, current state, what I'm thinking about)
2. We discuss, design, and make decisions together
3. When I say "generate lore instructions" or "generate lore package", you output a structured Lore Package
4. That package is what the conductor reads and acts on

When generating a Lore Package, follow the format exactly — the conductor parses it programmatically.
If I paste lore files (CONTEXT.md, GUARDRAILS.md, etc.), read them before responding.
Ask me clarifying questions. Don't assume. Surface edge cases and tradeoffs.
```

---

## The "Generate Lore Instructions" Command

When you say **"generate lore instructions"** or **"generate lore package"**, Web Claude outputs a Lore Package using the format below. This is the handoff artifact — the output of the web session that the conductor consumes.

---

## Lore Package Format

This is what Web Claude outputs. The conductor reads every section and applies it in order.

````markdown
# Lore Package — YYYY-MM-DD
**Session Type:** Ideation | Architecture | Feature Design | Debugging

## Summary
[2-3 sentences: what was discussed, what was decided, what the conductor needs to know]

---

## Kanban — Add to Backlog
<!-- One ticket per task that came out of this session -->
<!-- Conductor assigns real IDs (#001, #002...) when moving these to backlog -->
- [ ] #[TBD] [task description] `[source: Web, YYYY-MM-DD]`
- [ ] #[TBD] [task description] `[source: Web, YYYY-MM-DD]`

---

## Feature Files
<!-- Full content for each new or updated feature file -->
<!-- Omit this section if no feature files changed -->

### lore/features/[feature-name].md
```markdown
# Feature: [Name]

**Status:** Idea / In Progress / Done / Paused

## What It Does
[description]

## Edge Cases
- [edge case]

## Assumptions
- [assumption] — validate by: [how or when this gets confirmed or invalidated]

## Open Questions
- [question]

## Notes
[notes]
```

---

## Decision Files
<!-- Full content for each decision made this session -->
<!-- Omit this section if no decisions were made -->

### lore/decisions/[decision-slug].md
```markdown
# [Decision Title]

**Date:** YYYY-MM-DD
**Status:** Decided

## Decided
[what was chosen]

## Why
[reasoning]

## Rejected
[what lost and why]

## Consequences
[what this means going forward]
```

---

## Open Decisions
<!-- Forks that came up this session but weren't resolved -->
<!-- Conductor: treat each as a flagged blocker or research task -->
- [question] — options: [A vs B] — blocked on: [what's needed to decide]

---

## Architecture Updates
<!-- Only include sections that changed -->
<!-- Omit this section if no architecture changed -->

### lore/architecture/[file].md — [section name]
```markdown
[updated content for this section]
```

---

## CONTEXT.md Update

**Focus:** [updated focus — one line]
**Phase:** [Alpha / Beta / Prod / R&D]
**Open:** [open threads from this session]
**Next:** [what the conductor should prioritize]

Log entry to append:
```markdown
### YYYY-MM-DD — Web Session
[2-3 sentence summary of what was discussed and decided]
Loaded: N/A (web session)
Left open: [unresolved threads]
Carry forward: [what Web Claude should be re-briefed on at the start of the next session — paste this line to re-prime fast]
```

---

## Notes for Conductor
<!-- Anything the conductor specifically needs to know before delegating -->
<!-- Edge cases, dependencies between tasks, ordering constraints, risks -->
- [note]
````

---

## How the Conductor Consumes It

The conductor receives the Lore Package and processes it in this order:

1. Reads **Summary** — understands what came out of the web session
2. Adds **Kanban tickets** to `lore/kanban/backlog.md`
3. Reviews **Open Decisions** — creates a research or blocking ticket in backlog for each unresolved fork
4. Writes **Feature files** to `lore/features/`
5. Writes **Decision files** to `lore/decisions/`
6. Applies **Architecture updates** to the relevant files
7. Applies **CONTEXT.md Update** — updates header block, appends log entry
8. Reviews **Notes for Conductor** — flags dependencies, risks, ordering
9. Builds delegation plan from the new backlog items

---

## Tips for Web Sessions

- **Lore is the bible of this project.** Don't suggest anything that contradicts what's already decided there. If you think lore should change, flag it — don't silently contradict it.
- Paste the relevant lore files at the start: CONTEXT.md, GUARDRAILS.md, the feature or architecture file you're working through
- The more context you give Web Claude, the better the Lore Package it generates
- If a decision is complex, ask Web Claude to draft the `decisions/` file content during the session — not just at the end
- You can do multiple "generate lore package" calls in one session as sub-topics close out
- Web Claude can't read your repo — you have to paste the content in
