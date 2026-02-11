#!/usr/bin/env bash
# Build escalation breach evidence routing artifact and promotion snapshot.
# Usage:
#   ./scripts/route_monitor_drift_breach_evidence.sh
#   ./scripts/route_monitor_drift_breach_evidence.sh --promotion-mode enforced --fail-on-breach

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
ESCALATION_JSON=""
HANDOFF_JSON=""
DRILL_JSON=""
PROMOTION_POLICY_JSON="$ROOT/governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json"
PROMOTION_MODE="auto"
FAIL_ON_BREACH=0
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/route_monitor_drift_breach_evidence.sh [options]

Options:
  --analysis-dir <path>             Analysis directory (default: analysis/benchmark_reproducibility)
  --escalation-json <path>          Escalation JSON artifact (default: newest monitor_drift_escalation_*.json)
  --handoff-json <path>             Handoff JSON artifact (default: newest monitor_drift_release_handoff_*.json)
  --drill-json <path>               Drill JSON artifact (default: newest monitor_drift_release_readiness_drill_*.json)
  --promotion-policy-json <path>    Governance gate promotion policy JSON path
  --promotion-mode <auto|advisory|enforced>
  --fail-on-breach                  Exit non-zero when policy says breach should block
  --output <path>                   Output markdown path
  --output-json <path>              Output JSON path
  -h, --help                        Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --escalation-json)
      ESCALATION_JSON="${2:-}"
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
    --promotion-policy-json)
      PROMOTION_POLICY_JSON="${2:-}"
      shift 2
      ;;
    --promotion-mode)
      PROMOTION_MODE="${2:-}"
      shift 2
      ;;
    --fail-on-breach)
      FAIL_ON_BREACH=1
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

