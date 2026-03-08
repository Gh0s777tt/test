//! # Complete IPC System Performance Benchmarks
//!
//! Comprehensive benchmarks for the fully integrated IPC system with all 5 properties.
//!
//! ## Benchmark Categories
//! 1. Throughput benchmarks (messages/second)
//! 2. Latency benchmarks (microseconds)
//! 3. Scalability benchmarks (1-1000 processes)
//! 4. Memory usage benchmarks
//! 5. Comparison with individual properties

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::sync::Arc;
use std::thread;
use vantis_os::ipc_complete::*;

// ============================================================================
// THROUGHPUT BENCHMARKS
// ============================================================================

fn bench_throughput_small_messages(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput_small");
    
    for size in [64, 128, 256, 512, 1024].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let ipc = IpcSystem::new();
            let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
            let data = vec![0u8; size];
            
            b.iter(|| {
                let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
                ipc.send(msg).unwrap();
                ipc.receive(2).unwrap();
            });
        });
    }
    
    group.finish();
}

fn bench_throughput_large_messages(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput_large");
    
    for size in [2048, 3072, 4096].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let ipc = IpcSystem::new();
            let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
            let data = vec![0u8; size];
            
            b.iter(|| {
                let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
                ipc.send(msg).unwrap();
                ipc.receive(2).unwrap();
            });
        });
    }
    
    group.finish();
}

fn bench_throughput_burst(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput_burst");
    
    for burst_size in [10, 50, 100].iter() {
        group.throughput(Throughput::Elements(*burst_size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(burst_size), burst_size, |b, &burst_size| {
            let ipc = IpcSystem::new();
            let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
            let data = vec![0u8; 1024];
            
            b.iter(|| {
                // Send burst
                for _ in 0..burst_size {
                    let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
                    ipc.send(msg).unwrap();
                }
                
                // Receive burst
                for _ in 0..burst_size {
                    ipc.receive(2).unwrap();
                }
            });
        });
    }
    
    group.finish();
}

// ============================================================================
// LATENCY BENCHMARKS
// ============================================================================

fn bench_latency_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_roundtrip");
    
    group.bench_function("1kb_message", |b| {
        let ipc = IpcSystem::new();
        let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
        let data = vec![0u8; 1024];
        
        b.iter(|| {
            let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
            let start = std::time::Instant::now();
            ipc.send(msg).unwrap();
            ipc.receive(2).unwrap();
            start.elapsed()
        });
    });
    
    group.finish();
}

fn bench_latency_send_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_send");
    
    for size in [64, 256, 1024, 4096].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let ipc = IpcSystem::new();
            let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
            let data = vec![0u8; size];
            
            b.iter(|| {
                let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
                ipc.send(msg).unwrap();
            });
        });
    }
    
    group.finish();
}

fn bench_latency_receive_only(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency_receive");
    
    group.bench_function("receive_ready", |b| {
        let ipc = IpcSystem::new();
        let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
        
        b.iter_batched(
            || {
                // Setup: send message
                let data = vec![0u8; 1024];
                let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
                ipc.send(msg).unwrap();
            },
            |_| {
                // Measure: receive message
                ipc.receive(2).unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    group.finish();
}

// ============================================================================
// SCALABILITY BENCHMARKS
// ============================================================================

fn bench_scalability_processes(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability_processes");
    
    for num_processes in [10, 50, 100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(num_processes), num_processes, |b, &num_processes| {
            let ipc = IpcSystem::new();
            
            // Create capabilities for all processes
            let mut capabilities = vec![];
            for i in 0..num_processes {
                let (cap_id, token) = ipc.create_capability(i as u64, (i + 1) as u64).unwrap();
                capabilities.push((cap_id, token));
            }
            
            b.iter(|| {
                // Send from all processes
                for (i, (cap_id, token)) in capabilities.iter().enumerate() {
                    let data = format!("Process {}", i);
                    let msg = CompleteMessage::new(data.as_bytes(), i as u64, (i + 1) as u64, *cap_id, *token).unwrap();
                    ipc.send(msg).unwrap();
                }
                
                // Receive all messages
                for i in 0..num_processes {
                    ipc.receive((i + 1) as u64).unwrap();
                }
            });
        });
    }
    
    group.finish();
}

fn bench_scalability_concurrent(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability_concurrent");
    
    for num_threads in [2, 4, 8, 16].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(num_threads), num_threads, |b, &num_threads| {
            let ipc = Arc::new(IpcSystem::new());
            
            b.iter(|| {
                let mut handles = vec![];
                
                for i in 0..num_threads {
                    let ipc_clone = Arc::clone(&ipc);
                    let (cap_id, token) = ipc.create_capability(i as u64, (i + 1) as u64).unwrap();
                    
                    let handle = thread::spawn(move || {
                        for j in 0..10 {
                            let data = format!("Thread {} msg {}", i, j);
                            let msg = CompleteMessage::new(data.as_bytes(), i as u64, (i + 1) as u64, cap_id, token).unwrap();
                            ipc_clone.send(msg).unwrap();
                        }
                    });
                    handles.push(handle);
                }
                
                for handle in handles {
                    handle.join().unwrap();
                }
                
                // Receive all messages
                for i in 0..num_threads {
                    for _ in 0..10 {
                        ipc.receive((i + 1) as u64).unwrap();
                    }
                }
            });
        });
    }
    
    group.finish();
}

// ============================================================================
// MEMORY BENCHMARKS
// ============================================================================

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    
    group.bench_function("queue_fill", |b| {
        let ipc = IpcSystem::new();
        let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
        let data = vec![0u8; 1024];
        
        b.iter(|| {
            let initial_memory = ipc.memory_usage();
            
            // Fill queue
            for _ in 0..MAX_QUEUE_SIZE {
                let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
                ipc.send(msg).unwrap();
            }
            
            let peak_memory = ipc.memory_usage();
            
            // Empty queue
            for _ in 0..MAX_QUEUE_SIZE {
                ipc.receive(2).unwrap();
            }
            
            let final_memory = ipc.memory_usage();
            
            (initial_memory, peak_memory, final_memory)
        });
    });
    
    group.finish();
}

