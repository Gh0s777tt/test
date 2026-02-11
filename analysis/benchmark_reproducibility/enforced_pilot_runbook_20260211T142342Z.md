# Enforced Pilot Execution Runbook

**Generated at (UTC)**: 2026-02-11T14:23:42Z  
**Active mode**: `advisory`  
**Runbook stage**: `preflight`  
**Recommended action**: `promote_to_enforced_pilot`  
**Operator decision**: **go**  
**Rollback recommended**: **no**

## Inputs

- Promotion policy JSON: `governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json`
- Readiness JSON: `analysis/benchmark_reproducibility/governance_gate_promotion_readiness_20260211T142342Z.json`
- Breach route JSON: `analysis/benchmark_reproducibility/monitor_drift_breach_route_20260211T142342Z.json`
- Handoff JSON: `analysis/benchmark_reproducibility/monitor_drift_release_handoff_20260211T142341Z.json`
- Drill JSON: `analysis/benchmark_reproducibility/monitor_drift_release_readiness_drill_20260211T142342Z.json`

## Preflight Checklist

| Item | Required | Status | Details |
|---|---|---|---|
| `readiness_artifact_present` | yes | pass | analysis/benchmark_reproducibility/governance_gate_promotion_readiness_20260211T142342Z.json |
| `required_readiness_checks_passed` | yes | pass | all required checks passed |
| `readiness_ready` | yes | pass | overall_ready=yes |
| `readiness_go_signal` | yes | pass | pilot_go_no_go=go |
| `handoff_ready` | yes | pass | overall_status=ready |
| `drill_pass` | yes | pass | overall_status=pass |
| `ops_ack_token` | no | advisory | expected=PILOT-ACK, provided=no |

## Rollback Guardrails

| Guardrail | Value | Threshold | Breached |
|---|---:|---:|---|
| `consecutive_blocking_breach_routes` | 0 | 0 | no |
| `consecutive_blocked_handoffs` | 0 | 0 | no |
| `consecutive_failed_drills` | 0 | 0 | no |
| `consecutive_not_ready_assessments` | 0 | 0 | no |

- Latest breach detected: `no`
- Latest breach would block in active mode: `no`
- Latest breach sources: none

## Stage Runbook Actions

- Stage description: Validate readiness and evidence before entering enforced pilot mode.
- Confirm promotion readiness artifact reports overall_ready and pilot_go_no_go=go.
- Confirm latest release handoff is ready and latest drill status is pass.
- Verify rollback guardrail thresholds and required checks from policy.

## Decision Summary

- Preflight OK: `yes`
- Guardrails OK: `yes`
- Auto revert on breach: `yes`
- Recommended action: `promote_to_enforced_pilot`
