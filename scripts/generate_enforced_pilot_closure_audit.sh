#!/usr/bin/env bash
# Generate enforced pilot closure audit and governance rollout summary.
# Usage:
#   ./scripts/generate_enforced_pilot_closure_audit.sh
#   ./scripts/generate_enforced_pilot_closure_audit.sh --fail-on-audit

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
PROMOTION_POLICY_JSON="$ROOT/governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json"
RUNBOOK_JSON=""
BURN_IN_JSON=""
POSTMORTEM_JSON=""
HANDOFF_SIGNOFF_PACKET_JSON=""
TRANSITION_PACK_JSON=""
SIGNOFF_VALIDATION_JSON=""
TODO_PATH="$ROOT/todo.md"
FAIL_ON_AUDIT=0
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/generate_enforced_pilot_closure_audit.sh [options]

Options:
  --analysis-dir <path>            Analysis directory (default: analysis/benchmark_reproducibility)
  --promotion-policy-json <path>   Governance gate promotion policy JSON path
  --runbook-json <path>            Enforced pilot runbook JSON artifact
  --burn-in-json <path>            Burn-in SLO JSON artifact
  --postmortem-json <path>         Rollback postmortem JSON artifact
  --handoff-signoff-packet-json <path>
                                   Handoff signoff packet JSON artifact
  --transition-pack-json <path>    Governance transition pack JSON artifact
  --signoff-validation-json <path> MONPOL signoff validation JSON artifact
  --todo-path <path>               Week progress tracker path (default: todo.md)
  --fail-on-audit                  Exit non-zero when required audit criteria fail
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
    --runbook-json)
      RUNBOOK_JSON="${2:-}"
      shift 2
      ;;
    --burn-in-json)
      BURN_IN_JSON="${2:-}"
      shift 2
      ;;
    --postmortem-json)
      POSTMORTEM_JSON="${2:-}"
      shift 2
      ;;
    --handoff-signoff-packet-json)
      HANDOFF_SIGNOFF_PACKET_JSON="${2:-}"
      shift 2
      ;;
    --transition-pack-json)
      TRANSITION_PACK_JSON="${2:-}"
      shift 2
      ;;
    --signoff-validation-json)
      SIGNOFF_VALIDATION_JSON="${2:-}"
      shift 2
      ;;
    --todo-path)
      TODO_PATH="${2:-}"
      shift 2
      ;;
    --fail-on-audit)
      FAIL_ON_AUDIT=1
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

for optional in "$RUNBOOK_JSON" "$BURN_IN_JSON" "$POSTMORTEM_JSON" "$HANDOFF_SIGNOFF_PACKET_JSON" "$TRANSITION_PACK_JSON" "$SIGNOFF_VALIDATION_JSON"; do
  if [[ -n "$optional" && ! -f "$optional" ]]; then
    echo "Error: input JSON not found: $optional" >&2
    exit 1
  fi
done

if [[ -n "$TODO_PATH" && ! -f "$TODO_PATH" ]]; then
  echo "Error: todo tracker file not found: $TODO_PATH" >&2
  exit 1
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/enforced_pilot_closure_audit_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - \
  "$ROOT" \
  "$ANALYSIS_DIR" \
  "$PROMOTION_POLICY_JSON" \
  "$RUNBOOK_JSON" \
  "$BURN_IN_JSON" \
  "$POSTMORTEM_JSON" \
  "$HANDOFF_SIGNOFF_PACKET_JSON" \
  "$TRANSITION_PACK_JSON" \
  "$SIGNOFF_VALIDATION_JSON" \
  "$TODO_PATH" \
  "$FAIL_ON_AUDIT" \
  "$OUTPUT_PATH" \
  "$OUTPUT_JSON_PATH" <<'PY'
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
analysis_dir = Path(sys.argv[2])
policy_path = Path(sys.argv[3])
runbook_path_raw = sys.argv[4].strip()
burn_in_path_raw = sys.argv[5].strip()
postmortem_path_raw = sys.argv[6].strip()
handoff_packet_path_raw = sys.argv[7].strip()
transition_pack_path_raw = sys.argv[8].strip()
signoff_validation_path_raw = sys.argv[9].strip()
todo_path = Path(sys.argv[10])
fail_on_audit = bool(int(sys.argv[11]))
output_path = Path(sys.argv[12])
output_json_path = Path(sys.argv[13])


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


