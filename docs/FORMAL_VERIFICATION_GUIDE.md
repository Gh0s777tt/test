# 🔬 VANTIS OS - Formal Verification Guide

## 📋 Table of Contents

- [Introduction](#introduction)
- [Why Formal Verification?](#why-formal-verification)
- [Verification Tools](#verification-tools)
- [Setting Up Verus](#setting-up-verus)
- [Setting Up Kani](#setting-up-kani)
- [Writing Verified Code](#writing-verified-code)
- [Verification Workflow](#verification-workflow)
- [Best Practices](#best-practices)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

---

## Introduction

Formal verification is the process of mathematically proving that code behaves correctly according to its specification. For VANTIS OS, formal verification is essential for:

- **EAL 7+ Certification**: Requires mathematical proof of correctness
- **Security Guarantees**: Prove absence of vulnerabilities
- **Memory Safety**: Verify no buffer overflows or use-after-free
- **Correctness**: Ensure algorithms work as intended

---

## Why Formal Verification?

### Traditional Testing vs. Formal Verification

| Aspect | Testing | Formal Verification |
|--------|---------|---------------------|
| Coverage | Sample inputs | All possible inputs |
| Guarantees | "Works for tested cases" | "Mathematically proven correct" |
| Bugs Found | Some | All (within specification) |
| Cost | Lower | Higher |
| Confidence | Medium | Very High |

### VANTIS OS Requirements

For EAL 7+ certification, we need:
- **Formal specification** of all security-critical components
- **Mathematical proofs** that implementation matches specification
- **Verified absence** of common vulnerabilities
- **Documented proof methodology**

---

## Verification Tools

VANTIS OS uses two complementary verification tools:

### 1. Verus

**Purpose**: Deductive verification with SMT solvers

**Strengths**:
- Expressive specifications
- Interactive proof development
- Good for complex algorithms
- Supports ghost code and proof functions

**Use Cases**:
- Memory management algorithms
- Cryptographic implementations
- IPC mechanisms
- Scheduler logic

### 2. Kani

**Purpose**: Bounded model checking

**Strengths**:
- Automatic verification
- Finds counterexamples
- Good for finding bugs
- Easy to use

**Use Cases**:
- Safety property checking
- Undefined behavior detection
- Concurrency bugs
- Edge case discovery

---

## Setting Up Verus

### Installation

```bash
# Install Verus from source
git clone https://github.com/verus-lang/verus.git
cd verus
./tools/get-z3.sh  # Download Z3 SMT solver
cargo build --release

# Add to PATH
export PATH="$PATH:$(pwd)/target/release"

# Verify installation
verus --version
```

### Configuration

Create `.verus-config.toml` in project root:

```toml
[verus]
# SMT solver timeout (seconds)
smt_timeout = 30

# Number of parallel verification tasks
parallel = 4

# Enable detailed output
verbose = true

# Z3 path (if not in PATH)
z3_path = "/usr/local/bin/z3"
```

### IDE Integration

For VS Code, install the Verus extension:

```bash
# Install VS Code extension
code --install-extension verus-lang.verus-analyzer
```

---

## Setting Up Kani

### Installation

```bash
# Install Kani
cargo install --locked kani-verifier

# Set up Kani
cargo kani setup

# Verify installation
cargo kani --version
```

### Configuration

Create `Kani.toml` in project root:

```toml
[kani]
# Default harness to run
default-harness = "verify_all"

# Unwinding depth for loops
unwind = 10

# Enable CBMC options
cbmc-args = ["--bounds-check", "--pointer-check"]

# Solver options
solver = "minisat"
```

---

## Writing Verified Code

### Verus Basics

```rust
use verus::prelude::*;

verus! {
    // Function with preconditions and postconditions
    pub fn safe_add(a: u32, b: u32) -> (result: u32)
        requires a + b <= u32::MAX,  // Precondition
        ensures result == a + b,      // Postcondition
    {
        a + b
    }
    
    // Proof function
    proof fn prove_commutativity(a: u32, b: u32)
        requires a + b <= u32::MAX,
        ensures safe_add(a, b) == safe_add(b, a),
    {
        // Proof is automatic for this simple case
    }
    
    // Verified data structure
    pub struct VerifiedBuffer {
        data: Vec<u8>,
        capacity: usize,
    }
    
    impl VerifiedBuffer {
        pub fn new(capacity: usize) -> (result: Self)
            ensures result.capacity == capacity,
                    result.data.len() == 0,
        {
            VerifiedBuffer {
                data: Vec::new(),
                capacity,
            }
        }
        
        pub fn push(&mut self, value: u8) -> (result: Result<(), ()>)
            requires self.data.len() < self.capacity,
            ensures match result {
                Ok(()) => self.data.len() == old(self).data.len() + 1,
                Err(()) => self.data.len() == old(self).data.len(),
            },
        {
            if self.data.len() < self.capacity {
                self.data.push(value);
                Ok(())
            } else {
                Err(())
            }
        }
    }
}
```

### Kani Basics

```rust
#[cfg(kani)]
mod verification {
    use super::*;
    
    #[kani::proof]
    fn verify_safe_add() {
        // Generate arbitrary inputs
        let a: u32 = kani::any();
        let b: u32 = kani::any();
        
        // Assume precondition
        kani::assume(a as u64 + b as u64 <= u32::MAX as u64);
        
        // Call function
        let result = safe_add(a, b);
        
        // Assert postcondition
        assert!(result == a + b);
    }
    
    #[kani::proof]
    fn verify_no_overflow() {
        let a: u32 = kani::any();
        let b: u32 = kani::any();
        
        if a as u64 + b as u64 <= u32::MAX as u64 {
            let result = safe_add(a, b);
            assert!(result >= a && result >= b);
        }
    }
    
    #[kani::proof]
    #[kani::unwind(5)]
    fn verify_buffer_operations() {
        let mut buffer = VerifiedBuffer::new(10);
        
        // Verify multiple operations
        for i in 0..5 {
            let result = buffer.push(i as u8);
            assert!(result.is_ok());
        }
        
        assert!(buffer.data.len() == 5);
    }
}
```

---

## Verification Workflow

### 1. Write Specification

```rust
verus! {
    /// Memory allocation with verified safety
    pub fn allocate_memory(size: usize) -> (result: Result<*mut u8, AllocError>)
        requires 
            size > 0,
            size <= MAX_ALLOCATION_SIZE,
        ensures 
            match result {
                Ok(ptr) => {
                    // Pointer is non-null
                    ptr != std::ptr::null_mut() &&
                    // Pointer is properly aligned
                    (ptr as usize) % ALIGNMENT == 0
                },
                Err(_) => true,
            },
    {
        // Implementation
        todo!()
    }
}
```

### 2. Implement Function

```rust
verus! {
    pub fn allocate_memory(size: usize) -> (result: Result<*mut u8, AllocError>)
        requires 
            size > 0,
            size <= MAX_ALLOCATION_SIZE,
        ensures 
            match result {
                Ok(ptr) => {
                    ptr != std::ptr::null_mut() &&
                    (ptr as usize) % ALIGNMENT == 0
                },
                Err(_) => true,
            },
    {
        // Check size
        if size == 0 || size > MAX_ALLOCATION_SIZE {
            return Err(AllocError::InvalidSize);
        }
        
        // Allocate with proper alignment
        let layout = Layout::from_size_align(size, ALIGNMENT)
            .map_err(|_| AllocError::InvalidAlignment)?;
        
        let ptr = unsafe { alloc(layout) };
        
        if ptr.is_null() {
            Err(AllocError::OutOfMemory)
        } else {
            Ok(ptr)
        }
    }
}
```

### 3. Add Proofs

```rust
verus! {
    proof fn prove_allocation_safety(size: usize)
        requires 
            size > 0,
            size <= MAX_ALLOCATION_SIZE,
    {
        let result = allocate_memory(size);
        
        // Prove that successful allocation returns valid pointer
        if let Ok(ptr) = result {
            assert!(ptr != std::ptr::null_mut());
            assert!((ptr as usize) % ALIGNMENT == 0);
        }
    }
}
```

### 4. Run Verification

```bash
# Verify with Verus
verus src/kernel/memory.rs

# Verify with Kani
cargo kani --harness verify_allocation_safety

# Run all verifications
make verify
```

---

## Best Practices

### 1. Start Simple

```rust
// Good: Simple, verifiable function
verus! {
    pub fn min(a: i32, b: i32) -> (result: i32)
        ensures result <= a && result <= b,
                result == a || result == b,
    {
        if a < b { a } else { b }
    }
}

// Avoid: Complex function without clear specification
fn complex_algorithm(data: &[u8]) -> Vec<u8> {
    // Many operations without clear invariants
    todo!()
}
```

### 2. Use Ghost Code

```rust
verus! {
    pub struct VerifiedList<T> {
        data: Vec<T>,
        ghost len: nat,  // Ghost variable for specification
    }
    
    impl<T> VerifiedList<T> {
        pub fn new() -> (result: Self)
            ensures result.len == 0,
        {
            VerifiedList {
                data: Vec::new(),
                ghost len: 0,
            }
        }
        
        pub fn push(&mut self, value: T)
            ensures self.len == old(self).len + 1,
        {
            self.data.push(value);
            proof {
                self.len = self.len + 1;
            }
        }
    }
}
```

### 3. Specify Loop Invariants

```rust
verus! {
    pub fn sum_array(arr: &[u32]) -> (result: u32)
        requires arr.len() > 0,
        ensures result >= 0,
    {
        let mut sum: u32 = 0;
        let mut i: usize = 0;
        
        while i < arr.len()
            invariant 
                i <= arr.len(),
                sum >= 0,
        {
            sum = sum + arr[i];
            i = i + 1;
        }
        
        sum
    }
}
```

### 4. Handle Edge Cases

```rust
verus! {
    pub fn safe_divide(a: u32, b: u32) -> (result: Result<u32, DivError>)
        ensures 
            match result {
                Ok(r) => b != 0 && r == a / b,
                Err(_) => b == 0,
            },
    {
        if b == 0 {
            Err(DivError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }
}
```

---

## Examples

### Example 1: Verified Memory Allocator

```rust
use verus::prelude::*;

verus! {
    pub struct MemoryAllocator {
        heap_start: usize,
        heap_size: usize,
        allocated: usize,
    }
    
    impl MemoryAllocator {
        pub fn new(start: usize, size: usize) -> (result: Self)
            requires 
                size > 0,
                start + size > start,  // No overflow
            ensures 
                result.heap_start == start,
                result.heap_size == size,
                result.allocated == 0,
        {
            MemoryAllocator {
                heap_start: start,
                heap_size: size,
                allocated: 0,
            }
        }
        
        pub fn allocate(&mut self, size: usize) -> (result: Option<usize>)
            requires 
                size > 0,
                self.allocated <= self.heap_size,
            ensures 
                match result {
                    Some(addr) => {
                        addr >= self.heap_start &&
                        addr + size <= self.heap_start + self.heap_size &&
                        self.allocated == old(self).allocated + size
                    },
                    None => {
                        self.allocated == old(self).allocated
                    }
                },
        {
            if self.allocated + size <= self.heap_size {
                let addr = self.heap_start + self.allocated;
                self.allocated = self.allocated + size;
                Some(addr)
            } else {
                None
            }
        }
    }
}
```

### Example 2: Verified Ring Buffer

```rust
use verus::prelude::*;

verus! {
    pub struct RingBuffer<T> {
        data: Vec<T>,
        capacity: usize,
        head: usize,
        tail: usize,
        ghost size: nat,
    }
    
    impl<T> RingBuffer<T> {
        pub fn new(capacity: usize) -> (result: Self)
            requires capacity > 0,
            ensures 
                result.capacity == capacity,
                result.size == 0,
        {
            RingBuffer {
                data: Vec::with_capacity(capacity),
                capacity,
                head: 0,
                tail: 0,
                ghost size: 0,
            }
        }
        
        pub fn push(&mut self, value: T) -> (result: Result<(), T>)
            ensures 
                match result {
                    Ok(()) => self.size == old(self).size + 1,
                    Err(_) => self.size == old(self).size,
                },
        {
            if self.size < self.capacity {
                self.data.push(value);
                self.tail = (self.tail + 1) % self.capacity;
                proof { self.size = self.size + 1; }
                Ok(())
            } else {
                Err(value)
            }
        }
        
        pub fn pop(&mut self) -> (result: Option<T>)
            ensures 
                match result {
                    Some(_) => self.size == old(self).size - 1,
                    None => self.size == 0,
                },
        {
            if self.size > 0 {
                let value = self.data.remove(self.head);
                self.head = (self.head + 1) % self.capacity;
                proof { self.size = self.size - 1; }
                Some(value)
            } else {
                None
            }
        }
    }
}
```

---

## Troubleshooting

### Common Issues

#### 1. Verification Timeout

```bash
# Increase timeout
verus --smt-timeout 60 src/kernel/memory.rs

# Simplify specification
# Break complex proofs into smaller lemmas
```

#### 2. Proof Fails

```rust
// Add intermediate assertions
verus! {
    pub fn complex_function(x: u32) -> (result: u32)
        ensures result > 0,
    {
        let y = x + 1;
        assert(y > x);  // Intermediate assertion
        
        let z = y * 2;
        assert(z > y);  // Another assertion
        
        z
    }
}
```

#### 3. Kani Counterexample

```bash
# Run with trace
cargo kani --harness verify_function --visualize

# Check counterexample
# Fix code or adjust assumptions
```

---

## CI/CD Integration

### GitHub Actions Workflow

```yaml
name: Formal Verification

on: [push, pull_request]

jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: nightly
          
      - name: Install Verus
        run: |
          git clone https://github.com/verus-lang/verus.git
          cd verus
          ./tools/get-z3.sh
          cargo build --release
          echo "$(pwd)/target/release" >> $GITHUB_PATH
          
      - name: Install Kani
        run: |
          cargo install --locked kani-verifier
          cargo kani setup
          
      - name: Run Verus Verification
        run: make verify-verus
        
      - name: Run Kani Verification
        run: make verify-kani
```

---

<div align="center">

**🔬 Formal Verification is the Foundation of Trust 🔬**

Made with ❤️ by the VANTIS Team

[⬆ Back to Top](#-vantis-os---formal-verification-guide)

</div>