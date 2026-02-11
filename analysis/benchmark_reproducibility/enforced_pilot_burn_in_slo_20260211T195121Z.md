# Enforced Pilot Burn-In SLO Assessment

**Generated at (UTC)**: 2026-02-11T19:51:21Z  
**Overall status**: **pass**  
**Window artifacts**: 5  
**Samples analyzed**: 4  
**Safe run rate (%)**: 100.00

## Burn-In SLO Configuration

- min_samples: 1
- max_rollback_recommendations: 0
- max_guardrail_breach_events: 0
- max_preflight_failures: 0
- max_breach_detected_events: 0
- allowed_latest_runbook_stages: preflight, enforced_pilot
- require_latest_operator_decision_go: yes

## Telemetry Summary

- rollback_recommendations: 0
- guardrail_breach_events: 0
- preflight_failures: 0
- breach_detected_events: 0
- latest_stage: `preflight`
- latest_operator_decision: `go`

## Criteria

| Criterion | Value | Target | Passed |
|---|---:|---|---|
| `samples_sufficient` | 4 | >= 1 | yes |
| `rollback_recommendations_within_limit` | 0 | <= 0 | yes |
| `guardrail_breach_events_within_limit` | 0 | <= 0 | yes |
| `preflight_failures_within_limit` | 0 | <= 0 | yes |
| `breach_detected_events_within_limit` | 0 | <= 0 | yes |
| `latest_stage_allowed` | preflight | preflight,enforced_pilot | yes |
| `latest_operator_decision_go` | go | go | yes |

## Window Artifacts

- `analysis/benchmark_reproducibility/enforced_pilot_runbook_20260211T050242Z.json`
- `analysis/benchmark_reproducibility/enforced_pilot_runbook_20260211T142342Z.json`
- `analysis/benchmark_reproducibility/enforced_pilot_runbook_20260211T144459Z.json`
- `analysis/benchmark_reproducibility/enforced_pilot_runbook_20260211T195121Z.json`
