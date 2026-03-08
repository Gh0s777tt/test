# 🔬 VANTIS OS - Formal Verification Examples

This document provides practical examples of formally verified code in VANTIS OS.

## 📋 Table of Contents

- [Basic Examples](#basic-examples)
- [Memory Management](#memory-management)
- [Cryptography](#cryptography)
- [Concurrency](#concurrency)
- [Advanced Patterns](#advanced-patterns)

---

## Basic Examples

### Example 1: Verified Addition

```rust
use verus::prelude::*;

verus! {
    /// Add two numbers with overflow protection
    pub fn safe_add(a: u32, b: u32) -> (result: u32)
        requires a as u64 + b as u64 <= u32::MAX as u64,
        ensures result == a + b,
    {
        a + b
    }
    
    /// Proof that addition is commutative
    proof fn prove_addition_commutative(a: u32, b: u32)
        requires a as u64 + b as u64 <= u32::MAX as u64,
        ensures safe_add(a, b) == safe_add(b, a),
    {
        // Proof is automatic
    }
    
    /// Proof that addition is associative
    proof fn prove_addition_associative(a: u32, b: u32, c: u32)
        requires 
            a as u64 + b as u64 <= u32::MAX as u64,
            (a + b) as u64 + c as u64 <= u32::MAX as u64,
            b as u64 + c as u64 <= u32::MAX as u64,
            a as u64 + (b + c) as u64 <= u32::MAX as u64,
        ensures safe_add(safe_add(a, b), c) == safe_add(a, safe_add(b, c)),
    {
        // Proof is automatic
    }
}
```

### Example 2: Verified Array Access

```rust
use verus::prelude::*;

verus! {
    /// Safely access array element
    pub fn safe_get(arr: &[u32], index: usize) -> (result: Option<u32>)
        ensures 
            match result {
                Some(val) => index < arr.len() && val == arr[index],
                None => index >= arr.len(),
            },
    {
        if index < arr.len() {
            Some(arr[index])
        } else {
            None
        }
    }
    
    /// Safely set array element
    pub fn safe_set(arr: &mut [u32], index: usize, value: u32) -> (result: Result<(), ()>)
        ensures 
            match result {
                Ok(()) => {
                    index < arr.len() &&
                    arr[index] == value
                },
                Err(()) => index >= arr.len(),
            },
    {
        if index < arr.len() {
            arr[index] = value;
            Ok(())
        } else {
            Err(())
        }
    }
}
```

---

## Memory Management

### Example 3: Verified Memory Allocator

```rust
use verus::prelude::*;

verus! {
    pub struct Allocator {
        base: usize,
        size: usize,
        used: usize,
        ghost allocated_regions: Seq<(usize, usize)>,
    }
    
    impl Allocator {
        /// Create new allocator
        pub fn new(base: usize, size: usize) -> (result: Self)
            requires 
                size > 0,
                base + size > base,
            ensures 
                result.base == base,
                result.size == size,
                result.used == 0,
                result.allocated_regions.len() == 0,
        {
            Allocator {
                base,
                size,
                used: 0,
                ghost allocated_regions: Seq::empty(),
            }
        }
        
        /// Allocate memory
        pub fn alloc(&mut self, size: usize) -> (result: Option<usize>)
            requires 
                size > 0,
                old(self).used <= old(self).size,
            ensures 
                match result {
                    Some(addr) => {
                        addr >= self.base &&
                        addr + size <= self.base + self.size &&
                        self.used == old(self).used + size &&
                        // No overlap with existing allocations
                        forall|i: int| 0 <= i < old(self).allocated_regions.len() ==> {
                            let (old_addr, old_size) = old(self).allocated_regions[i];
                            addr + size <= old_addr || addr >= old_addr + old_size
                        }
                    },
                    None => {
                        self.used == old(self).used &&
                        self.allocated_regions == old(self).allocated_regions
                    }
                },
        {
            if self.used + size <= self.size {
                let addr = self.base + self.used;
                self.used = self.used + size;
                proof {
                    self.allocated_regions = self.allocated_regions.push((addr, size));
                }
                Some(addr)
            } else {
                None
            }
        }
        
        /// Check if address is valid
        pub fn is_valid(&self, addr: usize, size: usize) -> (result: bool)
            ensures 
                result == (
                    addr >= self.base &&
                    addr + size <= self.base + self.size
                ),
        {
            addr >= self.base && addr + size <= self.base + self.size
        }
    }
}
```

### Example 4: Verified Ring Buffer

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
        /// Create new ring buffer
        pub fn new(capacity: usize) -> (result: Self)
            requires capacity > 0,
            ensures 
                result.capacity == capacity,
                result.size == 0,
                result.head == 0,
                result.tail == 0,
        {
            RingBuffer {
                data: Vec::with_capacity(capacity),
                capacity,
                head: 0,
                tail: 0,
                ghost size: 0,
            }
        }
        
        /// Push element
        pub fn push(&mut self, value: T) -> (result: Result<(), T>)
            ensures 
                match result {
                    Ok(()) => {
                        self.size == old(self).size + 1 &&
                        self.size <= self.capacity
                    },
                    Err(_) => {
                        self.size == old(self).size &&
                        old(self).size == self.capacity
                    }
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
        
        /// Pop element
        pub fn pop(&mut self) -> (result: Option<T>)
            ensures 
                match result {
                    Some(_) => {
                        self.size == old(self).size - 1 &&
                        old(self).size > 0
                    },
                    None => {
                        self.size == 0 &&
                        old(self).size == 0
                    }
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
        
        /// Get current size
        pub fn len(&self) -> (result: usize)
            ensures result == self.size,
        {
            self.data.len()
        }
        
        /// Check if empty
        pub fn is_empty(&self) -> (result: bool)
            ensures result == (self.size == 0),
        {
            self.size == 0
        }
        
        /// Check if full
        pub fn is_full(&self) -> (result: bool)
            ensures result == (self.size == self.capacity),
        {
            self.size == self.capacity
        }
    }
}
```

---

## Cryptography

### Example 5: Verified XOR Cipher

```rust
use verus::prelude::*;

verus! {
    /// XOR two byte arrays
    pub fn xor_bytes(a: &[u8], b: &[u8]) -> (result: Vec<u8>)
        requires a.len() == b.len(),
        ensures 
            result.len() == a.len(),
            forall|i: int| 0 <= i < result.len() ==> 
                result[i] == a[i] ^ b[i],
    {
        let mut result = Vec::with_capacity(a.len());
        let mut i = 0;
        
        while i < a.len()
            invariant 
                i <= a.len(),
                result.len() == i,
                forall|j: int| 0 <= j < i ==> 
                    result[j] == a[j] ^ b[j],
        {
            result.push(a[i] ^ b[i]);
            i = i + 1;
        }
        
        result
    }
    
    /// Proof that XOR is its own inverse
    proof fn prove_xor_inverse(data: &[u8], key: &[u8])
        requires data.len() == key.len(),
    {
        let encrypted = xor_bytes(data, key);
        let decrypted = xor_bytes(&encrypted, key);
        
        assert(forall|i: int| 0 <= i < data.len() ==> 
            decrypted[i] == data[i]
        );
    }
}
```

### Example 6: Verified Hash Function Properties

```rust
use verus::prelude::*;

verus! {
    /// Simple hash function (for demonstration)
    pub fn simple_hash(data: &[u8]) -> (result: u32)
        ensures result >= 0,
    {
        let mut hash: u32 = 0;
        let mut i = 0;
        
        while i < data.len()
            invariant 
                i <= data.len(),
                hash >= 0,
        {
            hash = hash.wrapping_add(data[i] as u32);
            hash = hash.wrapping_mul(31);
            i = i + 1;
        }
        
        hash
    }
    
    /// Proof that same input produces same output
    proof fn prove_hash_deterministic(data: &[u8])
    {
        let hash1 = simple_hash(data);
        let hash2 = simple_hash(data);
        
        assert(hash1 == hash2);
    }
}
```

---

## Concurrency

### Example 7: Verified Lock-Free Counter

```rust
use verus::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

verus! {
    pub struct AtomicCounter {
        value: AtomicU32,
        ghost max_value: u32,
    }
    
    impl AtomicCounter {
        /// Create new counter
        pub fn new(max: u32) -> (result: Self)
            ensures result.max_value == max,
        {
            AtomicCounter {
                value: AtomicU32::new(0),
                ghost max_value: max,
            }
        }
        
        /// Increment counter
        pub fn increment(&self) -> (result: Result<u32, ()>)
            ensures 
                match result {
                    Ok(val) => val < self.max_value,
                    Err(()) => true,
                },
        {
            let current = self.value.load(Ordering::SeqCst);
            
            if current < self.max_value {
                self.value.fetch_add(1, Ordering::SeqCst);
                Ok(current + 1)
            } else {
                Err(())
            }
        }
        
        /// Get current value
        pub fn get(&self) -> (result: u32)
            ensures result <= self.max_value,
        {
            self.value.load(Ordering::SeqCst)
        }
    }
}
```

---

## Advanced Patterns

### Example 8: Verified State Machine

```rust
use verus::prelude::*;

verus! {
    #[derive(PartialEq, Eq)]
    pub enum State {
        Init,
        Running,
        Paused,
        Stopped,
    }
    
    pub struct StateMachine {
        state: State,
        ghost valid_transitions: Seq<(State, State)>,
    }
    
    impl StateMachine {
        /// Create new state machine
        pub fn new() -> (result: Self)
            ensures result.state == State::Init,
        {
            StateMachine {
                state: State::Init,
                ghost valid_transitions: seq![
                    (State::Init, State::Running),
                    (State::Running, State::Paused),
                    (State::Paused, State::Running),
                    (State::Running, State::Stopped),
                    (State::Paused, State::Stopped),
                ],
            }
        }
        
        /// Transition to new state
        pub fn transition(&mut self, new_state: State) -> (result: Result<(), ()>)
            ensures 
                match result {
                    Ok(()) => self.state == new_state,
                    Err(()) => self.state == old(self).state,
                },
        {
            let transition = (self.state, new_state);
            
            if self.is_valid_transition(transition) {
                self.state = new_state;
                Ok(())
            } else {
                Err(())
            }
        }
        
        /// Check if transition is valid
        fn is_valid_transition(&self, transition: (State, State)) -> (result: bool)
        {
            // Check against valid transitions
            transition == (State::Init, State::Running) ||
            transition == (State::Running, State::Paused) ||
            transition == (State::Paused, State::Running) ||
            transition == (State::Running, State::Stopped) ||
            transition == (State::Paused, State::Stopped)
        }
        
        /// Get current state
        pub fn get_state(&self) -> (result: State)
            ensures result == self.state,
        {
            self.state
        }
    }
}
```

### Example 9: Verified Binary Search

```rust
use verus::prelude::*;

verus! {
    /// Binary search in sorted array
    pub fn binary_search(arr: &[i32], target: i32) -> (result: Option<usize>)
        requires forall|i: int, j: int| 0 <= i < j < arr.len() ==> arr[i] <= arr[j],
        ensures 
            match result {
                Some(idx) => {
                    idx < arr.len() &&
                    arr[idx] == target
                },
                None => {
                    forall|i: int| 0 <= i < arr.len() ==> arr[i] != target
                }
            },
    {
        let mut left = 0;
        let mut right = arr.len();
        
        while left < right
            invariant 
                left <= right,
                right <= arr.len(),
                forall|i: int| 0 <= i < left ==> arr[i] < target,
                forall|i: int| right <= i < arr.len() ==> arr[i] > target,
        {
            let mid = left + (right - left) / 2;
            
            if arr[mid] == target {
                return Some(mid);
            } else if arr[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        
        None
    }
}
```

---

## Running Verification

### Verify with Verus

```bash
# Verify single file
verus src/verified/math.rs

# Verify with verbose output
verus --verbose src/verified/memory.rs

# Verify with increased timeout
verus --smt-timeout 60 src/verified/crypto.rs
```

### Verify with Kani

```bash
# Verify all harnesses
cargo kani

# Verify specific harness
cargo kani --harness verify_allocator

# Visualize counterexample
cargo kani --harness verify_buffer --visualize
```

---

<div align="center">

**🔬 Formal Verification Ensures Correctness 🔬**

Made with ❤️ by the VANTIS Team

[⬆ Back to Top](#-vantis-os---formal-verification-examples)

</div>