#!/usr/bin/env bash
# Generate governance-ready MONPOL threshold proposal draft from rolling evidence.
# Usage:
#   ./scripts/generate_monitor_threshold_proposal.sh
#   ./scripts/generate_monitor_threshold_proposal.sh --proposal-id MONPOL-002 --bench timer_queue_benchmark

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
CHANGELOG_PATH="$ROOT/governance/performance/MONITOR_THRESHOLD_CHANGELOG.md"
RECOMMENDATION_JSON=""
DASHBOARD_JSON=""
PROPOSAL_ID=""
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""
BENCH_FILTERS=()

usage() {
  cat <<'USAGE'
Usage: ./scripts/generate_monitor_threshold_proposal.sh [options]

Options:
  --analysis-dir <path>         Analysis directory (default: analysis/benchmark_reproducibility)
  --changelog <path>            Threshold changelog file path
  --recommendation-json <path>  Recommendation JSON snapshot (defaults to newest)
  --dashboard-json <path>       Dashboard JSON snapshot (defaults to newest)
  --proposal-id <MONPOL-NNN>    Explicit proposal ID (default: next from changelog)
  --bench <name>                Include only selected benchmark(s) (repeatable)
  --output <path>               Output markdown proposal path
  --output-json <path>          Output JSON proposal path
  -h, --help                    Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --changelog)
      CHANGELOG_PATH="${2:-}"
      shift 2
      ;;
    --recommendation-json)
      RECOMMENDATION_JSON="${2:-}"
      shift 2
      ;;
    --dashboard-json)
      DASHBOARD_JSON="${2:-}"
      shift 2
      ;;
    --proposal-id)
      PROPOSAL_ID="${2:-}"
      shift 2
      ;;
    --bench)
      BENCH_FILTERS+=("${2:-}")
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
  echo "Error: analysis dir not found: $ANALYSIS_DIR" >&2
  exit 1
fi

if [[ ! -f "$CHANGELOG_PATH" ]]; then
  echo "Error: changelog not found: $CHANGELOG_PATH" >&2
  exit 1
fi

