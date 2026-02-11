#!/usr/bin/env bash
# Generate a transition pack summarizing governance toolchain readiness.
# Usage:
#   ./scripts/generate_governance_transition_pack.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
WORKFLOW_PATH="$ROOT/.github/workflows/ci.yml"
CHANGELOG_PATH="$ROOT/governance/performance/MONITOR_THRESHOLD_CHANGELOG.md"
PROFILE_DOC_PATH="$ROOT/docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md"
TEMPLATE_PATH="$ROOT/governance/performance/MONITOR_THRESHOLD_PROPOSAL_TEMPLATE.md"
SIGNOFF_PATH="$ROOT/governance/performance/MONPOL_SIGNOFFS.json"
ESCALATION_POLICY_PATH="$ROOT/governance/performance/MONITOR_DRIFT_ESCALATION_POLICY.md"
ESCALATION_OWNERS_PATH="$ROOT/governance/performance/MONITOR_DRIFT_ESCALATION_OWNERS.json"

OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/generate_governance_transition_pack.sh [options]

Options:
  --analysis-dir <path>   Benchmark analysis directory
  --workflow <path>       CI workflow file path
  --changelog <path>      Monitor threshold changelog path
  --profile-doc <path>    Reproducibility profile doc path
  --template <path>       Proposal template path
  --signoff <path>        MONPOL signoff metadata JSON path
  --escalation-policy <path>
                           Monitor drift escalation policy doc path
  --escalation-owners <path>
                           Monitor drift escalation owner/SLA registry path
  --output <path>         Output markdown path
  --output-json <path>    Output JSON path
  -h, --help              Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --workflow)
      WORKFLOW_PATH="${2:-}"
      shift 2
      ;;
    --changelog)
      CHANGELOG_PATH="${2:-}"
      shift 2
      ;;
    --profile-doc)
      PROFILE_DOC_PATH="${2:-}"
      shift 2
      ;;
    --template)
      TEMPLATE_PATH="${2:-}"
      shift 2
      ;;
    --signoff)
      SIGNOFF_PATH="${2:-}"
      shift 2
      ;;
    --escalation-policy)
      ESCALATION_POLICY_PATH="${2:-}"
      shift 2
      ;;
    --escalation-owners)
      ESCALATION_OWNERS_PATH="${2:-}"
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

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/governance_transition_pack_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - "$ROOT" "$ANALYSIS_DIR" "$WORKFLOW_PATH" "$CHANGELOG_PATH" "$PROFILE_DOC_PATH" "$TEMPLATE_PATH" "$SIGNOFF_PATH" "$ESCALATION_POLICY_PATH" "$ESCALATION_OWNERS_PATH" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" <<'PY'
import json
import re
import sys
from datetime import datetime, timezone
from statistics import median
from pathlib import Path

root = Path(sys.argv[1])
analysis_dir = Path(sys.argv[2])
workflow_path = Path(sys.argv[3])
changelog_path = Path(sys.argv[4])
profile_doc_path = Path(sys.argv[5])
template_path = Path(sys.argv[6])
signoff_path = Path(sys.argv[7])
escalation_policy_path = Path(sys.argv[8])
escalation_owners_path = Path(sys.argv[9])
output_path = Path(sys.argv[10])
output_json_path = Path(sys.argv[11])


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(root.resolve()))
    except ValueError:
        return str(path)


def latest(glob: str):
    items = sorted(analysis_dir.glob(glob))
    return items[-1] if items else None


def yes_no(value: bool) -> str:
    return "yes" if value else "no"


