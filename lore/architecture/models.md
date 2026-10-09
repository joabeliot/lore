# Models — lore

Defined in `src/models.rs`. Stub: field-level detail is read from the source of truth.

- **Session** (`~/.lore/sessions/<uuid>.yml`): uid, project, shorthand, description,
  working_dir, lore_dir, config_file, created_at (date only).
- **Config** (`lore/config.yml`): project, session, description, working_dir, lore_dir, ticket path.
- **Ticket** (`lore/workspace/ticket.json`): id (`<SHORTHAND>-<n>`), name, description,
  context[], plan?, status, priority, tags[], source, created, assigned_to?,
  started_at?, completed_at?, logs[] (`{at, event, detail?}`).
- **TicketStatus**: backlog | todo | inprogress | done. **Priority**: P0–P3.
- **TicketFile**: shorthand, tickets[], counter.

## Quirks
- Optional/new fields use `#[serde(default)]`; this is what keeps old files loadable.
- Dates are date-only strings on tickets, RFC 3339 on log entries.
- No transition rules exist yet: any status can be set to any other (see ADR F2).
