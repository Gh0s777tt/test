#!/usr/bin/env bash
# Scaffold rollback postmortem template from enforced pilot telemetry.
# Usage:
#   ./scripts/scaffold_enforced_pilot_rollback_postmortem.sh
#   ./scripts/scaffold_enforced_pilot_rollback_postmortem.sh --require-rollback

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
PROMOTION_POLICY_JSON="$ROOT/governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json"
RUNBOOK_JSON=""
BURN_IN_JSON=""
READINESS_JSON=""
BREACH_ROUTE_JSON=""
HANDOFF_JSON=""
DRILL_JSON=""
REQUIRE_ROLLBACK=0
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/scaffold_enforced_pilot_rollback_postmortem.sh [options]

Options:
  --analysis-dir <path>            Analysis directory (default: analysis/benchmark_reproducibility)
  --promotion-policy-json <path>   Governance gate promotion policy JSON path
  --runbook-json <path>            Enforced pilot runbook JSON artifact
  --burn-in-json <path>            Burn-in SLO JSON artifact
  --readiness-json <path>          Promotion readiness JSON artifact
  --breach-route-json <path>       Breach route JSON artifact
  --handoff-json <path>            Release handoff JSON artifact
  --drill-json <path>              Release readiness drill JSON artifact
  --require-rollback               Exit non-zero if rollback is not currently required
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
    --require-rollback)
      REQUIRE_ROLLBACK=1
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

for optional in "$RUNBOOK_JSON" "$BURN_IN_JSON" "$READINESS_JSON" "$BREACH_ROUTE_JSON" "$HANDOFF_JSON" "$DRILL_JSON"; do
  if [[ -n "$optional" && ! -f "$optional" ]]; then
    echo "Error: input JSON not found: $optional" >&2
    exit 1
  fi
done

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/enforced_pilot_rollback_postmortem_${TIMESTAMP}.md"
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
  "$READINESS_JSON" \
  "$BREACH_ROUTE_JSON" \
  "$HANDOFF_JSON" \
  "$DRILL_JSON" \
  "$REQUIRE_ROLLBACK" \
  "$OUTPUT_PATH" \
  "$OUTPUT_JSON_PATH" <<'PY'
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
analysis_dir = Path(sys.argv[2])
policy_path = Path(sys.argv[3])
runbook_path_raw = sys.argv[4].strip()
burn_in_path_raw = sys.argv[5].strip()
readiness_path_raw = sys.argv[6].strip()
breach_path_raw = sys.argv[7].strip()
handoff_path_raw = sys.argv[8].strip()
drill_path_raw = sys.argv[9].strip()
require_rollback = bool(int(sys.argv[10]))
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


policy_payload = load_json(policy_path)
pilot_cfg = policy_payload.get("pilot_rollout", {})
if not isinstance(pilot_cfg, dict):
    pilot_cfg = {}
postmortem_cfg = pilot_cfg.get("rollback_postmortem", {})
if not isinstance(postmortem_cfg, dict):
    postmortem_cfg = {}

required_actions = postmortem_cfg.get(
    "required_on_actions",
    ["rollback_to_advisory", "hold_enforced_and_triage"],
)
if not isinstance(required_actions, list) or not required_actions:
    required_actions = ["rollback_to_advisory", "hold_enforced_and_triage"]
required_actions = [str(action) for action in required_actions if str(action).strip()]

template_sections = postmortem_cfg.get(
    "template_sections",
    [
        "Incident Summary",
        "Impact Assessment",
        "Timeline",
        "Detection and Guardrail Trigger",
        "Immediate Mitigation",
        "Root Cause",
        "Corrective Actions",
        "Follow-up Verification",
        "Approvals",
    ],
)
if not isinstance(template_sections, list) or not template_sections:
    template_sections = [
        "Incident Summary",
        "Impact Assessment",
        "Timeline",
        "Detection and Guardrail Trigger",
        "Immediate Mitigation",
        "Root Cause",
        "Corrective Actions",
        "Follow-up Verification",
        "Approvals",
    ]

runbook_path = Path(runbook_path_raw) if runbook_path_raw else latest("enforced_pilot_runbook_[0-9]*.json")
burn_in_path = Path(burn_in_path_raw) if burn_in_path_raw else latest("enforced_pilot_burn_in_slo_[0-9]*.json")
readiness_path = Path(readiness_path_raw) if readiness_path_raw else latest("governance_gate_promotion_readiness_[0-9]*.json")
breach_path = Path(breach_path_raw) if breach_path_raw else latest("monitor_drift_breach_route_[0-9]*.json")
handoff_path = Path(handoff_path_raw) if handoff_path_raw else latest("monitor_drift_release_handoff_[0-9]*.json")
drill_path = Path(drill_path_raw) if drill_path_raw else latest("monitor_drift_release_readiness_drill_[0-9]*.json")

runbook_payload = load_json(runbook_path) if runbook_path and runbook_path.exists() else {}
burn_in_payload = load_json(burn_in_path) if burn_in_path and burn_in_path.exists() else {}
readiness_payload = load_json(readiness_path) if readiness_path and readiness_path.exists() else {}
breach_payload = load_json(breach_path) if breach_path and breach_path.exists() else {}
handoff_payload = load_json(handoff_path) if handoff_path and handoff_path.exists() else {}
drill_payload = load_json(drill_path) if drill_path and drill_path.exists() else {}

