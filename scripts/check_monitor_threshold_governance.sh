#!/usr/bin/env bash
# Enforce governance requirements for benchmark monitor threshold policy changes.
#
# Trigger conditions (pull_request context):
# - threshold-affecting changes in:
#   - .github/workflows/ci.yml
#   - scripts/run_benchmark_ci_gate.sh
#   - scripts/recommend_monitor_policy.sh
#
# Enforcement when triggered:
# - PR title/body includes MONPOL-<NNN>
# - governance/performance/MONITOR_THRESHOLD_CHANGELOG.md is part of changed files
# - docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md is part of changed files
# - optional promotion-aware breach requirements when mode is enforced:
#   - BREACH-ACK token and mitigation notes in PR metadata/body
#   - incident-closure handoff signoff packet presence for rollback-triggered paths

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
PROMOTION_POLICY_JSON="$ROOT/governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json"
BREACH_ROUTE_JSON=""
PILOT_RUNBOOK_JSON=""
BURN_IN_SLO_JSON=""
ROLLBACK_POSTMORTEM_JSON=""
HANDOFF_SIGNOFF_PACKET_JSON=""
CLOSURE_AUDIT_JSON=""
PROMOTION_MODE="auto"

usage() {
  cat <<'USAGE'
Usage: ./scripts/check_monitor_threshold_governance.sh [options]

Options:
  --analysis-dir <path>             Analysis directory (default: analysis/benchmark_reproducibility)
  --promotion-policy-json <path>    Governance gate promotion policy JSON path
  --breach-route-json <path>        Breach route JSON artifact path (default: newest monitor_drift_breach_route_*.json)
  --pilot-runbook-json <path>       Enforced pilot runbook JSON artifact path (default: newest enforced_pilot_runbook_*.json)
  --burn-in-slo-json <path>         Burn-in SLO JSON artifact path (default: newest enforced_pilot_burn_in_slo_*.json)
  --rollback-postmortem-json <path> Rollback postmortem JSON artifact path (default: newest enforced_pilot_rollback_postmortem_*.json)
  --handoff-signoff-packet-json <path>
                                   Handoff signoff packet JSON artifact path (default: newest enforced_pilot_handoff_signoff_packet_*.json)
  --closure-audit-json <path>      Closure audit JSON artifact path (default: newest enforced_pilot_closure_audit_*.json)
  --promotion-mode <auto|advisory|enforced>
  -h, --help                        Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --promotion-policy-json)
      PROMOTION_POLICY_JSON="${2:-}"
      shift 2
      ;;
    --breach-route-json)
      BREACH_ROUTE_JSON="${2:-}"
      shift 2
      ;;
    --pilot-runbook-json)
      PILOT_RUNBOOK_JSON="${2:-}"
      shift 2
      ;;
    --burn-in-slo-json)
      BURN_IN_SLO_JSON="${2:-}"
      shift 2
      ;;
    --rollback-postmortem-json)
      ROLLBACK_POSTMORTEM_JSON="${2:-}"
      shift 2
      ;;
    --handoff-signoff-packet-json)
      HANDOFF_SIGNOFF_PACKET_JSON="${2:-}"
      shift 2
      ;;
    --closure-audit-json)
      CLOSURE_AUDIT_JSON="${2:-}"
      shift 2
      ;;
    --promotion-mode)
      PROMOTION_MODE="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "ERR Unknown option: $1" >&2
      usage
      exit 1
      ;;
  esac
done

case "$PROMOTION_MODE" in
  auto|advisory|enforced) ;;
  *)
    echo "ERR --promotion-mode must be one of auto|advisory|enforced" >&2
    exit 1
    ;;
esac

pass() { echo "OK  $1"; }
info() { echo "INFO $1"; }
fail() { echo "ERR $1"; exit 1; }

EVENT_PATH="${GITHUB_EVENT_PATH:-}"
if [[ -z "$EVENT_PATH" || ! -f "$EVENT_PATH" ]]; then
  info "No pull_request event context found. Skipping monitor-threshold governance gate."
  exit 0
fi

HAS_PR="$(python3 - "$EVENT_PATH" <<'PY'
import json
import sys
with open(sys.argv[1], "r", encoding="utf-8") as fh:
    event = json.load(fh)
print("yes" if "pull_request" in event else "no")
PY
)"

