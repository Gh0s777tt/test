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

if [[ -f "store/Cargo.toml" ]]; then
  if [[ -f "store/manifest.schema.json" ]]; then
    pass "store manifest schema exists"
    if python3 - <<'PY' >/dev/null 2>&1
import json
from pathlib import Path
json.loads(Path("store/manifest.schema.json").read_text(encoding="utf-8"))
PY
    then
      pass "store manifest schema is valid JSON"
    else
      fail "store manifest schema is invalid JSON"
    fi
  else
    fail "store manifest schema missing"
  fi
fi

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

if [[ -x "scripts/check_monitor_threshold_governance.sh" ]]; then
  if ./scripts/check_monitor_threshold_governance.sh >/dev/null; then
    pass "monitor-threshold governance check passed or skipped"
  else
    fail "monitor-threshold governance check failed"
  fi
else
  warn "scripts/check_monitor_threshold_governance.sh is missing or not executable"
fi

if [[ -x "scripts/validate_monpol_signoff_metadata.sh" ]]; then
  if ./scripts/validate_monpol_signoff_metadata.sh >/dev/null; then
    pass "MONPOL signoff metadata validation passed"
  else
    fail "MONPOL signoff metadata validation failed"
  fi
else
  warn "scripts/validate_monpol_signoff_metadata.sh is missing or not executable"
fi

