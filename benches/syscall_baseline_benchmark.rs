//! # Baseline Syscall Performance Benchmarks
//!
//! Simplified benchmark suite to establish performance baseline
//! for VantisOS syscalls without requiring full module imports.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::time::Duration;

// ============================================================================
// BASELINE OVERHEAD BENCHMARKS
// ============================================================================

fn bench_baseline_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("baseline_overhead");
    
    // Measure function call overhead
    group.bench_function("function_call", |b| {
        b.iter(|| {
            black_box(simple_function(black_box(42)))
        });
    });
    
    // Measure validation overhead
    group.bench_function("validation", |b| {
        b.iter(|| {
            black_box(validate_input(black_box(42)))
        });
    });
    
    // Measure error handling overhead
    group.bench_function("error_handling", |b| {
        b.iter(|| {
            black_box(function_with_result(black_box(42)))
        });
    });
    
    group.finish();
}

// ============================================================================
// DATA STRUCTURE BENCHMARKS
// ============================================================================

fn bench_data_structures(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_structures");
    
    // Vec operations
    group.bench_function("vec_push", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            for i in 0..100 {
                v.push(black_box(i));
            }
            v
        });
    });
    
    // HashMap operations
    group.bench_function("hashmap_insert", |b| {
        use std::collections::HashMap;
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..100 {
                map.insert(black_box(i), black_box(i * 2));
            }
            map
        });
    });
    
    // Option operations
    group.bench_function("option_unwrap", |b| {
        let opt = Some(42);
        b.iter(|| {
            black_box(opt.unwrap())
        });
    });
    
    // Result operations
    group.bench_function("result_unwrap", |b| {
        let res: Result<i32, ()> = Ok(42);
        b.iter(|| {
            black_box(res.unwrap())
        });
    });
    
    group.finish();
}

// ============================================================================
// MEMORY OPERATIONS BENCHMARKS
// ============================================================================

fn bench_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");
    
    // Small allocation
    group.bench_function("alloc_small", |b| {
        b.iter(|| {
            let v: Vec<u8> = vec![0; 64];
            black_box(v)
        });
    });
    
    // Medium allocation
    group.bench_function("alloc_medium", |b| {
        b.iter(|| {
            let v: Vec<u8> = vec![0; 4096];
            black_box(v)
        });
    });
    
    // Large allocation
    group.bench_function("alloc_large", |b| {
        b.iter(|| {
            let v: Vec<u8> = vec![0; 65536];
            black_box(v)
        });
    });
    
    // Memory copy
    group.bench_function("memcpy", |b| {
        let src = vec![0u8; 4096];
        b.iter(|| {
            let dst = src.clone();
            black_box(dst)
        });
    });
    
    group.finish();
}

// ============================================================================
// PATH OPERATIONS BENCHMARKS
// ============================================================================

fn bench_path_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_operations");
    
    use std::path::{Path, PathBuf};
    
    // Path creation
    group.bench_function("path_new", |b| {
        b.iter(|| {
            black_box(Path::new("/usr/local/bin"))
        });
    });
    
    // PathBuf creation
    group.bench_function("pathbuf_new", |b| {
        b.iter(|| {
            black_box(PathBuf::from("/usr/local/bin"))
        });
    });
    
    // Path join
    group.bench_function("path_join", |b| {
        let base = Path::new("/usr");
        b.iter(|| {
            black_box(base.join("local"))
        });
    });
    
    // Path components
    group.bench_function("path_components", |b| {
        let path = Path::new("/usr/local/bin");
        b.iter(|| {
            black_box(path.components().count())
        });
    });
    
    group.finish();
}

// ============================================================================
// STRING OPERATIONS BENCHMARKS
// ============================================================================

fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");
    
    // String creation
    group.bench_function("string_new", |b| {
        b.iter(|| {
            black_box(String::from("test string"))
        });
    });
    
    // String clone
    group.bench_function("string_clone", |b| {
        let s = String::from("test string");
        b.iter(|| {
            black_box(s.clone())
        });
    });
    
    // String concatenation
    group.bench_function("string_concat", |b| {
        b.iter(|| {
            let mut s = String::from("hello");
            s.push_str(" world");
            black_box(s)
        });
    });
    
    // String comparison
    group.bench_function("string_compare", |b| {
        let s1 = String::from("test");
        let s2 = String::from("test");
        b.iter(|| {
            black_box(s1 == s2)
        });
    });
    
    group.finish();
}

// ============================================================================
// LOCK OPERATIONS BENCHMARKS
// ============================================================================

fn bench_lock_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("lock_operations");
    
    use std::sync::{Arc, Mutex, RwLock};
    
    // Mutex lock/unlock
    group.bench_function("mutex_lock", |b| {
        let mutex = Arc::new(Mutex::new(0));
        b.iter(|| {
            let mut guard = mutex.lock().unwrap();
            *guard += 1;
            black_box(*guard)
        });
    });
    
    // RwLock read
    group.bench_function("rwlock_read", |b| {
        let rwlock = Arc::new(RwLock::new(0));
        b.iter(|| {
            let guard = rwlock.read().unwrap();
            black_box(*guard)
        });
    });
    
    // RwLock write
    group.bench_function("rwlock_write", |b| {
        let rwlock = Arc::new(RwLock::new(0));
        b.iter(|| {
            let mut guard = rwlock.write().unwrap();
            *guard += 1;
            black_box(*guard)
        });
    });
    
    group.finish();
}

// ============================================================================
// TIME OPERATIONS BENCHMARKS
// ============================================================================

fn bench_time_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("time_operations");
    
    use std::time::{Duration, Instant};
    
    // Duration creation
    group.bench_function("duration_new", |b| {
        b.iter(|| {
            black_box(Duration::from_millis(100))
        });
    });
    
    // Instant now
    group.bench_function("instant_now", |b| {
        b.iter(|| {
            black_box(Instant::now())
        });
    });
    
    // Duration arithmetic
    group.bench_function("duration_add", |b| {
        let d1 = Duration::from_millis(100);
        let d2 = Duration::from_millis(50);
        b.iter(|| {
            black_box(d1 + d2)
        });
    });
    
    group.finish();
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

#[inline(never)]
fn simple_function(x: i32) -> i32 {
    x * 2
}

#[inline(never)]
fn validate_input(x: i32) -> bool {
    x >= 0 && x < 1000
}

#[inline(never)]
fn function_with_result(x: i32) -> Result<i32, &'static str> {
    if x >= 0 {
        Ok(x * 2)
    } else {
        Err("negative input")
    }
}

// ============================================================================
// BENCHMARK GROUPS
// ============================================================================

criterion_group!(
    baseline,
    bench_baseline_overhead,
    bench_data_structures,
    bench_memory_operations,
);

criterion_group!(
    operations,
    bench_path_operations,
    bench_string_operations,
    bench_lock_operations,
    bench_time_operations,
);

criterion_main!(baseline, operations);