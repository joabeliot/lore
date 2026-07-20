# Gemini CLI

**Role:** Research, large context analysis, and cross-file search tasks. Useful when the task requires reasoning over a large number of files simultaneously.
**Strengths:** Large context window, codebase-wide analysis, summarizing broad system state, cross-file dependency tracing
**Delegate when:** Auditing the full codebase for a pattern, generating architecture docs from a large set of files, cross-service analysis
**Avoid:** Fine-grained editing of existing files — better to use analysis output as input for Claude Code to act on
**Invocation:** Via Gemini CLI. Receives a research question or analysis task. Best output is a structured report that Claude Code or the conductor can act on.
