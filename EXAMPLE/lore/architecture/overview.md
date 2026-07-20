# Architecture Overview

## Services
- **my-project-api** — Django REST API (port 8000). Main backend. Handles auth, users, giving, payments.
- **my-project-tasks** — Celery worker. Handles async: scheduled giving, webhook processing, email sends.

## Data Flow
```
Flutter App → API (Django) → PostgreSQL
                           → Celery (via Redis) → external services (Stripe, Plaid)
                           → Cloudflare (CDN/WAF)
```

## Infra
- EC2 (t3.medium) — API + Celery on same instance for now
- RDS PostgreSQL — primary database
- Redis (ElastiCache) — Celery broker + Django cache
- Cloudflare — DNS, WAF, CDN for static assets
- S3 — file storage (receipts, profile images)

## External Dependencies
- **Stripe Connect** — payment processing and payout management
- **Plaid** — bank account linking and balance checks
- **AWS KMS** — envelope encryption for sensitive payment fields
- **SendGrid** — transactional email
