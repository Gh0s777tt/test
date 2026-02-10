#!/usr/bin/env bash
# Generate changelog-entry scaffold for MONPOL proposal drafts.
# Usage:
#   ./scripts/scaffold_monpol_changelog_entry.sh
#   ./scripts/scaffold_monpol_changelog_entry.sh --proposal-json analysis/benchmark_reproducibility/monitor_threshold_proposal_MONPOL-002_*.json
#   ./scripts/scaffold_monpol_changelog_entry.sh --apply

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
CHANGELOG_PATH="$ROOT/governance/performance/MONITOR_THRESHOLD_CHANGELOG.md"
PROPOSAL_JSON=""
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""
APPLY=0

usage() {
  cat <<'USAGE'
Usage: ./scripts/scaffold_monpol_changelog_entry.sh [options]

Options:
  --analysis-dir <path>     Analysis directory with proposal artifacts
  --proposal-json <path>    Proposal JSON file path (defaults to newest)
  --changelog <path>        MONPOL changelog path
  --output <path>           Output markdown scaffold path
  --output-json <path>      Output JSON scaffold path
  --apply                   Append generated snippet to changelog (if MONPOL entry absent)
  -h, --help                Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --proposal-json)
      PROPOSAL_JSON="${2:-}"
      shift 2
      ;;
    --changelog)
      CHANGELOG_PATH="${2:-}"
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
    --apply)
      APPLY=1
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

if [[ ! -d "$ANALYSIS_DIR" ]]; then
  echo "Error: analysis dir not found: $ANALYSIS_DIR" >&2
  exit 1
fi

if [[ ! -f "$CHANGELOG_PATH" ]]; then
  echo "Error: changelog not found: $CHANGELOG_PATH" >&2
  exit 1
fi

