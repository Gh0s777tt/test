#!/usr/bin/env bash
# Generate enforced-pilot execution runbook with rollback guardrails.
# Usage:
#   ./scripts/generate_enforced_pilot_runbook.sh
#   ./scripts/generate_enforced_pilot_runbook.sh --pilot-mode enforced --fail-on-rollback

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
PROMOTION_POLICY_JSON="$ROOT/governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json"
READINESS_JSON=""
BREACH_ROUTE_JSON=""
HANDOFF_JSON=""
DRILL_JSON=""
PILOT_MODE="auto"
PILOT_ACK_TOKEN=""
FAIL_ON_ROLLBACK=0
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/generate_enforced_pilot_runbook.sh [options]

Options:
  --analysis-dir <path>            Analysis directory (default: analysis/benchmark_reproducibility)
  --promotion-policy-json <path>   Governance gate promotion policy JSON path
  --readiness-json <path>          Promotion readiness JSON artifact
  --breach-route-json <path>       Breach route JSON artifact
  --handoff-json <path>            Release handoff JSON artifact
  --drill-json <path>              Release readiness drill JSON artifact
  --pilot-mode <auto|advisory|enforced>
  --pilot-ack-token <token>        Optional pilot operator acknowledgment token
  --fail-on-rollback               Exit non-zero when rollback is recommended
  --output <path>                  Output markdown path
  --output-json <path>             Output JSON path
  -h, --help                       Show this help
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
    --readiness-json)
      READINESS_JSON="${2:-}"
      shift 2
      ;;
    --breach-route-json)
      BREACH_ROUTE_JSON="${2:-}"
      shift 2
      ;;
    --handoff-json)
      HANDOFF_JSON="${2:-}"
      shift 2
      ;;
    --drill-json)
      DRILL_JSON="${2:-}"
      shift 2
      ;;
    --pilot-mode)
      PILOT_MODE="${2:-}"
      shift 2
      ;;
    --pilot-ack-token)
      PILOT_ACK_TOKEN="${2:-}"
      shift 2
      ;;
    --fail-on-rollback)
      FAIL_ON_ROLLBACK=1
      shift
      ;;
    --output)
      OUTPUT_PATH="${2:-}"
      shift 2
      ;;
    --output-json)
      OUTPUT_JSON_PATH="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage
      exit 1
      ;;
  esac
done

if [[ ! -d "$ANALYSIS_DIR" ]]; then
  echo "Error: analysis directory not found: $ANALYSIS_DIR" >&2
  exit 1
fi

if [[ ! -f "$PROMOTION_POLICY_JSON" ]]; then
  echo "Error: promotion policy JSON not found: $PROMOTION_POLICY_JSON" >&2
  exit 1
fi

case "$PILOT_MODE" in
  auto|advisory|enforced) ;;
  *)
    echo "Error: --pilot-mode must be one of auto|advisory|enforced" >&2
    exit 1
    ;;
esac

if [[ -n "$READINESS_JSON" && ! -f "$READINESS_JSON" ]]; then
  echo "Error: readiness JSON not found: $READINESS_JSON" >&2
  exit 1
fi

if [[ -n "$BREACH_ROUTE_JSON" && ! -f "$BREACH_ROUTE_JSON" ]]; then
  echo "Error: breach route JSON not found: $BREACH_ROUTE_JSON" >&2
  exit 1
fi

if [[ -n "$HANDOFF_JSON" && ! -f "$HANDOFF_JSON" ]]; then
  echo "Error: handoff JSON not found: $HANDOFF_JSON" >&2
  exit 1
fi

