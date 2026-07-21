# Claude Code

**Priority:** #2 — Architect + Reviewer. Reserve for complexity, do NOT default to Claude for everything.

**Role:** Primary backend and full-stack executor. Handles Django views, models, migrations, API endpoints, Celery tasks, and Flutter integration work.
**Strengths:** Reading and editing existing code in context, multi-file changes, understanding project conventions from lore, writing tests
**Delegate when:** Backend feature work, bug fixes, refactors, migrations, DRF views, Celery tasks, test writing, complex multi-file work
**Avoid:** Tasks requiring browser interaction or live environment access. Simple one-file edits or grindy UI boilerplate (use agy for those).
**Invocation:** Claude Code runs inside the project repo with full file access. Receives delegation packets via prompt. Reports back with file changes and lore updates.
