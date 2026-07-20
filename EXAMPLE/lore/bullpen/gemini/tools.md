# Gemini — Available Tools

## CLI Access
- `gemini` CLI with `--all_files` flag for full codebase analysis
- Read access to the full project directory
- No write access — outputs analysis only; Claude Code applies changes

## What to Use Gemini For
- "Audit the entire codebase for X pattern" — large context tasks
- "Find all places that touch PaymentMethod and summarize" — cross-file tracing
- "Generate architecture/overview.md from the full repo" — doc generation from code
- "Find dead code or unused imports across all apps" — codebase hygiene analysis

## Output Format for Conductor
Always structure Gemini output as:
1. **Summary** — 3-5 sentences of what was found
2. **Findings** — bulleted list, specific and actionable
3. **Recommended next tasks** — what Claude Code should do with this analysis
