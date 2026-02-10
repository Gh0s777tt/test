#!/usr/bin/env bash
# Run CI benchmark gate with strict and monitor stages.
# Usage:
#   ./scripts/run_benchmark_ci_gate.sh
#   ./scripts/run_benchmark_ci_gate.sh --runs 2 --strict-threshold-pct 50 --monitor-threshold-pct 25

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"

RUNS=2
STRICT_BENCH="path_lookup_cache_benchmark"
STRICT_THRESHOLD_PCT=50
MONITOR_BENCHES=("timer_queue_benchmark" "directory_entry_cache_benchmark")
MONITOR_THRESHOLD_PCT=25
MONITOR_BUDGET_SECONDS=240
MONITOR_CASE_TIMEOUT_SECONDS=0
BASELINE_PREFIX="ci_repro"

usage() {
  cat <<'USAGE'
Usage: ./scripts/run_benchmark_ci_gate.sh [options]

Options:
  --runs <n>                     Number of repeated runs per bench (default: 2)
  --strict-bench <name>          Blocking benchmark target (default: path_lookup_cache_benchmark)
  --strict-threshold-pct <n>     Strict spread threshold percent (default: 50)
  --monitor-bench <name>         Scenario monitor benchmark target (repeatable)
  --monitor-benches <csv>        Comma-separated monitor benchmark targets
  --monitor-threshold-pct <n>    Monitor spread threshold percent (default: 25)
  --monitor-budget-seconds <n>   Monitor-stage wall-clock budget (default: 240, 0 disables)
  --monitor-case-timeout-seconds <n>
                                 Per-monitor benchmark timeout (default: 0, disabled)
  --baseline-prefix <prefix>     Baseline prefix family (default: ci_repro)
  -h, --help                     Show this help
USAGE
}

trim_whitespace() {
  local value="$1"
  value="${value#"${value%%[![:space:]]*}"}"
  value="${value%"${value##*[![:space:]]}"}"
  printf '%s' "$value"
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
      MONITOR_BENCHES+=("${2:-}")
      shift 2
      ;;
    --monitor-benches)
      IFS=',' read -r -a parsed_monitor_benches <<< "${2:-}"
      for bench in "${parsed_monitor_benches[@]}"; do
        bench="$(trim_whitespace "$bench")"
        if [[ -n "$bench" ]]; then
          MONITOR_BENCHES+=("$bench")
        fi
      done
      shift 2
      ;;
    --monitor-threshold-pct)
      MONITOR_THRESHOLD_PCT="${2:-}"
      shift 2
      ;;
    --monitor-budget-seconds)
      MONITOR_BUDGET_SECONDS="${2:-}"
      shift 2
      ;;
    --monitor-case-timeout-seconds)
      MONITOR_CASE_TIMEOUT_SECONDS="${2:-}"
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

if ! [[ "$MONITOR_BUDGET_SECONDS" =~ ^[0-9]+$ ]]; then
  echo "Error: --monitor-budget-seconds must be integer >= 0" >&2
  exit 1
fi

if ! [[ "$MONITOR_CASE_TIMEOUT_SECONDS" =~ ^[0-9]+$ ]]; then
  echo "Error: --monitor-case-timeout-seconds must be integer >= 0" >&2
  exit 1
fi

# Remove duplicates while preserving insertion order.
declare -A MONITOR_SEEN=()
UNIQUE_MONITOR_BENCHES=()
for bench in "${MONITOR_BENCHES[@]}"; do
  bench="$(trim_whitespace "$bench")"
  if [[ -z "$bench" ]]; then
    continue
  fi
  if [[ -z "${MONITOR_SEEN[$bench]+x}" ]]; then
    MONITOR_SEEN["$bench"]=1
    UNIQUE_MONITOR_BENCHES+=("$bench")
  fi
done
MONITOR_BENCHES=("${UNIQUE_MONITOR_BENCHES[@]}")

mkdir -p "$ANALYSIS_DIR"