def parse_ci_policy(workflow_text: str):
    strict_bench = ""
    strict_threshold = ""
    monitor_budget = ""
    monitor_case_timeout = ""
    monitor_benches = []

    for raw_line in workflow_text.splitlines():
        line = raw_line.strip()
        strict_bench_match = re.search(r"--strict-bench\s+([^\s]+)", line)
        if strict_bench_match:
            strict_bench = strict_bench_match.group(1)

        strict_threshold_match = re.search(r"--strict-threshold-pct\s+([^\s]+)", line)
        if strict_threshold_match:
            strict_threshold = strict_threshold_match.group(1)

        budget_match = re.search(r"--monitor-budget-seconds\s+([^\s]+)", line)
        if budget_match:
            monitor_budget = budget_match.group(1)

        timeout_match = re.search(r"--monitor-case-timeout-seconds\s+([^\s]+)", line)
        if timeout_match:
            monitor_case_timeout = timeout_match.group(1)

        monitor_match = re.search(r"--monitor-bench\s+([^\s]+)", line)
        if monitor_match:
            token = monitor_match.group(1)
            if ":" in token:
                bench, threshold = token.split(":", 1)
            else:
                bench, threshold = token, ""
            monitor_benches.append(
                {
                    "bench": bench,
                    "threshold_pct": threshold,
                }
            )

    # Preserve order and de-duplicate by bench (last entry wins).
    dedup = {}
    order = []
    for item in monitor_benches:
        bench = item["bench"]
        if bench not in dedup:
            order.append(bench)
        dedup[bench] = item
    monitor_benches = [dedup[bench] for bench in order]

    return {
        "strict_bench": strict_bench or "n/a",
        "strict_threshold_pct": strict_threshold or "n/a",
        "monitor_benches": monitor_benches,
        "monitor_budget_seconds": monitor_budget or "n/a",
        "monitor_case_timeout_seconds": monitor_case_timeout or "n/a",
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


def parse_changelog_entries(changelog_text: str):
    header_re = re.compile(r"^###\s+(MONPOL-\d{3})\s+\(([^)]+)\)\s*$", re.MULTILINE)
    headers = list(header_re.finditer(changelog_text))
    entries = []
    for idx, match in enumerate(headers):
        start = match.start()
        end = headers[idx + 1].start() if idx + 1 < len(headers) else len(changelog_text)
        block = changelog_text[start:end]
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


def parse_signoff_records(path: Path):
    if not path.exists():
        return []
    payload = json.loads(path.read_text(encoding="utf-8"))
    records = payload.get("records", [])
    if not isinstance(records, list):
        records = []
    normalized = []
    for record in records:
        if not isinstance(record, dict):
            continue
        proposal_id = str(record.get("proposal_id", "")).strip()
        decision = normalize_decision(str(record.get("decision", "")))
        owner = str(record.get("owner", "")).strip() or "n/a"
        approved_at = str(record.get("approved_at_utc", "")).strip() or "n/a"
        reviewers = record.get("reviewers", [])
        normalized.append(
            {
                "proposal_id": proposal_id,
                "decision": decision,
                "owner": owner,
                "approved_at_utc": approved_at,
                "reviewer_count": len(reviewers) if isinstance(reviewers, list) else 0,
            }
        )
    return normalized


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

    match = proposal_file_re.match(path.name)
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
        match = proposal_file_re.match(path.name)
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
        history = by_id.get(proposal_id, [])
        decision = entry.get("decision", "unknown")

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


def parse_escalation_payload(path: Path):
    if not path:
        return {}
    if not path.exists():
        return {}
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError):
        return {}

    level_counts = payload.get("level_counts", {})
    if not isinstance(level_counts, dict):
        level_counts = {}

    rows = payload.get("rows", [])
    if not isinstance(rows, list):
        rows = []

    return {
        "source": rel(path),
        "overall_level": str(payload.get("overall_level", "n/a")),
        "level_counts": level_counts,
        "escalated_benches": payload.get("escalated_benches", []),
        "critical_benches": payload.get("critical_benches", []),
        "fail_triggered": bool(payload.get("fail_triggered", False)),
        "fail_on_level": str(payload.get("fail_on_level", "none")),
        "owners_config_source": str(payload.get("owners_config_source", "n/a")),
        "overall_owner_profile": payload.get("overall_owner_profile", {}),
        "next_drill_due_utc": str(payload.get("next_drill_due_utc", "n/a")),
        "rows": rows,
        "generated_at_utc": str(payload.get("generated_at_utc", "n/a")),
        "window": payload.get("window", {}),
    }


