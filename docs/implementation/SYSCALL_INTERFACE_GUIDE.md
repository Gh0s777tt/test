# VantisOS Syscall Interface Guide

Implementation-focused guide for the verified syscall surface used in `src/verified`.

**Date**: 2026-02-10  
**Scope**: Week 7-8 Day 8 deliverable  
**Primary audience**: kernel and userspace runtime contributors

---

## 1) What this guide covers

This document is the practical companion to `SYSCALL_INTERFACE_SPECIFICATION.md`.
It focuses on:

1. The current syscall catalog (all 39 syscall numbers).
2. Which syscalls are wired through `SyscallHandler::dispatch`.
3. Which syscalls are currently used via typed module APIs.
4. Usage examples from current code.
5. Day 7 performance characteristics.
6. Best practices and troubleshooting.

Primary code references:

- `src/verified/syscall.rs` (numbers, validator, trap dispatcher)
- `src/verified/syscall_file_ops.rs`
- `src/verified/syscall_dir_ops.rs`
- `src/verified/syscall_advanced_ops.rs`
- `src/verified/syscall_time_ops.rs`
- `WEEK_7_DAY_7_PERFORMANCE_VALIDATION.md`

---

## 2) Interface model and current integration state

There are two practical call paths today:

1. **Trap-style dispatcher path**
   - Entry: `SyscallHandler::dispatch(&SyscallContext)`
   - Types: `SyscallNumber`, `SyscallArgs`, `SyscallError`, `SyscallResult`
   - Best for low-level ABI/trap simulation and validation.

2. **Typed direct-call path**
   - Entry: module-level functions in `syscall_*_ops.rs`
   - Best for tests, benchmarks, and internal integration where richer typed APIs are needed.

Important current behavior:

- Some syscall numbers exist in `SyscallNumber` but return `InvalidSyscall` in dispatcher.
- Several dispatcher handlers are intentionally stubbed and return `InvalidState`.
- Extended filesystem/directory/fd/timer operations are implemented in dedicated modules and are currently consumed directly.

---

## 3) Complete syscall catalog (39)

Status legend:

- **Dispatch**: handled in `SyscallHandler::dispatch`
- **Stub**: dispatch handler exists but returns `InvalidState`
- **Direct module**: implemented via typed API in dedicated module
- **Number only**: number defined, dispatch currently returns `InvalidSyscall`

### 3.1 Process management (6)

| No. | Name | Primary interface | Return | Typical errors | Status | Notes |
|---|---|---|---|---|---|---|
| 0 | Exit | `sys_exit(context)` | `Ok(exit_code)` | - | Dispatch | `arg0` interpreted as `i32` exit code. |
| 1 | Fork | `sys_fork(context)` | - | `InvalidState` | Stub | Placeholder for process-manager integration. |
| 2 | Exec | `sys_exec(context)` | - | `InvalidState` | Stub | Placeholder for executable loader path. |
| 3 | Wait | `sys_wait(context)` | - | `InvalidState` | Stub | Placeholder for child wait semantics. |
| 4 | GetPid | `sys_getpid(context)` | caller pid as `i64` | - | Dispatch | Uses `context.caller_pid()`. |
| 5 | GetParentPid | `sys_getppid(context)` | `Ok(0)` | - | Dispatch | Placeholder parent lookup. |

### 3.2 Memory management (4)

| No. | Name | Primary interface | Return | Typical errors | Status | Notes |
|---|---|---|---|---|---|---|
| 10 | Allocate | `sys_allocate(context)` | order as `i64` | `InvalidParameter` | Dispatch | `arg0` validated by `validate_order`. |
| 11 | Deallocate | `sys_deallocate(context)` | `Ok(0)` | `InvalidParameter` | Dispatch | `arg0=addr`, `arg1=order`, pointer + bounds validated. |
| 12 | MapMemory | `sys_map_memory(context)` | - | `InvalidState` | Stub | Placeholder for VM mapper integration. |
| 13 | UnmapMemory | `sys_unmap_memory(context)` | - | `InvalidState` | Stub | Placeholder for VM unmapper integration. |

