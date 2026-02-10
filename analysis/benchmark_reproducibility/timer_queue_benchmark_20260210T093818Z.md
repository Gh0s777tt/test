# Benchmark Reproducibility Report: `timer_queue_benchmark`

**Baseline family**: `repro_timer_queue_benchmark_20260210T093818Z`  
**Runs**: 2  
**Spread threshold**: 5.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 4
- Metrics above spread threshold: 1
- Median spread: 3.632%
- Max spread: 7.814%
- Median CV: 1.816%
- Max CV: 3.907%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| timer_queue_set/set_timer_single | 372.894719 | 403.215970 | 388.055344 | 3.907 | 7.814 |
| timer_queue_advance/advance_time_sparse_expiry | 398.780516 | 379.651547 | 389.216032 | 2.457 | 4.915 |
| timer_queue_advance/advance_time_dense_expiry | 7042.184648 | 7209.579421 | 7125.882035 | 1.175 | 2.349 |
| timer_queue_set/set_timer_batch_255 | 11766.646861 | 11832.401739 | 11799.524300 | 0.279 | 0.557 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
