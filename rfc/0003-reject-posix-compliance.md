# RFC-0003: Reject POSIX Compliance

## Status

Accepted

## Author

VantisOS Team (@vantisTeam)

## Created

2025-02-24

## Summary

This RFC proposes that VantisOS consciously reject POSIX compliance. VantisOS will not be POSIX-compliant and will implement custom APIs that prioritize security, formal verification, and innovation over compatibility with Unix-like systems.

## Motivation

POSIX (Portable Operating System Interface) is a standard from the 1970s that:
- Enforces compatibility with Unix-like systems
- Includes many obsolete design decisions
- Prioritizes compatibility over security
- Contains design flaws that cannot be formally verified

**Problems with POSIX**:

1. **Insecure APIs**:
   - Signals: Asynchronous, no context, race conditions
   - Shared memory: No bounds checking, data races
   - Pipes/queues: No access control, any process can write

2. **Impossible to verify**:
   - Undefined behavior is rampant
   - Many APIs have race conditions
   - Memory safety is not guaranteed

3. **Inhibits innovation**:
   - Forces old design patterns
   - Prevents new approaches
   - Maintains legacy debt

4. **VantisOS requirements**:
   - Formal verification: All APIs must be verifiable
   - Security: No insecure abstractions
   - Innovation: Freedom to design optimal solutions

## Detailed Design

### Decision: Full POSIX Rejection

VantisOS will **NOT** implement POSIX-compliant APIs.

### Custom APIs

**System Call Interface**:
- Custom syscall ABI
- Memory-safe parameters
- Capability-based access control
- No undefined behavior

**File System (VantisFS)**:
- Custom file system semantics
- No Unix file permissions (use capabilities)
- Modern file system features

**IPC System**:
- Custom capability-based IPC
- No Unix pipes or message queues
- End-to-end encryption

**Process Model**:
- No Unix signals or fork
- Modern process management
- Capability-based isolation

**Standard Library**:
- Custom Vantis stdlib
- No POSIX stdlib
- Memory-safe APIs

### Benefits of POSIX Rejection

**Security**:
- No insecure POSIX abstractions
- All APIs can be formally verified
- Capability-based security model

**Formal Verification**:
- All APIs can be verified
- No undefined behavior
- Proven security properties

**Innovation**:
- Freedom to design optimal solutions
- No legacy constraints
- Modern design patterns

**Simplicity**:
- No compatibility layers
- No legacy debt
- Cleaner architecture

### Challenges of POSIX Rejection

**No compatibility**:
- No POSIX applications work natively
- Requires Legacy Airlock (RFC-0002) for compatibility
- Porting required for all software

**Developer friction**:
- Developers must learn new APIs
- Existing experience doesn't transfer
- Learning curve

**Ecosystem impact**:
- Smaller initial ecosystem
- Must build from scratch
- Slower initial adoption

## Drawbacks

1. **No compatibility**: Cannot run POSIX applications natively
2. **Porting required**: All software must be ported or rewritten
3. **Developer friction**: Developers must learn new APIs
4. **Limited ecosystem**: No existing software works out-of-the-box
5. **Community resistance**: Some users expect POSIX compliance
6. **Legacy Airlock required**: Need compatibility layer (adds complexity)

## Rationale and Alternatives

### Why Full Rejection?

**Alternative 1: Full POSIX Compliance**
- **Pros**: Maximum compatibility, existing software works
- **Cons**: Insecure, impossible to verify, forces bad design
- **Rejected**: Security and formal verification are non-negotiable

**Alternative 2: Partial POSIX Compatibility (Linux-style)**
- **Pros**: Some compatibility, still mostly custom
- **Cons**: Still inherits POSIX design flaws, verification harder
- **Rejected**: Partial POSIX still compromises security

**Alternative 3: POSIX Compatibility Layer**
- **Pros**: Can run POSIX apps via emulation
- **Cons**: Complexity overhead, still need to implement POSIX
- **Rejected**: Waste of development effort

**Alternative 4: Full Rejection**
- **Pros**: Clean slate, security, verifiable
- **Cons**: No compatibility
- **Accepted**: Security and verification are non-negotiable

### Justification

**Security is non-negotiable**: POSIX has inherent design flaws that cannot be fixed.

**Formal verification is required**: Many POSIX APIs are impossible to verify.

**Innovation is priority**: We want to design optimal solutions, not carry 50 years of legacy.

