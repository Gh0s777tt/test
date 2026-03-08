# ADR-0008: WebAssembly as Primary Application Format

## Status

**Accepted**

## Context

Traditional executable formats have security issues:
- **ELF (Linux)**: Can execute arbitrary code, hard to sandbox
- **PE (Windows)**: Same issues as ELF
- **Machine code**: Cannot verify before execution

WebAssembly (WASM) provides:
- Sandboxed execution environment
- Deterministic execution
- Web-standard, well-specified
- Can be formally verified
- Portable across architectures

## Decision

VantisOS will use **WebAssembly** as the primary application format with extension `.vnt`:

**Why WebAssembly?**:
1. **Sandboxing**: Applications run in isolated sandbox
2. **Deterministic**: Same input always produces same output
3. **Formal verification**: WASM bytecode can be verified
4. **Portability**: Write once, run on any architecture
5. **Safety**: No direct hardware access

**VantisOS WASM Runtime**:
- Custom WASM runtime optimized for OS applications
- Native IPC integration with capabilities
- No browser overhead
- System call interface for OS access
- Ahead-of-time (AOT) compilation for performance

**Application Development**:
- Developers can use Rust, C++, AssemblyScript, etc.
- Compile to WASM (.vnt)
- Deploy to VantisOS
- Guaranteed to run safely

## Consequences

### Positive
- **Security**: Applications cannot compromise OS
- **Verification**: Can verify applications before execution
- **Portability**: One binary for all architectures
- **Safety**: No buffer overflows in WASM (sandbox catches them)
- **Web compatibility**: Can run web apps natively

### Negative
- **Performance**: WASM has overhead vs native
- **Development**: Requires WASM toolchain
- **Library support**: Not all libraries support WASM
- **Debugging**: WASM debugging is harder
- **Legacy apps**: Existing apps must be recompiled

### Affected Systems
- Application format (.vnt = WASM)
- Application runtime (WASM VM)
- Security model (sandbox isolation)
- Developer toolchain (WASM compilers)
- IPC system (WASM integration)

## Alternatives Considered

### Native Machine Code
- **Pros**: Maximum performance, no overhead
- **Cons**: No safety, cannot sandbox
- **Rejected**: Security is non-negotiable

### ELF with Sandboxing
- **Pros**: Native performance, familiar
- **Cons**: Hard to sandbox securely, complex
- **Rejected**: WASM provides better safety

### Java/JVM
- **Pros**: Mature, portable
- **Cons**: Garbage collector, slower startup
- **Rejected**: WASM is more modern and flexible

### Lua/Python Scripts
- **Pros**: Easy to write
- **Cons**: Too slow, no type safety
- **Rejected**: Performance critical for OS

## Related Decisions

- **ADR-0001**: Use Rust as primary language
- **ADR-0007**: Legacy Airlock for compatibility
- **ADR-0009**: WASM sandbox architecture

## References

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [Wasmtime Runtime](https://wasmtime.dev/)
- [WAVM (WebAssembly Virtual Machine)](https://github.com/wavm/wavm)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [ ] Implemented
- [ ] Tested

---

**Author**: VantisOS Team  
**Date**: 2024-04-15  
**Last Updated**: 2025-02-24