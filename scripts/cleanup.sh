#!/usr/bin/env bash
# VantisOS cleanup script.
# Usage:
#   ./scripts/cleanup.sh [--dry-run] [--tracked]
# Flags:
#   --dry-run  Show what would be removed, do not modify files
#   --tracked  Also remove tracked garbage/build artifacts from Git index

set -euo pipefail

DRY_RUN=0
REMOVE_TRACKED=0

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY_RUN=1 ;;
    --tracked) REMOVE_TRACKED=1 ;;
    *)
      echo "Unknown argument: $arg" >&2
      exit 1
      ;;
  esac
done

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== VantisOS Cleanup =="
echo "Root: $ROOT"
echo "Dry run: $DRY_RUN"
echo "Remove tracked artifacts: $REMOVE_TRACKED"
echo

run_cmd() {
  if [[ "$DRY_RUN" -eq 1 ]]; then
    echo "[dry-run] $*"
  else
    eval "$*"
  fi
}

remove_if_exists() {
  local p="$1"
  if [[ -e "$p" ]]; then
    if [[ "$DRY_RUN" -eq 1 ]]; then
      echo "Would remove: $p"
    else
      rm -rf "$p"
      echo "Removed: $p"
    fi
  fi
}

remove_if_exists "target"
remove_if_exists "src/verified/target"

UNTRACKED_TRASH="$(rg --files . -g '*.rs.bk' -g '*.backup' -g '*.tmp' -g '*.orig' -g '*.rej' -g '*.long-type-*.txt' || true)"
if [[ -n "$UNTRACKED_TRASH" ]]; then
  while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    remove_if_exists "$f"
  done <<< "$UNTRACKED_TRASH"
else
  echo "No matching trash files found in working tree."
fi

TRACKED_GARBAGE="$(git ls-files | rg '^src/verified/target/|\.rs\.bk$|\.backup$|\.tmp$|\.orig$|\.rej$' || true)"
if [[ -n "$TRACKED_GARBAGE" ]]; then
  if [[ "$REMOVE_TRACKED" -eq 1 ]]; then
    while IFS= read -r f; do
      [[ -z "$f" ]] && continue
      run_cmd "git rm -r -- \"$f\""
      echo "Removed tracked artifact: $f"
    done <<< "$TRACKED_GARBAGE"
  else
    echo "Tracked garbage/artifacts detected (not removed):"
    echo "$TRACKED_GARBAGE"
    echo "Re-run with --tracked to remove from Git index."
  fi
else
  echo "No tracked garbage/artifacts detected."
fi

echo
echo "Cleanup finished."
echo "Next step: git status"