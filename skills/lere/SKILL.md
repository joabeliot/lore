---
name: lere
description: "How to orchestrate AI agents — planning, delegation, build loops, and the Narrator role"
version: 1.0.0
author: Joab Eliot
license: MIT
metadata:
  hermes:
    tags: [orchestration, delegation, narrator, build-loop]
---

# Lere Skill — Orchestrating AI Agents

**Lere** (from Old English *lǣran*, "to teach, guide") is the Narrator skill. It teaches orchestrator agents how to guide other agents through the build process.

## Overview

As the orchestrator (Narrator), you don't write code. You:
1. Plan the work
2. Delegate to the right agents
3. Verify the output
4. Close the loop

## Workflow

### 1. Plan
- Write feature docs in `lore/features/`
- Write architecture docs in `lore/architecture/`
- Create tickets with context references:
  ```sh
  lore ticket add --name "Build auth" --context "features/auth.md,architecture/models.md"
  ```

### 2. Delegate
```sh
lore ticket start <session> ABC-1 --agent agy
```

Give the agent the ticket ID and let it read the context files from lore.

### 3. Verify
```sh
lore inspect <session> ABC-1
```

### 4. Close
```sh
lore ticket done <session> ABC-1
lore session log <session> "Completed auth module - agy"
```

## Build Loop

```
1. Write lore docs (features, architecture)
2. Create ticket with --context
3. Delegate to agent
4. Agent builds from context files
5. Run inspect (pre-PR gate)
6. Mark done
7. Repeat
```
