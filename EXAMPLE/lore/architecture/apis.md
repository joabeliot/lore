# API Contracts

## Internal API
**Base URL:** `https://api.myproject.com/api/`  
**Auth:** JWT Bearer token (`Authorization: Bearer <token>`)  
**Versioning:** URL prefix (`/api/v1/`, `/api/v2/`)

### Payment Methods
| Method | Endpoint | Description |
|---|---|---|
| GET | `/api/v2/payment-methods/` | List user's instruments |
| POST | `/api/v2/payment-methods/bank/` | Add bank via Plaid |
| POST | `/api/v2/payment-methods/card/` | Add card via Stripe |
| DELETE | `/api/v2/payment-methods/{id}/` | Soft-delete instrument |
| PATCH | `/api/v2/payment-methods/{id}/default/` | Set as default |

### Giving
| Method | Endpoint | Description |
|---|---|---|
| POST | `/api/v1/giving/one-time/` | One-time gift |
| POST | `/api/v1/giving/recurring/` | Start recurring giving |
| GET | `/api/v1/giving/history/` | Transaction history |

## External Services

### Stripe Connect
- **Dashboard:** https://dashboard.stripe.com
- **Webhook endpoint:** `/api/v1/webhooks/stripe/`
- **Key events:** `payment_intent.succeeded`, `account.updated`, `payout.paid`
- **Gotcha:** Stripe sends webhooks in test mode even for live accounts — always check `livemode` flag

### Plaid
- **Webhook endpoint:** `/api/v1/webhooks/plaid/`
- **Key events:** `TRANSACTIONS_SYNC`, `ITEM_ERROR`
- **Gotcha:** Balance check can return `null` for some account types — handle gracefully
