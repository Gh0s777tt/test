#!/usr/bin/env bash
# Run ISO live+installer smoke flow with onboarding rollup threshold enforcement.
# Usage:
#   ./scripts/run_iso_onboarding_ci_gate.sh
#   ./scripts/run_iso_onboarding_ci_gate.sh --policy-profile local_fast --dry-run

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_ISO="$ROOT/build/VantisOS-live.iso"
POLICY_JSON="$ROOT/governance/performance/ISO_ONBOARDING_ROLLUP_GATE_POLICY.json"
POLICY_PROFILE=""
USE_POLICY=1
DRY_RUN=0

QEMU_TIMEOUT_SECONDS=180
INSTALLER_TIMEOUT_SECONDS=320
WINDOW=30
MAX_LOCKOUT_RATIO="1.0"
MAX_MEAN_FAILURES="3.0"
REQUIRE_FINAL_SOURCE="import_encrypted"
REQUIRE_FINAL_LAST_EVENT="guard_cleared"
MIN_GUARD_CLEARED_RATIO="1.0"

HAS_QEMU_TIMEOUT=0
HAS_INSTALLER_TIMEOUT=0
HAS_WINDOW=0
HAS_MAX_LOCKOUT_RATIO=0
HAS_MAX_MEAN_FAILURES=0
HAS_REQUIRE_FINAL_SOURCE=0
HAS_REQUIRE_FINAL_LAST_EVENT=0
HAS_MIN_GUARD_CLEARED_RATIO=0

usage() {
  cat <<'USAGE'
Usage: ./scripts/run_iso_onboarding_ci_gate.sh [options]

Options:
  --output <path>                Output ISO path (default: build/VantisOS-live.iso)
  --policy-json <path>           Policy JSON path (default: governance/performance/ISO_ONBOARDING_ROLLUP_GATE_POLICY.json)
  --policy-profile <name>        Policy profile name (default: active_profile from policy JSON)
  --no-policy                    Do not load policy JSON; use script defaults + CLI flags
  --qemu-timeout <seconds>       QEMU smoke timeout
  --installer-timeout <seconds>  Installer smoke timeout
  --window <n>                   Rollup window size
  --max-lockout-ratio <n>        Threshold max lockout ratio 0.0..1.0
  --max-mean-failures <n>        Threshold max mean failures
  --require-final-source <name>  Required latest onboarding source
  --require-final-last-event <name>
                                  Required latest onboarding telemetry last_event
  --min-guard-cleared-ratio <n>  Minimum required guard_cleared run ratio 0.0..1.0
  --allow-any-final-source       Disable final source threshold check
  --allow-any-final-last-event   Disable latest last_event threshold check
  --dry-run                      Print effective config and build command only
  -h, --help                     Show this help
USAGE
}

is_number() {
  local value="$1"
  [[ "$value" =~ ^[0-9]+([.][0-9]+)?$ ]]
}

