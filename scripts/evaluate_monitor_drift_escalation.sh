#!/usr/bin/env bash
# Evaluate monitor drift escalation levels from dashboard telemetry.
# Usage:
#   ./scripts/evaluate_monitor_drift_escalation.sh
#   ./scripts/evaluate_monitor_drift_escalation.sh --window-runs 6 --fail-on-level critical

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
DASHBOARD_JSON=""
WINDOW_RUNS=5

WATCH_DRIFT_RATE_PCT=20
ESCALATE_DRIFT_RATE_PCT=40
CRITICAL_DRIFT_RATE_PCT=60

ESCALATE_CONSECUTIVE_DRIFT=2
CRITICAL_CONSECUTIVE_DRIFT=3

ESCALATE_FAILURE_RATE_PCT=10
CRITICAL_FAILURE_RATE_PCT=20

FAIL_ON_LEVEL="none"
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/evaluate_monitor_drift_escalation.sh [options]

Options:
  --analysis-dir <path>                  Analysis directory (default: analysis/benchmark_reproducibility)
  --dashboard-json <path>                Dashboard JSON file (default: newest monitor_policy_dashboard_*.json)
  --window-runs <n>                      Recent runs window for streak analysis (default: 5)
  --watch-drift-rate-pct <pct>           Watch drift-rate threshold (default: 20)
  --escalate-drift-rate-pct <pct>        Escalated drift-rate threshold (default: 40)
  --critical-drift-rate-pct <pct>        Critical drift-rate threshold (default: 60)
  --escalate-consecutive-drift <n>       Escalated consecutive drift threshold (default: 2)
  --critical-consecutive-drift <n>       Critical consecutive drift threshold (default: 3)
  --escalate-failure-rate-pct <pct>      Escalated failure-rate threshold (default: 10)
  --critical-failure-rate-pct <pct>      Critical failure-rate threshold (default: 20)
  --fail-on-level <none|watch|escalated|critical>
                                          Exit non-zero if overall level >= this value (default: none)
  --output <path>                        Output markdown path
  --output-json <path>                   Output JSON path
  -h, --help                             Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --dashboard-json)
      DASHBOARD_JSON="${2:-}"
      shift 2
      ;;
    --window-runs)
      WINDOW_RUNS="${2:-}"
      shift 2
      ;;
    --watch-drift-rate-pct)
      WATCH_DRIFT_RATE_PCT="${2:-}"
      shift 2
      ;;
    --escalate-drift-rate-pct)
      ESCALATE_DRIFT_RATE_PCT="${2:-}"
      shift 2
      ;;
    --critical-drift-rate-pct)
      CRITICAL_DRIFT_RATE_PCT="${2:-}"
      shift 2
      ;;
    --escalate-consecutive-drift)
      ESCALATE_CONSECUTIVE_DRIFT="${2:-}"
      shift 2
      ;;
    --critical-consecutive-drift)
      CRITICAL_CONSECUTIVE_DRIFT="${2:-}"
      shift 2
      ;;
    --escalate-failure-rate-pct)
      ESCALATE_FAILURE_RATE_PCT="${2:-}"
      shift 2
      ;;
    --critical-failure-rate-pct)
      CRITICAL_FAILURE_RATE_PCT="${2:-}"
      shift 2
      ;;
    --fail-on-level)
      FAIL_ON_LEVEL="${2:-}"
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

if ! [[ "$WINDOW_RUNS" =~ ^[0-9]+$ ]] || (( WINDOW_RUNS < 1 )); then
  echo "Error: --window-runs must be integer >= 1" >&2
  exit 1
fi

