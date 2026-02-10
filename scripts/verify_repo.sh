#!/usr/bin/env bash
# VantisOS repository verification script.
# Usage:
#   ./scripts/verify_repo.sh          # quick checks
#   ./scripts/verify_repo.sh --full   # includes tests and clippy

set -euo pipefail

MODE="quick"
if [[ "${1:-}" == "--full" ]]; then
  MODE="full"
fi

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

CHECKS=0
WARNINGS=0
ERRORS=0

pass() { echo -e "${GREEN}OK${NC}  $1"; CHECKS=$((CHECKS + 1)); }
warn() { echo -e "${YELLOW}WARN${NC} $1"; WARNINGS=$((WARNINGS + 1)); }
fail() { echo -e "${RED}ERR${NC}  $1"; ERRORS=$((ERRORS + 1)); }
info() { echo -e "${BLUE}INFO${NC} $1"; }

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "== VantisOS Repository Verification =="
echo "Mode: $MODE"
echo "Root: $ROOT"
echo

if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  pass "Git repository detected"
else
  fail "Current directory is not a Git repository"
fi

CURRENT_BRANCH="$(git branch --show-current || true)"
if [[ -n "$CURRENT_BRANCH" ]]; then
  info "Current branch: $CURRENT_BRANCH"
fi

if git diff --quiet && git diff --cached --quiet; then
  pass "Worktree is clean"
else
  warn "Worktree has uncommitted changes"
fi

REQUIRED_DIRS=(
  ".github"
  "docs"
  "scripts"
  "src"
  "src/verified"
)
for d in "${REQUIRED_DIRS[@]}"; do
  [[ -d "$d" ]] && pass "Directory exists: $d" || fail "Missing directory: $d"
done

REQUIRED_FILES=(
  ".gitignore"
  "README.md"
  "CONTRIBUTING.md"
  "SECURITY.MD"
  "docs/README.md"
  "src/verified/Cargo.toml"
)
for f in "${REQUIRED_FILES[@]}"; do
  [[ -f "$f" ]] && pass "File exists: $f" || fail "Missing file: $f"
done

if [[ -f ".gitignore" ]]; then
  pass ".gitignore present"
  for p in "src/verified/target/" "**/*.rs.bk" "*.env" "node_modules/"; do
    if rg -F -q "$p" ".gitignore"; then
      pass "Ignore rule present: $p"
    else
      warn "Missing ignore rule: $p"
    fi
  done
fi

TRACKED_TARGET_COUNT="$(git ls-files "src/verified/target/**" | wc -l | tr -d ' ')"
if [[ "$TRACKED_TARGET_COUNT" == "0" ]]; then
  pass "No tracked build artifacts under src/verified/target"
else
  fail "Tracked build artifacts detected under src/verified/target ($TRACKED_TARGET_COUNT files)"
fi

TRACKED_TRASH="$(git ls-files | rg '\.backup$|\.bak$|\.tmp$|\.orig$|\.rej$|\.rs\.bk$' || true)"
if [[ -z "$TRACKED_TRASH" ]]; then
  pass "No tracked backup or patch-reject files"
else
  fail "Tracked backup/trash files detected"
  echo "$TRACKED_TRASH"
fi

SCRIPT_COUNT=0
for s in scripts/*.sh; do
  [[ -f "$s" ]] || continue
  SCRIPT_COUNT=$((SCRIPT_COUNT + 1))
  if bash -n "$s"; then
    pass "Script syntax OK: $s"
  else
    fail "Script syntax error: $s"
  fi
  if [[ -x "$s" ]]; then
    pass "Script executable: $s"
  else
    warn "Script is not executable: $s"
  fi
done
info "Shell scripts checked: $SCRIPT_COUNT"

if command -v cargo >/dev/null 2>&1; then
  pass "cargo available"
else
  fail "cargo not found in PATH"
fi

pushd src/verified >/dev/null
if cargo check --locked >/dev/null; then
  pass "cargo check --locked passed"
else
  fail "cargo check --locked failed"
fi

if [[ "$MODE" == "full" ]]; then
  if cargo test --locked >/dev/null; then
    pass "cargo test --locked passed"
  else
    fail "cargo test --locked failed"
  fi

  if cargo clippy --locked -- -D warnings >/dev/null; then
    pass "cargo clippy --locked -- -D warnings passed"
  else
    fail "cargo clippy strict mode failed"
  fi
fi
popd >/dev/null

run_optional_crate_checks() {
  local crate_dir="$1"
  local crate_label="$2"

  if [[ ! -f "${crate_dir}/Cargo.toml" ]]; then
    return
  fi

  pushd "$crate_dir" >/dev/null
  if cargo check --locked >/dev/null; then
    pass "${crate_label} cargo check --locked passed"
  else
    fail "${crate_label} cargo check --locked failed"
  fi

  if [[ "$MODE" == "full" ]]; then
    if cargo test --locked >/dev/null; then
      pass "${crate_label} cargo test --locked passed"
    else
      fail "${crate_label} cargo test --locked failed"
    fi

    if cargo clippy --locked -- -D warnings >/dev/null; then
      pass "${crate_label} cargo clippy strict mode passed"
    else
      fail "${crate_label} cargo clippy strict mode failed"
    fi
  fi
  popd >/dev/null
}

run_optional_crate_checks "security" "security crate"
run_optional_crate_checks "cortex" "cortex crate"
run_optional_crate_checks "cytadela" "cytadela crate"
run_optional_crate_checks "horizon" "horizon crate"
run_optional_crate_checks "store" "store crate"

if [[ -x "scripts/check_traceability.sh" ]]; then
  if ./scripts/check_traceability.sh >/dev/null; then
    pass "traceability check passed"
  else
    fail "traceability check failed"
  fi
else
  warn "scripts/check_traceability.sh is missing or not executable"
fi

if [[ -x "scripts/check_requirement_ids.sh" ]]; then
  if ./scripts/check_requirement_ids.sh >/dev/null; then
    pass "requirement-id check passed or skipped"
  else
    fail "requirement-id check failed"
  fi
else
  warn "scripts/check_requirement_ids.sh is missing or not executable"
fi

BRANCH_COUNT="$(git branch -a | wc -l | tr -d ' ')"
TAG_COUNT="$(git tag | wc -l | tr -d ' ')"
COMMITS_COUNT="$(git rev-list --count HEAD | tr -d ' ')"
info "Branch refs: $BRANCH_COUNT"
info "Tags: $TAG_COUNT"
info "Commits on current branch: $COMMITS_COUNT"

echo
echo "== Summary =="
echo "Passed checks:  $CHECKS"
echo "Warnings:       $WARNINGS"
echo "Errors:         $ERRORS"

if [[ "$ERRORS" -gt 0 ]]; then
  exit 1
fi
exit 0