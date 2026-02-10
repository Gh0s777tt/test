#!/usr/bin/env bash
# Build monitor policy drift dashboard from benchmark gate summaries and policy recommendations.
# Usage:
#   ./scripts/build_monitor_policy_dashboard.sh
#   ./scripts/build_monitor_policy_dashboard.sh --lookback-runs 10 --lookback-recommendations 6

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
CHANGELOG_PATH="$ROOT/governance/performance/MONITOR_THRESHOLD_CHANGELOG.md"
SIGNOFF_PATH="$ROOT/governance/performance/MONPOL_SIGNOFFS.json"

LOOKBACK_RUNS=10
LOOKBACK_RECOMMENDATIONS=6
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/build_monitor_policy_dashboard.sh [options]

Options:
  --analysis-dir <path>              Directory with benchmark reproducibility artifacts
  --changelog <path>                 MONPOL changelog path
  --signoff <path>                   MONPOL signoff metadata JSON path
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
    --changelog)
      CHANGELOG_PATH="${2:-}"
      shift 2
      ;;
    --signoff)
      SIGNOFF_PATH="${2:-}"
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

if [[ ! -f "$CHANGELOG_PATH" ]]; then
  echo "Error: changelog not found: $CHANGELOG_PATH" >&2
  exit 1
fi

if [[ ! -f "$SIGNOFF_PATH" ]]; then
  echo "Error: signoff metadata not found: $SIGNOFF_PATH" >&2
  exit 1
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/monitor_policy_dashboard_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - "$ANALYSIS_DIR" "$CHANGELOG_PATH" "$SIGNOFF_PATH" "$LOOKBACK_RUNS" "$LOOKBACK_RECOMMENDATIONS" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" <<'PY'
import json
import re
import sys
from datetime import datetime, timezone
from statistics import median
from pathlib import Path

analysis_dir = Path(sys.argv[1])
changelog_path = Path(sys.argv[2])
signoff_path = Path(sys.argv[3])
lookback_runs = int(sys.argv[4])
lookback_recommendations = int(sys.argv[5])
output_path = Path(sys.argv[6])
output_json_path = Path(sys.argv[7])

summary_re = re.compile(r"^ci_benchmark_gate_summary_(\d{8}T\d{6}Z)\.md$")
recommend_re = re.compile(r"^monitor_policy_recommendations_(\d{8}T\d{6}Z)\.json$")
proposal_re = re.compile(r"^monitor_threshold_proposal_(MONPOL-\d{3})_(\d{8}T\d{6}Z)\.json$")

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
    monitor_case_drift = sum(1 for case in monitor_cases if case["status"] == "drift")
    monitor_metric_drift = sum(
        1
        for metric in metrics
        if metric["stage"] == "monitor"
        and metric["over_threshold"].isdigit()
        and int(metric["over_threshold"]) > 0
    )
    monitor_drift = max(monitor_case_drift, monitor_metric_drift)
    monitor_failures = sum(
        1 for case in monitor_cases if case["status"] in {"failed", "timeout", "missing_report"}
    )
    strict_status = strict_cases[0]["status"] if strict_cases else "unknown"
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


def normalize_decision(text: str) -> str:
    value = (text or "").strip().lower()
    if "approved" in value:
        return "approved"
    if "reject" in value:
        return "rejected"
    if "withdraw" in value:
        return "withdrawn"
    if "defer" in value:
        return "deferred"
    if "pending" in value:
        return "pending"
    return "unknown"


def parse_changelog(path: Path):
    text = path.read_text(encoding="utf-8")
    header_re = re.compile(r"^###\s+(MONPOL-\d{3})\s+\(([^)]+)\)\s*$", re.MULTILINE)
    headers = list(header_re.finditer(text))
    entries = []
    for idx, match in enumerate(headers):
        start = match.start()
        end = headers[idx + 1].start() if idx + 1 < len(headers) else len(text)
        block = text[start:end]
        decision_match = re.search(r"^- \*\*Decision\*\*:\s*(.+)$", block, re.MULTILINE)
        raw_decision = decision_match.group(1).strip() if decision_match else "n/a"
        entries.append(
            {
                "proposal_id": match.group(1),
                "date": match.group(2),
                "raw_decision": raw_decision,
                "decision": normalize_decision(raw_decision),
            }
        )
    return entries


