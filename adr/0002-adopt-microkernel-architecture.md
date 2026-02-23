# ADR-0002: Adopt Microkernel Architecture

## Status

**Accepted**

## Context

Traditional monolithic kernels (Linux, Windows) have:
- Large Trusted Computing Base (TCB) - millions of lines in kernel space
- Vulnerabilities in drivers can compromise entire system
- Difficult to formally verify due to size and complexity
- Single point of failure for system stability

Monolithic kernels provide:
- Better performance (fewer context switches)
- Simpler development (everything in one address space)
- Richer feature sets

## Decision

VantisOS will use a **microkernel architecture** where:
- **Minimal kernel**: Only essential services run in kernel space
- **User-space services**: Device drivers, file systems, networking run in user space
- **Capability-based security**: Fine-grained access control via capabilities
- **Message passing**: IPC (Inter-Process Communication) for all communication

**Kernel-Only Services**:
1. Thread scheduling
2. Memory management (page tables)
3. Inter-Process Communication (IPC)
4. Interrupt handling
5. Capability management

**User-Space Services**:
1. All device drivers (GPU, network, storage, etc.)
2. File systems (VantisFS)
3. Network stack (TCP/IP)
4. System services (authentication, logging, etc.)
5. Window manager and GUI

## Consequences

### Positive
- **Smaller TCB**: Kernel is ~10,000 lines instead of millions, easier to verify
- **Fault isolation**: Driver crashes don't crash kernel
- **Better security**: Security properties can be proven per-component
- **Self-healing**: Failed services can be restarted without reboot
- **Formal verification**: Each component can be verified independently
- **Modularity**: Clear boundaries between components

### Negative
- **Performance overhead**: More context switches for IPC
- **Development complexity**: Harder to develop drivers in user space
- **Hardware access**: Some hardware requires kernel-space access (MMIO, DMA)
- **Debugging**: More complex due to message passing

### Affected Systems
- Kernel architecture (minimal)
- IPC system (critical path)
- Driver architecture (user-space)
- Memory management (capability-based)
- Security model (capability isolation)

## Alternatives Considered

### Monolithic Kernel
- **Pros**: Better performance, simpler development, mature ecosystem
- **Cons**: Large TCB, difficult to verify, single point of failure
- **Rejected**: Formal verification and security are non-negotiable requirements

### Unikernel
- **Pros**: Smallest possible TCB, single application optimized
- **Cons**: Not suitable for general-purpose OS, no multi-application support
- **Rejected**: VantisOS is a general-purpose OS

### Hybrid Kernel
- **Pros**: Balance between performance and security
- **Cons**: Still large TCB, difficult to define boundaries
- **Rejected**: Pure microkernel provides better security and verification

### Exokernel
- **Pros**: Maximum flexibility, applications manage hardware directly
- **Cons**: Extremely complex security model, difficult to use
- **Rejected**: Too experimental and complex for production use

## Related Decisions

- **ADR-0001**: Use Rust as primary language
- **ADR-0003**: Reject POSIX compliance
- **ADR-0004**: Capability-based IPC system
- **ADR-0006**: No global allocator in kernel

## References

- [L4 Microkernel Family](https://l4.re/)
- [MINIX 3 Microkernel](https://www.minix3.org/)
- [seL4 Microkernel](https://sel4.systems/)
- [VantisOS MANIFEST.md](MANIFEST.md) - Microkernel Architecture principle

## Implementation Status

- [x] Proposed
- [x] Accepted
- [x] Implemented
- [x] Tested

---

**Author**: VantisOS Team  
**Date**: 2024-01-20  
**Last Updated**: 2025-02-24