#!/usr/bin/env bash
# Generate monitor-threshold recommendations from rolling reproducibility evidence.
# Usage:
#   ./scripts/recommend_monitor_policy.sh
#   ./scripts/recommend_monitor_policy.sh --bench timer_queue_benchmark --bench directory_entry_cache_benchmark

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"

LOOKBACK=6
MIN_SAMPLES=2
HEADROOM_PCT=15
FLOOR_PCT=5
CEIL_PCT=80
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""
BENCH_FILTERS=()

usage() {
  cat <<'USAGE'
Usage: ./scripts/recommend_monitor_policy.sh [options]

Options:
  --analysis-dir <path>      Directory containing reproducibility markdown reports
  --bench <name>             Benchmark to include (repeatable)
  --lookback <n>             Newest reports per benchmark to analyze (default: 6)
  --min-samples <n>          Minimum samples to issue recommendation (default: 2)
  --headroom-pct <n>         Safety margin added to percentile spread (default: 15)
  --floor-pct <n>            Minimum allowed recommendation (default: 5)
  --ceil-pct <n>             Maximum allowed recommendation (default: 80)
  --output <path>            Output markdown report path
  --output-json <path>       Output JSON report path
  -h, --help                 Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --bench)
      BENCH_FILTERS+=("${2:-}")
      shift 2
      ;;
    --lookback)
      LOOKBACK="${2:-}"
      shift 2
      ;;
    --min-samples)
      MIN_SAMPLES="${2:-}"
      shift 2
      ;;
    --headroom-pct)
      HEADROOM_PCT="${2:-}"
      shift 2
      ;;
    --floor-pct)
      FLOOR_PCT="${2:-}"
      shift 2
      ;;
    --ceil-pct)
      CEIL_PCT="${2:-}"
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

if ! [[ "$LOOKBACK" =~ ^[0-9]+$ ]] || (( LOOKBACK < 1 )); then
  echo "Error: --lookback must be integer >= 1" >&2
  exit 1
fi

if ! [[ "$MIN_SAMPLES" =~ ^[0-9]+$ ]] || (( MIN_SAMPLES < 1 )); then
  echo "Error: --min-samples must be integer >= 1" >&2
  exit 1
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/monitor_policy_recommendations_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