policy_payload = load_json(policy_path)
pilot_cfg = policy_payload.get("pilot_rollout", {})
if not isinstance(pilot_cfg, dict):
    pilot_cfg = {}
closure_cfg = pilot_cfg.get("closure_audit", {})
if not isinstance(closure_cfg, dict):
    closure_cfg = {}

require_transition_pack_ready = safe_bool(closure_cfg.get("require_transition_pack_ready"), True)
require_signoff_validation_clean = safe_bool(closure_cfg.get("require_signoff_validation_clean"), True)
require_packet_ready_when_required = safe_bool(closure_cfg.get("require_packet_ready_when_required"), True)
require_postmortem_when_rollback = safe_bool(closure_cfg.get("require_postmortem_when_rollback"), True)
require_burn_in_pass_when_no_rollback = safe_bool(
    closure_cfg.get("require_burn_in_pass_when_no_rollback"), True
)
require_runbook_artifact = safe_bool(closure_cfg.get("require_runbook_artifact"), True)
require_rollout_target_complete = safe_bool(closure_cfg.get("require_rollout_target_complete"), False)
rollout_target_day = max(1, safe_int(closure_cfg.get("rollout_target_day"), 14))

runbook_path = Path(runbook_path_raw) if runbook_path_raw else latest("enforced_pilot_runbook_[0-9]*.json")
burn_in_path = Path(burn_in_path_raw) if burn_in_path_raw else latest("enforced_pilot_burn_in_slo_[0-9]*.json")
postmortem_path = (
    Path(postmortem_path_raw)
    if postmortem_path_raw
    else latest("enforced_pilot_rollback_postmortem_[0-9]*.json")
)
handoff_packet_path = (
    Path(handoff_packet_path_raw)
    if handoff_packet_path_raw
    else latest("enforced_pilot_handoff_signoff_packet_[0-9]*.json")
)
transition_pack_path = (
    Path(transition_pack_path_raw)
    if transition_pack_path_raw
    else latest("governance_transition_pack_[0-9]*.json")
)
signoff_validation_path = (
    Path(signoff_validation_path_raw)
    if signoff_validation_path_raw
    else latest("monpol_signoff_validation_[0-9]*.json")
)

runbook_payload = load_json(runbook_path) if runbook_path and runbook_path.exists() else {}
burn_in_payload = load_json(burn_in_path) if burn_in_path and burn_in_path.exists() else {}
postmortem_payload = load_json(postmortem_path) if postmortem_path and postmortem_path.exists() else {}
handoff_packet_payload = (
    load_json(handoff_packet_path) if handoff_packet_path and handoff_packet_path.exists() else {}
)
transition_pack_payload = (
    load_json(transition_pack_path) if transition_pack_path and transition_pack_path.exists() else {}
)
signoff_validation_payload = (
    load_json(signoff_validation_path)
    if signoff_validation_path and signoff_validation_path.exists()
    else {}
)

active_mode = str(policy_payload.get("active_mode", "advisory"))
if active_mode not in {"advisory", "enforced"}:
    active_mode = "advisory"

runbook_stage = str(runbook_payload.get("runbook_stage", "n/a"))
runbook_action = str(runbook_payload.get("recommended_action", "n/a"))
rollback_recommended = bool(runbook_payload.get("rollback_recommended", False))
preflight_ok = bool(runbook_payload.get("preflight_ok", False))

burn_in_status = str(burn_in_payload.get("overall_status", "n/a"))
postmortem_status = str(postmortem_payload.get("status", "n/a"))
postmortem_required = bool(postmortem_payload.get("required", False))

closure = handoff_packet_payload.get("closure", {})
if not isinstance(closure, dict):
    closure = {}
closure_required = bool(closure.get("closure_required", False))
packet_status = str(closure.get("packet_status", "n/a"))
packet_overall_ready = bool(closure.get("overall_ready", False))
target_monpol_id = str(closure.get("target_monpol_id", "n/a"))
packet_action = str(closure.get("recommended_action", "n/a"))