if [[ -n "$DRILL_JSON" && ! -f "$DRILL_JSON" ]]; then
  echo "Error: drill JSON not found: $DRILL_JSON" >&2
  exit 1
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/enforced_pilot_runbook_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - \
  "$ROOT" \
  "$ANALYSIS_DIR" \
  "$PROMOTION_POLICY_JSON" \
  "$READINESS_JSON" \
  "$BREACH_ROUTE_JSON" \
  "$HANDOFF_JSON" \
  "$DRILL_JSON" \
  "$PILOT_MODE" \
  "$PILOT_ACK_TOKEN" \
  "$FAIL_ON_ROLLBACK" \
  "$OUTPUT_PATH" \
  "$OUTPUT_JSON_PATH" <<'PY'
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
analysis_dir = Path(sys.argv[2])
policy_path = Path(sys.argv[3])
readiness_path_raw = sys.argv[4].strip()
breach_path_raw = sys.argv[5].strip()
handoff_path_raw = sys.argv[6].strip()
drill_path_raw = sys.argv[7].strip()
pilot_mode_arg = sys.argv[8].strip()
pilot_ack_token = sys.argv[9].strip()
fail_on_rollback = bool(int(sys.argv[10]))
output_path = Path(sys.argv[11])
output_json_path = Path(sys.argv[12])


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(root.resolve()))
    except ValueError:
        return str(path)


def load_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def latest(glob: str):
    items = sorted(analysis_dir.glob(glob))
    return items[-1] if items else None


def yes_no(value: bool) -> str:
    return "yes" if value else "no"


def safe_int(value, default: int) -> int:
    try:
        return int(value)
    except (TypeError, ValueError):
        return default


def safe_bool(value, default: bool) -> bool:
    if isinstance(value, bool):
        return value
    if isinstance(value, (int, float)):
        return bool(value)
    if isinstance(value, str):
        normalized = value.strip().lower()
        if normalized in {"1", "true", "yes", "y", "on"}:
            return True
        if normalized in {"0", "false", "no", "n", "off"}:
            return False
    return default


def default_execution_stages():
    return [
        {
            "stage": "preflight",
            "description": "Validate readiness and evidence before enforced pilot.",
            "actions": [
                "Confirm readiness artifact is overall_ready and pilot_go_no_go=go.",
                "Confirm latest handoff is ready and latest drill status is pass.",
                "Confirm rollback guardrails are configured in policy JSON.",
            ],
        },
        {
            "stage": "enforced_pilot",
            "description": "Operate governance gate in enforced mode with active monitoring.",
            "actions": [
                "Run promotion-aware governance gate for threshold-affecting PRs.",
                "Track consecutive blocking breach/handoff/drill/not-ready indicators.",
                "Attach runbook JSON and transition-pack snapshot to governance evidence bundle.",
            ],
        },
        {
            "stage": "rollback",
            "description": "Rollback to advisory mode on guardrail breach.",
            "actions": [
                "Set active_mode=advisory in promotion policy JSON.",
                "Record rollback rationale in MONPOL changelog and transition pack.",
                "Open follow-up mitigation task for breached guardrail indicators.",
            ],
        },
    ]


def consecutive_true(paths, predicate):
    count = 0
    for path in reversed(paths):
        payload = load_json(path)
        if predicate(payload):
            count += 1
        else:
            break
    return count


policy_payload = load_json(policy_path)
pilot_cfg = policy_payload.get("pilot_rollout", {})
if not isinstance(pilot_cfg, dict):
    pilot_cfg = {}

active_mode_cfg = str(policy_payload.get("active_mode", "advisory"))
if active_mode_cfg not in {"advisory", "enforced"}:
    active_mode_cfg = "advisory"

active_mode = active_mode_cfg if pilot_mode_arg == "auto" else pilot_mode_arg
if active_mode not in {"advisory", "enforced"}:
    active_mode = "advisory"

execution_stages = pilot_cfg.get("execution_stages", [])
if not isinstance(execution_stages, list) or not execution_stages:
    execution_stages = default_execution_stages()

required_checks = pilot_cfg.get("required_checks", [])
if not isinstance(required_checks, list):
    required_checks = []

guardrails_cfg = pilot_cfg.get("rollback_guardrails", {})
if not isinstance(guardrails_cfg, dict):
    guardrails_cfg = {}

