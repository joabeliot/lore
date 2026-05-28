# lore

A structured folder you commit to your project that makes any codebase immediately
readable — by you, by teammates, and by AI agents like Claude Code.

No runtime. No dependencies. Just markdown files with a purpose.

---

## The Problem

You think through architecture in Claude Web. You build in Claude Code. Nothing carries over.
Every new session starts cold — you're re-explaining the project, re-establishing context,
re-answering questions the last session already answered.

`lore` fixes that. It's a shared source of truth that lives in the repo and travels with it.

---

## How It Works

Add a `lore/` folder to your project. Fill it in. Commit it with your code.

```
project/
  README.md              ← Human-facing overview (this file)
  CLAUDE.md              ← Claude Code entry point
  lore/
    OG.md                ← Your raw dev journal — human only, never AI
    context.md           ← Where the project stands right now
    architecture.md      ← How the system is designed
    models.md            ← Data models and schemas
    apis.md              ← API contracts and external services
    DECISIONS.md         ← Why things are the way they are
    features/            ← One file per feature (in-progress or committed)
    ideas/               ← Pre-feature captures — unvalidated, low friction
    skills/              ← Claude skills, custom and shared
    rules/               ← Guardrails for AI and developers
```

Every file has a defined audience, purpose, and update cadence. The full spec is in
[`SKILLS.md`](SKILLS.md).

---

## Install

Clone this repo, then run:

```bash
./install.sh
```

This copies `SKILLS.md` to `~/.claude/skills/lore/SKILL.md` — where Claude Code can find it.

To update after pulling new changes, run `./install.sh` again.

---

## Using the Skill

Once installed, tell Claude Code:

> "Init `lore` for this project"

or for an existing repo:

> "Read this repo and generate `lore` from what you find"

Claude will scaffold the folder, populate what it can infer, and tell you what still
needs your input.

---

## The Web-to-Code Workflow

```
1. Think through ideas in Claude Web
2. Ask Claude Web to generate a lore file from the discussion
3. Paste it into the right lore file in your repo
4. Claude Code picks it up on next session via CLAUDE.md
5. After a build session, ask Claude Code to update context.md
6. Commit lore alongside your code
```

---

## Full Spec

Everything — templates, init flows, file contracts — is in [`SKILLS.md`](SKILLS.md).
