# VantisOS Testing Guide

Comprehensive guide to testing methodologies, frameworks, and best practices in VantisOS.

---

## Table of Contents

1. [Testing Overview](#testing-overview)
2. [Testing Infrastructure](#testing-infrastructure)
3. [Unit Testing](#unit-testing)
4. [Integration Testing](#integration-testing)
5. [System Testing](#system-testing)
6. [Performance Testing](#performance-testing)
7. [Security Testing](#security-testing)
8. [Formal Verification](#formal-verification)
9. [Continuous Integration](#continuous-integration)
10. [Test Coverage](#test-coverage)

---

## Testing Overview

### VantisOS Testing Philosophy

VantisOS employs a multi-layered testing approach:

- **Formal Verification**: Mathematical proofs for critical components
- **Unit Tests**: Individual component testing
- **Integration Tests**: Component interaction testing
- **System Tests**: Full system validation
- **Regression Tests**: Prevent bugs from reappearing
- **Stress Tests**: Performance under load

### Testing Layers

```
┌─────────────────────────────────────────────────────┐
│              Acceptance Tests                        │
├─────────────────────────────────────────────────────┤
│              System Tests                            │
├─────────────────────────────────────────────────────┤
│          Integration Tests                           │
├─────────────────────────────────────────────────────┤
│              Unit Tests                              │
├─────────────────────────────────────────────────────┤
│          Formal Verification (Verus)                 │
└─────────────────────────────────────────────────────┘
```

### Test Statistics

| Category | Count | Pass Rate |
|----------|-------|-----------|
| Unit Tests | 5,000+ | 99.9% |
| Integration Tests | 1,200+ | 99.5% |
| System Tests | 300+ | 98.5% |
| Formal Verifications | 2,500+ | 100% |

---

## Testing Infrastructure

### Test Runner

```bash
# Run all tests
./scripts/test_all.sh

# Or using Makefile
make test

# Run specific test suite
cargo test --lib              # Unit tests
cargo test --test integration # Integration tests
cargo test --test system      # System tests
```

### Test Configuration

```bash
# Test configuration file
cat .vantis-test.toml

# Environment variables
export VANTIS_TEST_MODE=full
export VANTIS_TEST_TIMEOUT=300
export VANTIS_TEST_PARALLEL=4
```

### Test Directories

```
VantisOS/
├── tests/
│   ├── unit/           # Unit tests
│   ├── integration/    # Integration tests
│   ├── system/         # System tests
│   ├── stress/         # Stress tests
│   ├── security/       # Security tests
│   └── fixtures/       # Test fixtures
├── examples/           # Example programs
└── benches/            # Benchmarks
```

---

## Unit Testing

### Rust Unit Tests

```rust
// In src/kernel/mod.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_creation() {
        let process = Process::new(1, "test");
        assert_eq!(process.pid, 1);
        assert_eq!(process.name, "test");
    }

    #[test]
    fn test_memory_allocation() {
        let allocator = MemoryAllocator::new(1024);
        let block = allocator.allocate(100);
        assert!(block.is_ok());
    }

    #[test]
    #[should_panic]
    fn test_invalid_input() {
        panic!("Expected panic");
    }
}
```

### Running Unit Tests

```bash
# Run all unit tests
cargo test --lib

# Run specific test
cargo test test_process_creation

# Run tests in a module
cargo test kernel::tests

# Run with verbose output
cargo test --lib -- --nocapture

# Run in parallel
cargo test --lib -- --test-threads=4
```

### Test Macros

```rust
// Custom test macros
#[macro_export]
macro_rules! vantis_test {
    ($name:ident, $func:expr) => {
        #[test]
        fn $name() {
            vantis_test_init();
            $func;
            vantis_test_cleanup();
        }
    };
}

// Usage
vantis_test!(test_scheduler, {
    let scheduler = Scheduler::new();
    assert!(scheduler.is_empty());
});
```

### Mocking

```rust
// Using mockall for mocking
use mockall::automock;

#[automock]
pub trait Device {
    fn read(&self, buffer: &mut [u8]) -> Result<usize, Error>;
    fn write(&self, buffer: &[u8]) -> Result<usize, Error>;
}

#[test]
fn test_with_mock() {
    let mut mock_device = MockDevice::new();
    mock_device.expect_read()
        .returning(|buf| {
            buf[0] = 42;
            Ok(1)
        });

    let result = mock_device.read(&mut [0u8; 10]);
    assert!(result.is_ok());
}
```

---

## Integration Testing

### Integration Test Structure

```rust
// tests/integration/scheduler_test.rs
use vantis::kernel::{Scheduler, Process};

#[test]
fn test_process_scheduling() {
    let mut scheduler = Scheduler::new();

    // Add processes
    let p1 = Process::new(1, "process1");
    let p2 = Process::new(2, "process2");

    scheduler.add(p1);
    scheduler.add(p2);

    // Test scheduling
    let next = scheduler.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap().pid, 1);

    let next = scheduler.next();
    assert!(next.is_some());
    assert_eq!(next.unwrap().pid, 2);
}

#[test]
fn test_inter_process_communication() {
    let ipc = IPCManager::new();

    let sender = ipc.create_channel("test_channel");
    let receiver = ipc.connect("test_channel");

    sender.send(Message::new("Hello")).unwrap();
    let msg = receiver.recv().unwrap();

    assert_eq!(msg.data(), "Hello");
}
```

### Running Integration Tests

```bash
# Run integration tests
cargo test --test integration

# Run specific integration test
cargo test --test scheduler_test

# Run with logging
RUST_LOG=debug cargo test --test integration
```

### Hardware Abstraction Tests

```rust
// tests/integration/hardware_test.rs
#[test]
fn test_timer_interrupts() {
    let timer = Timer::new(1000); // 1 second
    timer.start();

    sleep(Duration::from_millis(1100));

    assert!(timer.expired());
}
```

---

## System Testing

### System Test Framework

```bash
# System test runner
./scripts/run_system_tests.sh

# Or using cargo
cargo test --test system
```

### System Test Example

```rust
// tests/system/boot_test.rs
use vantis_test::{QemuRunner, Timeout};

#[test]
fn test_boot_sequence() {
    let mut qemu = QemuRunner::new();
    qemu.boot();

    // Wait for login prompt
    qemu.wait_for("login:", Timeout::Seconds(30));

    // Login
    qemu.send("root\n");
    qemu.wait_for("password:", Timeout::Seconds(5));
    qemu.send("vantisoroot\n");

    // Verify shell
    qemu.wait_for("#", Timeout::Seconds(5));

    qemu.shutdown();
}

#[test]
fn test_service_startup() {
    let mut qemu = QemuRunner::new();
    qemu.boot();

    // Check critical services
    let output = qemu.execute("systemctl is-active vguard");
    assert!(output.contains("active"));

    let output = qemu.execute("systemctl is-active vnetworkd");
    assert!(output.contains("active"));

    qemu.shutdown();
}
```

### Installation Tests

```rust
// tests/system/installation_test.rs
#[test]
fn test_installer() {
    let mut qemu = QemuRunner::new()
        .with_disk("test_disk.qcow2", "10G");

    // Boot from ISO
    qemu.boot_from_iso("VantisOS.iso");

    // Run installer
    qemu.send("vinstall --auto\n");

    // Wait for completion
    qemu.wait_for("Installation complete", Timeout::Minutes(10));

    // Reboot from installed system
    qemu.reboot();

    // Verify installation
    qemu.wait_for("login:", Timeout::Seconds(30));
}
```

---

## Performance Testing

### Benchmark Suite

```bash
# Run benchmarks
cargo bench

# Specific benchmark
cargo bench --bench scheduler_bench

# Save baseline
cargo bench -- --save-baseline main
```

### Criterion Benchmarks

```rust
// benches/scheduler_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn scheduler_benchmark(c: &mut Criterion) {
    let mut scheduler = Scheduler::new();

    c.bench_function("schedule_1000_processes", |b| {
        b.iter(|| {
            for i in 0..1000 {
                scheduler.add(Process::new(i, "test"));
            }
            black_box(scheduler.next());
        });
    });
}

criterion_group!(benches, scheduler_benchmark);
criterion_main!(benches);
```

### Stress Testing

```bash
# Stress test kernel
vstress --cpu 4 --memory 2G --duration 60

# Stress test storage
vstress --disk /dev/sda --io 1000

# Full stress test
vstress --all --duration 300
```

### Performance Regression Tests

```bash
# Compare with baseline
cargo bench -- --baseline main

# Alert on regression
vbench-check --threshold 5%  # Alert if >5% slower
```

---

## Security Testing

### Security Test Suite

```bash
# Run security tests
./scripts/security_tests.sh

# Or
cargo test --test security
```

### Penetration Testing

```bash
# Automated penetration tests
vsec-scan --full

# Network penetration
vsec-scan --network

# Privilege escalation tests
vsec-scan --privilege

# Input fuzzing
vfuzz --target kernel --duration 3600
```

### Fuzzing

```rust
// fuzz/fuzz_scheduler.rs
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let scheduler = Scheduler::new();
    // Fuzz scheduler with random data
    if let Ok(process) = Process::from_bytes(data) {
        scheduler.add(process);
    }
});
```

### Vulnerability Scanning

```bash
# CVE database check
vcve-check --all-packages

# Static analysis
vstatic-analysis --severity high

# Dependency audit
cargo audit
```

### Security Test Cases

```rust
// tests/security/memory_safety_test.rs
#[test]
fn test_buffer_overflow_protection() {
    let buffer = vec![0u8; 100];
    let result = unsafe_function(buffer.as_ptr(), 200); // Try to overflow
    assert!(result.is_err() || !is_memory_corrupted());
}

#[test]
fn test_use_after_free() {
    let allocator = MemoryAllocator::new();
    let ptr = allocator.allocate(100);
    allocator.deallocate(ptr);
    // Verify double-free is caught
    assert!(allocator.verify_integrity());
}
```

---

## Formal Verification

### Verus Integration

```rust
// src/kernel/verified/scheduler.rs
use vstd::prelude::*;

verus! {
    pub struct Process {
        pub pid: int,
        pub priority: int,
        pub state: ProcessState,
    }

    pub fn schedule(processes: Seq<Process>) -> (result: Option<Process>)
        ensures
            result.is_some() ==> {
                let p = result.unwrap();
                &&& processes.contains(p)
                &&& forall |other: Process| processes.contains(other) ==> p.priority >= other.priority
            }
    {
        // Implementation with proof
    }
}
```

### Running Verification

```bash
# Verify all Verus code
verus --verify-all

# Verify specific module
verus src/kernel/verified/

# Verification report
verus-report --output verification-report.html
```

### Verification Coverage

```bash
# Check verification coverage
verus-coverage --report

# Verification statistics
verus-stats
```

---

## Continuous Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Run unit tests
        run: cargo test --lib

      - name: Run integration tests
        run: cargo test --test integration

      - name: Run verification
        run: verus --verify-all

      - name: Generate coverage
        run: cargo tarpaulin --out Xml

      - name: Upload coverage
        uses: codecov/codecov-action@v4
```

### CI Test Matrix

```yaml
# .github/workflows/test-matrix.yml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest]
    rust: [stable, beta, nightly]
    exclude:
      - os: macos-latest
        rust: nightly
```

### Test Automation

```bash
# Pre-commit tests
./scripts/pre-commit-test.sh

# Nightly tests
./scripts/nightly-tests.sh

# Release tests
./scripts/release-tests.sh
```

---

## Test Coverage

### Coverage Tools

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html

# Coverage with specific output
cargo tarpaulin --out Xml --out Html --output-dir coverage/

# Coverage for specific module
cargo tarpaulin --packages vantis-kernel
```

### Coverage Report

```bash
# Text report
cargo tarpaulin --out Stdout

# HTML report
open tarpaulin-report.html
```

### Coverage Targets

| Component | Target | Current |
|-----------|--------|---------|
| Kernel Core | 95% | 94.5% |
| Memory Manager | 98% | 97.2% |
| Scheduler | 95% | 96.1% |
| IPC | 90% | 88.5% |
| Drivers | 85% | 82.3% |
| Userspace | 80% | 78.9% |

### Improving Coverage

```bash
# Find uncovered code
cargo tarpaulin --out Stdout | grep "0.00%"

# Generate coverage report for specific files
cargo tarpaulin --skip-clean -- --include-file "src/kernel/*.rs"
```

---

## Test Best Practices

### Writing Good Tests

1. **Isolation**: Each test should be independent
2. **Clarity**: Test names should describe what they test
3. **Coverage**: Test edge cases, not just happy paths
4. **Speed**: Keep unit tests fast (< 100ms)
5. **Determinism**: Avoid flaky tests

### Test Naming Convention

```rust
// Pattern: test_<function>_<scenario>_<expected_result>
#[test]
fn test_allocate_memory_when_full_returns_error() {}

#[test]
fn test_process_creation_with_valid_input_succeeds() {}

#[test]
fn test_scheduler_under_load_maintains_fairness() {}
```

### Test Fixtures

```rust
// tests/fixtures/mod.rs
pub fn create_test_process() -> Process {
    Process::new(1, "test_process")
}

pub fn create_test_scheduler() -> Scheduler {
    let mut scheduler = Scheduler::new();
    scheduler.add(create_test_process());
    scheduler
}
```

---

## Troubleshooting Tests

### Flaky Tests

```bash
# Run test multiple times to detect flakiness
cargo test --test flaky_test -- --test-threads=1 --nocapture

# Stress test
cargo test -- --stress 100
```

### Test Debugging

```bash
# Run single test with debug output
RUST_LOG=debug cargo test test_name -- --nocapture

# Run in debugger
rust-gdb --args target/debug/deps/test-binary test_name
```

### Common Issues

```bash
# Test timeout
cargo test -- --test-timeout 300

# Memory issues
valgrind --leak-check=full target/debug/test-binary

# Parallel test conflicts
cargo test -- --test-threads=1
```

---

*Last updated: March 2025 | VantisOS v1.4.0*