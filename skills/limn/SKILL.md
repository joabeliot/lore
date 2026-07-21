---
name: limn
description: "How to ideate, design, and package ideas into lore-ready form"
version: 1.0.0
author: Joab Eliot
license: MIT
metadata:
  hermes:
    tags: [ideation, design, planning, lore-package]
---

# Limn Skill — Ideation and Design

**Limn** (from Old English *limnen*, "to illuminate, to depict vividly") is the ideation skill. It teaches how to take raw ideas and shape them into lore-ready packages.

## Overview

Ideas start in your head (or in conversation). Limn helps you transform them into structured lore content that the build agents can work from.

## What to Produce

When ideating, produce lore-ready content:

### Feature Files
Write clear feature descriptions in `lore/features/<name>.md`:
```markdown
# Feature: Auth

## What It Does
JWT-based authentication with refresh tokens.

## Edge Cases
- Token expiry during long sessions
- Concurrent login from multiple devices

## Open Questions
- Should we support OAuth providers?
```

### Decision Records
Capture architectural decisions in `lore/decisions/<slug>.md`:
```markdown
# Decision: Use JWT over Session Tokens

**Decided:** JWT with refresh tokens
**Why:** Stateless, scales horizontally without shared session store
**Rejected:** Redis sessions (added infrastructure complexity)
```

### Ticket Creation
Structure tickets that reference your ideation output:
```sh
lore ticket add --name "Implement auth" \
  --context "features/auth.md,architecture/models.md,decisions/jwt-over-sessions.md" \
  --priority P1
```

## Lore Packages

A lore package is a zip file containing a complete `lore/` folder structure with:
- Feature files
- Architecture docs
- Decision records
- Ticket definitions

Use `lore create project --unzip <package.zip>` to load it into a project.
