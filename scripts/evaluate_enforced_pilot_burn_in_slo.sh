#!/usr/bin/env bash
# Evaluate enforced pilot burn-in telemetry SLO.
# Usage:
#   ./scripts/evaluate_enforced_pilot_burn_in_slo.sh
#   ./scripts/evaluate_enforced_pilot_burn_in_slo.sh --window-artifacts 5 --fail-on-slo-breach

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
PROMOTION_POLICY_JSON="$ROOT/governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json"
WINDOW_ARTIFACTS=""
FAIL_ON_SLO_BREACH=0
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/evaluate_enforced_pilot_burn_in_slo.sh [options]

Options:
  --analysis-dir <path>            Analysis directory (default: analysis/benchmark_reproducibility)
  --promotion-policy-json <path>   Governance gate promotion policy JSON path
  --window-artifacts <n>           Override burn-in rolling artifact window size
  --fail-on-slo-breach             Exit non-zero when burn-in SLO criteria fail
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
    --window-artifacts)
      WINDOW_ARTIFACTS="${2:-}"
      shift 2
      ;;
    --fail-on-slo-breach)
      FAIL_ON_SLO_BREACH=1
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

if [[ -n "$WINDOW_ARTIFACTS" ]]; then
  if ! [[ "$WINDOW_ARTIFACTS" =~ ^[0-9]+$ ]] || (( WINDOW_ARTIFACTS < 1 )); then
    echo "Error: --window-artifacts must be integer >= 1" >&2
    exit 1
  fi
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/enforced_pilot_burn_in_slo_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - \
  "$ROOT" \
  "$ANALYSIS_DIR" \
  "$PROMOTION_POLICY_JSON" \
  "$WINDOW_ARTIFACTS" \
  "$FAIL_ON_SLO_BREACH" \
  "$OUTPUT_PATH" \
  "$OUTPUT_JSON_PATH" <<'PY'
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
analysis_dir = Path(sys.argv[2])
policy_path = Path(sys.argv[3])
window_artifacts_arg = sys.argv[4].strip()
fail_on_slo_breach = bool(int(sys.argv[5]))
output_path = Path(sys.argv[6])
output_json_path = Path(sys.argv[7])


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


def yes_no(value: bool) -> str:
    return "yes" if value else "no"


policy_payload = load_json(policy_path)
pilot_cfg = policy_payload.get("pilot_rollout", {})
if not isinstance(pilot_cfg, dict):
    pilot_cfg = {}
burn_in_cfg = pilot_cfg.get("burn_in_slo", {})
if not isinstance(burn_in_cfg, dict):
    burn_in_cfg = {}

window_artifacts = safe_int(window_artifacts_arg, safe_int(burn_in_cfg.get("window_artifacts"), 5)) if window_artifacts_arg else safe_int(burn_in_cfg.get("window_artifacts"), 5)
window_artifacts = max(1, window_artifacts)

min_samples = max(1, safe_int(burn_in_cfg.get("min_samples"), 1))
max_rollback_recommendations = max(0, safe_int(burn_in_cfg.get("max_rollback_recommendations"), 0))
max_guardrail_breach_events = max(0, safe_int(burn_in_cfg.get("max_guardrail_breach_events"), 0))
max_preflight_failures = max(0, safe_int(burn_in_cfg.get("max_preflight_failures"), 0))
max_breach_detected_events = max(0, safe_int(burn_in_cfg.get("max_breach_detected_events"), 0))
require_latest_operator_decision_go = safe_bool(burn_in_cfg.get("require_latest_operator_decision_go"), True)
allowed_latest_stages = burn_in_cfg.get("allowed_latest_runbook_stages", ["preflight", "enforced_pilot"])
if not isinstance(allowed_latest_stages, list) or not allowed_latest_stages:
    allowed_latest_stages = ["preflight", "enforced_pilot"]