transition_readiness = transition_pack_payload.get("readiness_checks", {})
if not isinstance(transition_readiness, dict):
    transition_readiness = {}
transition_ready = (
    bool(transition_readiness.get("scripts_ready", False))
    and bool(transition_readiness.get("docs_ready", False))
    and bool(transition_readiness.get("artifacts_ready", False))
    and bool(transition_readiness.get("handoff_signoff_packet_ready_or_not_required", False))
    and bool(transition_readiness.get("incident_closure_required_has_ready_packet", False))
)

signoff_summary = signoff_validation_payload.get("summary", {})
if not isinstance(signoff_summary, dict):
    signoff_summary = {}
signoff_errors = safe_int(signoff_summary.get("errors"), 0)
signoff_warnings = safe_int(signoff_summary.get("warnings"), 0)
signoff_clean = signoff_errors == 0

todo_text = todo_path.read_text(encoding="utf-8")
completed_days = sorted(
    {
        int(match.group(1))
        for match in re.finditer(r"^- Week 10 Day (\d+): ✅ COMPLETE", todo_text, re.MULTILINE)
    }
)
rollout_completed_days = len(completed_days)
latest_completed_day = max(completed_days) if completed_days else 0
rollout_progress_pct = round(min(100.0, (100.0 * rollout_completed_days / rollout_target_day)), 2)
rollout_status = "complete" if rollout_completed_days >= rollout_target_day else "in_progress"
next_step_match = re.search(r"^- Next:\s*(.+)$", todo_text, re.MULTILINE)
next_step = next_step_match.group(1).strip() if next_step_match else "n/a"

criteria = [
    {
        "id": "runbook_artifact_present",
        "description": "Runbook artifact is present.",
        "required": require_runbook_artifact,
        "value": yes_no(bool(runbook_payload)),
        "target": "yes",
        "passed": bool(runbook_payload),
    },
    {
        "id": "handoff_signoff_packet_artifact_present",
        "description": "Handoff signoff packet artifact is present.",
        "required": True,
        "value": yes_no(bool(handoff_packet_payload)),
        "target": "yes",
        "passed": bool(handoff_packet_payload),
    },
    {
        "id": "transition_pack_artifact_present",
        "description": "Transition pack artifact is present.",
        "required": require_transition_pack_ready,
        "value": yes_no(bool(transition_pack_payload)),
        "target": "yes",
        "passed": bool(transition_pack_payload),
    },
    {
        "id": "signoff_validation_artifact_present",
        "description": "Signoff validation artifact is present.",
        "required": require_signoff_validation_clean,
        "value": yes_no(bool(signoff_validation_payload)),
        "target": "yes",
        "passed": bool(signoff_validation_payload),
    },
    {
        "id": "rollback_postmortem_required_on_rollback",
        "description": "Rollback path has required postmortem evidence when configured.",
        "required": rollback_recommended and require_postmortem_when_rollback,
        "value": yes_no(postmortem_required),
        "target": "yes",
        "passed": postmortem_required if (rollback_recommended and require_postmortem_when_rollback) else True,
    },
    {
        "id": "burn_in_pass_when_no_rollback",
        "description": "Burn-in SLO passes for non-rollback closure when configured.",
        "required": (not rollback_recommended) and require_burn_in_pass_when_no_rollback,
        "value": burn_in_status,
        "target": "pass",
        "passed": burn_in_status == "pass"
        if ((not rollback_recommended) and require_burn_in_pass_when_no_rollback)
        else True,
    },
    {
        "id": "handoff_packet_ready_when_required",
        "description": "Handoff signoff packet is ready when closure is required.",
        "required": closure_required and require_packet_ready_when_required,
        "value": yes_no(packet_overall_ready),
        "target": "yes",
        "passed": packet_overall_ready if (closure_required and require_packet_ready_when_required) else True,
    },
    {
        "id": "transition_pack_ready",
        "description": "Transition pack readiness checks pass.",
        "required": require_transition_pack_ready,
        "value": yes_no(transition_ready),
        "target": "yes",
        "passed": transition_ready if require_transition_pack_ready else True,
    },
    {
        "id": "signoff_validation_clean",
        "description": "MONPOL signoff validation has zero errors.",
        "required": require_signoff_validation_clean,
        "value": signoff_errors,
        "target": "0",
        "passed": signoff_clean if require_signoff_validation_clean else True,
    },
    {
        "id": "rollout_target_complete",
        "description": f"Week 10 rollout completed target days ({rollout_target_day}).",
        "required": require_rollout_target_complete,
        "value": rollout_completed_days,
        "target": f">= {rollout_target_day}",
        "passed": rollout_completed_days >= rollout_target_day,
    },
]

