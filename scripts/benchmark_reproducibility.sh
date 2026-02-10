#!/usr/bin/env bash
# Run repeated benchmark passes with a reproducibility profile.
# Usage:
#   ./scripts/benchmark_reproducibility.sh --bench timer_queue_benchmark
#   ./scripts/benchmark_reproducibility.sh --bench ipc_complete_benchmark --runs 2 --strict

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CRATE_DIR="$ROOT/src/verified"
CRITERION_DIR="$CRATE_DIR/target/criterion"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"

BENCH=""
RUNS=3
BASELINE_PREFIX="repro"
RETAIN_FAMILIES=5
SPREAD_THRESHOLD_PCT=5
STRICT=0

usage() {
  cat <<'USAGE'
Usage: ./scripts/benchmark_reproducibility.sh --bench <name> [options]

Options:
  --bench <name>              Criterion bench target (required)
  --runs <n>                  Number of repeated runs (default: 3)
  --baseline-prefix <prefix>  Baseline family prefix (default: repro)
  --retain-families <n>       Keep newest baseline families per case (default: 5)
  --spread-threshold-pct <n>  Spread threshold for unstable marker (default: 5)
  --strict                    Exit non-zero when any metric exceeds threshold
  -h, --help                  Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bench)
      BENCH="${2:-}"
      shift 2
      ;;
    --runs)
      RUNS="${2:-}"
      shift 2
      ;;
    --baseline-prefix)
      BASELINE_PREFIX="${2:-}"
      shift 2
      ;;
    --retain-families)
      RETAIN_FAMILIES="${2:-}"
      shift 2
      ;;
    --spread-threshold-pct)
      SPREAD_THRESHOLD_PCT="${2:-}"
      shift 2
      ;;
    --strict)
      STRICT=1
      shift
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

if [[ -z "$BENCH" ]]; then
  echo "Error: --bench is required" >&2
  usage
  exit 1
fi

if ! [[ "$RUNS" =~ ^[0-9]+$ ]] || (( RUNS < 2 )); then
  echo "Error: --runs must be an integer >= 2" >&2
  exit 1
fi

if ! [[ "$RETAIN_FAMILIES" =~ ^[0-9]+$ ]] || (( RETAIN_FAMILIES < 1 )); then
  echo "Error: --retain-families must be an integer >= 1" >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo is required but not installed" >&2
  exit 1
fi

GOVERNOR_FILE="/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor"
if [[ -f "$GOVERNOR_FILE" ]]; then
  GOVERNOR="$(<"$GOVERNOR_FILE")"
  echo "Detected CPU governor: $GOVERNOR"
  if [[ "$GOVERNOR" != "performance" ]]; then
    echo "WARN: Use 'performance' governor for tighter reproducibility." >&2
  fi
else
  echo "INFO: CPU governor file not found; skipping governor check."
fi

TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
FAMILY="${BASELINE_PREFIX}_${BENCH}_${TIMESTAMP}"
REPORT_PATH="$ANALYSIS_DIR/${BENCH}_${TIMESTAMP}.md"

mkdir -p "$ANALYSIS_DIR"
cd "$CRATE_DIR"

# Reproducibility-oriented cargo profile overrides.
export CARGO_INCREMENTAL=0
export CARGO_PROFILE_BENCH_CODEGEN_UNITS=1
export CARGO_PROFILE_BENCH_LTO=true
export CARGO_PROFILE_BENCH_DEBUG=false

echo "== Benchmark reproducibility run =="
echo "Bench target: $BENCH"
echo "Runs: $RUNS"
echo "Baseline family: $FAMILY"
echo "Spread threshold (%): $SPREAD_THRESHOLD_PCT"
echo

for ((run=1; run<=RUNS; run++)); do
  BASELINE="${FAMILY}_run${run}"
  echo "== Run ${run}/${RUNS}: baseline ${BASELINE} =="
  cargo bench --bench "$BENCH" -- --save-baseline "$BASELINE"
  echo
done

python3 - "$CRITERION_DIR" "$FAMILY" "$RUNS" "$REPORT_PATH" "$SPREAD_THRESHOLD_PCT" "$BASELINE_PREFIX" "$BENCH" "$RETAIN_FAMILIES" "$STRICT" <<'PY'
import json
import re
import shutil
import statistics
import sys
from collections import defaultdict
from pathlib import Path

criterion_root = Path(sys.argv[1])
family = sys.argv[2]
runs = int(sys.argv[3])
report_path = Path(sys.argv[4])
threshold = float(sys.argv[5])
prefix = sys.argv[6]
bench = sys.argv[7]
retain_families = int(sys.argv[8])
strict = bool(int(sys.argv[9]))


