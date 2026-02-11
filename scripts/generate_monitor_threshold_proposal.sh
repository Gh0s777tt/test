#!/usr/bin/env bash
# Generate governance-ready MONPOL threshold proposal draft from rolling evidence.
# Usage:
#   ./scripts/generate_monitor_threshold_proposal.sh
#   ./scripts/generate_monitor_threshold_proposal.sh --proposal-id MONPOL-002 --bench timer_queue_benchmark

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
CHANGELOG_PATH="$ROOT/governance/performance/MONITOR_THRESHOLD_CHANGELOG.md"
SIGNOFF_PATH="$ROOT/governance/performance/MONPOL_SIGNOFFS.json"
ESCALATION_JSON=""
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
  --signoff <path>              Signoff metadata JSON path
  --escalation-json <path>      Drift escalation JSON snapshot (defaults to newest if available)
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
    --signoff)
      SIGNOFF_PATH="${2:-}"
      shift 2
      ;;
    --escalation-json)
      ESCALATION_JSON="${2:-}"
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

if [[ ! -f "$SIGNOFF_PATH" ]]; then
  echo "Error: signoff metadata not found: $SIGNOFF_PATH" >&2
  exit 1
fi

if [[ -z "$ESCALATION_JSON" ]]; then
  shopt -s nullglob
  escalation_candidates=("$ANALYSIS_DIR"/monitor_drift_escalation_[0-9]*.json)
  shopt -u nullglob
  if (( ${#escalation_candidates[@]} > 0 )); then
    readarray -t sorted_escalation < <(printf '%s\n' "${escalation_candidates[@]}" | sort)
    ESCALATION_JSON="${sorted_escalation[${#sorted_escalation[@]}-1]}"
  fi
fi

if [[ -n "$ESCALATION_JSON" ]] && [[ ! -f "$ESCALATION_JSON" ]]; then
  echo "Error: escalation json not found: $ESCALATION_JSON" >&2
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

python3 - "$ROOT" "$ANALYSIS_DIR" "$CHANGELOG_PATH" "$SIGNOFF_PATH" "$ESCALATION_JSON" "$RECOMMENDATION_JSON" "$DASHBOARD_JSON" "$PROPOSAL_ID" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" "$FILTER_CSV" <<'PY'
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
analysis_dir = Path(sys.argv[2])
changelog_path = Path(sys.argv[3])
signoff_path = Path(sys.argv[4])
escalation_path_raw = sys.argv[5].strip()
recommendation_path = Path(sys.argv[6])
dashboard_path = Path(sys.argv[7])
proposal_id_arg = sys.argv[8].strip()
output_path_arg = sys.argv[9].strip()
output_json_path_arg = sys.argv[10].strip()
filter_csv = sys.argv[11]
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


proposal_file_re = re.compile(r"^monitor_threshold_proposal_(MONPOL-\d{3})_(\d{8}T\d{6}Z)\.json$")


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


def parse_proposal_generated_at(path: Path):
    payload_dt = None
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
        payload_dt = parse_iso_utc(str(payload.get("generated_at_utc", "")).strip())
    except (json.JSONDecodeError, OSError):
        payload_dt = None

    if payload_dt is not None:
        return payload_dt

    match = proposal_file_re.match(path.name)
    if match:
        return parse_compact_utc(match.group(2))
    return None


def collect_proposal_history(directory: Path, proposal_id: str):
    history = []
    for path in sorted(directory.glob(f"monitor_threshold_proposal_{proposal_id}_*.json")):
        generated_dt = parse_proposal_generated_at(path)
        if generated_dt is None:
            continue
        history.append(
            {
                "file": path.name,
                "generated_dt": generated_dt,
                "generated_at_utc": generated_dt.isoformat().replace("+00:00", "Z"),
            }
        )
    history.sort(key=lambda item: item["generated_dt"])
    return history


def present(value):
    return value if value is not None else "n/a"


changelog_text = changelog_path.read_text(encoding="utf-8")
proposal_id = proposal_id_arg or parse_next_monpol_id(changelog_text)
if not re.fullmatch(r"MONPOL-\d{3}", proposal_id):
    raise SystemExit(f"Invalid proposal ID format: {proposal_id}")

recommendation_payload = json.loads(recommendation_path.read_text(encoding="utf-8"))
dashboard_payload = json.loads(dashboard_path.read_text(encoding="utf-8"))
signoff_payload = json.loads(signoff_path.read_text(encoding="utf-8"))
signoff_records = signoff_payload.get("records", [])
if not isinstance(signoff_records, list):
    signoff_records = []

normalized_signoff = []
for record in signoff_records:
    if not isinstance(record, dict):
        continue
    normalized_signoff.append(
        {
            "proposal_id": str(record.get("proposal_id", "")).strip(),
            "decision": normalize_decision(str(record.get("decision", ""))),
            "owner": str(record.get("owner", "")).strip() or "n/a",
            "approved_at_utc": str(record.get("approved_at_utc", "")).strip() or "n/a",
            "reviewers": record.get("reviewers", []) if isinstance(record.get("reviewers", []), list) else [],
        }
    )

signoff_by_id = {record["proposal_id"]: record for record in normalized_signoff if record["proposal_id"]}
proposal_signoff = signoff_by_id.get(proposal_id)
decision_counts = {}
for record in normalized_signoff:
    decision = record["decision"]
    decision_counts[decision] = decision_counts.get(decision, 0) + 1

escalation_path = Path(escalation_path_raw) if escalation_path_raw else None
escalation_payload = {}
if escalation_path and escalation_path.is_file():
    escalation_payload = json.loads(escalation_path.read_text(encoding="utf-8"))

escalation_overall_level = str(escalation_payload.get("overall_level", "n/a"))
escalation_level_counts = escalation_payload.get("level_counts", {})
if not isinstance(escalation_level_counts, dict):
    escalation_level_counts = {}
escalation_rows = escalation_payload.get("rows", [])
if not isinstance(escalation_rows, list):
    escalation_rows = []
escalation_fail_triggered = bool(escalation_payload.get("fail_triggered", False))
escalation_source = str(escalation_payload.get("dashboard_source", "n/a"))
escalation_owners_source = str(escalation_payload.get("owners_config_source", "n/a"))
escalation_overall_owner = escalation_payload.get("overall_owner_profile", {})
if not isinstance(escalation_overall_owner, dict):
    escalation_overall_owner = {}
escalation_next_drill_due = str(escalation_payload.get("next_drill_due_utc", "n/a"))

recommendations = recommendation_payload.get("recommendations", [])
if bench_filter:
    recommendations = [item for item in recommendations if item.get("bench") in bench_filter]

if not recommendations:
    raise SystemExit("No recommendations available for proposal generation after filtering")

bench_health = dashboard_payload.get("bench_health", {})
latest_recommendation = dashboard_payload.get("latest_recommendation") or {}
recent_runs = dashboard_payload.get("recent_runs", [])
latest_run = recent_runs[-1] if recent_runs else {}
dashboard_latency = dashboard_payload.get("latency_telemetry", {})
dashboard_latency_summary = dashboard_latency.get("summary", {})
dashboard_latency_rows = dashboard_latency.get("rows", [])
dashboard_latest_latency = dashboard_latency.get("latest_ok_entry")

proposal_timestamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
now_utc = datetime.now(timezone.utc)
generated_at_iso = now_utc.replace(microsecond=0).isoformat().replace("+00:00", "Z")

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
latest_handoff_json = sorted(analysis_dir.glob("monitor_drift_release_handoff_[0-9]*.json"))
latest_drill_json = sorted(analysis_dir.glob("monitor_drift_release_readiness_drill_[0-9]*.json"))
latest_breach_route_json = sorted(analysis_dir.glob("monitor_drift_breach_route_[0-9]*.json"))
latest_promotion_readiness_json = sorted(analysis_dir.glob("governance_gate_promotion_readiness_[0-9]*.json"))
latest_pilot_runbook_json = sorted(analysis_dir.glob("enforced_pilot_runbook_[0-9]*.json"))
latest_burn_in_slo_json = sorted(analysis_dir.glob("enforced_pilot_burn_in_slo_[0-9]*.json"))
latest_rollback_postmortem_json = sorted(analysis_dir.glob("enforced_pilot_rollback_postmortem_[0-9]*.json"))
latest_handoff_signoff_packet_json = sorted(analysis_dir.glob("enforced_pilot_handoff_signoff_packet_[0-9]*.json"))
if escalation_path and escalation_path.is_file():
    evidence_files.append(rel(escalation_path))
if latest_handoff_json:
    evidence_files.append(rel(latest_handoff_json[-1]))
if latest_drill_json:
    evidence_files.append(rel(latest_drill_json[-1]))
if latest_breach_route_json:
    evidence_files.append(rel(latest_breach_route_json[-1]))
if latest_promotion_readiness_json:
    evidence_files.append(rel(latest_promotion_readiness_json[-1]))
if latest_pilot_runbook_json:
    evidence_files.append(rel(latest_pilot_runbook_json[-1]))
if latest_burn_in_slo_json:
    evidence_files.append(rel(latest_burn_in_slo_json[-1]))
if latest_rollback_postmortem_json:
    evidence_files.append(rel(latest_rollback_postmortem_json[-1]))
if latest_handoff_signoff_packet_json:
    evidence_files.append(rel(latest_handoff_signoff_packet_json[-1]))
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

current_history = collect_proposal_history(analysis_dir, proposal_id)
current_first = current_history[0] if current_history else None
current_last = current_history[-1] if current_history else None
current_age_days = (now_utc.date() - current_first["generated_dt"].date()).days if current_first else None

with output_path.open("w", encoding="utf-8") as fh:
    fh.write(f"# Monitor Threshold Change Proposal Draft: `{proposal_id}`\n\n")
    fh.write(f"**Status**: Draft  \n")
    fh.write(f"**Generated at (UTC)**: {generated_at_iso}  \n")
    fh.write(f"**Proposal ID**: `{proposal_id}`  \n")
    fh.write(f"**Changelog target**: `{rel(changelog_path)}`\n\n")

    fh.write("## Inputs\n\n")
    fh.write(f"- Recommendation snapshot: `{rel(recommendation_path)}`\n")
    fh.write(f"- Dashboard snapshot: `{rel(dashboard_path)}`\n")
    if escalation_path and escalation_path.is_file():
        fh.write(f"- Escalation snapshot: `{rel(escalation_path)}`\n")
    else:
        fh.write("- Escalation snapshot: `n/a`\n")
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
    fh.write("- [ ] If decision is approved, add/update signoff metadata in `governance/performance/MONPOL_SIGNOFFS.json`.\n")
    fh.write("- [ ] If escalation level is `escalated`/`critical`, include mitigation and owner plan.\n")
    fh.write(f"- [ ] Include `{proposal_id}` in PR title or body.\n")
    fh.write("- [ ] Link evidence bundle below in PR description.\n\n")

    fh.write("## Signoff Telemetry\n\n")
    if decision_counts:
        ordered = ", ".join(
            f"{decision}={count}" for decision, count in sorted(decision_counts.items())
        )
        fh.write(f"- Signoff decision distribution: {ordered}\n")
    else:
        fh.write("- Signoff decision distribution: none\n")

    if proposal_signoff:
        fh.write(f"- Existing signoff record for `{proposal_id}`: yes\n")
        fh.write(f"- Decision: `{proposal_signoff.get('decision', 'n/a')}`\n")
        fh.write(f"- Owner: `{proposal_signoff.get('owner', 'n/a')}`\n")
        fh.write(f"- Reviewers: {len(proposal_signoff.get('reviewers', []))}\n")
        fh.write(f"- approved_at_utc: `{proposal_signoff.get('approved_at_utc', 'n/a')}`\n\n")
    else:
        fh.write(f"- Existing signoff record for `{proposal_id}`: no\n")
        fh.write("- Add signoff metadata when decision becomes approved.\n\n")

    fh.write("| Proposal | Decision | Owner | Reviewers | approved_at_utc |\n")
    fh.write("|---|---|---|---:|---|\n")
    if normalized_signoff:
        for record in sorted(normalized_signoff, key=lambda item: (item.get("proposal_id", ""), item.get("approved_at_utc", ""))):
            fh.write(
                f"| `{record.get('proposal_id', 'n/a')}` | {record.get('decision', 'n/a')} | "
                f"{record.get('owner', 'n/a')} | {len(record.get('reviewers', []))} | {record.get('approved_at_utc', 'n/a')} |\n"
            )
    else:
        fh.write("| _none_ | n/a | n/a | 0 | n/a |\n")
    fh.write("\n")

    fh.write("## Proposal-to-Merge Latency Telemetry\n\n")
    fh.write(
        f"- Historical latency samples (days): {present(dashboard_latency_summary.get('latency_samples'))}\n"
    )
    fh.write(
        f"- Historical median latency (days): {present(dashboard_latency_summary.get('median_latency_days'))}\n"
    )
    fh.write(
        f"- Historical P90 latency (days): {present(dashboard_latency_summary.get('p90_latency_days'))}\n"
    )
    fh.write(
        f"- Historical max latency (days): {present(dashboard_latency_summary.get('max_latency_days'))}\n"
    )
    fh.write(
        f"- Changelog entries missing proposal artifacts: {present(dashboard_latency_summary.get('missing_proposal_artifact_count'))}\n"
    )
    fh.write(
        f"- Changelog chronology errors: {present(dashboard_latency_summary.get('chronology_error_count'))}\n"
    )
    if dashboard_latest_latency:
        fh.write(
            f"- Latest merged proposal latency: `{dashboard_latest_latency.get('proposal_id', 'n/a')}` "
            f"({dashboard_latest_latency.get('latency_days', 'n/a')} days)\n"
        )
    else:
        fh.write("- Latest merged proposal latency: none\n")
    fh.write(f"- Current proposal drafts discovered: {len(current_history)}\n")
    if current_first:
        fh.write(f"- Current proposal first seen: `{current_first.get('generated_at_utc', 'n/a')}`\n")
        fh.write(f"- Current proposal latest seen: `{current_last.get('generated_at_utc', 'n/a')}`\n")
        fh.write(f"- Current proposal age (days): {current_age_days}\n\n")
    else:
        fh.write("- Current proposal first seen: n/a\n")
        fh.write("- Current proposal latest seen: n/a\n")
        fh.write("- Current proposal age (days): n/a\n\n")

    fh.write("| Proposal | Decision | Merge date | First proposal generated_at_utc | Latency (days) | Status |\n")
    fh.write("|---|---|---|---|---:|---|\n")
    if dashboard_latency_rows:
        for row in dashboard_latency_rows:
            latency_cell = row.get("latency_days", "n/a")
            if latency_cell is None:
                latency_cell = "n/a"
            fh.write(
                f"| `{row.get('proposal_id', 'n/a')}` | {row.get('decision', 'n/a')} | {row.get('merge_date', 'n/a')} | "
                f"{row.get('first_proposal_generated_at_utc', 'n/a')} | {latency_cell} | {row.get('status', 'n/a')} |\n"
            )
    else:
        fh.write("| _none_ | n/a | n/a | n/a | n/a | n/a |\n")
    fh.write("\n")

    fh.write("## Drift Escalation Policy Snapshot\n\n")
    if escalation_payload:
        fh.write(f"- Escalation artifact: `{rel(escalation_path)}`\n")
        fh.write(f"- Overall escalation level: **{escalation_overall_level}**\n")
        fh.write(f"- Escalation owners source: `{escalation_owners_source}`\n")
        fh.write(
            f"- Overall owner/SLA: {escalation_overall_owner.get('owner', 'n/a')} "
            f"(backup: {escalation_overall_owner.get('backup_owner', 'n/a')}, "
            f"SLA={escalation_overall_owner.get('response_sla_hours', 'n/a')}h)\n"
        )
        fh.write(f"- Next drill due (UTC): {escalation_next_drill_due}\n")
        fh.write(
            f"- Release handoff required: "
            f"{'yes' if escalation_overall_owner.get('release_handoff_required') else 'no'}\n"
        )
        fh.write(
            f"- Level counts: normal={present(escalation_level_counts.get('normal'))}, "
            f"watch={present(escalation_level_counts.get('watch'))}, "
            f"escalated={present(escalation_level_counts.get('escalated'))}, "
            f"critical={present(escalation_level_counts.get('critical'))}\n"
        )
        fh.write(f"- Escalation fail trigger active: {'yes' if escalation_fail_triggered else 'no'}\n")
        fh.write(f"- Escalation dashboard source: `{escalation_source}`\n\n")
        fh.write("| Benchmark | Level | Latest status | Consecutive drift | Drift rate (%) | Failure rate (%) | Owner | SLA (h) | Drill (d) | Handoff | Required action |\n")
        fh.write("|---|---|---|---:|---:|---:|---|---:|---:|---|---|\n")
        if escalation_rows:
            for row in sorted(escalation_rows, key=lambda item: item.get("bench", "")):
                fh.write(
                    f"| `{row.get('bench', 'n/a')}` | {row.get('level', 'n/a')} | {row.get('latest_status', 'n/a')} | "
                    f"{present(row.get('consecutive_drift'))} | {present(row.get('drift_rate_pct'))} | "
                    f"{present(row.get('failure_rate_pct'))} | {row.get('owner', 'n/a')} | "
                    f"{present(row.get('response_sla_hours'))} | {present(row.get('drill_cadence_days'))} | "
                    f"{'yes' if row.get('release_handoff_required') else 'no'} | {row.get('required_action', 'n/a')} |\n"
                )
        else:
            fh.write("| _none_ | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a |\n")
        fh.write("\n")
    else:
        fh.write("- Escalation artifact: not available\n")
        fh.write("- Overall escalation level: n/a\n\n")

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
        "signoff_json": rel(signoff_path),
        "escalation_json": rel(escalation_path) if escalation_path and escalation_path.is_file() else "n/a",
    },
    "bench_filter": sorted(bench_filter),
    "rows": rows,
    "proposal_changes": proposal_changes,
    "suggested_monitor_flags": monitor_flag_suggestions,
    "evidence_bundle": dedup_evidence,
    "signoff_telemetry": {
        "decision_counts": decision_counts,
        "proposal_signoff": proposal_signoff,
        "record_count": len(normalized_signoff),
    },
    "latency_telemetry": {
        "historical_summary": dashboard_latency_summary,
        "historical_latest_ok_entry": dashboard_latest_latency,
        "historical_rows": dashboard_latency_rows,
        "current_proposal": {
            "proposal_id": proposal_id,
            "draft_count": len(current_history),
            "first_seen_utc": current_first.get("generated_at_utc", "n/a") if current_first else "n/a",
            "latest_seen_utc": current_last.get("generated_at_utc", "n/a") if current_last else "n/a",
            "age_days": current_age_days,
            "draft_files": [item["file"] for item in current_history],
        },
    },
    "escalation_telemetry": {
        "source_json": rel(escalation_path) if escalation_path and escalation_path.is_file() else "n/a",
        "overall_level": escalation_overall_level,
        "owners_config_source": escalation_owners_source,
        "overall_owner_profile": escalation_overall_owner,
        "next_drill_due_utc": escalation_next_drill_due,
        "level_counts": escalation_level_counts,
        "rows": escalation_rows,
        "fail_triggered": escalation_fail_triggered,
    },
}
output_json_path.write_text(json.dumps(proposal_json, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Monitor threshold proposal markdown: {output_path}")
print(f"Monitor threshold proposal JSON: {output_json_path}")
print(f"Proposal ID: {proposal_id}")
print(f"Benchmarks included: {len(rows)}")
PY