// ============================================================================
// CAPABILITY BENCHMARKS
// ============================================================================

fn bench_capability_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("capability_ops");
    
    group.bench_function("create", |b| {
        let ipc = IpcSystem::new();
        let mut counter = 0u64;
        
        b.iter(|| {
            counter += 1;
            ipc.create_capability(counter, counter + 1).unwrap()
        });
    });
    
    group.bench_function("revoke", |b| {
        let ipc = IpcSystem::new();
        
        b.iter_batched(
            || {
                // Setup: create capability
                ipc.create_capability(1, 2).unwrap().0
            },
            |cap_id| {
                // Measure: revoke capability
                ipc.revoke_capability(cap_id).unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });
    
    group.finish();
}

// ============================================================================
// VERIFICATION OVERHEAD BENCHMARKS
// ============================================================================

fn bench_verification_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("verification_overhead");
    
    group.bench_function("integrity_check", |b| {
        let data = vec![0u8; 1024];
        let msg = CompleteMessage::new(&data, 1, 2, 100, 0xDEADBEEF).unwrap();
        
        b.iter(|| {
            msg.verify_integrity().unwrap();
        });
    });
    
    group.bench_function("message_creation", |b| {
        let data = vec![0u8; 1024];
        
        b.iter(|| {
            CompleteMessage::new(&data, 1, 2, 100, 0xDEADBEEF).unwrap()
        });
    });
    
    group.finish();
}

// ============================================================================
// COMPARISON BENCHMARKS
// ============================================================================

fn bench_comparison_with_baseline(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison");
    
    group.bench_function("complete_system", |b| {
        let ipc = IpcSystem::new();
        let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
        let data = vec![0u8; 1024];
        
        b.iter(|| {
            let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
            ipc.send(msg).unwrap();
            ipc.receive(2).unwrap();
        });
    });
    
    group.bench_function("message_only", |b| {
        let data = vec![0u8; 1024];
        
        b.iter(|| {
            CompleteMessage::new(&data, 1, 2, 100, 0xDEADBEEF).unwrap()
        });
    });
    
    group.finish();
}

// ============================================================================
// STRESS BENCHMARKS
// ============================================================================

fn bench_stress_high_load(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress_high_load");
    group.sample_size(10); // Reduce sample size for stress tests
    
    group.bench_function("1000_messages", |b| {
        let ipc = IpcSystem::new();
        let (cap_id, token) = ipc.create_capability(1, 2).unwrap();
        let data = vec![0u8; 1024];
        
        b.iter(|| {
            for _ in 0..1000 {
                let msg = CompleteMessage::new(&data, 1, 2, cap_id, token).unwrap();
                ipc.send(msg).unwrap();
            }
            
            for _ in 0..1000 {
                ipc.receive(2).unwrap();
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
    bench_latency_roundtrip,
    bench_latency_send_only,
    bench_latency_receive_only,
    bench_scalability_processes,
    bench_scalability_concurrent,
    bench_memory_usage,
    bench_capability_operations,
    bench_verification_overhead,
    bench_comparison_with_baseline,
    bench_stress_high_load,
);

criterion_main!(benches);