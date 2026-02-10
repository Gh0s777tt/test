# Benchmark Reproducibility Report: `directory_entry_cache_benchmark`

**Baseline family**: `ci_repro_monitor_directory_entry_cache_benchmark_20260210T112142Z`  
**Runs**: 2  
**Spread threshold**: 25.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 7
- Metrics above spread threshold: 0
- Median spread: 0.741%
- Max spread: 3.558%
- Median CV: 0.370%
- Max CV: 1.779%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| directory_entry_cache_raw/insert_and_evict | 2977521.528824 | 2873441.059444 | 2925481.294134 | 1.779 | 3.558 |
| directory_entry_cache_syscalls/sys_chdir_with_cache_hit | 109.034338 | 110.535939 | 109.785138 | 0.684 | 1.368 |
| directory_entry_cache_syscalls/sys_getcwd_with_cache | 31.401596 | 31.668819 | 31.535207 | 0.424 | 0.847 |
| directory_entry_cache_raw/hit | 38.889674 | 38.602708 | 38.746191 | 0.370 | 0.741 |
| directory_entry_cache_syscalls/sys_rmdir_with_cache | 231.756817 | 230.169811 | 230.963314 | 0.344 | 0.687 |
| directory_entry_cache_syscalls/sys_mkdir_with_cache | 148.085539 | 147.214477 | 147.650008 | 0.295 | 0.590 |
| directory_entry_cache_raw/miss | 1.008157 | 1.011268 | 1.009713 | 0.154 | 0.308 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
