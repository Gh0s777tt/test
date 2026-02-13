#!/usr/bin/env bash
# Validate ISO onboarding rollup gate policy JSON schema and threshold bounds.
# Usage:
#   ./scripts/validate_iso_onboarding_rollup_gate_policy.sh
#   ./scripts/validate_iso_onboarding_rollup_gate_policy.sh --require-profile ci_default

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
POLICY_JSON="$ROOT/governance/performance/ISO_ONBOARDING_ROLLUP_GATE_POLICY.json"
REQUIRED_PROFILES=()

usage() {
  cat <<'USAGE'
Usage: ./scripts/validate_iso_onboarding_rollup_gate_policy.sh [options]

Options:
  --policy-json <path>       Policy JSON path (default: governance/performance/ISO_ONBOARDING_ROLLUP_GATE_POLICY.json)
  --require-profile <name>   Require profile to exist (repeatable)
  -h, --help                 Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --policy-json)
      POLICY_JSON="${2:-}"
      shift 2
      ;;
    --require-profile)
      REQUIRED_PROFILES+=("${2:-}")
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

python3 - "$POLICY_JSON" "${REQUIRED_PROFILES[@]}" <<'PY'
import json
import sys
from pathlib import Path

policy_path = Path(sys.argv[1])
required_profiles = sys.argv[2:]

if not policy_path.exists():
    raise SystemExit(f"Error: policy file not found: {policy_path}")

try:
    payload = json.loads(policy_path.read_text(encoding="utf-8"))
except Exception as exc:
    raise SystemExit(f"Error: failed to parse policy file {policy_path}: {exc}") from exc

if not isinstance(payload, dict):
    raise SystemExit("Error: policy root must be a JSON object")

version = payload.get("version")
if not isinstance(version, int) or version < 1:
    raise SystemExit("Error: policy field 'version' must be integer >= 1")

active_profile = payload.get("active_profile")
if not isinstance(active_profile, str) or not active_profile:
    raise SystemExit("Error: policy field 'active_profile' must be non-empty string")

profiles = payload.get("profiles")
if not isinstance(profiles, dict) or not profiles:
    raise SystemExit("Error: policy field 'profiles' must be a non-empty object")

if active_profile not in profiles:
    raise SystemExit(f"Error: active_profile '{active_profile}' is not defined in profiles")

for required in required_profiles:
    if required not in profiles:
        raise SystemExit(f"Error: required profile missing: {required}")

required_fields = [
    "qemu_timeout_seconds",
    "installer_timeout_seconds",
    "window",
    "max_lockout_ratio",
    "max_mean_failures",
]

for profile_name, profile in profiles.items():
    if not isinstance(profile, dict):
        raise SystemExit(f"Error: profile '{profile_name}' must be an object")
    for field in required_fields:
        if field not in profile:
            raise SystemExit(f"Error: profile '{profile_name}' missing field '{field}'")

    qemu_timeout = profile["qemu_timeout_seconds"]
    installer_timeout = profile["installer_timeout_seconds"]
    window = profile["window"]
    max_lockout_ratio = profile["max_lockout_ratio"]
    max_mean_failures = profile["max_mean_failures"]
    require_final_source = profile.get("require_final_source", "")

    if not isinstance(qemu_timeout, int) or qemu_timeout < 5:
        raise SystemExit(f"Error: profile '{profile_name}' qemu_timeout_seconds must be integer >= 5")
    if not isinstance(installer_timeout, int) or installer_timeout < 20:
        raise SystemExit(f"Error: profile '{profile_name}' installer_timeout_seconds must be integer >= 20")
    if not isinstance(window, int) or window < 1:
        raise SystemExit(f"Error: profile '{profile_name}' window must be integer >= 1")
    if not isinstance(max_lockout_ratio, (int, float)) or not (0.0 <= float(max_lockout_ratio) <= 1.0):
        raise SystemExit(f"Error: profile '{profile_name}' max_lockout_ratio must be numeric in range 0.0..1.0")
    if not isinstance(max_mean_failures, (int, float)) or float(max_mean_failures) < 0.0:
        raise SystemExit(f"Error: profile '{profile_name}' max_mean_failures must be numeric >= 0.0")
    if require_final_source is not None and not isinstance(require_final_source, str):
        raise SystemExit(f"Error: profile '{profile_name}' require_final_source must be string or null")

print(f"OK policy_valid path={policy_path}")
print(f"OK active_profile={active_profile}")
print(f"OK profiles={','.join(sorted(profiles.keys()))}")
PY