if [[ -z "$PROPOSAL_JSON" ]]; then
  shopt -s nullglob
  proposal_candidates=("$ANALYSIS_DIR"/monitor_threshold_proposal_MONPOL-*_*.json)
  shopt -u nullglob
  if (( ${#proposal_candidates[@]} == 0 )); then
    echo "Error: no proposal json found in $ANALYSIS_DIR" >&2
    exit 1
  fi
  readarray -t sorted_proposals < <(printf '%s\n' "${proposal_candidates[@]}" | sort)
  PROPOSAL_JSON="${sorted_proposals[${#sorted_proposals[@]}-1]}"
fi

if [[ ! -f "$PROPOSAL_JSON" ]]; then
  echo "Error: proposal json not found: $PROPOSAL_JSON" >&2
  exit 1
fi

python3 - "$ROOT" "$PROPOSAL_JSON" "$CHANGELOG_PATH" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" "$APPLY" <<'PY'
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
proposal_path = Path(sys.argv[2])
changelog_path = Path(sys.argv[3])
output_path_arg = sys.argv[4].strip()
output_json_path_arg = sys.argv[5].strip()
apply_flag = bool(int(sys.argv[6]))


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(root.resolve()))
    except ValueError:
        return str(path)


proposal = json.loads(proposal_path.read_text(encoding="utf-8"))
proposal_id = str(proposal.get("proposal_id", "")).strip()
if not re.fullmatch(r"MONPOL-\d{3}", proposal_id):
    raise SystemExit(f"Invalid or missing proposal_id in {proposal_path}: {proposal_id!r}")

rows = proposal.get("rows", [])
changes = proposal.get("proposal_changes", [])
evidence_bundle = proposal.get("evidence_bundle", [])
generated_at = proposal.get("generated_at_utc", "")

now = datetime.now(timezone.utc)
today = now.strftime("%Y-%m-%d")
timestamp = now.strftime("%Y%m%dT%H%M%SZ")

if output_path_arg:
    output_path = Path(output_path_arg)
else:
    output_path = root / "analysis" / "benchmark_reproducibility" / f"monpol_changelog_scaffold_{proposal_id}_{timestamp}.md"

if output_json_path_arg:
    output_json_path = Path(output_json_path_arg)
else:
    output_json_path = output_path.with_suffix(".json")

output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

if changes and not (len(changes) == 1 and changes[0].get("bench") == "none"):
    change_lines = []
    for item in changes:
        bench = item.get("bench", "n/a")
        from_pct = item.get("from_threshold_pct", "n/a")
        to_pct = item.get("to_threshold_pct", "n/a")
        action = item.get("action", "n/a")
        change_lines.append(
            f"  - `{bench}`: `{from_pct}%` -> `{to_pct}%` ({action})"
        )
else:
    change_lines = ["  - No threshold value changes proposed (hold)."]

if rows:
    rationale_bullets = [
        f"  - `{row.get('bench', 'n/a')}`: action `{row.get('action', 'n/a')}`, "
        f"drift reports `{row.get('drift_reports', 'n/a')}`, "
        f"drift rate `{row.get('drift_rate_pct', 'n/a')}%`."
        for row in rows
    ]
else:
    rationale_bullets = ["  - No benchmark rows available in proposal input."]

entry_lines = []
entry_lines.append(f"### {proposal_id} ({today})")
entry_lines.append("")
entry_lines.append("- **Scope**: Threshold policy decision scaffold generated from MONPOL proposal draft.")
entry_lines.append("- **Decision**: _pending reviewer approval_")
entry_lines.append("- **Changes**:")
entry_lines.extend(change_lines)
entry_lines.append("- **Rationale**:")
entry_lines.extend(rationale_bullets)
entry_lines.append("- **Evidence**:")
if evidence_bundle:
    for item in evidence_bundle:
        entry_lines.append(f"  - `{item}`")
else:
    entry_lines.append(f"  - `{rel(proposal_path)}`")
entry_lines.append("- **Proposal Source**:")
entry_lines.append(f"  - `{rel(proposal_path)}`")
if generated_at:
    entry_lines.append(f"  - generated at: `{generated_at}`")
entry_lines.append("- **Reviewer / Owner**: _to be assigned_")
entry_lines.append("- **Signoff Metadata**:")
entry_lines.append("  - add/update `governance/performance/MONPOL_SIGNOFFS.json` if decision becomes approved")
entry_lines.append("")

entry_text = "\n".join(entry_lines).rstrip() + "\n"

with output_path.open("w", encoding="utf-8") as fh:
    fh.write(f"# MONPOL Changelog Entry Scaffold: `{proposal_id}`\n\n")
    fh.write(f"**Source proposal**: `{rel(proposal_path)}`  \n")
    fh.write(f"**Target changelog**: `{rel(changelog_path)}`  \n")
    fh.write(f"**Generated at (UTC)**: {now.replace(microsecond=0).isoformat().replace('+00:00', 'Z')}\n\n")
    fh.write("## Suggested changelog entry\n\n")
    fh.write(entry_text)

changelog_updated = False
if apply_flag:
    changelog_text = changelog_path.read_text(encoding="utf-8")
    if re.search(rf"^###\s+{re.escape(proposal_id)}\s+\(", changelog_text, re.MULTILINE):
        raise SystemExit(f"{proposal_id} already exists in changelog: {changelog_path}")

    if not changelog_text.endswith("\n"):
        changelog_text += "\n"
    changelog_text += "\n" + entry_text
    changelog_path.write_text(changelog_text, encoding="utf-8")
    changelog_updated = True

payload = {
    "proposal_id": proposal_id,
    "source_proposal_json": rel(proposal_path),
    "target_changelog": rel(changelog_path),
    "generated_at_utc": now.replace(microsecond=0).isoformat().replace("+00:00", "Z"),
    "applied_to_changelog": changelog_updated,
    "entry_markdown": entry_text,
    "evidence_bundle": evidence_bundle,
    "row_count": len(rows),
    "signoff_scaffold": {
        "proposal_id": proposal_id,
        "decision": "approved",
        "owner": "to be assigned",
        "approved_at_utc": "YYYY-MM-DDTHH:MM:SSZ",
        "reviewers": [
            {
                "name": "Reviewer Name",
                "role": "Role",
                "signed_at_utc": "YYYY-MM-DDTHH:MM:SSZ",
            }
        ],
    },
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"MONPOL changelog scaffold markdown: {output_path}")
print(f"MONPOL changelog scaffold JSON: {output_json_path}")
print(f"Proposal ID: {proposal_id}")
print(f"Applied to changelog: {'yes' if changelog_updated else 'no'}")
PY