max_consecutive_blocking_breach_routes = max(
    0, safe_int(guardrails_cfg.get("max_consecutive_blocking_breach_routes"), 0)
)
max_consecutive_blocked_handoffs = max(
    0, safe_int(guardrails_cfg.get("max_consecutive_blocked_handoffs"), 0)
)
max_consecutive_failed_drills = max(
    0, safe_int(guardrails_cfg.get("max_consecutive_failed_drills"), 0)
)
max_consecutive_not_ready_assessments = max(
    0, safe_int(guardrails_cfg.get("max_consecutive_not_ready_assessments"), 0)
)
require_readiness_ready = safe_bool(guardrails_cfg.get("require_readiness_ready"), True)
require_pilot_go_signal = safe_bool(guardrails_cfg.get("require_pilot_go_signal"), True)
require_ops_ack_token = safe_bool(guardrails_cfg.get("require_ops_ack_token"), False)
auto_revert_to_advisory = safe_bool(
    guardrails_cfg.get("auto_revert_to_advisory_on_guardrail_breach"), True
)

expected_ack = str(pilot_cfg.get("ops_ack_token", "PILOT-ACK")).strip() or "PILOT-ACK"
ack_token_valid = bool(pilot_ack_token) and pilot_ack_token == expected_ack

readiness_path = Path(readiness_path_raw) if readiness_path_raw else latest("governance_gate_promotion_readiness_[0-9]*.json")
breach_path = Path(breach_path_raw) if breach_path_raw else latest("monitor_drift_breach_route_[0-9]*.json")
handoff_path = Path(handoff_path_raw) if handoff_path_raw else latest("monitor_drift_release_handoff_[0-9]*.json")
drill_path = Path(drill_path_raw) if drill_path_raw else latest("monitor_drift_release_readiness_drill_[0-9]*.json")

readiness_payload = load_json(readiness_path) if readiness_path and readiness_path.exists() else {}
breach_payload = load_json(breach_path) if breach_path and breach_path.exists() else {}
handoff_payload = load_json(handoff_path) if handoff_path and handoff_path.exists() else {}
drill_payload = load_json(drill_path) if drill_path and drill_path.exists() else {}

readiness_overall_ready = bool(readiness_payload.get("overall_ready", False))
readiness_go = str(readiness_payload.get("pilot_go_no_go", "no-go")) == "go"
readiness_recommended_mode = str(readiness_payload.get("recommended_mode", "n/a"))

handoff_status = str(handoff_payload.get("overall_status", "n/a"))
drill_status = str(drill_payload.get("overall_status", "n/a"))
handoff_ready = handoff_status == "ready"
drill_pass = drill_status == "pass"

promotion = breach_payload.get("promotion", {})
if not isinstance(promotion, dict):
    promotion = {}
breach_detected = bool(breach_payload.get("breach_detected", False))
breach_sources = breach_payload.get("breach_sources", [])
if not isinstance(breach_sources, list):
    breach_sources = []
latest_would_block = bool(promotion.get("would_block_in_active_mode", False))

readiness_check_status = {}
for item in readiness_payload.get("pilot_checklist", []):
    if isinstance(item, dict):
        check_id = str(item.get("id", "")).strip()
        if check_id:
            readiness_check_status[check_id] = str(item.get("status", "unknown"))

missing_required_checks = [
    check_id
    for check_id in required_checks
    if readiness_check_status.get(str(check_id), "missing") != "pass"
]

