# Week 7 Day 7: Performance Validation Report

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: Syscall and optimization-path benchmark validation

---

## 1) Benchmark execution status

| Suite | Command | Status |
|---|---|---|
| Baseline synthetic microbenchmarks | `cargo bench --bench performance_baseline` | PASS |
| Simple syscall microbenchmarks | `cargo bench --bench syscall_performance_simple` | PASS |
| Day 5 path cache benchmarks | `cargo bench --bench path_lookup_cache_benchmark` | PASS |
| Day 6 FD allocator benchmarks | `cargo bench --bench fd_allocator_benchmark` | PASS |
| Legacy scheduler/filesystem suites | `./scripts/run_benchmarks.sh` | PASS |
| Legacy IPC complete benchmark | `cargo bench --bench ipc_complete_benchmark` | FAIL (build-time mismatch) |

---

## 2) Actual benchmark highlights

### 2.1 Baseline and syscall timing snapshot

| Metric | Observed |
|---|---|
| `syscall_performance_simple/syscall_overhead` | ~48.49 ns |
| `file_operations/seek` | ~48.36 ns |
| `file_operations/stat` | ~48.23 ns |
| `directory_operations/chdir` | ~49.34 ns |
| `advanced_operations/dup` | ~60.43 ns |
| `advanced_operations/dup2` | ~58.44 ns |
| `timer_operations/resume_timer` | ~52.04 ns |

### 2.2 Day 5 path lookup cache metrics

| Metric | Observed |
|---|---|
| `path_lookup_cache_raw/hit` | ~44.66 ns |
| `path_lookup_cache_raw/miss` | ~1.07 ns |
| `path_lookup_cache_raw/insert_and_evict` | ~2.99 ms |
| `sys_stat_with_cache_hit` | ~48.13 ns |
| `sys_unlink_with_cache` | ~170.71 ns |
| `sys_rename_with_cache` | ~271.26 ns (high variance) |

### 2.3 Day 6 FD allocator metrics

| Metric | Observed |
|---|---|
| `fd_allocator_bitmap/sys_dup_single` | ~1.49-2.02 us |
| `fd_allocator_bitmap/sys_pipe_single` | ~1.40 us |
| `fd_allocator_bitmap/sys_dup2_rebind_existing` | ~1.46-1.60 us |
| `fd_allocator_bitmap/mass_dup_until_full` | ~42.80 us |

### 2.4 Scheduler/filesystem suite highlights

| Metric | Observed |
|---|---|
| `scheduling_decision/neural/100` | ~278.90 ns |
| `scheduling_decision/cfs/100` | ~736.90 ns |
| `context_switch/neural` | ~170.82 us |
| `context_switch/cfs` | ~400.49 us |
| `partition_switch/vantisfs_ab_switch` | ~6.16 us |
| `partition_switch/btrfs_snapshot` | ~121.97 us |

---

## 3) Comparison vs theoretical targets

Reference targets are taken from `PERFORMANCE_BASELINE_RESULTS.md`:

- syscall overhead `< 100ns`
- file operations `500ns - 5us`
- directory operations `500ns - 5us`
- advanced operations `500ns - 1us`
- timer operations `100ns - 1us`

| Category | Target | Observed | Assessment |
|---|---|---|---|
| Syscall overhead | `< 100ns` | ~48.49ns | PASS |
| File operations | `500ns - 5us` | ~48-52ns (simple benchmark) | Below range (synthetic) |
| Directory operations | `500ns - 5us` | ~48-52ns (simple benchmark) | Below range (synthetic) |
| Advanced operations | `500ns - 1us` | ~58-60ns (simple), ~1.4-2.0us (bitmap alloc bench) | Mixed |
| Timer operations | `100ns - 1us` | ~48-53ns (simple) | Below range (synthetic) |
| Path cache fast-path | N/A (new) | ~44-48ns hit path | Strong |
| FD allocator stress | N/A (new) | ~42.8us fill-to-limit | Expected for stress path |

---

## 4) Discrepancies identified

1. **Benchmark fidelity gap (synthetic microbenchmarks)**  
   Several baseline/simple benchmarks produce sub-target (very fast) nanosecond values because they simulate reduced logic paths and are heavily optimization-friendly.

2. **FD benchmark variance under batch setup**  
   `sys_dup_single` and `sys_dup2_rebind_existing` show broad confidence ranges due to per-iteration table setup and allocator state effects.

3. **Legacy IPC benchmark is stale and fails to build**  
   `ipc_complete_benchmark` references `vantis_os::ipc_complete::*` and unresolved symbols in current crate topology. This suite is currently non-actionable until migrated.

---

## 5) Updated optimization priorities (post-Day 7)

### Priority 0 (correctness/tooling)
1. Migrate `ipc_complete_benchmark` to current crate/module layout and make it runnable again.

### Priority 1 (measurement quality)
2. Harden synthetic syscall benchmarks to reduce compiler-optimization artifacts and better model real syscall work.
3. Add a stable benchmark profile/flags set for CI reproducibility.

### Priority 2 (performance follow-up)
4. Optimize FD allocator hot path overhead in `dup/dup2` benchmark scenario (reduce setup/cloning costs).
5. Reduce variance in path-cache rename benchmarks and evaluate O(1) LRU structures for heavy invalidation workloads.

---

## 6) Day 7 conclusion

Day 7 goals were completed:

- Benchmark suites executed for syscall-focused and optimization-focused paths.
- Theory vs actual comparison documented.
- Discrepancies identified with concrete root causes.
- Optimization priorities updated for next sprint steps.