FILTER_CSV=""
if (( ${#BENCH_FILTERS[@]} > 0 )); then
  FILTER_CSV="$(IFS=,; echo "${BENCH_FILTERS[*]}")"
fi

python3 - "$ANALYSIS_DIR" "$LOOKBACK" "$MIN_SAMPLES" "$HEADROOM_PCT" "$FLOOR_PCT" "$CEIL_PCT" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" "$FILTER_CSV" <<'PY'
import json
import math
import re
import statistics
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional

analysis_dir = Path(sys.argv[1])
lookback = int(sys.argv[2])
min_samples = int(sys.argv[3])
headroom_pct = float(sys.argv[4])
floor_pct = float(sys.argv[5])
ceil_pct = float(sys.argv[6])
output_path = Path(sys.argv[7])
output_json_path = Path(sys.argv[8])
filter_csv = sys.argv[9]
bench_filter = {entry.strip() for entry in filter_csv.split(",") if entry.strip()}

report_file_re = re.compile(r"^(?P<bench>[a-zA-Z0-9_]+)_(?P<ts>\d{8}T\d{6}Z)\.md$")


@dataclass
class ReportSample:
    bench: str
    timestamp: str
    path: Path
    spread_threshold_pct: float
    metrics_over_threshold: int
    median_spread_pct: float
    max_spread_pct: float


def extract_float(pattern: str, text: str) -> Optional[float]:
    match = re.search(pattern, text, re.MULTILINE)
    if not match:
        return None
    return float(match.group(1))


def extract_int(pattern: str, text: str) -> Optional[int]:
    match = re.search(pattern, text, re.MULTILINE)
    if not match:
        return None
    return int(match.group(1))


def percentile(values: List[float], pct: float) -> float:
    if not values:
        return 0.0
    ordered = sorted(values)
    rank = max(1, math.ceil((pct / 100.0) * len(ordered)))
    idx = min(len(ordered) - 1, rank - 1)
    return ordered[idx]


def clamp(value: float, low: float, high: float) -> float:
    return max(low, min(high, value))


samples_by_bench: Dict[str, List[ReportSample]] = {}

if not analysis_dir.exists():
    raise SystemExit(f"Analysis directory not found: {analysis_dir}")

for path in sorted(analysis_dir.glob("*.md")):
    match = report_file_re.match(path.name)
    if not match:
        continue
    bench = match.group("bench")
    timestamp = match.group("ts")

    if bench == "ci_benchmark_gate_summary":
        continue
    if bench_filter and bench not in bench_filter:
        continue

    text = path.read_text(encoding="utf-8")
    spread_threshold = extract_float(r"\*\*Spread threshold\*\*:\s*([0-9.]+)%", text)
    metrics_over = extract_int(r"^- Metrics above spread threshold:\s*([0-9]+)$", text)
    median_spread = extract_float(r"^- Median spread:\s*([0-9.]+)%$", text)
    max_spread = extract_float(r"^- Max spread:\s*([0-9.]+)%$", text)
    if None in (spread_threshold, metrics_over, median_spread, max_spread):
        continue

    sample = ReportSample(
        bench=bench,
        timestamp=timestamp,
        path=path,
        spread_threshold_pct=float(spread_threshold),
        metrics_over_threshold=int(metrics_over),
        median_spread_pct=float(median_spread),
        max_spread_pct=float(max_spread),
    )
    samples_by_bench.setdefault(bench, []).append(sample)

recommendations = []
for bench, samples in sorted(samples_by_bench.items()):
    ordered = sorted(samples, key=lambda item: item.timestamp)
    window = ordered[-lookback:]

    latest = window[-1]
    latest_threshold = latest.spread_threshold_pct
    max_spreads = [item.max_spread_pct for item in window]
    median_spreads = [item.median_spread_pct for item in window]
    drift_count = sum(1 for item in window if item.metrics_over_threshold > 0)

    p90_max = percentile(max_spreads, 90.0)
    p75_median = percentile(median_spreads, 75.0)
    base_signal = max(p90_max, p75_median)
    target = base_signal * (1.0 + headroom_pct / 100.0)
    target = clamp(target, floor_pct, ceil_pct)

    if len(window) < min_samples:
        recommended = latest_threshold
        action = "insufficient_samples"
    else:
        recommended = round(target, 2)
        delta = recommended - latest_threshold
        if abs(delta) < 2.0:
            action = "hold"
        elif delta > 0:
            action = "relax"
        else:
            action = "tighten"

    drift_rate = (drift_count / len(window) * 100.0) if window else 0.0
    recommendations.append(
        {
            "bench": bench,
            "samples": len(window),
            "latest_threshold_pct": round(latest_threshold, 2),
            "drift_reports": drift_count,
            "drift_rate_pct": round(drift_rate, 2),
            "p90_max_spread_pct": round(p90_max, 3),
            "p75_median_spread_pct": round(p75_median, 3),
            "max_spread_peak_pct": round(max(max_spreads), 3) if max_spreads else 0.0,
            "recommended_threshold_pct": round(recommended, 2),
            "action": action,
            "latest_report": latest.path.name,
        }
    )

output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Monitor Policy Recommendations (Rolling Evidence)\n\n")
    fh.write(f"**Source directory**: `{analysis_dir}`  \n")
    fh.write(f"**Lookback per benchmark**: {lookback}  \n")
    fh.write(f"**Minimum samples**: {min_samples}  \n")
    fh.write(f"**Headroom**: {headroom_pct:.2f}%  \n")
    fh.write(f"**Clamp range**: [{floor_pct:.2f}%, {ceil_pct:.2f}%]\n\n")

    if bench_filter:
        fh.write(
            "**Bench filter**: "
            + ", ".join(f"`{bench}`" for bench in sorted(bench_filter))
            + "\n\n"
        )
    else:
        fh.write("**Bench filter**: _none (all matching benchmarks)_\n\n")

    fh.write(
        "Recommendation heuristic: `max(p90(max_spread), p75(median_spread)) * (1 + headroom)` "
        "with clamp bounds.\n\n"
    )

    headers = [
        "Benchmark",
        "Samples",
        "Latest threshold (%)",
        "Drift reports",
        "Drift rate (%)",
        "p90 max spread (%)",
        "p75 median spread (%)",
        "Peak max spread (%)",
        "Recommended (%)",
        "Action",
        "Latest report",
    ]
    fh.write("| " + " | ".join(headers) + " |\n")
    fh.write("|" + "|".join(["---"] * len(headers)) + "|\n")

    if recommendations:
        for item in recommendations:
            fh.write(
                "| "
                + " | ".join(
                    [
                        f"`{item['bench']}`",
                        str(item["samples"]),
                        f"{item['latest_threshold_pct']:.2f}",
                        str(item["drift_reports"]),
                        f"{item['drift_rate_pct']:.2f}",
                        f"{item['p90_max_spread_pct']:.3f}",
                        f"{item['p75_median_spread_pct']:.3f}",
                        f"{item['max_spread_peak_pct']:.3f}",
                        f"{item['recommended_threshold_pct']:.2f}",
                        item["action"],
                        f"`{item['latest_report']}`",
                    ]
                )
                + " |\n"
            )
    else:
        fh.write("| _no data_ | 0 | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a |\n")

    fh.write("\n")
    fh.write(
        "This report is advisory; CI thresholds should be updated intentionally and reviewed.\n"
    )

json_payload = {
    "source_directory": str(analysis_dir),
    "lookback": lookback,
    "min_samples": min_samples,
    "headroom_pct": headroom_pct,
    "floor_pct": floor_pct,
    "ceil_pct": ceil_pct,
    "bench_filter": sorted(bench_filter),
    "recommendations": recommendations,
}
output_json_path.write_text(json.dumps(json_payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Monitor policy markdown report: {output_path}")
print(f"Monitor policy JSON report: {output_json_path}")
print(f"Benchmarks analyzed: {len(recommendations)}")
PY
