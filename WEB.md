# lore — Web Claude Workflow

## The Role of Web Claude

Web Claude is the **ideation and planning layer**. Hermes and sub-agents are the **execution layer**. lore is the bridge between them.

When you're on the go — thinking through architecture, planning features, designing systems — you do that in Claude Web. When you're ready to build, you ask Web Claude to generate a Lore Package. That package is what the conductor picks up and executes from.

```
Claude Web (you + Web Claude)
    ↓  discuss, ideate, design
    ↓  "generate lore instructions"
    ↓
Lore Package (structured output)
    ↓  paste into repo files
    ↓
Conductor (Hermes) reads it
    ↓  applies to lore, assigns to agents
    ↓
Sub-agents execute
    ↓  update lore on completion
    ↓
lore reflects reality
```

---

## Starting a Web Session

Paste this at the start of any Claude Web session (or set it as Project Instructions on claude.ai):

---

```
You are working with a project that uses lore — a structured project memory system committed to the repo.

Your role: IDEATION LAYER. You think, design, plan, and architect. You do not execute — an orchestrating agent (the conductor) picks up your output and delegates to sub-agents who build.

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

## Summary
[2-3 sentences: what was discussed, what was decided, what the conductor needs to know]

---

## Kanban — Add to Backlog
<!-- One ticket per task that came out of this session -->
- [ ] #[auto] [task description] `[source: Web, YYYY-MM-DD]`
- [ ] #[auto] [task description] `[source: Web, YYYY-MM-DD]`

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
Left open: [anything unresolved]
```

---

## Notes for Conductor
<!-- Anything the conductor specifically needs to know before delegating -->
<!-- Edge cases, dependencies between tasks, ordering constraints, risks -->
- [note]
````

---

## How the Conductor Consumes It

The conductor (Hermes) receives the Lore Package and processes it in this order:

1. Reads **Summary** — understands what came out of the web session
2. Adds **Kanban tickets** to `lore/kanban/backlog.md`
3. Writes **Feature files** to `lore/features/`
4. Writes **Decision files** to `lore/decisions/`
5. Applies **Architecture updates** to the relevant files
6. Applies **CONTEXT.md Update** — updates header block, appends log entry
7. Reviews **Notes for Conductor** — flags dependencies, risks, ordering
8. Builds delegation plan from the new backlog items

---

## Tips for Web Sessions

- Paste the relevant lore files at the start: CONTEXT.md, GUARDRAILS.md, the feature or architecture file you're working through
- The more context you give Web Claude, the better the Lore Package it generates
- If a decision is complex, ask Web Claude to draft the `decisions/` file content during the session — not just at the end
- You can do multiple "generate lore package" calls in one session as sub-topics close out
- Web Claude can't read your repo — you have to paste the content in