allowed_latest_stages = [str(item) for item in allowed_latest_stages if str(item).strip()]
if not allowed_latest_stages:
    allowed_latest_stages = ["preflight", "enforced_pilot"]

runbook_candidates = sorted(analysis_dir.glob("enforced_pilot_runbook_[0-9]*.json"))
window_rows = runbook_candidates[-window_artifacts:]

rows = []
for path in window_rows:
    payload = load_json(path)
    rows.append(
        {
            "path": path,
            "rollback_recommended": bool(payload.get("rollback_recommended", False)),
            "preflight_ok": bool(payload.get("preflight_ok", False)),
            "guardrail_breach_count": len(payload.get("guardrail_breaches", []))
            if isinstance(payload.get("guardrail_breaches", []), list)
            else 0,
            "runbook_stage": str(payload.get("runbook_stage", "n/a")),
            "operator_decision": str(payload.get("operator_decision", "n/a")),
            "breach_detected": bool(
                (payload.get("telemetry_snapshot", {}) if isinstance(payload.get("telemetry_snapshot", {}), dict) else {}).get(
                    "breach_detected", False
                )
            ),
        }
    )

samples = len(rows)
rollback_recommendations = sum(1 for row in rows if row["rollback_recommended"])
guardrail_breach_events = sum(1 for row in rows if row["guardrail_breach_count"] > 0)
preflight_failures = sum(1 for row in rows if not row["preflight_ok"])
breach_detected_events = sum(1 for row in rows if row["breach_detected"])
safe_runs = samples - rollback_recommendations
safe_rate_pct = (100.0 * safe_runs / samples) if samples else 0.0

latest_row = rows[-1] if rows else None
latest_stage = latest_row["runbook_stage"] if latest_row else "n/a"
latest_operator_decision = latest_row["operator_decision"] if latest_row else "n/a"

criteria = [
    {
        "id": "samples_sufficient",
        "description": f"Burn-in runbook samples >= {min_samples}",
        "value": samples,
        "target": f">= {min_samples}",
        "passed": samples >= min_samples,
    },
    {
        "id": "rollback_recommendations_within_limit",
        "description": f"Rollback recommendations <= {max_rollback_recommendations}",
        "value": rollback_recommendations,
        "target": f"<= {max_rollback_recommendations}",
        "passed": rollback_recommendations <= max_rollback_recommendations,
    },
    {
        "id": "guardrail_breach_events_within_limit",
        "description": f"Guardrail breach events <= {max_guardrail_breach_events}",
        "value": guardrail_breach_events,
        "target": f"<= {max_guardrail_breach_events}",
        "passed": guardrail_breach_events <= max_guardrail_breach_events,
    },
    {
        "id": "preflight_failures_within_limit",
        "description": f"Preflight failures <= {max_preflight_failures}",
        "value": preflight_failures,
        "target": f"<= {max_preflight_failures}",
        "passed": preflight_failures <= max_preflight_failures,
    },
    {
        "id": "breach_detected_events_within_limit",
        "description": f"Breach-detected runbook snapshots <= {max_breach_detected_events}",
        "value": breach_detected_events,
        "target": f"<= {max_breach_detected_events}",
        "passed": breach_detected_events <= max_breach_detected_events,
    },
    {
        "id": "latest_stage_allowed",
        "description": "Latest runbook stage is allowed by burn-in policy.",
        "value": latest_stage,
        "target": ",".join(allowed_latest_stages),
        "passed": latest_stage in set(allowed_latest_stages),
    },
    {
        "id": "latest_operator_decision_go",
        "description": "Latest operator decision is go when required.",
        "value": latest_operator_decision,
        "target": "go" if require_latest_operator_decision_go else "advisory",
        "passed": (latest_operator_decision == "go") if require_latest_operator_decision_go else True,
    },
]

