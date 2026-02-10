# Monitor Threshold Change Proposal Template

Use this template for `MONPOL` governance proposals that update benchmark monitor thresholds.

---

## Metadata

- **Proposal ID**: `MONPOL-XXX`
- **Status**: Draft / Proposed / Approved / Rejected
- **Date (UTC)**: `YYYY-MM-DD`
- **Author / Owner**: `<name or team>`
- **Changelog target**: `governance/performance/MONITOR_THRESHOLD_CHANGELOG.md`

---

## Scope

Describe which monitor benchmarks are affected and why this proposal is needed.

---

## Proposed Threshold Review Table

| Benchmark | Current (%) | Recommended (%) | Delta (%) | Action | Drift reports | Drift rate (%) | Latest status | Latest report |
|---|---:|---:|---:|---|---:|---:|---|---|
| `example_benchmark` | 25 | 30 | 5 | relax | 2 | 40.0 | pass | `analysis/benchmark_reproducibility/example.md` |

---

## Suggested CI Monitor Flags

```bash
--monitor-bench benchmark_name:threshold
```

---

## Evidence Bundle

- `analysis/benchmark_reproducibility/monitor_policy_recommendations_<timestamp>.md`
- `analysis/benchmark_reproducibility/monitor_policy_recommendations_<timestamp>.json`
- `analysis/benchmark_reproducibility/monitor_policy_dashboard_<timestamp>.md`
- `analysis/benchmark_reproducibility/monitor_policy_dashboard_<timestamp>.json`
- latest `ci_benchmark_gate_summary_<timestamp>.md`
- benchmark report(s) referenced by proposal

---

## Rationale

Summarize:

1. observed drift/failure behavior,
2. recommendation outputs,
3. expected impact of threshold change.

---

## Risk Assessment

- **False positives impact**:
- **False negatives impact**:
- **Rollback approach**:
- **Monitoring plan after merge**:

---

## Governance Checklist

- [ ] `MONPOL-XXX` is included in PR title/body.
- [ ] `.github/workflows/ci.yml` monitor thresholds updated.
- [ ] `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md` updated.
- [ ] `governance/performance/MONITOR_THRESHOLD_CHANGELOG.md` updated with final decision.
- [ ] Evidence bundle links included and valid.

