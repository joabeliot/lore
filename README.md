<div align="center">
  <h1>🗂️ Lore</h1>
  <p><em>Keep your AI agents in the loop — project memory for the age of AI-assisted development.</em></p>
</div>

<p align="center">
  <a href="#quick-start">Quick Start</a> •
  <a href="#claude-code-setup">Claude Code Setup</a> •
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
# Install the CLI + wire into Claude Code automatically
curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash

# Confirm Claude Code is wired up
./install.sh --verify

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

## Claude Code Setup

Lore is designed to work hands-free with Claude Code. The installer auto-detects Claude Code and wires everything in — no manual config.

### What the installer does

1. **Installs the `lore` CLI** binary to `~/.local/bin/lore`
2. **Injects a `SessionStart` hook** into `~/.claude/settings.json` — Claude automatically reads `lore/GUARDRAILS.md` and the last 150 lines of `lore/CONTEXT.md` at the start of every session in any project that has a `lore/` folder
3. **Installs the lore skill** to `~/.claude/skills/lore/SKILL.md` — teaches Claude how to use the lore system

### Install

```bash
curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash
```

### Verify

After installing, confirm everything is wired correctly:

```bash
./install.sh --verify
```

Expected output:
```
[lore] ✓ SessionStart hook found
[lore]   matcher  : startup|resume|clear|compact
[lore]   GUARDRAILS.md loaded : yes
[lore]   CONTEXT.md loaded    : yes
[lore] ✓ Skill installed at ~/.claude/skills/lore/SKILL.md
```

If any line shows `✗`, the output tells you exactly which command to run to fix it.

### Re-install or repair

```bash
./install.sh --claude
```

This is idempotent — safe to run multiple times. It skips anything already in place and re-verifies at the end.

### How it works in practice

Once wired, Claude Code reads your project's lore at the start of every session automatically. You don't need to tell it to — it just knows the project rules, current state, and what's in progress before you type your first message.

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

Skills for Claude Code are installed automatically by the default installer. For other tools:

```bash
./install.sh --skill-dir ~/.hermes/skills/lore       # For Hermes
./install.sh --skill-dir ~/.claude/skills/lore        # For Claude Code (manual)
```

---

## Installation

### Via curl — recommended, does everything in one shot

```bash
curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash
```

Installs the CLI, wires the Claude Code hook, and copies the skill. Run `./install.sh --verify` after to confirm.

### Flags

| Flag | What it does |
|---|---|
| *(none)* | Install CLI + auto-wire Claude Code if detected |
| `--claude` | (Re-)install Claude Code hook and skill only |
| `--verify` | Check that the Claude Code hook is correctly installed — no writes |
| `--skill-dir <path>` | Install a skill to a custom directory |
| `--skill <name>` | Which skill to install: `lore` (default), `larn`, `limn`, `all` |
| `--hooks <path>` | Install git hooks into a project directory |

### From source

```bash
git clone https://github.com/joabeliot/lore.git
cd lore
./install.sh
```

Requires Rust if no pre-built binary is available for your platform. See [rustup.rs](https://rustup.rs).

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
