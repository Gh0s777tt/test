#!/usr/bin/env bash
# Run strict-mode release-readiness drill scenarios for escalation handoff checks.
# Usage:
#   ./scripts/run_monitor_drift_release_readiness_drill.sh
#   ./scripts/run_monitor_drift_release_readiness_drill.sh --require-transition-pack

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
ESCALATION_JSON=""
OWNERS_JSON="$ROOT/governance/performance/MONITOR_DRIFT_ESCALATION_OWNERS.json"
POLICY_DOC="$ROOT/governance/performance/MONITOR_DRIFT_ESCALATION_POLICY.md"
HANDOFF_SCRIPT="$ROOT/scripts/generate_monitor_drift_release_handoff.sh"
EXPECTED_BLOCKED_EXIT_CODE=2
REQUIRE_TRANSITION_PACK=0
SCENARIO_OUTPUT_DIR=""
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/run_monitor_drift_release_readiness_drill.sh [options]

Options:
  --analysis-dir <path>            Analysis directory (default: analysis/benchmark_reproducibility)
  --escalation-json <path>         Escalation JSON artifact (default: newest monitor_drift_escalation_*.json)
  --owners-json <path>             Escalation owner/SLA registry JSON path
  --policy-doc <path>              Escalation policy markdown path
  --handoff-script <path>          Handoff checklist generator script path
  --expected-blocked-exit-code <n> Expected strict exit code in blocked scenario (default: 2)
  --require-transition-pack        Require transition pack in drill handoff checks
  --scenario-output-dir <path>     Directory for simulated/baseline/blocked scenario artifacts
  --output <path>                  Drill output markdown path
  --output-json <path>             Drill output JSON path
  -h, --help                       Show this help
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
    --handoff-script)
      HANDOFF_SCRIPT="${2:-}"
      shift 2
      ;;
    --expected-blocked-exit-code)
      EXPECTED_BLOCKED_EXIT_CODE="${2:-}"
      shift 2
      ;;
    --require-transition-pack)
      REQUIRE_TRANSITION_PACK=1
      shift
      ;;
    --scenario-output-dir)
      SCENARIO_OUTPUT_DIR="${2:-}"
      shift 2
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

if [[ ! -f "$OWNERS_JSON" ]]; then
  echo "Error: owners JSON not found: $OWNERS_JSON" >&2
  exit 1
fi

if [[ ! -f "$POLICY_DOC" ]]; then
  echo "Error: policy doc not found: $POLICY_DOC" >&2
  exit 1
fi

if [[ ! -x "$HANDOFF_SCRIPT" ]]; then
  echo "Error: handoff script missing or not executable: $HANDOFF_SCRIPT" >&2
  exit 1
fi

