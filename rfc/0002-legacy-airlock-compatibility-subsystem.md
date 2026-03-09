# RFC-0002: Legacy Airlock Compatibility Subsystem

## Status

Accepted

## Author

VantisOS Team (@vantisTeam)

## Created

2025-02-24

## Summary

This RFC proposes the Legacy Airlock subsystem, a compatibility layer for running existing software on VantisOS. Legacy Airlock provides sandboxed execution of Linux ELF binaries, Windows PE executables, Android APKs, and other formats.

## Motivation

VantisOS rejects POSIX (RFC-0003) and uses WebAssembly (RFC-0001) for applications. However, for practical adoption, we need to run existing software:

- **User adoption**: Users have existing software they want to run
- **Gradual migration**: Time to port software to WASM
- **Developer flexibility**: Not forced to port immediately
- **Ecosystem growth**: More software available

## Detailed Design

### Architecture

```
Legacy Application (ELF/PE/APK)
    ↓
Legacy Airlock (Compatibility Layer)
    ↓
Sandbox (Isolated)
    ↓
System Calls (Translation)
    ↓
Kernel
```

### Supported Formats

**Priority 1: WebAssembly (.vnt)**
- Native VantisOS format
- Primary application format
- See RFC-0001

**Priority 2: Linux ELF Binaries**
- **Approach**: Containerization (like Docker but for binaries)
- **Sandbox**: Full OS-level sandbox
- **Syscall translation**: Translate Linux syscalls to VantisOS syscalls
- **Libraries**: Linux user-space libraries in container

**Priority 3: Windows PE Executables**
- **Approach**: Translation layer (like Wine but for VantisOS)
- **API translation**: Win32 API → VantisOS APIs
- **DLL loading**: Load Windows DLLs in sandbox
- **DirectX**: Translate DirectX to Vulkan

**Priority 4: Android APK**
- **Approach**: Runtime (Android Runtime for VantisOS)
- **Dalvik/ART**: Run Android bytecode
- **Android APIs**: Implement Android APIs on VantisOS
- **Apps**: Run Android applications

### Security Model

**Principles**:
1. **Sandboxed execution**: All legacy code runs in isolated sandbox
2. **Minimal compatibility**: Only essential APIs implemented
3. **No security compromises**: Legacy code cannot compromise VantisOS
4. **Performance overhead accepted**: Security > performance for legacy

**Sandbox Isolation**:
- Process isolation
- File system isolation (chroot)
- Network isolation (optional)
- Resource limits (CPU, memory)
- Capability-based access control

### API Translation

**Linux → VantisOS**:
- File I/O: Linux file syscalls → VantisOS file syscalls
- IPC: Unix sockets → VantisOS capability-based IPC
- Networking: Linux network syscalls → VantisOS network stack
- Process management: Linux process syscalls → VantisOS process API

**Windows → VantisOS**:
- Graphics: DirectX → Vulkan (VantisOS graphics)
- I/O: Win32 I/O → VantisOS I/O
- Threading: Windows threads → VantisOS threads
- Memory: Windows memory management → VantisOS memory

**Android → VantisOS**:
- Graphics: Android graphics → VantisOS graphics
- I/O: Android I/O → VantisOS I/O
- Sensors: Android sensors → VantisOS drivers
- Apps: Android lifecycle → VantisOS process model

### Performance

**Expected Overhead**:
- Linux ELF: 10-20% overhead (containerization)
- Windows PE: 20-30% overhead (API translation)
- Android APK: 30-40% overhead (runtime)
- WebAssembly: 10-20% overhead (but primary format)

**Optimizations**:
- AOT compilation for ELF/PE
- JIT for hot paths
- Caching of translated code
- Fast path for common operations

## Drawbacks

1. **Complexity**: Compatibility layer is complex to implement
2. **Performance**: Sandbox adds overhead
3. **Maintenance**: Must track Linux/Windows/Android API changes
4. **Security surface**: Larger attack surface from compatibility code
5. **Verification**: Harder to verify compatibility layer
6. **Dependency**: Dependent on external API specifications

## Rationale and Alternatives

### Why Legacy Airlock?

**Alternative 1: No Compatibility (Pure VantisOS)**
- **Pros**: Simple, secure, clean architecture
- **Cons**: No existing software works
- **Rejected**: Adoption would be impossible

**Alternative 2: Full Linux Kernel Module**
- **Pros**: Perfect Linux compatibility
- **Cons**: Compromises VantisOS security, complex
- **Rejected**: Defeats purpose of VantisOS

**Alternative 3: Wine for Windows**
- **Pros**: Existing project, proven
- **Cons**: Too complex, Linux-specific
- **Rejected**: Want VantisOS-native solution

