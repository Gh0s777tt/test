#!/usr/bin/env bash
# Validate reviewer signoff metadata for MONPOL approvals.
# Usage:
#   ./scripts/validate_monpol_signoff_metadata.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CHANGELOG_PATH="$ROOT/governance/performance/MONITOR_THRESHOLD_CHANGELOG.md"
SIGNOFF_PATH="$ROOT/governance/performance/MONPOL_SIGNOFFS.json"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/validate_monpol_signoff_metadata.sh [options]

Options:
  --changelog <path>      MONPOL changelog path
  --signoff <path>        MONPOL signoff metadata JSON path
  --analysis-dir <path>   Output directory for validation reports
  --output <path>         Markdown output path
  --output-json <path>    JSON output path
  -h, --help              Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --changelog)
      CHANGELOG_PATH="${2:-}"
      shift 2
      ;;
    --signoff)
      SIGNOFF_PATH="${2:-}"
      shift 2
      ;;
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
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
  OUTPUT_PATH="$ANALYSIS_DIR/monpol_signoff_validation_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - "$ROOT" "$CHANGELOG_PATH" "$SIGNOFF_PATH" "$OUTPUT_PATH" "$OUTPUT_JSON_PATH" <<'PY'
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

root = Path(sys.argv[1])
changelog_path = Path(sys.argv[2])
signoff_path = Path(sys.argv[3])
output_path = Path(sys.argv[4])
output_json_path = Path(sys.argv[5])


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(root.resolve()))
    except ValueError:
        return str(path)


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


errors = []
warnings = []

changelog_text = changelog_path.read_text(encoding="utf-8")
header_pattern = re.compile(r"^###\s+(MONPOL-\d{3})\s+\(([^)]+)\)\s*$", re.MULTILINE)
headers = list(header_pattern.finditer(changelog_text))

entries = []
for idx, match in enumerate(headers):
    start = match.start()
    end = headers[idx + 1].start() if idx + 1 < len(headers) else len(changelog_text)
    block = changelog_text[start:end]
    decision_match = re.search(r"^- \*\*Decision\*\*:\s*(.+)$", block, re.MULTILINE)
    raw_decision = decision_match.group(1).strip() if decision_match else ""
    decision = normalize_decision(raw_decision)
    entries.append(
        {
            "proposal_id": match.group(1),
            "date": match.group(2),
            "decision": decision,
            "raw_decision": raw_decision or "n/a",
        }
    )

approved_ids = {entry["proposal_id"] for entry in entries if entry["decision"] == "approved"}

try:
    signoff_payload = json.loads(signoff_path.read_text(encoding="utf-8"))
except json.JSONDecodeError as exc:
    raise SystemExit(f"Invalid JSON in signoff metadata: {exc}") from exc

if not isinstance(signoff_payload, dict):
    raise SystemExit("Signoff metadata root must be an object")

records = signoff_payload.get("records")
if not isinstance(records, list):
    raise SystemExit("Signoff metadata must contain 'records' list")

records_by_id = {}
for idx, record in enumerate(records):
    if not isinstance(record, dict):
        errors.append(f"Record #{idx + 1} is not an object")
        continue

    proposal_id = str(record.get("proposal_id", "")).strip()
    decision = normalize_decision(str(record.get("decision", "")).strip())
    owner = str(record.get("owner", "")).strip()
    reviewers = record.get("reviewers", [])
    approved_at_utc = str(record.get("approved_at_utc", "")).strip()

    if not re.fullmatch(r"MONPOL-\d{3}", proposal_id):
        errors.append(f"Record #{idx + 1} has invalid proposal_id: {proposal_id!r}")
        continue

    if proposal_id in records_by_id:
        errors.append(f"Duplicate signoff record for {proposal_id}")
        continue

    if decision not in {"approved", "rejected", "withdrawn", "deferred", "pending", "unknown"}:
        errors.append(f"{proposal_id}: invalid decision value")

    if not owner:
        errors.append(f"{proposal_id}: missing owner")

    if decision == "approved":
        if not approved_at_utc:
            errors.append(f"{proposal_id}: approved decision requires approved_at_utc")
        elif not re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", approved_at_utc):
            errors.append(f"{proposal_id}: invalid approved_at_utc format (expected UTC ISO-8601 with Z)")

        if not isinstance(reviewers, list) or len(reviewers) == 0:
            errors.append(f"{proposal_id}: approved decision requires non-empty reviewers list")
        else:
            for ridx, reviewer in enumerate(reviewers):
                if not isinstance(reviewer, dict):
                    errors.append(f"{proposal_id}: reviewer #{ridx + 1} is not an object")
                    continue
                name = str(reviewer.get("name", "")).strip()
                role = str(reviewer.get("role", "")).strip()
                signed_at = str(reviewer.get("signed_at_utc", "")).strip()
                if not name:
                    errors.append(f"{proposal_id}: reviewer #{ridx + 1} missing name")
                if not role:
                    errors.append(f"{proposal_id}: reviewer #{ridx + 1} missing role")
                if not signed_at or not re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", signed_at):
                    errors.append(
                        f"{proposal_id}: reviewer #{ridx + 1} missing/invalid signed_at_utc (UTC ISO-8601 Z)"
                    )

    records_by_id[proposal_id] = {
        "proposal_id": proposal_id,
        "decision": decision,
        "owner": owner or "n/a",
        "reviewer_count": len(reviewers) if isinstance(reviewers, list) else 0,
        "approved_at_utc": approved_at_utc or "n/a",
    }

