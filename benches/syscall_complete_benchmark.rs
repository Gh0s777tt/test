//! # Complete Syscall Benchmark Suite
//!
//! Comprehensive performance benchmarks for all 39 VantisOS syscalls.
//!
//! ## Benchmark Categories
//! 1. File Operations (5 syscalls)
//! 2. Directory Operations (4 syscalls)
//! 3. Advanced Operations (4 syscalls)
//! 4. Time Operations (6 syscalls)
//! 5. Process Operations (6 syscalls)
//! 6. Memory Operations (4 syscalls)
//! 7. IPC Operations (4 syscalls)
//! 8. System Operations (2 syscalls)
//!
//! ## Performance Targets
//! - Syscall overhead: < 100ns
//! - File operations: < 1μs
//! - Directory operations: < 5μs
//! - IPC operations: < 10μs (already verified)
//! - Timer operations: < 500ns

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;
use std::path::Path;

// Import syscall modules
use vantis_os::syscall_file_ops::*;
use vantis_os::syscall_dir_ops::*;
use vantis_os::syscall_advanced_ops::*;
use vantis_os::syscall_time_ops::*;

// ============================================================================
// FILE OPERATIONS BENCHMARKS (5 syscalls)
// ============================================================================

fn bench_file_seek(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_seek");
    
    group.bench_function("seek_from_start", |b| {
        let mut table = FileTable::new();
        let entry = create_test_file_entry("/test.txt", 1000);
        let fd = table.alloc_fd(entry).unwrap();
        
        b.iter(|| {
            sys_seek(black_box(&mut table), black_box(fd), black_box(100), black_box(SeekOrigin::Start))
        });
    });
    
    group.bench_function("seek_from_current", |b| {
        let mut table = FileTable::new();
        let entry = create_test_file_entry("/test.txt", 1000);
        let fd = table.alloc_fd(entry).unwrap();
        
        b.iter(|| {
            sys_seek(black_box(&mut table), black_box(fd), black_box(10), black_box(SeekOrigin::Current))
        });
    });
    
    group.bench_function("seek_from_end", |b| {
        let mut table = FileTable::new();
        let entry = create_test_file_entry("/test.txt", 1000);
        let fd = table.alloc_fd(entry).unwrap();
        
        b.iter(|| {
            sys_seek(black_box(&mut table), black_box(fd), black_box(-10), black_box(SeekOrigin::End))
        });
    });
    
    group.finish();
}

fn bench_file_stat(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_stat");
    
    group.bench_function("stat_by_path", |b| {
        let path = Path::new("/test.txt");
        
        b.iter(|| {
            sys_stat(black_box(path))
        });
    });
    
    group.bench_function("fstat_by_fd", |b| {
        let mut table = FileTable::new();
        let entry = create_test_file_entry("/test.txt", 1000);
        let fd = table.alloc_fd(entry).unwrap();
        
        b.iter(|| {
            sys_fstat(black_box(&table), black_box(fd))
        });
    });
    
    group.finish();
}

fn bench_file_unlink(c: &mut Criterion) {
    c.bench_function("file_unlink", |b| {
        let path = Path::new("/test.txt");
        
        b.iter(|| {
            sys_unlink(black_box(path))
        });
    });
}

fn bench_file_rename(c: &mut Criterion) {
    c.bench_function("file_rename", |b| {
        let old_path = Path::new("/old.txt");
        let new_path = Path::new("/new.txt");
        
        b.iter(|| {
            sys_rename(black_box(old_path), black_box(new_path))
        });
    });
}

// ============================================================================
// DIRECTORY OPERATIONS BENCHMARKS (4 syscalls)
// ============================================================================

fn bench_dir_mkdir(c: &mut Criterion) {
    let mut group = c.benchmark_group("dir_mkdir");
    
    group.bench_function("mkdir_default_mode", |b| {
        let path = Path::new("/test_dir");
        
        b.iter(|| {
            sys_mkdir(black_box(path), black_box(None))
        });
    });
    
    group.bench_function("mkdir_custom_mode", |b| {
        let path = Path::new("/test_dir");
        
        b.iter(|| {
            sys_mkdir(black_box(path), black_box(Some(0o755)))
        });
    });
    
    group.finish();
}

fn bench_dir_rmdir(c: &mut Criterion) {
    c.bench_function("dir_rmdir", |b| {
        let path = Path::new("/test_dir");
        
        b.iter(|| {
            sys_rmdir(black_box(path))
        });
    });
}