def read_point_estimate(path: Path) -> float:
    payload = json.loads(path.read_text(encoding="utf-8"))
    return float(payload["mean"]["point_estimate"])


run_values = {}
for run in range(1, runs + 1):
    baseline = f"{family}_run{run}"
    values = {}
    for estimate in criterion_root.rglob("estimates.json"):
        if estimate.parent.name != baseline:
            continue
        key = estimate.parent.parent.relative_to(criterion_root).as_posix()
        values[key] = read_point_estimate(estimate)
    run_values[run] = values

if any(not values for values in run_values.values()):
    missing = [str(run) for run, values in run_values.items() if not values]
    raise SystemExit(f"No estimates captured for run(s): {', '.join(missing)}")

common_keys = set.intersection(*(set(values.keys()) for values in run_values.values()))
if not common_keys:
    raise SystemExit("No common benchmark estimate keys across all runs")

rows = []
for key in sorted(common_keys):
    values = [run_values[run][key] for run in range(1, runs + 1)]
    mean = statistics.fmean(values)
    stdev = statistics.pstdev(values) if len(values) > 1 else 0.0
    cv_pct = (stdev / mean * 100.0) if mean else 0.0
    spread_pct = ((max(values) - min(values)) / mean * 100.0) if mean else 0.0
    rows.append((key, values, mean, cv_pct, spread_pct))

rows.sort(key=lambda row: row[4], reverse=True)
spread_values = [row[4] for row in rows]
cv_values = [row[3] for row in rows]
unstable = [row for row in rows if row[4] > threshold]

report_path.parent.mkdir(parents=True, exist_ok=True)
with report_path.open("w", encoding="utf-8") as fh:
    fh.write(f"# Benchmark Reproducibility Report: `{bench}`\n\n")
    fh.write(f"**Baseline family**: `{family}`  \n")
    fh.write(f"**Runs**: {runs}  \n")
    fh.write(f"**Spread threshold**: {threshold:.2f}%  \n")
    fh.write(f"**Criterion root**: `{criterion_root}`\n\n")
    fh.write("---\n\n")
    fh.write("## Summary\n\n")
    fh.write(f"- Common benchmark metrics: {len(rows)}\n")
    fh.write(f"- Metrics above spread threshold: {len(unstable)}\n")
    fh.write(f"- Median spread: {statistics.median(spread_values):.3f}%\n")
    fh.write(f"- Max spread: {max(spread_values):.3f}%\n")
    fh.write(f"- Median CV: {statistics.median(cv_values):.3f}%\n")
    fh.write(f"- Max CV: {max(cv_values):.3f}%\n\n")

    fh.write("## Top spread metrics\n\n")
    headers = ["Metric"] + [f"run{i}" for i in range(1, runs + 1)] + ["mean", "cv_pct", "spread_pct"]
    fh.write("| " + " | ".join(headers) + " |\n")
    fh.write("|" + "|".join(["---"] * len(headers)) + "|\n")

    for key, values, mean, cv_pct, spread_pct in rows[:20]:
        fields = [key] + [f"{value:.6f}" for value in values] + [f"{mean:.6f}", f"{cv_pct:.3f}", f"{spread_pct:.3f}"]
        fh.write("| " + " | ".join(fields) + " |\n")

    fh.write("\n")
    fh.write("Note: Criterion point estimates are raw benchmark time units (typically ns).\n")


# Baseline retention policy:
# Keep the newest N families for this bench+prefix per benchmark case directory.
pattern = re.compile(
    rf"^{re.escape(prefix)}_{re.escape(bench)}_\d{{8}}T\d{{6}}Z_run\d+$"
)
case_families = defaultdict(lambda: defaultdict(list))

for directory in criterion_root.rglob("*"):
    if not directory.is_dir():
        continue
    if not pattern.match(directory.name):
        continue
    family_name = re.sub(r"_run\d+$", "", directory.name)
    case_families[directory.parent][family_name].append(directory)

pruned_dirs = 0
for case_dir, families in case_families.items():
    ordered = sorted(
        families.keys(),
        key=lambda name: max(path.stat().st_mtime for path in families[name]),
        reverse=True,
    )
    for family_name in ordered[retain_families:]:
        for path in families[family_name]:
            shutil.rmtree(path, ignore_errors=True)
            pruned_dirs += 1

print(f"Reproducibility report written to: {report_path}")
print(f"Common metrics: {len(rows)}")
print(f"Metrics over spread threshold ({threshold:.2f}%): {len(unstable)}")
print(f"Median spread: {statistics.median(spread_values):.3f}%")
print(f"Max spread: {max(spread_values):.3f}%")
print(f"Pruned baseline directories: {pruned_dirs}")

if strict and unstable:
    raise SystemExit(
        f"Strict mode failure: {len(unstable)} metrics exceed spread threshold"
    )
PY
