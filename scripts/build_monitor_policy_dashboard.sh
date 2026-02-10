#!/usr/bin/env bash
# Build monitor policy drift dashboard from benchmark gate summaries and policy recommendations.
# Usage:
#   ./scripts/build_monitor_policy_dashboard.sh
#   ./scripts/build_monitor_policy_dashboard.sh --lookback-runs 10 --lookback-recommendations 6

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"

LOOKBACK_RUNS=10
LOOKBACK_RECOMMENDATIONS=6
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/build_monitor_policy_dashboard.sh [options]

Options:
  --analysis-dir <path>              Directory with benchmark reproducibility artifacts
  --lookback-runs <n>                Number of newest CI summary runs to analyze (default: 10)
  --lookback-recommendations <n>     Number of newest recommendation files to analyze (default: 6)
  --output <path>                    Output markdown dashboard path
  --output-json <path>               Output JSON dashboard path
  -h, --help                         Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --lookback-runs)
      LOOKBACK_RUNS="${2:-}"
      shift 2
      ;;
    --lookback-recommendations)
      LOOKBACK_RECOMMENDATIONS="${2:-}"
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

if ! [[ "$LOOKBACK_RUNS" =~ ^[0-9]+$ ]] || (( LOOKBACK_RUNS < 1 )); then
  echo "Error: --lookback-runs must be integer >= 1" >&2
  exit 1
fi

if ! [[ "$LOOKBACK_RECOMMENDATIONS" =~ ^[0-9]+$ ]] || (( LOOKBACK_RECOMMENDATIONS < 1 )); then
  echo "Error: --lookback-recommendations must be integer >= 1" >&2
  exit 1
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/monitor_policy_dashboard_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - "$ANALYSIS_DIR" "$LOOKBACK_RUNS" "$LOOKBACK_RECOMMENDATIONS" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" <<'PY'
import json
import re
import sys
from pathlib import Path

analysis_dir = Path(sys.argv[1])
lookback_runs = int(sys.argv[2])
lookback_recommendations = int(sys.argv[3])
output_path = Path(sys.argv[4])
output_json_path = Path(sys.argv[5])

summary_re = re.compile(r"^ci_benchmark_gate_summary_(\d{8}T\d{6}Z)\.md$")
recommend_re = re.compile(r"^monitor_policy_recommendations_(\d{8}T\d{6}Z)\.json$")

if not analysis_dir.exists():
    raise SystemExit(f"Analysis directory not found: {analysis_dir}")


def parse_summary(path: Path):
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    section = None
    cases = []
    metrics = []
    status_counts = {}

    for line in lines:
        if line.startswith("Case status counts:"):
            tail = line.split(":", 1)[1].strip()
            for chunk in tail.split(","):
                chunk = chunk.strip()
                if not chunk or "=" not in chunk:
                    continue
                key, raw = chunk.split("=", 1)
                key = key.strip()
                raw = raw.strip()
                if raw.isdigit():
                    status_counts[key] = int(raw)
            continue

        if line.startswith("## Case status"):
            section = "case"
            continue
        if line.startswith("## Reproducibility metrics"):
            section = "metrics"
            continue
        if not line.startswith("|"):
            continue
        if line.startswith("|---"):
            continue

        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if section == "case":
            if len(cells) < 6 or cells[0] == "Stage":
                continue
            bench = cells[1].strip("`")
            cases.append(
                {
                    "stage": cells[0],
                    "bench": bench,
                    "threshold_pct": cells[2],
                    "status": cells[3],
                    "duration_s": cells[4],
                    "note": cells[5],
                }
            )
        elif section == "metrics":
            if len(cells) < 6 or cells[0] == "Stage":
                continue
            metrics.append(
                {
                    "stage": cells[0],
                    "report": cells[1].strip("`"),
                    "metrics": cells[2],
                    "over_threshold": cells[3],
                    "median_spread_pct": cells[4],
                    "max_spread_pct": cells[5],
                }
            )

    monitor_cases = [case for case in cases if case["stage"] == "monitor"]
    strict_cases = [case for case in cases if case["stage"] == "strict"]
    monitor_drift = sum(1 for case in monitor_cases if case["status"] == "drift")
    monitor_failures = sum(
        1 for case in monitor_cases if case["status"] in {"failed", "timeout", "missing_report"}
    )
    strict_status = strict_cases[0]["status"] if strict_cases else "n/a"
    duration_sum = 0
    for case in cases:
        raw = case["duration_s"]
        if raw.isdigit():
            duration_sum += int(raw)

    return {
        "timestamp": path.stem.rsplit("_", 1)[-1],
        "file": path.name,
        "status_counts": status_counts,
        "cases": cases,
        "metrics": metrics,
        "strict_status": strict_status,
        "monitor_case_count": len(monitor_cases),
        "monitor_drift_count": monitor_drift,
        "monitor_failure_count": monitor_failures,
        "duration_total_s": duration_sum,
    }


