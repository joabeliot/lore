# AGENTS.md — Lore AI Agent Guide

This file is the entry point for any AI agent that encounters the lore repository. If you're an AI agent reading this, here's what you need to know.

## What Is This?

This repo contains **Lore** — a project memory system for AI-assisted development. It includes:

1. **A CLI tool** (`lore`) — manages project sessions, tickets, and context
2. **AI skills** in `skills/` — instructions for agents on how to use lore

## Quick Start for Agents

### If someone asks you to install lore:
```bash
curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash
```

### If someone asks you to use lore on a project:
1. The project should have a `lore/` folder with `config.yml`
2. Run `lore recall` from inside the project to load context
3. Use `lore ticket list` to see what needs doing
4. Use `lore ticket start <id> --agent <your-name>` when you begin work
5. Use `lore inspect <session> <id>` before marking done (pre-PR gate)
6. Use `lore ticket done <id>` when you finish

### If you're being asked questions about this repo:
- The CLI source is in `src/` (Rust, compiled binary at install time)
- AI agent instructions live in `skills/` — load the relevant skill for your role
- `README.md` is the human-facing documentation

## Skills Index

| Skill | File | Who reads it | What it covers |
|---|---|---|---|
| **lore** | `skills/lore/SKILL.md` | Any agent working on a project with lore | Full system: file contracts, session workflow, tiered loading, CLI commands, init flows, bullpen |
| **lere** | `skills/lere/SKILL.md` | Conductor / narrator agents (Hermes, Jerry) | Orchestration: startup protocol, delegation packets, build loop, session close |
| **limn** | `skills/limn/SKILL.md` | Ideation agents (Claude Web, design sessions) | Generating Lore Packages from ideas and handing off to the conductor |

Load **lore** if you're a builder or solo agent.  
Load **lere** if you're orchestrating other agents.  
Load **limn** if you're in an ideation/design session generating structured output.

## Key Rules

- **Never edit `workspace/ticket.json` manually** — always use the `lore ticket` CLI
- **Tickets reference context files** — the `context` field points to `features/`, `architecture/`, etc.
- **Run `lore inspect` as a pre-PR gate** — it verifies context files exist and the project builds
- **The install script handles everything** — binary installation, skill installation, git hooks
- **Lore is the bible** — read it before every action, update it after every change
