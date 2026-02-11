#!/usr/bin/env bash
# Evaluate governance gate promotion readiness and enforced pilot checklist.
# Usage:
#   ./scripts/evaluate_governance_gate_promotion_readiness.sh
#   ./scripts/evaluate_governance_gate_promotion_readiness.sh --window-days 14 --fail-on-not-ready

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ANALYSIS_DIR="$ROOT/analysis/benchmark_reproducibility"
PROMOTION_POLICY_JSON="$ROOT/governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json"
BREACH_ROUTE_JSON=""
WINDOW_DAYS=""
PILOT_ACK_TOKEN=""
FAIL_ON_NOT_READY=0
OUTPUT_PATH=""
OUTPUT_JSON_PATH=""

usage() {
  cat <<'USAGE'
Usage: ./scripts/evaluate_governance_gate_promotion_readiness.sh [options]

Options:
  --analysis-dir <path>            Analysis directory (default: analysis/benchmark_reproducibility)
  --promotion-policy-json <path>   Governance gate promotion policy JSON path
  --breach-route-json <path>       Explicit breach route JSON artifact path
  --window-days <n>                Override readiness rolling window days
  --pilot-ack-token <token>        Optional operator ack token for pilot checklist
  --fail-on-not-ready              Exit non-zero if readiness thresholds are not met
  --output <path>                  Output markdown path
  --output-json <path>             Output JSON path
  -h, --help                       Show this help
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --analysis-dir)
      ANALYSIS_DIR="${2:-}"
      shift 2
      ;;
    --promotion-policy-json)
      PROMOTION_POLICY_JSON="${2:-}"
      shift 2
      ;;
    --breach-route-json)
      BREACH_ROUTE_JSON="${2:-}"
      shift 2
      ;;
    --window-days)
      WINDOW_DAYS="${2:-}"
      shift 2
      ;;
    --pilot-ack-token)
      PILOT_ACK_TOKEN="${2:-}"
      shift 2
      ;;
    --fail-on-not-ready)
      FAIL_ON_NOT_READY=1
      shift
      ;;
    --output)
      OUTPUT_PATH="${2:-}"
      shift 2
      ;;
    --output-json)
      OUTPUT_JSON_PATH="${2:-}"
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

if [[ ! -d "$ANALYSIS_DIR" ]]; then
  echo "Error: analysis directory not found: $ANALYSIS_DIR" >&2
  exit 1
fi

if [[ ! -f "$PROMOTION_POLICY_JSON" ]]; then
  echo "Error: promotion policy JSON not found: $PROMOTION_POLICY_JSON" >&2
  exit 1
fi

if [[ -n "$WINDOW_DAYS" ]]; then
  if ! [[ "$WINDOW_DAYS" =~ ^[0-9]+$ ]] || (( WINDOW_DAYS < 1 )); then
    echo "Error: --window-days must be integer >= 1" >&2
    exit 1
  fi
fi

if [[ -n "$BREACH_ROUTE_JSON" && ! -f "$BREACH_ROUTE_JSON" ]]; then
  echo "Error: breach route JSON not found: $BREACH_ROUTE_JSON" >&2
  exit 1
fi

if [[ -z "$OUTPUT_PATH" ]]; then
  TIMESTAMP="$(date -u +%Y%m%dT%H%M%SZ)"
  OUTPUT_PATH="$ANALYSIS_DIR/governance_gate_promotion_readiness_${TIMESTAMP}.md"
fi

if [[ -z "$OUTPUT_JSON_PATH" ]]; then
  OUTPUT_JSON_PATH="${OUTPUT_PATH%.md}.json"
fi

python3 - \
  "$ROOT" \
  "$ANALYSIS_DIR" \
  "$PROMOTION_POLICY_JSON" \
  "$BREACH_ROUTE_JSON" \
  "$WINDOW_DAYS" \
  "$PILOT_ACK_TOKEN" \
  "$FAIL_ON_NOT_READY" \
  "$OUTPUT_PATH" \
  "$OUTPUT_JSON_PATH" <<'PY'
