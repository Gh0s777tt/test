# RFC-0001: WebAssembly as Primary Application Format

## Status

Accepted

## Author

VantisOS Team (@vantisTeam)

## Created

2025-02-24

## Summary

This RFC proposes using WebAssembly (WASM) as the primary application format for VantisOS. Applications will be distributed as .vnt files (WASM binaries) and run in a sandboxed WASM runtime.

## Motivation

VantisOS requires maximum security and formal verification. Traditional executable formats (ELF, PE) cannot guarantee security:

- **No sandboxing**: Applications can access hardware directly
- **Memory safety**: Buffer overflows and use-after-free vulnerabilities
- **Verification**: Cannot verify applications before execution
- **Portability**: Binaries are architecture-specific

WebAssembly provides:
- **Sandboxing**: Applications run in isolated sandbox
- **Deterministic execution**: Same input = same output
- **Formal verification**: WASM bytecode can be verified
- **Portability**: Write once, run on any architecture

## Detailed Design

### Architecture

```
Application (WASM .vnt)
    ↓
VantisOS WASM Runtime
    ↓
System Calls (via capability-based IPC)
    ↓
Kernel
```

### WASM Runtime Features

1. **Sandboxed execution**: Applications run in isolated WASM sandbox
2. **Native IPC**: Integration with VantisOS capability-based IPC
3. **No browser overhead**: Optimized for OS applications
4. **System call interface**: Safe syscall ABI for OS access
5. **AOT compilation**: Ahead-of-time compilation for performance
6. **Streaming compilation**: Compile while loading for fast startup

### Application Development

**Supported Languages**:
- Rust (primary)
- C/C++
- AssemblyScript
- Any language that compiles to WASM

**Development Workflow**:
1. Write application in supported language
2. Compile to WASM
3. Package as .vnt (WASM + metadata)
4. Deploy to VantisOS
5. Application runs safely in sandbox

### Security Properties

- **Memory safety**: WASM prevents buffer overflows
- **Type safety**: WASM enforces type safety
- **Sandbox isolation**: Applications cannot access hardware
- **Capability-based access**: System access via capabilities
- **Verification**: Can verify bytecode before execution

### Performance

**Optimizations**:
- AOT compilation (compile to native code)
- Streaming compilation (compile while loading)
- JIT compilation for hot paths
- Zero-copy IPC with kernel
- SIMD and multi-threading support

**Expected Performance**:
- Startup: < 100ms (streaming AOT)
- Execution: 80-90% of native performance
- Memory: 2-3x overhead (sandbox)

### Compatibility

**Format**: `.vnt` = WASM binary + VantisOS metadata

**Metadata includes**:
- Application manifest
- Required capabilities
- Version information
- Dependencies

## Drawbacks

1. **Performance overhead**: WASM has overhead vs native code
2. **Development overhead**: Requires WASM toolchain
3. **Library support**: Not all libraries support WASM
4. **Debugging**: WASM debugging is harder than native
5. **Legacy apps**: Existing apps must be recompiled

## Rationale and Alternatives

### Why WASM over other options?

**Alternative 1: Native Machine Code**
- **Pros**: Maximum performance, no overhead
- **Cons**: No safety, cannot sandbox, cannot verify
- **Rejected**: Security is non-negotiable

**Alternative 2: ELF with Sandboxing**
- **Pros**: Native performance, familiar
- **Cons**: Hard to sandbox securely, complex
- **Rejected**: WASM provides better safety

**Alternative 3: Java/JVM**
- **Pros**: Mature, portable
- **Cons**: Garbage collector, slower startup
- **Rejected**: WASM is more modern and flexible

**Alternative 4: Lua/Python Scripts**
- **Pros**: Easy to write
- **Cons**: Too slow, no type safety
- **Rejected**: Performance critical for OS

## Prior Art

- **WebAssembly**: Web standard for sandboxed code
- **Wasmtime**: Standalone WASM runtime
- **WAVM**: High-performance WASM VM
- **Linux with WASM**: Some Linux distributions experimenting with WASM
- **Fuchsia OS**: Uses component-based model (similar concepts)

## Unresolved Questions

None - design is complete and ready for implementation.

## Implementation Plan

### Phase 1: WASM Runtime Core (4 weeks)

**Timeline**: Weeks 1-4

**Milestones**:
- [ ] WASM parser and validator
- [ ] Basic runtime with AOT compilation
- [ ] Sandbox isolation
- [ ] System call interface

### Phase 2: IPC Integration (2 weeks)

**Timeline**: Weeks 5-6

**Milestones**:
- [ ] Capability-based IPC integration
- [ ] Zero-copy message passing
- [ ] Async/sync system calls

### Phase 3: Performance Optimization (3 weeks)

**Timeline**: Weeks 7-9

**Milestones**:
- [ ] Streaming AOT compilation
- [ ] JIT for hot paths
- [ ] SIMD and multi-threading

### Phase 4: Tooling (2 weeks)

**Timeline**: Weeks 10-11

**Milestones**:
- [ ] .vnt packaging tool
- [ ] Development toolchain
- [ ] Debugging tools

### Phase 5: Testing (2 weeks)

**Timeline**: Weeks 12-13

**Milestones**:
- [ ] Unit tests
- [ ] Integration tests
- [ ] Performance benchmarks

**Total**: 13 weeks

## Testing

1. **Unit tests**: Test each component
2. **Integration tests**: Test WASM runtime with kernel
3. **Performance tests**: Benchmark against native
4. **Security tests**: Verify sandbox isolation
5. **Formal verification**: Verify critical properties

## Risks and Mitigations

### Risk 1: Performance overhead too high

**Mitigation**: AOT compilation, JIT, zero-copy IPC

### Risk 2: Library support insufficient

**Mitigation**: Prioritize Rust ecosystem, provide native libraries

### Risk 3: Debugging difficulty

**Mitigation**: Good debugging tools, native-like debugging experience

### Risk 4: Developer adoption

**Mitigation**: Excellent documentation, tooling, examples

## Success Criteria

- [ ] WASM runtime performance: ≥ 80% of native
- [ ] Security: No sandbox escapes in testing
- [ ] Formal verification: Key properties verified
- [ ] Developer adoption: 10+ apps in first month
- [ ] Bug count: < 5 critical bugs in first year

## Dependencies

- **ADR-0001**: Use Rust as primary language
- **ADR-0002**: Adopt microkernel architecture
- **ADR-0004**: Capability-based IPC system

## References

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [Wasmtime Runtime](https://wasmtime.dev/)
- [WAVM](https://github.com/wavm/wavm)
- [ADR-0008: WebAssembly as Primary Application Format](../adr/0008-webassembly-primary-application-format.md)

## Appendix

### Example Application

```rust
// hello.vnt
fn main() {
    print!("Hello, VantisOS!");
}
```

**Compile**:
```bash
rustc --target wasm32-unknown-unknown -o hello.wasm hello.rs
vantis-package hello.wasm -o hello.vnt
```

**Run**:
```bash
vantis-run hello.vnt
```

---

**Discussion**: https://github.com/vantisCorp/VantisOS/discussions/1
**Issue**: https://github.com/vantisCorp/VantisOS/issues/1
**PR**: https://github.com/vantisCorp/VantisOS/pull/1

**Last Updated**: 2025-02-24