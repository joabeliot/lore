# Project profiles: shared core + backend/frontend structures

**Date:** 2026-10-09
**Status:** Decided (spec written in SKILL.md, awaiting JB review; CLI and hook not built)
**Tickets:** LOR-8 (8a, spec), LOR-9 (8b, CLI), LOR-10 (8c, hook). Related: LOR-7.

## Decided
Replace the single lore/ structure with a shared core plus two profiles (`backend`, `frontend`) chosen at init. A project gets only its profile's folders. Full trees and per-file contracts live in `skills/lore/SKILL.md` (sections *Folder Structure*, *Code-Linked Front Matter*, *Architecture and Profile File Contracts*, *Migrating a legacy lore*). Do not duplicate them here.

Key moves:
- `CONTEXT.md` splits into `STATE.md` (current Focus/Phase/Open/Next, rewritten each session) and `log/YYYY-MM.md` (append-only, one file per month).
- New shared folders: `local/` (gitignored, machine-specific), `sessions/` (raw, never auto-loaded), `domain/` (glossary, rules), `references.md` (pointers to docs outside lore).
- Files under `features/`, `architecture/modules|services/`, `apis/`, `contracts/` carry front matter: `paths`, `tests`, `verified_at`.
- Decisions are numbered `NNNN-<slug>.md` going forward.

## Why
Audit of a real 134k-line Flutter app: architecture/ assumed a backend; nothing linked lore to code so staleness was undetectable; the session hook loaded the last 150 lines of CONTEXT.md so the header got cut off; business rules, contracts, release knowledge and machine-specific notes had no home (a machine-specific "switch GitHub account" rule was committed as a guardrail and was wrong on a second machine).

## Rejected
- One structure for all projects: leaves frontends with empty backend stubs and nowhere for navigation/design/platform.
- Keeping header + log in one file: any tail-based loader eventually cuts the header.

## Consequences / state of the work
**Done (8a, LOR-8):** `skills/lore/SKILL.md` and `README.md` rewritten. Profiles are marked as the *target* structure; the legacy structure is documented as what the CLI still scaffolds. Legacy fallbacks (`CONTEXT.md` when no `STATE.md`) are stated in tiers, session workflow, multi-agent protocol, hook section.

**Not done:**
- LOR-9: `lore create project --profile`, scaffold only that profile with honest stubs, add `lore/local/` to the project's `.gitignore`, migration (CONTEXT.md -> STATE.md + log/, architecture/models.md + apis.md -> new locations, content preserved, idempotent). Entry point: `cmd_create_project` in `src/commands.rs` calls `lore_dir.create_structure()` (`src/models.rs`). Tests first, hermetic temp `HOME`. The skill mentions a `lore migrate` command as planned; decide the real command name in LOR-9 and fix the skill to match.
- LOR-10: session-start hook lives in JB's `~/.claude/settings.json`, not this repo. Needs JB present. Load INDEX, GUARDRAILS, STATE, latest log; fall back to full CONTEXT.md header for legacy projects; later match front-matter `paths` to the folder being edited.
- `lore doctor` (staleness), monorepo support: separate tickets, not created yet.

**Gotchas for the next agent**
- `SKILL.md` version is still 3.0.0; a bump to 3.1.0 is JB's call.
- `SKILL.md` still claims a `post-commit` hook auto-generates `CHANGELOG.md`; the hook file is not in the repo (LOR-7). Left untouched on purpose.
- Lore's own `lore/` is still on the legacy layout (CONTEXT.md). Migrating it is the first real test of the migration steps, so do it as part of LOR-9, not before.
- `git status` shows JB's uncommitted `install.sh`, `src/commands.rs`, `src/main.rs`, `sessions/`. Not touched, not reviewed. Do not commit them without JB.
- Never push, merge, release or run `lore update` without JB's OK (GUARDRAILS).