load_policy_profile() {
  local policy_path="$1"
  local requested_profile="$2"
  python3 - "$policy_path" "$requested_profile" <<'PY'
import json
import sys
from pathlib import Path

policy_path = Path(sys.argv[1])
requested_profile = sys.argv[2]

if not policy_path.exists():
    raise SystemExit(f"Error: policy file not found: {policy_path}")

try:
    payload = json.loads(policy_path.read_text(encoding="utf-8"))
except Exception as exc:
    raise SystemExit(f"Error: failed to parse policy file {policy_path}: {exc}") from exc

if not isinstance(payload, dict):
    raise SystemExit("Error: policy root must be a JSON object")

profiles = payload.get("profiles")
if not isinstance(profiles, dict) or not profiles:
    raise SystemExit("Error: policy JSON must contain non-empty object field 'profiles'")

profile_name = requested_profile or payload.get("active_profile", "")
if not profile_name:
    raise SystemExit("Error: policy does not define active_profile and no --policy-profile was provided")

profile = profiles.get(profile_name)
if not isinstance(profile, dict):
    raise SystemExit(f"Error: policy profile not found: {profile_name}")

required_int_fields = [
    "qemu_timeout_seconds",
    "installer_timeout_seconds",
    "window",
]
required_float_fields = [
    "max_lockout_ratio",
    "max_mean_failures",
    "min_guard_cleared_ratio",
]
for field in required_int_fields:
    if field not in profile:
        raise SystemExit(f"Error: missing required policy field '{field}' in profile '{profile_name}'")
    value = profile[field]
    if not isinstance(value, int):
        raise SystemExit(f"Error: policy field '{field}' in profile '{profile_name}' must be integer")

for field in required_float_fields:
    if field not in profile:
        raise SystemExit(f"Error: missing required policy field '{field}' in profile '{profile_name}'")
    value = profile[field]
    if not isinstance(value, (int, float)):
        raise SystemExit(f"Error: policy field '{field}' in profile '{profile_name}' must be numeric")

require_final_source = profile.get("require_final_source", "")
if require_final_source is None:
    require_final_source = ""
if not isinstance(require_final_source, str):
    raise SystemExit(f"Error: policy field 'require_final_source' in profile '{profile_name}' must be string or null")
require_final_last_event = profile.get("require_final_last_event", "")
if require_final_last_event is None:
    require_final_last_event = ""
if not isinstance(require_final_last_event, str):
    raise SystemExit(
        f"Error: policy field 'require_final_last_event' in profile '{profile_name}' must be string or null"
    )

print(f"PROFILE_NAME={profile_name}")
print(f"QEMU_TIMEOUT_SECONDS={profile['qemu_timeout_seconds']}")
print(f"INSTALLER_TIMEOUT_SECONDS={profile['installer_timeout_seconds']}")
print(f"WINDOW={profile['window']}")
print(f"MAX_LOCKOUT_RATIO={float(profile['max_lockout_ratio'])}")
print(f"MAX_MEAN_FAILURES={float(profile['max_mean_failures'])}")
print(f"MIN_GUARD_CLEARED_RATIO={float(profile['min_guard_cleared_ratio'])}")
print(f"REQUIRE_FINAL_SOURCE={require_final_source}")
print(f"REQUIRE_FINAL_LAST_EVENT={require_final_last_event}")
PY
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --output)
      OUTPUT_ISO="${2:-}"
      shift 2
      ;;
    --policy-json)
      POLICY_JSON="${2:-}"
      shift 2
      ;;
    --policy-profile)
      POLICY_PROFILE="${2:-}"
      shift 2
      ;;
    --no-policy)
      USE_POLICY=0
      shift
      ;;
    --qemu-timeout)
      QEMU_TIMEOUT_SECONDS="${2:-}"
      HAS_QEMU_TIMEOUT=1
      shift 2
      ;;
    --installer-timeout)
      INSTALLER_TIMEOUT_SECONDS="${2:-}"
      HAS_INSTALLER_TIMEOUT=1
      shift 2
      ;;
    --window)
      WINDOW="${2:-}"
      HAS_WINDOW=1
      shift 2
      ;;
    --max-lockout-ratio)
      MAX_LOCKOUT_RATIO="${2:-}"
      HAS_MAX_LOCKOUT_RATIO=1
      shift 2
      ;;
    --max-mean-failures)
      MAX_MEAN_FAILURES="${2:-}"
      HAS_MAX_MEAN_FAILURES=1
      shift 2
      ;;
    --require-final-source)
      REQUIRE_FINAL_SOURCE="${2:-}"
      HAS_REQUIRE_FINAL_SOURCE=1
      shift 2
      ;;
    --require-final-last-event)
      REQUIRE_FINAL_LAST_EVENT="${2:-}"
      HAS_REQUIRE_FINAL_LAST_EVENT=1
      shift 2
      ;;
    --min-guard-cleared-ratio)
      MIN_GUARD_CLEARED_RATIO="${2:-}"
      HAS_MIN_GUARD_CLEARED_RATIO=1
      shift 2
      ;;
    --allow-any-final-source)
      REQUIRE_FINAL_SOURCE=""
      HAS_REQUIRE_FINAL_SOURCE=1
      shift
      ;;
    --allow-any-final-last-event)
      REQUIRE_FINAL_LAST_EVENT=""
      HAS_REQUIRE_FINAL_LAST_EVENT=1
      shift
      ;;
    --dry-run)
      DRY_RUN=1
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

