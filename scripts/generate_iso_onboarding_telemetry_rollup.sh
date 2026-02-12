#!/usr/bin/env bash
# Generate rolling onboarding telemetry trend report from ISO summary artifacts.
# Usage:
#   ./scripts/generate_iso_onboarding_telemetry_rollup.sh
#   ./scripts/generate_iso_onboarding_telemetry_rollup.sh --window 30
#   ./scripts/generate_iso_onboarding_telemetry_rollup.sh --output-json analysis/benchmark_reproducibility/custom_rollup.json

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
WINDOW=30
OUTPUT_JSON=""
OUTPUT_MD=""
LATEST_JSON=""
LATEST_MD=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/generate_iso_onboarding_telemetry_rollup.sh [options]

Options:
  --analysis-dir <path>   Directory containing iso_onboarding_telemetry_summary_*.json
  --window <n>            Number of newest summary runs to aggregate (default: 30)
  --output-json <path>    Output JSON rollup path (default: timestamped in analysis dir)
  --output-md <path>      Output Markdown rollup path (default: alongside output JSON)
  --latest-json <path>    Latest JSON alias path (default: analysis dir latest path)
  --latest-md <path>      Latest Markdown alias path (default: analysis dir latest path)
  -h, --help              Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --window)
      WINDOW="${2:-}"
      shift 2
      ;;
    --output-json)
      OUTPUT_JSON="${2:-}"
      shift 2
      ;;
    --output-md)
      OUTPUT_MD="${2:-}"
      shift 2
      ;;
    --latest-json)
      LATEST_JSON="${2:-}"
      shift 2
      ;;
    --latest-md)
      LATEST_MD="${2:-}"
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

if ! [[ "$WINDOW" =~ ^[0-9]+$ ]] || (( WINDOW < 1 )); then
  echo "Error: --window must be an integer >= 1" >&2
  exit 1
fi

if [[ -z "$OUTPUT_JSON" ]]; then
  TS="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_JSON="$ANALYSIS_DIR/iso_onboarding_telemetry_rollup_${TS}.json"
fi
if [[ -z "$OUTPUT_MD" ]]; then
  OUTPUT_MD="${OUTPUT_JSON%.json}.md"
fi
if [[ -z "$LATEST_JSON" ]]; then
  LATEST_JSON="$ANALYSIS_DIR/iso_onboarding_telemetry_rollup_latest.json"
fi
if [[ -z "$LATEST_MD" ]]; then
  LATEST_MD="$ANALYSIS_DIR/iso_onboarding_telemetry_rollup_latest.md"
fi

mkdir -p "$(dirname "$OUTPUT_JSON")"
mkdir -p "$(dirname "$OUTPUT_MD")"
mkdir -p "$(dirname "$LATEST_JSON")"
mkdir -p "$(dirname "$LATEST_MD")"

python3 - "$ANALYSIS_DIR" "$WINDOW" "$OUTPUT_JSON" "$OUTPUT_MD" "$LATEST_JSON" "$LATEST_MD" <<'PY'
import json
import re
import statistics
import sys
from collections import Counter
from pathlib import Path

analysis_dir = Path(sys.argv[1])
window = int(sys.argv[2])
output_json = Path(sys.argv[3])
output_md = Path(sys.argv[4])
latest_json = Path(sys.argv[5])
latest_md = Path(sys.argv[6])

pattern = re.compile(r"^iso_onboarding_telemetry_summary_(\d{8}T\d{6}Z)\.json$")

def parse_summary(path: Path) -> dict | None:
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        return None
    if payload.get("schema") != "vantis.iso.onboarding_telemetry_summary.v1":
        return None
    aggregate = payload.get("aggregate", {})
    boot = payload.get("boot_log", {})
    reboot = payload.get("reboot_log", {})
    return {
        "path": str(path),
        "timestamp": path.stem.split("_")[-1],
        "lockout_triggered": bool(aggregate.get("lockout_triggered", False)),
        "max_failures_observed": int(aggregate.get("max_failures_observed", 0)),
        "final_onboarding_source": str(aggregate.get("final_onboarding_source", "unknown")),
        "final_telemetry_last_event": str(aggregate.get("final_telemetry_last_event", "unknown")),
        "history_contains_lockout": bool(aggregate.get("history_contains_lockout", False)),
        "history_contains_guard_cleared": bool(aggregate.get("history_contains_guard_cleared", False)),
        "boot_failed_attempt_messages": int(boot.get("failed_attempt_messages", 0)),
        "boot_lockout_messages": int(boot.get("lockout_messages", 0)),
        "reboot_failed_attempt_messages": int(reboot.get("failed_attempt_messages", 0)),
        "reboot_lockout_messages": int(reboot.get("lockout_messages", 0)),
    }

