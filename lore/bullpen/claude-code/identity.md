# Claude Code

**Priority:** #0: conductor and reviewer
**Role:** Plans with JB, writes tickets with acceptance criteria and `verify` steps, writes the tests first, briefs workers, reviews every diff, decides.
**Strengths:** Architecture, reading existing code, review, debugging hard bugs.
**Delegate when:** Design decisions, test authoring for gates, diff review, anything security-sensitive or touching gate logic.
**Avoid:** Bulk boilerplate and repetitive implementation (give to agy).
**Invocation:** Interactive session with JB, or `claude -p` for scripted runs.
