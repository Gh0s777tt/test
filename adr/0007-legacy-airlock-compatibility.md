# ADR-0007: Legacy Airlock for Application Compatibility

## Status

**Accepted**

## Context

VantisOS rejects POSIX (ADR-0003), which means:
- No native Linux/Unix applications work
- No Windows applications work
- Developers must port all software
- Adoption barrier for users

However, for practical adoption, we need a way to run existing software.

## Decision

VantisOS will provide **Legacy Airlock**, a compatibility subsystem for running:

**Supported Formats**:
1. **Linux ELF binaries** (via containerization)
2. **Windows PE executables** (via translation layer)
3. **WebAssembly applications** (native .vnt format)
4. **Android APK** (via runtime)

**Architecture**:
- **Sandboxed execution**: All legacy code runs in isolated sandbox
- **Minimal compatibility layer**: Only essential APIs implemented
- **No security compromises**: Legacy code cannot compromise VantisOS
- **Performance overhead accepted**: Security > performance for legacy

**Priority**:
1. WebAssembly (native .vnt) - Primary format
2. Linux ELF - High priority (containerization)
3. Windows PE - Medium priority (translation)
4. Android APK - Low priority (runtime)

## Consequences

### Positive
- **Lower adoption barrier**: Users can run existing software
- **Gradual migration**: Legacy apps during transition period
- **Developer flexibility**: Not forced to port immediately
- **Ecosystem growth**: More software available

### Negative
- **Complexity**: Compatibility layer is complex to implement
- **Performance**: Sandbox adds overhead
- **Maintenance**: Must track Linux/Windows API changes
- **Security surface**: Larger attack surface from compatibility code
- **Verification**: Harder to verify compatibility layer

### Affected Systems
- Security model (sandbox isolation)
- IPC system (compatibility layer communication)
- User-space services (compatibility daemons)
- Developer experience (porting vs compatibility)

## Alternatives Considered

### No Compatibility (Pure VantisOS)
- **Pros**: Simple, secure, clean architecture
- **Cons**: No existing software works
- **Rejected**: Adoption would be impossible

### Full Linux Kernel Module
- **Pros**: Perfect Linux compatibility
- **Cons**: Compromises VantisOS security, complex
- **Rejected**: Defeats purpose of VantisOS

### Wine for Windows
- **Pros**: Existing project, proven
- **Cons**: Too complex, Linux-specific
- **Rejected**: Want VantisOS-native solution

### Our own compatibility layer
- **Pros**: Tailored to VantisOS, minimal APIs
- **Cons**: Development effort
- **Accepted**: Best balance of security and compatibility

## Related Decisions

- **ADR-0003**: Reject POSIX compliance
- **ADR-0002**: Adopt microkernel architecture
- **ADR-0009**: WebAssembly applications (.vnt format)

## References

- [Wine Compatibility Layer](https://www.winehq.org/)
- [Box86/Box64 for ARM](https://github.com/ptitSeb/box64)
- [FEX-Emu for ARM](https://github.com/FEX-Emu/FEX)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [ ] Implemented
- [ ] Tested

---

**Author**: VantisOS Team  
**Date**: 2024-04-01  
**Last Updated**: 2025-02-24