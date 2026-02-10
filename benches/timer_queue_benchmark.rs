//! Benchmarks for min-heap timer queue optimization (Day 12).

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use vantis_verified::syscall_time_ops::{sys_set_timer, TimerManager, TimerMode};

fn bench_timer_queue_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("timer_queue_set");

    group.bench_function("set_timer_single", |b| {
        b.iter_batched(
            TimerManager::new,
            |mut manager| {
                black_box(sys_set_timer(
                    &mut manager,
                    Duration::from_millis(10),
                    TimerMode::OneShot,
                    None,
                ))
                .expect("set_timer should succeed");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("set_timer_batch_255", |b| {
        b.iter(|| {
            let mut manager = TimerManager::new();
            for idx in 1..=255u64 {
                let interval = Duration::from_micros(idx * 10);
                black_box(sys_set_timer(
                    &mut manager,
                    interval,
                    TimerMode::OneShot,
                    None,
                ))
                .expect("set_timer should succeed");
            }
        });
    });

    group.finish();
}

fn bench_timer_queue_advance(c: &mut Criterion) {
    let mut group = c.benchmark_group("timer_queue_advance");

    group.bench_function("advance_time_sparse_expiry", |b| {
        b.iter_batched(
            || {
                let mut manager = TimerManager::new();
                for idx in 0..128u64 {
                    let interval = Duration::from_millis(100 + idx);
                    let _ = sys_set_timer(&mut manager, interval, TimerMode::OneShot, None);
                }
                manager
            },
            |mut manager| {
                black_box(manager.advance_time_and_collect_expired(Duration::from_millis(50)));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("advance_time_dense_expiry", |b| {
        b.iter_batched(
            || {
                let mut manager = TimerManager::new();
                for idx in 0..128u64 {
                    let interval = Duration::from_millis(1 + (idx % 4));
                    let _ = sys_set_timer(&mut manager, interval, TimerMode::Periodic, None);
                }
                manager
            },
            |mut manager| {
                black_box(manager.advance_time_and_collect_expired(Duration::from_millis(5)));
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_timer_queue_set, bench_timer_queue_advance);
criterion_main!(benches);
