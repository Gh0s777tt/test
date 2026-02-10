//! Benchmarks for bitmap-based file descriptor allocation.

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use vantis_verified::syscall_advanced_ops::{sys_dup, sys_dup2, sys_pipe, FdTable};

fn bench_fd_allocator(c: &mut Criterion) {
    let mut group = c.benchmark_group("fd_allocator_bitmap");

    group.bench_function("sys_dup_single", |b| {
        b.iter_batched(
            FdTable::new,
            |mut table| {
                black_box(sys_dup(&mut table, 1)).expect("dup should allocate a fd");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sys_pipe_single", |b| {
        b.iter_batched(
            FdTable::new,
            |mut table| {
                black_box(sys_pipe(&mut table)).expect("pipe should allocate read/write fds");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sys_dup2_rebind_existing", |b| {
        b.iter_batched(
            || {
                let mut table = FdTable::new();
                // pre-open target fd
                let _ = sys_dup(&mut table, 1).expect("pre-open should work");
                table
            },
            |mut table| {
                black_box(sys_dup2(&mut table, 2, 3)).expect("dup2 should rebind target fd");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("mass_dup_until_full", |b| {
        b.iter(|| {
            let mut table = FdTable::new();
            let mut allocated = 0usize;
            while sys_dup(&mut table, 1).is_ok() {
                allocated += 1;
            }
            black_box(allocated);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_fd_allocator);
criterion_main!(benches);
