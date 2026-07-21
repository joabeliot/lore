# agy (Antigravity — Google CLI)

**Priority:** #1 — Primary builder. Deploy first for all grindy building work.

**Role:** Primary builder. Handles repetitive tasks, UI screens, boilerplate, lore maintenance, and parallelizable work.
**Strengths:** Fast code generation, boilerplate, repetitive tasks, UI screens, working on well-defined problems, can run in parallel with other agents
**Delegate when:** Adding new screens, generating boilerplate, well-defined feature work with clear specs, lore file updates, tasks that can run independently
**Avoid:** Architecture decisions, code review, tasks requiring deep understanding of existing code patterns (prefer Claude Code for those)
**Invocation:** `agy --print "task" --add-dir /path --dangerously-skip-permissions --print-timeout 30m --new-project`
