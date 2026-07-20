# Claude Code — Project Skills

## Django Patterns Used Here
- All views: DRF class-based views (`APIView` or `GenericAPIView`) — never function-based
- All serializers live in `serializers.py` per app — never inline in views
- Celery tasks live in `tasks.py` per app — never in views or models
- Soft delete via `is_active = False` — never hard delete user data
- Custom managers: always check `is_active=True` in default querysets

## Flutter Patterns Used Here
- BLoC for state management — no Provider, no Riverpod
- No business logic in widgets — everything through the BLoC layer
- API calls live in `repositories/` — never directly in BLoCs or widgets

## Migration Rules
- Two-step for NOT NULL additions: add nullable → backfill → enforce
- Always name migrations descriptively: `0042_add_payment_method_v2_fields`
- Never squash migrations in production branches