def parse_signoff(path: Path):
    payload = json.loads(path.read_text(encoding="utf-8"))
    records = payload.get("records", [])
    if not isinstance(records, list):
        records = []
    normalized = []
    for record in records:
        if not isinstance(record, dict):
            continue
        proposal_id = str(record.get("proposal_id", "")).strip()
        decision = normalize_decision(str(record.get("decision", "")).strip())
        owner = str(record.get("owner", "")).strip() or "n/a"
        approved_at = str(record.get("approved_at_utc", "")).strip() or "n/a"
        reviewers = record.get("reviewers", [])
        reviewer_count = len(reviewers) if isinstance(reviewers, list) else 0
        normalized.append(
            {
                "proposal_id": proposal_id,
                "decision": decision,
                "owner": owner,
                "approved_at_utc": approved_at,
                "reviewer_count": reviewer_count,
            }
        )
    return normalized


def parse_iso_utc(value: str):
    if not value:
        return None
    normalized = value.strip()
    if not normalized:
        return None
    if normalized.endswith("Z"):
        normalized = normalized[:-1] + "+00:00"
    try:
        parsed = datetime.fromisoformat(normalized)
    except ValueError:
        return None
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed.astimezone(timezone.utc)


def parse_compact_utc(value: str):
    try:
        parsed = datetime.strptime(value, "%Y%m%dT%H%M%SZ")
    except ValueError:
        return None
    return parsed.replace(tzinfo=timezone.utc)


def parse_ymd(value: str):
    try:
        return datetime.strptime(value.strip(), "%Y-%m-%d").date()
    except ValueError:
        return None


def parse_proposal_generated_at(path: Path):
    payload_dt = None
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
        payload_dt = parse_iso_utc(str(payload.get("generated_at_utc", "")).strip())
    except (json.JSONDecodeError, OSError):
        payload_dt = None

    if payload_dt is not None:
        return payload_dt

    match = proposal_re.match(path.name)
    if match:
        return parse_compact_utc(match.group(2))
    return None


def percentile_nearest_rank(values, pct: float):
    if not values:
        return None
    rank = int((pct * len(values)) + 0.999999)
    rank = max(1, min(rank, len(values)))
    return values[rank - 1]


