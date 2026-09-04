#!/bin/bash
# lore — Project Memory System Installer
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash
#   ./install.sh                    # Install CLI + Claude Code hooks (if Claude detected)
#   ./install.sh --skill-dir <path> # Install a skill
#   ./install.sh --claude           # (Re-)install Claude Code hooks and skill
#   ./install.sh --verify           # Verify the Claude Code hook is correctly installed
#
# Flags:
#   --skill-dir <path>  Install a specific skill (e.g., ~/.hermes/skills/lore)
#   --skill <name>      Skill to install: lore (default), larn, limn, all
#   --hooks <path>      Install git hooks into a project
#   --claude            Wire lore into Claude Code (settings.json SessionStart hook + skill)
#   --verify            Check that the Claude Code hook is present and correct (no writes)
#   --help              Show this help
#
# Examples:
#   curl -fsSL https://raw.githubusercontent.com/joabeliot/lore/main/install.sh | bash
#   ./install.sh --skill-dir ~/.hermes/skills/lore --skill all
#   ./install.sh --claude
#   ./install.sh --verify

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
HOOKS_DIR="$SCRIPT_DIR/hooks"
BIN_DIR="${HOME}/.local/bin"
SKILL_DIR=""
SKILL_NAME="lore"
PROJECT_DIR=""
INSTALL_CLAUDE=false
VERIFY_CLAUDE=false
DID_SOMETHING=false