def parse_handoff_payload(path: Path):
    if not path:
        return {}
    if not path.exists():
        return {}
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError):
        return {}

    items = payload.get("items", [])
    if not isinstance(items, list):
        items = []
    required_failures = payload.get("required_failures", [])
    if not isinstance(required_failures, list):
        required_failures = []

    return {
        "source": rel(path),
        "overall_status": str(payload.get("overall_status", "n/a")),
        "overall_level": str(payload.get("overall_level", "n/a")),
        "overall_owner_profile": payload.get("overall_owner_profile", {}),
        "next_drill_due_utc": str(payload.get("next_drill_due_utc", "n/a")),
        "release_handoff_required": bool(payload.get("release_handoff_required", False)),
        "strict_mode": bool(payload.get("strict_mode", False)),
        "items": items,
        "required_failures": required_failures,
        "generated_at_utc": str(payload.get("generated_at_utc", "n/a")),
    }


latest_summary = latest("ci_benchmark_gate_summary_*.md")
latest_recommendation_md = latest("monitor_policy_recommendations_*.md")
latest_recommendation_json = latest("monitor_policy_recommendations_*.json")
latest_dashboard_md = latest("monitor_policy_dashboard_*.md")
latest_dashboard_json = latest("monitor_policy_dashboard_*.json")
latest_proposal_md = latest("monitor_threshold_proposal_MONPOL-*_*.md")
latest_proposal_json = latest("monitor_threshold_proposal_MONPOL-*_*.json")
latest_scaffold_md = latest("monpol_changelog_scaffold_MONPOL-*_*.md")
latest_scaffold_json = latest("monpol_changelog_scaffold_MONPOL-*_*.json")
latest_signoff_validation_md = latest("monpol_signoff_validation_*.md")
latest_signoff_validation_json = latest("monpol_signoff_validation_*.json")
latest_escalation_md = latest("monitor_drift_escalation_*.md")
latest_escalation_json = latest("monitor_drift_escalation_*.json")
latest_handoff_md = latest("monitor_drift_release_handoff_*.md")
latest_handoff_json = latest("monitor_drift_release_handoff_*.json")

workflow_text = workflow_path.read_text(encoding="utf-8") if workflow_path.exists() else ""
ci_policy = parse_ci_policy(workflow_text) if workflow_text else {
    "strict_bench": "n/a",
    "strict_threshold_pct": "n/a",
    "monitor_benches": [],
    "monitor_budget_seconds": "n/a",
    "monitor_case_timeout_seconds": "n/a",
}

changelog_text = changelog_path.read_text(encoding="utf-8") if changelog_path.exists() else ""
monpol_matches = list(re.finditer(r"###\s+(MONPOL-\d{3})\s+\(([^)]+)\)", changelog_text))
latest_monpol_id = monpol_matches[-1].group(1) if monpol_matches else "n/a"
latest_monpol_date = monpol_matches[-1].group(2) if monpol_matches else "n/a"
monpol_entries = parse_changelog_entries(changelog_text)
signoff_records = parse_signoff_records(signoff_path)
signoff_by_id = {record["proposal_id"]: record for record in signoff_records if record["proposal_id"]}
latency_telemetry = collect_latency_telemetry(monpol_entries, analysis_dir)
escalation_telemetry = parse_escalation_payload(latest_escalation_json) if latest_escalation_json else {}
handoff_telemetry = parse_handoff_payload(latest_handoff_json) if latest_handoff_json else {}
approved_entries = [entry["proposal_id"] for entry in monpol_entries if entry["decision"] == "approved"]
approved_with_signoff = [proposal_id for proposal_id in approved_entries if proposal_id in signoff_by_id]
approved_missing_signoff = [proposal_id for proposal_id in approved_entries if proposal_id not in signoff_by_id]
signoff_decision_counts = {}
for record in signoff_records:
    decision = record["decision"]
    signoff_decision_counts[decision] = signoff_decision_counts.get(decision, 0) + 1

