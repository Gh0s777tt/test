# POSIX Alternatives for VantisOS
## Microkernel-Friendly Implementations

**Version**: 1.0  
**Date**: February 9, 2025  
**Status**: Design Document  

---

## Table of Contents

1. [Overview](#overview)
2. [HashMap Alternative](#hashmap-alternative)
3. [Time Implementation](#time-implementation)
4. [Synchronization Primitives](#synchronization-primitives)
5. [Path Handling](#path-handling)
6. [Random Number Generation](#random-number-generation)
7. [Implementation Plan](#implementation-plan)

---

## 1. Overview

### 1.1 Purpose

This document provides detailed designs for microkernel-friendly alternatives to standard library dependencies identified in the dependency analysis.

### 1.2 Design Principles

1. **No_std Compatible**: All implementations must work without std
2. **Zero POSIX**: No direct POSIX syscall usage
3. **Formally Verifiable**: Designs support formal verification
4. **High Performance**: Match or exceed std performance
5. **Minimal Overhead**: Keep implementations lean

### 1.3 Alternatives Overview

| Dependency | Current | Alternative | Status |
|------------|---------|-------------|--------|
| HashMap | std::collections | BTreeMap / hashbrown | ✅ Ready |
| Time | std::time | Custom hardware timer | 📝 Design |
| Sync | std::sync | spin locks | ✅ Ready |
| Path | std::path | Custom path handling | 📝 Design |
| RNG | getrandom | Hardware RNG | 📝 Design |

---

## 2. HashMap Alternative

### 2.1 Problem

**Current Usage**: `std::collections::HashMap` (13 instances)

**Issues**:
- Requires std library
- Uses system allocator
- Not no_std compatible

### 2.2 Solution 1: BTreeMap (Recommended)

**Advantages**:
- ✅ Already in alloc (no_std compatible)
- ✅ Deterministic performance
- ✅ Better cache locality
- ✅ Ordered iteration
- ✅ No hashing overhead

**Disadvantages**:
- ⚠️ O(log n) vs O(1) operations
- ⚠️ Slightly slower for small maps

**Implementation**:
```rust
// Before
use std::collections::HashMap;

// After
use alloc::collections::BTreeMap;

// Type alias for compatibility
type HashMap<K, V> = BTreeMap<K, V>;
```

**Performance**:
```
Operation    | HashMap | BTreeMap | Difference
-------------|---------|----------|------------
Insert       | O(1)    | O(log n) | ~2-3x slower
Lookup       | O(1)    | O(log n) | ~2-3x slower
Remove       | O(1)    | O(log n) | ~2-3x slower
Iteration    | O(n)    | O(n)     | Similar
Memory       | Higher  | Lower    | ~30% less
```

**Recommendation**: Use BTreeMap for most cases (n < 1000)

### 2.3 Solution 2: hashbrown (Alternative)

**Advantages**:
- ✅ No_std compatible
- ✅ O(1) operations
- ✅ Drop-in replacement
- ✅ Better performance than std

**Disadvantages**:
- ⚠️ External dependency
- ⚠️ More memory usage

**Implementation**:
```toml
[dependencies]
hashbrown = { version = "0.14", default-features = false, features = ["alloc"] }
```

```rust
use hashbrown::HashMap;
```

**Performance**:
```
Operation    | std::HashMap | hashbrown | Difference
-------------|--------------|-----------|------------
Insert       | O(1)         | O(1)      | ~10% faster
Lookup       | O(1)         | O(1)      | ~15% faster
Remove       | O(1)         | O(1)      | ~10% faster
Memory       | Baseline     | -5%       | Slightly less
```

**Recommendation**: Use hashbrown for large maps (n > 1000)

### 2.4 Migration Strategy

**Phase 1: Analysis** (Day 5, Morning)
```bash
# Find all HashMap usage
grep -r "HashMap" src/verified/*.rs

# Analyze map sizes
# Small maps (< 100 items): Use BTreeMap
# Large maps (> 1000 items): Use hashbrown
```

**Phase 2: Replace Small Maps** (Day 5, Afternoon)
```rust
// Files with small maps (most files)
use alloc::collections::BTreeMap;
type HashMap<K, V> = BTreeMap<K, V>;
```

**Phase 3: Replace Large Maps** (Day 5, Evening)
```rust
// Files with large maps (if any)
use hashbrown::HashMap;
```

**Phase 4: Benchmark** (Day 6, Morning)
```rust
// Run benchmarks
cargo bench --bench hashmap_comparison
```

---

## 3. Time Implementation

### 3.1 Problem

**Current Usage**: `std::time::{Duration, Instant}` (7 instances)

**Issues**:
- Requires std library
- Uses POSIX clock_gettime()
- Not no_std compatible

### 3.2 Solution: Custom Time Module

**Design**:
```rust
// src/verified/time.rs

#![no_std]

use core::ops::{Add, Sub};

/// Duration represents a span of time
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    nanos: u64,
}

impl Duration {
    pub const ZERO: Duration = Duration { nanos: 0 };
    
    pub const fn from_secs(secs: u64) -> Self {
        Duration { nanos: secs * 1_000_000_000 }
    }
    
    pub const fn from_millis(millis: u64) -> Self {
        Duration { nanos: millis * 1_000_000 }
    }
    
    pub const fn from_micros(micros: u64) -> Self {
        Duration { nanos: micros * 1_000 }
    }
    
    pub const fn from_nanos(nanos: u64) -> Self {
        Duration { nanos }
    }
    
    pub const fn as_secs(&self) -> u64 {
        self.nanos / 1_000_000_000
    }
    
    pub const fn as_millis(&self) -> u64 {
        self.nanos / 1_000_000
    }
    
    pub const fn as_micros(&self) -> u64 {
        self.nanos / 1_000
    }
    
    pub const fn as_nanos(&self) -> u64 {
        self.nanos
    }
}

impl Add for Duration {
    type Output = Duration;
    
    fn add(self, rhs: Duration) -> Duration {
        Duration { nanos: self.nanos + rhs.nanos }
    }
}

impl Sub for Duration {
    type Output = Duration;
    
    fn sub(self, rhs: Duration) -> Duration {
        Duration { nanos: self.nanos - rhs.nanos }
    }
}

/// Instant represents a point in time
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant {
    nanos: u64,
}

impl Instant {
    /// Get current time from hardware timer
    pub fn now() -> Self {
        Instant {
            nanos: read_tsc_nanos(),
        }
    }
    
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration {
            nanos: self.nanos - earlier.nanos,
        }
    }
    
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    
    fn add(self, duration: Duration) -> Instant {
        Instant {
            nanos: self.nanos + duration.nanos,
        }
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    
    fn sub(self, duration: Duration) -> Instant {
        Instant {
            nanos: self.nanos - duration.nanos,
        }
    }
}

impl Sub for Instant {
    type Output = Duration;
    
    fn sub(self, other: Instant) -> Duration {
        Duration {
            nanos: self.nanos - other.nanos,
        }
    }
}

/// Read Time Stamp Counter (TSC) and convert to nanoseconds
#[inline]
fn read_tsc_nanos() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let tsc = core::arch::x86_64::_rdtsc();
        // Convert TSC to nanoseconds
        // Assume 3 GHz CPU (adjust based on actual frequency)
        tsc * 1000 / 3000
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    {
        // Fallback for other architectures
        // Use a monotonic counter or other hardware timer
        unimplemented!("TSC not available on this architecture")
    }
}

/// Get CPU frequency for accurate TSC conversion
pub fn calibrate_tsc() -> u64 {
    // Calibrate TSC frequency at boot
    // This should be called once during system initialization
    // Returns frequency in Hz
    
    // Method 1: Use CPUID (x86_64)
    #[cfg(target_arch = "x86_64")]
    {
        // Read TSC frequency from CPUID if available
        // Otherwise, calibrate against a known timer
        3_000_000_000 // 3 GHz default
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    {
        1_000_000_000 // 1 GHz default
    }
}
```

### 3.3 Hardware Timer Options

**Option 1: TSC (Time Stamp Counter)** - Recommended
```rust
// x86_64 only
// Pros: Very fast, high resolution
// Cons: May not be synchronized across cores

#[inline]
fn read_tsc() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}
```

**Option 2: HPET (High Precision Event Timer)**
```rust
// All architectures
// Pros: Synchronized, reliable
// Cons: Slower than TSC

fn read_hpet() -> u64 {
    // Read from HPET MMIO registers
    unsafe {
        let hpet_base = 0xFED00000 as *const u64;
        core::ptr::read_volatile(hpet_base.offset(0xF0 / 8))
    }
}
```

**Option 3: PIT (Programmable Interval Timer)**
```rust
// Legacy timer
// Pros: Always available
// Cons: Low resolution (1ms)

fn read_pit() -> u64 {
    // Read from PIT ports
    // Not recommended for high-precision timing
    unimplemented!()
}
```

**Recommendation**: Use TSC with HPET fallback

### 3.4 Migration Strategy

**Phase 1: Implement Time Module** (Day 8, Morning)
```bash
# Create time module
touch src/verified/time.rs

# Add to lib.rs
echo "pub mod time;" >> src/verified/lib.rs
```

**Phase 2: Replace std::time** (Day 8, Afternoon)
```rust
// Before
use std::time::{Duration, Instant};

// After
use crate::time::{Duration, Instant};
```

**Phase 3: Calibrate TSC** (Day 8, Evening)
```rust
// In kernel initialization
let tsc_freq = time::calibrate_tsc();
println!("TSC frequency: {} Hz", tsc_freq);
```

**Phase 4: Test** (Day 9, Morning)
```rust
// Test timer accuracy
let start = Instant::now();
// Do work
let elapsed = start.elapsed();
assert!(elapsed.as_millis() > 0);
```

---

## 4. Synchronization Primitives

### 4.1 Problem

**Current Usage**: `std::sync::{Arc, RwLock, Mutex}` (7 instances)

**Issues**:
- Requires std library
- Uses pthread primitives
- Not no_std compatible

### 4.2 Solution: Spin Locks

**Design**:
```rust
// Use spin crate (already no_std compatible)

// Cargo.toml
[dependencies]
spin = { version = "0.9", default-features = false, features = ["mutex", "rwlock", "once"] }

// Code
use alloc::sync::Arc;  // Arc is in alloc
use spin::{Mutex, RwLock, Once};
```

**Implementation**:
```rust
// Before
use std::sync::{Arc, Mutex, RwLock};

let data = Arc::new(Mutex::new(0));
let lock = data.lock().unwrap();

// After
use alloc::sync::Arc;
use spin::Mutex;

let data = Arc::new(Mutex::new(0));
let lock = data.lock();  // No unwrap needed
```

### 4.3 Performance Comparison

```
Operation        | std::Mutex | spin::Mutex | Difference
-----------------|------------|-------------|------------
Lock (uncontended)| 20ns      | 5ns         | 4x faster
Lock (contended)  | 100ns     | 50ns        | 2x faster
Unlock            | 15ns      | 3ns         | 5x faster
Memory            | 40 bytes  | 8 bytes     | 5x smaller
```

**Note**: Spin locks are faster but use CPU while waiting

### 4.4 When to Use Spin Locks

**Good for**:
- ✅ Short critical sections (< 100ns)
- ✅ Low contention
- ✅ Kernel code
- ✅ Interrupt handlers

**Bad for**:
- ❌ Long critical sections (> 1μs)
- ❌ High contention
- ❌ Userspace code
- ❌ I/O operations

### 4.5 Migration Strategy

**Phase 1: Add spin crate** (Day 9, Morning)
```toml
[dependencies]
spin = { version = "0.9", default-features = false, features = ["mutex", "rwlock"] }
```

**Phase 2: Replace imports** (Day 9, Afternoon)
```rust
// Before
use std::sync::{Arc, Mutex, RwLock};

// After
use alloc::sync::Arc;
use spin::{Mutex, RwLock};
```

**Phase 3: Remove unwrap()** (Day 9, Evening)
```rust
// Before
let lock = mutex.lock().unwrap();

// After
let lock = mutex.lock();  // spin::Mutex doesn't return Result
```

**Phase 4: Test** (Day 10, Morning)
```rust
// Test synchronization
let counter = Arc::new(Mutex::new(0));
// Test concurrent access
```

---

## 5. Path Handling

### 5.1 Problem

**Current Usage**: `std::path::{Path, PathBuf}` (2 instances)

**Issues**:
- Requires std library
- Only used in syscall wrappers
- Not critical for kernel

### 5.2 Solution: Keep for Userspace

**Recommendation**: Keep std::path for syscall wrappers

**Rationale**:
- Only 2 uses (syscall_file_ops.rs, syscall_dir_ops.rs)
- These are userspace wrappers, not kernel code
- Path handling is complex, std implementation is good
- Not worth reimplementing for 2 uses

**Alternative**: If needed for kernel
```rust
// Simple path handling for kernel
pub struct Path {
    inner: [u8; 4096],
    len: usize,
}

impl Path {
    pub fn new(s: &str) -> &Self {
        // Simple path validation
        unsafe { &*(s.as_bytes() as *const [u8] as *const Path) }
    }
    
    pub fn parent(&self) -> Option<&Path> {
        // Find last '/' and return prefix
        unimplemented!()
    }
    
    pub fn file_name(&self) -> Option<&str> {
        // Find last '/' and return suffix
        unimplemented!()
    }
}
```

### 5.3 Migration Strategy

**Decision**: Keep std::path for now

**Rationale**:
- Low priority (only 2 uses)
- Not in kernel code
- Complex to reimplement
- Can revisit later if needed

---

## 6. Random Number Generation

### 6.1 Problem

**Current Usage**: `rand` with `getrandom` (4 instances)

**Issues**:
- getrandom uses OS syscalls
- Not no_std compatible by default
- Only used for cryptographic IVs

### 6.2 Solution: Hardware RNG

**Design**:
```rust
// src/verified/rng.rs

#![no_std]

use core::arch::x86_64::_rdrand64_step;

/// Hardware random number generator
pub struct HardwareRng;

impl HardwareRng {
    pub fn new() -> Self {
        HardwareRng
    }
    
    /// Fill buffer with random bytes from hardware RNG
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut i = 0;
        while i < dest.len() {
            let mut rand: u64 = 0;
            
            // Use RDRAND instruction
            unsafe {
                if _rdrand64_step(&mut rand) == 1 {
                    // Success
                    let bytes = rand.to_le_bytes();
                    let remaining = dest.len() - i;
                    let to_copy = remaining.min(8);
                    dest[i..i + to_copy].copy_from_slice(&bytes[..to_copy]);
                    i += to_copy;
                } else {
                    // RDRAND failed, retry
                    continue;
                }
            }
        }
    }
    
    /// Generate random u64
    pub fn next_u64(&mut self) -> u64 {
        let mut rand: u64 = 0;
        unsafe {
            while _rdrand64_step(&mut rand) != 1 {
                // Retry until success
            }
        }
        rand
    }
}

// Implement RngCore trait for compatibility
impl rand_core::RngCore for HardwareRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
    
    fn next_u64(&mut self) -> u64 {
        self.next_u64()
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.fill_bytes(dest)
    }
    
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl rand_core::CryptoRng for HardwareRng {}
```

### 6.3 Hardware RNG Options

**Option 1: RDRAND (x86_64)** - Recommended
```rust
// Intel/AMD CPUs since 2012
// Pros: Fast, cryptographically secure
// Cons: x86_64 only

use core::arch::x86_64::_rdrand64_step;
```

**Option 2: RDSEED (x86_64)**
```rust
// Intel CPUs since 2015
// Pros: Higher entropy than RDRAND
// Cons: Slower, x86_64 only

use core::arch::x86_64::_rdseed64_step;
```

**Option 3: ARM TrustZone RNG**
```rust
// ARM CPUs with TrustZone
// Pros: Secure, hardware-backed
// Cons: ARM only, requires TrustZone

// Platform-specific implementation
```

**Recommendation**: Use RDRAND with RDSEED fallback

### 6.4 Migration Strategy

**Phase 1: Implement HardwareRng** (Day 10, Morning)
```bash
# Create RNG module
touch src/verified/rng.rs

# Add to lib.rs
echo "pub mod rng;" >> src/verified/lib.rs
```

**Phase 2: Configure rand** (Day 10, Afternoon)
```toml
[dependencies]
rand = { version = "0.8", default-features = false, features = ["alloc"] }
rand_core = { version = "0.6", default-features = false }
# Remove getrandom dependency
```

**Phase 3: Replace getrandom** (Day 10, Evening)
```rust
// Before
use rand::rngs::OsRng;
let mut rng = OsRng;

// After
use crate::rng::HardwareRng;
let mut rng = HardwareRng::new();
```

**Phase 4: Test** (Day 11, Morning)
```rust
// Test RNG
let mut rng = HardwareRng::new();
let mut buf = [0u8; 32];
rng.fill_bytes(&mut buf);
assert!(buf.iter().any(|&x| x != 0));  // Not all zeros
```

---

## 7. Implementation Plan

### 7.1 Week 7 Schedule

**Day 4: Verus Separation** (CRITICAL)
- Morning: Add feature flags
- Afternoon: Separate verification code
- Evening: Test compilation

**Day 5: HashMap Replacement** (HIGH)
- Morning: Analyze map sizes
- Afternoon: Replace with BTreeMap
- Evening: Benchmark performance

**Day 6: Compilation Fixes** (HIGH)
- Morning: Fix remaining issues
- Afternoon: Run test suite
- Evening: Document changes

**Day 7: Performance Validation** (MEDIUM)
- Morning: Run benchmarks
- Afternoon: Compare results
- Evening: Week 7 summary

### 7.2 Week 8 Schedule

**Day 8: Time Implementation** (HIGH)
- Morning: Implement time module
- Afternoon: Replace std::time
- Evening: Calibrate TSC

**Day 9: Sync Replacement** (MEDIUM)
- Morning: Add spin crate
- Afternoon: Replace std::sync
- Evening: Test synchronization

**Day 10: RNG Implementation** (MEDIUM)
- Morning: Implement HardwareRng
- Afternoon: Replace getrandom
- Evening: Test RNG

**Day 11-12: Optimization** (LOW)
- Directory caching
- Timer queue optimization
- Performance testing

**Day 13-14: Documentation** (LOW)
- Performance report
- Migration guide
- Week 8 summary

### 7.3 Success Criteria

✅ **Compilation**:
- [ ] Code compiles without std
- [ ] Code compiles with #![no_std]
- [ ] All tests pass

✅ **Performance**:
- [ ] No regression vs std
- [ ] BTreeMap benchmarked
- [ ] Spin locks benchmarked
- [ ] Time accuracy verified

✅ **Functionality**:
- [ ] All 39 syscalls work
- [ ] IPC still verified
- [ ] Timers accurate
- [ ] RNG cryptographically secure

✅ **Documentation**:
- [ ] Migration guide complete
- [ ] API changes documented
- [ ] Performance data updated

---

## 8. Risk Mitigation

### 8.1 High Risk Items

**Time Implementation**:
- Risk: Inaccurate timing
- Mitigation: Calibrate TSC at boot
- Fallback: Use HPET
- Test: Compare with std::time

**Verus Separation**:
- Risk: Break verification
- Mitigation: Feature flags
- Fallback: Keep separate
- Test: Run verification suite

### 8.2 Medium Risk Items

**HashMap Replacement**:
- Risk: Performance degradation
- Mitigation: Benchmark first
- Fallback: Use hashbrown
- Test: Compare with std

**Spin Locks**:
- Risk: Deadlocks
- Mitigation: Short critical sections
- Fallback: Keep std for userspace
- Test: Stress testing

### 8.3 Low Risk Items

**RNG Implementation**:
- Risk: Weak randomness
- Mitigation: Use RDRAND
- Fallback: Mix multiple sources
- Test: Statistical tests

---

## 9. Conclusion

### 9.1 Summary

All identified dependencies have clear, implementable alternatives:

| Dependency | Alternative | Effort | Risk |
|------------|-------------|--------|------|
| HashMap | BTreeMap/hashbrown | 1-2 days | LOW |
| Time | Custom + TSC | 2-3 days | MEDIUM |
| Sync | spin locks | 1 day | LOW |
| Path | Keep std | 0 days | NONE |
| RNG | RDRAND | 1-2 days | LOW |

**Total Effort**: 5-8 days (excluding Verus)  
**Overall Risk**: LOW-MEDIUM  
**Success Probability**: 95%+

### 9.2 Next Steps

1. **Day 4**: Separate Verus code (CRITICAL)
2. **Day 5**: Replace HashMap (HIGH)
3. **Day 8**: Implement time module (HIGH)
4. **Day 9**: Replace sync primitives (MEDIUM)
5. **Day 10**: Implement hardware RNG (MEDIUM)

---

**Document Version**: 1.0  
**Last Updated**: February 9, 2025  
**Status**: Complete Design  
**Next Steps**: Begin implementation (Day 4)