if [[ -x "scripts/evaluate_monitor_drift_escalation.sh" ]]; then
  shopt -s nullglob
  DASHBOARD_CANDIDATES=(analysis/benchmark_reproducibility/monitor_policy_dashboard_*.json)
  shopt -u nullglob
  if (( ${#DASHBOARD_CANDIDATES[@]} == 0 )); then
    warn "monitor drift escalation check skipped (no dashboard artifacts)"
  else
    readarray -t SORTED_DASHBOARDS < <(printf '%s\n' "${DASHBOARD_CANDIDATES[@]}" | sort)
    LATEST_DASHBOARD="${SORTED_DASHBOARDS[${#SORTED_DASHBOARDS[@]}-1]}"
    TMP_ESCALATION_MD="$(mktemp /tmp/vantis_monitor_drift_escalation_verify_XXXXXX.md)"
    TMP_ESCALATION_JSON="$(mktemp /tmp/vantis_monitor_drift_escalation_verify_XXXXXX.json)"
    TMP_HANDOFF_MD=""
    TMP_HANDOFF_JSON=""
    TMP_DRILL_MD=""
    TMP_DRILL_JSON=""
    TMP_DRILL_SCENARIOS=""
    if ./scripts/evaluate_monitor_drift_escalation.sh --dashboard-json "$LATEST_DASHBOARD" --output "$TMP_ESCALATION_MD" --output-json "$TMP_ESCALATION_JSON" >/dev/null; then
      pass "monitor drift escalation evaluation passed"
    else
      fail "monitor drift escalation evaluation failed"
    fi

    if [[ -x "scripts/generate_monitor_drift_release_handoff.sh" ]]; then
      TMP_HANDOFF_MD="$(mktemp /tmp/vantis_monitor_drift_handoff_verify_XXXXXX.md)"
      TMP_HANDOFF_JSON="$(mktemp /tmp/vantis_monitor_drift_handoff_verify_XXXXXX.json)"
      if ./scripts/generate_monitor_drift_release_handoff.sh --escalation-json "$TMP_ESCALATION_JSON" --output "$TMP_HANDOFF_MD" --output-json "$TMP_HANDOFF_JSON" >/dev/null; then
        pass "monitor drift release handoff generation passed"
      else
        fail "monitor drift release handoff generation failed"
      fi
    else
      warn "scripts/generate_monitor_drift_release_handoff.sh is missing or not executable"
    fi

    if [[ -x "scripts/run_monitor_drift_release_readiness_drill.sh" ]]; then
      TMP_DRILL_MD="$(mktemp /tmp/vantis_release_readiness_drill_verify_XXXXXX.md)"
      TMP_DRILL_JSON="$(mktemp /tmp/vantis_release_readiness_drill_verify_XXXXXX.json)"
      TMP_DRILL_SCENARIOS="$(mktemp -d /tmp/vantis_release_readiness_drill_scenarios_XXXXXX)"
      if ./scripts/run_monitor_drift_release_readiness_drill.sh --escalation-json "$TMP_ESCALATION_JSON" --output "$TMP_DRILL_MD" --output-json "$TMP_DRILL_JSON" --scenario-output-dir "$TMP_DRILL_SCENARIOS" >/dev/null; then
        pass "monitor drift release-readiness drill passed"
      else
        fail "monitor drift release-readiness drill failed"
      fi
    else
      warn "scripts/run_monitor_drift_release_readiness_drill.sh is missing or not executable"
    fi

    if [[ -x "scripts/route_monitor_drift_breach_evidence.sh" ]]; then
      TMP_BREACH_ROUTE_MD="$(mktemp /tmp/vantis_breach_route_verify_XXXXXX.md)"
      TMP_BREACH_ROUTE_JSON="$(mktemp /tmp/vantis_breach_route_verify_XXXXXX.json)"
      if [[ -n "$TMP_HANDOFF_JSON" && -f "$TMP_HANDOFF_JSON" && -n "$TMP_DRILL_JSON" && -f "$TMP_DRILL_JSON" ]]; then
        if ./scripts/route_monitor_drift_breach_evidence.sh --escalation-json "$TMP_ESCALATION_JSON" --handoff-json "$TMP_HANDOFF_JSON" --drill-json "$TMP_DRILL_JSON" --output "$TMP_BREACH_ROUTE_MD" --output-json "$TMP_BREACH_ROUTE_JSON" >/dev/null; then
          pass "monitor drift breach route generation passed"
        else
          fail "monitor drift breach route generation failed"
        fi
      else
        warn "monitor drift breach route generation skipped (handoff/drill prerequisites unavailable)"
      fi
      rm -f "$TMP_BREACH_ROUTE_MD" "$TMP_BREACH_ROUTE_JSON"
    else
      warn "scripts/route_monitor_drift_breach_evidence.sh is missing or not executable"
    fi

    rm -f "$TMP_HANDOFF_MD" "$TMP_HANDOFF_JSON"
    rm -f "$TMP_DRILL_MD" "$TMP_DRILL_JSON"
    if [[ -n "$TMP_DRILL_SCENARIOS" ]]; then
      rm -rf "$TMP_DRILL_SCENARIOS"
    fi
    rm -f "$TMP_ESCALATION_MD" "$TMP_ESCALATION_JSON"
  fi
else
  warn "scripts/evaluate_monitor_drift_escalation.sh is missing or not executable"
fi

if [[ -f "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md" ]]; then
  pass "monitor threshold changelog exists"
else
  fail "monitor threshold changelog missing"
fi

if [[ -f "governance/performance/MONPOL_SIGNOFFS.json" ]]; then
  pass "MONPOL signoff metadata registry exists"
else
  fail "MONPOL signoff metadata registry missing"
fi

if [[ -f "governance/performance/MONITOR_DRIFT_ESCALATION_POLICY.md" ]]; then
  pass "monitor drift escalation policy doc exists"
else
  fail "monitor drift escalation policy doc missing"
fi

if [[ -f "governance/performance/MONITOR_DRIFT_ESCALATION_OWNERS.json" ]]; then
  pass "monitor drift escalation owners registry exists"
else
  fail "monitor drift escalation owners registry missing"
fi

if [[ -f "governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.md" ]]; then
  pass "monitor threshold governance gate promotion doc exists"
else
  fail "monitor threshold governance gate promotion doc missing"
fi

if [[ -f "governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json" ]]; then
  pass "monitor threshold governance gate promotion JSON exists"
else
  fail "monitor threshold governance gate promotion JSON missing"
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