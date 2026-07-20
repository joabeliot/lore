#!/bin/bash
# Install or update the lore skill and optionally install git hooks.
# Usage:
#   ./install.sh                          — install/update the skill only
#   ./install.sh --hooks /path/to/project — also install post-commit hook in that project

SKILL_DIR="$HOME/.claude/skills/lore"
HOOKS_DIR="$(dirname "$0")/hooks"

install_skill() {
  mkdir -p "$SKILL_DIR"
  cp SKILLS.md "$SKILL_DIR/SKILL.md"
  echo "Skill installed at $SKILL_DIR/SKILL.md"
}

install_hooks() {
  local project_dir="$1"
  local git_hooks_dir="$project_dir/.git/hooks"

  if [ ! -d "$project_dir/.git" ]; then
    echo "Error: $project_dir is not a git repository."
    exit 1
  fi

  cp "$HOOKS_DIR/post-commit.sh" "$git_hooks_dir/post-commit"
  chmod +x "$git_hooks_dir/post-commit"
  echo "Hook installed at $git_hooks_dir/post-commit"
}

install_skill

if [ "$1" = "--hooks" ]; then
  if [ -z "$2" ]; then
    echo "Usage: ./install.sh --hooks /path/to/your/project"
    exit 1
  fi
  install_hooks "$2"
fi
