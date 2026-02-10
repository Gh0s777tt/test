#!/usr/bin/env bash
# Enforce governance requirements for benchmark monitor threshold policy changes.
#
# Trigger conditions (pull_request context):
# - threshold-affecting changes in:
#   - .github/workflows/ci.yml
#   - scripts/run_benchmark_ci_gate.sh
#   - scripts/recommend_monitor_policy.sh
#
# Enforcement when triggered:
# - PR title/body includes MONPOL-<NNN>
# - governance/performance/MONITOR_THRESHOLD_CHANGELOG.md is part of changed files
# - docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md is part of changed files

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

pass() { echo "OK  $1"; }
info() { echo "INFO $1"; }
fail() { echo "ERR $1"; exit 1; }

EVENT_PATH="${GITHUB_EVENT_PATH:-}"
if [[ -z "$EVENT_PATH" || ! -f "$EVENT_PATH" ]]; then
  info "No pull_request event context found. Skipping monitor-threshold governance gate."
  exit 0
fi

HAS_PR="$(python3 - "$EVENT_PATH" <<'PY'
import json
import sys
with open(sys.argv[1], "r", encoding="utf-8") as fh:
    event = json.load(fh)
print("yes" if "pull_request" in event else "no")
PY
)"

if [[ "$HAS_PR" != "yes" ]]; then
  info "Event payload is not a pull_request payload. Skipping."
  exit 0
fi

readarray -t PR_META < <(python3 - "$EVENT_PATH" <<'PY'
import json
import sys

with open(sys.argv[1], "r", encoding="utf-8") as fh:
    event = json.load(fh)

pr = event.get("pull_request", {})
base_sha = pr.get("base", {}).get("sha", "")
head_sha = pr.get("head", {}).get("sha", "")
title = pr.get("title", "")
body = pr.get("body", "") or ""

print(base_sha)
print(head_sha)
print(title.replace("\n", " "))
print(body.replace("\r", " ").replace("\n", " "))
PY
)

BASE_SHA="${PR_META[0]:-}"
HEAD_SHA="${PR_META[1]:-}"
PR_TITLE="${PR_META[2]:-}"
PR_BODY="${PR_META[3]:-}"

if [[ -z "$BASE_SHA" || -z "$HEAD_SHA" ]]; then
  fail "Missing PR base/head SHA in event payload."
fi

CHANGED_FILES="$(git diff --name-only "$BASE_SHA" "$HEAD_SHA")"
if [[ -z "$CHANGED_FILES" ]]; then
  info "No changed files in PR range. Nothing to enforce."
  exit 0
fi

POLICY_FILES_REGEX='^(\.github/workflows/ci\.yml|scripts/run_benchmark_ci_gate\.sh|scripts/recommend_monitor_policy\.sh)$'
if ! echo "$CHANGED_FILES" | rg -q "$POLICY_FILES_REGEX"; then
  info "No monitor-threshold policy files changed. Governance gate skipped."
  exit 0
fi

DIFF_CONTENT="$(git diff "$BASE_SHA" "$HEAD_SHA" -- \
  ".github/workflows/ci.yml" \
  "scripts/run_benchmark_ci_gate.sh" \
  "scripts/recommend_monitor_policy.sh")"

THRESHOLD_CHANGE_REGEX='^[+-].*(STRICT_THRESHOLD_PCT|MONITOR_THRESHOLD_PCT|MONITOR_THRESHOLD_OVERRIDES|--strict-threshold-pct|--monitor-threshold-pct|--monitor-bench [^ ]+:[0-9]|--headroom-pct|--floor-pct|--ceil-pct)'
if ! echo "$DIFF_CONTENT" | rg -q "$THRESHOLD_CHANGE_REGEX"; then
  info "Policy files changed, but no threshold-affecting lines changed. Governance gate skipped."
  exit 0
fi

pass "Detected threshold-affecting policy change"

REFERENCE_TEXT="${PR_TITLE}"$'\n'"${PR_BODY}"
if ! echo "$REFERENCE_TEXT" | rg -q 'MONPOL-[0-9]{3}'; then
  fail "Missing MONPOL-<NNN> reference in PR title/body for threshold-affecting policy change."
fi
POLICY_ID="$(echo "$REFERENCE_TEXT" | rg -o 'MONPOL-[0-9]{3}' | sort -u | awk 'NR==1{print $0}')"
pass "PR includes policy reference: ${POLICY_ID}"

if ! echo "$CHANGED_FILES" | rg -q '^governance/performance/MONITOR_THRESHOLD_CHANGELOG\.md$'; then
  fail "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md must be updated for threshold-affecting policy changes."
fi
pass "Threshold changelog file is included in changed files"

if ! echo "$CHANGED_FILES" | rg -q '^docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE\.md$'; then
  fail "docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md must be updated for threshold-affecting policy changes."
fi
pass "Reproducibility profile documentation is included in changed files"

if [[ ! -f "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md" ]]; then
  fail "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md does not exist at repository head."
fi

if ! rg -q "$POLICY_ID" "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md"; then
  fail "Policy reference ${POLICY_ID} not found in governance/performance/MONITOR_THRESHOLD_CHANGELOG.md."
fi
pass "Policy reference found in threshold changelog"

POLICY_DECISION="$(python3 - "$POLICY_ID" "governance/performance/MONITOR_THRESHOLD_CHANGELOG.md" <<'PY'
import re
import sys
from pathlib import Path

policy_id = sys.argv[1]
path = Path(sys.argv[2])
text = path.read_text(encoding="utf-8")

header_re = re.compile(r"^###\s+(MONPOL-\d{3})\s+\([^)]+\)\s*$", re.MULTILINE)
headers = list(header_re.finditer(text))
block = ""
for idx, match in enumerate(headers):
    if match.group(1) != policy_id:
        continue
    start = match.start()
    end = headers[idx + 1].start() if idx + 1 < len(headers) else len(text)
    block = text[start:end]
    break

if not block:
    print("unknown")
    raise SystemExit(0)

decision_match = re.search(r"^- \*\*Decision\*\*:\s*(.+)$", block, re.MULTILINE)
if not decision_match:
    print("unknown")
    raise SystemExit(0)

value = decision_match.group(1).strip().lower()
if "approved" in value:
    print("approved")
elif "reject" in value:
    print("rejected")
elif "withdraw" in value:
    print("withdrawn")
elif "defer" in value:
    print("deferred")
elif "pending" in value:
    print("pending")
else:
    print("unknown")
PY
)"
info "Detected changelog decision for ${POLICY_ID}: ${POLICY_DECISION}"

if [[ "$POLICY_DECISION" == "approved" ]]; then
  if ! echo "$CHANGED_FILES" | rg -q '^governance/performance/MONPOL_SIGNOFFS\.json$'; then
    fail "Approved policy decision requires governance/performance/MONPOL_SIGNOFFS.json update in this PR."
  fi
  pass "Signoff metadata registry is included in changed files"

  if [[ ! -x "scripts/validate_monpol_signoff_metadata.sh" ]]; then
    fail "scripts/validate_monpol_signoff_metadata.sh is required for approved policy decisions."
  fi
  if ./scripts/validate_monpol_signoff_metadata.sh >/dev/null; then
    pass "MONPOL signoff metadata validation passed for approved decision"
  else
    fail "MONPOL signoff metadata validation failed for approved decision"
  fi
fi

echo "Monitor threshold governance gate passed."