if (( USE_POLICY == 1 )); then
  readarray -t POLICY_KV < <(load_policy_profile "$POLICY_JSON" "$POLICY_PROFILE")
  EFFECTIVE_PROFILE=""
  POLICY_QEMU_TIMEOUT=""
  POLICY_INSTALLER_TIMEOUT=""
  POLICY_WINDOW=""
  POLICY_MAX_LOCKOUT_RATIO=""
  POLICY_MAX_MEAN_FAILURES=""
  POLICY_MIN_GUARD_CLEARED_RATIO=""
  POLICY_REQUIRE_FINAL_SOURCE=""
  POLICY_REQUIRE_FINAL_LAST_EVENT=""
  for kv in "${POLICY_KV[@]}"; do
    key="${kv%%=*}"
    value="${kv#*=}"
    case "$key" in
      PROFILE_NAME) EFFECTIVE_PROFILE="$value" ;;
      QEMU_TIMEOUT_SECONDS) POLICY_QEMU_TIMEOUT="$value" ;;
      INSTALLER_TIMEOUT_SECONDS) POLICY_INSTALLER_TIMEOUT="$value" ;;
      WINDOW) POLICY_WINDOW="$value" ;;
      MAX_LOCKOUT_RATIO) POLICY_MAX_LOCKOUT_RATIO="$value" ;;
      MAX_MEAN_FAILURES) POLICY_MAX_MEAN_FAILURES="$value" ;;
      MIN_GUARD_CLEARED_RATIO) POLICY_MIN_GUARD_CLEARED_RATIO="$value" ;;
      REQUIRE_FINAL_SOURCE) POLICY_REQUIRE_FINAL_SOURCE="$value" ;;
      REQUIRE_FINAL_LAST_EVENT) POLICY_REQUIRE_FINAL_LAST_EVENT="$value" ;;
    esac
  done
  if (( HAS_QEMU_TIMEOUT == 0 )); then QEMU_TIMEOUT_SECONDS="$POLICY_QEMU_TIMEOUT"; fi
  if (( HAS_INSTALLER_TIMEOUT == 0 )); then INSTALLER_TIMEOUT_SECONDS="$POLICY_INSTALLER_TIMEOUT"; fi
  if (( HAS_WINDOW == 0 )); then WINDOW="$POLICY_WINDOW"; fi
  if (( HAS_MAX_LOCKOUT_RATIO == 0 )); then MAX_LOCKOUT_RATIO="$POLICY_MAX_LOCKOUT_RATIO"; fi
  if (( HAS_MAX_MEAN_FAILURES == 0 )); then MAX_MEAN_FAILURES="$POLICY_MAX_MEAN_FAILURES"; fi
  if (( HAS_MIN_GUARD_CLEARED_RATIO == 0 )); then MIN_GUARD_CLEARED_RATIO="$POLICY_MIN_GUARD_CLEARED_RATIO"; fi
  if (( HAS_REQUIRE_FINAL_SOURCE == 0 )); then REQUIRE_FINAL_SOURCE="$POLICY_REQUIRE_FINAL_SOURCE"; fi
  if (( HAS_REQUIRE_FINAL_LAST_EVENT == 0 )); then REQUIRE_FINAL_LAST_EVENT="$POLICY_REQUIRE_FINAL_LAST_EVENT"; fi
  echo "Loaded ISO onboarding gate policy profile: ${EFFECTIVE_PROFILE} (${POLICY_JSON})"
fi

if [[ -z "$OUTPUT_ISO" ]]; then
  echo "Error: --output path cannot be empty" >&2
  exit 1
fi

if ! [[ "$QEMU_TIMEOUT_SECONDS" =~ ^[0-9]+$ ]] || (( QEMU_TIMEOUT_SECONDS < 5 )); then
  echo "Error: --qemu-timeout must be an integer >= 5" >&2
  exit 1