**Alternative 4: Our own compatibility layer**
- **Pros**: Tailored to VantisOS, minimal APIs
- **Cons**: Development effort
- **Accepted**: Best balance of security and compatibility

## Prior Art

- **Wine**: Windows compatibility for Linux
- **Box86/Box64**: x86 emulation on ARM
- **FEX-Emu**: x86 emulation on ARM
- **Docker**: Containerization for Linux
- **WINE on macOS**: Windows apps on Mac

## Unresolved Questions

1. **How much of Linux/Windows/Android API to support?**
   - **Initial**: Core APIs only (10-20%)
   - **Goal**: Commonly used APIs (50-70%)
   - **Full**: All APIs (unlikely)

2. **How to handle missing APIs?**
   - **Approach**: Graceful degradation, app fails gracefully
   - **Documentation**: Clearly document supported APIs

3. **How to prioritize formats?**
   - **Decision**: WASM first, ELF second, PE third, APK last

## Implementation Plan

### Phase 1: Core Sandbox (4 weeks)

**Timeline**: Weeks 1-4

**Milestones**:
- [ ] Sandbox framework
- [ ] Process isolation
- [ ] File system isolation
- [ ] Resource limits

### Phase 2: Linux ELF Support (6 weeks)

**Timeline**: Weeks 5-10

**Milestones**:
- [ ] ELF loader
- [ ] Linux syscall translator
- [ ] Linux libraries in container
- [ ] Testing with common apps

### Phase 3: Windows PE Support (8 weeks)

**Timeline**: Weeks 11-18

**Milestones**:
- [ ] PE loader
- [ ] Win32 API translator
- [ ] DirectX → Vulkan translator
- [ ] DLL loader
- [ ] Testing with common apps

### Phase 4: Android APK Support (6 weeks)

**Timeline**: Weeks 19-24

**Milestones**:
- [ ] APK loader
- [ ] Android Runtime
- [ ] Android APIs implementation
- [ ] Testing with common apps

### Phase 5: Testing and Optimization (4 weeks)

**Timeline**: Weeks 25-28

**Milestones**:
- [ ] Comprehensive testing
- [ ] Performance optimization
- [ ] Security testing
- [ ] Documentation

**Total**: 28 weeks

## Testing

1. **Unit tests**: Test each translator component
2. **Integration tests**: Test legacy apps in sandbox
3. **Security tests**: Verify sandbox isolation
4. **Performance tests**: Benchmark overhead
5. **Compatibility tests**: Test with real applications

## Risks and Mitigations

### Risk 1: Complexity explosion

**Mitigation**: Start with minimal APIs, incremental approach

### Risk 2: Performance overhead too high

**Mitigation**: AOT compilation, caching, fast paths

### Risk 3: Security vulnerabilities in compatibility layer

**Mitigation**: Formal verification of core, fuzzing, security review

### Risk 4: Maintenance burden

**Mitigation**: Limit supported APIs, clear deprecation policy

## Success Criteria

- [ ] Linux ELF: Run 50+ common applications
- [ ] Windows PE: Run 20+ common applications
- [ ] Android APK: Run 10+ common applications
- [ ] Security: No sandbox escapes in testing
- [ ] Performance: < 30% overhead for most apps

## Dependencies

- **ADR-0002**: Adopt microkernel architecture (enables isolation)
- **ADR-0008**: WebAssembly as primary application format (native format)
- **RFC-0001**: WebAssembly as Primary Application Format
- **RFC-0003**: Reject POSIX Compliance

## References

- [Wine Compatibility Layer](https://www.winehq.org/)
- [Box86/Box64](https://github.com/ptitSeb/box64)
- [FEX-Emu](https://github.com/FEX-Emu/FEX)
- [Docker](https://www.docker.com/)
- [ADR-0007: Legacy Airlock for Compatibility](../adr/0007-legacy-airlock-compatibility.md)

## Appendix

### Example Usage

**Run Linux ELF**:
```bash
legacy-run --format elf myapp
```

**Run Windows PE**:
```bash
legacy-run --format pe myapp.exe
```

**Run Android APK**:
```bash
legacy-run --format apk myapp.apk
```

### Supported Applications (Planned)

**Linux**:
- Shell tools (bash, coreutils)
- Editors (vim, nano)
- Development tools (git, make)
- Browsers (Firefox, Chrome)

**Windows**:
- Notepad
- Calculator
- Simple games
- Utilities

**Android**:
- Simple apps
- Utilities
- (Complex apps later)

---

**Discussion**: https://github.com/vantisCorp/VantisOS/discussions/2
**Issue**: https://github.com/vantisCorp/VantisOS/issues/2
**PR**: https://github.com/vantisCorp/VantisOS/pull/2

**Last Updated**: 2025-02-24