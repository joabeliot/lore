# Hermes Agent (Nous Research)

**Priority:** heartbeat only (not in the build loop)
**Role:** Notifications to JB (`hermes send`), cron-triggered launches, persistent memory.
**Strengths:** Messaging gateway, cron, sessions.
**Delegate when:** "Ping JB", "start the approved overnight queue".
**Avoid:** Sitting between the conductor and workers; writing or judging code. OpenRouter credit runway is thin.
**Invocation (verified):** `hermes chat -Q -q "<prompt>" --max-turns 1`. Note `hermes -z` returned empty output.