import json
import re
import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path

root = Path(sys.argv[1])
analysis_dir = Path(sys.argv[2])
policy_path = Path(sys.argv[3])
explicit_breach_route_raw = sys.argv[4].strip()
window_days_arg = sys.argv[5].strip()
pilot_ack_token = sys.argv[6].strip()
fail_on_not_ready = bool(int(sys.argv[7]))
output_path = Path(sys.argv[8])
output_json_path = Path(sys.argv[9])


def rel(path: Path) -> str:
    try:
        return str(path.resolve().relative_to(root.resolve()))
    except ValueError:
        return str(path)


def load_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return {}


def parse_iso_utc(value: str):
    if not value:
        return None
    normalized = value.strip()
    if not normalized:
        return None
    if normalized.endswith("Z"):
        normalized = normalized[:-1] + "+00:00"
    try:
        parsed = datetime.fromisoformat(normalized)
    except ValueError:
        return None
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed.astimezone(timezone.utc)


def latest(glob: str):
    items = sorted(analysis_dir.glob(glob))
    return items[-1] if items else None


def yes_no(value: bool) -> str:
    return "yes" if value else "no"


def safe_int(value, default):
    try:
        return int(value)
    except (TypeError, ValueError):
        return default


def safe_float(value, default):
    try:
        return float(value)
    except (TypeError, ValueError):
        return default


def evaluate_records(paths, status_key: str):
    rows = []
    skipped = 0
    for path in paths:
        payload = load_json(path)
        generated_at = parse_iso_utc(str(payload.get("generated_at_utc", "")).strip())
        if generated_at is None:
            skipped += 1
            continue
        rows.append(
            {
                "path": path,
                "generated_at": generated_at,
                "status": str(payload.get(status_key, "n/a")),
                "payload": payload,
            }
        )
    rows.sort(key=lambda item: item["generated_at"])
    return rows, skipped


policy = load_json(policy_path)
readiness_cfg = policy.get("readiness_thresholds", {})
if not isinstance(readiness_cfg, dict):
    readiness_cfg = {}
pilot_cfg = policy.get("pilot_rollout", {})
if not isinstance(pilot_cfg, dict):
    pilot_cfg = {}

configured_window_days = safe_int(readiness_cfg.get("window_days"), 14)
window_days = safe_int(window_days_arg, configured_window_days) if window_days_arg else configured_window_days
window_days = max(1, window_days)

min_drill_samples = max(1, safe_int(readiness_cfg.get("min_drill_samples"), 3))
min_drill_pass_rate_pct = max(0.0, min(100.0, safe_float(readiness_cfg.get("min_drill_pass_rate_pct"), 95.0)))
max_blocked_handoff_count = max(0, safe_int(readiness_cfg.get("max_blocked_handoff_count"), 0))
max_breach_detected_count = max(0, safe_int(readiness_cfg.get("max_breach_detected_count"), 0))

active_mode = str(policy.get("active_mode", "advisory"))
if active_mode not in {"advisory", "enforced"}:
    active_mode = "advisory"

modes = policy.get("modes", {})
if not isinstance(modes, dict):
    modes = {}
enforced_cfg = modes.get("enforced", {})
if not isinstance(enforced_cfg, dict):
    enforced_cfg = {}
enforced_controls_defined = all(
    key in enforced_cfg
    for key in ("enforce_on_breach", "require_pr_mitigation", "require_breach_ack_token")
)

pilot_expected_ack = str(pilot_cfg.get("ops_ack_token", "PILOT-ACK")).strip() or "PILOT-ACK"
pilot_ack_present = bool(pilot_ack_token) and pilot_ack_token == pilot_expected_ack

now = datetime.now(timezone.utc)
window_start = now - timedelta(days=window_days)

drill_candidates = sorted(analysis_dir.glob("monitor_drift_release_readiness_drill_[0-9]*.json"))
handoff_candidates = sorted(analysis_dir.glob("monitor_drift_release_handoff_[0-9]*.json"))
breach_candidates = sorted(analysis_dir.glob("monitor_drift_breach_route_[0-9]*.json"))

