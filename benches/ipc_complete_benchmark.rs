//! IPC benchmark suite migrated to current `vantis_verified::ipc` APIs.
//!
//! This benchmark replaces the legacy `vantis_os::ipc_complete::*` dependency
//! and measures message throughput, latency, scalability, queue pressure, and
//! capability-path costs using `IpcManager`.

use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use vantis_verified::ipc::{
    Capability, IpcManager, Message, MessageId, Priority, MAX_MESSAGE_SIZE, MAX_QUEUE_SIZE,
};
use vantis_verified::process::Pid;

fn pid(id: u32) -> Pid {
    Pid::new(id).expect("pid must be non-zero")
}

fn ensure_queue(manager: &mut IpcManager, process: Pid) {
    if manager.queue_stats(process).is_none() {
        manager
            .create_queue(process)
            .expect("queue creation should succeed");
    }
}

fn setup_channel(manager: &mut IpcManager, sender_id: u32, receiver_id: u32) {
    let sender = pid(sender_id);
    let receiver = pid(receiver_id);
    ensure_queue(manager, sender);
    ensure_queue(manager, receiver);
    manager
        .grant_capability(sender, receiver, Capability::Send)
        .expect("capability grant should succeed");
}

fn send_receive_once(
    manager: &mut IpcManager,
    sender_id: u32,
    receiver_id: u32,
    payload: &[u8],
    priority: Priority,
) {
    let sender = pid(sender_id);
    let receiver = pid(receiver_id);
    manager
        .send(sender, receiver, payload.to_vec(), priority)
        .expect("send should succeed");
    let received = manager
        .receive(receiver)
        .expect("receive call should succeed")
        .expect("message should be available");
    black_box(received.id());
}

fn bench_throughput_small_messages(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_throughput_small");

    for size in [64usize, 128, 256, 512, 1024] {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut manager = IpcManager::new();
            setup_channel(&mut manager, 1, 2);
            let payload = vec![0u8; size];

            b.iter(|| {
                send_receive_once(
                    black_box(&mut manager),
                    1,
                    2,
                    black_box(payload.as_slice()),
                    Priority::Normal,
                );
            });
        });
    }

    group.finish();
}

fn bench_throughput_large_messages(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_throughput_large");

    for size in [2048usize, 3072, MAX_MESSAGE_SIZE] {
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut manager = IpcManager::new();
            setup_channel(&mut manager, 1, 2);
            let payload = vec![0u8; size];

            b.iter(|| {
                send_receive_once(
                    black_box(&mut manager),
                    1,
                    2,
                    black_box(payload.as_slice()),
                    Priority::High,
                );
            });
        });
    }

    group.finish();
}

fn bench_throughput_burst(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_throughput_burst");

    for burst in [8usize, 16, 32, MAX_QUEUE_SIZE] {
        group.throughput(Throughput::Elements(burst as u64));
        group.bench_with_input(BenchmarkId::from_parameter(burst), &burst, |b, &burst| {
            let mut manager = IpcManager::new();
            setup_channel(&mut manager, 1, 2);
            let payload = vec![0u8; 256];
            let sender = pid(1);
            let receiver = pid(2);

            b.iter(|| {
                for _ in 0..burst {
                    manager
                        .send(sender, receiver, payload.clone(), Priority::Normal)
                        .expect("burst send should succeed");
                }
                for _ in 0..burst {
                    black_box(
                        manager
                            .receive(receiver)
                            .expect("burst receive call should succeed")
                            .expect("burst receive should return message"),
                    );
                }
            });
        });
    }

    group.finish();
}

