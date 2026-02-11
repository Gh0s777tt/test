#!/usr/bin/env bash
# Generate enforced pilot incident-closure handoff signoff packet.
# Usage:
#   ./scripts/generate_enforced_pilot_handoff_signoff_packet.sh
#   ./scripts/generate_enforced_pilot_handoff_signoff_packet.sh --require-ready

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
PROMOTION_POLICY_JSON="$ROOT/governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json"
SIGNOFF_JSON="$ROOT/governance/performance/MONPOL_SIGNOFFS.json"
RUNBOOK_JSON=""
BURN_IN_JSON=""
POSTMORTEM_JSON=""
READINESS_JSON=""
BREACH_ROUTE_JSON=""
HANDOFF_JSON=""
DRILL_JSON=""
REQUIRE_READY=0
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/generate_enforced_pilot_handoff_signoff_packet.sh [options]

Options:
  --analysis-dir <path>            Analysis directory (default: analysis/benchmark_reproducibility)
  --promotion-policy-json <path>   Governance gate promotion policy JSON path
  --signoff-json <path>            MONPOL signoff metadata JSON path
  --runbook-json <path>            Enforced pilot runbook JSON artifact
  --burn-in-json <path>            Burn-in SLO JSON artifact
  --postmortem-json <path>         Rollback postmortem JSON artifact
  --readiness-json <path>          Promotion readiness JSON artifact
  --breach-route-json <path>       Breach route JSON artifact
  --handoff-json <path>            Release handoff JSON artifact
  --drill-json <path>              Release readiness drill JSON artifact
  --require-ready                  Exit non-zero when packet is required but not ready
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
    --signoff-json)
      SIGNOFF_JSON="${2:-}"
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
    --require-ready)
      REQUIRE_READY=1
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

if [[ ! -f "$SIGNOFF_JSON" ]]; then
  echo "Error: signoff metadata JSON not found: $SIGNOFF_JSON" >&2
  exit 1
fi

for optional in "$RUNBOOK_JSON" "$BURN_IN_JSON" "$POSTMORTEM_JSON" "$READINESS_JSON" "$BREACH_ROUTE_JSON" "$HANDOFF_JSON" "$DRILL_JSON"; do
  if [[ -n "$optional" && ! -f "$optional" ]]; then
    echo "Error: input JSON not found: $optional" >&2
    exit 1
  fi
done

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/enforced_pilot_handoff_signoff_packet_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - \
  "$ROOT" \
  "$ANALYSIS_DIR" \
  "$PROMOTION_POLICY_JSON" \
  "$SIGNOFF_JSON" \
  "$RUNBOOK_JSON" \
  "$BURN_IN_JSON" \
  "$POSTMORTEM_JSON" \
  "$READINESS_JSON" \
  "$BREACH_ROUTE_JSON" \
  "$HANDOFF_JSON" \
  "$DRILL_JSON" \
  "$REQUIRE_READY" \
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
signoff_path = Path(sys.argv[4])
runbook_path_raw = sys.argv[5].strip()
burn_in_path_raw = sys.argv[6].strip()
postmortem_path_raw = sys.argv[7].strip()
readiness_path_raw = sys.argv[8].strip()
breach_path_raw = sys.argv[9].strip()
handoff_path_raw = sys.argv[10].strip()
drill_path_raw = sys.argv[11].strip()
require_ready = bool(int(sys.argv[12]))
output_path = Path(sys.argv[13])
output_json_path = Path(sys.argv[14])


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


policy_payload = load_json(policy_path)
pilot_cfg = policy_payload.get("pilot_rollout", {})
if not isinstance(pilot_cfg, dict):
    pilot_cfg = {}
incident_cfg = pilot_cfg.get("incident_closure", {})
if not isinstance(incident_cfg, dict):
    incident_cfg = {}

require_signoff_packet_on_rollback = bool(incident_cfg.get("require_signoff_packet_on_rollback", True))
always_require_handoff_signoff = bool(incident_cfg.get("always_require_handoff_signoff", False))
require_owner_signoff = bool(incident_cfg.get("require_owner_signoff", True))
require_postmortem_for_closure = bool(incident_cfg.get("require_postmortem_for_closure", True))
require_burn_in_pass_for_closure = bool(incident_cfg.get("require_burn_in_pass_for_closure", True))
min_signoff_count = max(0, safe_int(incident_cfg.get("min_signoff_count"), 1))
required_roles = incident_cfg.get("required_roles", [])
if not isinstance(required_roles, list):
    required_roles = []
required_roles = [str(role).strip() for role in required_roles if str(role).strip()]

