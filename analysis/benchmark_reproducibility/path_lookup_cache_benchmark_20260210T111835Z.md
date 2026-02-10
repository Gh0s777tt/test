# Benchmark Reproducibility Report: `path_lookup_cache_benchmark`

**Baseline family**: `ci_repro_strict_path_lookup_cache_benchmark_20260210T111835Z`  
**Runs**: 2  
**Spread threshold**: 50.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 6
- Metrics above spread threshold: 0
- Median spread: 5.763%
- Max spread: 41.725%
- Median CV: 2.881%
- Max CV: 20.863%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| path_lookup_cache_syscalls/sys_stat_with_cache_hit | 85.394547 | 55.913766 | 70.654156 | 20.863 | 41.725 |
| path_lookup_cache_raw/hit | 57.361211 | 44.550331 | 50.955771 | 12.571 | 25.141 |
| path_lookup_cache_syscalls/sys_rename_with_cache | 265.477816 | 286.585494 | 276.031655 | 3.823 | 7.647 |
| path_lookup_cache_raw/insert_and_evict | 2975054.406471 | 3092715.104118 | 3033884.755294 | 1.939 | 3.878 |
| path_lookup_cache_syscalls/sys_unlink_with_cache | 161.776026 | 163.478468 | 162.627247 | 0.523 | 1.047 |
| path_lookup_cache_raw/miss | 1.064817 | 1.063965 | 1.064391 | 0.040 | 0.080 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
