#!/usr/bin/env bash
# Generate branch/tag/release audit report.
# Usage:
#   ./scripts/audit_git_refs.sh [output_file]

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

OUTPUT_FILE="${1:-analysis/GIT_REFS_AUDIT.md}"
mkdir -p "$(dirname "$OUTPUT_FILE")"

NOW_UTC="$(date -u +"%Y-%m-%d %H:%M:%SZ")"
NOW_UNIX="$(date +%s)"
STALE_DAYS=180
STALE_SECONDS=$((STALE_DAYS * 24 * 60 * 60))

REMOTE_URL_RAW="$(git remote get-url origin 2>/dev/null || echo "N/A")"
REMOTE_URL="$(echo "$REMOTE_URL_RAW" | sed -E 's#https://[^/@]+@github.com/#https://github.com/#')"
CURRENT_BRANCH="$(git branch --show-current || true)"
HEAD_COMMIT="$(git rev-parse --short HEAD)"
TOTAL_COMMITS="$(git rev-list --count HEAD)"

{
  echo "# Git refs audit report"
  echo
  echo "- Generated (UTC): $NOW_UTC"
  echo "- Repository root: \`$ROOT\`"
  echo "- Remote: \`$REMOTE_URL\`"
  echo "- Current branch: \`$CURRENT_BRANCH\`"
  echo "- HEAD: \`$HEAD_COMMIT\`"
  echo "- Commit count on current branch: $TOTAL_COMMITS"
  echo
  echo "## Local branches"
  echo
  echo "| Branch | Last commit date | Commit | Subject |"
  echo "|---|---|---|---|"
  git for-each-ref \
    --sort=-committerdate \
    --format='| `%(refname:short)` | %(committerdate:short) | `%(objectname:short)` | %(subject) |' \
    refs/heads
  echo
  echo "## Remote branches (origin)"
  echo
  echo "| Branch | Last commit date | Commit | Subject |"
  echo "|---|---|---|---|"
  git for-each-ref \
    --sort=-committerdate \
    --format='| `%(refname:short)` | %(committerdate:short) | `%(objectname:short)` | %(subject) |' \
    refs/remotes/origin/* \
    | rg -v '^\| `origin` \|'
  echo
  echo "## Top-level structure by remote branch"
  echo
  echo "| Branch | Tracked files | Top-level entries | Sample top-level structure |"
  echo "|---|---|---|---|"
  git for-each-ref --sort=refname --format='%(refname:short)' refs/remotes/origin/* \
    | rg -v '^origin$' \
    | while IFS= read -r branch; do
        file_count="$(git ls-tree -r --name-only "$branch" | wc -l | tr -d ' ')"
        top_count="$(git ls-tree --name-only "$branch" | wc -l | tr -d ' ')"
        sample="$(git ls-tree --name-only "$branch" | awk 'NR<=12 {printf "%s, ", $0} END {if (NR > 12) printf "..."}' | sed 's/, $//')"
        [[ -z "$sample" ]] && sample="(empty)"
        echo "| \`$branch\` | $file_count | $top_count | $sample |"
      done
  echo
  echo "## Stale remote branch candidates (>${STALE_DAYS} days)"
  echo
  echo "_Candidates only. Review manually before deletion._"
  echo
  echo "| Branch | Last commit date | Age (days) | Commit |"
  echo "|---|---|---|---|"
  git for-each-ref \
    --format='%(refname:short)|%(committerdate:short)|%(committerdate:unix)|%(objectname:short)' \
    refs/remotes/origin/* \
    | while IFS='|' read -r branch date_str ts short_sha; do
        [[ "$branch" == "origin" ]] && continue
        age_seconds=$((NOW_UNIX - ts))
        if (( age_seconds > STALE_SECONDS )); then
          age_days=$((age_seconds / 86400))
          echo "| \`$branch\` | $date_str | $age_days | \`$short_sha\` |"
        fi
      done
  echo
  echo "## Tags"
  echo
  echo "| Tag | Date | Commit | Subject |"
  echo "|---|---|---|---|"
  git for-each-ref \
    --sort=-creatordate \
    --format='| `%(refname:short)` | %(creatordate:short) | `%(objectname:short)` | %(subject) |' \
    refs/tags
  echo
  echo "## Releases (GitHub)"
  echo
  if command -v gh >/dev/null 2>&1; then
    release_tmp="$(mktemp)"
    if gh release list --limit 50 > "$release_tmp" 2>/dev/null; then
      if [[ -s "$release_tmp" ]]; then
        echo "| Title | Tag | Date | State |"
        echo "|---|---|---|---|"
        awk -F'\t' '{print "| " $1 " | `" $3 "` | " $4 " | " $2 " |"}' "$release_tmp"
      else
        echo "_No releases found._"
      fi
    else
      echo "_Unable to read releases via GitHub CLI._"
    fi
    rm -f "$release_tmp"
  else
    echo "_GitHub CLI (gh) not installed; release section skipped._"
  fi
  echo
  echo "## Recent commits (HEAD)"
  echo
  echo "| Commit | Date | Subject |"
  echo "|---|---|---|"
  git log --date=short --pretty='| `'%h'` | '%ad' | '%s' |' -n 30
} > "$OUTPUT_FILE"

echo "Audit report generated: $OUTPUT_FILE"
