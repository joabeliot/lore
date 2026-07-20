# Feature: Payment Instruments v2

**Status:** In Progress

## What It Does
Redesigns how payment instruments are stored. Instead of one monolithic `PaymentMethod` model, v2 splits bank accounts (Plaid) and cards (Stripe) into separate models (`BankPlaid`, `CardStripe`) that reference a shared `PaymentMethod` base. Sensitive fields are encrypted at rest using KMS envelope encryption.

## Why
The original model tried to handle both bank and card with optional fields — messy to query, unsafe for sensitive data, hard to extend. V2 makes the type split explicit and adds proper encryption.

## Edge Cases
- User has both a bank and a card — both can be active, only one is default
- Plaid re-links an already-linked account — should update existing `BankPlaid`, not create a new one
- KMS key rotation — encryption should be re-applied, not just re-keyed at the KMS level

## Open Questions
- Do we expose `BankPlaid` and `CardStripe` as separate API endpoints or a unified `/payment-methods/` endpoint that returns typed objects?

## Notes
- Decision logged: `decisions/payment-v2-architecture.md`
- Migration strategy: add new fields as nullable, backfill, then enforce NOT NULL in a second migration