usage() {
  sed -n '3,20p' "$0" | sed 's/^# \{0,1\}//'
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

  # Build from source
  if command -v cargo &>/dev/null; then
    echo "[lore] Building from source (cargo)..."
    (cd "$SCRIPT_DIR" && cargo build --release) || {
      echo "[lore] Error: cargo build failed."
      exit 1
    }
    cp "$SCRIPT_DIR/target/release/lore" "$BIN_DIR/lore"
    chmod +x "$BIN_DIR/lore"
    echo "[lore] CLI installed → $BIN_DIR/lore"
  else
    echo "[lore] Error: Rust not found. Install Rust or use a pre-built release."
    echo "[lore] See https://rustup.rs"
    exit 1
  fi
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

verify_claude() {
  local settings="$HOME/.claude/settings.json"

  if [ ! -f "$settings" ]; then
    echo "[lore] ✗ $settings not found"
    echo "[lore]   Fix: run  ./install.sh --claude"
    return 1
  fi

  if ! command -v python3 &>/dev/null; then
    echo "[lore] ✗ python3 not found — cannot verify"
    return 1
  fi

  python3 - <<'PYEOF'
import json, os, sys

settings_path = os.path.expanduser("~/.claude/settings.json")

try:
    with open(settings_path) as f:
        settings = json.load(f)
except Exception as e:
    print(f"[lore] ✗ Could not parse settings.json: {e}")
    sys.exit(1)

session_start = settings.get("hooks", {}).get("SessionStart", [])

lore_entry = next(
    (e for e in session_start
     if any("lore/GUARDRAILS.md" in h.get("command", "")
            for h in e.get("hooks", []))),
    None,
)

if not lore_entry:
    print("[lore] ✗ SessionStart hook NOT found in ~/.claude/settings.json")
    print("[lore]   Fix: run  ./install.sh --claude")
    sys.exit(1)

cmd = (lore_entry.get("hooks") or [{}])[0].get("command", "")
guardrails_ok = "lore/GUARDRAILS.md" in cmd
context_ok    = "lore/CONTEXT.md"    in cmd
matcher       = lore_entry.get("matcher", "")

print("[lore] ✓ SessionStart hook found")
print(f"[lore]   matcher  : {matcher or '(empty — fires on every session start)'}")
print(f"[lore]   GUARDRAILS.md loaded : {'yes' if guardrails_ok else 'NO — hook may be incomplete'}")
print(f"[lore]   CONTEXT.md loaded    : {'yes' if context_ok    else 'NO — hook may be incomplete'}")

skill_path = os.path.expanduser("~/.claude/skills/lore/SKILL.md")
if os.path.exists(skill_path):
    print(f"[lore] ✓ Skill installed at {skill_path}")
else:
    print(f"[lore] ✗ Skill not found at {skill_path}")
    print("[lore]   Fix: run  ./install.sh --claude")

if not (guardrails_ok and context_ok):
    sys.exit(1)
PYEOF
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

install_claude() {
  local settings="$HOME/.claude/settings.json"
  local claude_dir="$HOME/.claude"

  if [ ! -d "$claude_dir" ]; then
    echo "[lore] Claude Code not detected ($claude_dir not found) — skipping."
    return
  fi

  if ! command -v python3 &>/dev/null; then
    echo "[lore] Warning: python3 not found — cannot write Claude hook. Add manually:"
    echo '  SessionStart hook command: if [ -d "lore" ]; then cat lore/GUARDRAILS.md 2>/dev/null; tail -150 lore/CONTEXT.md 2>/dev/null; fi'
    return
  fi

  python3 - <<'PYEOF'
import json, os, sys

settings_path = os.path.expanduser("~/.claude/settings.json")

hook_entry = {
    "matcher": "startup|resume|clear|compact",
    "hooks": [
        {
            "type": "command",
            "command": (
                'if [ -d "lore" ]; then'
                ' echo "=== Project lore auto-loaded by global hook ===";'
                ' echo "";'
                ' echo "--- lore/GUARDRAILS.md ---";'
                ' cat lore/GUARDRAILS.md 2>/dev/null;'
                ' echo "";'
                ' echo "--- lore/CONTEXT.md (last 150 lines) ---";'
                ' tail -150 lore/CONTEXT.md 2>/dev/null;'
                ' fi'
            )
        }
    ]
}

# Load or create settings
if os.path.exists(settings_path):
    with open(settings_path, 'r') as f:
        try:
            settings = json.load(f)
        except json.JSONDecodeError:
            settings = {}
else:
    settings = {}

hooks = settings.setdefault("hooks", {})
session_start = hooks.setdefault("SessionStart", [])

# Idempotent: skip if our hook is already there
already = any(
    any(
        "lore/GUARDRAILS.md" in h.get("command", "")
        for h in entry.get("hooks", [])
    )
    for entry in session_start
)

if already:
    print("[lore] Claude Code hook already installed — skipping")
    sys.exit(0)

session_start.append(hook_entry)

os.makedirs(os.path.dirname(settings_path), exist_ok=True)
with open(settings_path, 'w') as f:
    json.dump(settings, f, indent=2)

print(f"[lore] Claude Code SessionStart hook installed → {settings_path}")
PYEOF

  # Also copy the lore skill to ~/.claude/skills/
  local claude_skill_dir="$claude_dir/skills/lore"
  local skill_source=""

  # Prefer local source if running from the repo; fall back to download
  if [ -f "$SCRIPT_DIR/skills/lore/SKILL.md" ]; then
    skill_source="$SCRIPT_DIR/skills/lore/SKILL.md"
  else
    echo "[lore] Downloading lore skill..."
    local tmp_skill="/tmp/lore_SKILL.md"
    if curl -fsSL "https://raw.githubusercontent.com/joabeliot/lore/main/skills/lore/SKILL.md" -o "$tmp_skill" 2>/dev/null; then
      skill_source="$tmp_skill"
    else
      echo "[lore] Warning: could not download lore skill — install manually with --skill-dir"
    fi
  fi

  if [ -n "$skill_source" ]; then
    mkdir -p "$claude_skill_dir"
    cp "$skill_source" "$claude_skill_dir/SKILL.md"
    echo "[lore] Skill installed → $claude_skill_dir/SKILL.md"
  fi

  echo ""
  verify_claude
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
    --claude)
      INSTALL_CLAUDE=true; shift ;;
    --verify)
      VERIFY_CLAUDE=true; shift ;;
    --help|-h)
      usage ;;
    *)
      echo "[lore] Unknown flag: $1. Run ./install.sh --help for usage." >&2
      exit 1 ;;
  esac
done

# --verify: read-only check, exits immediately
if [ "$VERIFY_CLAUDE" = true ]; then
  verify_claude
  exit $?
fi

# Default: install CLI, then auto-wire Claude Code if detected
if [ -z "$SKILL_DIR" ] && [ -z "$PROJECT_DIR" ] && [ "$INSTALL_CLAUDE" = false ]; then
  install_cli
  install_claude
  echo "[lore] Done! Run 'lore --help' to get started."
  exit 0
fi

# Install requested components
if [ "$INSTALL_CLAUDE" = true ]; then
  install_claude
  DID_SOMETHING=true
fi

if [ "$VERIFY_CLAUDE" = true ]; then
  verify_claude
  DID_SOMETHING=true
fi

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
