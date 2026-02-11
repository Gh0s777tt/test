#!/usr/bin/env bash
# Generate monitor drift release handoff checklist from escalation telemetry.
# Usage:
#   ./scripts/generate_monitor_drift_release_handoff.sh
#   ./scripts/generate_monitor_drift_release_handoff.sh --strict

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
ESCALATION_JSON=""
OWNERS_JSON="$ROOT/governance/performance/MONITOR_DRIFT_ESCALATION_OWNERS.json"
POLICY_DOC="$ROOT/governance/performance/MONITOR_DRIFT_ESCALATION_POLICY.md"
STRICT=0
REQUIRE_TRANSITION_PACK=0
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/generate_monitor_drift_release_handoff.sh [options]

Options:
  --analysis-dir <path>       Analysis directory (default: analysis/benchmark_reproducibility)
  --escalation-json <path>    Escalation JSON artifact (default: newest monitor_drift_escalation_*.json)
  --owners-json <path>        Escalation owner/SLA registry JSON path
  --policy-doc <path>         Escalation policy markdown path
  --strict                    Exit non-zero when required checklist items fail
  --require-transition-pack   Require transition pack artifact in strict checklist
  --output <path>             Output markdown path
  --output-json <path>        Output JSON path
  -h, --help                  Show this help
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
    --owners-json)
      OWNERS_JSON="${2:-}"
      shift 2
      ;;
    --policy-doc)
      POLICY_DOC="${2:-}"
      shift 2
      ;;
    --strict)
      STRICT=1
      shift
      ;;
    --require-transition-pack)
      REQUIRE_TRANSITION_PACK=1
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
  escalation_candidates=("$ANALYSIS_DIR"/monitor_drift_escalation_[0-9]*.json)
  shopt -u nullglob
  if (( ${#escalation_candidates[@]} > 0 )); then
    readarray -t sorted_escalation < <(printf '%s\n' "${escalation_candidates[@]}" | sort)
    ESCALATION_JSON="${sorted_escalation[${#sorted_escalation[@]}-1]}"
  fi
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/monitor_drift_release_handoff_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - "$ROOT" "$ANALYSIS_DIR" "$ESCALATION_JSON" "$OWNERS_JSON" "$POLICY_DOC" "$STRICT" "$REQUIRE_TRANSITION_PACK" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" <<'PY'
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
analysis_dir = Path(sys.argv[2])
escalation_path_raw = sys.argv[3].strip()
owners_path = Path(sys.argv[4])
policy_doc_path = Path(sys.argv[5])
strict = bool(int(sys.argv[6]))
require_transition_pack = bool(int(sys.argv[7]))
output_path = Path(sys.argv[8])
output_json_path = Path(sys.argv[9])


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(root.resolve()))
    except ValueError:
        return str(path)


def latest(glob: str):
    items = sorted(analysis_dir.glob(glob))
    return items[-1] if items else None


def load_json(path: Path):
    if not path or not path.exists():
        return {}
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError):
        return {}


def safe_int(value, default=0):
    try:
        return int(value)
    except (TypeError, ValueError):
        return default


def owner_profile_for_level(level: str, owners_cfg: dict):
    levels = owners_cfg.get("levels", {})
    if not isinstance(levels, dict):
        levels = {}
    default_level = str(owners_cfg.get("default_level", "normal"))
    source_level = level if level in levels else default_level
    source = levels.get(source_level, {})
    if not isinstance(source, dict):
        source = {}
    return {
        "owner": str(source.get("owner", "n/a")),
        "backup_owner": str(source.get("backup_owner", "n/a")),
        "response_sla_hours": safe_int(source.get("response_sla_hours"), 0),
        "drill_cadence_days": safe_int(source.get("drill_cadence_days"), 0),
        "release_handoff_required": bool(source.get("release_handoff_required", False)),
        "source_level": source_level,
    }


escalation_path = Path(escalation_path_raw) if escalation_path_raw else None
escalation_payload = load_json(escalation_path) if escalation_path else {}
owners_payload = load_json(owners_path)

overall_level = str(escalation_payload.get("overall_level", "n/a"))
overall_owner_profile = escalation_payload.get("overall_owner_profile")
if not isinstance(overall_owner_profile, dict):
    overall_owner_profile = owner_profile_for_level(overall_level, owners_payload)

next_drill_due_utc = str(escalation_payload.get("next_drill_due_utc", "n/a"))
release_handoff_required = bool(overall_owner_profile.get("release_handoff_required", False))

latest_dashboard_json = latest("monitor_policy_dashboard_*.json")
latest_proposal_json = latest("monitor_threshold_proposal_MONPOL-*_*.json")
latest_transition_json = latest("governance_transition_pack_*.json")
latest_signoff_validation_json = latest("monpol_signoff_validation_*.json")

items = []

def add_item(item_id: str, description: str, required: bool, ok: bool, details: str):
    items.append(
        {
            "id": item_id,
            "description": description,
            "required": required,
            "status": "pass" if ok else ("fail" if required else "advisory"),
            "details": details,
        }
    )


add_item(
    "policy_doc_present",
    "Escalation policy document is present.",
    True,
    policy_doc_path.exists(),
    rel(policy_doc_path),
)

add_item(
    "owners_registry_present",
    "Escalation owner/SLA registry JSON is present.",
    True,
    owners_path.exists(),
    rel(owners_path),
)

add_item(
    "escalation_artifact_present",
    "Escalation assessment artifact is available.",
    True,
    bool(escalation_payload),
    rel(escalation_path) if escalation_path else "n/a",
)

owner_assigned = str(overall_owner_profile.get("owner", "n/a")) not in {"", "n/a"}
sla_defined = int(overall_owner_profile.get("response_sla_hours", 0) or 0) > 0

add_item(
    "overall_owner_assigned",
    "Overall escalation owner and SLA are defined.",
    True,
    owner_assigned and sla_defined,
    f"owner={overall_owner_profile.get('owner', 'n/a')}, sla_hours={overall_owner_profile.get('response_sla_hours', 'n/a')}",
)

drill_defined = int(overall_owner_profile.get("drill_cadence_days", 0) or 0) > 0 and next_drill_due_utc != "n/a"
add_item(
    "drill_due_defined",
    "Next escalation drill due time is defined.",
    True,
    drill_defined,
    f"next_drill_due_utc={next_drill_due_utc}",
)

add_item(
    "dashboard_artifact_present",
    "Latest monitor policy dashboard artifact is present.",
    True,
    bool(latest_dashboard_json),
    rel(latest_dashboard_json) if latest_dashboard_json else "n/a",
)

add_item(
    "proposal_artifact_present",
    "MONPOL proposal artifact present when release handoff is required.",
    release_handoff_required,
    bool(latest_proposal_json),
    rel(latest_proposal_json) if latest_proposal_json else "n/a",
)

add_item(
    "transition_pack_present",
    "Governance transition pack artifact present when release handoff is required.",
    release_handoff_required and require_transition_pack,
    bool(latest_transition_json),
    rel(latest_transition_json) if latest_transition_json else "n/a",
)

add_item(
    "signoff_validation_present",
    "Signoff validation artifact present when release handoff is required.",
    release_handoff_required,
    bool(latest_signoff_validation_json),
    rel(latest_signoff_validation_json) if latest_signoff_validation_json else "n/a",
)

required_failures = [item for item in items if item["required"] and item["status"] == "fail"]
overall_status = "ready" if not required_failures else "blocked"

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Monitor Drift Release Handoff Checklist\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Escalation level**: `{overall_level}`  \n")
    fh.write(f"**Overall handoff status**: **{overall_status}**\n\n")

    fh.write("## Owner and SLA Snapshot\n\n")
    fh.write(f"- Owner: {overall_owner_profile.get('owner', 'n/a')}\n")
    fh.write(f"- Backup owner: {overall_owner_profile.get('backup_owner', 'n/a')}\n")
    fh.write(f"- Response SLA (hours): {overall_owner_profile.get('response_sla_hours', 'n/a')}\n")
    fh.write(f"- Drill cadence (days): {overall_owner_profile.get('drill_cadence_days', 'n/a')}\n")
    fh.write(f"- Release handoff required: {'yes' if release_handoff_required else 'no'}\n")
    fh.write(f"- Transition pack required by checklist: {'yes' if require_transition_pack else 'no'}\n")
    fh.write(f"- Next drill due (UTC): {next_drill_due_utc}\n\n")

    fh.write("## Checklist\n\n")
    fh.write("| ID | Required | Status | Description | Details |\n")
    fh.write("|---|---|---|---|---|\n")
    for item in items:
        fh.write(
            f"| `{item['id']}` | {'yes' if item['required'] else 'no'} | {item['status']} | "
            f"{item['description']} | {item['details']} |\n"
        )
    fh.write("\n")

    if required_failures:
        fh.write("## Blocking Items\n\n")
        for item in required_failures:
            fh.write(f"- `{item['id']}`: {item['description']} ({item['details']})\n")
        fh.write("\n")

    fh.write("Checklist is advisory unless --strict is used.\n")

payload = {
    "generated_at_utc": generated_at,
    "overall_status": overall_status,
    "overall_level": overall_level,
    "escalation_artifact": rel(escalation_path) if escalation_path else "n/a",
    "owners_registry": rel(owners_path),
    "policy_doc": rel(policy_doc_path),
    "overall_owner_profile": overall_owner_profile,
    "next_drill_due_utc": next_drill_due_utc,
    "release_handoff_required": release_handoff_required,
    "require_transition_pack": require_transition_pack,
    "latest_artifacts": {
        "dashboard_json": rel(latest_dashboard_json) if latest_dashboard_json else "n/a",
        "proposal_json": rel(latest_proposal_json) if latest_proposal_json else "n/a",
        "transition_pack_json": rel(latest_transition_json) if latest_transition_json else "n/a",
        "signoff_validation_json": rel(latest_signoff_validation_json) if latest_signoff_validation_json else "n/a",
    },
    "items": items,
    "required_failures": required_failures,
    "strict_mode": strict,
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Monitor drift release handoff markdown: {output_path}")
print(f"Monitor drift release handoff JSON: {output_json_path}")
print(f"Overall handoff status: {overall_status}")

if strict and required_failures:
    print("Release handoff strict mode failure: required checklist items are missing", file=sys.stderr)
    raise SystemExit(2)
PY

