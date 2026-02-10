#!/usr/bin/env bash
# Run CI benchmark gate with strict + scenario monitor stages.
# Usage:
#   ./scripts/run_benchmark_ci_gate.sh
#   ./scripts/run_benchmark_ci_gate.sh --runs 2 --strict-threshold-pct 50 --monitor-threshold-pct 25

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"

RUNS=2
STRICT_BENCH="path_lookup_cache_benchmark"
STRICT_THRESHOLD_PCT=50
MONITOR_BENCH="timer_queue_benchmark"
MONITOR_THRESHOLD_PCT=25
BASELINE_PREFIX="ci_repro"

usage() {
  cat <<'USAGE'
Usage: ./scripts/run_benchmark_ci_gate.sh [options]

Options:
  --runs <n>                     Number of repeated runs per bench (default: 2)
  --strict-bench <name>          Blocking benchmark target (default: path_lookup_cache_benchmark)
  --strict-threshold-pct <n>     Strict spread threshold percent (default: 50)
  --monitor-bench <name>         Scenario monitor benchmark target (default: timer_queue_benchmark)
  --monitor-threshold-pct <n>    Monitor spread threshold percent (default: 25)
  --baseline-prefix <prefix>     Baseline prefix family (default: ci_repro)
  -h, --help                     Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --runs)
      RUNS="${2:-}"
      shift 2
      ;;
    --strict-bench)
      STRICT_BENCH="${2:-}"
      shift 2
      ;;
    --strict-threshold-pct)
      STRICT_THRESHOLD_PCT="${2:-}"
      shift 2
      ;;
    --monitor-bench)
      MONITOR_BENCH="${2:-}"
      shift 2
      ;;
    --monitor-threshold-pct)
      MONITOR_THRESHOLD_PCT="${2:-}"
      shift 2
      ;;
    --baseline-prefix)
      BASELINE_PREFIX="${2:-}"
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

if ! [[ "$RUNS" =~ ^[0-9]+$ ]] || (( RUNS < 2 )); then
  echo "Error: --runs must be integer >= 2" >&2
  exit 1
fi

mkdir -p "$ANALYSIS_DIR"

REPORTS=()
STRICT_FAILED=0
TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
SUMMARY_PATH="$ANALYSIS_DIR/ci_benchmark_gate_summary_${TIMESTAMP}.md"

run_repro_case() {
  local bench="$1"
  local threshold="$2"
  local strict_flag="$3"
  local label="$4"

  local cmd=(
    "$ROOT/scripts/benchmark_reproducibility.sh"
    --bench "$bench"
    --runs "$RUNS"
    --spread-threshold-pct "$threshold"
    --baseline-prefix "${BASELINE_PREFIX}_${label}"
  )
  if [[ "$strict_flag" == "1" ]]; then
    cmd+=(--strict)
  fi

  local output=""
  local status=0
  if ! output="$("${cmd[@]}" 2>&1)"; then
    status=$?
  fi
  echo "$output"

  local report_path
  report_path="$(printf '%s\n' "$output" | awk -F': ' '/Reproducibility report written to/{print $2}' | tail -1)"
  if [[ -n "$report_path" && -f "$report_path" ]]; then
    REPORTS+=("$report_path")
  fi

  if (( status != 0 )); then
    if [[ "$strict_flag" == "1" ]]; then
      echo "ERR Strict reproducibility case failed: $bench" >&2
      STRICT_FAILED=1
    else
      echo "WARN Monitor reproducibility case failed: $bench" >&2
    fi
  fi
}

echo "== CI benchmark gate (strict + monitor) =="
echo "Strict case: $STRICT_BENCH (threshold ${STRICT_THRESHOLD_PCT}%)"
echo "Monitor case: $MONITOR_BENCH (threshold ${MONITOR_THRESHOLD_PCT}%)"
echo "Runs per case: $RUNS"
echo

echo "-- Strict case --"
run_repro_case "$STRICT_BENCH" "$STRICT_THRESHOLD_PCT" 1 "strict"
echo
echo "-- Monitor case --"
run_repro_case "$MONITOR_BENCH" "$MONITOR_THRESHOLD_PCT" 0 "monitor"
echo

python3 - "$SUMMARY_PATH" "${REPORTS[@]}" <<'PY'
import re
import sys
from pathlib import Path

summary_path = Path(sys.argv[1])
reports = [Path(p) for p in sys.argv[2:] if p]

summary_path.parent.mkdir(parents=True, exist_ok=True)

def extract(label: str, text: str) -> str:
    pattern = rf"- {re.escape(label)}: (.+)"
    m = re.search(pattern, text)
    return m.group(1).strip() if m else "n/a"

with summary_path.open("w", encoding="utf-8") as fh:
    fh.write("# CI Benchmark Gate Summary\n\n")
    fh.write(f"Generated reports: {len(reports)}\n\n")
    fh.write("| Report | Metrics | Above threshold | Median spread | Max spread |\n")
    fh.write("|---|---:|---:|---:|---:|\n")
    for report in reports:
        text = report.read_text(encoding="utf-8")
        metrics = extract("Common benchmark metrics", text)
        unstable = extract("Metrics above spread threshold", text)
        median = extract("Median spread", text)
        max_spread = extract("Max spread", text)
        fh.write(
            f"| `{report.name}` | {metrics} | {unstable} | {median} | {max_spread} |\n"
        )
    if not reports:
        fh.write("| _none_ | n/a | n/a | n/a | n/a |\n")

print(f"CI benchmark gate summary: {summary_path}")
PY

if (( STRICT_FAILED != 0 )); then
  echo "CI benchmark gate failed due to strict reproducibility case." >&2
  exit 1
fi

echo "CI benchmark gate completed successfully."
