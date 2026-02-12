#!/usr/bin/env bash
# Run ISO live+installer smoke flow with onboarding rollup threshold enforcement.
# Usage:
#   ./scripts/run_iso_onboarding_ci_gate.sh
#   ./scripts/run_iso_onboarding_ci_gate.sh --window 20 --max-lockout-ratio 0.8

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_ISO="$ROOT/build/VantisOS-live.iso"
QEMU_TIMEOUT_SECONDS=180
INSTALLER_TIMEOUT_SECONDS=320
WINDOW=30
MAX_LOCKOUT_RATIO="1.0"
MAX_MEAN_FAILURES="3.0"
REQUIRE_FINAL_SOURCE="import_encrypted"

usage() {
  cat <<'USAGE'
Usage: ./scripts/run_iso_onboarding_ci_gate.sh [options]

Options:
  --output <path>                Output ISO path (default: build/VantisOS-live.iso)
  --qemu-timeout <seconds>       QEMU smoke timeout (default: 180)
  --installer-timeout <seconds>  Installer smoke timeout (default: 320)
  --window <n>                   Rollup window size (default: 30)
  --max-lockout-ratio <n>        Threshold max lockout ratio 0.0..1.0 (default: 1.0)
  --max-mean-failures <n>        Threshold max mean failures (default: 3.0)
  --require-final-source <name>  Required latest onboarding source (default: import_encrypted)
  --allow-any-final-source       Disable final source threshold check
  -h, --help                     Show this help
USAGE
}

is_number() {
  local value="$1"
  [[ "$value" =~ ^[0-9]+([.][0-9]+)?$ ]]
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --output)
      OUTPUT_ISO="${2:-}"
      shift 2
      ;;
    --qemu-timeout)
      QEMU_TIMEOUT_SECONDS="${2:-}"
      shift 2
      ;;
    --installer-timeout)
      INSTALLER_TIMEOUT_SECONDS="${2:-}"
      shift 2
      ;;
    --window)
      WINDOW="${2:-}"
      shift 2
      ;;
    --max-lockout-ratio)
      MAX_LOCKOUT_RATIO="${2:-}"
      shift 2
      ;;
    --max-mean-failures)
      MAX_MEAN_FAILURES="${2:-}"
      shift 2
      ;;
    --require-final-source)
      REQUIRE_FINAL_SOURCE="${2:-}"
      shift 2
      ;;
    --allow-any-final-source)
      REQUIRE_FINAL_SOURCE=""
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

python3 - "$MAX_LOCKOUT_RATIO" "$MAX_MEAN_FAILURES" <<'PY'
import sys

ratio = float(sys.argv[1])
mean_failures = float(sys.argv[2])
if not (0.0 <= ratio <= 1.0):
    raise SystemExit("Error: --max-lockout-ratio must be in range 0.0..1.0")
if mean_failures < 0.0:
    raise SystemExit("Error: --max-mean-failures must be >= 0.0")
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
)

if [[ -n "$REQUIRE_FINAL_SOURCE" ]]; then
  BUILD_CMD+=(--onboarding-rollup-require-final-source "$REQUIRE_FINAL_SOURCE")
fi

BUILD_CMD+=(--enforce-onboarding-rollup-thresholds)

echo "Running ISO onboarding CI gate..."
"${BUILD_CMD[@]}"