if [[ -z "$RECOMMENDATION_JSON" ]]; then
  shopt -s nullglob
  recommendation_candidates=("$ANALYSIS_DIR"/monitor_policy_recommendations_*.json)
  shopt -u nullglob
  if (( ${#recommendation_candidates[@]} == 0 )); then
    echo "Error: no monitor_policy_recommendations_*.json found in $ANALYSIS_DIR" >&2
    exit 1
  fi
  readarray -t sorted_recommendations < <(printf '%s\n' "${recommendation_candidates[@]}" | sort)
  RECOMMENDATION_JSON="${sorted_recommendations[${#sorted_recommendations[@]}-1]}"
fi

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

if [[ ! -f "$RECOMMENDATION_JSON" ]]; then
  echo "Error: recommendation json not found: $RECOMMENDATION_JSON" >&2
  exit 1
fi

if [[ ! -f "$DASHBOARD_JSON" ]]; then
  echo "Error: dashboard json not found: $DASHBOARD_JSON" >&2
  exit 1
fi

FILTER_CSV=""
if (( ${#BENCH_FILTERS[@]} > 0 )); then
  FILTER_CSV="$(IFS=,; echo "${BENCH_FILTERS[*]}")"
fi

python3 - "$ROOT" "$CHANGELOG_PATH" "$RECOMMENDATION_JSON" "$DASHBOARD_JSON" "$PROPOSAL_ID" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" "$FILTER_CSV" <<'PY'
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
changelog_path = Path(sys.argv[2])
recommendation_path = Path(sys.argv[3])
dashboard_path = Path(sys.argv[4])
proposal_id_arg = sys.argv[5].strip()
output_path_arg = sys.argv[6].strip()
output_json_path_arg = sys.argv[7].strip()
filter_csv = sys.argv[8]
bench_filter = {entry.strip() for entry in filter_csv.split(",") if entry.strip()}


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(root.resolve()))
    except ValueError:
        return str(path)


def parse_next_monpol_id(changelog_text: str) -> str:
    ids = [int(match.group(1)) for match in re.finditer(r"MONPOL-(\d{3})", changelog_text)]
    next_id = (max(ids) + 1) if ids else 1
    return f"MONPOL-{next_id:03d}"


def fmt_number(value: float) -> str:
    text = f"{value:.2f}"
    text = text.rstrip("0").rstrip(".")
    return text if text else "0"


changelog_text = changelog_path.read_text(encoding="utf-8")
proposal_id = proposal_id_arg or parse_next_monpol_id(changelog_text)
if not re.fullmatch(r"MONPOL-\d{3}", proposal_id):
    raise SystemExit(f"Invalid proposal ID format: {proposal_id}")

recommendation_payload = json.loads(recommendation_path.read_text(encoding="utf-8"))
dashboard_payload = json.loads(dashboard_path.read_text(encoding="utf-8"))

recommendations = recommendation_payload.get("recommendations", [])
if bench_filter:
    recommendations = [item for item in recommendations if item.get("bench") in bench_filter]

if not recommendations:
    raise SystemExit("No recommendations available for proposal generation after filtering")

bench_health = dashboard_payload.get("bench_health", {})
latest_recommendation = dashboard_payload.get("latest_recommendation") or {}
recent_runs = dashboard_payload.get("recent_runs", [])
latest_run = recent_runs[-1] if recent_runs else {}

proposal_timestamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
generated_at_iso = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")

if output_path_arg:
    output_path = Path(output_path_arg)
else:
    output_path = root / "analysis" / "benchmark_reproducibility" / f"monitor_threshold_proposal_{proposal_id}_{proposal_timestamp}.md"

if output_json_path_arg:
    output_json_path = Path(output_json_path_arg)
else:
    output_json_path = output_path.with_suffix(".json")

output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

rows = []
proposal_changes = []
for item in sorted(recommendations, key=lambda value: value.get("bench", "")):
    bench = item.get("bench", "n/a")
    action = str(item.get("action", "n/a"))
    latest_threshold = float(item.get("latest_threshold_pct", 0.0))
    recommended = float(item.get("recommended_threshold_pct", latest_threshold))
    delta = round(recommended - latest_threshold, 2)
    drift_reports = int(item.get("drift_reports", 0))
    drift_rate_pct = float(item.get("drift_rate_pct", 0.0))
    latest_report = item.get("latest_report", "")
    health = bench_health.get(bench, {})
    latest_status = health.get("latest_status", "n/a")
    failures = int(health.get("failures", 0))
    timeouts = int(health.get("timeouts", 0))

    if action in {"tighten", "relax"}:
        proposal_changes.append(
            {
                "bench": bench,
                "from_threshold_pct": latest_threshold,
                "to_threshold_pct": recommended,
                "delta_pct": delta,
                "action": action,
            }
        )

    rows.append(
        {
            "bench": bench,
            "latest_threshold_pct": latest_threshold,
            "recommended_threshold_pct": recommended,
            "delta_pct": delta,
            "action": action,
            "drift_reports": drift_reports,
            "drift_rate_pct": drift_rate_pct,
            "latest_status": latest_status,
            "failure_count": failures,
            "timeout_count": timeouts,
            "latest_report": latest_report,
        }
    )

if not proposal_changes:
    proposal_changes.append(
        {
            "bench": "none",
            "from_threshold_pct": 0.0,
            "to_threshold_pct": 0.0,
            "delta_pct": 0.0,
            "action": "hold",
        }
    )

monitor_flag_suggestions = []
for row in rows:
    target_threshold = row["recommended_threshold_pct"] if row["action"] in {"tighten", "relax"} else row["latest_threshold_pct"]
    monitor_flag_suggestions.append(f"--monitor-bench {row['bench']}:{fmt_number(target_threshold)}")

evidence_files = [
    rel(recommendation_path),
    rel(dashboard_path),
]
if latest_recommendation:
    evidence_files.append(
        rel(root / "analysis" / "benchmark_reproducibility" / str(latest_recommendation.get("file", "")))
    )
if latest_run.get("file"):
    evidence_files.append(rel(root / "analysis" / "benchmark_reproducibility" / str(latest_run["file"])))
for row in rows:
    if row["latest_report"]:
        evidence_files.append(rel(root / "analysis" / "benchmark_reproducibility" / row["latest_report"]))

# Keep order and deduplicate.
seen = set()
dedup_evidence = []
for entry in evidence_files:
    if not entry or entry in seen:
        continue
    seen.add(entry)
    dedup_evidence.append(entry)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write(f"# Monitor Threshold Change Proposal Draft: `{proposal_id}`\n\n")
    fh.write(f"**Status**: Draft  \n")
    fh.write(f"**Generated at (UTC)**: {generated_at_iso}  \n")
    fh.write(f"**Proposal ID**: `{proposal_id}`  \n")
    fh.write(f"**Changelog target**: `{rel(changelog_path)}`\n\n")

    fh.write("## Inputs\n\n")
    fh.write(f"- Recommendation snapshot: `{rel(recommendation_path)}`\n")
    fh.write(f"- Dashboard snapshot: `{rel(dashboard_path)}`\n")
    benches_joined = ", ".join(f"`{row['bench']}`" for row in rows)
    fh.write(f"- Benchmarks included: {benches_joined}\n\n")

    fh.write("## Proposed Threshold Review Table\n\n")
    fh.write(
        "| Benchmark | Current (%) | Recommended (%) | Delta (%) | Action | Drift reports | Drift rate (%) | Latest status | Failures | Timeouts | Latest report |\n"
    )
    fh.write("|---|---:|---:|---:|---|---:|---:|---|---:|---:|---|\n")
    for row in rows:
        fh.write(
            "| "
            + " | ".join(
                [
                    f"`{row['bench']}`",
                    fmt_number(row["latest_threshold_pct"]),
                    fmt_number(row["recommended_threshold_pct"]),
                    fmt_number(row["delta_pct"]),
                    row["action"],
                    str(row["drift_reports"]),
                    fmt_number(row["drift_rate_pct"]),
                    row["latest_status"],
                    str(row["failure_count"]),
                    str(row["timeout_count"]),
                    f"`{row['latest_report']}`" if row["latest_report"] else "n/a",
                ]
            )
            + " |\n"
        )
    fh.write("\n")

    fh.write("## Suggested CI Monitor Flags\n\n")
    fh.write("```bash\n")
    for line in monitor_flag_suggestions:
        fh.write(f"{line}\n")
    fh.write("```\n\n")

    fh.write("## Governance Checklist\n\n")
    fh.write("- [ ] Confirm final decision for each benchmark (`tighten` / `hold` / `relax`).\n")
    fh.write("- [ ] Update `.github/workflows/ci.yml` monitor thresholds.\n")
    fh.write("- [ ] Update `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md`.\n")
    fh.write(f"- [ ] Add `{proposal_id}` entry to `governance/performance/MONITOR_THRESHOLD_CHANGELOG.md`.\n")
    fh.write(f"- [ ] Include `{proposal_id}` in PR title or body.\n")
    fh.write("- [ ] Link evidence bundle below in PR description.\n\n")

    fh.write("## Evidence Bundle Links\n\n")
    for entry in dedup_evidence:
        fh.write(f"- `{entry}`\n")
    fh.write("\n")
    fh.write("This draft is advisory and intended to accelerate governance-ready PR preparation.\n")

proposal_json = {
    "proposal_id": proposal_id,
    "status": "draft",
    "generated_at_utc": generated_at_iso,
    "inputs": {
        "recommendation_json": rel(recommendation_path),
        "dashboard_json": rel(dashboard_path),
        "changelog_path": rel(changelog_path),
    },
    "bench_filter": sorted(bench_filter),
    "rows": rows,
    "proposal_changes": proposal_changes,
    "suggested_monitor_flags": monitor_flag_suggestions,
    "evidence_bundle": dedup_evidence,
}
output_json_path.write_text(json.dumps(proposal_json, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Monitor threshold proposal markdown: {output_path}")
print(f"Monitor threshold proposal JSON: {output_json_path}")
print(f"Proposal ID: {proposal_id}")
print(f"Benchmarks included: {len(rows)}")
PY
