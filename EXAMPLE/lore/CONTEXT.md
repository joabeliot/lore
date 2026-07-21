# Context

**Focus:** Epic 3 — payment instrument v2 (BankPlaid + CardStripe split)
**Phase:** Beta
**Open:** views_v2.py not started, Plaid webhook edge cases unresolved
**Next:** Build views_v2.py, write migration for PaymentMethod v2 fields

---

## Log

### 2026-07-20 — JB
Designed v2 payment instrument architecture: split BankPlaid and CardStripe into separate models, both referencing a shared PaymentMethod base. KMS envelope encryption confirmed for sensitive fields. Decision logged.
Loaded: `architecture/models.md`, `decisions/payment-v2-architecture.md`
Left open: views_v2.py implementation, Plaid balance edge cases on low-balance accounts
Carry forward: v2 architecture decided — next web session re-prime on BankPlaid/CardStripe split and KMS encryption approach before designing views_v2.py

### 2026-07-15 — JB
Wired Stripe Connect onboarding flow end-to-end. Fixed redirect URI mismatch in staging. Account status now syncs via webhook.
Loaded: `features/stripe-connect.md`, `architecture/apis.md`
Left open: payout scheduling not yet triggered from frontend
Carry forward: Stripe Connect onboarding live — payout trigger from frontend pending, re-prime on webhook sync approach before designing payout scheduling

---
