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

python3 - "$ROOT" "$ANALYSIS_DIR" "$WORKFLOW_PATH" "$CHANGELOG_PATH" "$PROFILE_DOC_PATH" "$TEMPLATE_PATH" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" <<'PY'
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
analysis_dir = Path(sys.argv[2])
workflow_path = Path(sys.argv[3])
changelog_path = Path(sys.argv[4])
profile_doc_path = Path(sys.argv[5])
template_path = Path(sys.argv[6])
output_path = Path(sys.argv[7])
output_json_path = Path(sys.argv[8])


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


latest_summary = latest("ci_benchmark_gate_summary_*.md")
latest_recommendation_md = latest("monitor_policy_recommendations_*.md")
latest_recommendation_json = latest("monitor_policy_recommendations_*.json")
latest_dashboard_md = latest("monitor_policy_dashboard_*.md")
latest_dashboard_json = latest("monitor_policy_dashboard_*.json")
latest_proposal_md = latest("monitor_threshold_proposal_MONPOL-*_*.md")
latest_proposal_json = latest("monitor_threshold_proposal_MONPOL-*_*.json")
latest_scaffold_md = latest("monpol_changelog_scaffold_MONPOL-*_*.md")
latest_scaffold_json = latest("monpol_changelog_scaffold_MONPOL-*_*.json")

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

scripts_required = [
    root / "scripts/run_benchmark_ci_gate.sh",
    root / "scripts/recommend_monitor_policy.sh",
    root / "scripts/build_monitor_policy_dashboard.sh",
    root / "scripts/generate_monitor_threshold_proposal.sh",
    root / "scripts/scaffold_monpol_changelog_entry.sh",
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
    {
        "path": rel(root / "governance/performance/MONPOL_SIGNOFFS.json"),
        "exists": (root / "governance/performance/MONPOL_SIGNOFFS.json").exists(),
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
]

readiness_checks = {
    "scripts_ready": all(item["exists"] and item["executable"] for item in script_status),
    "docs_ready": all(item["exists"] for item in docs_status),
    "artifacts_ready": all(item["exists"] for item in artifact_status),
    "ci_workflow_present": workflow_path.exists(),
    "changelog_has_entries": len(monpol_matches) > 0,
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

    fh.write("## Readiness Checks\n\n")
    for key, value in readiness_checks.items():
        fh.write(f"- {key}: **{yes_no(value)}**\n")
    fh.write("\n")

    fh.write("## Week 10 Transition Plan (Suggested)\n\n")
    fh.write("1. Validate proposal quality gates against real PR scenarios.\n")
    fh.write("2. Validate scaffold quality against reviewer approval workflow.\n")
    fh.write("3. Introduce reviewer sign-off metadata in proposal JSON artifacts.\n")
    fh.write("4. Track policy change latency (proposal -> merge) in dashboard trends.\n")
    fh.write("5. Define escalation policy for repeated monitor drift across releases.\n")
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
    "readiness_checks": readiness_checks,
}
output_json_path.write_text(json.dumps(transition_json, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Governance transition pack markdown: {output_path}")
print(f"Governance transition pack JSON: {output_json_path}")
print(f"Scripts ready: {yes_no(readiness_checks['scripts_ready'])}")
print(f"Artifacts ready: {yes_no(readiness_checks['artifacts_ready'])}")
PY
