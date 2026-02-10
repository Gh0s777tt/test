# Benchmark Reproducibility Report: `timer_queue_benchmark`

**Baseline family**: `ci_repro_monitor_timer_queue_benchmark_20260210T112029Z`  
**Runs**: 2  
**Spread threshold**: 25.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 4
- Metrics above spread threshold: 1
- Median spread: 5.265%
- Max spread: 51.073%
- Median CV: 2.632%
- Max CV: 25.536%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| timer_queue_advance/advance_time_sparse_expiry | 330.379482 | 556.979188 | 443.679335 | 25.536 | 51.073 |
| timer_queue_set/set_timer_single | 410.800699 | 371.261618 | 391.031158 | 5.056 | 10.111 |
| timer_queue_advance/advance_time_dense_expiry | 6936.979188 | 6966.043712 | 6951.511450 | 0.209 | 0.418 |
| timer_queue_set/set_timer_batch_255 | 11830.136219 | 11797.597719 | 11813.866969 | 0.138 | 0.275 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