def parse_recommendation(path: Path):
    payload = json.loads(path.read_text(encoding="utf-8"))
    return {
        "timestamp": path.stem.rsplit("_", 1)[-1],
        "file": path.name,
        "recommendations": payload.get("recommendations", []),
        "headroom_pct": payload.get("headroom_pct"),
        "lookback": payload.get("lookback"),
        "min_samples": payload.get("min_samples"),
    }


summary_paths = []
recommend_paths = []

for path in analysis_dir.glob("*.md"):
    if summary_re.match(path.name):
        summary_paths.append(path)

for path in analysis_dir.glob("*.json"):
    if recommend_re.match(path.name):
        recommend_paths.append(path)

summary_paths.sort(key=lambda p: summary_re.match(p.name).group(1))
recommend_paths.sort(key=lambda p: recommend_re.match(p.name).group(1))

summaries = [parse_summary(path) for path in summary_paths[-lookback_runs:]]
recommendations = [parse_recommendation(path) for path in recommend_paths[-lookback_recommendations:]]

bench_health = {}
for summary in summaries:
    for case in summary["cases"]:
        if case["stage"] != "monitor":
            continue
        bench = case["bench"]
        slot = bench_health.setdefault(
            bench,
            {
                "samples": 0,
                "drift": 0,
                "failures": 0,
                "timeouts": 0,
                "latest_status": "n/a",
                "latest_threshold_pct": "n/a",
                "latest_timestamp": "",
            },
        )
        slot["samples"] += 1
        if case["status"] == "drift":
            slot["drift"] += 1
        if case["status"] == "timeout":
            slot["timeouts"] += 1
        if case["status"] in {"failed", "timeout", "missing_report"}:
            slot["failures"] += 1
        if summary["timestamp"] >= slot["latest_timestamp"]:
            slot["latest_status"] = case["status"]
            slot["latest_threshold_pct"] = case["threshold_pct"]
            slot["latest_timestamp"] = summary["timestamp"]

for slot in bench_health.values():
    samples = slot["samples"]
    slot["drift_rate_pct"] = round((slot["drift"] / samples * 100.0), 2) if samples else 0.0
    slot["failure_rate_pct"] = round((slot["failures"] / samples * 100.0), 2) if samples else 0.0

latest_recommendation = recommendations[-1] if recommendations else None
latest_recommendation_map = {}
if latest_recommendation:
    for item in latest_recommendation["recommendations"]:
        latest_recommendation_map[item.get("bench", "")] = item

strict_fail_runs = sum(1 for item in summaries if item["strict_status"] != "pass")
monitor_drift_runs = sum(1 for item in summaries if item["monitor_drift_count"] > 0)
monitor_failure_runs = sum(1 for item in summaries if item["monitor_failure_count"] > 0)