latest_signoff_record = None
for record in signoff_records:
    current = record.get("approved_at_utc", "n/a")
    if current == "n/a":
        continue
    if latest_signoff_record is None or current > latest_signoff_record.get("approved_at_utc", "n/a"):
        latest_signoff_record = record

scripts_required = [
    root / "scripts/run_benchmark_ci_gate.sh",
    root / "scripts/recommend_monitor_policy.sh",
    root / "scripts/build_monitor_policy_dashboard.sh",
    root / "scripts/evaluate_monitor_drift_escalation.sh",
    root / "scripts/generate_monitor_threshold_proposal.sh",
    root / "scripts/scaffold_monpol_changelog_entry.sh",
    root / "scripts/generate_monitor_drift_release_handoff.sh",
    root / "scripts/validate_monpol_signoff_metadata.sh",
    root / "scripts/check_monitor_threshold_governance.sh",
]
script_status = [
    {
        "path": rel(path),
        "exists": path.exists(),
        "executable": path.exists() and bool(path.stat().st_mode & 0o111),
    }
    for path in scripts_required
]

docs_status = [
    {"path": rel(profile_doc_path), "exists": profile_doc_path.exists()},
    {"path": rel(changelog_path), "exists": changelog_path.exists()},
    {"path": rel(template_path), "exists": template_path.exists()},
    {"path": rel(escalation_policy_path), "exists": escalation_policy_path.exists()},
    {"path": rel(escalation_owners_path), "exists": escalation_owners_path.exists()},
    {
        "path": rel(signoff_path),
        "exists": signoff_path.exists(),
    },
]

artifact_status = [
    {"kind": "ci_summary", "path": rel(latest_summary) if latest_summary else "n/a", "exists": bool(latest_summary)},
    {
        "kind": "recommendation_md",
        "path": rel(latest_recommendation_md) if latest_recommendation_md else "n/a",
        "exists": bool(latest_recommendation_md),
    },
    {
        "kind": "recommendation_json",
        "path": rel(latest_recommendation_json) if latest_recommendation_json else "n/a",
        "exists": bool(latest_recommendation_json),
    },
    {"kind": "dashboard_md", "path": rel(latest_dashboard_md) if latest_dashboard_md else "n/a", "exists": bool(latest_dashboard_md)},
    {
        "kind": "dashboard_json",
        "path": rel(latest_dashboard_json) if latest_dashboard_json else "n/a",
        "exists": bool(latest_dashboard_json),
    },
    {"kind": "proposal_md", "path": rel(latest_proposal_md) if latest_proposal_md else "n/a", "exists": bool(latest_proposal_md)},
    {
        "kind": "proposal_json",
        "path": rel(latest_proposal_json) if latest_proposal_json else "n/a",
        "exists": bool(latest_proposal_json),
    },
    {"kind": "scaffold_md", "path": rel(latest_scaffold_md) if latest_scaffold_md else "n/a", "exists": bool(latest_scaffold_md)},
    {
        "kind": "scaffold_json",
        "path": rel(latest_scaffold_json) if latest_scaffold_json else "n/a",
        "exists": bool(latest_scaffold_json),
    },
    {
        "kind": "signoff_validation_md",
        "path": rel(latest_signoff_validation_md) if latest_signoff_validation_md else "n/a",
        "exists": bool(latest_signoff_validation_md),
    },
    {
        "kind": "signoff_validation_json",
        "path": rel(latest_signoff_validation_json) if latest_signoff_validation_json else "n/a",
        "exists": bool(latest_signoff_validation_json),
    },
    {
        "kind": "escalation_md",
        "path": rel(latest_escalation_md) if latest_escalation_md else "n/a",
        "exists": bool(latest_escalation_md),
    },
    {
        "kind": "escalation_json",
        "path": rel(latest_escalation_json) if latest_escalation_json else "n/a",
        "exists": bool(latest_escalation_json),
    },
    {
        "kind": "handoff_md",
        "path": rel(latest_handoff_md) if latest_handoff_md else "n/a",
        "exists": bool(latest_handoff_md),
    },
    {
        "kind": "handoff_json",
        "path": rel(latest_handoff_json) if latest_handoff_json else "n/a",
        "exists": bool(latest_handoff_json),
    },
]