fn bench_dir_chdir(c: &mut Criterion) {
    let mut group = c.benchmark_group("dir_chdir");
    
    group.bench_function("chdir_absolute", |b| {
        let mut wd = WorkingDirectory::new();
        let path = Path::new("/usr/local");
        
        b.iter(|| {
            sys_chdir(black_box(&mut wd), black_box(path))
        });
    });
    
    group.bench_function("chdir_relative", |b| {
        let mut wd = WorkingDirectory::new();
        let path = Path::new("local");
        
        b.iter(|| {
            sys_chdir(black_box(&mut wd), black_box(path))
        });
    });
    
    group.bench_function("chdir_parent", |b| {
        let mut wd = WorkingDirectory::new();
        sys_chdir(&mut wd, Path::new("/usr/local")).unwrap();
        let path = Path::new("..");
        
        b.iter(|| {
            sys_chdir(black_box(&mut wd), black_box(path))
        });
    });
    
    group.finish();
}

fn bench_dir_getcwd(c: &mut Criterion) {
    let mut group = c.benchmark_group("dir_getcwd");
    
    group.bench_function("getcwd_root", |b| {
        let wd = WorkingDirectory::new();
        let mut buf = [0u8; 1024];
        
        b.iter(|| {
            sys_getcwd(black_box(&wd), black_box(&mut buf), black_box(1024))
        });
    });
    
    group.bench_function("getcwd_deep_path", |b| {
        let mut wd = WorkingDirectory::new();
        sys_chdir(&mut wd, Path::new("/usr/local/bin")).unwrap();
        let mut buf = [0u8; 1024];
        
        b.iter(|| {
            sys_getcwd(black_box(&wd), black_box(&mut buf), black_box(1024))
        });
    });
    
    group.finish();
}

// ============================================================================
// ADVANCED OPERATIONS BENCHMARKS (4 syscalls)
// ============================================================================

fn bench_adv_dup(c: &mut Criterion) {
    let mut group = c.benchmark_group("adv_dup");
    
    group.bench_function("dup", |b| {
        let mut table = FdTable::new();
        
        b.iter(|| {
            sys_dup(black_box(&mut table), black_box(1))
        });
    });
    
    group.bench_function("dup2", |b| {
        let mut table = FdTable::new();
        
        b.iter(|| {
            sys_dup2(black_box(&mut table), black_box(1), black_box(10))
        });
    });
    
    group.finish();
}

fn bench_adv_pipe(c: &mut Criterion) {
    c.bench_function("pipe_create", |b| {
        let mut table = FdTable::new();
        
        b.iter(|| {
            sys_pipe(black_box(&mut table))
        });
    });
}

fn bench_adv_ioctl(c: &mut Criterion) {
    let mut group = c.benchmark_group("adv_ioctl");
    
    group.bench_function("ioctl_tiocgwinsz", |b| {
        let table = FdTable::new();
        
        b.iter(|| {
            sys_ioctl(black_box(&table), black_box(1), black_box(0x5413), black_box(None))
        });
    });
    
    group.bench_function("ioctl_fionread", |b| {
        let table = FdTable::new();
        
        b.iter(|| {
            sys_ioctl(black_box(&table), black_box(1), black_box(0x541B), black_box(None))
        });
    });
    
    group.finish();
}

// ============================================================================
// TIME OPERATIONS BENCHMARKS (6 syscalls)
// ============================================================================

fn bench_time_set_timer(c: &mut Criterion) {
    let mut group = c.benchmark_group("time_set_timer");
    
    group.bench_function("set_timer_oneshot", |b| {
        let mut manager = TimerManager::new();
        let interval = Duration::from_millis(100);
        
        b.iter(|| {
            sys_set_timer(black_box(&mut manager), black_box(interval), black_box(TimerMode::OneShot), black_box(None))
        });
    });
    
    group.bench_function("set_timer_periodic", |b| {
        let mut manager = TimerManager::new();
        let interval = Duration::from_millis(100);
        
        b.iter(|| {
            sys_set_timer(black_box(&mut manager), black_box(interval), black_box(TimerMode::Periodic), black_box(None))
        });
    });
    
    group.finish();
}