REPORTS=()
STRICT_FAILED=0
TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
SUMMARY_PATH="$ANALYSIS_DIR/ci_benchmark_gate_summary_${TIMESTAMP}.md"
CASE_LOG="$(mktemp)"
trap 'rm -f "$CASE_LOG"' EXIT

append_case_log() {
  local stage="$1"
  local bench="$2"
  local threshold="$3"
  local status="$4"
  local duration_seconds="$5"
  local report_path="$6"
  local note="$7"
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$stage" "$bench" "$threshold" "$status" "$duration_seconds" "$report_path" "$note" \
    >> "$CASE_LOG"
}

run_repro_case() {
  local bench="$1"
  local threshold="$2"
  local stage="$3"
  local strict_flag="$4"
  local case_timeout_seconds="$5"

  local cmd=(
    "$ROOT/scripts/benchmark_reproducibility.sh"
    --bench "$bench"
    --runs "$RUNS"
    --spread-threshold-pct "$threshold"
    --baseline-prefix "${BASELINE_PREFIX}_${stage}"
  )
  if [[ "$strict_flag" == "1" ]]; then
    cmd+=(--strict)
  fi

  local output=""
  local status=0
  local case_start
  case_start="$(date +%s)"
  if (( case_timeout_seconds > 0 )) && command -v timeout >/dev/null 2>&1; then
    if ! output="$(timeout --foreground "${case_timeout_seconds}" "${cmd[@]}" 2>&1)"; then
      status=$?
    fi
  else
    if ! output="$("${cmd[@]}" 2>&1)"; then
      status=$?
    fi
  fi
  local case_end
  case_end="$(date +%s)"
  local duration_seconds=$((case_end - case_start))

  echo "$output"

  local report_path=""
  report_path="$(printf '%s\n' "$output" | awk -F': ' '/Reproducibility report written to/{print $2}' | tail -1)"
  if [[ -n "$report_path" && -f "$report_path" ]]; then
    REPORTS+=("$report_path")
  fi

  local case_status="pass"
  local note=""
  if (( status != 0 )); then
    if (( status == 124 )); then
      case_status="timeout"
      note="timed_out_after_${case_timeout_seconds}s"
    else
      case_status="failed"
    fi
  fi
  append_case_log "$stage" "$bench" "$threshold" "$case_status" "$duration_seconds" "$report_path" "$note"

  if (( status != 0 )); then
    if [[ "$strict_flag" == "1" ]]; then
      echo "ERR Strict reproducibility case failed: $bench" >&2
      STRICT_FAILED=1
    else
      if (( status == 124 )); then
        echo "WARN Monitor reproducibility case timed out: $bench" >&2
      else
        echo "WARN Monitor reproducibility case failed: $bench" >&2
      fi
    fi
  fi

  return "$status"
}

