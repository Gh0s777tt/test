# Benchmark Reproducibility Report: `timer_queue_benchmark`

**Baseline family**: `ci_repro_monitor_timer_queue_benchmark_20260210T143840Z`  
**Runs**: 2  
**Spread threshold**: 60.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 4
- Metrics above spread threshold: 0
- Median spread: 3.739%
- Max spread: 7.940%
- Median CV: 1.870%
- Max CV: 3.970%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| timer_queue_advance/advance_time_sparse_expiry | 386.466263 | 418.420972 | 402.443618 | 3.970 | 7.940 |
| timer_queue_set/set_timer_single | 393.134019 | 422.196159 | 407.665089 | 3.564 | 7.129 |
| timer_queue_set/set_timer_batch_255 | 11791.646407 | 11832.944768 | 11812.295587 | 0.175 | 0.350 |
| timer_queue_advance/advance_time_dense_expiry | 6980.130485 | 6967.166018 | 6973.648251 | 0.093 | 0.186 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
