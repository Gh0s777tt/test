# ADR-0003: Reject POSIX Compliance

## Status

**Accepted**

## Context

POSIX (Portable Operating System Interface) is a standard from the 1970s that:
- Enforces compatibility with Unix-like systems
- Includes many obsolete design decisions
- Prioritizes compatibility over security
- Contains design flaws that cannot be formally verified

POSIX forces us to:
- Implement insecure system calls (e.g., signals, shared memory)
- Use outdated IPC mechanisms (pipes, message queues)
- Maintain legacy APIs (stdio.h, stdlib.h)
- Follow Unix file system semantics
- Support obsolete process model

## Decision

VantisOS will **NOT be POSIX-compliant**. We consciously reject POSIX in favor of:

**Our Own APIs**:
- Modern IPC system with formal verification
- Capability-based security model
- Memory-safe system call interface
- New file system semantics
- Safe concurrency primitives

**Justification**:
1. **Security**: POSIX contains inherent design flaws
2. **Formal Verification**: Many POSIX APIs are impossible to verify
3. **Innovation**: POSIX prevents architectural improvements
4. **Performance**: Modern APIs can be more efficient
5. **Simplicity**: Not carrying decades of legacy debt

## Consequences

### Positive
- **Clean slate**: No legacy constraints on design
- **Formal verification**: All APIs can be verified
- **Better security**: No insecure POSIX abstractions
- **Simpler implementation**: No compatibility layers needed
- **Innovation**: Free to design optimal solutions

### Negative
- **No compatibility**: Cannot run POSIX applications natively
- **Porting required**: All software must be ported or rewritten
- **Developer friction**: Developers must learn new APIs
- **Limited ecosystem**: No existing software works out-of-the-box
- **Community resistance**: Some users expect POSIX compliance

### Affected Systems
- System call interface (completely custom)
- File system (VantisFS with custom semantics)
- IPC (no Unix pipes/queues)
- Process model (no Unix signals/forks)
- Standard library (custom Vantis stdlib)

## Alternatives Considered

### Full POSIX Compliance
- **Pros**: Maximum compatibility, existing software works
- **Cons**: Insecure, impossible to verify, forces bad design
- **Rejected**: Security and formal verification are non-negotiable

### Partial POSIX Compatibility (Linux-style)
- **Pros**: Some compatibility, still mostly custom
- **Cons**: Still inherits POSIX design flaws, verification harder
- **Rejected**: Partial POSIX still compromises security

### POSIX Compatibility Layer
- **Pros**: Can run POSIX apps via emulation
- **Cons**: Complexity overhead, still need to implement POSIX
- **Rejected**: Waste of development effort

### Compatibility via WebAssembly
- **Pros**: Modern sandbox, portable applications
- **Cons**: Performance overhead, requires all apps to be WASM
- **Accepted as Alternative**: Legacy Airlock subsystem for .exe compatibility

## Related Decisions

- **ADR-0001**: Use Rust as primary language
- **ADR-0002**: Adopt microkernel architecture
- **ADR-0007**: Legacy Airlock for compatibility

## References

- [POSIX: Why It's Time to Move On](https://queue.acm.org/detail.cfm?id=3031035)
- [The POSIX Horror Show](https://github.com/rust-osdev/posix-nomicon)
- [VantisOS MANIFEST.md](MANIFEST.md) - POSIX Rejection principle
- [POSIX Debloading Progress Report](docs/reports/POSIX_DEBLOADING_PROGRESS_REPORT_FEB_22_2025.md)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [x] Implemented
- [x] Documented

---

**Author**: VantisOS Team  
**Date**: 2024-02-01  
**Last Updated**: 2025-02-24