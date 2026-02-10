# Week 7 Day 13: Consolidated Performance Report

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: Consolidated benchmark results for Days 5-12 optimizations and syscall path validation

---

## 1) Goal and method

Day 13 objective was to consolidate performance results across completed optimization work:

- Day 5: Path lookup cache
- Day 6: FD allocation optimization
- Day 11: Directory entry cache
- Day 12: Timer queue optimization

Data sources used:

1. `WEEK_7_DAY_7_PERFORMANCE_VALIDATION.md`
2. `WEEK_7_DAY_11_DIRECTORY_ENTRY_CACHING.md`
3. `WEEK_7_DAY_12_TIMER_QUEUE_OPTIMIZATION.md`
4. Fresh Day 13 benchmark refresh runs in `src/verified`:
   - `cargo bench --bench syscall_performance_simple -- --save-baseline syscall_simple_day13`
   - `cargo bench --bench path_lookup_cache_benchmark -- --save-baseline path_lookup_cache_day13`
   - `cargo bench --bench fd_allocator_benchmark -- --save-baseline fd_allocator_day13`

---

## 2) Consolidated benchmark snapshot

### 2.1 Syscall-simple refresh (Day 13)

Selected metrics:

| Metric | Observed |
|---|---|
| `syscall_overhead` | ~48.79-49.90 ns |
| `file_operations/seek` | ~48.45-49.25 ns |
| `directory_operations/chdir` | ~48.23-48.38 ns |
| `advanced_operations/dup` | ~48.20-48.21 ns |
| `timer_operations/resume_timer` | ~48.42-48.67 ns |

Interpretation:

- This suite remains synthetic and strongly optimization-friendly.
- It is useful for trend detection, not for end-to-end workload latency estimation.

### 2.2 Path lookup cache (Day 13 refresh)

| Metric | Observed |
|---|---|
| `path_lookup_cache_raw/hit` | ~44.34-44.72 ns |
| `path_lookup_cache_raw/miss` | ~1.06-1.07 ns |
| `path_lookup_cache_raw/insert_and_evict` | ~2.98 ms |
| `sys_stat_with_cache_hit` | ~52.22-52.32 ns |
| `sys_unlink_with_cache` | ~172.42-174.83 ns |
| `sys_rename_with_cache` | ~271.26-284.63 ns |

Interpretation:

- Hit/miss fast path remains stable and strong.
- Rename path still shows variance under benchmark setup.

### 2.3 FD allocator benchmark (Day 13 refresh)

| Metric | Observed |
|---|---|
| `fd_allocator_bitmap/sys_dup_single` | ~1.40-1.42 us |
| `fd_allocator_bitmap/sys_pipe_single` | ~1.47-1.48 us |
| `fd_allocator_bitmap/sys_dup2_rebind_existing` | ~1.47-1.49 us |
| `fd_allocator_bitmap/mass_dup_until_full` | ~42.42-42.62 us |

Interpretation:

- FD allocator remains in expected microsecond envelope for realistic setup path.
- Stress behavior is stable near previously measured values.

### 2.4 Directory entry cache (Day 11)

| Metric | Observed |
|---|---|
| `directory_entry_cache_raw/hit` | ~38.66-39.10 ns |
| `sys_chdir_with_cache_hit` | ~108.58-108.95 ns |
| `sys_getcwd_with_cache` | ~31.32-31.36 ns |
| `sys_mkdir_with_cache` | ~146.23-146.48 ns |
| `sys_rmdir_with_cache` | ~221.83-222.95 ns |

### 2.5 Timer queue benchmark (Day 12)

| Metric | Observed |
|---|---|
| `timer_queue_set/set_timer_single` | ~476-498 ns |
| `timer_queue_set/set_timer_batch_255` | ~11.85-11.89 us |
| `timer_queue_advance/advance_time_sparse_expiry` | ~470-500 ns |
| `timer_queue_advance/advance_time_dense_expiry` | ~7.06-7.10 us |

---

## 3) Before/after trend highlights

Using Day 7 report as baseline reference for dedicated optimization suites:

| Optimization area | Day 7 reference | Day 13 refresh | Trend |
|---|---|---|---|
| FD `sys_dup_single` | ~1.49-2.02 us | ~1.40-1.42 us | Improved |
| FD `mass_dup_until_full` | ~42.80 us | ~42.42-42.62 us | Stable/slightly improved |
| Path cache raw hit | ~44.66 ns | ~44.34-44.72 ns | Stable |
| Path `sys_unlink_with_cache` | ~170.71 ns | ~172.42-174.83 ns | Slight regression/noise |
| Path `sys_rename_with_cache` | ~271.26 ns | ~271.26-284.63 ns | Variance persists |

Key summary:

1. FD allocator optimization remains beneficial and stable.
2. Path cache still provides expected hit-path acceleration.
3. Rename path variance remains a measurable hot spot.
4. Directory and timer queue optimizations introduced new benchmark visibility with acceptable latency envelopes.

---

## 4) Optimization techniques documented

1. **Path metadata caching** (`PathLookupCache`, LRU)
   - Cache hit/miss accounting and invalidation hooks.
2. **Bitmap-based FD allocation**
   - Deterministic lowest-available descriptor behavior.
3. **Directory entry cache**
   - Coherent path-based cache for directory-centric operations.
4. **Min-heap timer queue**
   - Deadline-priority scheduling with epoch-based stale-entry invalidation.
5. **Mode-driven benchmark runner**
   - `scripts/run_benchmarks.sh --syscall|--all` for structured execution.

---

## 5) Performance usage guide (for contributors)

Recommended quick workflow:

```bash
# 1) Validate build/health
./scripts/verify_repo.sh

# 2) Run syscall-focused benchmark suite
./scripts/run_benchmarks.sh --syscall

# 3) Run targeted optimization suites when needed
cd src/verified
cargo bench --bench path_lookup_cache_benchmark
cargo bench --bench fd_allocator_benchmark
cargo bench --bench directory_entry_cache_benchmark
cargo bench --bench timer_queue_benchmark
```

Interpretation guidance:

1. Compare medians and confidence intervals, not single runs.
2. Treat nanosecond synthetic paths as trend indicators only.
3. Use dedicated scenario benches (`fd_allocator`, `timer_queue`) for realism.
4. Track variance outliers to prioritize benchmark hardening work.

---

## 6) Remaining risks and recommendations

### 6.1 Current risks

1. Synthetic benchmark artifacts remain significant in `syscall_performance_simple`.
2. Legacy `ipc_complete_benchmark` migration is still pending.
3. Some benchmark files still emit warnings (`unused import`, `dead_code`) and should be cleaned.

### 6.2 Recommended next actions

1. Add a strict benchmark profile for CI reproducibility (fixed CPU governor guidance and baseline policy).
2. Migrate/replace stale IPC benchmark to current module topology.
3. Add scenario-level benchmarks with realistic mixed syscall sequences.
4. Reduce rename-path variance by profiling cache invalidation and path normalization cost.

---

## 7) Day 13 conclusion

Day 13 deliverables are complete:

- Consolidated benchmark report prepared.
- Before/after trends summarized.
- Optimization techniques documented.
- Practical performance usage guidance added.
- Follow-up recommendations captured for Day 14 and beyond.