drill_rows_all, drill_skipped = evaluate_records(drill_candidates, "overall_status")
handoff_rows_all, handoff_skipped = evaluate_records(handoff_candidates, "overall_status")
breach_rows_all, breach_skipped = evaluate_records(breach_candidates, "breach_detected")

drill_rows = [row for row in drill_rows_all if row["generated_at"] >= window_start]
handoff_rows = [row for row in handoff_rows_all if row["generated_at"] >= window_start]
breach_rows = [row for row in breach_rows_all if row["generated_at"] >= window_start]

drill_samples = len(drill_rows)
drill_pass_count = sum(1 for row in drill_rows if row["status"] == "pass")
drill_pass_rate_pct = (100.0 * drill_pass_count / drill_samples) if drill_samples else 0.0

blocked_handoff_count = sum(1 for row in handoff_rows if row["status"] == "blocked")

breach_detected_count = 0
blocking_breach_count = 0
for row in breach_rows:
    payload = row["payload"]
    breach_detected = bool(payload.get("breach_detected", False))
    promotion = payload.get("promotion", {})
    if not isinstance(promotion, dict):
        promotion = {}
    would_block = bool(promotion.get("would_block_in_active_mode", False))
    if breach_detected:
        breach_detected_count += 1
    if would_block:
        blocking_breach_count += 1

latest_breach_route = Path(explicit_breach_route_raw) if explicit_breach_route_raw else (breach_rows_all[-1]["path"] if breach_rows_all else None)
latest_handoff = handoff_rows_all[-1]["path"] if handoff_rows_all else None
latest_drill = drill_rows_all[-1]["path"] if drill_rows_all else None

latest_breach_payload = load_json(latest_breach_route) if latest_breach_route and latest_breach_route.exists() else {}
latest_handoff_payload = load_json(latest_handoff) if latest_handoff and latest_handoff.exists() else {}

latest_breach_route_not_blocking = True
if latest_breach_payload:
    promotion_payload = latest_breach_payload.get("promotion", {})
    if isinstance(promotion_payload, dict):
        latest_breach_route_not_blocking = not bool(promotion_payload.get("would_block_in_active_mode", False))

latest_handoff_not_blocked = True
if latest_handoff_payload:
    latest_handoff_not_blocked = str(latest_handoff_payload.get("overall_status", "n/a")) != "blocked"

criteria = [
    {
        "id": "drill_sample_sufficient",
        "description": f"Drill samples in rolling window >= {min_drill_samples}",
        "value": drill_samples,
        "target": f">= {min_drill_samples}",
        "passed": drill_samples >= min_drill_samples,
    },
    {
        "id": "drill_pass_rate_ok",
        "description": f"Drill pass rate (%) >= {min_drill_pass_rate_pct:.2f}",
        "value": round(drill_pass_rate_pct, 2),
        "target": f">= {min_drill_pass_rate_pct:.2f}",
        "passed": drill_pass_rate_pct >= min_drill_pass_rate_pct,
    },
    {
        "id": "blocked_handoff_limit_ok",
        "description": f"Blocked handoff count <= {max_blocked_handoff_count}",
        "value": blocked_handoff_count,
        "target": f"<= {max_blocked_handoff_count}",
        "passed": blocked_handoff_count <= max_blocked_handoff_count,
    },
    {
        "id": "breach_detected_limit_ok",
        "description": f"Breach-detected route count <= {max_breach_detected_count}",
        "value": breach_detected_count,
        "target": f"<= {max_breach_detected_count}",
        "passed": breach_detected_count <= max_breach_detected_count,
    },
]

overall_ready = all(item["passed"] for item in criteria)
recommended_mode = "enforced" if overall_ready else "advisory"

