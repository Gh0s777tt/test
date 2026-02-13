#!/usr/bin/env bash
# Validate ISO onboarding rollup gate policy JSON schema and threshold bounds.
# Usage:
#   ./scripts/validate_iso_onboarding_rollup_gate_policy.sh
#   ./scripts/validate_iso_onboarding_rollup_gate_policy.sh --require-profile ci_default
#   ./scripts/validate_iso_onboarding_rollup_gate_policy.sh --validate-workflow-options

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
POLICY_JSON="$ROOT/governance/performance/ISO_ONBOARDING_ROLLUP_GATE_POLICY.json"
WORKFLOW_YAML="$ROOT/.github/workflows/iso-onboarding-rollup-gate.yml"
REQUIRED_PROFILES=()
VALIDATE_WORKFLOW_OPTIONS=0

usage() {
  cat <<'USAGE'
Usage: ./scripts/validate_iso_onboarding_rollup_gate_policy.sh [options]

Options:
  --policy-json <path>             Policy JSON path (default: governance/performance/ISO_ONBOARDING_ROLLUP_GATE_POLICY.json)
  --require-profile <name>         Require profile to exist (repeatable)
  --workflow-yaml <path>           Workflow YAML path used for option sync check
  --validate-workflow-options      Validate workflow_dispatch.policy_profile options against policy profiles
  -h, --help                       Show this help
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
    --workflow-yaml)
      WORKFLOW_YAML="${2:-}"
      shift 2
      ;;
    --validate-workflow-options)
      VALIDATE_WORKFLOW_OPTIONS=1
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

python3 - "$POLICY_JSON" "$WORKFLOW_YAML" "$VALIDATE_WORKFLOW_OPTIONS" "${REQUIRED_PROFILES[@]}" <<'PY'
import json
import re
import sys
from pathlib import Path

policy_path = Path(sys.argv[1])
workflow_path = Path(sys.argv[2])
validate_workflow_options = sys.argv[3] == "1"
required_profiles = sys.argv[4:]

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
    "min_guard_cleared_ratio",
    "require_final_last_event",
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
    min_guard_cleared_ratio = profile["min_guard_cleared_ratio"]
    require_final_source = profile.get("require_final_source", "")
    require_final_last_event = profile.get("require_final_last_event", "")

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
    if not isinstance(min_guard_cleared_ratio, (int, float)) or not (0.0 <= float(min_guard_cleared_ratio) <= 1.0):
        raise SystemExit(
            f"Error: profile '{profile_name}' min_guard_cleared_ratio must be numeric in range 0.0..1.0"
        )
    if require_final_source is not None and not isinstance(require_final_source, str):
        raise SystemExit(f"Error: profile '{profile_name}' require_final_source must be string or null")
    if require_final_last_event is not None and not isinstance(require_final_last_event, str):
        raise SystemExit(f"Error: profile '{profile_name}' require_final_last_event must be string or null")

if validate_workflow_options:
    if not workflow_path.exists():
        raise SystemExit(f"Error: workflow YAML not found: {workflow_path}")

    workflow_text = workflow_path.read_text(encoding="utf-8")
    lines = workflow_text.splitlines()
    profile_idx = None
    profile_indent = None
    for idx, line in enumerate(lines):
        match = re.match(r"^(\s*)policy_profile:\s*$", line)
        if match:
            profile_idx = idx
            profile_indent = len(match.group(1))
            break
    if profile_idx is None:
        raise SystemExit("Error: workflow does not define workflow_dispatch input 'policy_profile'")

    options_idx = None
    options_indent = None
    for idx in range(profile_idx + 1, len(lines)):
        line = lines[idx]
        stripped = line.strip()
        if not stripped:
            continue
        indent = len(line) - len(line.lstrip(" "))
        if indent <= profile_indent:
            break
        if re.match(r"^\s*options:\s*$", line):
            options_idx = idx
            options_indent = indent
            break
    if options_idx is None:
        raise SystemExit("Error: workflow policy_profile input does not define options list")

    observed_options = []
    for idx in range(options_idx + 1, len(lines)):
        line = lines[idx]
        stripped = line.strip()
        if not stripped:
            continue
        indent = len(line) - len(line.lstrip(" "))
        if indent <= options_indent:
            break
        item_match = re.match(r"^\s*-\s*(.+?)\s*$", line)
        if not item_match:
            continue
        value = item_match.group(1).strip().strip("'\"")
        if value:
            observed_options.append(value)

    if not observed_options:
        raise SystemExit("Error: workflow policy_profile options list is empty")

    expected = sorted(profiles.keys())
    observed = sorted(dict.fromkeys(observed_options))
    if observed != expected:
        raise SystemExit(
            "Error: workflow policy_profile options do not match policy profiles; "
            f"expected={expected}, observed={observed}"
        )
    print(f"OK workflow_options_synced path={workflow_path}")

print(f"OK policy_valid path={policy_path}")
print(f"OK active_profile={active_profile}")
print(f"OK profiles={','.join(sorted(profiles.keys()))}")
PY