for value in \
  "$WATCH_DRIFT_RATE_PCT" \
  "$ESCALATE_DRIFT_RATE_PCT" \
  "$CRITICAL_DRIFT_RATE_PCT" \
  "$ESCALATE_FAILURE_RATE_PCT" \
  "$CRITICAL_FAILURE_RATE_PCT"; do
  if ! [[ "$value" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
    echo "Error: threshold values must be numeric percentages" >&2
    exit 1
  fi
done

if ! [[ "$ESCALATE_CONSECUTIVE_DRIFT" =~ ^[0-9]+$ ]] || (( ESCALATE_CONSECUTIVE_DRIFT < 1 )); then
  echo "Error: --escalate-consecutive-drift must be integer >= 1" >&2
  exit 1
fi

if ! [[ "$CRITICAL_CONSECUTIVE_DRIFT" =~ ^[0-9]+$ ]] || (( CRITICAL_CONSECUTIVE_DRIFT < 1 )); then
  echo "Error: --critical-consecutive-drift must be integer >= 1" >&2
  exit 1
fi

case "$FAIL_ON_LEVEL" in
  none|watch|escalated|critical) ;;
  *)
    echo "Error: --fail-on-level must be one of: none, watch, escalated, critical" >&2
    exit 1
    ;;
esac

