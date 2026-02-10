# Monitor Threshold Change Proposal Draft: `MONPOL-002`

**Status**: Draft  
**Generated at (UTC)**: 2026-02-10T22:56:38Z  
**Proposal ID**: `MONPOL-002`  
**Changelog target**: `governance/performance/MONITOR_THRESHOLD_CHANGELOG.md`

## Inputs

- Recommendation snapshot: `analysis/benchmark_reproducibility/monitor_policy_recommendations_20260210T150630Z.json`
- Dashboard snapshot: `analysis/benchmark_reproducibility/monitor_policy_dashboard_20260210T225637Z.json`
- Benchmarks included: `directory_entry_cache_benchmark`, `timer_queue_benchmark`

## Proposed Threshold Review Table

| Benchmark | Current (%) | Recommended (%) | Delta (%) | Action | Drift reports | Drift rate (%) | Latest status | Failures | Timeouts | Latest report |
|---|---:|---:|---:|---|---:|---:|---|---:|---:|---|
| `directory_entry_cache_benchmark` | 25 | 18.24 | -6.76 | tighten | 0 | 0 | pass | 0 | 0 | `directory_entry_cache_benchmark_20260210T143955Z.md` |
| `timer_queue_benchmark` | 60 | 58.73 | -1.27 | hold | 2 | 50 | pass | 0 | 0 | `timer_queue_benchmark_20260210T143840Z.md` |

## Suggested CI Monitor Flags

```bash
--monitor-bench directory_entry_cache_benchmark:18.24
--monitor-bench timer_queue_benchmark:60
```

## Governance Checklist

- [ ] Confirm final decision for each benchmark (`tighten` / `hold` / `relax`).
- [ ] Update `.github/workflows/ci.yml` monitor thresholds.
- [ ] Update `docs/development/BENCHMARK_REPRODUCIBILITY_PROFILE.md`.
- [ ] Add `MONPOL-002` entry to `governance/performance/MONITOR_THRESHOLD_CHANGELOG.md`.
- [ ] Include `MONPOL-002` in PR title or body.
- [ ] Link evidence bundle below in PR description.

## Evidence Bundle Links

- `analysis/benchmark_reproducibility/monitor_policy_recommendations_20260210T150630Z.json`
- `analysis/benchmark_reproducibility/monitor_policy_dashboard_20260210T225637Z.json`
- `analysis/benchmark_reproducibility/ci_benchmark_gate_summary_20260210T143639Z.md`
- `analysis/benchmark_reproducibility/directory_entry_cache_benchmark_20260210T143955Z.md`
- `analysis/benchmark_reproducibility/timer_queue_benchmark_20260210T143840Z.md`

This draft is advisory and intended to accelerate governance-ready PR preparation.
