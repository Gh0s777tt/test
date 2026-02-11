# Monitor Drift Escalation Assessment

**Generated at (UTC)**: 2026-02-11T00:41:11Z  
**Dashboard source**: `analysis/benchmark_reproducibility/monitor_policy_dashboard_20260211T004110Z.json`  
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

- Response owner: Benchmark Governance Automation
- Backup owner: Performance Working Group
- Response SLA (hours): 168
- Drill cadence (days): 30
- Release handoff required: no
- Next drill due (UTC): 2026-03-13T00:41:11Z

| Benchmark | Samples | Latest status | Drift rate (%) | Failure rate (%) | Drift in window | Failure in window | Consecutive drift | Level | Owner | SLA (h) | Drill (d) | Handoff | Triggers | Required action |
|---|---:|---|---:|---:|---:|---:|---:|---|---|---:|---:|---|---|---|
| `directory_entry_cache_benchmark` | 2 | pass | 0.00 | 0.00 | 0 | 0 | 0 | normal | Benchmark Governance Automation | 168 | 30 | no | none | Routine monitoring. |
| `timer_queue_benchmark` | 2 | pass | 0.00 | 0.00 | 0 | 0 | 0 | normal | Benchmark Governance Automation | 168 | 30 | no | none | Routine monitoring. |

Assessment is advisory unless `--fail-on-level` is configured.
