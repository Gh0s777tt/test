#!/usr/bin/env bash
# Enforce REQ-* references for security-critical PR changes.
#
# Behavior:
# - On pull_request events (via GITHUB_EVENT_PATH), checks changed files
#   between PR base/head commits.
# - If changes touch security-critical paths, PR title/body must contain
#   at least one requirement ID matching REQ-<DOMAIN>-NNN.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

pass() { echo "OK  $1"; }
fail() { echo "ERR $1"; exit 1; }
info() { echo "INFO $1"; }

event_path="${GITHUB_EVENT_PATH:-}"

if [[ -z "$event_path" || ! -f "$event_path" ]]; then
  info "No pull_request event context found. Skipping requirement-ID gate."
  exit 0
fi

event_name="$(python3 - "$event_path" <<'PY'
import json
import sys
with open(sys.argv[1], "r", encoding="utf-8") as fh:
    event = json.load(fh)
print(event.get("action", ""))
PY
)"

# Only enforce on PR-like payloads that contain pull_request metadata.
has_pr="$(python3 - "$event_path" <<'PY'
import json
import sys
with open(sys.argv[1], "r", encoding="utf-8") as fh:
    event = json.load(fh)
print("yes" if "pull_request" in event else "no")
PY
)"

if [[ "$has_pr" != "yes" ]]; then
  info "Event payload is not a pull_request payload. Skipping."
  exit 0
fi

readarray -t pr_meta < <(python3 - "$event_path" <<'PY'
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

base_sha="${pr_meta[0]:-}"
head_sha="${pr_meta[1]:-}"
pr_title="${pr_meta[2]:-}"
pr_body="${pr_meta[3]:-}"

if [[ -z "$base_sha" || -z "$head_sha" ]]; then
  fail "Missing PR base/head SHA in event payload."
fi

changed_files="$(git diff --name-only "$base_sha" "$head_sha")"

if [[ -z "$changed_files" ]]; then
  info "No changed files in PR range. Nothing to enforce."
  exit 0
fi

critical_paths='^(security/|src/verified/|governance/|\.github/workflows/)'
if ! echo "$changed_files" | rg -q "$critical_paths"; then
  info "PR does not touch security-critical paths. Requirement-ID gate skipped."
  exit 0
fi

pass "Detected security-critical path changes"

reference_text="${pr_title}"$'\n'"${pr_body}"
if echo "$reference_text" | rg -q 'REQ-[A-Z]+-[0-9]{3}'; then
  pass "PR title/body includes requirement ID reference"
  exit 0
fi

fail "Missing REQ-* reference in PR title/body for security-critical changes."
