# Migration Test Plan
## Testing Strategy for POSIX Dependency Removal

**Version**: 1.0  
**Date**: February 9, 2025  
**Status**: Test Plan  

---

## Table of Contents

1. [Overview](#overview)
2. [Test Categories](#test-categories)
3. [HashMap Migration Tests](#hashmap-migration-tests)
4. [Time Implementation Tests](#time-implementation-tests)
5. [Sync Primitives Tests](#sync-primitives-tests)
6. [RNG Implementation Tests](#rng-implementation-tests)
7. [Integration Tests](#integration-tests)
8. [Performance Tests](#performance-tests)

---

## 1. Overview

### 1.1 Purpose

This document defines the comprehensive testing strategy for migrating from std library dependencies to no_std alternatives.

### 1.2 Test Objectives

1. **Correctness**: Verify functionality matches std behavior
2. **Performance**: Ensure no significant regression
3. **Safety**: Verify memory safety and thread safety
4. **Compatibility**: Ensure existing code works unchanged
5. **Verification**: Maintain formal verification properties

### 1.3 Test Levels

```
┌─────────────────────────────────────┐
│         Unit Tests                  │  ← Test individual functions
├─────────────────────────────────────┤
│      Integration Tests              │  ← Test module interactions
├─────────────────────────────────────┤
│      System Tests                   │  ← Test full system
├─────────────────────────────────────┤
│    Performance Tests                │  ← Benchmark performance
├─────────────────────────────────────┤
│    Verification Tests               │  ← Formal verification
└─────────────────────────────────────┘
```

---

## 2. Test Categories

### 2.1 Functional Tests

**Purpose**: Verify correct behavior

**Coverage**:
- All public APIs
- Edge cases
- Error conditions
- Boundary values

**Success Criteria**: 100% pass rate

### 2.2 Performance Tests

**Purpose**: Ensure acceptable performance

**Coverage**:
- Operation latency
- Throughput
- Memory usage
- CPU usage

**Success Criteria**: <20% regression vs std

### 2.3 Safety Tests

**Purpose**: Verify memory and thread safety

**Coverage**:
- No data races
- No deadlocks
- No memory leaks
- No undefined behavior

**Success Criteria**: Zero safety violations

### 2.4 Compatibility Tests

**Purpose**: Ensure existing code works

**Coverage**:
- All existing tests pass
- No API changes required
- Drop-in replacement

**Success Criteria**: 100% backward compatibility

---

## 3. HashMap Migration Tests

### 3.1 Unit Tests

```rust
#[cfg(test)]
mod hashmap_tests {
    use super::*;
    
    #[test]
    fn test_insert_and_get() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get("key"), Some(&"value"));
    }
    
    #[test]
    fn test_remove() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.remove("key"), Some("value"));
        assert_eq!(map.get("key"), None);
    }
    
    #[test]
    fn test_iteration() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        
        let sum: i32 = map.values().sum();
        assert_eq!(sum, 6);
    }
    
    #[test]
    fn test_empty() {
        let map: HashMap<&str, i32> = HashMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }
    
    #[test]
    fn test_large_map() {
        let mut map = HashMap::new();
        for i in 0..10000 {
            map.insert(i, i * 2);
        }
        assert_eq!(map.len(), 10000);
        assert_eq!(map.get(&5000), Some(&10000));
    }
}
```

### 3.2 Performance Tests

```rust
#[cfg(test)]
mod hashmap_benchmarks {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    fn bench_insert(c: &mut Criterion) {
        c.bench_function("hashmap_insert", |b| {
            b.iter(|| {
                let mut map = HashMap::new();
                for i in 0..1000 {
                    map.insert(black_box(i), black_box(i * 2));
                }
            });
        });
    }
    
    fn bench_lookup(c: &mut Criterion) {
        let mut map = HashMap::new();
        for i in 0..1000 {
            map.insert(i, i * 2);
        }
        
        c.bench_function("hashmap_lookup", |b| {
            b.iter(|| {
                for i in 0..1000 {
                    black_box(map.get(&black_box(i)));
                }
            });
        });
    }
    
    criterion_group!(benches, bench_insert, bench_lookup);
    criterion_main!(benches);
}
```

### 3.3 Compatibility Tests

```rust
#[test]
fn test_btreemap_compatibility() {
    // Test that BTreeMap can be used as drop-in replacement
    type HashMap<K, V> = alloc::collections::BTreeMap<K, V>;
    
    let mut map = HashMap::new();
    map.insert("key", "value");
    assert_eq!(map.get("key"), Some(&"value"));
}
```

---

## 4. Time Implementation Tests

### 4.1 Unit Tests

```rust
#[cfg(test)]
mod time_tests {
    use super::*;
    
    #[test]
    fn test_duration_creation() {
        let d1 = Duration::from_secs(1);
        assert_eq!(d1.as_secs(), 1);
        
        let d2 = Duration::from_millis(1000);
        assert_eq!(d2.as_secs(), 1);
        
        let d3 = Duration::from_micros(1_000_000);
        assert_eq!(d3.as_secs(), 1);
    }
    
    #[test]
    fn test_duration_arithmetic() {
        let d1 = Duration::from_secs(1);
        let d2 = Duration::from_secs(2);
        
        assert_eq!(d1 + d2, Duration::from_secs(3));
        assert_eq!(d2 - d1, Duration::from_secs(1));
    }
    
    #[test]
    fn test_instant_now() {
        let start = Instant::now();
        // Do some work
        for _ in 0..1000 {
            black_box(42);
        }
        let end = Instant::now();
        
        assert!(end > start);
        let elapsed = end.duration_since(start);
        assert!(elapsed.as_nanos() > 0);
    }
    
    #[test]
    fn test_instant_elapsed() {
        let start = Instant::now();
        // Sleep for 1ms (using busy wait for testing)
        let target = start + Duration::from_millis(1);
        while Instant::now() < target {}
        
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() >= 1);
    }
}
```

### 4.2 Accuracy Tests

```rust
#[test]
fn test_timer_accuracy() {
    // Test that timer is accurate to within 1%
    let start = Instant::now();
    
    // Busy wait for 10ms
    let target = start + Duration::from_millis(10);
    while Instant::now() < target {}
    
    let elapsed = start.elapsed();
    let millis = elapsed.as_millis();
    
    // Allow 1% error
    assert!(millis >= 9 && millis <= 11, 
            "Timer inaccurate: expected 10ms, got {}ms", millis);
}

#[test]
fn test_tsc_calibration() {
    let freq = calibrate_tsc();
    
    // TSC frequency should be reasonable (1-5 GHz)
    assert!(freq >= 1_000_000_000);  // At least 1 GHz
    assert!(freq <= 5_000_000_000);  // At most 5 GHz
}
```

### 4.3 Performance Tests

```rust
#[bench]
fn bench_instant_now(b: &mut Bencher) {
    b.iter(|| {
        black_box(Instant::now());
    });
}

#[bench]
fn bench_duration_since(b: &mut Bencher) {
    let start = Instant::now();
    let end = Instant::now();
    
    b.iter(|| {
        black_box(end.duration_since(start));
    });
}
```

---

## 5. Sync Primitives Tests

### 5.1 Unit Tests

```rust
#[cfg(test)]
mod sync_tests {
    use super::*;
    use alloc::sync::Arc;
    use spin::Mutex;
    
    #[test]
    fn test_mutex_lock() {
        let mutex = Mutex::new(0);
        {
            let mut guard = mutex.lock();
            *guard = 42;
        }
        assert_eq!(*mutex.lock(), 42);
    }
    
    #[test]
    fn test_rwlock_read() {
        let lock = RwLock::new(42);
        let r1 = lock.read();
        let r2 = lock.read();
        assert_eq!(*r1, 42);
        assert_eq!(*r2, 42);
    }
    
    #[test]
    fn test_rwlock_write() {
        let lock = RwLock::new(0);
        {
            let mut w = lock.write();
            *w = 42;
        }
        assert_eq!(*lock.read(), 42);
    }
}
```

### 5.2 Concurrency Tests

```rust
#[test]
fn test_mutex_concurrent() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(*counter.lock(), 10000);
}

#[test]
fn test_no_deadlock() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));
    
    let lock1_clone = Arc::clone(&lock1);
    let lock2_clone = Arc::clone(&lock2);
    
    let handle = std::thread::spawn(move || {
        let _g1 = lock1_clone.lock();
        let _g2 = lock2_clone.lock();
    });
    
    {
        let _g2 = lock2.lock();
        let _g1 = lock1.lock();
    }
    
    handle.join().unwrap();
}
```

### 5.3 Performance Tests

```rust
#[bench]
fn bench_mutex_lock_unlock(b: &mut Bencher) {
    let mutex = Mutex::new(0);
    
    b.iter(|| {
        let _guard = mutex.lock();
    });
}

#[bench]
fn bench_rwlock_read(b: &mut Bencher) {
    let lock = RwLock::new(0);
    
    b.iter(|| {
        let _guard = lock.read();
    });
}
```

---

## 6. RNG Implementation Tests

### 6.1 Unit Tests

```rust
#[cfg(test)]
mod rng_tests {
    use super::*;
    
    #[test]
    fn test_rng_basic() {
        let mut rng = HardwareRng::new();
        let n1 = rng.next_u64();
        let n2 = rng.next_u64();
        
        // Should be different (with very high probability)
        assert_ne!(n1, n2);
    }
    
    #[test]
    fn test_fill_bytes() {
        let mut rng = HardwareRng::new();
        let mut buf = [0u8; 32];
        
        rng.fill_bytes(&mut buf);
        
        // Should not be all zeros
        assert!(buf.iter().any(|&x| x != 0));
    }
    
    #[test]
    fn test_rng_distribution() {
        let mut rng = HardwareRng::new();
        let mut counts = [0u32; 256];
        
        // Generate 10000 random bytes
        for _ in 0..10000 {
            let byte = (rng.next_u64() & 0xFF) as u8;
            counts[byte as usize] += 1;
        }
        
        // Each byte should appear roughly 39 times (10000/256)
        // Allow 50% deviation
        for count in counts.iter() {
            assert!(*count >= 20 && *count <= 60,
                    "Distribution test failed: count = {}", count);
        }
    }
}
```

### 6.2 Statistical Tests

```rust
#[test]
fn test_rng_chi_square() {
    // Chi-square test for randomness
    let mut rng = HardwareRng::new();
    let mut observed = [0u32; 256];
    let n = 10000;
    
    for _ in 0..n {
        let byte = (rng.next_u64() & 0xFF) as u8;
        observed[byte as usize] += 1;
    }
    
    let expected = n as f64 / 256.0;
    let mut chi_square = 0.0;
    
    for &obs in observed.iter() {
        let diff = obs as f64 - expected;
        chi_square += (diff * diff) / expected;
    }
    
    // Chi-square critical value for 255 degrees of freedom at 0.05 significance
    let critical_value = 293.25;
    assert!(chi_square < critical_value,
            "Chi-square test failed: {} > {}", chi_square, critical_value);
}
```

### 6.3 Performance Tests

```rust
#[bench]
fn bench_rng_next_u64(b: &mut Bencher) {
    let mut rng = HardwareRng::new();
    
    b.iter(|| {
        black_box(rng.next_u64());
    });
}

#[bench]
fn bench_rng_fill_bytes(b: &mut Bencher) {
    let mut rng = HardwareRng::new();
    let mut buf = [0u8; 32];
    
    b.iter(|| {
        rng.fill_bytes(&mut buf);
        black_box(&buf);
    });
}
```

---

## 7. Integration Tests

### 7.1 System-Wide Tests

```rust
#[test]
fn test_full_system() {
    // Test that all components work together
    
    // 1. Time
    let start = Instant::now();
    
    // 2. HashMap
    let mut map = HashMap::new();
    map.insert("key", "value");
    
    // 3. Sync
    let data = Arc::new(Mutex::new(map));
    
    // 4. RNG
    let mut rng = HardwareRng::new();
    let random = rng.next_u64();
    
    // 5. Verify
    let elapsed = start.elapsed();
    assert!(elapsed.as_nanos() > 0);
    assert_eq!(data.lock().get("key"), Some(&"value"));
    assert_ne!(random, 0);
}
```

### 7.2 Syscall Tests

```rust
#[test]
fn test_syscalls_with_new_deps() {
    // Test that all 39 syscalls still work
    
    // File operations
    let fd = sys_open("/tmp/test", O_RDWR | O_CREAT, 0o644).unwrap();
    sys_write(fd, b"test").unwrap();
    sys_close(fd).unwrap();
    
    // Timer operations
    let timer = sys_set_timer(Duration::from_millis(100), TimerMode::ONE_SHOT, None).unwrap();
    sys_cancel_timer(timer).unwrap();
    
    // IPC operations
    let msg = Message::new(b"test", capability).unwrap();
    sys_send(target_pid, &msg).unwrap();
}
```

---

## 8. Performance Tests

### 8.1 Benchmark Suite

```rust
// benches/migration_benchmarks.rs

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_all(c: &mut Criterion) {
    // HashMap benchmarks
    c.bench_function("hashmap_insert_1000", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for i in 0..1000 {
                map.insert(i, i * 2);
            }
        });
    });
    
    // Time benchmarks
    c.bench_function("instant_now", |b| {
        b.iter(|| Instant::now());
    });
    
    // Sync benchmarks
    c.bench_function("mutex_lock_unlock", |b| {
        let mutex = Mutex::new(0);
        b.iter(|| {
            let _guard = mutex.lock();
        });
    });
    
    // RNG benchmarks
    c.bench_function("rng_next_u64", |b| {
        let mut rng = HardwareRng::new();
        b.iter(|| rng.next_u64());
    });
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
```

### 8.2 Performance Targets

| Operation | std | Target | Max Regression |
|-----------|-----|--------|----------------|
| HashMap insert | 50ns | 100ns | 2x |
| HashMap lookup | 30ns | 60ns | 2x |
| Instant::now() | 20ns | 30ns | 1.5x |
| Mutex lock | 20ns | 10ns | 0.5x (faster!) |
| RNG next_u64 | 100ns | 50ns | 0.5x (faster!) |

---

## 9. Test Execution Plan

### 9.1 Daily Testing

**Every Day**:
```bash
# Run unit tests
cargo test

# Run benchmarks
cargo bench

# Check for regressions
./scripts/check_performance.sh
```

### 9.2 Weekly Testing

**Every Week**:
```bash
# Full test suite
cargo test --all-features

# Long-running tests
cargo test --release -- --ignored

# Stress tests
./scripts/stress_test.sh

# Memory leak detection
valgrind --leak-check=full ./target/release/vantis-verified
```

### 9.3 Pre-Commit Testing

**Before Every Commit**:
```bash
# Format check
cargo fmt --check

# Lint check
cargo clippy -- -D warnings

# Unit tests
cargo test

# Quick benchmarks
cargo bench --no-run
```

---

## 10. Success Criteria

### 10.1 Functional Criteria

- ✅ All unit tests pass (100%)
- ✅ All integration tests pass (100%)
- ✅ All syscalls work correctly
- ✅ No functionality regressions

### 10.2 Performance Criteria

- ✅ <20% regression vs std
- ✅ All benchmarks within targets
- ✅ No memory leaks
- ✅ No performance anomalies

### 10.3 Safety Criteria

- ✅ No data races
- ✅ No deadlocks
- ✅ No undefined behavior
- ✅ All Miri tests pass

### 10.4 Verification Criteria

- ✅ All formal proofs still valid
- ✅ Kani checks pass
- ✅ Verus verification succeeds
- ✅ No verification regressions

---

**Document Version**: 1.0  
**Last Updated**: February 9, 2025  
**Status**: Complete Test Plan  
**Next Steps**: Execute tests during migration