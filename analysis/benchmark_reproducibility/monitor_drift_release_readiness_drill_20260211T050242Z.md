# Monitor Drift Release Readiness Drill

**Generated at (UTC)**: 2026-02-11T05:02:42Z  
**Overall drill status**: **pass**  
**Transition pack required in handoff checks**: no

## Inputs

- Real escalation artifact: `analysis/benchmark_reproducibility/monitor_drift_escalation_20260211T050242Z.json`
- Simulated escalation artifact: `analysis/benchmark_reproducibility/monitor_drift_escalation_simulated_escalated_20260211T050242Z.json`
- Expected blocked strict exit code: `2`

## Scenarios

| Scenario | Expected exit | Actual exit | Status | Output markdown | Output json |
|---|---:|---:|---|---|---|
| `strict_baseline_real` | 0 | 0 | pass | `analysis/benchmark_reproducibility/monitor_drift_release_handoff_dryrun_baseline_20260211T050242Z.md` | `analysis/benchmark_reproducibility/monitor_drift_release_handoff_dryrun_baseline_20260211T050242Z.json` |
| `strict_blocked_simulated_escalated` | 2 | 2 | pass | `analysis/benchmark_reproducibility/monitor_drift_release_handoff_dryrun_blocked_20260211T050242Z.md` | `analysis/benchmark_reproducibility/monitor_drift_release_handoff_dryrun_blocked_20260211T050242Z.json` |

This drill is successful only when strict baseline passes and blocked strict simulation fails as expected.
