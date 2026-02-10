# Week 9 Day 3: Synthetic Benchmark Fidelity Hardening Report

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: Hardening `syscall_performance_simple` to reduce optimizer artifacts and add mixed workloads

---

## 1) Objective

Day 3 goal was to improve synthetic benchmark fidelity by:

1. Reducing constant-folding/optimizer-friendly benchmark paths.
2. Switching from trivial mock paths to stateful and typed syscall module paths where possible.
3. Adding mixed syscall-sequence scenarios.
4. Updating guidance with observed drift against prior synthetic results.

---

## 2) Changes implemented

### 2.1 Reworked `benches/syscall_performance_simple.rs`

Key improvements:

- removed stale mock syscall enum and minimal constant-only paths,
- integrated real typed modules:
  - file-path cache operations (`sys_stat_with_cache`, `sys_unlink_with_cache`, `sys_rename_with_cache`),
  - directory operations with explicit cache state (`sys_*_with_cache`),
  - advanced FD operations (`sys_dup`, `sys_dup2`, `sys_pipe`, `sys_ioctl`),
  - timer lifecycle ops (`sys_set_timer`, `sys_pause_timer`, `sys_resume_timer`, `sys_get_timer_info`, `sys_cancel_timer`),
- syscall overhead benchmark now uses actual `SyscallHandler::dispatch` path (`GetPid`),
- used `iter_batched` to isolate setup/teardown and reduce cache-pollution artifacts from previous iterations.

### 2.2 Added mixed-scenario workloads

New benchmark group: `mixed_workloads`

- `file_dir_fd_sequence`
- `timer_sequence`

These intentionally chain multiple operation classes in one iteration to provide scenario-level visibility beyond micro-only numbers.

### 2.3 Cleanup in baseline benchmark file

Updated `benches/performance_baseline.rs` imports to remove unused warnings in top-level benchmark harness.

---

## 3) Validation executed

Commands:

```bash
cd src/verified
cargo bench --bench syscall_performance_simple -- --save-baseline syscall_simple_day9_hardened
cargo bench --bench performance_baseline -- --save-baseline performance_baseline_day9_hardened
```

Repository verification:

```bash
./scripts/verify_repo.sh
```

All commands passed.

---

## 4) Observed impact (Day 3 run)

Selected metrics from hardened `syscall_performance_simple`:

| Metric | Observed |
|---|---|
| `syscall_overhead` | ~1.52 ns |
| `file_operations/stat` | ~161-169 ns |
| `file_operations/unlink` | ~179-182 ns |
| `file_operations/rename` | ~264-268 ns |
| `directory_operations/rmdir` | ~605-706 ns |
| `advanced_operations/dup` | ~1.68-1.70 us |
| `advanced_operations/pipe` | ~1.71-1.73 us |
| `timer_operations/cancel_timer` | ~87-88 ns |
| `mixed_workloads/file_dir_fd_sequence` | ~2.53-2.55 us |
| `mixed_workloads/timer_sequence` | ~57.8-58.9 ns |

Compared to previous all-~48ns synthetic plateaus, results now show clearer operation-dependent cost tiers (ns to us), especially in file/dir/fd paths.

---

## 5) Remaining fidelity gaps

1. `syscall_overhead` remains very low because `GetPid` dispatch path is minimal.
2. Some operations still represent typed-module pathways, not full trap/ABI wiring.
3. Mixed workload coverage should be expanded with IPC and memory pressure sequences in subsequent iterations.

---

## 6) Outcome

Day 3 objectives were met:

- synthetic benchmark fidelity improved,
- mixed scenario workloads added,
- operation-cost differentiation improved,
- benchmark suite remains stable and integrated with repository verification.

Next step: continue Week 9-10 with deeper scenario-driven benchmark sets and CI trend policy hardening.

