# Context

**Focus:** LOR-8 (8a) done pending JB review: SKILL.md + README now specify shared core + backend/frontend profiles. 8b = LOR-9 (CLI scaffold, migration), 8c = LOR-10 (hook).
**Phase:** R&D (v0.1.0, zero tests)
**Open:** Review LOR-8 diff; ADR 0001 review; LOR-7 audit triage (destructive bugs ahead of LOR-2?); branch/commit policy for uncommitted files
**Next:** Slice 0 (LOR-1) tests + F9/F4/F8; then LOR-9 once 8a is approved

---

## Log

### 2026-10-08 — Claude Code (lore inception)
Read the whole CLI (~2.7k lines), found 9 gate-integrity problems (F1–F9), wrote ADR 0001 with a four-layer test plan, and scaffolded this `lore/` (session `514ee393`, shorthand LOR). Decided: Claude Code conducts, agy builds, Codex reviews, Hermes is heartbeat only, `lore conductor` is the enforcement layer, no extra framework.
Loaded: `src/models.rs`, `src/main.rs`, `src/commands.rs`, `src/dashboard.rs` (routes)
Left open: ADR review; nothing committed; JB's uncommitted edits untouched

---

### 2026-10-09 — Claude Code (findings recorded)
Wrote down everything learned so far: `decisions/agent-orchestration-model.md` (hybrid model, Hermes/agy/Codex findings, classifier walls, unverified list), `features/conductor.md`, `features/takeme.md` (JB's finished-but-uncommitted feature, read-only reviewed: no overlap with slice 0), `ideas/doorstallor-pilot-findings.md` (verified vs agy-reported-only), `ideas/overnight-runs.md` (preconditions). Verified/unverified tags kept on every claim.
Loaded: `lore/decisions/conductor-and-gates.md`, uncommitted diff of `commands.rs`/`main.rs`/`install.sh`
Left open: same as header; doorstallor's own `lore/` doesn't have the pilot findings yet

---

### 2026-10-09 — Claude Code (LOR-8 / 8a)
Added LOR-7 (audit backlog) and LOR-8 (profiles), split LOR-8 into 8a/8b/8c (LOR-8/9/10). Implemented 8a: rewrote SKILL.md structure, tiers, session workflow, STATE/log/local/references/domain contracts, front-matter spec, backend+frontend file contracts, migration steps; README structure section updated. Docs mark profiles as target; CLI still scaffolds legacy.
Handoff: full state + gotchas in `decisions/project-profiles.md`. Open tickets point to it.
Loaded: `skills/lore/SKILL.md`, `README.md`, `src/commands.rs` (create_project)
Left open: JB review of 8a; LOR-8 still `inprogress`, not marked done; no code changed; nothing committed

---
