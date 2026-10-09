# Idea: doorstallor as the pilot project for the agentic cycle

Captured 2026-10-09. doorstallor (Flutter crew/admin apps + Cloudflare Worker, `~/projects/arque/doorstallor`, branch `backend-rebuild`) is the first real project to run through lore's conductor. These findings come from an agy read-only survey plus our own verification. Belongs in doorstallor's own `lore/` too (not yet copied there).

## Verified in code (by Claude)
- Admin's **release** Android manifest lacks `INTERNET` (only debug/profile have it); crew's main manifest has it. Release admin builds can't reach the network.
- `exchangeFirebaseToken` queries `users` by phone with `limit: 1`, no `active`/`role` filter: the root cause class of the 2026-10-06 login bug (data was fixed, code wasn't). `POST /workers` doesn't reject a duplicate phone/employee id.
- `deploy.sh` still targets `installer/` and `inspector/`; `inspector/` no longer exists.
- `sendPush`/`sendPushToMany` in `worker/src/lib/fcm.js` are never called; crew never registers an FCM token (admin does).
- Both apps' `api_client.dart:61` create `http.Client()` per request and never close it.
- `kGold` is `0xFF2563EB` (blue) in both themes.
- The 7 `models/*.dart` files are never imported. `legacy.js` is still the fallback in `worker/src/index.js`.

## Reported by agy, NOT verified
- Crew `screens_golden_test.dart` doesn't compile (old package name, deleted screen, missing arg).
- Timezone mismatch: `attendance_screen.dart` local date vs `dates.js` UTC day range.
- `main.dart` overlay code violates "keep main.dart thin"; no byte-level upload progress.
- Image index shift from `whereType<String>()` on null presigned URLs.
- Wrong in agy's report: it called `installer/build/` and `worker/src/db/` empty; they are not.

## Gate readiness (why this pilot needs LOR-3/LOR-4 first)
- `lore inspect` on doorstallor currently passes vacuously ("No build system detected"): no Cargo.toml/package.json at the root.
- No CI, no `AGENTS.md`. Tests: placeholder widget tests, stale goldens, tiny untracked Patrol smoke tests (13 lines each), no worker test runner. Only worker coverage = FML flows against live staging.
- Credentials: `google-services.json` (dev/prod) tracked in git; agents run with JB's wrangler/Firebase logins, so worktrees isolate files, not credentials.
- Overnight runs would hit the live staging Worker/Firestore JB just wiped for manual testing.

## First pilot tickets (candidates)
1. Admin release `INTERNET` permission (static check: manifest assert; plus release build).
2. Harden login lookup + duplicate-phone rejection (FML red-then-green, including the fitter OTP flow FML doesn't cover today).
3. Repair `deploy.sh` paths.

## Not decided
Whether doorstallor tickets live in its own lore session (DRS) as today, with lore's conductor operating on them: almost certainly yes.
