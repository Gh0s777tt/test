//! Syscall-focused benchmark suite with stateful workloads.
//!
//! Day 3 hardening note:
//! - avoids constant-only synthetic paths,
//! - uses real typed syscall modules where available,
//! - adds mixed sequences to reduce optimizer artifacts.

use std::path::{Path, PathBuf};
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use vantis_verified::process::Pid;
use vantis_verified::syscall::{SyscallArgs, SyscallContext, SyscallHandler, SyscallNumber};
use vantis_verified::syscall_advanced_ops::{sys_dup, sys_dup2, sys_ioctl, sys_pipe, FdTable, IoctlRequest};
use vantis_verified::syscall_dir_ops::{
    sys_chdir_with_cache, sys_getcwd_with_cache, sys_mkdir_with_cache, sys_rmdir_with_cache,
    DirectoryEntryPathCache, WorkingDirectory,
};
use vantis_verified::syscall_file_ops::{
    sys_rename_with_cache, sys_stat_with_cache, sys_unlink_with_cache, FileStatPathCache,
};
use vantis_verified::syscall_time_ops::{
    sys_cancel_timer, sys_get_timer_info, sys_get_timer_resolution, sys_pause_timer,
    sys_resume_timer, sys_set_timer, TimerManager, TimerMode,
};

#[derive(Debug, Clone, Copy)]
enum SeekOriginSim {
    Start,
    Current,
    End,
}

#[derive(Debug, Clone, Copy)]
struct SeekState {
    position: i64,
    size: i64,
}

fn seek_simulated(
    state: &mut SeekState,
    offset: i64,
    origin: SeekOriginSim,
) -> Result<u64, &'static str> {
    let base = match origin {
        SeekOriginSim::Start => 0,
        SeekOriginSim::Current => state.position,
        SeekOriginSim::End => state.size,
    };
    let next = base.checked_add(offset).ok_or("overflow")?;
    if next < 0 {
        return Err("invalid offset");
    }
    state.position = next;
    Ok(next as u64)
}

fn pid(id: u32) -> Pid {
    Pid::new(id).expect("pid must be non-zero")
}

fn bench_syscall_overhead(c: &mut Criterion) {
    let caller = pid(42);
    c.bench_function("syscall_overhead", |b| {
        b.iter(|| {
            let context = SyscallContext::new(caller, SyscallNumber::GetPid, SyscallArgs::zero());
            black_box(SyscallHandler::dispatch(black_box(&context)).expect("getpid should succeed"))
        });
    });
}