if [[ "$HAS_PR" != "yes" ]]; then
  info "Event payload is not a pull_request payload. Skipping."
  exit 0
fi

readarray -t PR_META < <(python3 - "$EVENT_PATH" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as fh:
    event = json.load(fh)

pr = event.get("pull_request", {})
base_sha = pr.get("base", {}).get("sha", "")
head_sha = pr.get("head", {}).get("sha", "")
title = pr.get("title", "")
body = pr.get("body", "") or ""

print(base_sha)
print(head_sha)
print(title.replace("\n", " "))
print(body.replace("\r", " ").replace("\n", " "))
PY
)

BASE_SHA="${PR_META[0]:-}"
HEAD_SHA="${PR_META[1]:-}"
PR_TITLE="${PR_META[2]:-}"
PR_BODY="${PR_META[3]:-}"

if [[ -z "$BASE_SHA" || -z "$HEAD_SHA" ]]; then
  fail "Missing PR base/head SHA in event payload."
fi

CHANGED_FILES="$(git diff --name-only "$BASE_SHA" "$HEAD_SHA")"
if [[ -z "$CHANGED_FILES" ]]; then
  info "No changed files in PR range. Nothing to enforce."
  exit 0
fi

POLICY_FILES_REGEX='^(\.github/workflows/ci\.yml|scripts/run_benchmark_ci_gate\.sh|scripts/recommend_monitor_policy\.sh)$'
if ! echo "$CHANGED_FILES" | rg -q "$POLICY_FILES_REGEX"; then
  info "No monitor-threshold policy files changed. Governance gate skipped."
  exit 0
fi

DIFF_CONTENT="$(git diff "$BASE_SHA" "$HEAD_SHA" -- \
  ".github/workflows/ci.yml" \
  "scripts/run_benchmark_ci_gate.sh" \
  "scripts/recommend_monitor_policy.sh")"

THRESHOLD_CHANGE_REGEX='^[+-].*(STRICT_THRESHOLD_PCT|MONITOR_THRESHOLD_PCT|MONITOR_THRESHOLD_OVERRIDES|--strict-threshold-pct|--monitor-threshold-pct|--monitor-bench [^ ]+:[0-9]|--headroom-pct|--floor-pct|--ceil-pct)'
if ! echo "$DIFF_CONTENT" | rg -q "$THRESHOLD_CHANGE_REGEX"; then
  info "Policy files changed, but no threshold-affecting lines changed. Governance gate skipped."
  exit 0
fi

pass "Detected threshold-affecting policy change"
THRESHOLD_CHANGE_DETECTED="yes"

REFERENCE_TEXT="${PR_TITLE}"$'\n'"${PR_BODY}"
if ! echo "$REFERENCE_TEXT" | rg -q 'MONPOL-[0-9]{3}'; then
  fail "Missing MONPOL-<NNN> reference in PR title/body for threshold-affecting policy change."
fi
POLICY_ID="$(echo "$REFERENCE_TEXT" | rg -o 'MONPOL-[0-9]{3}' | sort -u | awk 'NR==1{print $0}')"
pass "PR includes policy reference: ${POLICY_ID}"

if ! echo "$CHANGED_FILES" | rg -q '^governance/performance/MONITOR_THRESHOLD_CHANGELOG\.md$'; then
  fail "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md must be updated for threshold-affecting policy changes."
fi
pass "Threshold changelog file is included in changed files"

if ! echo "$CHANGED_FILES" | rg -q '^docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE\.md$'; then
  fail "docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md must be updated for threshold-affecting policy changes."
fi
pass "Reproducibility profile documentation is included in changed files"

if [[ ! -f "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md" ]]; then
  fail "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md does not exist at repository head."
fi

if ! rg -q "$POLICY_ID" "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md"; then
  fail "Policy reference ${POLICY_ID} not found in governance/performance/MONITOR_THRESHOLD_CHANGELOG.md."
fi
pass "Policy reference found in threshold changelog"

POLICY_DECISION="$(python3 - "$POLICY_ID" "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md" <<'PY'
import re
import sys
from pathlib import Path

policy_id = sys.argv[1]
path = Path(sys.argv[2])
text = path.read_text(encoding="utf-8")

