# Monitor Threshold Changelog

This changelog records governance decisions for benchmark monitor threshold policy changes.

Each policy change entry should include:

1. Policy ID (`MONPOL-<NNN>`)
2. Date
3. Changed thresholds
4. Rationale
5. Evidence references
6. Reviewer/owner

Approved decisions must also have corresponding reviewer signoff metadata in:

- `governance/performance/MONPOL_SIGNOFFS.json`

---

## Entries

### MONPOL-001 (2026-02-10)

- **Scope**: Week 9 Day 7 monitor threshold recalibration.
- **Changes**:
  - `timer_queue_benchmark`: `25%` -> `60%`
  - `directory_entry_cache_benchmark`: retained at `25%`
  - strict threshold retained at `50%` for `path_lookup_cache_benchmark`
- **Rationale**:
  - Repeated shared-runner evidence showed timer sparse-expiry spread spikes
    causing non-actionable drift noise with a 25% threshold.
  - Raising timer monitor threshold reduced false positives while preserving
    signal in monitor-stage trend analysis.
- **Evidence**:
  - `analysis/benchmark_reproducibility/ci_benchmark_gate_summary_20260210T143639Z.md`
  - `analysis/benchmark_reproducibility/timer_queue_benchmark_20260210T143840Z.md`
  - `WEEK_9_DAY_7_MONITOR_NOISE_STABILIZATION_AND_THRESHOLD_RECALIBRATION.md`
- **Owner**: Benchmark governance automation

