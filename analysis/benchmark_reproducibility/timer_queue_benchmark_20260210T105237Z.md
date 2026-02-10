# Benchmark Reproducibility Report: `timer_queue_benchmark`

**Baseline family**: `ci_repro_monitor_timer_queue_benchmark_20260210T105237Z`  
**Runs**: 2  
**Spread threshold**: 25.00%  
**Criterion root**: `/workspace/src/verified/target/criterion`

---

## Summary

- Common benchmark metrics: 4
- Metrics above spread threshold: 0
- Median spread: 1.872%
- Max spread: 2.951%
- Median CV: 0.936%
- Max CV: 1.475%

## Top spread metrics

| Metric | run1 | run2 | mean | cv_pct | spread_pct |
|---|---|---|---|---|---|
| timer_queue_advance/advance_time_sparse_expiry | 352.634429 | 363.196042 | 357.915236 | 1.475 | 2.951 |
| timer_queue_set/set_timer_single | 404.807230 | 394.953935 | 399.880583 | 1.232 | 2.464 |
| timer_queue_advance/advance_time_dense_expiry | 7049.046493 | 6959.392370 | 7004.219431 | 0.640 | 1.280 |
| timer_queue_set/set_timer_batch_255 | 11783.506671 | 11843.391273 | 11813.448972 | 0.253 | 0.507 |

Note: Criterion point estimates are raw benchmark time units (typically ns).
