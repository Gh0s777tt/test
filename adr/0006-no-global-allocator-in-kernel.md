# ADR-0006: No Global Allocator in Kernel

## Status

**Accepted**

## Context

In user space, Rust uses a global allocator (usually jemalloc or the system allocator) for dynamic memory allocation.

However, in kernel space, a global allocator presents problems:
- **No standard allocator**: Kernel manages its own memory
- **Determinism**: Allocation/deallocation must be predictable
- **Concurrent access**: Multiple CPUs accessing allocator
- **Memory fragmentation**: Critical in kernel space
- **Performance**: Kernel allocations must be fast
- **Error handling**: Kernel cannot panic on allocation failure

## Decision

VantisOS kernel will **NOT use a global allocator**. Instead:

**Memory Management Strategy**:
1. **Static allocation**: All kernel structures use fixed-size arrays
2. **Object pools**: Pre-allocated pools for common objects
3. **Buddy allocator**: Kernel's own memory allocator for dynamic needs
4. **No heap allocation**: Kernel code cannot use `Box`, `Vec`, `HashMap`, etc.

**Exceptions**:
- User-space services can use standard allocators
- Drivers in user space can use allocators
- Only kernel space is allocator-free

**Implementation**:
- `#![no_std]` attribute for kernel
- Custom `alloc_error_handler` for user-space services
- Static data structures with compile-time size
- Object pools with fixed capacity

## Consequences

### Positive
- **Determinism**: No unpredictable allocation delays
- **No heap corruption**: Static allocation eliminates heap bugs
- **Performance**: Known, fixed memory usage
- **Verification**: Easier to verify without dynamic allocation
- **Simpler**: No allocator bugs to debug

### Negative
- **Less flexible**: Cannot dynamically grow data structures
- **More planning**: Must size structures at compile time
- **Limited capacity**: Fixed-size arrays have limits
- **Code complexity**: Manual memory management for dynamic needs

### Affected Systems
- Kernel code (all modules)
- Data structures (fixed-size arrays)
- IPC system (message pools)
- Scheduler (task pools)
- Memory management (buddy allocator)

## Alternatives Considered

### Standard Global Allocator
- **Pros**: Convenient, familiar, flexible
- **Cons**: Unpredictable performance, hard to verify
- **Rejected**: Determinism and verification are critical

### Custom Global Allocator
- **Pros**: Tailored to kernel needs
- **Cons**: Still unpredictable, complex to verify
- **Rejected**: Static allocation is simpler and more predictable

### Hybrid (Limited Heap)
- **Pros**: Some flexibility where needed
- **Cons**: Complexity overhead, harder to verify
- **Rejected**: Full static allocation is cleaner

## Related Decisions

- **ADR-0001**: Use Rust as primary language
- **ADR-0002**: Adopt microkernel architecture
- **ADR-0011**: Object pool pattern for dynamic needs

## References

- [Rust Embedded Book - No Std](https://rust-embedded.github.io/book/intro/no-std.html)
- [Linux Kernel Memory Management](https://www.kernel.org/doc/html/latest/mm/index.html)
- [Static vs Dynamic Allocation in Kernels](https://lwn.net/Articles/833655/)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [x] Implemented
- [x] Verified

---

**Author**: VantisOS Team  
**Date**: 2024-03-10  
**Last Updated**: 2025-02-24