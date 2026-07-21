<div align="center">
  <h1>🗂️ Lore</h1>
  <p><em>Keep your AI agents in the loop — project memory for the age of AI-assisted development.</em></p>
</div>

<p align="center">
  <a href="#quick-start">Quick Start</a> •
  <a href="#how-it-works">How It Works</a> •
  <a href="#cli-commands">CLI Commands</a> •
  <a href="#skills">Skills</a> •
  <a href="#installation">Installation</a>
</p>

---

**Lore** is a project memory system for AI-assisted development. It gives your AI agents (Claude, Gemini, Codex, Hermes) the context they need to work effectively on your projects — without you having to re-explain everything every session.

It's two things in one:

1. **A CLI tool** (`lore`) — manage project sessions, tickets, and context
2. **A set of AI skills** — teach your agents how to use lore effectively

---

## Quick Start

```bash
# Install the CLI
curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash

# Create a project
lore create project \
  --name "my-app" \
  --description "What my app does" \
  --wrk-dir "/path/to/project" \
  --shorthand MYA

# Add tickets
lore ticket add --name "Build auth" --priority P1 --context "features/auth.md"

# Work with agents
lore ticket start MYA-1 --agent claude
# ... agent builds the feature ...
lore inspect MYA-1   # Pre-PR gate
lore ticket done MYA-1
```

---

## How It Works

Lore stores project context in two places:

| Where | What | Purpose |
|---|---|---|
| `~/.lore/sessions/` | Global session registry (YAML) | One file per project — links the CLI to the project's lore folder |
| `lore/` in your project | Project lore folder | Markdown files: features, architecture, decisions, testing, guardrails |

The `lore` CLI bridges these two — it reads your session to know which project you're working on, then reads/writes tickets and context from the project's lore folder.

**The key insight:** Tickets reference context files in your lore folder. When you delegate to an AI agent, lore hands over the ticket + the relevant feature docs + architecture docs + decisions. The agent builds from your lore, not from guesswork.

---

## CLI Commands

```
lore create project     Create a new project session
lore recall             Print project context (human or --json)
lore list projects      List all registered sessions
lore delete project     Remove a session
lore ticket add         Add a ticket (auto-incrementing ID)
lore ticket list        List tickets (filter by status/priority)
lore ticket show        Show ticket details
lore ticket schedule    backlog → todo
lore ticket start       todo → inprogress (assign an agent)
lore ticket done        inprogress → done
lore ticket edit        Edit ticket fields (context, priority, etc.)
lore session attach     Register an existing project's lore folder
lore session close      Close a session
lore session status     Show ticket counts per state
lore session log        Log a message to a ticket
lore inspect            Pre-PR gate — verify context, build, tests
lore edit project       Edit project settings
lore update             Self-update
```

---

## Skills

Lore ships with AI agent skills that teach your tools how to use the system:

| Skill | What it teaches |
|---|---|
| **lore** | How the lore memory system works — CLI usage, ticket lifecycle, context files |
| **larn** | How to orchestrate agents — planning, delegation, build loops, inspect |
| **limn** | How to ideate and package ideas into lore-ready form |

Install skills with:
```bash
./install.sh --skill-dir ~/.hermes/skills/lore       # For Hermes
./install.sh --skill-dir ~/.claude/skills/lore        # For Claude Code
```

---

## Installation

### Via curl (recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash
```

### Via install.sh
```bash
git clone https://github.com/joabeliot/lore.git
cd lore
./install.sh [--skill-dir <path>] [--hooks <project-path>]
```

### From source
```bash
# Requires Rust
cargo build --release
cp target/release/lore ~/.local/bin/lore
```

---

## Project Structure

```
project/
├── lore/
│   ├── config.yml              ← Project config (references session)
│   ├── workspace/
│   │   └── ticket.json         ← All tickets as structured JSON
│   ├── features/               ← Feature descriptions
│   ├── architecture/           ← System design docs
│   ├── decisions/              ← Architecture Decision Records
│   ├── testing/                ← Test coverage registry
│   ├── INDEX.md                ← TOC for AI agents
│   ├── GUARDRAILS.md           ← Project rules
│   └── CONTEXT.md              ← Current state + session log
└── CLAUDE.md or AGENTS.md      ← AI entry point
```

---

## License

MIT