### 3.3 IPC and capability operations (4)

| No. | Name | Primary interface | Return | Typical errors | Status | Notes |
|---|---|---|---|---|---|---|
| 20 | Send | `sys_send(context)` | receiver pid as `i64` | `InvalidParameter` | Dispatch | `arg0=pid`, `arg1=ptr`, `arg2=len`, `arg3=priority`; len max 4096. |
| 21 | Receive | `sys_receive(context)` | `Ok(0)` | - | Dispatch | Placeholder receive path. |
| 22 | GrantCapability | `sys_grant_capability(context)` | target pid as `i64` | `InvalidParameter` | Dispatch | `arg0=pid`, `arg1=capability`. |
| 23 | RevokeCapability | `sys_revoke_capability(context)` | source pid as `i64` | `InvalidParameter` | Dispatch | `arg0=pid`, `arg1=capability`. |

### 3.4 File operations (9)

| No. | Name | Primary interface | Return | Typical errors | Status | Notes |
|---|---|---|---|---|---|---|
| 30 | Open | `sys_open(context)` | - | `InvalidState` | Stub | Number reserved; runtime open path not wired in dispatcher yet. |
| 31 | Close | `sys_close(context)` | - | `InvalidState` | Stub | Number reserved; runtime close path not wired in dispatcher yet. |
| 32 | Read | `sys_read(context)` | - | `InvalidState` | Stub | Number reserved; runtime read path not wired in dispatcher yet. |
| 33 | Write | `sys_write(context)` | - | `InvalidState` | Stub | Number reserved; runtime write path not wired in dispatcher yet. |
| 34 | Seek | `sys_seek(file_table, fd, offset, origin)` | `FileOpResult<u64>` | `InvalidFd`, `InvalidOffset` | Direct module | Typed API in `syscall_file_ops.rs`. |
| 35 | Stat | `sys_stat(path)` | `FileOpResult<FileStat>` | `InvalidPath` | Direct module | Path lookup cache integrated. |
| 36 | Fstat | `sys_fstat(file_table, fd)` | `FileOpResult<FileStat>` | `InvalidFd` | Direct module | Converts table entry to metadata. |
| 37 | Unlink | `sys_unlink(path)` | `FileOpResult<()>` | `InvalidPath` | Direct module | Invalidates matching cache entry. |
| 38 | Rename | `sys_rename(old_path, new_path)` | `FileOpResult<()>` | `InvalidPath` | Direct module | Moves/invalidate cached metadata coherently. |

### 3.5 Time basic (2)

| No. | Name | Primary interface | Return | Typical errors | Status | Notes |
|---|---|---|---|---|---|---|
| 40 | GetTime | `sys_gettime(context)` | `Ok(1704902400)` | - | Dispatch | Current implementation returns a fixed mock timestamp. |
| 41 | Sleep | `sys_sleep(context)` | `Ok(0)` | `InvalidParameter` | Dispatch | `arg0` ms duration, must be `1..=3_600_000`. |

### 3.6 Directory operations (4)

| No. | Name | Primary interface | Return | Typical errors | Status | Notes |
|---|---|---|---|---|---|---|
| 50 | Mkdir | `sys_mkdir(path, mode)` | `DirOpResult<()>` | `InvalidPath`, `PathTooLong` | Direct module | Invalidates created path and parent cache entry. |
| 51 | Rmdir | `sys_rmdir(path)` | `DirOpResult<()>` | `InvalidPath`, `PathTooLong`, `PermissionDenied` | Direct module | Rejects `/`, invalidates subtree cache prefix. |
| 52 | Chdir | `sys_chdir(wd, path)` | `DirOpResult<()>` | `InvalidPath`, `PathTooLong` | Direct module | Supports relative/absolute normalization. |
| 53 | Getcwd | `sys_getcwd(wd, buf, size)` | `DirOpResult<usize>` | `PathTooLong` | Direct module | Returns copied byte length of cwd. |

