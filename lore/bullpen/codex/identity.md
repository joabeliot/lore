# Codex (OpenAI CLI)

**Priority:** #2: independent reviewer and QA
**Role:** Reviews the builder's diff and runs verification. Must never review its own work.
**Strengths:** Focused QA, test running, second-opinion review; free credits.
**Delegate when:** A ticket is built and needs an independent verdict.
**Avoid:** Multi-step feature building; architecture.
**Invocation:** `codex exec "<task>"` inside a git repo (codex-cli 0.144.5 installed). Permission model for unattended runs still to be verified.