if [[ -z "$ESCALATION_JSON" ]]; then
  shopt -s nullglob
  escalation_candidates=("$ANALYSIS_DIR"/monitor_drift_escalation_[0-9]*.json)
  shopt -u nullglob
  if (( ${#escalation_candidates[@]} == 0 )); then
    echo "Error: no canonical monitor_drift_escalation_<timestamp>.json found in $ANALYSIS_DIR" >&2
    exit 1
  fi
  readarray -t sorted_escalation < <(printf '%s\n' "${escalation_candidates[@]}" | sort)
  ESCALATION_JSON="${sorted_escalation[${#sorted_escalation[@]}-1]}"
fi

if [[ ! -f "$ESCALATION_JSON" ]]; then
  echo "Error: escalation JSON not found: $ESCALATION_JSON" >&2
  exit 1
fi

if ! [[ "$EXPECTED_BLOCKED_EXIT_CODE" =~ ^[0-9]+$ ]]; then
  echo "Error: --expected-blocked-exit-code must be integer >= 0" >&2
  exit 1
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/monitor_drift_release_readiness_drill_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

if [[ -z "$SCENARIO_OUTPUT_DIR" ]]; then
  SCENARIO_OUTPUT_DIR="$ANALYSIS_DIR"
fi

mkdir -p "$SCENARIO_OUTPUT_DIR"

TIMESTAMP_TAG="$(basename "$OUTPUT_PATH" | sed -E 's/.*_([0-9]{8}T[0-9]{6}Z)\.md$/\1/')"
if [[ -z "$TIMESTAMP_TAG" || "$TIMESTAMP_TAG" == "$(basename "$OUTPUT_PATH")" ]]; then
  TIMESTAMP_TAG="$(date -u +%Y%m%dT%H%M%SZ)"
fi

SIM_ESCALATION_JSON="$SCENARIO_OUTPUT_DIR/monitor_drift_escalation_simulated_escalated_${TIMESTAMP_TAG}.json"
BASELINE_MD="$SCENARIO_OUTPUT_DIR/monitor_drift_release_handoff_dryrun_baseline_${TIMESTAMP_TAG}.md"
BASELINE_JSON="$SCENARIO_OUTPUT_DIR/monitor_drift_release_handoff_dryrun_baseline_${TIMESTAMP_TAG}.json"
BLOCKED_MD="$SCENARIO_OUTPUT_DIR/monitor_drift_release_handoff_dryrun_blocked_${TIMESTAMP_TAG}.md"
BLOCKED_JSON="$SCENARIO_OUTPUT_DIR/monitor_drift_release_handoff_dryrun_blocked_${TIMESTAMP_TAG}.json"

python3 - "$OWNERS_JSON" "$SIM_ESCALATION_JSON" <<'PY'
import json
import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path

owners_path = Path(sys.argv[1])
sim_output_path = Path(sys.argv[2])
owners = json.loads(owners_path.read_text(encoding="utf-8"))
levels = owners.get("levels", {})
if not isinstance(levels, dict):
    levels = {}

fallback = {
    "owner": "n/a",
    "backup_owner": "n/a",
    "response_sla_hours": 0,
    "drill_cadence_days": 0,
    "release_handoff_required": True,
}
profile = levels.get("escalated", {})
if not isinstance(profile, dict):
    profile = {}
merged = {**fallback, **profile}

now = datetime.now(timezone.utc).replace(microsecond=0)
next_drill_due = "n/a"
try:
    cadence_days = int(merged.get("drill_cadence_days", 0) or 0)
except (TypeError, ValueError):
    cadence_days = 0
if cadence_days > 0:
    next_drill_due = (now + timedelta(days=cadence_days)).isoformat().replace("+00:00", "Z")

payload = {
    "generated_at_utc": now.isoformat().replace("+00:00", "Z"),
    "dashboard_source": "simulated/escalation_dry_run",
    "owners_config_source": str(owners_path),
    "overall_level": "escalated",
    "level_counts": {
        "normal": 0,
        "watch": 0,
        "escalated": 1,
        "critical": 0,
    },
    "rows": [
        {
            "bench": "simulated_release_readiness_benchmark",
            "samples": 5,
            "latest_status": "drift",
            "drift_rate_pct": 42.0,
            "failure_rate_pct": 12.0,
            "drift_count_window": 3,
            "failure_count_window": 1,
            "consecutive_drift": 2,
            "level": "escalated",
            "triggers": ["simulated_dry_run"],
            "required_action": "Open/update MONPOL proposal with owner and mitigation plan.",
            "owner": str(merged.get("owner", "n/a")),
            "backup_owner": str(merged.get("backup_owner", "n/a")),
            "response_sla_hours": int(merged.get("response_sla_hours", 0) or 0),
            "drill_cadence_days": cadence_days,
            "release_handoff_required": bool(merged.get("release_handoff_required", True)),
        }
    ],
    "escalated_benches": ["simulated_release_readiness_benchmark"],
    "critical_benches": [],
    "overall_owner_profile": {
        "owner": str(merged.get("owner", "n/a")),
        "backup_owner": str(merged.get("backup_owner", "n/a")),
        "response_sla_hours": int(merged.get("response_sla_hours", 0) or 0),
        "drill_cadence_days": cadence_days,
        "release_handoff_required": bool(merged.get("release_handoff_required", True)),
        "source_level": "escalated",
    },
    "next_drill_due_utc": next_drill_due,
    "fail_on_level": "none",
    "fail_triggered": False,
    "window": {"configured_runs": 5, "evaluated_runs": 5},
}
sim_output_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY

run_handoff() {
  local analysis_dir="$1"
  local escalation_json="$2"
  local out_md="$3"
  local out_json="$4"

  local args=(
    "--analysis-dir" "$analysis_dir"
    "--escalation-json" "$escalation_json"
    "--owners-json" "$OWNERS_JSON"
    "--policy-doc" "$POLICY_DOC"
    "--strict"
    "--output" "$out_md"
    "--output-json" "$out_json"
  )
  if (( REQUIRE_TRANSITION_PACK == 1 )); then
    args+=("--require-transition-pack")
  fi

  set +e
  "$HANDOFF_SCRIPT" "${args[@]}" >/dev/null 2>&1
  local exit_code=$?
  set -e
  echo "$exit_code"
}

BASELINE_EXIT_CODE="$(run_handoff "$ANALYSIS_DIR" "$ESCALATION_JSON" "$BASELINE_MD" "$BASELINE_JSON")"

TMP_EMPTY_ANALYSIS="$(mktemp -d /tmp/vantis_release_readiness_drill_XXXXXX)"
BLOCKED_EXIT_CODE="$(run_handoff "$TMP_EMPTY_ANALYSIS" "$SIM_ESCALATION_JSON" "$BLOCKED_MD" "$BLOCKED_JSON")"
rm -rf "$TMP_EMPTY_ANALYSIS"

python3 - \
  "$ROOT" \
  "$OUTPUT_PATH" \
  "$OUTPUT_JSON_PATH" \
  "$ESCALATION_JSON" \
  "$SIM_ESCALATION_JSON" \
  "$BASELINE_MD" \
  "$BASELINE_JSON" \
  "$BLOCKED_MD" \
  "$BLOCKED_JSON" \
  "$BASELINE_EXIT_CODE" \
  "$BLOCKED_EXIT_CODE" \
  "$EXPECTED_BLOCKED_EXIT_CODE" \
  "$REQUIRE_TRANSITION_PACK" <<'PY'
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
output_md = Path(sys.argv[2])
output_json = Path(sys.argv[3])
real_escalation_json = Path(sys.argv[4])
sim_escalation_json = Path(sys.argv[5])
baseline_md = Path(sys.argv[6])
baseline_json = Path(sys.argv[7])
blocked_md = Path(sys.argv[8])
blocked_json = Path(sys.argv[9])
baseline_exit = int(sys.argv[10])
blocked_exit = int(sys.argv[11])
expected_blocked_exit = int(sys.argv[12])
require_transition_pack = bool(int(sys.argv[13]))


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(root.resolve()))
    except ValueError:
        return str(path)


def scenario_status(ok: bool) -> str:
    return "pass" if ok else "fail"


baseline_ok = baseline_exit == 0
blocked_ok = blocked_exit == expected_blocked_exit
overall_ok = baseline_ok and blocked_ok
overall_status = "pass" if overall_ok else "fail"

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_md.parent.mkdir(parents=True, exist_ok=True)
output_json.parent.mkdir(parents=True, exist_ok=True)

scenarios = [
    {
        "name": "strict_baseline_real",
        "description": "Strict handoff against latest real escalation artifact.",
        "expected_exit_code": 0,
        "actual_exit_code": baseline_exit,
        "status": scenario_status(baseline_ok),
        "outputs": {
            "markdown": rel(baseline_md),
            "json": rel(baseline_json),
        },
    },
    {
        "name": "strict_blocked_simulated_escalated",
        "description": "Strict handoff against simulated escalated artifact with missing governance artifacts.",
        "expected_exit_code": expected_blocked_exit,
        "actual_exit_code": blocked_exit,
        "status": scenario_status(blocked_ok),
        "outputs": {
            "markdown": rel(blocked_md),
            "json": rel(blocked_json),
        },
    },
]

with output_md.open("w", encoding="utf-8") as fh:
    fh.write("# Monitor Drift Release Readiness Drill\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Overall drill status**: **{overall_status}**  \n")
    fh.write(f"**Transition pack required in handoff checks**: {'yes' if require_transition_pack else 'no'}\n\n")

    fh.write("## Inputs\n\n")
    fh.write(f"- Real escalation artifact: `{rel(real_escalation_json)}`\n")
    fh.write(f"- Simulated escalation artifact: `{rel(sim_escalation_json)}`\n")
    fh.write(f"- Expected blocked strict exit code: `{expected_blocked_exit}`\n\n")

    fh.write("## Scenarios\n\n")
    fh.write("| Scenario | Expected exit | Actual exit | Status | Output markdown | Output json |\n")
    fh.write("|---|---:|---:|---|---|---|\n")
    for item in scenarios:
        fh.write(
            f"| `{item['name']}` | {item['expected_exit_code']} | {item['actual_exit_code']} | {item['status']} | "
            f"`{item['outputs']['markdown']}` | `{item['outputs']['json']}` |\n"
        )
    fh.write("\n")

    if not overall_ok:
        fh.write("## Drill Failures\n\n")
        if not baseline_ok:
            fh.write("- `strict_baseline_real` failed: strict run did not exit with code 0.\n")
        if not blocked_ok:
            fh.write(
                "- `strict_blocked_simulated_escalated` failed: blocked strict run did not return expected exit code.\n"
            )
        fh.write("\n")

    fh.write("This drill is successful only when strict baseline passes and blocked strict simulation fails as expected.\n")

payload = {
    "generated_at_utc": generated_at,
    "overall_status": overall_status,
    "require_transition_pack": require_transition_pack,
    "expected_blocked_exit_code": expected_blocked_exit,
    "inputs": {
        "real_escalation_json": rel(real_escalation_json),
        "simulated_escalation_json": rel(sim_escalation_json),
    },
    "scenarios": scenarios,
}
output_json.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Release readiness drill markdown: {output_md}")
print(f"Release readiness drill JSON: {output_json}")
print(f"Overall drill status: {overall_status}")

if not overall_ok:
    raise SystemExit(2)
PY

