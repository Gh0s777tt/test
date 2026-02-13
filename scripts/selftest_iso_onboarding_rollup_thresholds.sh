#!/usr/bin/env bash
# Fast self-test for ISO onboarding rollup threshold evaluation logic.
# Usage:
#   ./scripts/selftest_iso_onboarding_rollup_thresholds.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ROLLUP_SCRIPT="$ROOT/scripts/generate_iso_onboarding_telemetry_rollup.sh"

if [[ ! -x "$ROLLUP_SCRIPT" ]]; then
  echo "Error: rollup script not executable: $ROLLUP_SCRIPT" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

ANALYSIS_DIR="$TMP_DIR/analysis"
mkdir -p "$ANALYSIS_DIR"

cat > "$ANALYSIS_DIR/iso_onboarding_telemetry_summary_20260101T000000Z.json" <<'JSON'
{
  "schema": "vantis.iso.onboarding_telemetry_summary.v1",
  "aggregate": {
    "lockout_triggered": true,
    "max_failures_observed": 3,
    "final_onboarding_source": "import_encrypted",
    "final_telemetry_last_event": "guard_cleared",
    "history_contains_lockout": true,
    "history_contains_guard_cleared": true
  },
  "boot_log": {
    "failed_attempt_messages": 3,
    "lockout_messages": 1
  },
  "reboot_log": {
    "failed_attempt_messages": 0,
    "lockout_messages": 0
  }
}
JSON

cat > "$ANALYSIS_DIR/iso_onboarding_telemetry_summary_20260101T010000Z.json" <<'JSON'
{
  "schema": "vantis.iso.onboarding_telemetry_summary.v1",
  "aggregate": {
    "lockout_triggered": true,
    "max_failures_observed": 3,
    "final_onboarding_source": "import_encrypted",
    "final_telemetry_last_event": "guard_cleared",
    "history_contains_lockout": true,
    "history_contains_guard_cleared": true
  },
  "boot_log": {
    "failed_attempt_messages": 3,
    "lockout_messages": 1
  },
  "reboot_log": {
    "failed_attempt_messages": 0,
    "lockout_messages": 0
  }
}
JSON

PASS_JSON="$TMP_DIR/pass.json"
PASS_MD="$TMP_DIR/pass.md"
PASS_LATEST_JSON="$TMP_DIR/pass_latest.json"
PASS_LATEST_MD="$TMP_DIR/pass_latest.md"

bash "$ROLLUP_SCRIPT" \
  --analysis-dir "$ANALYSIS_DIR" \
  --window 2 \
  --max-lockout-ratio 1.0 \
  --max-mean-failures 3.0 \
  --require-final-source import_encrypted \
  --require-final-last-event guard_cleared \
  --min-guard-cleared-ratio 1.0 \
  --output-json "$PASS_JSON" \
  --output-md "$PASS_MD" \
  --latest-json "$PASS_LATEST_JSON" \
  --latest-md "$PASS_LATEST_MD"

python3 - "$PASS_JSON" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
status = payload.get("threshold_evaluation", {}).get("status")
guard_ratio = payload.get("aggregate", {}).get("guard_cleared_run_ratio")
if status != "pass":
    raise SystemExit(f"expected pass status, got: {status!r}")
if guard_ratio != 1.0:
    raise SystemExit(f"expected guard_cleared_run_ratio=1.0, got: {guard_ratio!r}")
PY

FAIL_JSON="$TMP_DIR/fail.json"
FAIL_MD="$TMP_DIR/fail.md"
FAIL_LATEST_JSON="$TMP_DIR/fail_latest.json"
FAIL_LATEST_MD="$TMP_DIR/fail_latest.md"

set +e
bash "$ROLLUP_SCRIPT" \
  --analysis-dir "$ANALYSIS_DIR" \
  --window 2 \
  --max-lockout-ratio 1.0 \
  --max-mean-failures 3.0 \
  --require-final-source import_encrypted \
  --require-final-last-event blocked_attempt \
  --min-guard-cleared-ratio 1.0 \
  --fail-on-threshold-breach \
  --output-json "$FAIL_JSON" \
  --output-md "$FAIL_MD" \
  --latest-json "$FAIL_LATEST_JSON" \
  --latest-md "$FAIL_LATEST_MD" >/dev/null 2>&1
RC=$?
set -e

if [[ "$RC" -ne 2 ]]; then
  echo "Error: expected fail-on-threshold-breach exit code 2, got $RC" >&2
  exit 1
fi

python3 - "$FAIL_JSON" <<'PY'
import json
import sys
from pathlib import Path

payload = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
status = payload.get("threshold_evaluation", {}).get("status")
breaches = payload.get("threshold_evaluation", {}).get("breaches", [])
if status != "fail":
    raise SystemExit(f"expected fail status, got: {status!r}")
if not any("final_telemetry_last_event" in breach for breach in breaches):
    raise SystemExit(f"expected final_telemetry_last_event breach, got: {breaches!r}")
PY

echo "OK ISO onboarding rollup threshold self-test passed"
