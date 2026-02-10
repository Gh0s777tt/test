# CI Benchmark Gate Summary

Generated reports: 3  
Monitor budget (s): 240

## Case status

| Stage | Benchmark | Threshold (%) | Status | Duration (s) | Note |
|---|---|---:|---|---:|---|
| strict | `path_lookup_cache_benchmark` | 50 | pass | 114 |  |
| monitor | `timer_queue_benchmark` | 25 | pass | 73 |  |
| monitor | `directory_entry_cache_benchmark` | 25 | pass | 139 |  |

## Reproducibility metrics

| Stage | Report | Metrics | Above threshold | Median spread | Max spread |
|---|---|---:|---:|---:|---:|
| strict | `path_lookup_cache_benchmark_20260210T111835Z.md` | 6 | 0 | 5.763% | 41.725% |
| monitor | `timer_queue_benchmark_20260210T112029Z.md` | 4 | 1 | 5.265% | 51.073% |
| monitor | `directory_entry_cache_benchmark_20260210T112142Z.md` | 7 | 0 | 0.741% | 3.558% |