preflight_checks = [
    {
        "id": "readiness_artifact_present",
        "required": True,
        "status": "pass" if bool(readiness_payload) else "fail",
        "details": rel(readiness_path) if readiness_path else "n/a",
    },
    {
        "id": "required_readiness_checks_passed",
        "required": True,
        "status": "pass" if not missing_required_checks else "fail",
        "details": "all required checks passed" if not missing_required_checks else ",".join(missing_required_checks),
    },
    {
        "id": "readiness_ready",
        "required": require_readiness_ready,
        "status": "pass" if readiness_overall_ready else "fail",
        "details": f"overall_ready={yes_no(readiness_overall_ready)}",
    },
    {
        "id": "readiness_go_signal",
        "required": require_pilot_go_signal,
        "status": "pass" if readiness_go else "fail",
        "details": f"pilot_go_no_go={'go' if readiness_go else 'no-go'}",
    },
    {
        "id": "handoff_ready",
        "required": True,
        "status": "pass" if handoff_ready else "fail",
        "details": f"overall_status={handoff_status}",
    },
    {
        "id": "drill_pass",
        "required": True,
        "status": "pass" if drill_pass else "fail",
        "details": f"overall_status={drill_status}",
    },
    {
        "id": "ops_ack_token",
        "required": require_ops_ack_token,
        "status": "pass" if ack_token_valid else ("advisory" if not require_ops_ack_token else "fail"),
        "details": f"expected={expected_ack}, provided={'yes' if bool(pilot_ack_token) else 'no'}",
    },
]

preflight_required_failures = [
    item for item in preflight_checks if item["required"] and item["status"] == "fail"
]

breach_history = sorted(analysis_dir.glob("monitor_drift_breach_route_[0-9]*.json"))
handoff_history = sorted(analysis_dir.glob("monitor_drift_release_handoff_[0-9]*.json"))
drill_history = sorted(analysis_dir.glob("monitor_drift_release_readiness_drill_[0-9]*.json"))
readiness_history = sorted(analysis_dir.glob("governance_gate_promotion_readiness_[0-9]*.json"))

consecutive_blocking_breach_routes = consecutive_true(
    breach_history,
    lambda payload: bool(
        (payload.get("promotion", {}) if isinstance(payload.get("promotion", {}), dict) else {}).get(
            "would_block_in_active_mode", False
        )
    ),
)
consecutive_blocked_handoffs = consecutive_true(
    handoff_history,
    lambda payload: str(payload.get("overall_status", "n/a")) == "blocked",
)
consecutive_failed_drills = consecutive_true(
    drill_history,
    lambda payload: str(payload.get("overall_status", "n/a")) != "pass",
)
consecutive_not_ready_assessments = consecutive_true(
    readiness_history,
    lambda payload: not bool(payload.get("overall_ready", False))
    or str(payload.get("pilot_go_no_go", "no-go")) != "go",
)

guardrail_breaches = []
if consecutive_blocking_breach_routes > max_consecutive_blocking_breach_routes:
    guardrail_breaches.append(
        {
            "id": "consecutive_blocking_breach_routes",
            "value": consecutive_blocking_breach_routes,
            "threshold": max_consecutive_blocking_breach_routes,
            "description": "Consecutive blocking breach-route artifacts exceed configured guardrail.",
        }
    )
if consecutive_blocked_handoffs > max_consecutive_blocked_handoffs:
    guardrail_breaches.append(
        {
            "id": "consecutive_blocked_handoffs",
            "value": consecutive_blocked_handoffs,
            "threshold": max_consecutive_blocked_handoffs,
            "description": "Consecutive blocked handoff artifacts exceed configured guardrail.",
        }
    )
if consecutive_failed_drills > max_consecutive_failed_drills:
    guardrail_breaches.append(
        {
            "id": "consecutive_failed_drills",
            "value": consecutive_failed_drills,
            "threshold": max_consecutive_failed_drills,
            "description": "Consecutive failed drill artifacts exceed configured guardrail.",
        }
    )
if consecutive_not_ready_assessments > max_consecutive_not_ready_assessments:
    guardrail_breaches.append(
        {
            "id": "consecutive_not_ready_assessments",
            "value": consecutive_not_ready_assessments,
            "threshold": max_consecutive_not_ready_assessments,
            "description": "Consecutive not-ready assessments exceed configured guardrail.",
        }
    )

preflight_ok = len(preflight_required_failures) == 0
guardrails_ok = len(guardrail_breaches) == 0

rollback_recommended = active_mode == "enforced" and (not preflight_ok or not guardrails_ok)