runbook_path = Path(runbook_path_raw) if runbook_path_raw else latest("enforced_pilot_runbook_[0-9]*.json")
burn_in_path = Path(burn_in_path_raw) if burn_in_path_raw else latest("enforced_pilot_burn_in_slo_[0-9]*.json")
postmortem_path = Path(postmortem_path_raw) if postmortem_path_raw else latest("enforced_pilot_rollback_postmortem_[0-9]*.json")
readiness_path = Path(readiness_path_raw) if readiness_path_raw else latest("governance_gate_promotion_readiness_[0-9]*.json")
breach_path = Path(breach_path_raw) if breach_path_raw else latest("monitor_drift_breach_route_[0-9]*.json")
handoff_path = Path(handoff_path_raw) if handoff_path_raw else latest("monitor_drift_release_handoff_[0-9]*.json")
drill_path = Path(drill_path_raw) if drill_path_raw else latest("monitor_drift_release_readiness_drill_[0-9]*.json")

runbook_payload = load_json(runbook_path) if runbook_path and runbook_path.exists() else {}
burn_in_payload = load_json(burn_in_path) if burn_in_path and burn_in_path.exists() else {}
postmortem_payload = load_json(postmortem_path) if postmortem_path and postmortem_path.exists() else {}
readiness_payload = load_json(readiness_path) if readiness_path and readiness_path.exists() else {}
breach_payload = load_json(breach_path) if breach_path and breach_path.exists() else {}
handoff_payload = load_json(handoff_path) if handoff_path and handoff_path.exists() else {}
drill_payload = load_json(drill_path) if drill_path and drill_path.exists() else {}
signoff_payload = load_json(signoff_path)

runbook_action = str(runbook_payload.get("recommended_action", "n/a"))
runbook_stage = str(runbook_payload.get("runbook_stage", "n/a"))
rollback_recommended = bool(runbook_payload.get("rollback_recommended", False))
rollback_from_postmortem = bool(postmortem_payload.get("required", False))
rollback_required = rollback_recommended or rollback_from_postmortem
closure_required = always_require_handoff_signoff or (
    require_signoff_packet_on_rollback and rollback_required
)

latest_monpol_id = "n/a"
if breach_payload:
    latest_monpol_id = str(breach_payload.get("latest_monpol_id", "n/a"))
if not re.fullmatch(r"MONPOL-\d{3}", latest_monpol_id):
    changelog_path = root / "governance" / "performance" / "MONITOR_THRESHOLD_CHANGELOG.md"
    if changelog_path.exists():
        text = changelog_path.read_text(encoding="utf-8")
        matches = list(re.finditer(r"MONPOL-(\d{3})", text))
        if matches:
            latest_monpol_id = f"MONPOL-{int(matches[-1].group(1)):03d}"
        else:
            latest_monpol_id = "n/a"
    else:
        latest_monpol_id = "n/a"

records = signoff_payload.get("records", [])
if not isinstance(records, list):
    records = []
signoff_record = None
for record in records:
    if not isinstance(record, dict):
        continue
    if str(record.get("proposal_id", "")).strip() == latest_monpol_id:
        signoff_record = record
        break

reviewers = signoff_record.get("reviewers", []) if isinstance(signoff_record, dict) else []
if not isinstance(reviewers, list):
    reviewers = []
reviewer_roles = []
for reviewer in reviewers:
    if not isinstance(reviewer, dict):
        continue
    role = str(reviewer.get("role", "")).strip()
    if role:
        reviewer_roles.append(role)
roles_present = sorted(set(reviewer_roles))

owner_value = str(signoff_record.get("owner", "")).strip() if isinstance(signoff_record, dict) else ""
owner_present = bool(owner_value)
signoff_count = len(reviewers) + (1 if owner_present else 0)
signoff_decision = str(signoff_record.get("decision", "n/a")).strip() if isinstance(signoff_record, dict) else "n/a"

role_coverage = [
    {"role": role, "covered": role in set(roles_present)}
    for role in required_roles
]
missing_roles = [item["role"] for item in role_coverage if not item["covered"]]

burn_in_status = str(burn_in_payload.get("overall_status", "n/a")) if burn_in_payload else "n/a"
postmortem_status = str(postmortem_payload.get("status", "n/a")) if postmortem_payload else "n/a"
postmortem_required = bool(postmortem_payload.get("required", False)) if postmortem_payload else False
readiness_ready = bool(readiness_payload.get("overall_ready", False)) if readiness_payload else False
handoff_status = str(handoff_payload.get("overall_status", "n/a")) if handoff_payload else "n/a"
drill_status = str(drill_payload.get("overall_status", "n/a")) if drill_payload else "n/a"
breach_detected = bool(breach_payload.get("breach_detected", False)) if breach_payload else False