header_re = re.compile(r"^###\s+(MONPOL-\d{3})\s+\([^)]+\)\s*$", re.MULTILINE)
headers = list(header_re.finditer(text))
block = ""
for idx, match in enumerate(headers):
    if match.group(1) != policy_id:
        continue
    start = match.start()
    end = headers[idx + 1].start() if idx + 1 < len(headers) else len(text)
    block = text[start:end]
    break

if not block:
    print("unknown")
    raise SystemExit(0)

decision_match = re.search(r"^- \*\*Decision\*\*:\s*(.+)$", block, re.MULTILINE)
if not decision_match:
    print("unknown")
    raise SystemExit(0)

value = decision_match.group(1).strip().lower()
if "approved" in value:
    print("approved")
elif "reject" in value:
    print("rejected")
elif "withdraw" in value:
    print("withdrawn")
elif "defer" in value:
    print("deferred")
elif "pending" in value:
    print("pending")
else:
    print("unknown")
PY
)"
info "Detected changelog decision for ${POLICY_ID}: ${POLICY_DECISION}"

if [[ "$POLICY_DECISION" == "approved" ]]; then
  if ! echo "$CHANGED_FILES" | rg -q '^governance/performance/MONPOL_SIGNOFFS\.json$'; then
    fail "Approved policy decision requires governance/performance/MONPOL_SIGNOFFS.json update in this PR."
  fi
  pass "Signoff metadata registry is included in changed files"

  if [[ ! -x "scripts/validate_monpol_signoff_metadata.sh" ]]; then
    fail "scripts/validate_monpol_signoff_metadata.sh is required for approved policy decisions."
  fi
  if ./scripts/validate_monpol_signoff_metadata.sh >/dev/null; then
    pass "MONPOL signoff metadata validation passed for approved decision"
  else
    fail "MONPOL signoff metadata validation failed for approved decision"
  fi
fi