fi

if ! [[ "$INSTALLER_TIMEOUT_SECONDS" =~ ^[0-9]+$ ]] || (( INSTALLER_TIMEOUT_SECONDS < 20 )); then
  echo "Error: --installer-timeout must be an integer >= 20" >&2
  exit 1
fi

if ! [[ "$WINDOW" =~ ^[0-9]+$ ]] || (( WINDOW < 1 )); then
  echo "Error: --window must be an integer >= 1" >&2
  exit 1
fi

if ! is_number "$MAX_LOCKOUT_RATIO"; then
  echo "Error: --max-lockout-ratio must be numeric" >&2
  exit 1
fi

if ! is_number "$MAX_MEAN_FAILURES"; then
  echo "Error: --max-mean-failures must be numeric" >&2
  exit 1
fi

if ! is_number "$MIN_GUARD_CLEARED_RATIO"; then
  echo "Error: --min-guard-cleared-ratio must be numeric" >&2
  exit 1
fi

python3 - "$MAX_LOCKOUT_RATIO" "$MAX_MEAN_FAILURES" "$MIN_GUARD_CLEARED_RATIO" <<'PY'
import sys

ratio = float(sys.argv[1])
mean_failures = float(sys.argv[2])
min_guard_cleared_ratio = float(sys.argv[3])
if not (0.0 <= ratio <= 1.0):
    raise SystemExit("Error: --max-lockout-ratio must be in range 0.0..1.0")
if mean_failures < 0.0:
    raise SystemExit("Error: --max-mean-failures must be >= 0.0")
if not (0.0 <= min_guard_cleared_ratio <= 1.0):
    raise SystemExit("Error: --min-guard-cleared-ratio must be in range 0.0..1.0")
PY

BUILD_CMD=(
  "$ROOT/scripts/build_iso.sh"
  --output "$OUTPUT_ISO"
  --run-qemu-smoke
  --run-installer-smoke
  --qemu-timeout "$QEMU_TIMEOUT_SECONDS"
  --installer-timeout "$INSTALLER_TIMEOUT_SECONDS"
  --onboarding-rollup-window "$WINDOW"
  --onboarding-rollup-max-lockout-ratio "$MAX_LOCKOUT_RATIO"
  --onboarding-rollup-max-mean-failures "$MAX_MEAN_FAILURES"
  --onboarding-rollup-min-guard-cleared-ratio "$MIN_GUARD_CLEARED_RATIO"
)

if [[ -n "$REQUIRE_FINAL_SOURCE" ]]; then
  BUILD_CMD+=(--onboarding-rollup-require-final-source "$REQUIRE_FINAL_SOURCE")
fi
if [[ -n "$REQUIRE_FINAL_LAST_EVENT" ]]; then
  BUILD_CMD+=(--onboarding-rollup-require-final-last-event "$REQUIRE_FINAL_LAST_EVENT")
fi

BUILD_CMD+=(--enforce-onboarding-rollup-thresholds)

if (( DRY_RUN == 1 )); then
  echo "ISO onboarding CI gate dry run:"
  echo "  output_iso=$OUTPUT_ISO"
  echo "  qemu_timeout_seconds=$QEMU_TIMEOUT_SECONDS"
  echo "  installer_timeout_seconds=$INSTALLER_TIMEOUT_SECONDS"
  echo "  window=$WINDOW"
  echo "  max_lockout_ratio=$MAX_LOCKOUT_RATIO"
  echo "  max_mean_failures=$MAX_MEAN_FAILURES"
  echo "  min_guard_cleared_ratio=$MIN_GUARD_CLEARED_RATIO"
  echo "  require_final_source=${REQUIRE_FINAL_SOURCE:-<any>}"
  echo "  require_final_last_event=${REQUIRE_FINAL_LAST_EVENT:-<any>}"
  printf "  command="
  printf '%q ' "${BUILD_CMD[@]}"
  echo
  exit 0
fi

echo "Running ISO onboarding CI gate..."
"${BUILD_CMD[@]}"
