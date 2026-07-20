# Guardrails

## Always
- Read `lore/CONTEXT.md` before starting any work
- Write migrations for every model change
- Log decisions to `lore/decisions/` when something significant is chosen or rejected
- Move kanban tasks when their state changes

## Never
- Push to main directly — staging branch only
- Delete entries from `CONTEXT.md`, `kanban/done.md`, or `decisions/`
- Invent API responses — check `architecture/apis.md` first
- Skip the session log at the end of a session

## Conventions
- Model names: PascalCase singular (`PaymentMethod`, not `payment_methods`)
- API endpoints: kebab-case (`/api/payment-methods/`, not `/api/paymentMethods/`)
- Feature files: kebab-case slug matching the feature name (`features/payment-instruments.md`)
- Decision files: kebab-case slug (`decisions/payment-v2-architecture.md`)

## Backend
- All views use DRF class-based views
- Celery tasks live in `tasks.py` per app, never in `views.py`
- Soft-delete pattern: `is_active` flag, never hard delete user data

## Frontend
- Flutter: BLoC for state management
- No business logic in widgets — everything goes through the BLoC layer
