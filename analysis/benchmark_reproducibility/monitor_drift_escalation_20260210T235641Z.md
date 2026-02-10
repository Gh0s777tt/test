# Monitor Drift Escalation Assessment

**Generated at (UTC)**: 2026-02-10T23:56:41Z  
**Dashboard source**: `analysis/benchmark_reproducibility/monitor_policy_dashboard_20260210T235640Z.json`  
**Recent runs window**: 3 (configured=5)

## Policy Thresholds

- watch drift rate (%): 20.0
- escalated drift rate (%): 40.0
- critical drift rate (%): 60.0
- escalated consecutive drift: 2
- critical consecutive drift: 3
- escalated failure rate (%): 10.0
- critical failure rate (%): 20.0

## Overall Escalation Status

- Overall level: **normal**
- Level counts: normal=2, watch=0, escalated=0, critical=0
- Escalated benches: 0
- Critical benches: 0

| Benchmark | Samples | Latest status | Drift rate (%) | Failure rate (%) | Drift in window | Failure in window | Consecutive drift | Level | Triggers | Required action |
|---|---:|---|---:|---:|---:|---:|---:|---|---|---|
| `directory_entry_cache_benchmark` | 2 | pass | 0.00 | 0.00 | 0 | 0 | 0 | normal | none | Routine monitoring. |
| `timer_queue_benchmark` | 2 | pass | 0.00 | 0.00 | 0 | 0 | 0 | normal | none | Routine monitoring. |

Assessment is advisory unless `--fail-on-level` is configured.