run_monitor_stage() {
  local stage_start
  stage_start="$(date +%s)"

  if (( ${#MONITOR_BENCHES[@]} == 0 )); then
    echo "WARN No monitor benchmarks configured." >&2
    return 0
  fi

  local i
  for ((i=0; i<${#MONITOR_BENCHES[@]}; i++)); do
    local monitor_bench="${MONITOR_BENCHES[$i]}"
    local now
    now="$(date +%s)"
    local elapsed=$((now - stage_start))
    if (( MONITOR_BUDGET_SECONDS > 0 && elapsed >= MONITOR_BUDGET_SECONDS )); then
      echo "WARN Monitor budget exhausted (${MONITOR_BUDGET_SECONDS}s), skipping remaining monitor cases." >&2
      local j
      for ((j=i; j<${#MONITOR_BENCHES[@]}; j++)); do
        append_case_log "monitor" "${MONITOR_BENCHES[$j]}" "$MONITOR_THRESHOLD_PCT" "skipped" "0" "" "budget_exhausted"
      done
      break
    fi

    run_repro_case \
      "$monitor_bench" \
      "$MONITOR_THRESHOLD_PCT" \
      "monitor" \
      0 \
      "$MONITOR_CASE_TIMEOUT_SECONDS" || true
  done
}

echo "== CI benchmark gate (strict + monitor with budget) =="
echo "Strict case: $STRICT_BENCH (threshold ${STRICT_THRESHOLD_PCT}%)"
if (( ${#MONITOR_BENCHES[@]} > 0 )); then
  echo "Monitor cases: ${MONITOR_BENCHES[*]} (threshold ${MONITOR_THRESHOLD_PCT}%)"
else
  echo "Monitor cases: none"
fi
echo "Runs per case: $RUNS"
echo "Monitor budget (s): $MONITOR_BUDGET_SECONDS"
echo "Monitor case timeout (s): $MONITOR_CASE_TIMEOUT_SECONDS"
echo

echo "-- Strict case --"
run_repro_case "$STRICT_BENCH" "$STRICT_THRESHOLD_PCT" "strict" 1 0 || true
echo
echo "-- Monitor cases --"
run_monitor_stage
echo

python3 - "$SUMMARY_PATH" "$CASE_LOG" "$MONITOR_BUDGET_SECONDS" "${REPORTS[@]}" <<'PY'
import csv
import re
import sys
from pathlib import Path

summary_path = Path(sys.argv[1])
case_log_path = Path(sys.argv[2])
monitor_budget_seconds = int(sys.argv[3])
reports = [Path(p) for p in sys.argv[4:] if p]

summary_path.parent.mkdir(parents=True, exist_ok=True)

def extract(label: str, text: str) -> str:
    pattern = rf"- {re.escape(label)}: (.+)"
    m = re.search(pattern, text)
    return m.group(1).strip() if m else "n/a"

cases = []
if case_log_path.exists():
    with case_log_path.open("r", encoding="utf-8") as fh:
        reader = csv.reader(fh, delimiter="\t")
        for row in reader:
            if len(row) != 7:
                continue
            cases.append(
                {
                    "stage": row[0],
                    "bench": row[1],
                    "threshold": row[2],
                    "status": row[3],
                    "duration_seconds": row[4],
                    "report_path": row[5],
                    "note": row[6],
                }
            )

case_by_report = {case["report_path"]: case for case in cases if case["report_path"]}

with summary_path.open("w", encoding="utf-8") as fh:
    fh.write("# CI Benchmark Gate Summary\n\n")
    fh.write(f"Generated reports: {len(reports)}  \n")
    fh.write(f"Monitor budget (s): {monitor_budget_seconds}\n\n")

    fh.write("## Case status\n\n")
    fh.write("| Stage | Benchmark | Threshold (%) | Status | Duration (s) | Note |\n")
    fh.write("|---|---|---:|---|---:|---|\n")
    if cases:
        for case in cases:
            fh.write(
                "| "
                + " | ".join(
                    [
                        case["stage"],
                        f"`{case['bench']}`",
                        case["threshold"],
                        case["status"],
                        case["duration_seconds"] if case["duration_seconds"] else "0",
                        case["note"] if case["note"] else "",
                    ]
                )
                + " |\n"
            )
    else:
        fh.write("| _none_ | _none_ | n/a | n/a | 0 | |\n")

    fh.write("\n## Reproducibility metrics\n\n")
    fh.write("| Stage | Report | Metrics | Above threshold | Median spread | Max spread |\n")
    fh.write("|---|---|---:|---:|---:|---:|\n")
    if reports:
        for report in reports:
            text = report.read_text(encoding="utf-8")
            metrics = extract("Common benchmark metrics", text)
            unstable = extract("Metrics above spread threshold", text)
            median = extract("Median spread", text)
            max_spread = extract("Max spread", text)
            case = case_by_report.get(str(report), {})
            stage = case.get("stage", "n/a")
            fh.write(
                f"| {stage} | `{report.name}` | {metrics} | {unstable} | {median} | {max_spread} |\n"
            )
    else:
        fh.write("| n/a | _none_ | n/a | n/a | n/a | n/a |\n")

print(f"CI benchmark gate summary: {summary_path}")
PY

if (( STRICT_FAILED != 0 )); then
  echo "CI benchmark gate failed due to strict reproducibility case." >&2
  exit 1
fi

echo "CI benchmark gate completed successfully."