### 3.7 Advanced FD operations (4)

| No. | Name | Primary interface | Return | Typical errors | Status | Notes |
|---|---|---|---|---|---|---|
| 60 | Dup | `sys_dup(fd_table, oldfd)` | `AdvOpResult<FileDescriptor>` | `InvalidFd`, `TooManyFiles` | Direct module | Bitmap allocator chooses lowest free FD. |
| 61 | Dup2 | `sys_dup2(fd_table, oldfd, newfd)` | `AdvOpResult<FileDescriptor>` | `InvalidFd` | Direct module | Handles `oldfd == newfd` no-op correctly. |
| 62 | Pipe | `sys_pipe(fd_table)` | `AdvOpResult<PipeFds>` | `TooManyFiles` | Direct module | Atomic two-end allocation with cleanup on failure. |
| 63 | Ioctl | `sys_ioctl(fd_table, fd, request, arg)` | `AdvOpResult<i32>` | `InvalidFd`, `InvalidRequest` | Direct module | Validates request codes via `IoctlRequest`. |

### 3.8 Timer operations (6)

| No. | Name | Primary interface | Return | Typical errors | Status | Notes |
|---|---|---|---|---|---|---|
| 70 | SetTimer | `sys_set_timer(manager, interval, mode, callback)` | `TimeOpResult<TimerId>` | `InvalidInterval`, `TooManyTimers` | Direct module | Validates against manager resolution bounds. |
| 71 | CancelTimer | `sys_cancel_timer(manager, timer_id)` | `TimeOpResult<()>` | `InvalidTimer` | Direct module | Deletes active timer slot atomically. |
| 72 | PauseTimer | `sys_pause_timer(manager, timer_id)` | `TimeOpResult<()>` | `InvalidTimer`, `TimerNotActive` | Direct module | Requires current state `Active`. |
| 73 | ResumeTimer | `sys_resume_timer(manager, timer_id)` | `TimeOpResult<()>` | `InvalidTimer`, `InvalidArgument` | Direct module | Requires current state `Paused`. |
| 74 | GetTimerInfo | `sys_get_timer_info(manager, timer_id)` | `TimeOpResult<TimerInfo>` | `InvalidTimer` | Direct module | Returns snapshot copy of timer metadata. |
| 75 | GetTimerResolution | `sys_get_timer_resolution(manager)` | `TimerResolution` | - | Direct module | Always returns manager resolution. |

---

## 4) Trap-dispatch argument map (ABI view)

`SyscallArgs` always carries six `u64` fields (`arg0..arg5`).
Current dispatcher handlers consume arguments as follows:

| Syscall | arg0 | arg1 | arg2 | arg3 | arg4/arg5 |
|---|---|---|---|---|---|
| Exit | exit code | - | - | - | - |
| Allocate | allocator order | - | - | - | - |
| Deallocate | address | allocator order | - | - | - |
| Send | receiver pid | data pointer | data length | priority | unused |
| GrantCapability | target pid | capability | - | - | unused |
| RevokeCapability | source pid | capability | - | - | unused |
| Sleep | duration (ms) | - | - | - | - |

Validation helpers used by dispatcher:

- `validate_pointer(ptr, size)`
- `validate_pid(pid)`
- `validate_size(size, max_size)`
- `validate_order(order)`
- `validate_priority(priority)`
- `validate_capability(cap)`

---

## 5) Usage examples

### 5.1 Dispatcher usage (`GetPid`, `Sleep`)

```rust
use vantis_verified::process::Pid;
use vantis_verified::syscall::{
    SyscallArgs, SyscallContext, SyscallHandler, SyscallNumber,
};

let pid = Pid::new(42).expect("valid pid");

let getpid_ctx = SyscallContext::new(pid, SyscallNumber::GetPid, SyscallArgs::zero());
let getpid = SyscallHandler::dispatch(&getpid_ctx)?;
assert_eq!(getpid, 42);

let sleep_ctx = SyscallContext::new(
    pid,
    SyscallNumber::Sleep,
    SyscallArgs::new(100, 0, 0, 0, 0, 0), // 100 ms
);
let _ = SyscallHandler::dispatch(&sleep_ctx)?;
```