readiness_checks = {
    "scripts_ready": all(item["exists"] and item["executable"] for item in script_status),
    "docs_ready": all(item["exists"] for item in docs_status),
    "artifacts_ready": all(item["exists"] for item in artifact_status),
    "ci_workflow_present": workflow_path.exists(),
    "changelog_has_entries": len(monpol_matches) > 0,
    "approved_entries_have_signoff": len(approved_missing_signoff) == 0,
    "latency_telemetry_present": latency_telemetry["summary"]["entries_evaluated"] > 0,
    "escalation_policy_present": escalation_policy_path.exists(),
    "escalation_owners_present": escalation_owners_path.exists(),
    "escalation_artifact_present": bool(latest_escalation_json),
    "handoff_artifact_present": bool(latest_handoff_json),
    "handoff_not_blocked": bool(latest_handoff_json) and handoff_telemetry.get("overall_status", "n/a") != "blocked",
}

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Week 9 Governance Toolchain Transition Pack\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write("**Purpose**: package transition summary and rollout baseline for Week 10.\n\n")

    fh.write("## Toolchain Components\n\n")
    fh.write("| Component | Path | Exists | Executable |\n")
    fh.write("|---|---|---|---|\n")
    for item in script_status:
        fh.write(
            f"| script | `{item['path']}` | {yes_no(item['exists'])} | {yes_no(item['executable'])} |\n"
        )
    fh.write("\n")

    fh.write("## Governance Assets\n\n")
    fh.write("| Asset | Path | Exists |\n")
    fh.write("|---|---|---|\n")
    for item in docs_status:
        fh.write(f"| governance-doc | `{item['path']}` | {yes_no(item['exists'])} |\n")
    fh.write("\n")

    fh.write("## Latest Evidence Artifacts\n\n")
    fh.write("| Kind | Path | Exists |\n")
    fh.write("|---|---|---|\n")
    for item in artifact_status:
        fh.write(f"| {item['kind']} | `{item['path']}` | {yes_no(item['exists'])} |\n")
    fh.write("\n")

    fh.write("## Current CI Policy Snapshot\n\n")
    fh.write(f"- Strict bench: `{ci_policy['strict_bench']}`\n")
    fh.write(f"- Strict threshold: `{ci_policy['strict_threshold_pct']}%`\n")
    fh.write(f"- Monitor budget seconds: `{ci_policy['monitor_budget_seconds']}`\n")
    fh.write(f"- Monitor case timeout seconds: `{ci_policy['monitor_case_timeout_seconds']}`\n\n")

    fh.write("| Monitor bench | Threshold (%) |\n")
    fh.write("|---|---:|\n")
    if ci_policy["monitor_benches"]:
        for item in ci_policy["monitor_benches"]:
            threshold = item["threshold_pct"] or "n/a"
            fh.write(f"| `{item['bench']}` | {threshold} |\n")
    else:
        fh.write("| _none_ | n/a |\n")
    fh.write("\n")

    fh.write("## MONPOL Registry Snapshot\n\n")
    fh.write(f"- Changelog path: `{rel(changelog_path)}`\n")
    fh.write(f"- Total recorded entries: {len(monpol_matches)}\n")
    fh.write(f"- Latest entry: `{latest_monpol_id}` ({latest_monpol_date})\n\n")

    fh.write("## Signoff Review-Status Telemetry\n\n")
    fh.write(f"- Signoff metadata path: `{rel(signoff_path)}`\n")
    fh.write(f"- Total signoff records: {len(signoff_records)}\n")
    fh.write(f"- Approved changelog entries: {len(approved_entries)}\n")
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
            f"({latest_signoff_record.get('decision', 'n/a')}, {latest_signoff_record.get('approved_at_utc', 'n/a')})\n"
        )
    else:
        fh.write("- Latest signoff record: none\n")
    fh.write("\n")

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

    fh.write("## Proposal-to-Merge Latency Telemetry\n\n")
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

    fh.write("## Monitor Drift Escalation Snapshot\n\n")
    if escalation_telemetry:
        level_counts = escalation_telemetry.get("level_counts", {})
        escalated_benches = escalation_telemetry.get("escalated_benches", [])
        critical_benches = escalation_telemetry.get("critical_benches", [])
        overall_owner = escalation_telemetry.get("overall_owner_profile", {})
        fh.write(f"- Escalation policy doc: `{rel(escalation_policy_path)}`\n")
        fh.write(f"- Escalation owners registry: `{rel(escalation_owners_path)}`\n")
        fh.write(f"- Escalation owners source: `{escalation_telemetry.get('owners_config_source', 'n/a')}`\n")
        fh.write(f"- Escalation artifact: `{escalation_telemetry.get('source', 'n/a')}`\n")
        fh.write(f"- Artifact generated at (UTC): {escalation_telemetry.get('generated_at_utc', 'n/a')}\n")
        fh.write(f"- Overall escalation level: **{escalation_telemetry.get('overall_level', 'n/a')}**\n")
        fh.write(
            f"- Overall owner/SLA: {overall_owner.get('owner', 'n/a')} "
            f"(backup: {overall_owner.get('backup_owner', 'n/a')}, "
            f"SLA={overall_owner.get('response_sla_hours', 'n/a')}h)\n"
        )
        fh.write(f"- Next drill due (UTC): {escalation_telemetry.get('next_drill_due_utc', 'n/a')}\n")
        fh.write(
            f"- Level counts: normal={level_counts.get('normal', 'n/a')}, "
            f"watch={level_counts.get('watch', 'n/a')}, "
            f"escalated={level_counts.get('escalated', 'n/a')}, "
            f"critical={level_counts.get('critical', 'n/a')}\n"
        )
        fh.write(f"- Escalated benches: {len(escalated_benches)}\n")
        fh.write(f"- Critical benches: {len(critical_benches)}\n")
        fh.write(f"- Escalation fail trigger active: {'yes' if escalation_telemetry.get('fail_triggered') else 'no'}\n\n")

        fh.write("| Benchmark | Level | Latest status | Consecutive drift | Drift rate (%) | Failure rate (%) | Owner | SLA (h) | Drill (d) | Handoff | Required action |\n")
        fh.write("|---|---|---|---:|---:|---:|---|---:|---:|---|---|\n")
        rows = escalation_telemetry.get("rows", [])
        if rows:
            for row in sorted(rows, key=lambda item: item.get("bench", "")):
                fh.write(
                    f"| `{row.get('bench', 'n/a')}` | {row.get('level', 'n/a')} | {row.get('latest_status', 'n/a')} | "
                    f"{row.get('consecutive_drift', 'n/a')} | {row.get('drift_rate_pct', 'n/a')} | "
                    f"{row.get('failure_rate_pct', 'n/a')} | {row.get('owner', 'n/a')} | "
                    f"{row.get('response_sla_hours', 'n/a')} | {row.get('drill_cadence_days', 'n/a')} | "
                    f"{'yes' if row.get('release_handoff_required') else 'no'} | {row.get('required_action', 'n/a')} |\n"
                )
        else:
            fh.write("| _none_ | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a | n/a |\n")
        fh.write("\n")
    else:
        fh.write(f"- Escalation policy doc: `{rel(escalation_policy_path)}`\n")
        fh.write("- Escalation artifact: not available\n")
        fh.write("- Overall escalation level: n/a\n\n")

    fh.write("## Release Handoff Checklist Snapshot\n\n")
    if handoff_telemetry:
        fh.write(f"- Handoff artifact: `{handoff_telemetry.get('source', 'n/a')}`\n")
        fh.write(f"- Handoff generated at (UTC): {handoff_telemetry.get('generated_at_utc', 'n/a')}\n")
        fh.write(f"- Overall handoff status: **{handoff_telemetry.get('overall_status', 'n/a')}**\n")
        fh.write(f"- Escalation level: `{handoff_telemetry.get('overall_level', 'n/a')}`\n")
        profile = handoff_telemetry.get("overall_owner_profile", {})
        fh.write(
            f"- Owner/SLA: {profile.get('owner', 'n/a')} "
            f"(backup: {profile.get('backup_owner', 'n/a')}, SLA={profile.get('response_sla_hours', 'n/a')}h)\n"
        )
        fh.write(f"- Next drill due (UTC): {handoff_telemetry.get('next_drill_due_utc', 'n/a')}\n")
        fh.write(
            f"- Release handoff required: {'yes' if handoff_telemetry.get('release_handoff_required') else 'no'}\n\n"
        )

        fh.write("| Checklist ID | Required | Status | Description |\n")
        fh.write("|---|---|---|---|\n")
        for item in handoff_telemetry.get("items", []):
            fh.write(
                f"| `{item.get('id', 'n/a')}` | {'yes' if item.get('required') else 'no'} | "
                f"{item.get('status', 'n/a')} | {item.get('description', 'n/a')} |\n"
            )
        if not handoff_telemetry.get("items"):
            fh.write("| _none_ | no | n/a | n/a |\n")
        fh.write("\n")

        required_failures = handoff_telemetry.get("required_failures", [])
        if required_failures:
            fh.write("Blocking items:\n")
            for item in required_failures:
                fh.write(f"- `{item.get('id', 'n/a')}`: {item.get('description', 'n/a')}\n")
            fh.write("\n")
    else:
        fh.write("- Handoff artifact: not available\n")
        fh.write("- Overall handoff status: n/a\n\n")

    fh.write("## Readiness Checks\n\n")
    for key, value in readiness_checks.items():
        fh.write(f"- {key}: **{yes_no(value)}**\n")
    fh.write("\n")

    fh.write("## Week 10 Transition Plan (Suggested)\n\n")
    fh.write("1. Validate proposal quality gates against real PR scenarios.\n")
    fh.write("2. Validate scaffold quality against reviewer approval workflow.\n")
    fh.write("3. Review signoff telemetry weekly for drift and decision latency.\n")
    fh.write("4. Track policy change latency (proposal -> merge) in dashboard trends.\n")
    fh.write("5. Operationalize escalation policy with owner/SLA drills.\n")
    fh.write("\n")

