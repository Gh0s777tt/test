# CI Benchmark Gate Summary

Generated reports: 3  
Monitor budget (s): 240

Case status counts: pass=3

## Case status

| Stage | Benchmark | Threshold (%) | Status | Duration (s) | Note |
|---|---|---:|---|---:|---|
| strict | `path_lookup_cache_benchmark` | 50 | pass | 121 |  |
| monitor | `timer_queue_benchmark` | 60 | pass | 75 |  |
| monitor | `directory_entry_cache_benchmark` | 25 | pass | 136 |  |

## Reproducibility metrics

| Stage | Report | Metrics | Above threshold | Median spread | Max spread |
|---|---|---:|---:|---:|---:|
| strict | `path_lookup_cache_benchmark_20260210T143639Z.md` | 6 | 0 | 6.687% | 30.318% |
| monitor | `timer_queue_benchmark_20260210T143840Z.md` | 4 | 0 | 3.739% | 7.940% |
| monitor | `directory_entry_cache_benchmark_20260210T143955Z.md` | 7 | 0 | 0.353% | 15.860% |
