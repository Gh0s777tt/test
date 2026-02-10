# Monitor Policy Recommendations (Rolling Evidence)

**Source directory**: `/workspace/analysis/benchmark_reproducibility`  
**Lookback per benchmark**: 6  
**Minimum samples**: 2  
**Headroom**: 15.00%  
**Clamp range**: [5.00%, 80.00%]

**Bench filter**: `directory_entry_cache_benchmark`, `timer_queue_benchmark`

Recommendation heuristic: `max(p90(max_spread), p75(median_spread)) * (1 + headroom)` with clamp bounds.

| Benchmark | Samples | Latest threshold (%) | Drift reports | Drift rate (%) | p90 max spread (%) | p75 median spread (%) | Peak max spread (%) | Recommended (%) | Action | Latest report |
|---|---|---|---|---|---|---|---|---|---|---|
| `directory_entry_cache_benchmark` | 2 | 25.00 | 0 | 0.00 | 15.860 | 0.741 | 15.860 | 18.24 | tighten | `directory_entry_cache_benchmark_20260210T143955Z.md` |
| `timer_queue_benchmark` | 4 | 60.00 | 2 | 50.00 | 51.073 | 3.739 | 51.073 | 58.73 | hold | `timer_queue_benchmark_20260210T143840Z.md` |

This report is advisory; CI thresholds should be updated intentionally and reviewed.