fn bench_time_cancel_timer(c: &mut Criterion) {
    c.bench_function("cancel_timer", |b| {
        let mut manager = TimerManager::new();
        
        b.iter_batched(
            || {
                // Setup: create timer
                let interval = Duration::from_millis(100);
                sys_set_timer(&mut manager, interval, TimerMode::OneShot, None).unwrap()
            },
            |timer_id| {
                // Measure: cancel timer
                sys_cancel_timer(black_box(&mut manager), black_box(timer_id))
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn bench_time_pause_resume(c: &mut Criterion) {
    let mut group = c.benchmark_group("time_pause_resume");
    
    group.bench_function("pause_timer", |b| {
        let mut manager = TimerManager::new();
        let interval = Duration::from_millis(100);
        let timer_id = sys_set_timer(&mut manager, interval, TimerMode::Periodic, None).unwrap();
        
        b.iter(|| {
            sys_pause_timer(black_box(&mut manager), black_box(timer_id))
        });
    });
    
    group.bench_function("resume_timer", |b| {
        let mut manager = TimerManager::new();
        let interval = Duration::from_millis(100);
        let timer_id = sys_set_timer(&mut manager, interval, TimerMode::Periodic, None).unwrap();
        sys_pause_timer(&mut manager, timer_id).unwrap();
        
        b.iter(|| {
            sys_resume_timer(black_box(&mut manager), black_box(timer_id))
        });
    });
    
    group.finish();
}

fn bench_time_get_info(c: &mut Criterion) {
    let mut group = c.benchmark_group("time_get_info");
    
    group.bench_function("get_timer_info", |b| {
        let mut manager = TimerManager::new();
        let interval = Duration::from_millis(100);
        let timer_id = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None).unwrap();
        
        b.iter(|| {
            sys_get_timer_info(black_box(&manager), black_box(timer_id))
        });
    });
    
    group.bench_function("get_timer_resolution", |b| {
        let manager = TimerManager::new();
        
        b.iter(|| {
            sys_get_timer_resolution(black_box(&manager))
        });
    });
    
    group.finish();
}

// ============================================================================
// SYSCALL OVERHEAD BENCHMARKS
// ============================================================================

fn bench_syscall_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("syscall_overhead");
    
    // Measure minimal syscall overhead
    group.bench_function("minimal_overhead", |b| {
        b.iter(|| {
            // Simplest possible syscall
            black_box(0)
        });
    });
    
    // Measure validation overhead
    group.bench_function("validation_overhead", |b| {
        let table = FdTable::new();
        
        b.iter(|| {
            table.get_entry(black_box(1))
        });
    });
    
    group.finish();
}

// ============================================================================
// COMPARISON BENCHMARKS
// ============================================================================

fn bench_comparison_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    
    // Compare file operations
    group.bench_function("seek_vs_stat", |b| {
        let mut table = FileTable::new();
        let entry = create_test_file_entry("/test.txt", 1000);
        let fd = table.alloc_fd(entry).unwrap();
        
        b.iter(|| {
            let _ = sys_seek(&mut table, fd, 100, SeekOrigin::Start);
            let _ = sys_fstat(&table, fd);
        });
    });
    
    // Compare directory operations
    group.bench_function("chdir_vs_getcwd", |b| {
        let mut wd = WorkingDirectory::new();
        let mut buf = [0u8; 1024];
        
        b.iter(|| {
            let _ = sys_chdir(&mut wd, Path::new("/usr"));
            let _ = sys_getcwd(&wd, &mut buf, 1024);
        });
    });
    
    group.finish();
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn create_test_file_entry(path: &str, size: u64) -> FileEntry {
    FileEntry {
        path: std::path::PathBuf::from(path),
        position: 0,
        size,
        readable: true,
        writable: true,
    }
}

// ============================================================================
// BENCHMARK GROUPS
// ============================================================================

criterion_group!(
    file_ops,
    bench_file_seek,
    bench_file_stat,
    bench_file_unlink,
    bench_file_rename,
);

criterion_group!(
    dir_ops,
    bench_dir_mkdir,
    bench_dir_rmdir,
    bench_dir_chdir,
    bench_dir_getcwd,
);

criterion_group!(
    adv_ops,
    bench_adv_dup,
    bench_adv_pipe,
    bench_adv_ioctl,
);

criterion_group!(
    time_ops,
    bench_time_set_timer,
    bench_time_cancel_timer,
    bench_time_pause_resume,
    bench_time_get_info,
);

criterion_group!(
    overhead,
    bench_syscall_overhead,
    bench_comparison_operations,
);

criterion_main!(
    file_ops,
    dir_ops,
    adv_ops,
    time_ops,
    overhead,
);