readarray -t PROMOTION_META < <(python3 - "$PROMOTION_POLICY_JSON" "$PROMOTION_MODE" <<'PY'
import json
import sys
from pathlib import Path

policy_path = Path(sys.argv[1])
mode_arg = sys.argv[2]

payload = {}
policy_exists = policy_path.exists()
if policy_exists:
    try:
        payload = json.loads(policy_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        payload = {}

policy_mode = str(payload.get("active_mode", "advisory"))
if policy_mode not in {"advisory", "enforced"}:
    policy_mode = "advisory"

active_mode = policy_mode if mode_arg == "auto" else mode_arg
if active_mode not in {"advisory", "enforced"}:
    active_mode = "advisory"

modes = payload.get("modes", {})
if not isinstance(modes, dict):
    modes = {}
cfg = modes.get(active_mode, {})
if not isinstance(cfg, dict):
    cfg = {}

enforce_on_breach = bool(cfg.get("enforce_on_breach", active_mode == "enforced"))
require_pr_mitigation = bool(cfg.get("require_pr_mitigation", active_mode == "enforced"))
require_breach_ack = bool(cfg.get("require_breach_ack_token", active_mode == "enforced"))

print(active_mode)
print("yes" if enforce_on_breach else "no")
print("yes" if require_pr_mitigation else "no")
print("yes" if require_breach_ack else "no")
print(policy_mode)
print("yes" if policy_exists else "no")
PY
)

ACTIVE_PROMOTION_MODE="${PROMOTION_META[0]:-advisory}"
ENFORCE_ON_BREACH="${PROMOTION_META[1]:-no}"
REQUIRE_PR_MITIGATION="${PROMOTION_META[2]:-no}"
REQUIRE_BREACH_ACK="${PROMOTION_META[3]:-no}"
POLICY_MODE_FROM_FILE="${PROMOTION_META[4]:-advisory}"
PROMOTION_POLICY_EXISTS="${PROMOTION_META[5]:-no}"

info "Promotion policy mode: active=${ACTIVE_PROMOTION_MODE}, policy_file=${POLICY_MODE_FROM_FILE}, policy_exists=${PROMOTION_POLICY_EXISTS}"
info "Promotion controls: enforce_on_breach=${ENFORCE_ON_BREACH}, require_pr_mitigation=${REQUIRE_PR_MITIGATION}, require_breach_ack=${REQUIRE_BREACH_ACK}"

if [[ -z "$BREACH_ROUTE_JSON" ]]; then
  shopt -s nullglob
  ROUTE_CANDIDATES=("$ANALYSIS_DIR"/monitor_drift_breach_route_[0-9]*.json)
  shopt -u nullglob
  if (( ${#ROUTE_CANDIDATES[@]} > 0 )); then
    readarray -t SORTED_ROUTE_CANDIDATES < <(printf '%s\n' "${ROUTE_CANDIDATES[@]}" | sort)
    BREACH_ROUTE_JSON="${SORTED_ROUTE_CANDIDATES[${#SORTED_ROUTE_CANDIDATES[@]}-1]}"
  fi
fi

if [[ -z "$PILOT_RUNBOOK_JSON" ]]; then
  shopt -s nullglob
  RUNBOOK_CANDIDATES=("$ANALYSIS_DIR"/enforced_pilot_runbook_[0-9]*.json)
  shopt -u nullglob
  if (( ${#RUNBOOK_CANDIDATES[@]} > 0 )); then
    readarray -t SORTED_RUNBOOK_CANDIDATES < <(printf '%s\n' "${RUNBOOK_CANDIDATES[@]}" | sort)
    PILOT_RUNBOOK_JSON="${SORTED_RUNBOOK_CANDIDATES[${#SORTED_RUNBOOK_CANDIDATES[@]}-1]}"
  fi
fi

if [[ -z "$BURN_IN_SLO_JSON" ]]; then
  shopt -s nullglob
  BURN_IN_CANDIDATES=("$ANALYSIS_DIR"/enforced_pilot_burn_in_slo_[0-9]*.json)
  shopt -u nullglob
  if (( ${#BURN_IN_CANDIDATES[@]} > 0 )); then
    readarray -t SORTED_BURN_IN_CANDIDATES < <(printf '%s\n' "${BURN_IN_CANDIDATES[@]}" | sort)
    BURN_IN_SLO_JSON="${SORTED_BURN_IN_CANDIDATES[${#SORTED_BURN_IN_CANDIDATES[@]}-1]}"
  fi
fi

if [[ -z "$ROLLBACK_POSTMORTEM_JSON" ]]; then
  shopt -s nullglob
  POSTMORTEM_CANDIDATES=("$ANALYSIS_DIR"/enforced_pilot_rollback_postmortem_[0-9]*.json)
  shopt -u nullglob
  if (( ${#POSTMORTEM_CANDIDATES[@]} > 0 )); then
    readarray -t SORTED_POSTMORTEM_CANDIDATES < <(printf '%s\n' "${POSTMORTEM_CANDIDATES[@]}" | sort)
    ROLLBACK_POSTMORTEM_JSON="${SORTED_POSTMORTEM_CANDIDATES[${#SORTED_POSTMORTEM_CANDIDATES[@]}-1]}"
  fi
fi

if [[ -z "$HANDOFF_SIGNOFF_PACKET_JSON" ]]; then
  shopt -s nullglob
  HANDOFF_SIGNOFF_CANDIDATES=("$ANALYSIS_DIR"/enforced_pilot_handoff_signoff_packet_[0-9]*.json)
  shopt -u nullglob
  if (( ${#HANDOFF_SIGNOFF_CANDIDATES[@]} > 0 )); then
    readarray -t SORTED_HANDOFF_SIGNOFF_CANDIDATES < <(printf '%s\n' "${HANDOFF_SIGNOFF_CANDIDATES[@]}" | sort)
    HANDOFF_SIGNOFF_PACKET_JSON="${SORTED_HANDOFF_SIGNOFF_CANDIDATES[${#SORTED_HANDOFF_SIGNOFF_CANDIDATES[@]}-1]}"
  fi
fi

if [[ -z "$CLOSURE_AUDIT_JSON" ]]; then
  shopt -s nullglob
  CLOSURE_AUDIT_CANDIDATES=("$ANALYSIS_DIR"/enforced_pilot_closure_audit_[0-9]*.json)
  shopt -u nullglob
  if (( ${#CLOSURE_AUDIT_CANDIDATES[@]} > 0 )); then
    readarray -t SORTED_CLOSURE_AUDIT_CANDIDATES < <(printf '%s\n' "${CLOSURE_AUDIT_CANDIDATES[@]}" | sort)
    CLOSURE_AUDIT_JSON="${SORTED_CLOSURE_AUDIT_CANDIDATES[${#SORTED_CLOSURE_AUDIT_CANDIDATES[@]}-1]}"
  fi
fi

BREACH_DETECTED="no"
BREACH_SOURCES="none"
ROUTE_WOULD_BLOCK="no"
ROUTE_MODE="n/a"
if [[ -n "$BREACH_ROUTE_JSON" && -f "$BREACH_ROUTE_JSON" ]]; then
  readarray -t ROUTE_META < <(python3 - "$BREACH_ROUTE_JSON" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
payload = json.loads(path.read_text(encoding="utf-8"))

breach = bool(payload.get("breach_detected", False))
sources = payload.get("breach_sources", [])
if not isinstance(sources, list):
    sources = []
promotion = payload.get("promotion", {})
if not isinstance(promotion, dict):
    promotion = {}

print("yes" if breach else "no")
print(",".join(str(item) for item in sources) if sources else "none")
print("yes" if bool(promotion.get("would_block_in_active_mode", False)) else "no")
print(str(promotion.get("active_mode", "n/a")))
PY
  )
  BREACH_DETECTED="${ROUTE_META[0]:-no}"
  BREACH_SOURCES="${ROUTE_META[1]:-none}"
  ROUTE_WOULD_BLOCK="${ROUTE_META[2]:-no}"
  ROUTE_MODE="${ROUTE_META[3]:-n/a}"
  info "Breach route artifact: ${BREACH_ROUTE_JSON}"
  info "Breach route summary: breach=${BREACH_DETECTED}, sources=${BREACH_SOURCES}, route_mode=${ROUTE_MODE}, route_would_block=${ROUTE_WOULD_BLOCK}"
else
  info "Breach route artifact not available for promotion-aware checks."
fi

RUNBOOK_ROLLBACK_RECOMMENDED="no"
RUNBOOK_RECOMMENDED_ACTION="n/a"
RUNBOOK_STAGE="n/a"
if [[ -n "$PILOT_RUNBOOK_JSON" && -f "$PILOT_RUNBOOK_JSON" ]]; then
  readarray -t RUNBOOK_META < <(python3 - "$PILOT_RUNBOOK_JSON" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
payload = json.loads(path.read_text(encoding="utf-8"))

print("yes" if bool(payload.get("rollback_recommended", False)) else "no")
print(str(payload.get("recommended_action", "n/a")))
print(str(payload.get("runbook_stage", "n/a")))
PY
  )
  RUNBOOK_ROLLBACK_RECOMMENDED="${RUNBOOK_META[0]:-no}"
  RUNBOOK_RECOMMENDED_ACTION="${RUNBOOK_META[1]:-n/a}"
  RUNBOOK_STAGE="${RUNBOOK_META[2]:-n/a}"
  info "Enforced pilot runbook artifact: ${PILOT_RUNBOOK_JSON}"
  info "Runbook summary: stage=${RUNBOOK_STAGE}, action=${RUNBOOK_RECOMMENDED_ACTION}, rollback=${RUNBOOK_ROLLBACK_RECOMMENDED}"
else
  info "Enforced pilot runbook artifact not available for rollback-guardrail checks."
fi

BURN_IN_SLO_STATUS="n/a"
BURN_IN_SLO_FAILS="0"
if [[ -n "$BURN_IN_SLO_JSON" && -f "$BURN_IN_SLO_JSON" ]]; then
  readarray -t BURN_IN_META < <(python3 - "$BURN_IN_SLO_JSON" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
payload = json.loads(path.read_text(encoding="utf-8"))
fails = payload.get("required_failures", [])
if not isinstance(fails, list):
    fails = []

print(str(payload.get("overall_status", "n/a")))
print(str(len(fails)))
PY
  )
  BURN_IN_SLO_STATUS="${BURN_IN_META[0]:-n/a}"
  BURN_IN_SLO_FAILS="${BURN_IN_META[1]:-0}"
  info "Burn-in SLO artifact: ${BURN_IN_SLO_JSON}"
  info "Burn-in SLO summary: status=${BURN_IN_SLO_STATUS}, failed_criteria=${BURN_IN_SLO_FAILS}"
else
  info "Burn-in SLO artifact not available for enforced pilot checks."
fi

POSTMORTEM_REQUIRED="no"
POSTMORTEM_STATUS="n/a"
if [[ -n "$ROLLBACK_POSTMORTEM_JSON" && -f "$ROLLBACK_POSTMORTEM_JSON" ]]; then
  readarray -t POSTMORTEM_META < <(python3 - "$ROLLBACK_POSTMORTEM_JSON" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
payload = json.loads(path.read_text(encoding="utf-8"))
print("yes" if bool(payload.get("required", False)) else "no")
print(str(payload.get("status", "n/a")))
PY
  )
  POSTMORTEM_REQUIRED="${POSTMORTEM_META[0]:-no}"
  POSTMORTEM_STATUS="${POSTMORTEM_META[1]:-n/a}"
  info "Rollback postmortem artifact: ${ROLLBACK_POSTMORTEM_JSON}"
  info "Rollback postmortem summary: required=${POSTMORTEM_REQUIRED}, status=${POSTMORTEM_STATUS}"
else
  info "Rollback postmortem artifact not available for enforced pilot checks."
fi

HANDOFF_PACKET_REQUIRED="no"
HANDOFF_PACKET_STATUS="n/a"
HANDOFF_PACKET_READY="no"
HANDOFF_PACKET_ACTION="n/a"
if [[ -n "$HANDOFF_SIGNOFF_PACKET_JSON" && -f "$HANDOFF_SIGNOFF_PACKET_JSON" ]]; then
  readarray -t HANDOFF_PACKET_META < <(python3 - "$HANDOFF_SIGNOFF_PACKET_JSON" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
payload = json.loads(path.read_text(encoding="utf-8"))
closure = payload.get("closure", {})
if not isinstance(closure, dict):
    closure = {}

print("yes" if bool(closure.get("closure_required", False)) else "no")
print(str(closure.get("packet_status", "n/a")))
print("yes" if bool(closure.get("overall_ready", False)) else "no")
print(str(closure.get("recommended_action", "n/a")))
PY
  )
  HANDOFF_PACKET_REQUIRED="${HANDOFF_PACKET_META[0]:-no}"
  HANDOFF_PACKET_STATUS="${HANDOFF_PACKET_META[1]:-n/a}"
  HANDOFF_PACKET_READY="${HANDOFF_PACKET_META[2]:-no}"
  HANDOFF_PACKET_ACTION="${HANDOFF_PACKET_META[3]:-n/a}"
  info "Handoff signoff packet artifact: ${HANDOFF_SIGNOFF_PACKET_JSON}"
  info "Handoff signoff summary: required=${HANDOFF_PACKET_REQUIRED}, status=${HANDOFF_PACKET_STATUS}, ready=${HANDOFF_PACKET_READY}, action=${HANDOFF_PACKET_ACTION}"
else
  info "Handoff signoff packet artifact not available for incident-closure checks."
fi

CLOSURE_AUDIT_STATUS="n/a"
CLOSURE_AUDIT_GATE_STATE="n/a"
CLOSURE_AUDIT_REQUIRED_FAILS="0"
CLOSURE_AUDIT_PACKET_REQUIRED="no"
CLOSURE_AUDIT_PACKET_READY="no"
if [[ -n "$CLOSURE_AUDIT_JSON" && -f "$CLOSURE_AUDIT_JSON" ]]; then
  readarray -t CLOSURE_AUDIT_META < <(python3 - "$CLOSURE_AUDIT_JSON" <<'PY'
import json
import sys
from pathlib import Path

path = Path(sys.argv[1])
payload = json.loads(path.read_text(encoding="utf-8"))
snapshot = payload.get("incident_closure_snapshot", {})
if not isinstance(snapshot, dict):
    snapshot = {}
fails = payload.get("required_failures", [])
if not isinstance(fails, list):
    fails = []

print(str(payload.get("overall_status", "n/a")))
print(str(payload.get("closure_gate_state", "n/a")))
print(str(len(fails)))
print("yes" if bool(snapshot.get("packet_closure_required", False)) else "no")
print("yes" if bool(snapshot.get("packet_overall_ready", False)) else "no")
PY
  )
  CLOSURE_AUDIT_STATUS="${CLOSURE_AUDIT_META[0]:-n/a}"
  CLOSURE_AUDIT_GATE_STATE="${CLOSURE_AUDIT_META[1]:-n/a}"
  CLOSURE_AUDIT_REQUIRED_FAILS="${CLOSURE_AUDIT_META[2]:-0}"
  CLOSURE_AUDIT_PACKET_REQUIRED="${CLOSURE_AUDIT_META[3]:-no}"
  CLOSURE_AUDIT_PACKET_READY="${CLOSURE_AUDIT_META[4]:-no}"
  info "Closure audit artifact: ${CLOSURE_AUDIT_JSON}"
  info "Closure audit summary: status=${CLOSURE_AUDIT_STATUS}, gate=${CLOSURE_AUDIT_GATE_STATE}, required_failures=${CLOSURE_AUDIT_REQUIRED_FAILS}"
else
  info "Closure audit artifact not available for incident-closure audit checks."
fi

if [[ "$ACTIVE_PROMOTION_MODE" == "enforced" ]]; then
  if [[ "$ENFORCE_ON_BREACH" == "yes" ]]; then
    if [[ -z "$BREACH_ROUTE_JSON" || ! -f "$BREACH_ROUTE_JSON" ]]; then
      fail "Enforced promotion mode requires a breach route artifact (monitor_drift_breach_route_*.json)."
    fi
    if [[ "$BREACH_DETECTED" == "yes" ]]; then
      if [[ "$REQUIRE_BREACH_ACK" == "yes" ]]; then
        if ! echo "$REFERENCE_TEXT" | rg -q 'BREACH-ACK'; then
          fail "Breach detected under enforced mode: PR title/body must include BREACH-ACK."
        fi
        pass "PR includes BREACH-ACK token for breach-aware enforced mode"
      fi
      if [[ "$REQUIRE_PR_MITIGATION" == "yes" ]]; then
        if ! echo "$PR_BODY" | rg -i -q 'mitigation'; then
          fail "Breach detected under enforced mode: PR body must include mitigation plan details."
        fi
        pass "PR body includes mitigation details for breach-aware enforced mode"
      fi
    else
      info "No active breach detected; enforced promotion controls satisfied."
    fi
  fi
  if [[ "$BURN_IN_SLO_STATUS" == "fail" ]]; then
    fail "Enforced pilot burn-in SLO failed; rollback or mitigation required before continuing enforced mode."
  fi
  if [[ "$RUNBOOK_ROLLBACK_RECOMMENDED" == "yes" ]]; then
    if [[ -z "$HANDOFF_SIGNOFF_PACKET_JSON" || ! -f "$HANDOFF_SIGNOFF_PACKET_JSON" ]]; then
      fail "Rollback recommended under enforced mode: handoff signoff packet artifact is required."
    fi
    if [[ "$HANDOFF_PACKET_REQUIRED" != "yes" ]]; then
      fail "Rollback recommended but handoff signoff packet is not marked as required."
    fi
    if [[ -n "$ROLLBACK_POSTMORTEM_JSON" && -f "$ROLLBACK_POSTMORTEM_JSON" ]]; then
      if [[ "$POSTMORTEM_REQUIRED" != "yes" ]]; then
        fail "Rollback recommended but rollback postmortem artifact is not marked required."
      fi
    else
      fail "Rollback recommended under enforced mode: rollback postmortem artifact is required."
    fi
    fail "Enforced pilot rollback guardrail triggered (runbook action: ${RUNBOOK_RECOMMENDED_ACTION})."
  fi
  if [[ "$HANDOFF_PACKET_REQUIRED" == "yes" && "$HANDOFF_PACKET_READY" != "yes" ]]; then
    fail "Incident closure handoff signoff packet is required but not ready (${HANDOFF_PACKET_STATUS})."
  fi
  if [[ "$HANDOFF_PACKET_REQUIRED" == "yes" ]]; then
    if [[ -z "$CLOSURE_AUDIT_JSON" || ! -f "$CLOSURE_AUDIT_JSON" ]]; then
      fail "Incident closure requires closure audit artifact for enforced mode."
    fi
    if [[ "$CLOSURE_AUDIT_PACKET_REQUIRED" != "yes" ]]; then
      fail "Closure audit does not mark packet closure requirement as expected."
    fi
    if [[ "$CLOSURE_AUDIT_PACKET_READY" != "yes" ]]; then
      fail "Closure audit reports packet is not ready for required closure path."
    fi
    if [[ "$CLOSURE_AUDIT_STATUS" != "pass" ]]; then
      fail "Closure audit failed (${CLOSURE_AUDIT_GATE_STATE}); resolve required failures before closure."
    fi
  fi
fi

echo "Monitor threshold governance gate passed."