**Compatibility via Legacy Airlock**: We can run POSIX apps via Legacy Airlock (RFC-0002) without compromising VantisOS.

## Prior Art

- **seL4**: Not POSIX-compliant, focuses on security
- **MINIX 3**: POSIX-like but not compliant
- **Plan 9**: Different design, not POSIX
- **The POSIX Horror Show**: Documentation of POSIX problems
- [POSIX: Why It's Time to Move On](https://queue.acm.org/detail.cfm?id=3031035)

## Unresolved Questions

None - decision is clear and final.

## Implementation Plan

### Phase 1: Custom System Call Interface (4 weeks)

**Timeline**: Weeks 1-4

**Milestones**:
- [ ] Custom syscall ABI
- [ ] Memory-safe parameter validation
- [ ] Capability-based access control
- [ ] Documentation

### Phase 2: Custom IPC (3 weeks)

**Timeline**: Weeks 5-7

**Milestones**:
- [ ] Capability-based IPC
- [ ] End-to-end encryption
- [ ] No POSIX pipes/queues
- [ ] Documentation

### Phase 3: Custom File System (6 weeks)

**Timeline**: Weeks 8-13

**Milestones**:
- [ ] VantisFS design
- [ ] Custom file system semantics
- [ ] No Unix permissions
- [ ] Documentation

### Phase 4: Custom Process Model (3 weeks)

**Timeline**: Weeks 14-16

**Milestones**:
- [ ] Modern process management
- [ ] No Unix signals/fork
- [ ] Documentation

### Phase 5: Custom Standard Library (6 weeks)

**Timeline**: Weeks 17-22

**Milestones**:
- [ ] Vantis stdlib design
- [ ] No POSIX stdlib
- [ ] Memory-safe APIs
- [ ] Documentation

**Total**: 22 weeks

## Testing

1. **Unit tests**: Test each custom API
2. **Integration tests**: Test APIs together
3. **Formal verification**: Verify critical properties
4. **Security tests**: Verify no security issues
5. **Performance tests**: Benchmark vs POSIX

## Risks and Mitigations

### Risk 1: Developer adoption

**Mitigation**: Excellent documentation, examples, tools

### Risk 2: Ecosystem growth

**Mitigation**: Legacy Airlock (RFC-0002), community building

### Risk 3: Porting effort

**Mitigation**: Provide migration guides, tools, support

### Risk 4: Community resistance

**Mitigation**: Clear communication, benefits explained

## Success Criteria

- [ ] All custom APIs implemented
- [ ] All APIs formally verified
- [ ] Documentation complete
- [ ] No POSIX dependencies
- [ ] Community adoption: 50+ developers in first year

## Dependencies

- **ADR-0001**: Use Rust as primary language
- **ADR-0002**: Adopt microkernel architecture
- **ADR-0004**: Capability-based IPC system
- **RFC-0002**: Legacy Airlock Compatibility Subsystem

## References

- [POSIX Specification](https://pubs.opengroup.org/onlinepubs/9699919799/)
- [The POSIX Horror Show](https://github.com/rust-osdev/posix-nomicon)
- [POSIX: Why It's Time to Move On](https://queue.acm.org/detail.cfm?id=3031035)
- [VantisOS MANIFEST.md](MANIFEST.md) - POSIX Rejection principle
- [POSIX Debloading Progress Report](../docs/reports/POSIX_DEBLOADING_PROGRESS_REPORT_FEB_22_2025.md)

## Appendix

### Examples of POSIX vs VantisOS

**POSIX signal** (insecure):
```c
// POSIX: Asynchronous, no context
signal(SIGINT, handler);  // Can happen at any time
```

**VantisOS capability** (secure):
```rust
// VantisOS: Capability-based, explicit
let cap = process::request_capability("terminate");
// Receiver must explicitly accept
```

**POSIX shared memory** (insecure):
```c
// POSIX: No bounds checking
void *shm = shmat(shmid, NULL, 0);
// Any process can read/write
```

**VantisOS shared memory** (secure):
```rust
// VantisOS: Capability-based
let shm = SharedMemory::new(capability)?;
// Only holders of capability can access
```

---

**Discussion**: https://github.com/vantisCorp/VantisOS/discussions/3
**Issue**: https://github.com/vantisCorp/VantisOS/issues/3
**PR**: https://github.com/vantisCorp/VantisOS/pull/3

**Last Updated**: 2025-02-24