fn bench_latency_send_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_latency_send_only");

    for size in [64usize, 256, 1024, MAX_MESSAGE_SIZE] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter_batched(
                || {
                    let mut manager = IpcManager::new();
                    setup_channel(&mut manager, 1, 2);
                    let payload = vec![0u8; size];
                    (manager, payload)
                },
                |(mut manager, payload)| {
                    let sender = pid(1);
                    let receiver = pid(2);
                    manager
                        .send(sender, receiver, payload, Priority::Normal)
                        .expect("send should succeed");
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_latency_receive_ready(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_latency_receive");

    group.bench_function("receive_ready_1kb", |b| {
        b.iter_batched(
            || {
                let mut manager = IpcManager::new();
                setup_channel(&mut manager, 1, 2);
                let sender = pid(1);
                let receiver = pid(2);
                manager
                    .send(sender, receiver, vec![0u8; 1024], Priority::Normal)
                    .expect("setup send should succeed");
                manager
            },
            |mut manager| {
                black_box(
                    manager
                        .receive(pid(2))
                        .expect("receive call should succeed")
                        .expect("message should be ready"),
                );
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_scalability_processes(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_scalability_processes");

    for process_count in [10u32, 50, 100, 250, 500] {
        group.bench_with_input(
            BenchmarkId::from_parameter(process_count),
            &process_count,
            |b, &process_count| {
                let mut manager = IpcManager::new();
                for id in 1..=(process_count + 1) {
                    ensure_queue(&mut manager, pid(id));
                }
                for id in 1..=process_count {
                    manager
                        .grant_capability(pid(id), pid(id + 1), Capability::Send)
                        .expect("capability grant should succeed");
                }

                b.iter(|| {
                    for id in 1..=process_count {
                        manager
                            .send(
                                pid(id),
                                pid(id + 1),
                                black_box(vec![0xAAu8; 128]),
                                Priority::Normal,
                            )
                            .expect("fanout send should succeed");
                    }
                    for id in 2..=(process_count + 1) {
                        black_box(
                            manager
                                .receive(pid(id))
                                .expect("fanout receive call should succeed")
                                .expect("fanout receive should return message"),
                        );
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_queue_pressure(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_queue_pressure");

    group.bench_function("fill_and_drain_max_queue", |b| {
        b.iter_batched(
            || {
                let mut manager = IpcManager::new();
                setup_channel(&mut manager, 1, 2);
                manager
            },
            |mut manager| {
                let sender = pid(1);
                let receiver = pid(2);

                for _ in 0..MAX_QUEUE_SIZE {
                    manager
                        .send(sender, receiver, vec![0u8; 512], Priority::Normal)
                        .expect("queue fill send should succeed");
                }

                let (len, capacity) = manager
                    .queue_stats(receiver)
                    .expect("receiver queue should exist");
                black_box((len, capacity));

                for _ in 0..MAX_QUEUE_SIZE {
                    manager
                        .receive(receiver)
                        .expect("queue drain receive call should succeed")
                        .expect("queue drain should return message");
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

fn bench_capability_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_capability_ops");

    group.bench_function("grant_send_capability", |b| {
        b.iter_batched(
            || {
                let mut manager = IpcManager::new();
                ensure_queue(&mut manager, pid(1));
                ensure_queue(&mut manager, pid(2));
                manager
            },
            |mut manager| {
                manager
                    .grant_capability(pid(1), pid(2), Capability::Send)
                    .expect("grant should succeed");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("revoke_send_capability", |b| {
        b.iter_batched(
            || {
                let mut manager = IpcManager::new();
                ensure_queue(&mut manager, pid(1));
                ensure_queue(&mut manager, pid(2));
                manager
                    .grant_capability(pid(1), pid(2), Capability::Send)
                    .expect("setup grant should succeed");
                manager
            },
            |mut manager| {
                manager
                    .revoke_capability(pid(1), pid(2), Capability::Send)
                    .expect("revoke should succeed");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("lookup_capability", |b| {
        let mut manager = IpcManager::new();
        for id in 1..=512u32 {
            ensure_queue(&mut manager, pid(id));
            ensure_queue(&mut manager, pid(id + 1));
            manager
                .grant_capability(pid(id), pid(id + 1), Capability::Send)
                .expect("bulk capability grant should succeed");
        }

        b.iter(|| {
            black_box(manager.has_capability(pid(256), pid(257), Capability::Send));
        });
    });

    group.finish();
}

fn bench_message_verification_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_message_verification");

    group.bench_function("message_creation_1kb", |b| {
        let sender = pid(1);
        let receiver = pid(2);
        let payload = vec![0u8; 1024];
        let mut next_id = 1u64;

        b.iter(|| {
            let message = Message::new(
                MessageId::new(next_id),
                sender,
                receiver,
                payload.clone(),
                Priority::Normal,
            )
            .expect("message creation should succeed");
            next_id = next_id.saturating_add(1);
            black_box(message);
        });
    });

    group.bench_function("message_is_valid_1kb", |b| {
        let sender = pid(1);
        let receiver = pid(2);
        let message = Message::new(
            MessageId::new(1),
            sender,
            receiver,
            vec![0u8; 1024],
            Priority::Normal,
        )
        .expect("message creation should succeed");

        b.iter(|| {
            black_box(message.is_valid());
        });
    });

    group.finish();
}

fn bench_stress_high_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("ipc_stress_high_load");
    group.sample_size(10);

    group.bench_function("1000_roundtrips", |b| {
        let mut manager = IpcManager::new();
        setup_channel(&mut manager, 1, 2);
        let payload = vec![0u8; 512];

        b.iter(|| {
            for _ in 0..1000 {
                send_receive_once(&mut manager, 1, 2, payload.as_slice(), Priority::Normal);
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_throughput_small_messages,
    bench_throughput_large_messages,
    bench_throughput_burst,
    bench_latency_send_only,
    bench_latency_receive_ready,
    bench_scalability_processes,
    bench_queue_pressure,
    bench_capability_operations,
    bench_message_verification_overhead,
    bench_stress_high_load
);
criterion_main!(benches);
