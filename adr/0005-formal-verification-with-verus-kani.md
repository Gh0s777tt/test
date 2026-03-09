# ADR-0005: Formal Verification with Verus and Kani

## Status

**Accepted**

## Context

Testing has limitations:
- Can only find bugs, not prove absence of bugs
- Covers only tested inputs, not all possible inputs
- Cannot guarantee critical security properties
- Manual testing is error-prone

Formal verification provides:
- Mathematical proofs of correctness
- Coverage of all possible inputs
- Guarantees of critical properties
- Machine-checked proofs

**Challenge**: How to formally verify Rust code efficiently?

## Decision

VantisOS will use **Verus** and **Kani** for formal verification:

**Verus**:
- Verification-aware programming language (Rust dialect)
- Proves functional correctness properties
- Supports complex invariants
- Interactive proof assistant
- Best for: Data structures, algorithms, protocols

**Kani**:
- Rust verifier using CBMC (C Bounded Model Checker)
- Proves safety properties (no panics, no undefined behavior)
- Fully automated (no interactive proofs)
- Symbolic execution engine
- Best for: Memory safety, absence of panics

**Verification Strategy**:
1. **Critical kernel components**: Full verification with Verus
2. **Safety properties**: Automated verification with Kani
3. **Security properties**: Custom proofs with Verus
4. **Continuous verification**: CI/CD integration
5. **Proof publication**: All proofs open source

**Verification Coverage**:
- All kernel IPC components: 100% verified
- Scheduler: 100% verified
- Memory manager: 100% verified
- Security-critical drivers: 100% verified

## Consequences

### Positive
- **Mathematical guarantees**: Proven correctness, not just tested
- **Security properties**: Proven absence of entire bug classes
- **Regulatory compliance**: Required for aviation, automotive, medical
- **Confidence**: Highest possible assurance
- **Marketing**: Unique selling point

### Negative
- **Development overhead**: Formal verification takes time
- **Learning curve**: Developers must learn Verus/Kani
- **Proof complexity**: Some properties are hard to prove
- **Compilation time**: Verification adds build time
- **Limited coverage**: Cannot verify all code (too expensive)

### Affected Systems
- All kernel components
- CI/CD pipeline (verification steps)
- Developer workflow (prove before commit)
- Code review (proofs must be reviewed)

## Alternatives Considered

### Testing Only
- **Pros**: Fast, familiar, no overhead
- **Cons**: Cannot prove absence of bugs
- **Rejected**: Formal verification is non-negotiable requirement

### Coq/Isabelle
- **Pros**: Most powerful proof assistants
- **Cons**: Steep learning curve, poor integration with Rust
- **Rejected**: Verus/Kani are Rust-native

### Dafny
- **Pros**: Verification-focused language
- **Cons**: Not Rust, would require rewriting
- **Rejected**: Want to use Rust

### Why3
- **Pros**: Multi-backend verification
- **Cons**: Poor Rust integration, less mature
- **Rejected**: Verus/Kani are better for Rust

## Related Decisions

- **ADR-0001**: Use Rust as primary language
- **ADR-0002**: Adopt microkernel architecture
- **ADR-0004**: Capability-based IPC system

## References

- [Verus](https://verus-lang.github.io/)
- [Kani Rust Verifier](https://model-checking.github.io/kani/)
- [Formal Verification of seL4](https://sel4.systems/Info/About/)
- [VantisOS Verification Status](docs/reports/VERIFICATION_STATUS.md)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [x] Implemented
- [x] Verified (IPC components)

---

**Author**: VantisOS Team  
**Date**: 2024-03-01  
**Last Updated**: 2025-02-24