required_failures = [item for item in criteria if not item["passed"]]
overall_status = "pass" if not required_failures else "fail"

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Enforced Pilot Burn-In SLO Assessment\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Overall status**: **{overall_status}**  \n")
    fh.write(f"**Window artifacts**: {window_artifacts}  \n")
    fh.write(f"**Samples analyzed**: {samples}  \n")
    fh.write(f"**Safe run rate (%)**: {safe_rate_pct:.2f}\n\n")

    fh.write("## Burn-In SLO Configuration\n\n")
    fh.write(f"- min_samples: {min_samples}\n")
    fh.write(f"- max_rollback_recommendations: {max_rollback_recommendations}\n")
    fh.write(f"- max_guardrail_breach_events: {max_guardrail_breach_events}\n")
    fh.write(f"- max_preflight_failures: {max_preflight_failures}\n")
    fh.write(f"- max_breach_detected_events: {max_breach_detected_events}\n")
    fh.write(f"- allowed_latest_runbook_stages: {', '.join(allowed_latest_stages)}\n")
    fh.write(f"- require_latest_operator_decision_go: {yes_no(require_latest_operator_decision_go)}\n\n")

    fh.write("## Telemetry Summary\n\n")
    fh.write(f"- rollback_recommendations: {rollback_recommendations}\n")
    fh.write(f"- guardrail_breach_events: {guardrail_breach_events}\n")
    fh.write(f"- preflight_failures: {preflight_failures}\n")
    fh.write(f"- breach_detected_events: {breach_detected_events}\n")
    fh.write(f"- latest_stage: `{latest_stage}`\n")
    fh.write(f"- latest_operator_decision: `{latest_operator_decision}`\n\n")

    fh.write("## Criteria\n\n")
    fh.write("| Criterion | Value | Target | Passed |\n")
    fh.write("|---|---:|---|---|\n")
    for item in criteria:
        fh.write(
            f"| `{item['id']}` | {item['value']} | {item['target']} | {'yes' if item['passed'] else 'no'} |\n"
        )
    fh.write("\n")

    if required_failures:
        fh.write("Failed criteria:\n")
        for item in required_failures:
            fh.write(f"- `{item['id']}` ({item['description']})\n")
        fh.write("\n")

    fh.write("## Window Artifacts\n\n")
    if rows:
        for row in rows:
            fh.write(f"- `{rel(row['path'])}`\n")
    else:
        fh.write("- none\n")

payload = {
    "generated_at_utc": generated_at,
    "inputs": {
        "analysis_dir": rel(analysis_dir),
        "promotion_policy_json": rel(policy_path),
        "window_artifacts_override": window_artifacts_arg or "n/a",
    },
    "burn_in_slo": {
        "window_artifacts": window_artifacts,
        "min_samples": min_samples,
        "max_rollback_recommendations": max_rollback_recommendations,
        "max_guardrail_breach_events": max_guardrail_breach_events,
        "max_preflight_failures": max_preflight_failures,
        "max_breach_detected_events": max_breach_detected_events,
        "allowed_latest_runbook_stages": allowed_latest_stages,
        "require_latest_operator_decision_go": require_latest_operator_decision_go,
    },
    "telemetry_summary": {
        "samples": samples,
        "rollback_recommendations": rollback_recommendations,
        "guardrail_breach_events": guardrail_breach_events,
        "preflight_failures": preflight_failures,
        "breach_detected_events": breach_detected_events,
        "safe_rate_pct": round(safe_rate_pct, 2),
        "latest_stage": latest_stage,
        "latest_operator_decision": latest_operator_decision,
    },
    "criteria": criteria,
    "required_failures": required_failures,
    "overall_status": overall_status,
    "window_artifacts": [rel(row["path"]) for row in rows],
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Enforced pilot burn-in SLO markdown: {output_path}")
print(f"Enforced pilot burn-in SLO JSON: {output_json_path}")
print(f"Overall status: {overall_status}")
print(f"Samples analyzed: {samples}")

if fail_on_slo_breach and overall_status != "pass":
    print("Enforced pilot burn-in SLO criteria failed", file=sys.stderr)
    raise SystemExit(2)
PY

