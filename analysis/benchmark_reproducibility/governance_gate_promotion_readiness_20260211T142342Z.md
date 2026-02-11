# Governance Gate Promotion Readiness Assessment

**Generated at (UTC)**: 2026-02-11T14:23:42Z  
**Policy active mode**: `advisory`  
**Readiness overall**: **ready**  
**Pilot decision**: **go**  
**Recommended next mode**: `enforced`

## Rolling Window Configuration

- Window days: 14
- Window start (UTC): 2026-01-28T14:23:42Z
- Min drill samples: 3
- Min drill pass rate (%): 95.00
- Max blocked handoff count: 0
- Max breach-detected count: 0

## Telemetry Summary

- Drill artifacts in window: 5 (pass=5, pass_rate=100.00%)
- Handoff artifacts in window: 6 (blocked=0)
- Breach route artifacts in window: 4 (breach_detected=0, would_block=0)
- Skipped drill artifacts (unparseable timestamp): 0
- Skipped handoff artifacts (unparseable timestamp): 0
- Skipped breach route artifacts (unparseable timestamp): 0

## Readiness Criteria

| Criterion | Value | Target | Passed |
|---|---:|---|---|
| `drill_sample_sufficient` | 5 | >= 3 | yes |
| `drill_pass_rate_ok` | 100.0 | >= 95.00 | yes |
| `blocked_handoff_limit_ok` | 0 | <= 0 | yes |
| `breach_detected_limit_ok` | 0 | <= 0 | yes |

## Enforced Pilot Rollout Checklist

| Checklist item | Required | Status | Details |
|---|---|---|---|
| `promotion_policy_loaded` | yes | pass | governance/performance/MONITOR_THRESHOLD_GOVERNANCE_GATE_PROMOTION.json |
| `enforced_controls_defined` | yes | pass | required keys: enforce_on_breach, require_pr_mitigation, require_breach_ack_token |
| `latest_breach_route_present` | yes | pass | analysis/benchmark_reproducibility/monitor_drift_breach_route_20260211T142342Z.json |
| `latest_handoff_present` | yes | pass | analysis/benchmark_reproducibility/monitor_drift_release_handoff_20260211T142341Z.json |
| `latest_drill_present` | yes | pass | analysis/benchmark_reproducibility/monitor_drift_release_readiness_drill_20260211T142342Z.json |
| `readiness_thresholds_met` | yes | pass | recommended_mode=enforced |
| `current_route_not_blocking` | yes | pass | would_block_in_active_mode=no |
| `current_handoff_not_blocked` | yes | pass | overall_status=ready |
| `operator_ack_token` | no | advisory | expected=PILOT-ACK, provided=no |

Use this assessment to decide whether to switch promotion `active_mode` to `enforced`.