if active_mode == "enforced":
    if rollback_recommended:
        recommended_action = "rollback_to_advisory" if auto_revert_to_advisory else "hold_enforced_and_triage"
        runbook_stage = "rollback"
    else:
        recommended_action = "continue_enforced_pilot"
        runbook_stage = "enforced_pilot"
else:
    if preflight_ok and guardrails_ok and readiness_recommended_mode == "enforced":
        recommended_action = "promote_to_enforced_pilot"
    else:
        recommended_action = "remain_advisory"
    runbook_stage = "preflight"

operator_decision = "go" if not rollback_recommended else "rollback"

selected_stage = None
for stage in execution_stages:
    if not isinstance(stage, dict):
        continue
    if str(stage.get("stage", "")).strip() == runbook_stage:
        selected_stage = stage
        break

if selected_stage is None:
    selected_stage = {"stage": runbook_stage, "description": "n/a", "actions": []}

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Enforced Pilot Execution Runbook\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Active mode**: `{active_mode}`  \n")
    fh.write(f"**Runbook stage**: `{runbook_stage}`  \n")
    fh.write(f"**Recommended action**: `{recommended_action}`  \n")
    fh.write(f"**Operator decision**: **{operator_decision}**  \n")
    fh.write(f"**Rollback recommended**: **{yes_no(rollback_recommended)}**\n\n")

    fh.write("## Inputs\n\n")
    fh.write(f"- Promotion policy JSON: `{rel(policy_path)}`\n")
    fh.write(f"- Readiness JSON: `{rel(readiness_path) if readiness_path else 'n/a'}`\n")
    fh.write(f"- Breach route JSON: `{rel(breach_path) if breach_path else 'n/a'}`\n")
    fh.write(f"- Handoff JSON: `{rel(handoff_path) if handoff_path else 'n/a'}`\n")
    fh.write(f"- Drill JSON: `{rel(drill_path) if drill_path else 'n/a'}`\n\n")

    fh.write("## Preflight Checklist\n\n")
    fh.write("| Item | Required | Status | Details |\n")
    fh.write("|---|---|---|---|\n")
    for item in preflight_checks:
        fh.write(
            f"| `{item['id']}` | {'yes' if item['required'] else 'no'} | {item['status']} | {item['details']} |\n"
        )
    fh.write("\n")

    if preflight_required_failures:
        fh.write("Blocking preflight failures:\n")
        for item in preflight_required_failures:
            fh.write(f"- `{item['id']}`: {item['details']}\n")
        fh.write("\n")

    fh.write("## Rollback Guardrails\n\n")
    fh.write("| Guardrail | Value | Threshold | Breached |\n")
    fh.write("|---|---:|---:|---|\n")
    guardrail_rows = [
        (
            "consecutive_blocking_breach_routes",
            consecutive_blocking_breach_routes,
            max_consecutive_blocking_breach_routes,
        ),
        ("consecutive_blocked_handoffs", consecutive_blocked_handoffs, max_consecutive_blocked_handoffs),
        ("consecutive_failed_drills", consecutive_failed_drills, max_consecutive_failed_drills),
        (
            "consecutive_not_ready_assessments",
            consecutive_not_ready_assessments,
            max_consecutive_not_ready_assessments,
        ),
    ]
    breached_ids = {item["id"] for item in guardrail_breaches}
    for guardrail_id, value, threshold in guardrail_rows:
        fh.write(f"| `{guardrail_id}` | {value} | {threshold} | {'yes' if guardrail_id in breached_ids else 'no'} |\n")
    fh.write("\n")

    fh.write(f"- Latest breach detected: `{yes_no(breach_detected)}`\n")
    fh.write(f"- Latest breach would block in active mode: `{yes_no(latest_would_block)}`\n")
    if breach_sources:
        fh.write(f"- Latest breach sources: {', '.join(breach_sources)}\n")
    else:
        fh.write("- Latest breach sources: none\n")
    fh.write("\n")

    if guardrail_breaches:
        fh.write("Guardrail breaches:\n")
        for breach in guardrail_breaches:
            fh.write(
                f"- `{breach['id']}` value={breach['value']} threshold={breach['threshold']}: {breach['description']}\n"
            )
        fh.write("\n")

    fh.write("## Stage Runbook Actions\n\n")
    fh.write(f"- Stage description: {selected_stage.get('description', 'n/a')}\n")
    actions = selected_stage.get("actions", [])
    if isinstance(actions, list) and actions:
        for action in actions:
            fh.write(f"- {action}\n")
    else:
        fh.write("- n/a\n")
    fh.write("\n")

    fh.write("## Decision Summary\n\n")
    fh.write(f"- Preflight OK: `{yes_no(preflight_ok)}`\n")
    fh.write(f"- Guardrails OK: `{yes_no(guardrails_ok)}`\n")
    fh.write(f"- Auto revert on breach: `{yes_no(auto_revert_to_advisory)}`\n")
    fh.write(f"- Recommended action: `{recommended_action}`\n")

