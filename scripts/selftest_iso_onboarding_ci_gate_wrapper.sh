#!/usr/bin/env bash
# Fast self-test for run_iso_onboarding_ci_gate.sh policy loading and CLI override behavior.
# Usage:
#   ./scripts/selftest_iso_onboarding_ci_gate_wrapper.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GATE_SCRIPT="$ROOT/scripts/run_iso_onboarding_ci_gate.sh"

if [[ ! -x "$GATE_SCRIPT" ]]; then
  echo "Error: gate wrapper script not executable: $GATE_SCRIPT" >&2
  exit 1
fi

TMP_DIR="$(mktemp -d)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

run_case() {
  local name="$1"
  shift
  local out="$TMP_DIR/${name}.out"
  "$GATE_SCRIPT" "$@" --dry-run >"$out"
  printf '%s\n' "$out"
}

assert_contains() {
  local path="$1"
  local pattern="$2"
  local label="$3"
  if ! rg -q -- "$pattern" "$path"; then
    echo "Error: expected pattern not found ($label): $pattern" >&2
    echo "---- output ----" >&2
    cat "$path" >&2
    echo "----------------" >&2
    exit 1
  fi
}

assert_not_contains() {
  local path="$1"
  local pattern="$2"
  local label="$3"
  if rg -q -- "$pattern" "$path"; then
    echo "Error: unexpected pattern present ($label): $pattern" >&2
    echo "---- output ----" >&2
    cat "$path" >&2
    echo "----------------" >&2
    exit 1
  fi
}

# Case 1: local_fast profile values are loaded.
OUT_LOCAL_FAST="$(run_case local_fast --policy-profile local_fast)"
assert_contains "$OUT_LOCAL_FAST" "qemu_timeout_seconds=120" "local_fast qemu timeout"
assert_contains "$OUT_LOCAL_FAST" "installer_timeout_seconds=260" "local_fast installer timeout"
assert_contains "$OUT_LOCAL_FAST" "window=15" "local_fast window"
assert_contains "$OUT_LOCAL_FAST" "max_lockout_ratio=1.0" "local_fast max lockout ratio"
assert_contains "$OUT_LOCAL_FAST" "max_mean_failures=3.0" "local_fast max mean failures"
assert_contains "$OUT_LOCAL_FAST" "min_guard_cleared_ratio=1.0" "local_fast min guard cleared ratio"
assert_contains "$OUT_LOCAL_FAST" "require_final_source=import_encrypted" "local_fast final source"
assert_contains "$OUT_LOCAL_FAST" "require_final_last_event=guard_cleared" "local_fast final last event"

# Case 2: CLI overrides win over profile values.
OUT_OVERRIDES="$(run_case overrides \
  --policy-profile local_fast \
  --qemu-timeout 222 \
  --installer-timeout 333 \
  --window 9 \
  --max-lockout-ratio 0.7 \
  --max-mean-failures 1.5 \
  --min-guard-cleared-ratio 0.5 \
  --require-final-source override_source \
  --require-final-last-event override_event)"
assert_contains "$OUT_OVERRIDES" "qemu_timeout_seconds=222" "override qemu timeout"
assert_contains "$OUT_OVERRIDES" "installer_timeout_seconds=333" "override installer timeout"
assert_contains "$OUT_OVERRIDES" "window=9" "override window"
assert_contains "$OUT_OVERRIDES" "max_lockout_ratio=0.7" "override lockout ratio"
assert_contains "$OUT_OVERRIDES" "max_mean_failures=1.5" "override mean failures"
assert_contains "$OUT_OVERRIDES" "min_guard_cleared_ratio=0.5" "override min guard cleared ratio"
assert_contains "$OUT_OVERRIDES" "require_final_source=override_source" "override final source"
assert_contains "$OUT_OVERRIDES" "require_final_last_event=override_event" "override final last event"

# Case 3: allow-any flags remove final source/event requirements from command.
OUT_ALLOW_ANY="$(run_case allow_any \
  --policy-profile ci_default \
  --allow-any-final-source \
  --allow-any-final-last-event)"
assert_contains "$OUT_ALLOW_ANY" "require_final_source=<any>" "allow any final source"
assert_contains "$OUT_ALLOW_ANY" "require_final_last_event=<any>" "allow any final last event"
assert_not_contains "$OUT_ALLOW_ANY" "--onboarding-rollup-require-final-source" "command final source arg removed"
assert_not_contains "$OUT_ALLOW_ANY" "--onboarding-rollup-require-final-last-event" "command final last event arg removed"

# Case 4: --no-policy keeps script defaults.
OUT_NO_POLICY="$(run_case no_policy --no-policy)"
assert_contains "$OUT_NO_POLICY" "qemu_timeout_seconds=180" "no-policy default qemu timeout"
assert_contains "$OUT_NO_POLICY" "installer_timeout_seconds=320" "no-policy default installer timeout"
assert_contains "$OUT_NO_POLICY" "window=30" "no-policy default window"
assert_contains "$OUT_NO_POLICY" "min_guard_cleared_ratio=1.0" "no-policy default min guard cleared ratio"
assert_contains "$OUT_NO_POLICY" "require_final_last_event=guard_cleared" "no-policy default final last event"

echo "OK ISO onboarding CI gate wrapper self-test passed"
