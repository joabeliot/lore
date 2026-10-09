# Agent Orchestration Model

**Date:** 2026-10-09
**Status:** Decided (JB confirmed direction; revisit after the first supervised runs)

## Decided
Hybrid, no extra framework:
- **Claude Code = conductor.** Plans with JB, writes tickets with acceptance criteria and `verify` steps, writes the tests first, briefs workers, reviews every diff, decides.
- **agy = builder.** **Codex = independent QA/reviewer** (never reviews its own work).
- **Hermes = heartbeat only:** notifications to JB, cron-triggered launches, memory. Not in the build loop.
- **`lore` itself is the control plane:** tickets, inspect gate, events, resume, `lore conductor`, dashboard. Source of truth for tasks is `lore ticket`, not a second board.

## Why
- Enforcement must live in code, not prompts. A SKILL.md is advice; a CLI that refuses invalid transitions is a guarantee.
- Free credits are on agy and Codex; Claude is spent only on planning and review.
- Fewer hops = fewer failure points and costs. Hermes is itself an LLM agent; putting it between conductor and workers adds a model hop. Its OpenRouter runway was ~$1.72 of $5 per Hermes's own report (unverified by us).
- JB's goal: give a task, work out requirements together, tickets land in `lore/`, the system runs them, JB gets the product, with nothing overlooked.

## Rejected
- **Hermes as middle manager / Hermes kanban as the board** (JB's pasted plan had Claude below Hermes): duplicates `lore ticket`, adds a model hop and cost. JB later agreed to `lore conductor` as the control plane.
- **Third-party frameworks (LangGraph, CrewAI, etc.):** they wire models together; our workers are already CLIs.
- **Separate `conductor` binary:** duplicates the session/ticket model.

## Findings that shaped this (verified in-session unless marked)
- **Hermes** v0.14.0 installed, `doctor` clean. Programmatic call that works: `hermes chat -Q -q "<prompt>" --max-turns 1`. `hermes -z "<prompt>"` exits 0 with **empty output** (my misdiagnosis: it looked "broken" when only that entry point was wrong). Has `kanban` (swarm, decompose, dispatch, daemon), `cron`, `send`, `--worktree`, `--max-turns`. Nous auxiliary provider logs a payment/credit error; main provider (OpenRouter `pareto-code`) works. Discord gateway logged DNS errors when offline.
- **agy headless** (`--print`) cannot answer permission prompts: it auto-denies and exits with "no output produced". Fix = allow-rules in `~/.gemini/antigravity-cli/settings.json` (`command(<cmd>)`). Read-only set added: cat, ls, grep, head, tail, wc, git status/log/diff/show (plus pre-existing git add). `find` deliberately excluded (`-delete`/`-exec`).
- **agy brief matters:** with the allow-list but an unconstrained brief, the run still produced nothing (some uncovered command, not identified; likely `find`/pipe). A brief restricting it to the allowed single commands succeeded. After a dropped connection it sat ~14 min with no model calls before finishing; always set `--print-timeout` and watch the log (`--log-file`).
- **agy output reliability:** its survey was mostly right but wrong on at least one claim (called two non-empty dirs empty). Every agy claim gets verified before it is relayed.
- **Claude Code auto-mode classifier** blocked: running agy with `--dangerously-skip-permissions`, and editing agy's settings.json (as "creating unsafe agents"/self-modification), even after JB said yes in chat. It worked in manual mode, where JB approves each call. Plan: keep worker permissions narrow; no blanket skip flag; unattended write access needs an explicit per-task decision.
- **Unverified:** whether using a Claude subscription through a third-party harness is allowed (JB should check Anthropic's current terms; API key is the safe route). Whether Hermes has spend caps. Whether `main` on GitHub is branch-protected. Codex's permission model for unattended runs.

## Consequences
- `lore` grows a `conductor` subcommand; see `decisions/conductor-and-gates.md` (ADR 0001) and `features/conductor.md`.
- Overnight/unattended runs wait for: the test gate, credential/staging isolation, a worker-stuck rule, and a launcher script (see `ideas/overnight-runs.md`).