### 5.2 Path metadata + cache-aware file ops

```rust
use std::path::Path;
use vantis_verified::syscall_file_ops::{
    path_lookup_cache_stats, reset_path_lookup_cache, sys_rename, sys_stat, sys_unlink,
};

reset_path_lookup_cache();
let _s1 = sys_stat(Path::new("/tmp/demo"))?; // first lookup (miss)
let _s2 = sys_stat(Path::new("/tmp/demo"))?; // second lookup (hit)

sys_rename(Path::new("/tmp/demo"), Path::new("/tmp/demo2"))?;
sys_unlink(Path::new("/tmp/demo2"))?;

let stats = path_lookup_cache_stats();
println!("hits={}, misses={}, evictions={}", stats.hits, stats.misses, stats.evictions);
```

### 5.3 Directory operations with working directory state

```rust
use std::path::Path;
use vantis_verified::syscall_dir_ops::{sys_chdir, sys_getcwd, sys_mkdir, WorkingDirectory};

let mut wd = WorkingDirectory::new();
sys_mkdir(Path::new("/usr"), Some(0o755))?;
sys_chdir(&mut wd, Path::new("/usr"))?;

let mut buf = [0u8; 256];
let len = sys_getcwd(&wd, &mut buf, buf.len())?;
assert_eq!(&buf[..len], b"/usr");
```

### 5.4 FD operations with bitmap allocator

```rust
use vantis_verified::syscall_advanced_ops::{sys_dup, sys_dup2, sys_pipe, FdTable};

let mut table = FdTable::new();

let fd3 = sys_dup(&mut table, 1)?;     // duplicate stdout
let _ = sys_dup2(&mut table, fd3, 10)?; // duplicate to explicit fd 10
let pipe = sys_pipe(&mut table)?;
println!("pipe read={}, write={}", pipe.read_fd, pipe.write_fd);
```

### 5.5 Timer lifecycle

```rust
use std::time::Duration;
use vantis_verified::syscall_time_ops::{
    sys_cancel_timer, sys_get_timer_info, sys_pause_timer, sys_resume_timer, sys_set_timer,
    TimerManager, TimerMode,
};

let mut manager = TimerManager::new();
let tid = sys_set_timer(&mut manager, Duration::from_millis(100), TimerMode::Periodic, None)?;

sys_pause_timer(&mut manager, tid)?;
sys_resume_timer(&mut manager, tid)?;
let info = sys_get_timer_info(&manager, tid)?;
println!("timer state={:?} fire_count={}", info.state, info.fire_count);
sys_cancel_timer(&mut manager, tid)?;
```

---

## 6) Performance characteristics (Day 7 validated)

Source: `WEEK_7_DAY_7_PERFORMANCE_VALIDATION.md`.

### 6.1 Snapshot metrics

| Benchmark | Observed |
|---|---|
| `syscall_overhead` | ~48.49 ns |
| `file_operations/seek` | ~48.36 ns |
| `directory_operations/chdir` | ~49.34 ns |
| `advanced_operations/dup` | ~60.43 ns |
| `timer_operations/resume_timer` | ~52.04 ns |
| `path_lookup_cache_raw/hit` | ~44.66 ns |
| `sys_unlink_with_cache` | ~170.71 ns |
| `sys_rename_with_cache` | ~271.26 ns (high variance) |
| `fd_allocator_bitmap/sys_dup_single` | ~1.49-2.02 us |
| `fd_allocator_bitmap/sys_pipe_single` | ~1.40 us |
| `fd_allocator_bitmap/mass_dup_until_full` | ~42.80 us |

### 6.2 Interpretation notes