criteria = [
    {
        "id": "runbook_artifact_present",
        "description": "Runbook artifact is present.",
        "required": True,
        "value": yes_no(bool(runbook_payload)),
        "target": "yes",
        "passed": bool(runbook_payload),
    },
    {
        "id": "burn_in_artifact_present",
        "description": "Burn-in SLO artifact is present.",
        "required": True,
        "value": yes_no(bool(burn_in_payload)),
        "target": "yes",
        "passed": bool(burn_in_payload),
    },
    {
        "id": "postmortem_alignment",
        "description": "Postmortem artifact aligns with rollback requirement when configured.",
        "required": require_postmortem_for_closure and rollback_required,
        "value": yes_no(postmortem_required),
        "target": "yes",
        "passed": (postmortem_required if (require_postmortem_for_closure and rollback_required) else True),
    },
    {
        "id": "burn_in_pass_for_closure",
        "description": "Burn-in SLO status passes for non-rollback closure when configured.",
        "required": require_burn_in_pass_for_closure and not rollback_required,
        "value": burn_in_status,
        "target": "pass",
        "passed": (burn_in_status == "pass") if (require_burn_in_pass_for_closure and not rollback_required) else True,
    },
    {
        "id": "signoff_record_present",
        "description": "Signoff record exists for target MONPOL proposal.",
        "required": closure_required,
        "value": yes_no(bool(signoff_record)),
        "target": "yes",
        "passed": bool(signoff_record) if closure_required else True,
    },
    {
        "id": "signoff_count_threshold",
        "description": f"Signoff count meets minimum threshold ({min_signoff_count}).",
        "required": closure_required,
        "value": signoff_count,
        "target": f">= {min_signoff_count}",
        "passed": signoff_count >= min_signoff_count if closure_required else True,
    },
    {
        "id": "owner_signoff_present",
        "description": "Owner signoff is present when required.",
        "required": closure_required and require_owner_signoff,
        "value": yes_no(owner_present),
        "target": "yes",
        "passed": owner_present if (closure_required and require_owner_signoff) else True,
    },
    {
        "id": "required_roles_covered",
        "description": "Required reviewer roles are covered.",
        "required": closure_required and len(required_roles) > 0,
        "value": ",".join(roles_present) if roles_present else "none",
        "target": ",".join(required_roles) if required_roles else "n/a",
        "passed": len(missing_roles) == 0 if (closure_required and len(required_roles) > 0) else True,
    },
    {
        "id": "signoff_decision_approved",
        "description": "Signoff decision is approved for closure.",
        "required": closure_required,
        "value": signoff_decision,
        "target": "approved",
        "passed": signoff_decision == "approved" if closure_required else True,
    },
]

required_failures = [item for item in criteria if item["required"] and not item["passed"]]
overall_ready = len(required_failures) == 0

if not closure_required:
    packet_status = "not_required"
    recommended_action = "collect_for_reference"
elif overall_ready:
    packet_status = "ready"
    recommended_action = "closure_signoff_ready"