transition_json = {
    "generated_at_utc": generated_at,
    "source_analysis_dir": rel(analysis_dir),
    "workflow_path": rel(workflow_path),
    "ci_policy": ci_policy,
    "script_status": script_status,
    "docs_status": docs_status,
    "artifact_status": artifact_status,
    "monpol_registry": {
        "changelog_path": rel(changelog_path),
        "entry_count": len(monpol_matches),
        "latest_id": latest_monpol_id,
        "latest_date": latest_monpol_date,
    },
    "signoff_telemetry": {
        "signoff_path": rel(signoff_path),
        "record_count": len(signoff_records),
        "decision_counts": signoff_decision_counts,
        "approved_entries": approved_entries,
        "approved_with_signoff": approved_with_signoff,
        "approved_missing_signoff": approved_missing_signoff,
        "latest_signoff_record": latest_signoff_record,
    },
    "latency_telemetry": latency_telemetry,
    "escalation_telemetry": escalation_telemetry,
    "handoff_telemetry": handoff_telemetry,
    "readiness_checks": readiness_checks,
}
output_json_path.write_text(json.dumps(transition_json, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Governance transition pack markdown: {output_path}")
print(f"Governance transition pack JSON: {output_json_path}")
print(f"Scripts ready: {yes_no(readiness_checks['scripts_ready'])}")
print(f"Artifacts ready: {yes_no(readiness_checks['artifacts_ready'])}")
PY
