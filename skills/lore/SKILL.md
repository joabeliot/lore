---
name: lore
description: "How the lore memory system works — CLI usage, ticket lifecycle, context files, and project structure"
version: 1.0.0
author: Joab Eliot
license: MIT
metadata:
  hermes:
    tags: [lore, project-memory, context, cli]
---

# Lore Skill — Using the Lore Memory System

This skill teaches you how to use the lore CLI and project memory system.

## Overview

Lore is a project memory system for AI-assisted development. It stores project context in a `lore/` folder and manages work via tickets.

## CLI Usage

All ticket operations go through the `lore` CLI. Never edit ticket.json manually.

### Sessions
```sh
lore create project --name "project" --description "..." --wrk-dir "/path" --shorthand ABC
lore recall <uuid|prefix>
lore list projects
```

### Tickets
```sh
lore ticket add --name "task" --priority P1 --tags "backend,auth" --context "features/auth.md"
lore ticket list [--status todo] [--priority P1]
lore ticket schedule <session> ABC-1
lore ticket start <session> ABC-1 --agent agy
lore ticket done <session> ABC-1
lore ticket edit <session> ABC-1 --name "new name"
lore ticket show <session> ABC-1
```

### Pre-PR Gate
```sh
lore inspect <session> ABC-1
```

## Key Rules

- Never edit ticket.json directly — use `lore ticket`
- Tickets reference context files in `features/`, `architecture/`, etc.
- Use `lore inspect` before marking a ticket done
- Auto-detection from cwd works if inside a project with `lore/config.yml`