output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Monitor Policy Drift Dashboard\n\n")
    fh.write(f"**Source directory**: `{analysis_dir}`  \n")
    fh.write(f"**Runs analyzed**: {len(summaries)} (lookback={lookback_runs})  \n")
    fh.write(
        f"**Recommendation snapshots analyzed**: {len(recommendations)} (lookback={lookback_recommendations})\n\n"
    )

    fh.write("## Health Summary\n\n")
    fh.write(f"- Strict non-pass runs: {strict_fail_runs}\n")
    fh.write(f"- Runs with monitor drift cases: {monitor_drift_runs}\n")
    fh.write(f"- Runs with monitor failure/timeout/missing-report cases: {monitor_failure_runs}\n\n")

    fh.write("## Monitor Benchmark Health\n\n")
    fh.write(
        "| Benchmark | Samples | Drift count | Drift rate (%) | Failure count | Failure rate (%) | "
        "Latest status | Latest threshold (%) | Recommendation action | Recommended threshold (%) |\n"
    )
    fh.write("|---|---:|---:|---:|---:|---:|---|---:|---|---:|\n")
    if bench_health:
        for bench in sorted(bench_health):
            slot = bench_health[bench]
            rec = latest_recommendation_map.get(bench, {})
            rec_action = rec.get("action", "n/a")
            rec_threshold = rec.get("recommended_threshold_pct", "n/a")
            fh.write(
                "| "
                + " | ".join(
                    [
                        f"`{bench}`",
                        str(slot["samples"]),
                        str(slot["drift"]),
                        f"{slot['drift_rate_pct']:.2f}",
                        str(slot["failures"]),
                        f"{slot['failure_rate_pct']:.2f}",
                        slot["latest_status"],
                        str(slot["latest_threshold_pct"]),
                        str(rec_action),
                        str(rec_threshold),
                    ]
                )
                + " |\n"
            )
    else:
        fh.write("| _none_ | 0 | 0 | 0.00 | 0 | 0.00 | n/a | n/a | n/a | n/a |\n")

    fh.write("\n## Recent Run Timeline\n\n")
    fh.write("| Timestamp | Strict status | Monitor drift cases | Monitor failure cases | Total duration (s) | Summary |\n")
    fh.write("|---|---|---:|---:|---:|---|\n")
    for item in summaries:
        fh.write(
            "| "
            + " | ".join(
                [
                    item["timestamp"],
                    item["strict_status"],
                    str(item["monitor_drift_count"]),
                    str(item["monitor_failure_count"]),
                    str(item["duration_total_s"]),
                    f"`{item['file']}`",
                ]
            )
            + " |\n"
        )

    if latest_recommendation:
        fh.write("\n## Latest Recommendation Snapshot\n\n")
        fh.write(f"Source: `{latest_recommendation['file']}`\n\n")
        fh.write("| Benchmark | Latest threshold (%) | Recommended (%) | Action | Drift reports | Drift rate (%) |\n")
        fh.write("|---|---:|---:|---|---:|---:|\n")
        for item in sorted(latest_recommendation["recommendations"], key=lambda e: e.get("bench", "")):
            fh.write(
                "| "
                + " | ".join(
                    [
                        f"`{item.get('bench', 'n/a')}`",
                        str(item.get("latest_threshold_pct", "n/a")),
                        str(item.get("recommended_threshold_pct", "n/a")),
                        str(item.get("action", "n/a")),
                        str(item.get("drift_reports", "n/a")),
                        str(item.get("drift_rate_pct", "n/a")),
                    ]
                )
                + " |\n"
            )

    fh.write("\n")
    fh.write("Dashboard is advisory; policy changes should follow governance gate requirements.\n")

dashboard_json = {
    "source_directory": str(analysis_dir),
    "runs_analyzed": len(summaries),
    "recommendation_snapshots_analyzed": len(recommendations),
    "lookback_runs": lookback_runs,
    "lookback_recommendations": lookback_recommendations,
    "health_summary": {
        "strict_non_pass_runs": strict_fail_runs,
        "runs_with_monitor_drift": monitor_drift_runs,
        "runs_with_monitor_failures": monitor_failure_runs,
    },
    "bench_health": bench_health,
    "recent_runs": summaries,
    "latest_recommendation": latest_recommendation,
}
output_json_path.write_text(json.dumps(dashboard_json, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Monitor policy dashboard markdown: {output_path}")
print(f"Monitor policy dashboard JSON: {output_json_path}")
print(f"Runs analyzed: {len(summaries)}")
print(f"Recommendation snapshots analyzed: {len(recommendations)}")
PY