runbook_action = str(runbook_payload.get("recommended_action", "n/a"))
runbook_stage = str(runbook_payload.get("runbook_stage", "n/a"))
rollback_recommended = bool(runbook_payload.get("rollback_recommended", False))
guardrail_breaches = runbook_payload.get("guardrail_breaches", [])
if not isinstance(guardrail_breaches, list):
    guardrail_breaches = []

required = rollback_recommended or (runbook_action in set(required_actions))
status = "draft" if required else "not_required"

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Enforced Pilot Rollback Postmortem Scaffold\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Status**: `{status}`  \n")
    fh.write(f"**Rollback required**: `{yes_no(required)}`  \n")
    fh.write(f"**Runbook stage**: `{runbook_stage}`  \n")
    fh.write(f"**Runbook action**: `{runbook_action}`\n\n")

    fh.write("## Inputs\n\n")
    fh.write(f"- Promotion policy: `{rel(policy_path)}`\n")
    fh.write(f"- Runbook artifact: `{rel(runbook_path) if runbook_path else 'n/a'}`\n")
    fh.write(f"- Burn-in SLO artifact: `{rel(burn_in_path) if burn_in_path else 'n/a'}`\n")
    fh.write(f"- Readiness artifact: `{rel(readiness_path) if readiness_path else 'n/a'}`\n")
    fh.write(f"- Breach route artifact: `{rel(breach_path) if breach_path else 'n/a'}`\n")
    fh.write(f"- Handoff artifact: `{rel(handoff_path) if handoff_path else 'n/a'}`\n")
    fh.write(f"- Drill artifact: `{rel(drill_path) if drill_path else 'n/a'}`\n\n")

    fh.write("## Trigger Snapshot\n\n")
    fh.write(f"- rollback_recommended: `{yes_no(rollback_recommended)}`\n")
    fh.write(f"- burn-in overall status: `{burn_in_payload.get('overall_status', 'n/a') if burn_in_payload else 'n/a'}`\n")
    fh.write(f"- readiness overall ready: `{yes_no(bool(readiness_payload.get('overall_ready', False))) if readiness_payload else 'n/a'}`\n")
    fh.write(f"- handoff overall status: `{handoff_payload.get('overall_status', 'n/a') if handoff_payload else 'n/a'}`\n")
    fh.write(f"- drill overall status: `{drill_payload.get('overall_status', 'n/a') if drill_payload else 'n/a'}`\n")
    fh.write(f"- breach detected: `{yes_no(bool(breach_payload.get('breach_detected', False))) if breach_payload else 'n/a'}`\n")
    if guardrail_breaches:
        fh.write(f"- guardrail breaches: {', '.join(str(item.get('id', 'n/a')) for item in guardrail_breaches if isinstance(item, dict))}\n")
    else:
        fh.write("- guardrail breaches: none\n")
    fh.write("\n")

    if not required:
        fh.write("No active rollback trigger detected. Keep this scaffold as a ready template.\n\n")
    else:
        fh.write("Rollback is required. Complete all sections below before closure.\n\n")

    for section in template_sections:
        fh.write(f"## {section}\n\n")
        fh.write("- TODO: fill this section with incident-specific details.\n\n")

payload = {
    "generated_at_utc": generated_at,
    "inputs": {
        "analysis_dir": rel(analysis_dir),
        "promotion_policy_json": rel(policy_path),
        "runbook_json": rel(runbook_path) if runbook_path else "n/a",
        "burn_in_json": rel(burn_in_path) if burn_in_path else "n/a",
        "readiness_json": rel(readiness_path) if readiness_path else "n/a",
        "breach_route_json": rel(breach_path) if breach_path else "n/a",
        "handoff_json": rel(handoff_path) if handoff_path else "n/a",
        "drill_json": rel(drill_path) if drill_path else "n/a",
    },
    "required": required,
    "status": status,
    "trigger": {
        "runbook_stage": runbook_stage,
        "runbook_action": runbook_action,
        "rollback_recommended": rollback_recommended,
        "required_on_actions": required_actions,
        "guardrail_breaches": guardrail_breaches,
    },
    "snapshot": {
        "burn_in_overall_status": str(burn_in_payload.get("overall_status", "n/a")) if burn_in_payload else "n/a",
        "readiness_overall_ready": bool(readiness_payload.get("overall_ready", False)) if readiness_payload else None,
        "handoff_overall_status": str(handoff_payload.get("overall_status", "n/a")) if handoff_payload else "n/a",
        "drill_overall_status": str(drill_payload.get("overall_status", "n/a")) if drill_payload else "n/a",
        "breach_detected": bool(breach_payload.get("breach_detected", False)) if breach_payload else None,
    },
    "template_sections": template_sections,
    "evidence_bundle": [
        rel(path)
        for path in [runbook_path, burn_in_path, readiness_path, breach_path, handoff_path, drill_path]
        if path is not None
    ],
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Enforced pilot rollback postmortem markdown: {output_path}")
print(f"Enforced pilot rollback postmortem JSON: {output_json_path}")
print(f"Rollback required: {'yes' if required else 'no'}")
print(f"Status: {status}")

if require_rollback and not required:
    print("Rollback postmortem is not currently required", file=sys.stderr)
    raise SystemExit(2)
PY