def collect_latency_telemetry(entries, directory: Path):
    proposal_candidates = sorted(directory.glob("monitor_threshold_proposal_MONPOL-*_*.json"))
    by_id = {}
    skipped_proposal_files = 0

    for path in proposal_candidates:
        match = proposal_re.match(path.name)
        if not match:
            continue
        proposal_id = match.group(1)
        generated_dt = parse_proposal_generated_at(path)
        if generated_dt is None:
            skipped_proposal_files += 1
            continue
        by_id.setdefault(proposal_id, []).append(
            {
                "file": path.name,
                "generated_dt": generated_dt,
                "generated_at_utc": generated_dt.isoformat().replace("+00:00", "Z"),
            }
        )

    for values in by_id.values():
        values.sort(key=lambda item: item["generated_dt"])

    rows = []
    latencies = []
    missing_proposal_artifacts = []
    invalid_merge_dates = []
    chronology_errors = []

    for entry in entries:
        proposal_id = entry.get("proposal_id", "n/a")
        merge_date_text = str(entry.get("date", "n/a"))
        merge_date = parse_ymd(merge_date_text)
        decision = entry.get("decision", "unknown")
        history = by_id.get(proposal_id, [])

        if merge_date is None:
            invalid_merge_dates.append(proposal_id)
            rows.append(
                {
                    "proposal_id": proposal_id,
                    "decision": decision,
                    "merge_date": merge_date_text,
                    "first_proposal_generated_at_utc": "n/a",
                    "latency_days": None,
                    "status": "invalid_merge_date",
                    "proposal_file": "n/a",
                    "proposal_history_files": 0,
                }
            )
            continue

        if not history:
            missing_proposal_artifacts.append(proposal_id)
            rows.append(
                {
                    "proposal_id": proposal_id,
                    "decision": decision,
                    "merge_date": merge_date_text,
                    "first_proposal_generated_at_utc": "n/a",
                    "latency_days": None,
                    "status": "missing_proposal_artifact",
                    "proposal_file": "n/a",
                    "proposal_history_files": 0,
                }
            )
            continue

        first = history[0]
        latency_days = (merge_date - first["generated_dt"].date()).days
        status = "ok"
        if latency_days < 0:
            status = "chronology_error"
            chronology_errors.append(proposal_id)
        else:
            latencies.append(latency_days)

        rows.append(
            {
                "proposal_id": proposal_id,
                "decision": decision,
                "merge_date": merge_date_text,
                "first_proposal_generated_at_utc": first["generated_at_utc"],
                "latency_days": latency_days,
                "status": status,
                "proposal_file": first["file"],
                "proposal_history_files": len(history),
            }
        )

    rows.sort(key=lambda item: (item.get("merge_date", "9999-99-99"), item.get("proposal_id", "n/a")))
    sorted_latencies = sorted(latencies)
    latest_ok = [row for row in rows if row.get("status") == "ok"]
    latest_ok.sort(key=lambda item: (item.get("merge_date", ""), item.get("proposal_id", "")))
    latest_ok_entry = latest_ok[-1] if latest_ok else None

    return {
        "summary": {
            "entries_evaluated": len(entries),
            "proposal_artifacts_discovered": len(proposal_candidates),
            "proposal_artifacts_skipped": skipped_proposal_files,
            "latency_samples": len(sorted_latencies),
            "median_latency_days": float(median(sorted_latencies)) if sorted_latencies else None,
            "p90_latency_days": percentile_nearest_rank(sorted_latencies, 0.90),
            "max_latency_days": max(sorted_latencies) if sorted_latencies else None,
            "missing_proposal_artifact_count": len(missing_proposal_artifacts),
            "invalid_merge_date_count": len(invalid_merge_dates),
            "chronology_error_count": len(chronology_errors),
        },
        "rows": rows,
        "missing_proposal_artifacts": missing_proposal_artifacts,
        "invalid_merge_dates": invalid_merge_dates,
        "chronology_errors": chronology_errors,
        "latest_ok_entry": latest_ok_entry,
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
changelog_entries = parse_changelog(changelog_path)
signoff_records = parse_signoff(signoff_path)
signoff_by_id = {record["proposal_id"]: record for record in signoff_records if record["proposal_id"]}
latency_telemetry = collect_latency_telemetry(changelog_entries, analysis_dir)

signoff_decision_counts = {}
for record in signoff_records:
    decision = record["decision"]
    signoff_decision_counts[decision] = signoff_decision_counts.get(decision, 0) + 1

approved_entries = [entry for entry in changelog_entries if entry["decision"] == "approved"]
approved_with_signoff = [
    entry["proposal_id"] for entry in approved_entries if entry["proposal_id"] in signoff_by_id
]
approved_missing_signoff = [
    entry["proposal_id"] for entry in approved_entries if entry["proposal_id"] not in signoff_by_id
]

latest_signoff_record = None
for record in signoff_records:
    current = record.get("approved_at_utc", "n/a")
    if current == "n/a":
        continue
    if latest_signoff_record is None or current > latest_signoff_record.get("approved_at_utc", "n/a"):
        latest_signoff_record = record

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

strict_fail_runs = sum(
    1 for item in summaries if item["strict_status"] in {"failed", "timeout", "missing_report"}
)
strict_unknown_runs = sum(1 for item in summaries if item["strict_status"] == "unknown")
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
    fh.write(f"- Strict status unknown runs: {strict_unknown_runs}\n")
    fh.write(f"- Runs with monitor drift cases: {monitor_drift_runs}\n")
    fh.write(f"- Runs with monitor failure/timeout/missing-report cases: {monitor_failure_runs}\n\n")

    fh.write("## MONPOL Signoff Telemetry\n\n")
    fh.write(f"- Changelog MONPOL entries: {len(changelog_entries)}\n")
    fh.write(f"- Changelog approved entries: {len(approved_entries)}\n")
    fh.write(f"- Approved entries with signoff metadata: {len(approved_with_signoff)}\n")
    fh.write(f"- Approved entries missing signoff metadata: {len(approved_missing_signoff)}\n")
    if signoff_decision_counts:
        ordered = ", ".join(
            f"{decision}={count}" for decision, count in sorted(signoff_decision_counts.items())
        )
        fh.write(f"- Signoff decision distribution: {ordered}\n")
    else:
        fh.write("- Signoff decision distribution: none\n")
    if latest_signoff_record:
        fh.write(
            f"- Latest signoff record: `{latest_signoff_record.get('proposal_id', 'n/a')}` "
            f"({latest_signoff_record.get('decision', 'n/a')}, {latest_signoff_record.get('approved_at_utc', 'n/a')})\n\n"
        )
    else:
        fh.write("- Latest signoff record: none\n\n")

    fh.write("| Proposal | Decision | Owner | Reviewers | approved_at_utc |\n")
    fh.write("|---|---|---|---:|---|\n")
    if signoff_records:
        for record in sorted(signoff_records, key=lambda item: (item.get("proposal_id", ""), item.get("approved_at_utc", ""))):
            fh.write(
                f"| `{record.get('proposal_id', 'n/a')}` | {record.get('decision', 'n/a')} | "
                f"{record.get('owner', 'n/a')} | {record.get('reviewer_count', 0)} | {record.get('approved_at_utc', 'n/a')} |\n"
            )
    else:
        fh.write("| _none_ | n/a | n/a | 0 | n/a |\n")
    fh.write("\n")

    latency_summary = latency_telemetry["summary"]
    latest_latency = latency_telemetry.get("latest_ok_entry")

    fh.write("## MONPOL Proposal-to-Merge Latency\n\n")
    fh.write(f"- Changelog entries evaluated: {latency_summary['entries_evaluated']}\n")
    fh.write(f"- Proposal artifacts discovered: {latency_summary['proposal_artifacts_discovered']}\n")
    fh.write(f"- Proposal artifacts skipped (unparseable time): {latency_summary['proposal_artifacts_skipped']}\n")
    fh.write(f"- Latency samples (days): {latency_summary['latency_samples']}\n")
    fh.write(
        f"- Median latency (days): {latency_summary['median_latency_days'] if latency_summary['median_latency_days'] is not None else 'n/a'}\n"
    )
    fh.write(
        f"- P90 latency (days): {latency_summary['p90_latency_days'] if latency_summary['p90_latency_days'] is not None else 'n/a'}\n"
    )
    fh.write(
        f"- Max latency (days): {latency_summary['max_latency_days'] if latency_summary['max_latency_days'] is not None else 'n/a'}\n"
    )
    fh.write(f"- Entries missing proposal artifacts: {latency_summary['missing_proposal_artifact_count']}\n")
    fh.write(f"- Entries with invalid merge date: {latency_summary['invalid_merge_date_count']}\n")
    fh.write(f"- Entries with chronology errors: {latency_summary['chronology_error_count']}\n")
    if latest_latency:
        fh.write(
            f"- Latest merged entry with latency: `{latest_latency.get('proposal_id', 'n/a')}` "
            f"({latest_latency.get('latency_days', 'n/a')} days)\n\n"
        )
    else:
        fh.write("- Latest merged entry with latency: none\n\n")

    fh.write("| Proposal | Decision | Merge date | First proposal generated_at_utc | Latency (days) | Status | Proposal file | History files |\n")
    fh.write("|---|---|---|---|---:|---|---|---:|\n")
    if latency_telemetry["rows"]:
        for row in latency_telemetry["rows"]:
            latency_cell = row["latency_days"] if row["latency_days"] is not None else "n/a"
            fh.write(
                f"| `{row.get('proposal_id', 'n/a')}` | {row.get('decision', 'n/a')} | {row.get('merge_date', 'n/a')} | "
                f"{row.get('first_proposal_generated_at_utc', 'n/a')} | {latency_cell} | {row.get('status', 'n/a')} | "
                f"`{row.get('proposal_file', 'n/a')}` | {row.get('proposal_history_files', 0)} |\n"
            )
    else:
        fh.write("| _none_ | n/a | n/a | n/a | n/a | n/a | n/a | 0 |\n")
    fh.write("\n")

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
        "strict_unknown_runs": strict_unknown_runs,
        "runs_with_monitor_drift": monitor_drift_runs,
        "runs_with_monitor_failures": monitor_failure_runs,
    },
    "signoff_telemetry": {
        "changelog_entries": changelog_entries,
        "signoff_records": signoff_records,
        "signoff_decision_counts": signoff_decision_counts,
        "approved_entries": [entry["proposal_id"] for entry in approved_entries],
        "approved_with_signoff": approved_with_signoff,
        "approved_missing_signoff": approved_missing_signoff,
        "latest_signoff_record": latest_signoff_record,
    },
    "latency_telemetry": latency_telemetry,
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
