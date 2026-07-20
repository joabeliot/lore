# Data Models

## PaymentMethod (base)
Central record for any payment instrument a user adds.

| Field | Type | Notes |
|---|---|---|
| `id` | UUID | Primary key |
| `user` | FK → User | Owner |
| `type` | CharField | `bank` or `card` |
| `is_active` | Boolean | Soft delete — never hard delete |
| `is_default` | Boolean | One default per user |
| `created_at` | DateTime | — |

## BankPlaid
Extends PaymentMethod for Plaid-linked bank accounts.

| Field | Type | Notes |
|---|---|---|
| `payment_method` | OneToOne → PaymentMethod | — |
| `plaid_item_id` | CharField | Plaid item identifier |
| `account_number_enc` | BinaryField | KMS envelope encrypted |
| `routing_number_enc` | BinaryField | KMS envelope encrypted |
| `account_mask` | CharField | Last 4 digits, plaintext for display |
| `institution_name` | CharField | — |

## CardStripe
Extends PaymentMethod for Stripe-tokenized cards.

| Field | Type | Notes |
|---|---|---|
| `payment_method` | OneToOne → PaymentMethod | — |
| `stripe_pm_id` | CharField | Stripe PaymentMethod ID |
| `last4` | CharField | Display only |
| `brand` | CharField | visa / mastercard / etc |
| `exp_month` | IntegerField | — |
| `exp_year` | IntegerField | — |

## Key Patterns
- Soft delete: `is_active = False`, never `DELETE`
- Sensitive fields encrypted at rest via KMS envelope encryption
- `account_mask` / `last4` always plaintext — safe to display, never secret
