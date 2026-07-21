#!/bin/bash
# lore — Project Memory System Installer
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash
#   ./install.sh                    # Install CLI to ~/.local/bin
#   ./install.sh --skill-dir <path> # Install a skill
#
# Flags:
#   --skill-dir <path>  Install a specific skill (e.g., ~/.hermes/skills/lore)
#   --skill <name>      Skill to install: lore (default), larn, limn, all
#   --hooks <path>      Install git hooks into a project
#   --help              Show this help
#
# Examples:
#   curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash
#   ./install.sh --skill-dir ~/.hermes/skills/lore --skill all
#   ./install.sh --skill-dir ~/.claude/skills/lore

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
HOOKS_DIR="$SCRIPT_DIR/hooks"
BIN_DIR="${HOME}/.local/bin"
SKILL_DIR=""
SKILL_NAME="lore"
PROJECT_DIR=""
DID_SOMETHING=false

usage() {
  sed -n '3,18p' "$0" | sed 's/^# \{0,1\}//'
  exit 0
}

install_cli() {
  mkdir -p "$BIN_DIR"
  local binary=""

  # Check for pre-built binary in releases
  local arch=""
  local os=""
  case "$(uname -m)" in
    x86_64) arch="x64" ;;
    aarch64|arm64) arch="arm64" ;;
    *) echo "[lore] Unknown architecture: $(uname -m). Building from source..." ;;
  esac
  case "$(uname -s)" in
    Darwin) os="macos" ;;
    Linux) os="linux" ;;
    *) echo "[lore] Unknown OS: $(uname -s). Building from source..." ;;
  esac

  if [ -n "$arch" ] && [ -n "$os" ]; then
    local release_url="https://github.com/joabeliot/lore/releases/latest/download/lore-${os}-${arch}.tar.gz"
    echo "[lore] Downloading pre-built binary..."
    if curl -fsSL "$release_url" -o /tmp/lore-release.tar.gz 2>/dev/null; then
      tar -xzf /tmp/lore-release.tar.gz -C /tmp/ 2>/dev/null
      if [ -f /tmp/lore ]; then
        cp /tmp/lore "$BIN_DIR/lore"
        chmod +x "$BIN_DIR/lore"
        rm -f /tmp/lore /tmp/lore-release.tar.gz
        echo "[lore] CLI installed → $BIN_DIR/lore"
        return
      fi
      rm -f /tmp/lore-release.tar.gz
    fi
    echo "[lore] Pre-built binary not available. Building from source..."
  fi

  # Build from source (only works when running from a local clone)
  if [ -f "$SCRIPT_DIR/Cargo.toml" ] && command -v cargo &>/dev/null; then
    echo "[lore] Building from source (cargo)..."
    (cd "$SCRIPT_DIR" && cargo build --release) || {
      echo "[lore] Error: cargo build failed."
      exit 1
    }
    cp "$SCRIPT_DIR/target/release/lore" "$BIN_DIR/lore"
    chmod +x "$BIN_DIR/lore"
    echo "[lore] CLI installed → $BIN_DIR/lore"
    return
  fi

  # No pre-built binary and no local source — give clear instructions
  echo "[lore] No pre-built binary available for your platform yet."
  echo "[lore] To install from source:"
  echo "[lore]   git clone https://github.com/joabeliot/lore.git"
  echo "[lore]   cd lore"
  echo "[lore]   cargo build --release"
  echo "[lore]   cp target/release/lore ~/.local/bin/lore"
  exit 1
}

install_skill() {
  local skill_name="$1"
  local target_dir="$SKILL_DIR"

  case "$skill_name" in
    lore|larn|limn) ;;
    all)
      install_skill lore
      install_skill larn
      install_skill limn
      return
      ;;
    *)
      echo "[lore] Unknown skill: $skill_name. Valid: lore, larn, limn, all"
      exit 1
      ;;
  esac

  local skill_source="$SCRIPT_DIR/skills/$skill_name/SKILL.md"
  if [ ! -f "$skill_source" ]; then
    echo "[lore] Error: skill file not found: $skill_source"
    exit 1
  fi

  mkdir -p "$target_dir"
  cp "$skill_source" "$target_dir/SKILL.md"
  echo "[lore] Skill '$skill_name' installed → $target_dir/SKILL.md"

  # Copy scripts directory if it exists
  if [ -d "$SCRIPT_DIR/skills/$skill_name/scripts" ]; then
    local scripts_target="$target_dir/scripts"
    mkdir -p "$scripts_target"
    cp -r "$SCRIPT_DIR/skills/$skill_name/scripts/"* "$scripts_target/" 2>/dev/null || true
    echo "[lore] Skill scripts installed → $scripts_target/"
  fi
}

install_hooks() {
  local git_hooks_dir="$PROJECT_DIR/.git/hooks"
  if [ ! -d "$PROJECT_DIR/.git" ]; then
    echo "[lore] Error: $PROJECT_DIR is not a git repository."
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
    --skill)
      SKILL_NAME="$2"; shift 2 ;;
    --hooks)
      PROJECT_DIR="$2"; shift 2 ;;
    --help|-h)
      usage ;;
    *)
      echo "[lore] Unknown flag: $1. Run ./install.sh --help for usage." >&2
      exit 1 ;;
  esac
done

# Default: install CLI
if [ -z "$SKILL_DIR" ] && [ -z "$PROJECT_DIR" ]; then
  install_cli
  echo "[lore] Done! Run 'lore --help' to get started."
  exit 0
fi

# Install requested components
if [ -n "$SKILL_DIR" ]; then
  install_skill "$SKILL_NAME"
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

echo "[lore] Done!"