checklist = [
    {
        "id": "promotion_policy_loaded",
        "description": "Promotion policy JSON is readable and parsed.",
        "required": True,
        "status": "pass",
        "details": rel(policy_path),
    },
    {
        "id": "enforced_controls_defined",
        "description": "Enforced-mode governance controls are explicitly defined.",
        "required": True,
        "status": "pass" if enforced_controls_defined else "fail",
        "details": "required keys: enforce_on_breach, require_pr_mitigation, require_breach_ack_token",
    },
    {
        "id": "latest_breach_route_present",
        "description": "Latest canonical breach-route artifact is present.",
        "required": True,
        "status": "pass" if bool(latest_breach_route) else "fail",
        "details": rel(latest_breach_route) if latest_breach_route else "n/a",
    },
    {
        "id": "latest_handoff_present",
        "description": "Latest canonical handoff artifact is present.",
        "required": True,
        "status": "pass" if bool(latest_handoff) else "fail",
        "details": rel(latest_handoff) if latest_handoff else "n/a",
    },
    {
        "id": "latest_drill_present",
        "description": "Latest canonical drill artifact is present.",
        "required": True,
        "status": "pass" if bool(latest_drill) else "fail",
        "details": rel(latest_drill) if latest_drill else "n/a",
    },
    {
        "id": "readiness_thresholds_met",
        "description": "Promotion readiness thresholds are satisfied.",
        "required": True,
        "status": "pass" if overall_ready else "fail",
        "details": f"recommended_mode={recommended_mode}",
    },
    {
        "id": "current_route_not_blocking",
        "description": "Latest breach route does not indicate immediate blocking state.",
        "required": True,
        "status": "pass" if latest_breach_route_not_blocking else "fail",
        "details": f"would_block_in_active_mode={yes_no(not latest_breach_route_not_blocking)}",
    },
    {
        "id": "current_handoff_not_blocked",
        "description": "Latest handoff status is not blocked.",
        "required": True,
        "status": "pass" if latest_handoff_not_blocked else "fail",
        "details": f"overall_status={latest_handoff_payload.get('overall_status', 'n/a') if latest_handoff_payload else 'n/a'}",
    },
    {
        "id": "operator_ack_token",
        "description": "Operator pilot acknowledgment token supplied.",
        "required": False,
        "status": "pass" if pilot_ack_present else "advisory",
        "details": f"expected={pilot_expected_ack}, provided={'yes' if bool(pilot_ack_token) else 'no'}",
    },
]

required_failures = [item for item in checklist if item["required"] and item["status"] == "fail"]
pilot_go_no_go = "go" if not required_failures else "no-go"

generated_at = now.replace(microsecond=0).isoformat().replace("+00:00", "Z")
output_path.parent.mkdir(parents=True, exist_ok=True)
output_json_path.parent.mkdir(parents=True, exist_ok=True)

with output_path.open("w", encoding="utf-8") as fh:
    fh.write("# Governance Gate Promotion Readiness Assessment\n\n")
    fh.write(f"**Generated at (UTC)**: {generated_at}  \n")
    fh.write(f"**Policy active mode**: `{active_mode}`  \n")
    fh.write(f"**Readiness overall**: **{'ready' if overall_ready else 'not_ready'}**  \n")
    fh.write(f"**Pilot decision**: **{pilot_go_no_go}**  \n")
    fh.write(f"**Recommended next mode**: `{recommended_mode}`\n\n")

    fh.write("## Rolling Window Configuration\n\n")
    fh.write(f"- Window days: {window_days}\n")
    fh.write(f"- Window start (UTC): {window_start.replace(microsecond=0).isoformat().replace('+00:00', 'Z')}\n")
    fh.write(f"- Min drill samples: {min_drill_samples}\n")
    fh.write(f"- Min drill pass rate (%): {min_drill_pass_rate_pct:.2f}\n")
    fh.write(f"- Max blocked handoff count: {max_blocked_handoff_count}\n")
    fh.write(f"- Max breach-detected count: {max_breach_detected_count}\n\n")

    fh.write("## Telemetry Summary\n\n")
    fh.write(f"- Drill artifacts in window: {drill_samples} (pass={drill_pass_count}, pass_rate={drill_pass_rate_pct:.2f}%)\n")
    fh.write(f"- Handoff artifacts in window: {len(handoff_rows)} (blocked={blocked_handoff_count})\n")
    fh.write(f"- Breach route artifacts in window: {len(breach_rows)} (breach_detected={breach_detected_count}, would_block={blocking_breach_count})\n")
    fh.write(f"- Skipped drill artifacts (unparseable timestamp): {drill_skipped}\n")
    fh.write(f"- Skipped handoff artifacts (unparseable timestamp): {handoff_skipped}\n")
    fh.write(f"- Skipped breach route artifacts (unparseable timestamp): {breach_skipped}\n\n")

    fh.write("## Readiness Criteria\n\n")
    fh.write("| Criterion | Value | Target | Passed |\n")
    fh.write("|---|---:|---|---|\n")
    for item in criteria:
        fh.write(
            f"| `{item['id']}` | {item['value']} | {item['target']} | {'yes' if item['passed'] else 'no'} |\n"
        )
    fh.write("\n")

    fh.write("## Enforced Pilot Rollout Checklist\n\n")
    fh.write("| Checklist item | Required | Status | Details |\n")
    fh.write("|---|---|---|---|\n")
    for item in checklist:
        fh.write(
            f"| `{item['id']}` | {'yes' if item['required'] else 'no'} | {item['status']} | {item['details']} |\n"
        )
    fh.write("\n")

    if required_failures:
        fh.write("Blocking checklist items:\n")
        for item in required_failures:
            fh.write(f"- `{item['id']}`: {item['description']}\n")
        fh.write("\n")

    fh.write("Use this assessment to decide whether to switch promotion `active_mode` to `enforced`.\n")

