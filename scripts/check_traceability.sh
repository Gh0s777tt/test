#!/usr/bin/env bash
# Check governance traceability artifacts for basic consistency.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

TRACE_MD="governance/TRACEABILITY.md"
TRACE_CSV="governance/certification/do-178c/traceability-matrix.csv"

errors=0

pass() { echo "OK  $1"; }
fail() { echo "ERR $1"; errors=$((errors + 1)); }

check_path_exists() {
  local path="$1"
  if [[ "$path" == "ALL" ]]; then
    return
  fi
  if [[ -e "$path" ]]; then
    pass "Path exists: $path"
  else
    fail "Missing path: $path"
  fi
}

if [[ ! -f "$TRACE_MD" ]]; then
  fail "Missing markdown traceability file: $TRACE_MD"
else
  pass "Found markdown traceability file"
fi

if [[ ! -f "$TRACE_CSV" ]]; then
  fail "Missing CSV traceability file: $TRACE_CSV"
else
  pass "Found CSV traceability file"
fi

if [[ -f "$TRACE_MD" ]]; then
  rows="$(rg '^\| REQ-' "$TRACE_MD" || true)"
  if [[ -z "$rows" ]]; then
    fail "No REQ-* rows found in $TRACE_MD"
  else
    while IFS= read -r row; do
      [[ -z "$row" ]] && continue
      req_id="$(echo "$row" | awk -F'|' '{print $2}' | xargs)"
      impl_field="$(echo "$row" | awk -F'|' '{print $4}' | xargs)"

      if [[ -z "$req_id" || -z "$impl_field" ]]; then
        fail "Malformed traceability row: $row"
        continue
      fi

      while IFS= read -r impl; do
        impl="$(echo "$impl" | xargs)"
        [[ -z "$impl" ]] && continue
        check_path_exists "$impl"
      done < <(echo "$impl_field" | tr ',' '\n')
    done <<< "$rows"
  fi
fi

if [[ -f "$TRACE_CSV" ]]; then
  header="$(head -n 1 "$TRACE_CSV" | tr -d '\r')"
  if [[ "$header" != "HLR,LLR,Source File,Verification Method" ]]; then
    fail "Unexpected CSV header in $TRACE_CSV: $header"
  else
    pass "CSV header is valid"
  fi

  while IFS=, read -r hlr llr source_file verification_method; do
    [[ "$hlr" == "HLR" ]] && continue
    [[ -z "$hlr" ]] && continue
    if [[ -z "$source_file" || -z "$verification_method" ]]; then
      fail "Malformed CSV row: $hlr,$llr,$source_file,$verification_method"
      continue
    fi
    source_file="$(echo "$source_file" | xargs)"
    check_path_exists "$source_file"
  done < "$TRACE_CSV"
fi

if [[ "$errors" -gt 0 ]]; then
  echo "Traceability check failed with $errors error(s)."
  exit 1
fi

echo "Traceability check passed."