else:
    packet_status = "pending"
    recommended_action = "collect_required_signoffs"

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Enforced Pilot Incident Closure Handoff Signoff Packet\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Packet status**: `{packet_status}`  \n")
    fh.write(f"**Closure required**: `{yes_no(closure_required)}`  \n")
    fh.write(f"**Overall ready**: `{yes_no(overall_ready)}`  \n")
    fh.write(f"**Recommended action**: `{recommended_action}`\n\n")

    fh.write("## Inputs\n\n")
    fh.write(f"- Promotion policy: `{rel(policy_path)}`\n")
    fh.write(f"- Signoff metadata: `{rel(signoff_path)}`\n")
    fh.write(f"- Runbook artifact: `{rel(runbook_path) if runbook_path else 'n/a'}`\n")
    fh.write(f"- Burn-in SLO artifact: `{rel(burn_in_path) if burn_in_path else 'n/a'}`\n")
    fh.write(f"- Postmortem artifact: `{rel(postmortem_path) if postmortem_path else 'n/a'}`\n")
    fh.write(f"- Readiness artifact: `{rel(readiness_path) if readiness_path else 'n/a'}`\n")
    fh.write(f"- Breach route artifact: `{rel(breach_path) if breach_path else 'n/a'}`\n")
    fh.write(f"- Handoff artifact: `{rel(handoff_path) if handoff_path else 'n/a'}`\n")
    fh.write(f"- Drill artifact: `{rel(drill_path) if drill_path else 'n/a'}`\n\n")

    fh.write("## Incident Closure Snapshot\n\n")
    fh.write(f"- Target MONPOL ID: `{latest_monpol_id}`\n")
    fh.write(f"- Runbook stage/action: `{runbook_stage}` / `{runbook_action}`\n")
    fh.write(f"- Rollback required: `{yes_no(rollback_required)}`\n")
    fh.write(f"- Burn-in status: `{burn_in_status}`\n")
    fh.write(f"- Postmortem status: `{postmortem_status}` (required={yes_no(postmortem_required)})\n")
    fh.write(f"- Readiness overall: `{yes_no(readiness_ready)}`\n")
    fh.write(f"- Handoff status: `{handoff_status}`\n")
    fh.write(f"- Drill status: `{drill_status}`\n")
    fh.write(f"- Breach detected: `{yes_no(breach_detected)}`\n\n")

    fh.write("## Signoff Coverage\n\n")
    fh.write(f"- Signoff record present: `{yes_no(bool(signoff_record))}`\n")
    fh.write(f"- Signoff decision: `{signoff_decision}`\n")
    fh.write(f"- Owner present: `{yes_no(owner_present)}`\n")
    fh.write(f"- Signoff count: `{signoff_count}` (min required: `{min_signoff_count}`)\n")
    fh.write(f"- Reviewer roles present: `{', '.join(roles_present) if roles_present else 'none'}`\n")
    fh.write(f"- Missing required roles: `{', '.join(missing_roles) if missing_roles else 'none'}`\n\n")

    if required_roles:
        fh.write("| Required role | Covered |\n")
        fh.write("|---|---|\n")
        for item in role_coverage:
            fh.write(f"| `{item['role']}` | {yes_no(item['covered'])} |\n")
        fh.write("\n")

    fh.write("## Criteria\n\n")
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

payload = {
    "generated_at_utc": generated_at,
    "inputs": {
        "analysis_dir": rel(analysis_dir),
        "promotion_policy_json": rel(policy_path),
        "signoff_json": rel(signoff_path),
        "runbook_json": rel(runbook_path) if runbook_path else "n/a",
        "burn_in_json": rel(burn_in_path) if burn_in_path else "n/a",
        "postmortem_json": rel(postmortem_path) if postmortem_path else "n/a",
        "readiness_json": rel(readiness_path) if readiness_path else "n/a",
        "breach_route_json": rel(breach_path) if breach_path else "n/a",
        "handoff_json": rel(handoff_path) if handoff_path else "n/a",
        "drill_json": rel(drill_path) if drill_path else "n/a",
    },
    "policy": {
        "require_signoff_packet_on_rollback": require_signoff_packet_on_rollback,
        "always_require_handoff_signoff": always_require_handoff_signoff,
        "require_owner_signoff": require_owner_signoff,
        "require_postmortem_for_closure": require_postmortem_for_closure,
        "require_burn_in_pass_for_closure": require_burn_in_pass_for_closure,
        "min_signoff_count": min_signoff_count,
        "required_roles": required_roles,
    },
    "closure": {
        "target_monpol_id": latest_monpol_id,
        "runbook_stage": runbook_stage,
        "runbook_action": runbook_action,
        "rollback_required": rollback_required,
        "closure_required": closure_required,
        "packet_status": packet_status,
        "overall_ready": overall_ready,
        "recommended_action": recommended_action,
    },
    "snapshot": {
        "burn_in_status": burn_in_status,
        "postmortem_status": postmortem_status,
        "postmortem_required": postmortem_required,
        "readiness_overall_ready": readiness_ready,
        "handoff_status": handoff_status,
        "drill_status": drill_status,
        "breach_detected": breach_detected,
    },
    "signoff": {
        "record_present": bool(signoff_record),
        "decision": signoff_decision,
        "owner_present": owner_present,
        "owner": owner_value if owner_present else "n/a",
        "signoff_count": signoff_count,
        "reviewer_roles_present": roles_present,
        "required_roles": required_roles,
        "missing_required_roles": missing_roles,
    },
    "criteria": criteria,
    "required_failures": required_failures,
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Enforced pilot handoff signoff packet markdown: {output_path}")
print(f"Enforced pilot handoff signoff packet JSON: {output_json_path}")
print(f"Packet status: {packet_status}")
print(f"Closure required: {'yes' if closure_required else 'no'}")
print(f"Overall ready: {'yes' if overall_ready else 'no'}")

if require_ready and closure_required and not overall_ready:
    print("Handoff signoff packet is required but not ready", file=sys.stderr)
    raise SystemExit(2)
PY

