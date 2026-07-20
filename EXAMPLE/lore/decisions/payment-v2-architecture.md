# Payment Instrument v2 Architecture

**Date:** 2026-07-20
**Status:** Decided

## Decided
Split `PaymentMethod` into a base model + two subtypes: `BankPlaid` (OneToOne) and `CardStripe` (OneToOne). Sensitive bank fields encrypted via AWS KMS envelope encryption.

## Why
The original single-model design used nullable fields to handle both bank and card data. This made queries awkward, sensitive data co-mingled with display-safe data, and type-specific logic scattered across the codebase. The split makes types explicit, isolates sensitive data to `BankPlaid`, and gives each instrument type room to evolve independently.

## Rejected
- **Single model with type field** — rejected because of field sprawl and co-mingled sensitive data
- **Polymorphic model (django-polymorphic)** — rejected because it adds a heavy dependency for a two-type problem we can solve cleanly with OneToOne
- **Separate databases per instrument type** — rejected as over-engineering for current scale

## Consequences
- API consumers get typed objects — frontend knows what fields to expect per type
- Migrations are more complex (two-step: nullable add → backfill → enforce)
- KMS adds latency on reads of sensitive fields — acceptable for this use case since it's write-time and admin-only reads