if not analysis_dir.exists():
    raise SystemExit(f"analysis directory not found: {analysis_dir}")

summaries = []
for path in sorted(analysis_dir.glob("iso_onboarding_telemetry_summary_*.json")):
    match = pattern.match(path.name)
    if not match:
        continue
    parsed = parse_summary(path)
    if parsed is None:
        continue
    summaries.append(parsed)

summaries.sort(key=lambda entry: entry["timestamp"])
windowed = summaries[-window:]

total_runs = len(windowed)
source_counter = Counter(entry["final_onboarding_source"] for entry in windowed)
event_counter = Counter(entry["final_telemetry_last_event"] for entry in windowed)
lockout_runs = sum(1 for entry in windowed if entry["lockout_triggered"])
guard_cleared_runs = sum(1 for entry in windowed if entry["history_contains_guard_cleared"])
max_failures_values = [entry["max_failures_observed"] for entry in windowed]

latest = windowed[-1] if windowed else {}

rollup = {
    "schema": "vantis.iso.onboarding_telemetry_rollup.v1",
    "generated_unix": int(Path(output_json.parent).stat().st_mtime),
    "analysis_dir": str(analysis_dir),
    "window_size_requested": window,
    "window_size_effective": total_runs,
    "inputs": [entry["path"] for entry in windowed],
    "aggregate": {
        "lockout_runs": lockout_runs,
        "lockout_run_ratio": (lockout_runs / total_runs) if total_runs else 0.0,
        "guard_cleared_runs": guard_cleared_runs,
        "max_failures_peak": max(max_failures_values) if max_failures_values else 0,
        "max_failures_mean": (statistics.fmean(max_failures_values) if max_failures_values else 0.0),
        "final_onboarding_source_distribution": dict(source_counter),
        "final_telemetry_last_event_distribution": dict(event_counter),
        "latest_run": latest,
    },
    "runs": windowed,
}

output_json.write_text(json.dumps(rollup, indent=2) + "\n", encoding="utf-8")
latest_json.write_text(json.dumps(rollup, indent=2) + "\n", encoding="utf-8")

lines = [
    "# ISO Onboarding Telemetry Rollup",
    "",
    f"- Source directory: `{analysis_dir}`",
    f"- Window requested: `{window}`",
    f"- Runs aggregated: `{total_runs}`",
    f"- Lockout runs: `{lockout_runs}`",
    f"- Lockout ratio: `{rollup['aggregate']['lockout_run_ratio']:.2%}`",
    f"- Guard-cleared runs: `{guard_cleared_runs}`",
    f"- Max failures peak: `{rollup['aggregate']['max_failures_peak']}`",
    f"- Mean max failures: `{rollup['aggregate']['max_failures_mean']:.2f}`",
    "",
    "## Final onboarding source distribution",
    "",
]
if source_counter:
    for key, value in sorted(source_counter.items()):
        lines.append(f"- `{key}`: `{value}`")
else:
    lines.append("- no runs")

lines.extend([
    "",
    "## Final telemetry last_event distribution",
    "",
])
if event_counter:
    for key, value in sorted(event_counter.items()):
        lines.append(f"- `{key}`: `{value}`")
else:
    lines.append("- no runs")

lines.extend([
    "",
    "## Recent runs",
    "",
    "| timestamp | lockout | max_failures | final_source | final_last_event |",
    "|---|---:|---:|---|---|",
])
if windowed:
    for entry in reversed(windowed[-10:]):
        lines.append(
            f"| `{entry['timestamp']}` | `{entry['lockout_triggered']}` | `{entry['max_failures_observed']}` | "
            f"`{entry['final_onboarding_source']}` | `{entry['final_telemetry_last_event']}` |"
        )
else:
    lines.append("| n/a | n/a | n/a | n/a | n/a |")

text = "\n".join(lines) + "\n"
output_md.write_text(text, encoding="utf-8")
latest_md.write_text(text, encoding="utf-8")
PY

echo "Onboarding telemetry rollup JSON: $OUTPUT_JSON"
echo "Onboarding telemetry rollup Markdown: $OUTPUT_MD"
echo "Onboarding telemetry rollup latest JSON: $LATEST_JSON"
echo "Onboarding telemetry rollup latest Markdown: $LATEST_MD"
