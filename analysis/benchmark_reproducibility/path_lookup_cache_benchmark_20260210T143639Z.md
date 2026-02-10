# Benchmark Reproducibility Report: `path_lookup_cache_benchmark`

**Baseline family**: `ci_repro_strict_path_lookup_cache_benchmark_20260210T143639Z`  
**Runs**: 2  
**Spread threshold**: 50.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 6
- Metrics above spread threshold: 0
- Median spread: 6.687%
- Max spread: 30.318%
- Median CV: 3.343%
- Max CV: 15.159%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| path_lookup_cache_syscalls/sys_rename_with_cache | 341.863856 | 251.860538 | 296.862197 | 15.159 | 30.318 |
| path_lookup_cache_syscalls/sys_stat_with_cache_hit | 62.725422 | 53.962164 | 58.343793 | 7.510 | 15.020 |
| path_lookup_cache_raw/insert_and_evict | 3117330.181765 | 2899880.702222 | 3008605.441993 | 3.614 | 7.228 |
| path_lookup_cache_syscalls/sys_unlink_with_cache | 165.653099 | 155.775895 | 160.714497 | 3.073 | 6.146 |
| path_lookup_cache_raw/hit | 45.392262 | 44.557513 | 44.974887 | 0.928 | 1.856 |
| path_lookup_cache_raw/miss | 1.078196 | 1.078141 | 1.078168 | 0.003 | 0.005 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
