#!/bin/bash
# Install or update the lore skill to your Claude Code skills directory.
# Run this once to install, and again any time you pull updates.

SKILL_DIR="$HOME/.claude/skills/lore"

mkdir -p "$SKILL_DIR"
cp SKILLS.md "$SKILL_DIR/SKILL.md"

echo "Done. Skill installed at $SKILL_DIR/SKILL.md"
