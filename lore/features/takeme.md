# Feature: lore takeme

**Status:** Done (working, **uncommitted** as of 2026-10-09; no tests)

## What It Does
`lore takeme [session]` prints a session's working directory; a shell wrapper function then `cd`s there. Matches UUID prefix first, then shorthand or project name (case-insensitive). With no argument it lists sessions (shorthand, project, 8-char prefix). Ambiguous matches list candidates.

## Where
`cmd_takeme` in `src/commands.rs`, `Takeme` variant in `src/main.rs`, `install_shell_function` in `install.sh` (appends a marked `lore()` function to `~/.zshrc`/`~/.bashrc`, idempotent via `# __lore_takeme__`).

## Edge Cases
- Prefix match wins over shorthand: a shorthand that is also a UUID prefix resolves as the UUID.
- Stale: the installed binary already includes it (built 2026-07-22).

## Open Questions
- Commit on its own on `enhancements` before the `conductor` branch (proposed to JB, awaiting OK).
- Add tests when the harness lands (LOR-1): prefix, shorthand, project name, ambiguity, none.

## Notes
Untracked `sessions/*.md` (Aug–Sep notes) are unrelated and unreferenced by code.
