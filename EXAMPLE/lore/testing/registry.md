# Test Registry

## Covered
| Area | Test file | Type | Notes |
|---|---|---|---|
| Auth / login | `tests/test_auth.py` | Unit | Happy path + wrong password + expired token |
| Stripe Connect onboarding | `tests/test_stripe.py` | Integration | Mocked Stripe SDK |
| One-time giving | `tests/test_giving.py` | Integration | Covers success + card decline |

## Not Covered
- PaymentMethod v2 (BankPlaid / CardStripe) — in progress, tests pending
- Plaid webhook handling
- Celery recurring giving task
- KMS encryption/decryption

## Known Gaps (accepted)
- No E2E tests — Flutter + API integration tested manually for now
- Stripe webhooks tested with Stripe CLI in local dev, not in CI