for proposal_id in sorted(approved_ids):
    record = records_by_id.get(proposal_id)
    if record is None:
        errors.append(f"{proposal_id}: changelog decision is approved but signoff metadata record is missing")
        continue
    if record["decision"] != "approved":
        errors.append(
            f"{proposal_id}: changelog decision is approved but signoff metadata decision is {record['decision']}"
        )

for proposal_id, record in sorted(records_by_id.items()):
    if record["decision"] == "approved" and proposal_id not in approved_ids:
        warnings.append(
            f"{proposal_id}: signoff decision is approved but changelog decision is not approved/explicit"
        )

output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

generated_at = datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# MONPOL Signoff Metadata Validation Report\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Changelog**: `{rel(changelog_path)}`  \n")
    fh.write(f"**Signoff metadata**: `{rel(signoff_path)}`\n\n")

    fh.write("## Summary\n\n")
    fh.write(f"- Changelog entries: {len(entries)}\n")
    fh.write(f"- Approved changelog entries: {len(approved_ids)}\n")
    fh.write(f"- Signoff records: {len(records_by_id)}\n")
    fh.write(f"- Errors: {len(errors)}\n")
    fh.write(f"- Warnings: {len(warnings)}\n\n")

    fh.write("## Changelog Decisions\n\n")
    fh.write("| Proposal | Date | Decision | Raw decision |\n")
    fh.write("|---|---|---|---|\n")
    if entries:
        for entry in entries:
            fh.write(
                f"| `{entry['proposal_id']}` | {entry['date']} | {entry['decision']} | {entry['raw_decision']} |\n"
            )
    else:
        fh.write("| _none_ | n/a | n/a | n/a |\n")
    fh.write("\n")

    fh.write("## Signoff Records\n\n")
    fh.write("| Proposal | Decision | Owner | Reviewers | approved_at_utc |\n")
    fh.write("|---|---|---|---:|---|\n")
    if records_by_id:
        for proposal_id in sorted(records_by_id):
            record = records_by_id[proposal_id]
            fh.write(
                f"| `{proposal_id}` | {record['decision']} | {record['owner']} | {record['reviewer_count']} | {record['approved_at_utc']} |\n"
            )
    else:
        fh.write("| _none_ | n/a | n/a | 0 | n/a |\n")
    fh.write("\n")

    if errors:
        fh.write("## Errors\n\n")
        for item in errors:
            fh.write(f"- {item}\n")
        fh.write("\n")

    if warnings:
        fh.write("## Warnings\n\n")
        for item in warnings:
            fh.write(f"- {item}\n")
        fh.write("\n")

    fh.write("Validation fails when errors > 0.\n")

result = {
    "generated_at_utc": generated_at,
    "changelog_path": rel(changelog_path),
    "signoff_metadata_path": rel(signoff_path),
    "summary": {
        "changelog_entries": len(entries),
        "approved_changelog_entries": len(approved_ids),
        "signoff_records": len(records_by_id),
        "errors": len(errors),
        "warnings": len(warnings),
    },
    "changelog_entries": entries,
    "signoff_records": [records_by_id[pid] for pid in sorted(records_by_id)],
    "errors": errors,
    "warnings": warnings,
}
output_json_path.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"MONPOL signoff validation markdown: {output_path}")
print(f"MONPOL signoff validation JSON: {output_json_path}")
print(f"Errors: {len(errors)}")
print(f"Warnings: {len(warnings)}")

if errors:
    raise SystemExit(1)
PY