payload = {
    "generated_at_utc": generated_at,
    "inputs": {
        "analysis_dir": rel(analysis_dir),
        "promotion_policy_json": rel(policy_path),
        "readiness_json": rel(readiness_path) if readiness_path else "n/a",
        "breach_route_json": rel(breach_path) if breach_path else "n/a",
        "handoff_json": rel(handoff_path) if handoff_path else "n/a",
        "drill_json": rel(drill_path) if drill_path else "n/a",
        "pilot_mode_arg": pilot_mode_arg,
    },
    "policy": {
        "active_mode_from_policy": active_mode_cfg,
        "active_mode_used": active_mode,
        "required_checks": required_checks,
        "expected_ops_ack_token": expected_ack,
        "guardrails": {
            "max_consecutive_blocking_breach_routes": max_consecutive_blocking_breach_routes,
            "max_consecutive_blocked_handoffs": max_consecutive_blocked_handoffs,
            "max_consecutive_failed_drills": max_consecutive_failed_drills,
            "max_consecutive_not_ready_assessments": max_consecutive_not_ready_assessments,
            "require_readiness_ready": require_readiness_ready,
            "require_pilot_go_signal": require_pilot_go_signal,
            "require_ops_ack_token": require_ops_ack_token,
            "auto_revert_to_advisory_on_guardrail_breach": auto_revert_to_advisory,
        },
    },
    "preflight_checks": preflight_checks,
    "preflight_required_failures": preflight_required_failures,
    "preflight_ok": preflight_ok,
    "telemetry_snapshot": {
        "readiness_overall_ready": readiness_overall_ready,
        "readiness_pilot_go": readiness_go,
        "readiness_recommended_mode": readiness_recommended_mode,
        "handoff_status": handoff_status,
        "drill_status": drill_status,
        "breach_detected": breach_detected,
        "breach_sources": breach_sources,
        "breach_would_block_in_active_mode": latest_would_block,
    },
    "consecutive_counters": {
        "consecutive_blocking_breach_routes": consecutive_blocking_breach_routes,
        "consecutive_blocked_handoffs": consecutive_blocked_handoffs,
        "consecutive_failed_drills": consecutive_failed_drills,
        "consecutive_not_ready_assessments": consecutive_not_ready_assessments,
    },
    "guardrail_breaches": guardrail_breaches,
    "guardrails_ok": guardrails_ok,
    "runbook_stage": runbook_stage,
    "recommended_action": recommended_action,
    "operator_decision": operator_decision,
    "rollback_recommended": rollback_recommended,
    "selected_stage": selected_stage,
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Enforced pilot runbook markdown: {output_path}")
print(f"Enforced pilot runbook JSON: {output_json_path}")
print(f"Runbook stage: {runbook_stage}")
print(f"Recommended action: {recommended_action}")
print(f"Rollback recommended: {'yes' if rollback_recommended else 'no'}")

if fail_on_rollback and rollback_recommended:
    print("Rollback guardrail triggered for enforced pilot", file=sys.stderr)
    raise SystemExit(2)
PY

