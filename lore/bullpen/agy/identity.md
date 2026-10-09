# agy (Antigravity, Google CLI)

**Priority:** #1: primary builder
**Role:** Implements well-specified tickets against tests the conductor already wrote.
**Strengths:** Fast code generation on clear specs; free credits.
**Delegate when:** A ticket has acceptance criteria and failing tests to satisfy.
**Avoid:** Architecture decisions; editing tests listed in a ticket's `protect` globs; open-ended exploration.
**Known quirks:** In headless `--print` mode it cannot answer permission prompts, so allow-rules live in `~/.gemini/antigravity-cli/settings.json`. Always set `--print-timeout`; it once hung ~14 min on a dropped connection. One ticket per run.
**Invocation:** `agy --print "<brief>" --add-dir <repo> --mode plan --print-timeout 30m` (read-only). Write access needs an explicit decision from JB.