fn bench_file_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_operations");

    group.bench_function("seek", |b| {
        b.iter_batched(
            || SeekState {
                position: 256,
                size: 4096,
            },
            |mut state| {
                let from_start = seek_simulated(
                    &mut state,
                    black_box(128),
                    black_box(SeekOriginSim::Start),
                )
                .expect("seek simulation from start should succeed");
                let from_current = seek_simulated(
                    &mut state,
                    black_box(128),
                    black_box(SeekOriginSim::Current),
                )
                .expect("seek simulation from current should succeed");
                let from_end = seek_simulated(
                    &mut state,
                    black_box(-64),
                    black_box(SeekOriginSim::End),
                )
                .expect("seek simulation from end should succeed");
                black_box((from_start, from_current, from_end));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("stat", |b| {
        b.iter_batched(
            || FileStatPathCache::new(128),
            |mut cache| {
                black_box(
                    sys_stat_with_cache(Path::new("/tmp/bench-stat"), &mut cache)
                        .expect("stat should succeed"),
                );
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("fstat", |b| {
        let mut cache = FileStatPathCache::new(128);
        let hot_path = PathBuf::from("/tmp/bench-fstat");
        sys_stat_with_cache(hot_path.as_path(), &mut cache).expect("cache warmup should succeed");

        b.iter(|| {
            black_box(
                sys_stat_with_cache(black_box(hot_path.as_path()), &mut cache)
                    .expect("cached stat should succeed"),
            );
        });
    });

    group.bench_function("unlink", |b| {
        b.iter_batched(
            || {
                let mut cache = FileStatPathCache::new(128);
                let path = PathBuf::from("/tmp/bench-unlink");
                sys_stat_with_cache(path.as_path(), &mut cache).expect("cache warmup should succeed");
                (cache, path)
            },
            |(mut cache, path)| {
                black_box(sys_unlink_with_cache(path.as_path(), &mut cache).expect("unlink should succeed"));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("rename", |b| {
        b.iter_batched(
            || {
                let mut cache = FileStatPathCache::new(128);
                let old_path = PathBuf::from("/tmp/bench-old");
                let new_path = PathBuf::from("/tmp/bench-new");
                sys_stat_with_cache(old_path.as_path(), &mut cache).expect("cache warmup should succeed");
                (cache, old_path, new_path)
            },
            |(mut cache, old_path, new_path)| {
                black_box(
                    sys_rename_with_cache(old_path.as_path(), new_path.as_path(), &mut cache)
                        .expect("rename should succeed"),
                );
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_directory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("directory_operations");

    group.bench_function("mkdir", |b| {
        b.iter_batched(
            || DirectoryEntryPathCache::new(128),
            |mut cache| {
                black_box(
                    sys_mkdir_with_cache(Path::new("/tmp/newdir"), Some(0o755), &mut cache)
                        .expect("mkdir should succeed"),
                );
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("rmdir", |b| {
        b.iter_batched(
            || {
                let mut cache = DirectoryEntryPathCache::new(128);
                let target = Path::new("/tmp/olddir");
                sys_mkdir_with_cache(target, Some(0o755), &mut cache).expect("mkdir setup should succeed");
                cache
            },
            |mut cache| {
                black_box(
                    sys_rmdir_with_cache(Path::new("/tmp/olddir"), &mut cache)
                        .expect("rmdir should succeed"),
                );
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("chdir", |b| {
        b.iter_batched(
            || (WorkingDirectory::new(), DirectoryEntryPathCache::new(128)),
            |(mut wd, mut cache)| {
                black_box(
                    sys_chdir_with_cache(&mut wd, Path::new("/home/user"), &mut cache)
                        .expect("chdir should succeed"),
                );
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("getcwd", |b| {
        b.iter_batched(
            || {
                let mut wd = WorkingDirectory::new();
                let mut cache = DirectoryEntryPathCache::new(128);
                sys_chdir_with_cache(&mut wd, Path::new("/home/user/project"), &mut cache)
                    .expect("chdir setup should succeed");
                let buf = [0u8; 256];
                (wd, cache, buf)
            },
            |(wd, mut cache, mut buf)| {
                let size = buf.len();
                black_box(
                    sys_getcwd_with_cache(&wd, &mut buf, size, &mut cache)
                        .expect("getcwd should succeed"),
                );
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_advanced_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("advanced_operations");

    group.bench_function("dup", |b| {
        b.iter_batched(
            FdTable::new,
            |mut table| {
                black_box(sys_dup(&mut table, 1).expect("dup should succeed"));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("dup2", |b| {
        b.iter_batched(
            || {
                let mut table = FdTable::new();
                let _ = sys_dup(&mut table, 1).expect("dup setup should succeed");
                table
            },
            |mut table| {
                black_box(sys_dup2(&mut table, 2, 3).expect("dup2 should succeed"));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("pipe", |b| {
        b.iter_batched(
            FdTable::new,
            |mut table| {
                black_box(sys_pipe(&mut table).expect("pipe should succeed"));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("ioctl", |b| {
        b.iter_batched(
            FdTable::new,
            |table| {
                black_box(
                    sys_ioctl(&table, 1, IoctlRequest::Tcgets as u32, None)
                        .expect("ioctl should succeed"),
                );
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_timer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("timer_operations");

    group.bench_function("set_timer", |b| {
        b.iter_batched(
            TimerManager::new,
            |mut manager| {
                black_box(
                    sys_set_timer(
                        &mut manager,
                        Duration::from_millis(100),
                        TimerMode::OneShot,
                        None,
                    )
                    .expect("set_timer should succeed"),
                );
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("cancel_timer", |b| {
        b.iter_batched(
            || {
                let mut manager = TimerManager::new();
                let tid = sys_set_timer(
                    &mut manager,
                    Duration::from_millis(100),
                    TimerMode::OneShot,
                    None,
                )
                .expect("set_timer setup should succeed");
                (manager, tid)
            },
            |(mut manager, tid)| {
                black_box(sys_cancel_timer(&mut manager, tid).expect("cancel_timer should succeed"));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("pause_timer", |b| {
        b.iter_batched(
            || {
                let mut manager = TimerManager::new();
                let tid = sys_set_timer(
                    &mut manager,
                    Duration::from_millis(100),
                    TimerMode::Periodic,
                    None,
                )
                .expect("set_timer setup should succeed");
                (manager, tid)
            },
            |(mut manager, tid)| {
                black_box(sys_pause_timer(&mut manager, tid).expect("pause_timer should succeed"));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("resume_timer", |b| {
        b.iter_batched(
            || {
                let mut manager = TimerManager::new();
                let tid = sys_set_timer(
                    &mut manager,
                    Duration::from_millis(100),
                    TimerMode::Periodic,
                    None,
                )
                .expect("set_timer setup should succeed");
                sys_pause_timer(&mut manager, tid).expect("pause setup should succeed");
                (manager, tid)
            },
            |(mut manager, tid)| {
                black_box(sys_resume_timer(&mut manager, tid).expect("resume_timer should succeed"));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("get_timer_info", |b| {
        b.iter_batched(
            || {
                let mut manager = TimerManager::new();
                let tid = sys_set_timer(
                    &mut manager,
                    Duration::from_millis(100),
                    TimerMode::OneShot,
                    None,
                )
                .expect("set_timer setup should succeed");
                (manager, tid)
            },
            |(manager, tid)| {
                black_box(sys_get_timer_info(&manager, tid).expect("get_timer_info should succeed"));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("get_timer_resolution", |b| {
        let manager = TimerManager::new();
        b.iter(|| {
            black_box(sys_get_timer_resolution(black_box(&manager)));
        });
    });

    group.finish();
}

fn bench_mixed_workloads(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_workloads");

    group.bench_function("file_dir_fd_sequence", |b| {
        b.iter_batched(
            || {
                (
                    FileStatPathCache::new(128),
                    DirectoryEntryPathCache::new(128),
                    WorkingDirectory::new(),
                    FdTable::new(),
                )
            },
            |(mut file_cache, mut dir_cache, mut wd, mut fd_table)| {
                let old_path = Path::new("/tmp/mixed-old");
                let new_path = Path::new("/tmp/mixed-new");
                let dir = Path::new("/tmp/mixed-dir");
                let mut cwd_buf = [0u8; 128];
                let cwd_size = cwd_buf.len();

                let stat = sys_stat_with_cache(old_path, &mut file_cache).expect("stat should succeed");
                sys_rename_with_cache(old_path, new_path, &mut file_cache).expect("rename should succeed");
                sys_unlink_with_cache(new_path, &mut file_cache).expect("unlink should succeed");

                sys_mkdir_with_cache(dir, Some(0o755), &mut dir_cache).expect("mkdir should succeed");
                sys_chdir_with_cache(&mut wd, dir, &mut dir_cache).expect("chdir should succeed");
                let cwd_len = sys_getcwd_with_cache(&wd, &mut cwd_buf, cwd_size, &mut dir_cache)
                    .expect("getcwd should succeed");

                let dup_fd = sys_dup(&mut fd_table, 1).expect("dup should succeed");
                let pipe = sys_pipe(&mut fd_table).expect("pipe should succeed");
                let ioctl_code = sys_ioctl(&fd_table, 1, IoctlRequest::Tcgets as u32, None)
                    .expect("ioctl should succeed");

                black_box((
                    stat.size,
                    cwd_len,
                    dup_fd,
                    pipe.read_fd + pipe.write_fd,
                    ioctl_code,
                ));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("timer_sequence", |b| {
        b.iter_batched(
            TimerManager::new,
            |mut manager| {
                let tid = sys_set_timer(
                    &mut manager,
                    Duration::from_millis(50),
                    TimerMode::Periodic,
                    None,
                )
                .expect("set_timer should succeed");
                sys_pause_timer(&mut manager, tid).expect("pause should succeed");
                sys_resume_timer(&mut manager, tid).expect("resume should succeed");
                let info = sys_get_timer_info(&manager, tid).expect("info should succeed");
                sys_cancel_timer(&mut manager, tid).expect("cancel should succeed");
                black_box((info.fire_count, info.remaining));
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_syscall_overhead,
    bench_file_operations,
    bench_directory_operations,
    bench_advanced_operations,
    bench_timer_operations,
    bench_mixed_workloads
);
criterion_main!(benches);
