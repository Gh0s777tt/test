# 🚀 VANTIS OS - Developer Onboarding Guide

Welcome to the VANTIS OS development team! This guide will help you get started with contributing to the project.

## 📋 Table of Contents

- [Prerequisites](#prerequisites)
- [Development Environment Setup](#development-environment-setup)
- [Building VANTIS OS](#building-vantis-os)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Testing](#testing)
- [Formal Verification](#formal-verification)
- [Documentation](#documentation)
- [Getting Help](#getting-help)

---

## Prerequisites

### Required Knowledge

- **Rust Programming**: Intermediate to advanced level
- **Operating Systems**: Understanding of OS concepts (processes, memory, IPC)
- **Systems Programming**: Low-level programming experience
- **Git**: Version control basics

### Recommended Knowledge

- **Formal Verification**: Experience with proof systems (helpful but not required)
- **Cryptography**: Understanding of encryption algorithms
- **Kernel Development**: Linux/Unix kernel experience
- **Assembly**: x86_64 assembly basics

---

## Development Environment Setup

### 1. Install Rust Toolchain

```bash
# Install rustup (Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install nightly toolchain (required for some features)
rustup toolchain install nightly
rustup default nightly

# Add required targets
rustup target add x86_64-unknown-redox
rustup target add x86_64-unknown-linux-gnu
```

### 2. Install System Dependencies

#### Debian/Ubuntu
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    nasm \
    qemu-system-x86 \
    git \
    curl \
    pkg-config \
    libssl-dev \
    cmake \
    ninja-build
```

#### macOS
```bash
brew install nasm qemu cmake ninja
```

#### Arch Linux
```bash
sudo pacman -S base-devel nasm qemu cmake ninja
```

### 3. Install Formal Verification Tools

```bash
# Install Verus (formal verification for Rust)
cargo install --git https://github.com/verus-lang/verus verus

# Install Kani (model checker for Rust)
cargo install --locked kani-verifier
cargo kani setup
```

### 4. Clone the Repository

```bash
# Clone VANTIS OS repository
git clone https://github.com/vantisCorp/VantisOS.git
cd VantisOS

# Initialize submodules
git submodule update --init --recursive
```

### 5. Configure Git

```bash
# Set your identity
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Set up commit signing (recommended)
git config commit.gpgsign true
```

---

## Building VANTIS OS

### Quick Build

```bash
# Build the entire system
make all

# Build specific components
make kernel      # Build kernel only
make userspace   # Build userspace only
make drivers     # Build drivers only
```

### Running in QEMU

```bash
# Run with default settings
make qemu

# Run with specific memory
make qemu MEMORY=2048

# Run with debugging enabled
make qemu DEBUG=1

# Run with KVM acceleration (Linux only)
make qemu KVM=1
```

### Build Configurations

```bash
# Debug build (default)
make CONFIG=debug

# Release build (optimized)
make CONFIG=release

# Formal verification build
make CONFIG=verify
```

---

## Development Workflow

### 1. Create a Feature Branch

```bash
# Update main branch
git checkout 0.4.1
git pull origin 0.4.1

# Create feature branch
git checkout -b feature/your-feature-name
```

### 2. Make Changes

Follow the [Code Standards](#code-standards) section below.

### 3. Test Your Changes

```bash
# Run unit tests
cargo test

# Run integration tests
make test

# Run formal verification (if applicable)
make verify
```

### 4. Commit Your Changes

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "feat: Add your feature description

- Detailed change 1
- Detailed change 2
- Fixes #issue-number"
```

### 5. Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create pull request via GitHub CLI
gh pr create --title "feat: Your feature title" \
             --body "Description of your changes" \
             --base 0.4.1
```

---

## Code Standards

### Rust Style Guide

We follow the official Rust style guide with some additions:

```rust
// Use rustfmt for automatic formatting
cargo fmt

// Check for common mistakes
cargo clippy -- -D warnings

// Example: Good code style
pub fn process_data(input: &[u8]) -> Result<Vec<u8>, ProcessError> {
    // Clear documentation
    /// Process input data and return transformed output
    
    // Descriptive variable names
    let buffer_size = input.len();
    let mut output = Vec::with_capacity(buffer_size);
    
    // Error handling
    if buffer_size == 0 {
        return Err(ProcessError::EmptyInput);
    }
    
    // Implementation
    for &byte in input {
        output.push(byte.wrapping_add(1));
    }
    
    Ok(output)
}
```

### Naming Conventions

- **Functions**: `snake_case`
- **Types**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`

```rust
// Good examples
const MAX_BUFFER_SIZE: usize = 4096;
struct MemoryRegion { /* ... */ }
fn allocate_memory() -> Result<*mut u8, AllocError> { /* ... */ }
mod memory_manager;
```

### Documentation

All public APIs must be documented:

```rust
/// Allocates memory with specified size and alignment.
///
/// # Arguments
///
/// * `size` - The number of bytes to allocate
/// * `align` - The alignment requirement (must be power of 2)
///
/// # Returns
///
/// Returns a pointer to the allocated memory or an error.
///
/// # Safety
///
/// The caller must ensure the pointer is properly deallocated.
///
/// # Examples
///
/// ```
/// let ptr = allocate_memory(1024, 16)?;
/// // Use the memory
/// unsafe { deallocate_memory(ptr, 1024); }
/// ```
pub unsafe fn allocate_memory(size: usize, align: usize) 
    -> Result<*mut u8, AllocError> {
    // Implementation
}
```

### Error Handling

Use `Result` types for fallible operations:

```rust
// Define custom error types
#[derive(Debug)]
pub enum KernelError {
    OutOfMemory,
    InvalidParameter,
    PermissionDenied,
}

// Use Result for error handling
pub fn kernel_operation() -> Result<(), KernelError> {
    // Check conditions
    if !has_permission() {
        return Err(KernelError::PermissionDenied);
    }
    
    // Perform operation
    Ok(())
}
```

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_allocation() {
        let result = allocate_memory(1024, 16);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_alignment() {
        let result = allocate_memory(1024, 3); // Not power of 2
        assert!(result.is_err());
    }
}
```

### Integration Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_memory_allocation

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release
```

### Benchmarks

```rust
#[bench]
fn bench_memory_allocation(b: &mut Bencher) {
    b.iter(|| {
        let ptr = allocate_memory(1024, 16).unwrap();
        unsafe { deallocate_memory(ptr, 1024); }
    });
}
```

---

## Formal Verification

### Using Verus

```rust
use verus::prelude::*;

verus! {
    // Specify function contracts
    pub fn add_numbers(a: u32, b: u32) -> (result: u32)
        requires a + b <= u32::MAX,
        ensures result == a + b,
    {
        a + b
    }
    
    // Prove properties
    proof fn prove_addition_commutative(a: u32, b: u32)
        requires a + b <= u32::MAX,
        ensures add_numbers(a, b) == add_numbers(b, a),
    {
        // Proof implementation
    }
}
```

### Using Kani

```rust
#[kani::proof]
fn verify_memory_safety() {
    let size: usize = kani::any();
    kani::assume(size > 0 && size < 1024);
    
    let ptr = allocate_memory(size, 16);
    assert!(ptr.is_ok());
}
```

### Running Verification

```bash
# Verify with Verus
verus src/kernel/memory.rs

# Verify with Kani
cargo kani --harness verify_memory_safety

# Run all verification
make verify
```

---

## Documentation

### Writing Documentation

- Use clear, concise language
- Include code examples
- Document edge cases and error conditions
- Keep documentation up to date with code changes

### Building Documentation

```bash
# Build API documentation
cargo doc --no-deps --open

# Build all documentation
make docs

# Check documentation coverage
cargo doc --no-deps 2>&1 | grep "warning"
```

### Documentation Structure

```
docs/
├── API_DOCUMENTATION.md      # API reference
├── ARCHITECTURE.md            # System architecture
├── DEVELOPER_ONBOARDING.md   # This file
├── USER_MANUAL.md            # User guide
└── CONTRIBUTING.md           # Contribution guidelines
```

---

## Getting Help

### Resources

- 📚 **Documentation**: https://docs.vantis.dev
- 💬 **Discord**: https://discord.gg/vantis
- 🐦 **Twitter**: https://twitter.com/VantisOS
- 📧 **Email**: dev@vantis.dev

### Common Issues

#### Build Failures

```bash
# Clean build artifacts
make clean

# Update dependencies
cargo update

# Rebuild from scratch
make clean && make all
```

#### QEMU Issues

```bash
# Check QEMU installation
qemu-system-x86_64 --version

# Run with verbose output
make qemu VERBOSE=1
```

#### Verification Failures

```bash
# Check Verus installation
verus --version

# Run with detailed output
verus --verbose src/kernel/memory.rs
```

### Asking Questions

When asking for help:

1. **Search existing issues** on GitHub
2. **Provide context**: What are you trying to do?
3. **Include error messages**: Copy the full error output
4. **Share code**: Provide a minimal reproducible example
5. **Describe what you've tried**: What solutions have you attempted?

---

## Development Tips

### Performance Optimization

```rust
// Use inline for hot paths
#[inline(always)]
pub fn fast_operation() { /* ... */ }

// Use const for compile-time computation
const BUFFER_SIZE: usize = 4096;

// Avoid allocations in hot paths
fn process_data(buffer: &mut [u8]) { /* ... */ }
```

### Debugging

```bash
# Run with GDB
make qemu-gdb

# In another terminal
gdb target/x86_64-unknown-redox/debug/vantis
(gdb) target remote :1234
(gdb) break kernel_main
(gdb) continue
```

### Profiling

```bash
# Profile with perf (Linux)
perf record -g make qemu
perf report

# Profile with Instruments (macOS)
instruments -t "Time Profiler" make qemu
```

---

## Next Steps

1. **Read the Architecture**: Review [ARCHITECTURE.md](ARCHITECTURE.md)
2. **Check Open Issues**: Find issues labeled "good first issue"
3. **Join Discord**: Connect with other developers
4. **Start Small**: Begin with documentation or tests
5. **Ask Questions**: Don't hesitate to ask for help

---

## Code of Conduct

Please read and follow our [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md).

---

## License

VANTIS OS is licensed under the MIT License. See [LICENSE](../LICENSE) for details.

---

<div align="center">

**🚀 Welcome to the VANTIS OS Team! 🚀**

Made with ❤️ by the VANTIS Community

[⬆ Back to Top](#-vantis-os---developer-onboarding-guide)

</div>