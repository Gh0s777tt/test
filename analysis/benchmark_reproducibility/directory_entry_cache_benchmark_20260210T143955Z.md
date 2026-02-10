# Benchmark Reproducibility Report: `directory_entry_cache_benchmark`

**Baseline family**: `ci_repro_monitor_directory_entry_cache_benchmark_20260210T143955Z`  
**Runs**: 2  
**Spread threshold**: 25.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 7
- Metrics above spread threshold: 0
- Median spread: 0.353%
- Max spread: 15.860%
- Median CV: 0.177%
- Max CV: 7.930%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| directory_entry_cache_syscalls/sys_mkdir_with_cache | 148.735548 | 174.356544 | 161.546046 | 7.930 | 15.860 |
| directory_entry_cache_syscalls/sys_rmdir_with_cache | 217.796130 | 216.928686 | 217.362408 | 0.200 | 0.399 |
| directory_entry_cache_syscalls/sys_chdir_with_cache_hit | 108.045922 | 108.451125 | 108.248523 | 0.187 | 0.374 |
| directory_entry_cache_syscalls/sys_getcwd_with_cache | 31.278190 | 31.388827 | 31.333509 | 0.177 | 0.353 |
| directory_entry_cache_raw/hit | 38.678243 | 38.755160 | 38.716702 | 0.099 | 0.199 |
| directory_entry_cache_raw/insert_and_evict | 2951901.290588 | 2946343.870000 | 2949122.580294 | 0.094 | 0.188 |
| directory_entry_cache_raw/miss | 1.006391 | 1.007087 | 1.006739 | 0.035 | 0.069 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