if [[ -z "$DASHBOARD_JSON" ]]; then
  shopt -s nullglob
  dashboard_candidates=("$ANALYSIS_DIR"/monitor_policy_dashboard_*.json)
  shopt -u nullglob
  if (( ${#dashboard_candidates[@]} == 0 )); then
    echo "Error: no monitor_policy_dashboard_*.json found in $ANALYSIS_DIR" >&2
    exit 1
  fi
  readarray -t sorted_dashboards < <(printf '%s\n' "${dashboard_candidates[@]}" | sort)
  DASHBOARD_JSON="${sorted_dashboards[${#sorted_dashboards[@]}-1]}"
fi

if [[ ! -f "$DASHBOARD_JSON" ]]; then
  echo "Error: dashboard JSON not found: $DASHBOARD_JSON" >&2
  exit 1
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/monitor_drift_escalation_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - \
  "$ROOT" \
  "$DASHBOARD_JSON" \
  "$WINDOW_RUNS" \
  "$WATCH_DRIFT_RATE_PCT" \
  "$ESCALATE_DRIFT_RATE_PCT" \
  "$CRITICAL_DRIFT_RATE_PCT" \
  "$ESCALATE_CONSECUTIVE_DRIFT" \
  "$CRITICAL_CONSECUTIVE_DRIFT" \
  "$ESCALATE_FAILURE_RATE_PCT" \
  "$CRITICAL_FAILURE_RATE_PCT" \
  "$FAIL_ON_LEVEL" \
  "$OUTPUT_PATH" \
  "$OUTPUT_JSON_PATH" <<'PY'
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
dashboard_path = Path(sys.argv[2])
window_runs = int(sys.argv[3])
watch_drift_rate = float(sys.argv[4])
escalate_drift_rate = float(sys.argv[5])
critical_drift_rate = float(sys.argv[6])
escalate_consecutive_drift = int(sys.argv[7])
critical_consecutive_drift = int(sys.argv[8])
escalate_failure_rate = float(sys.argv[9])
critical_failure_rate = float(sys.argv[10])
fail_on_level = sys.argv[11]
output_path = Path(sys.argv[12])
output_json_path = Path(sys.argv[13])


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(root.resolve()))
    except ValueError:
        return str(path)


def safe_float(value, default=0.0):
    try:
        return float(value)
    except (TypeError, ValueError):
        return default


def trailing_streak(values, needle):
    streak = 0
    for item in reversed(values):
        if item == needle:
            streak += 1
        else:
            break
    return streak


severity = {
    "normal": 0,
    "watch": 1,
    "escalated": 2,
    "critical": 3,
}

failure_statuses = {"failed", "timeout", "missing_report"}

dashboard = json.loads(dashboard_path.read_text(encoding="utf-8"))
bench_health = dashboard.get("bench_health", {})
recent_runs = dashboard.get("recent_runs", [])
recent_runs = sorted(recent_runs, key=lambda item: item.get("timestamp", ""))
window = recent_runs[-window_runs:] if recent_runs else []

bench_set = set(bench_health.keys())
bench_history = {}

for run in window:
    for case in run.get("cases", []):
        if case.get("stage") != "monitor":
            continue
        bench = str(case.get("bench", "")).strip()
        if not bench:
            continue
        bench_set.add(bench)
        bench_history.setdefault(bench, []).append(str(case.get("status", "n/a")))

rows = []
for bench in sorted(bench_set):
    statuses = bench_history.get(bench, [])
    latest_status = statuses[-1] if statuses else "n/a"
    drift_count_window = sum(1 for status in statuses if status == "drift")
    failure_count_window = sum(1 for status in statuses if status in failure_statuses)
    consecutive_drift = trailing_streak(statuses, "drift")

    health = bench_health.get(bench, {})
    drift_rate_pct = safe_float(health.get("drift_rate_pct"), 0.0)
    failure_rate_pct = safe_float(health.get("failure_rate_pct"), 0.0)
    samples = int(health.get("samples", len(statuses)) or len(statuses))

    level = "normal"
    triggers = []

    if drift_count_window > 0 or drift_rate_pct >= watch_drift_rate:
        level = "watch"
        if drift_count_window > 0:
            triggers.append(f"drift_count_window={drift_count_window}")
        if drift_rate_pct >= watch_drift_rate:
            triggers.append(f"drift_rate_pct={drift_rate_pct:.2f}>={watch_drift_rate:.2f}")

    if (
        consecutive_drift >= escalate_consecutive_drift
        or drift_rate_pct >= escalate_drift_rate
        or failure_rate_pct >= escalate_failure_rate
    ):
        level = "escalated"
        if consecutive_drift >= escalate_consecutive_drift:
            triggers.append(
                f"consecutive_drift={consecutive_drift}>={escalate_consecutive_drift}"
            )
        if drift_rate_pct >= escalate_drift_rate:
            triggers.append(f"drift_rate_pct={drift_rate_pct:.2f}>={escalate_drift_rate:.2f}")
        if failure_rate_pct >= escalate_failure_rate:
            triggers.append(
                f"failure_rate_pct={failure_rate_pct:.2f}>={escalate_failure_rate:.2f}"
            )

    if (
        consecutive_drift >= critical_consecutive_drift
        or drift_rate_pct >= critical_drift_rate
        or failure_rate_pct >= critical_failure_rate
        or failure_count_window >= 2
        or latest_status in {"timeout", "missing_report"}
    ):
        level = "critical"
        if consecutive_drift >= critical_consecutive_drift:
            triggers.append(
                f"consecutive_drift={consecutive_drift}>={critical_consecutive_drift}"
            )
        if drift_rate_pct >= critical_drift_rate:
            triggers.append(f"drift_rate_pct={drift_rate_pct:.2f}>={critical_drift_rate:.2f}")
        if failure_rate_pct >= critical_failure_rate:
            triggers.append(
                f"failure_rate_pct={failure_rate_pct:.2f}>={critical_failure_rate:.2f}"
            )
        if failure_count_window >= 2:
            triggers.append(f"failure_count_window={failure_count_window}>=2")
        if latest_status in {"timeout", "missing_report"}:
            triggers.append(f"latest_status={latest_status}")

    if not triggers:
        triggers.append("none")

    if level == "normal":
        required_action = "Routine monitoring."
    elif level == "watch":
        required_action = "Review in governance cadence and track next release cycle."
    elif level == "escalated":
        required_action = "Open/update MONPOL proposal with owner and mitigation plan."
    else:
        required_action = "Immediate triage; containment and reviewer follow-up in current release."

    rows.append(
        {
            "bench": bench,
            "samples": samples,
            "latest_status": latest_status,
            "drift_rate_pct": round(drift_rate_pct, 2),
            "failure_rate_pct": round(failure_rate_pct, 2),
            "drift_count_window": drift_count_window,
            "failure_count_window": failure_count_window,
            "consecutive_drift": consecutive_drift,
            "level": level,
            "triggers": triggers,
            "required_action": required_action,
        }
    )

overall_level = "normal"
for row in rows:
    if severity[row["level"]] > severity[overall_level]:
        overall_level = row["level"]

level_counts = {name: 0 for name in ["normal", "watch", "escalated", "critical"]}
for row in rows:
    level_counts[row["level"]] = level_counts.get(row["level"], 0) + 1

escalated_benches = [row["bench"] for row in rows if severity[row["level"]] >= severity["escalated"]]
critical_benches = [row["bench"] for row in rows if row["level"] == "critical"]

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Monitor Drift Escalation Assessment\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Dashboard source**: `{rel(dashboard_path)}`  \n")
    fh.write(f"**Recent runs window**: {len(window)} (configured={window_runs})\n\n")

    fh.write("## Policy Thresholds\n\n")
    fh.write(f"- watch drift rate (%): {watch_drift_rate}\n")
    fh.write(f"- escalated drift rate (%): {escalate_drift_rate}\n")
    fh.write(f"- critical drift rate (%): {critical_drift_rate}\n")
    fh.write(f"- escalated consecutive drift: {escalate_consecutive_drift}\n")
    fh.write(f"- critical consecutive drift: {critical_consecutive_drift}\n")
    fh.write(f"- escalated failure rate (%): {escalate_failure_rate}\n")
    fh.write(f"- critical failure rate (%): {critical_failure_rate}\n\n")

    fh.write("## Overall Escalation Status\n\n")
    fh.write(f"- Overall level: **{overall_level}**\n")
    fh.write(
        f"- Level counts: normal={level_counts.get('normal', 0)}, "
        f"watch={level_counts.get('watch', 0)}, "
        f"escalated={level_counts.get('escalated', 0)}, "
        f"critical={level_counts.get('critical', 0)}\n"
    )
    fh.write(f"- Escalated benches: {len(escalated_benches)}\n")
    fh.write(f"- Critical benches: {len(critical_benches)}\n\n")

    fh.write("| Benchmark | Samples | Latest status | Drift rate (%) | Failure rate (%) | Drift in window | Failure in window | Consecutive drift | Level | Triggers | Required action |\n")
    fh.write("|---|---:|---|---:|---:|---:|---:|---:|---|---|---|\n")
    if rows:
        for row in rows:
            trigger_text = ", ".join(row["triggers"])
            fh.write(
                f"| `{row['bench']}` | {row['samples']} | {row['latest_status']} | "
                f"{row['drift_rate_pct']:.2f} | {row['failure_rate_pct']:.2f} | "
                f"{row['drift_count_window']} | {row['failure_count_window']} | {row['consecutive_drift']} | "
                f"{row['level']} | {trigger_text} | {row['required_action']} |\n"
            )
    else:
        fh.write("| _none_ | 0 | n/a | 0.00 | 0.00 | 0 | 0 | 0 | normal | none | Routine monitoring. |\n")
    fh.write("\n")

    fh.write("Assessment is advisory unless `--fail-on-level` is configured.\n")

fail_triggered = False
if fail_on_level != "none":
    fail_triggered = severity[overall_level] >= severity[fail_on_level]

payload = {
    "generated_at_utc": generated_at,
    "dashboard_source": rel(dashboard_path),
    "window": {
        "configured_runs": window_runs,
        "evaluated_runs": len(window),
    },
    "thresholds": {
        "watch_drift_rate_pct": watch_drift_rate,
        "escalated_drift_rate_pct": escalate_drift_rate,
        "critical_drift_rate_pct": critical_drift_rate,
        "escalated_consecutive_drift": escalate_consecutive_drift,
        "critical_consecutive_drift": critical_consecutive_drift,
        "escalated_failure_rate_pct": escalate_failure_rate,
        "critical_failure_rate_pct": critical_failure_rate,
    },
    "overall_level": overall_level,
    "level_counts": level_counts,
    "rows": rows,
    "escalated_benches": escalated_benches,
    "critical_benches": critical_benches,
    "fail_on_level": fail_on_level,
    "fail_triggered": fail_triggered,
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Monitor drift escalation markdown: {output_path}")
print(f"Monitor drift escalation JSON: {output_json_path}")
print(f"Overall level: {overall_level}")
print(f"Escalated benches: {len(escalated_benches)}")
print(f"Critical benches: {len(critical_benches)}")

if fail_triggered:
    print(f"Escalation level '{overall_level}' meets/exceeds fail-on-level '{fail_on_level}'", file=sys.stderr)
    raise SystemExit(2)
PY

