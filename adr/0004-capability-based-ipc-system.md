# ADR-0004: Capability-Based IPC System

## Status

**Accepted**

## Context

Traditional IPC mechanisms have significant security issues:
- **Unix pipes/queues**: No access control, any process can write
- **Shared memory**: No bounds checking, data races
- **Sockets**: Complex permission model, easy to misconfigure
- **Signals**: Asynchronous, no context, race conditions

Security challenges:
- How do we prevent unauthorized access to IPC channels?
- How do we ensure message integrity?
- How do we prevent message replay attacks?
- How do we enforce least privilege?

## Decision

VantisOS will use a **Capability-Based IPC System** with:

**Capabilities**:
- Unforgeable tokens granting access to resources
- Transferable between processes
- Revocable by resource owner
- Carry permissions (read, write, execute)

**IPC Features**:
1. **Verified Messaging**: All messages cryptographically signed
2. **Zero-Copy**: Transfer large buffers without copying
3. **Async/Sync**: Both blocking and non-blocking modes
4. **Priority**: Message priority scheduling
5. **Reliable**: Guaranteed delivery and ordering
6. **Formal Verification**: All IPC properties proven

**Security Properties**:
- **Integrity**: Messages cannot be tampered with
- **Confidentiality**: Messages encrypted end-to-end
- **Authentication**: Sender identity verified
- **Authorization**: Capabilities enforce access control

## Consequences

### Positive
- **Strong security**: Capabilities enforce least privilege
- **Composable**: Capabilities can be delegated
- **Revocable**: Access can be revoked anytime
- **Verifiable**: Formal proofs of security properties
- **Performance**: Zero-copy for large messages

### Negative
- **Complexity**: More complex than simple pipes
- **Overhead**: Cryptographic operations add latency
- **Learning curve**: Developers must understand capabilities
- **Compatibility**: No standard POSIX APIs

### Affected Systems
- IPC subsystem (critical)
- Security model (capability-based)
- Scheduler (priority messaging)
- Memory management (zero-copy)
- All user-space services

## Alternatives Considered

### Unix Pipes/Queues
- **Pros**: Simple, well-known
- **Cons**: No security, no access control
- **Rejected**: Security is non-negotiable

### Shared Memory
- **Pros**: Very fast for large data
- **Cons**: No synchronization, data races, hard to verify
- **Rejected**: Memory safety and verification

### Standard Message Queues
- **Pros**: Simple, familiar
- **Cons**: Weak security model
- **Rejected**: Insufficient for formal verification

### seL4 IPC
- **Pros**: Proven security, formally verified
- **Cons**: Specific to seL4, limited flexibility
- **Rejected**: Want custom VantisOS design

## Related Decisions

- **ADR-0002**: Adopt microkernel architecture
- **ADR-0005**: Formal verification with Verus/Kani
- **ADR-0010**: End-to-end encryption for all IPC

## References

- [seL4 IPC Documentation](https://docs.sel4.systems/projects/sel4/api-doc.html)
- [Capability Systems](https://cap-lore.com/)
- [VantisOS IPC Code](src/verified/ipc_verified.rs)
- [VantisOS IPC Security](src/verified/ipc_information_leakage.rs)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [x] Implemented
- [x] Formally Verified

---

**Author**: VantisOS Team  
**Date**: 2024-02-15  
**Last Updated**: 2025-02-24