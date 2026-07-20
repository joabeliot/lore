#!/bin/bash
# lore install tool
#
# A composable installer. Pass the flags you need — nothing is assumed.
# Any agent (Claude Code, Hermes, Codex, custom) calls this with its own params.
#
# Usage:
#   ./install.sh [--skill-dir <path>] [--conductor-dir <path>] [--hooks <project-path>]
#
# Flags:
#   --skill-dir <path>      Install SKILLS.md as SKILL.md into this directory
#   --conductor-dir <path>  Install CONDUCTOR.md into this directory
#   --hooks <path>          Install post-commit hook into this project's .git/hooks/
#   --help                  Show this help
#
# Examples:
#   Claude Code:
#     ./install.sh --skill-dir ~/.claude/skills/lore
#
#   Hermes / custom conductor:
#     ./install.sh --skill-dir ~/.hermes/skills/lore --conductor-dir ~/.hermes/skills/lore
#
#   With hooks:
#     ./install.sh --skill-dir ~/.claude/skills/lore --hooks /path/to/your/project
#
#   Full install for a conductor agent:
#     ./install.sh \
#       --skill-dir ~/.hermes/skills/lore \
#       --conductor-dir ~/.hermes/skills/lore \
#       --hooks /path/to/your/project

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
HOOKS_DIR="$SCRIPT_DIR/hooks"
SKILL_DIR=""
CONDUCTOR_DIR=""
PROJECT_DIR=""
DID_SOMETHING=false

usage() {
  sed -n '3,25p' "$0" | sed 's/^# \{0,1\}//'
  exit 0
}

install_skill() {
  mkdir -p "$SKILL_DIR"
  cp "$SCRIPT_DIR/SKILLS.md" "$SKILL_DIR/SKILL.md"
  echo "[lore] Skill installed → $SKILL_DIR/SKILL.md"
}

install_conductor() {
  mkdir -p "$CONDUCTOR_DIR"
  cp "$SCRIPT_DIR/CONDUCTOR.md" "$CONDUCTOR_DIR/CONDUCTOR.md"
  echo "[lore] Conductor installed → $CONDUCTOR_DIR/CONDUCTOR.md"
}

install_hooks() {
  local git_hooks_dir="$PROJECT_DIR/.git/hooks"

  if [ ! -d "$PROJECT_DIR/.git" ]; then
    echo "[lore] Error: $PROJECT_DIR is not a git repository." >&2
    exit 1
  fi

  cp "$HOOKS_DIR/post-commit.sh" "$git_hooks_dir/post-commit"
  chmod +x "$git_hooks_dir/post-commit"
  echo "[lore] Hook installed → $git_hooks_dir/post-commit"
}

# Parse flags
while [[ $# -gt 0 ]]; do
  case "$1" in
    --skill-dir)
      SKILL_DIR="$2"; shift 2 ;;
    --conductor-dir)
      CONDUCTOR_DIR="$2"; shift 2 ;;
    --hooks)
      PROJECT_DIR="$2"; shift 2 ;;
    --help|-h)
      usage ;;
    *)
      echo "[lore] Unknown flag: $1. Run ./install.sh --help for usage." >&2
      exit 1 ;;
  esac
done

# Execute requested installs
if [ -n "$SKILL_DIR" ]; then
  install_skill
  DID_SOMETHING=true
fi

if [ -n "$CONDUCTOR_DIR" ]; then
  install_conductor
  DID_SOMETHING=true
fi

if [ -n "$PROJECT_DIR" ]; then
  install_hooks
  DID_SOMETHING=true
fi

if [ "$DID_SOMETHING" = false ]; then
  echo "[lore] Nothing to install. Run ./install.sh --help for usage."
  exit 1
fi
