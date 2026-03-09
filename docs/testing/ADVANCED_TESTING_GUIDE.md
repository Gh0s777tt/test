# Advanced Testing Guide for VantisOS

## Overview

This guide provides comprehensive documentation for testing the VantisOS kernel and userspace applications. It covers unit testing, integration testing, code coverage, property-based testing, fuzzing, and quality assurance automation.

## Table of Contents

1. [Testing Philosophy](#testing-philosophy)
2. [Testing Infrastructure](#testing-infrastructure)
3. [Unit Testing](#unit-testing)
4. [Integration Testing](#integration-testing)
5. [Code Coverage](#code-coverage)
6. [Property-Based Testing](#property-based-testing)
7. [Fuzz Testing](#fuzz-testing)
8. [Benchmarking](#benchmarking)
9. [Quality Gates](#quality-gates)
10. [CI/CD Integration](#cicd-integration)
11. [Troubleshooting](#troubleshooting)

---

## Testing Philosophy

### Core Principles

1. **Test Early, Test Often**: Write tests alongside code development
2. **Coverage Matters**: Maintain high code coverage (>70% target)
3. **Fast Feedback**: Keep unit tests fast (<1 second per test)
4. **Clear Intent**: Test names and assertions should be self-documenting
5. **Isolation**: Tests should be independent and repeatable

### Testing Pyramid

```
        /\
       /  \      E2E Tests (10%)
      /____\     Integration Tests (20%)
     /      \    Unit Tests (70%)
    /________\   Property-based & Fuzz Tests
```

---

## Testing Infrastructure

### Required Tools

```bash
# Install testing tools
cargo install cargo-tarpaulin   # Code coverage
cargo install cargo-fuzz        # Fuzz testing
cargo install proptest          # Property-based testing
cargo install cargo-nextest     # Faster test runner
```

### Configuration Files

- `tarpaulin.toml` - Code coverage configuration
- `tests/test_config.toml` - General testing configuration
- `.github/workflows/test.yml` - CI/CD test pipeline

### Directory Structure

```
VantisOS/
├── tests/                    # Integration tests
│   ├── integration/         # Integration test suites
│   ├── fuzz/               # Fuzz test targets
│   └── test_config.toml    # Test configuration
├── benches/                # Performance benchmarks
├── scripts/
│   └── test_runner.sh      # Comprehensive test runner
└── docs/
    └── testing/           # Testing documentation
```

---

## Unit Testing

### Running Unit Tests

```bash
# Run all unit tests
cargo test --lib

# Run tests for a specific crate
cargo test -p kernel

# Run tests with output
cargo test --lib -- --nocapture

# Run tests with filtering
cargo test --lib test_name

# Run tests in parallel (faster)
cargo nextest run --lib
```

### Writing Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_initialization() {
        let result = initialize_kernel();
        assert!(result.is_ok(), "Kernel initialization should succeed");
    }

    #[test]
    fn test_memory_allocation() {
        let size = 1024;
        let ptr = allocate_memory(size);
        assert!(!ptr.is_null(), "Memory allocation should succeed");
        assert_eq!(allocated_size(ptr), size, "Size should match requested");
    }
}
```

### Best Practices

1. **Use descriptive test names**: `test_kernel_initialization` not `test_1`
2. **One assertion per test**: Multiple assertions can hide failures
3. **Arrange-Act-Assert pattern**: Structure tests clearly
4. **Use test fixtures**: Set up and tear down common test data
5. **Mock dependencies**: Isolate the code under test

---

## Integration Testing

### Running Integration Tests

```bash
# Run all integration tests
cargo test --test '*'

# Run specific integration test
cargo test --test integration_test_name

# Run integration tests with timeout
cargo test --test '*' -- --test-threads=1
```

### Writing Integration Tests

```rust
// tests/integration/kernel_boot_test.rs
#[test]
fn test_full_kernel_boot() {
    // Boot the kernel
    let kernel = Kernel::new();
    kernel.boot().expect("Kernel boot should succeed");

    // Verify boot sequence
    assert!(kernel.is_running(), "Kernel should be running");
    assert!(kernel.has_initialized_memory(), "Memory should be initialized");
    assert!(kernel.has_loaded_drivers(), "Drivers should be loaded");

    // Cleanup
    kernel.shutdown();
}
```

### Test Organization

```
tests/
├── integration/
│   ├── kernel_boot_test.rs
│   ├── process_management_test.rs
│   └── ipc_test.rs
└── common/
    └── test_helpers.rs
```

---

## Code Coverage

### Generating Coverage Reports

```bash
# Using the test runner script
./scripts/test_runner.sh --coverage

# Using tarpaulin directly
cargo tarpaulin --out Html --out Lcov --out Xml

# Generate coverage for specific crate
cargo tarpaulin -p kernel --out Html
```

### Coverage Reports

- **HTML Report**: `target/tarpaulin/index.html`
- **Lcov Format**: `target/tarpaulin/lcov.info`
- **XML Format**: `target/tarpaulin/cobertura.xml`

### Coverage Targets

| Metric | Target | Current |
|--------|--------|---------|
| Line Coverage | 70% | - |
| Branch Coverage | 60% | - |
| Function Coverage | 75% | - |

### Improving Coverage

1. **Run coverage analysis**: `cargo tarpaulin --show-missing-lines`
2. **Identify gaps**: Review uncovered lines
3. **Add tests**: Write tests for uncovered code
4. **Verify improvement**: Re-run coverage analysis

---

## Property-Based Testing

### What is Property-Based Testing?

Unlike example-based testing (checking specific inputs), property-based testing verifies that a property holds for **many** randomly generated inputs.

### Running Property Tests

```bash
# Run property tests
cargo test --features proptest

# Run with increased test cases
PROPTEST_CASES=1000 cargo test --features proptest

# Verbose output
cargo test --features proptest -- --nocapture
```

### Writing Property Tests

```rust
#[cfg(test)]
mod proptest_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_addition_commutative(a in any::<i32>(), b in any::<i32>()) {
            // Property: Addition is commutative
            assert_eq!(a + b, b + a, "Addition should be commutative");
        }

        #[test]
        fn test_memory_allocation_roundtrip(size in 1usize..1024) {
            // Property: Memory allocation returns usable memory
            let ptr = allocate_memory(size);
            prop_assert!(!ptr.is_null(), "Allocation should succeed");
            deallocate_memory(ptr);
        }
    }
}
```

### Common Properties

1. **Commutativity**: `f(a, b) == f(b, a)`
2. **Associativity**: `f(a, f(b, c)) == f(f(a, b), c)`
3. **Identity**: `f(identity, x) == x`
4. **Idempotency**: `f(f(x)) == f(x)`
5. **Round-trip**: `decode(encode(x)) == x`

---

## Fuzz Testing

### What is Fuzz Testing?

Fuzz testing automatically generates random inputs to find bugs, edge cases, and security vulnerabilities.

### Running Fuzz Tests

```bash
# Run fuzz tests
cargo fuzz run fuzz_target_name

# Run with custom settings
cargo fuzz run fuzz_target_name -- -max_total_time=300

# List fuzz targets
cargo fuzz list
```

### Creating Fuzz Targets

```rust
// fuzz/fuzz_targets/memory_management.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Parse input as size and content
    if data.len() < 4 {
        return;
    }
    
    let size = u32::from_be_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let content = &data[4..];
    
    // Test memory allocation with various sizes
    if size < 1024 * 1024 { // Limit to reasonable sizes
        let ptr = allocate_memory(size);
        if !ptr.is_null() {
            // Write content
            unsafe {
                std::ptr::copy_nonoverlapping(content.as_ptr(), ptr, 
                    std::cmp::min(content.len(), size));
            }
            deallocate_memory(ptr);
        }
    }
});
```

### Fuzzing Best Practices

1. **Limit input sizes**: Prevent memory exhaustion
2. **Handle all cases**: Don't panic on invalid input
3. **Use sanitizers**: Enable AddressSanitizer for memory bugs
4. **Corpus management**: Save interesting inputs for replay

---

## Benchmarking

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench benchmark_name

# Save baseline for comparison
cargo bench --save-baseline baseline

# Compare with baseline
cargo bench --baseline baseline
```

### Writing Benchmarks

```rust
// benches/memory_allocation.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn benchmark_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");
    
    for size in [64, 128, 256, 512, 1024].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let ptr = allocate_memory(black_box(size));
                deallocate_memory(ptr);
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_memory_allocation);
criterion_main!(benches);
```

### Performance Thresholds

Configure performance thresholds in `tests/test_config.toml`:

```toml
[benchmarking]
enabled = true
baseline_commit = "main"
threshold_percent = 10.0  # Fail if performance degrades > 10%
```

---

## Quality Gates

### Quality Gate Checks

Quality gates enforce minimum standards before code can be merged:

```bash
# Run all quality gate checks
./scripts/test_runner.sh --all

# Check specific quality metrics
cargo clippy -- -D warnings
cargo fmt -- --check
cargo tarpaulin --fail-under 70
```

### Quality Metrics

| Metric | Target | Status |
|--------|--------|--------|
| Clippy Warnings | 0 | ⏳ |
| Code Coverage | 70% | ⏳ |
| Compile Warnings | 0 | ⏳ |
| Documentation | 100% | ⏳ |

### Enforcing Quality Gates

Quality gates are enforced in CI/CD pipelines. Pull requests must pass all quality checks before merging.

---

## CI/CD Integration

### GitHub Actions Workflow

The `.github/workflows/test.yml` file runs all tests automatically on every push and pull request.

```yaml
name: Test Suite
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - name: Run tests
        run: ./scripts/test_runner.sh --all
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

### Local Testing Before Push

```bash
# Run full test suite locally
./scripts/test_runner.sh --all

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings
```

---

## Troubleshooting

### Common Issues

#### 1. Tests Time Out

**Problem**: Tests take too long to complete

**Solution**:
```bash
# Increase timeout in tarpaulin.toml
[config]
timeout = 600
```

#### 2. Coverage Not Generated

**Problem**: tarpaulin fails to generate coverage

**Solution**:
```bash
# Reinstall tarpaulin
cargo install cargo-tarpaulin --force

# Try different engine
cargo tarpaulin --engine Ptrace
```

#### 3. Fuzz Tests Find Bugs

**Problem**: Fuzz testing finds crashes or panics

**Solution**:
1. Review the failing input in `fuzz/artifacts/`
2. Create a minimal reproducer
3. Fix the bug
4. Add regression test
5. Re-run fuzz tests

#### 4. Property Tests Fail

**Problem**: Property tests fail with complex counterexamples

**Solution**:
1. Print the failing input using `proptest::prop_assert_eq!`
2. Simplify the input manually
3. Identify the root cause
4. Update code or adjust property

### Getting Help

- **Documentation**: Check `docs/testing/` for detailed guides
- **Issues**: Report bugs in GitHub issues
- **Community**: Join the VantisOS Discord community

---

## Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Proptest Documentation](https://altsysrq.github.io/proptest-book/intro.html)
- [Tarpaulin Documentation](https://github.com/xd009642/tarpaulin)
- [Rust-Fuzz Documentation](https://rust-fuzz.github.io/)

---

**Last Updated**: 2025-03-06
**Version**: 0.4.1