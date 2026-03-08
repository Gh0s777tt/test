# ADR-0001: Use Rust as the Primary Programming Language

## Status

**Accepted**

## Context

VantisOS requires a programming language that provides:
- Memory safety guarantees
- High performance comparable to C/C++
- Support for formal verification
- Modern tooling and ecosystem
- No runtime overhead (no garbage collector)

Traditional systems languages (C, C++) lack memory safety guarantees, leading to:
- Buffer overflows
- Use-after-free vulnerabilities
- Data races
- Null pointer dereferences

These vulnerabilities account for 70% of security issues in operating systems.

## Decision

VantisOS will use **Rust** as the primary programming language for:
- Kernel components
- User-space services
- Device drivers
- System tools

**Rationale**:
1. **Memory Safety**: Rust's ownership system prevents entire classes of memory errors at compile time
2. **Formal Verification**: Rust is uniquely suited for formal verification with Verus and Kani
3. **Performance**: Zero-cost abstractions with C-like performance
4. **Modern Ecosystem**: Cargo package manager, excellent documentation, growing community
5. **No Undefined Behavior**: In safe Rust, undefined behavior is impossible

## Consequences

### Positive
- **Eliminates Memory Safety Bugs**: Buffer overflows, use-after-free, data races prevented at compile time
- **Reduces Attack Surface**: Smaller TCB (Trusted Computing Base) possible due to safer code
- **Enables Formal Verification**: Verus and Kani provide machine-checked proofs
- **Faster Development**: Catch errors at compile time instead of runtime
- **Better Developer Experience**: Modern tooling, excellent error messages

### Negative
- **Learning Curve**: Rust has a steeper learning curve than C/C++
- **Ecosystem Size**: Smaller ecosystem than C/C++ (but growing rapidly)
- **Legacy Code**: Cannot directly reuse existing C/C++ kernel code
- **Compiler Overhead**: Longer compilation times

### Affected Systems
- All kernel components
- All drivers
- All user-space services
- Build system and toolchain
- CI/CD pipeline

## Alternatives Considered

### C/C++
- **Pros**: Mature ecosystem, familiar to kernel developers, maximum performance
- **Cons**: No memory safety, undefined behavior common, difficult to formally verify
- **Rejected**: Memory safety is a non-negotiable requirement

### Go
- **Pros**: Memory safe, simple syntax
- **Cons**: Garbage collector introduces unpredictable latency, less performant, not suitable for kernel
- **Rejected**: Garbage collector is unacceptable for kernel development

### Nim
- **Pros**: Memory safe options available, C-like performance
- **Cons**: Smaller ecosystem, less mature than Rust, limited formal verification support
- **Rejected**: Rust has better ecosystem and formal verification support

### Ada/SPARK
- **Pros**: Excellent formal verification support, safety-focused
- **Cons**: Very small ecosystem, steep learning curve, limited community
- **Rejected**: Rust provides similar safety benefits with better ecosystem

## Related Decisions

- **ADR-0002**: Adopt microkernel architecture
- **ADR-0003**: Reject POSIX compliance
- **ADR-0005**: Formal verification with Verus/Kani

## References

- [The Rust Language](https://www.rust-lang.org/)
- [Rust for Linux](https://rust-for-linux.com/)
- [Why Rust?](https://www.rust-lang.org/why-rust)
- [VantisOS MANIFEST.md](MANIFEST.md) - Formal Verification First principle

## Implementation Status

- [x] Proposed
- [x] Accepted
- [x] Implemented
- [x] Tested

---

**Author**: VantisOS Team  
**Date**: 2024-01-15  
**Last Updated**: 2025-02-24