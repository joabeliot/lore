# Codex — Project Instructions

## Output Formatting
- All generated Python must pass `black` formatting — use 88 char line length
- Import order: stdlib → third-party → Django → local app imports
- Generated files must include the module docstring: `"""[purpose of this file]"""`

## What to Generate vs What to Leave
- Generate: serializers, admin registrations, URL patterns, test stubs
- Leave for Claude Code: views with complex business logic, model relationships, migrations

## Handoff Format
When done, output:
1. Each file clearly delimited with `# === FILE: path/to/file.py ===`
2. A summary list of files generated
3. Any assumptions made that the conductor should verify