1. Many nanosecond-level simple syscall benchmarks are synthetic and compiler-friendly.
2. FD allocator numbers from dedicated bitmap benchmarks better represent realistic setup cost.
3. Path-cache hit path is strong; rename variance still needs follow-up.
4. Legacy `ipc_complete_benchmark` remains stale and must be migrated before using it for baseline decisions.

---

## 7) Best practices

1. **Prefer typed APIs for extended operations today.**  
   For syscall numbers `34+` in file/dir/fd/timer categories, use dedicated modules unless dispatcher wiring is added.

2. **Always validate user-facing buffers and lengths at boundaries.**  
   Mirror the dispatcher pattern: pointer checks + explicit max sizes.

3. **Treat `InvalidState` as "not yet integrated", not as runtime transient.**  
   Stubs in dispatcher should be surfaced clearly in userspace shims.

4. **Use cache invalidation deliberately after metadata mutations.**  
   `unlink`, `rename`, `mkdir`, and `rmdir` already integrate invalidation helpers; keep this coherent in future file ops.

5. **Keep path limits consistent (`4096`).**  
   Respect `MAX_FILE_PATH_LENGTH` and `MAX_PATH_LENGTH` to avoid inconsistent behavior.

6. **For FD-heavy flows, prefer deterministic descriptor handling.**  
   Bitmap allocator uses lowest available FD behavior; rely on it and avoid implicit assumptions about contiguous growth.

7. **For timers, preserve state transitions strictly.**  
   Only pause active timers and resume paused timers; map errors to caller-visible guidance.

8. **Use structured error handling, not catch-all retries.**  
   Distinguish invalid argument/path/timer/fd from actual resource pressure.

9. **Benchmark with both synthetic and scenario-driven suites.**  
   Keep `--syscall` and targeted benches together for balanced conclusions.

10. **Document dispatch wiring changes immediately.**  
    If a number transitions from `InvalidSyscall` to wired behavior, update this guide in the same PR.

---

## 8) Troubleshooting

### Symptom: `InvalidSyscall` from dispatcher on known number

Likely cause:

- Number exists in `SyscallNumber`, but dispatch branch returns `Err(SyscallError::InvalidSyscall)` by design.

What to do:

1. Confirm syscall number mapping in `SyscallNumber::from_u64`.
2. Check `SyscallHandler::dispatch` match arm for that number.
3. Use typed direct module API until dispatcher wiring is implemented.

### Symptom: `InvalidState` from dispatcher

Likely cause:

- Handler is present but still a stub (`fork`, `exec`, `wait`, `open`, `close`, `read`, `write`, `map`, `unmap`).

What to do:

1. Treat as unimplemented integration path.
2. Do not retry with same inputs.
3. Route to module-level API or defer until integration milestone.

### Symptom: path-related errors (`InvalidPath`, `PathTooLong`)

Likely cause:

- Empty path, path over 4096 bytes, or invalid normalization assumptions.

What to do:

1. Normalize and validate before syscall boundary.
2. Ensure buffer sizes in `sys_getcwd` include room for copied path.

### Symptom: FD pressure (`TooManyFiles`)

Likely cause:

- Descriptor table near full (size 1024, with 0/1/2 reserved).

What to do:

1. Close/release descriptors earlier.
2. Prefer reuse via `dup2` where appropriate.
3. Add stress tests for steady-state descriptor churn.

### Symptom: timer errors (`InvalidTimer`, `InvalidInterval`, `TimerNotActive`)

Likely cause:

- Wrong lifecycle order or interval outside manager resolution bounds.

What to do:

1. Check interval against `sys_get_timer_resolution`.
2. Verify active/paused state before pause/resume calls.
3. Cancel timers during teardown to avoid stale IDs.

---

## 9) Day 8 completion checklist

- [x] All 39 syscall numbers documented with current status.
- [x] Usage examples for dispatcher, file/dir/fd/timer paths.
- [x] Day 7 performance characteristics summarized.
- [x] Best practices section included.
- [x] Troubleshooting section included.