required_failures = [item for item in criteria if item["required"] and not item["passed"]]
overall_status = "pass" if not required_failures else "fail"

if rollback_recommended:
    closure_gate_state = "rollback_closure_ready" if overall_status == "pass" else "rollback_blocked"
elif overall_status == "pass":
    closure_gate_state = "pilot_closure_nominal"
else:
    closure_gate_state = "pilot_closure_attention"

if closure_gate_state == "rollback_closure_ready":
    recommended_action = "close_incident_and_record_signoff_packet"
elif closure_gate_state == "rollback_blocked":
    recommended_action = "resolve_required_closure_gaps_before_rollback_closure"
elif closure_gate_state == "pilot_closure_nominal":
    recommended_action = "continue_governance_rollout_monitoring"
else:
    recommended_action = "triage_closure_audit_failures"

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Enforced Pilot Closure Audit and Governance Rollout Summary\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Audit status**: **{overall_status}**  \n")
    fh.write(f"**Closure gate state**: `{closure_gate_state}`  \n")
    fh.write(f"**Recommended action**: `{recommended_action}`\n\n")

    fh.write("## Inputs\n\n")
    fh.write(f"- Promotion policy JSON: `{rel(policy_path)}`\n")
    fh.write(f"- Runbook JSON: `{rel(runbook_path) if runbook_path else 'n/a'}`\n")
    fh.write(f"- Burn-in JSON: `{rel(burn_in_path) if burn_in_path else 'n/a'}`\n")
    fh.write(f"- Postmortem JSON: `{rel(postmortem_path) if postmortem_path else 'n/a'}`\n")
    fh.write(
        f"- Handoff signoff packet JSON: `{rel(handoff_packet_path) if handoff_packet_path else 'n/a'}`\n"
    )
    fh.write(
        f"- Transition pack JSON: `{rel(transition_pack_path) if transition_pack_path else 'n/a'}`\n"
    )
    fh.write(
        f"- Signoff validation JSON: `{rel(signoff_validation_path) if signoff_validation_path else 'n/a'}`\n"
    )
    fh.write(f"- Todo tracker: `{rel(todo_path)}`\n\n")

    fh.write("## Incident Closure Snapshot\n\n")
    fh.write(f"- Active promotion mode: `{active_mode}`\n")
    fh.write(f"- Runbook stage/action: `{runbook_stage}` / `{runbook_action}`\n")
    fh.write(f"- Rollback recommended: `{yes_no(rollback_recommended)}`\n")
    fh.write(f"- Preflight OK: `{yes_no(preflight_ok)}`\n")
    fh.write(f"- Burn-in status: `{burn_in_status}`\n")
    fh.write(f"- Postmortem status: `{postmortem_status}` (required={yes_no(postmortem_required)})\n")
    fh.write(
        f"- Handoff packet: required={yes_no(closure_required)}, status=`{packet_status}`, ready={yes_no(packet_overall_ready)}\n"
    )
    fh.write(f"- Target MONPOL ID: `{target_monpol_id}`\n")
    fh.write(f"- Packet recommended action: `{packet_action}`\n")
    fh.write(
        f"- Signoff validation: errors={signoff_errors}, warnings={signoff_warnings}\n\n"
    )

    fh.write("## Governance Rollout Summary\n\n")
    fh.write(f"- Rollout status: `{rollout_status}`\n")
    fh.write(f"- Week 10 completed days: `{rollout_completed_days}` / `{rollout_target_day}`\n")
    fh.write(f"- Rollout progress (%): `{rollout_progress_pct}`\n")
    fh.write(f"- Latest completed day: `{latest_completed_day}`\n")
    fh.write(f"- Next step hint: {next_step}\n\n")

    fh.write("## Audit Criteria\n\n")
    fh.write("| Criterion | Required | Value | Target | Passed |\n")
    fh.write("|---|---|---|---|---|\n")
    for item in criteria:
        fh.write(
            f"| `{item['id']}` | {yes_no(item['required'])} | {item['value']} | {item['target']} | {yes_no(item['passed'])} |\n"
        )
    fh.write("\n")

    if required_failures:
        fh.write("Required failures:\n")
        for item in required_failures:
            fh.write(f"- `{item['id']}`: {item['description']}\n")
        fh.write("\n")