if [[ -z "$ESCALATION_JSON" ]]; then
  shopt -s nullglob
  candidates=("$ANALYSIS_DIR"/monitor_drift_escalation_[0-9]*.json)
  shopt -u nullglob
  if (( ${#candidates[@]} > 0 )); then
    readarray -t sorted < <(printf '%s\n' "${candidates[@]}" | sort)
    ESCALATION_JSON="${sorted[${#sorted[@]}-1]}"
  fi
fi

if [[ -z "$HANDOFF_JSON" ]]; then
  shopt -s nullglob
  candidates=("$ANALYSIS_DIR"/monitor_drift_release_handoff_[0-9]*.json)
  shopt -u nullglob
  if (( ${#candidates[@]} > 0 )); then
    readarray -t sorted < <(printf '%s\n' "${candidates[@]}" | sort)
    HANDOFF_JSON="${sorted[${#sorted[@]}-1]}"
  fi
fi

if [[ -z "$DRILL_JSON" ]]; then
  shopt -s nullglob
  candidates=("$ANALYSIS_DIR"/monitor_drift_release_readiness_drill_[0-9]*.json)
  shopt -u nullglob
  if (( ${#candidates[@]} > 0 )); then
    readarray -t sorted < <(printf '%s\n' "${candidates[@]}" | sort)
    DRILL_JSON="${sorted[${#sorted[@]}-1]}"
  fi
fi

if [[ -z "$ESCALATION_JSON" || ! -f "$ESCALATION_JSON" ]]; then
  echo "Error: escalation JSON not found: $ESCALATION_JSON" >&2
  exit 1
fi

if [[ -z "$HANDOFF_JSON" || ! -f "$HANDOFF_JSON" ]]; then
  echo "Error: handoff JSON not found: $HANDOFF_JSON" >&2
  exit 1
fi

if [[ -z "$DRILL_JSON" || ! -f "$DRILL_JSON" ]]; then
  echo "Error: drill JSON not found: $DRILL_JSON" >&2
  exit 1
fi

if [[ -n "$PROMOTION_POLICY_JSON" && ! -f "$PROMOTION_POLICY_JSON" ]]; then
  echo "Error: promotion policy JSON not found: $PROMOTION_POLICY_JSON" >&2
  exit 1
fi

case "$PROMOTION_MODE" in
  auto|advisory|enforced) ;;
  *)
    echo "Error: --promotion-mode must be one of auto|advisory|enforced" >&2
    exit 1
    ;;
esac

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/monitor_drift_breach_route_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - \
  "$ROOT" \
  "$ESCALATION_JSON" \
  "$HANDOFF_JSON" \
  "$DRILL_JSON" \
  "$PROMOTION_POLICY_JSON" \
  "$PROMOTION_MODE" \
  "$FAIL_ON_BREACH" \
  "$OUTPUT_PATH" \
  "$OUTPUT_JSON_PATH" <<'PY'
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
escalation_path = Path(sys.argv[2])
handoff_path = Path(sys.argv[3])
drill_path = Path(sys.argv[4])
policy_path = Path(sys.argv[5])
promotion_mode_arg = sys.argv[6].strip()
fail_on_breach = bool(int(sys.argv[7]))
output_path = Path(sys.argv[8])
output_json_path = Path(sys.argv[9])


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


def yes_no(value: bool) -> str:
    return "yes" if value else "no"


severity = {"normal": 0, "watch": 1, "escalated": 2, "critical": 3}

escalation = load_json(escalation_path)
handoff = load_json(handoff_path)
drill = load_json(drill_path)
policy = load_json(policy_path) if policy_path.is_file() else {}

policy_modes = policy.get("modes", {})
if not isinstance(policy_modes, dict):
    policy_modes = {}

policy_mode = str(policy.get("active_mode", "advisory"))
if policy_mode not in {"advisory", "enforced"}:
    policy_mode = "advisory"

active_mode = policy_mode if promotion_mode_arg == "auto" else promotion_mode_arg
if active_mode not in {"advisory", "enforced"}:
    active_mode = "advisory"

mode_cfg = policy_modes.get(active_mode, {})
if not isinstance(mode_cfg, dict):
    mode_cfg = {}

enforce_on_breach = bool(mode_cfg.get("enforce_on_breach", active_mode == "enforced"))
require_pr_mitigation = bool(mode_cfg.get("require_pr_mitigation", active_mode == "enforced"))
require_breach_ack_token = bool(mode_cfg.get("require_breach_ack_token", active_mode == "enforced"))

overall_level = str(escalation.get("overall_level", "n/a"))
handoff_status = str(handoff.get("overall_status", "n/a"))
drill_status = str(drill.get("overall_status", "n/a"))

breach_sources = []
if overall_level in {"escalated", "critical"}:
    breach_sources.append("escalation_level")
if bool(escalation.get("fail_triggered", False)):
    breach_sources.append("escalation_fail_trigger")
if handoff_status == "blocked":
    breach_sources.append("handoff_blocked")
if drill_status != "pass":
    breach_sources.append("drill_failed")

breach_detected = len(breach_sources) > 0

recommended_policy_id = "MONPOL-BREACH"
latest_monpol = "n/a"
changelog_path = root / "governance" / "performance" / "MONITOR_THRESHOLD_CHANGELOG.md"
if changelog_path.exists():
    text = changelog_path.read_text(encoding="utf-8")
    import re
    matches = list(re.finditer(r"MONPOL-(\d{3})", text))
    if matches:
        latest_monpol = f"MONPOL-{int(matches[-1].group(1)):03d}"

evidence_bundle = [
    rel(escalation_path),
    rel(handoff_path),
    rel(drill_path),
]

owner_profile = escalation.get("overall_owner_profile", {})
if not isinstance(owner_profile, dict):
    owner_profile = {}

recommended_actions = []
if breach_detected:
    recommended_actions.extend(
        [
            "Include breach route artifact in PR evidence bundle.",
            "Document mitigation plan and owner accountability in PR body.",
            "Link active MONPOL ID and changelog rationale.",
            "Confirm drill and handoff status in transition pack summary.",
        ]
    )
else:
    recommended_actions.append("Continue routine governance cadence; no active breach route.")

would_block_in_active_mode = breach_detected and enforce_on_breach

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Monitor Drift Breach Evidence Route\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Breach detected**: **{yes_no(breach_detected)}**  \n")
    fh.write(f"**Active promotion mode**: `{active_mode}`  \n")
    fh.write(f"**Would block in active mode**: **{yes_no(would_block_in_active_mode)}**\n\n")

    fh.write("## Inputs\n\n")
    fh.write(f"- Escalation artifact: `{rel(escalation_path)}`\n")
    fh.write(f"- Handoff artifact: `{rel(handoff_path)}`\n")
    fh.write(f"- Drill artifact: `{rel(drill_path)}`\n")
    fh.write(f"- Promotion policy: `{rel(policy_path)}`\n\n")

    fh.write("## Breach Classification\n\n")
    fh.write(f"- Escalation level: `{overall_level}`\n")
    fh.write(f"- Handoff status: `{handoff_status}`\n")
    fh.write(f"- Drill status: `{drill_status}`\n")
    if breach_sources:
        fh.write(f"- Breach sources: {', '.join(f'`{item}`' for item in breach_sources)}\n")
    else:
        fh.write("- Breach sources: none\n")
    fh.write("\n")

    fh.write("## Promotion Snapshot\n\n")
    fh.write(f"- Policy mode from file: `{policy_mode}`\n")
    fh.write(f"- Active mode used: `{active_mode}`\n")
    fh.write(f"- enforce_on_breach: `{yes_no(enforce_on_breach)}`\n")
    fh.write(f"- require_pr_mitigation: `{yes_no(require_pr_mitigation)}`\n")
    fh.write(f"- require_breach_ack_token: `{yes_no(require_breach_ack_token)}`\n\n")

    fh.write("## Routing Targets\n\n")
    fh.write("| Target | Purpose |\n")
    fh.write("|---|---|\n")
    fh.write("| `governance/performance/MONITOR_THRESHOLD_CHANGELOG.md` | Breach rationale and decision traceability |\n")
    fh.write("| `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md` | Policy profile updates for threshold calibration |\n")
    fh.write("| `governance/performance/MONPOL_SIGNOFFS.json` | Reviewer/owner signoff for approved decisions |\n")
    fh.write("| PR body/title metadata | MONPOL linkage, mitigation notes, breach acknowledgment |\n\n")

    fh.write("## Recommended Actions\n\n")
    for action in recommended_actions:
        fh.write(f"- {action}\n")
    fh.write("\n")

    fh.write("## Evidence Bundle\n\n")
    for item in evidence_bundle:
        fh.write(f"- `{item}`\n")
    fh.write("\n")

    fh.write("## Owner Snapshot\n\n")
    fh.write(f"- Owner: {owner_profile.get('owner', 'n/a')}\n")
    fh.write(f"- Backup owner: {owner_profile.get('backup_owner', 'n/a')}\n")
    fh.write(f"- Response SLA (hours): {owner_profile.get('response_sla_hours', 'n/a')}\n")
    fh.write(f"- Latest MONPOL in changelog: `{latest_monpol}`\n")
    fh.write(f"- Recommended breach policy id marker: `{recommended_policy_id}`\n")

payload = {
    "generated_at_utc": generated_at,
    "inputs": {
        "escalation_json": rel(escalation_path),
        "handoff_json": rel(handoff_path),
        "drill_json": rel(drill_path),
        "promotion_policy_json": rel(policy_path),
    },
    "breach_detected": breach_detected,
    "breach_sources": breach_sources,
    "classification": {
        "escalation_level": overall_level,
        "handoff_status": handoff_status,
        "drill_status": drill_status,
    },
    "promotion": {
        "policy_mode": policy_mode,
        "active_mode": active_mode,
        "enforce_on_breach": enforce_on_breach,
        "require_pr_mitigation": require_pr_mitigation,
        "require_breach_ack_token": require_breach_ack_token,
        "would_block_in_active_mode": would_block_in_active_mode,
    },
    "routing_targets": [
        "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md",
        "docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md",
        "governance/performance/MONPOL_SIGNOFFS.json",
        "pull_request_metadata",
    ],
    "recommended_actions": recommended_actions,
    "owner_snapshot": {
        "owner": owner_profile.get("owner", "n/a"),
        "backup_owner": owner_profile.get("backup_owner", "n/a"),
        "response_sla_hours": owner_profile.get("response_sla_hours", "n/a"),
    },
    "evidence_bundle": evidence_bundle,
    "latest_monpol_id": latest_monpol,
    "recommended_policy_id": recommended_policy_id,
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Monitor drift breach route markdown: {output_path}")
print(f"Monitor drift breach route JSON: {output_json_path}")
print(f"Breach detected: {'yes' if breach_detected else 'no'}")
print(f"Active mode: {active_mode}")
print(f"Would block in active mode: {'yes' if would_block_in_active_mode else 'no'}")

if fail_on_breach and would_block_in_active_mode:
    print("Breach route policy indicates blocking enforcement in current mode", file=sys.stderr)
    raise SystemExit(2)
PY
