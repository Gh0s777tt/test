# Benchmark Reproducibility Report: `path_lookup_cache_benchmark`

**Baseline family**: `ci_repro_path_lookup_cache_benchmark_20260210T101311Z`  
**Runs**: 2  
**Spread threshold**: 50.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 6
- Metrics above spread threshold: 0
- Median spread: 0.802%
- Max spread: 8.356%
- Median CV: 0.401%
- Max CV: 4.178%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| path_lookup_cache_syscalls/sys_rename_with_cache | 251.278081 | 273.191698 | 262.234889 | 4.178 | 8.356 |
| path_lookup_cache_syscalls/sys_unlink_with_cache | 159.083319 | 151.218258 | 155.150789 | 2.535 | 5.069 |
| path_lookup_cache_syscalls/sys_stat_with_cache_hit | 52.300416 | 52.727645 | 52.514031 | 0.407 | 0.814 |
| path_lookup_cache_raw/miss | 1.073133 | 1.064678 | 1.068906 | 0.395 | 0.791 |
| path_lookup_cache_raw/hit | 44.911408 | 44.639805 | 44.775606 | 0.303 | 0.607 |
| path_lookup_cache_raw/insert_and_evict | 2920558.213889 | 2911411.531111 | 2915984.872500 | 0.157 | 0.314 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