transition_readiness_subset = {
    key: transition_readiness.get(key)
    for key in [
        "scripts_ready",
        "docs_ready",
        "artifacts_ready",
        "handoff_signoff_packet_present",
        "handoff_signoff_packet_ready_or_not_required",
        "incident_closure_required_has_ready_packet",
    ]
    if key in transition_readiness
}

payload = {
    "generated_at_utc": generated_at,
    "inputs": {
        "analysis_dir": rel(analysis_dir),
        "promotion_policy_json": rel(policy_path),
        "runbook_json": rel(runbook_path) if runbook_path else "n/a",
        "burn_in_json": rel(burn_in_path) if burn_in_path else "n/a",
        "postmortem_json": rel(postmortem_path) if postmortem_path else "n/a",
        "handoff_signoff_packet_json": rel(handoff_packet_path) if handoff_packet_path else "n/a",
        "transition_pack_json": rel(transition_pack_path) if transition_pack_path else "n/a",
        "signoff_validation_json": rel(signoff_validation_path) if signoff_validation_path else "n/a",
        "todo_path": rel(todo_path),
    },
    "policy": {
        "active_mode": active_mode,
        "closure_audit": {
            "require_transition_pack_ready": require_transition_pack_ready,
            "require_signoff_validation_clean": require_signoff_validation_clean,
            "require_packet_ready_when_required": require_packet_ready_when_required,
            "require_postmortem_when_rollback": require_postmortem_when_rollback,
            "require_burn_in_pass_when_no_rollback": require_burn_in_pass_when_no_rollback,
            "require_runbook_artifact": require_runbook_artifact,
            "require_rollout_target_complete": require_rollout_target_complete,
            "rollout_target_day": rollout_target_day,
        },
    },
    "incident_closure_snapshot": {
        "runbook_stage": runbook_stage,
        "runbook_action": runbook_action,
        "rollback_recommended": rollback_recommended,
        "preflight_ok": preflight_ok,
        "burn_in_status": burn_in_status,
        "postmortem_status": postmortem_status,
        "postmortem_required": postmortem_required,
        "packet_closure_required": closure_required,
        "packet_status": packet_status,
        "packet_overall_ready": packet_overall_ready,
        "packet_recommended_action": packet_action,
        "target_monpol_id": target_monpol_id,
        "signoff_validation_errors": signoff_errors,
        "signoff_validation_warnings": signoff_warnings,
    },
    "governance_rollout_summary": {
        "status": rollout_status,
        "completed_days": rollout_completed_days,
        "target_day": rollout_target_day,
        "progress_pct": rollout_progress_pct,
        "latest_completed_day": latest_completed_day,
        "completed_day_list": completed_days,
        "next_step": next_step,
    },
    "transition_pack_readiness_subset": transition_readiness_subset,
    "criteria": criteria,
    "required_failures": required_failures,
    "overall_status": overall_status,
    "closure_gate_state": closure_gate_state,
    "recommended_action": recommended_action,
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Enforced pilot closure audit markdown: {output_path}")
print(f"Enforced pilot closure audit JSON: {output_json_path}")
print(f"Audit status: {overall_status}")
print(f"Closure gate state: {closure_gate_state}")
print(f"Recommended action: {recommended_action}")

if fail_on_audit and overall_status != "pass":
    print("Enforced pilot closure audit has required failures", file=sys.stderr)
    raise SystemExit(2)
PY