payload = {
    "generated_at_utc": generated_at,
    "inputs": {
        "analysis_dir": rel(analysis_dir),
        "promotion_policy_json": rel(policy_path),
        "explicit_breach_route_json": rel(Path(explicit_breach_route_raw)) if explicit_breach_route_raw else "n/a",
        "window_days_override": window_days_arg or "n/a",
    },
    "policy": {
        "active_mode": active_mode,
        "enforced_controls_defined": enforced_controls_defined,
        "pilot_expected_ack_token": pilot_expected_ack,
    },
    "thresholds": {
        "window_days": window_days,
        "min_drill_samples": min_drill_samples,
        "min_drill_pass_rate_pct": min_drill_pass_rate_pct,
        "max_blocked_handoff_count": max_blocked_handoff_count,
        "max_breach_detected_count": max_breach_detected_count,
    },
    "telemetry_summary": {
        "window_start_utc": window_start.replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "drill_samples": drill_samples,
        "drill_pass_count": drill_pass_count,
        "drill_pass_rate_pct": round(drill_pass_rate_pct, 2),
        "handoff_samples": len(handoff_rows),
        "blocked_handoff_count": blocked_handoff_count,
        "breach_route_samples": len(breach_rows),
        "breach_detected_count": breach_detected_count,
        "breach_would_block_count": blocking_breach_count,
        "drill_skipped": drill_skipped,
        "handoff_skipped": handoff_skipped,
        "breach_skipped": breach_skipped,
    },
    "criteria": criteria,
    "overall_ready": overall_ready,
    "recommended_mode": recommended_mode,
    "pilot_checklist": checklist,
    "pilot_required_failures": required_failures,
    "pilot_go_no_go": pilot_go_no_go,
    "latest_artifacts": {
        "latest_breach_route_json": rel(latest_breach_route) if latest_breach_route else "n/a",
        "latest_handoff_json": rel(latest_handoff) if latest_handoff else "n/a",
        "latest_drill_json": rel(latest_drill) if latest_drill else "n/a",
    },
}
output_json_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")

print(f"Governance gate promotion readiness markdown: {output_path}")
print(f"Governance gate promotion readiness JSON: {output_json_path}")
print(f"Overall readiness: {'ready' if overall_ready else 'not_ready'}")
print(f"Pilot decision: {pilot_go_no_go}")
print(f"Recommended mode: {recommended_mode}")

if fail_on_not_ready and not overall_ready:
    print("Promotion readiness criteria not satisfied", file=sys.stderr)
    raise SystemExit(2)
PY

