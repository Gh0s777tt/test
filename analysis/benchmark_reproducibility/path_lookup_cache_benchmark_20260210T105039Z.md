# Benchmark Reproducibility Report: `path_lookup_cache_benchmark`

**Baseline family**: `ci_repro_strict_path_lookup_cache_benchmark_20260210T105039Z`  
**Runs**: 2  
**Spread threshold**: 50.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 6
- Metrics above spread threshold: 0
- Median spread: 2.292%
- Max spread: 11.533%
- Median CV: 1.146%
- Max CV: 5.767%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| path_lookup_cache_raw/insert_and_evict | 3362932.888824 | 2996224.855882 | 3179578.872353 | 5.767 | 11.533 |
| path_lookup_cache_raw/miss | 1.154671 | 1.065347 | 1.110009 | 4.024 | 8.047 |
| path_lookup_cache_syscalls/sys_rename_with_cache | 280.340353 | 289.940170 | 285.140261 | 1.683 | 3.367 |
| path_lookup_cache_raw/hit | 47.720972 | 48.305638 | 48.013305 | 0.609 | 1.218 |
| path_lookup_cache_syscalls/sys_unlink_with_cache | 157.696326 | 158.817141 | 158.256734 | 0.354 | 0.708 |
| path_lookup_cache_syscalls/sys_stat_with_cache_hit | 52.787400 | 52.853159 | 52.820280 | 0.062 | 0.124 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
