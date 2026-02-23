# ADR-0012: Vendor-Agnostic Graphics Stack

## Status

**Accepted**

## Context

Traditional OS graphics stacks are vendor-specific:
- **Linux**: Multiple drivers (NVIDIA, AMD, Intel), inconsistent APIs
- **Windows**: DirectX, different driver models per vendor
- **macOS**: Metal, Apple-only

Problems:
- Applications must support multiple graphics APIs
- Vendor lock-in
- Inconsistent feature support across GPUs
- Driver bugs are hard to track

Modern graphics APIs provide abstraction:
- **Vulkan**: Cross-platform, explicit, low-level
- **DirectX 12**: Windows-only, similar to Vulkan
- **Metal**: Apple-only, similar to Vulkan

## Decision

VantisOS will use a **Vendor-Agnostic Graphics Stack**:

**Architecture**:
- **Vulkan as primary API**: All GPU access through Vulkan
- **Abstraction layer**: Single unified API for applications
- **Driver model**: Vendor drivers implement Vulkan interface
- **Feature detection**: Automatic GPU feature detection and fallback
- **Vendor-agnostic**: Applications work on any GPU with Vulkan driver

**Supported APIs**:
1. **Native Vulkan**: Primary API for VantisOS applications
2. **Vulkan compatibility**: Vulkan applications run natively
3. **DirectX 12 translation**: DX12 → Vulkan via DXVK-like layer
4. **Metal translation**: Metal → Vulkan (for compatibility, if needed)

**Ray Tracing Support**:
- **Vendor-agnostic ray tracing**: Abstraction over Vulkan ray tracing extensions
- **Fallback**: Software ray tracing if hardware unavailable
- **Feature detection**: Automatic detection of RT cores

## Consequences

### Positive
- **Unified API**: Single API for all applications
- **Vendor diversity**: Works on any GPU with Vulkan support
- **Easier development**: Developers learn one API
- **Better portability**: Applications work across hardware
- **Future-proof**: Vulkan is modern, actively developed

### Negative
- **Performance overhead**: Abstraction layer adds overhead
- **Feature limitations**: Limited to least-common-denominator features
- **Driver complexity**: Must translate to Vulkan
- **Missing features**: Some vendor-specific features unavailable

### Affected Systems
- Graphics stack (Vulkan-based)
- GUI/Horizon UI (uses abstraction)
- Driver development (Vulkan drivers)
- Game development (Vulkan API)

## Alternatives Considered

### Vendor-Specific Drivers
- **Pros**: Maximum performance per vendor
- **Cons**: Fragmentation, lock-in
- **Rejected**: Want unified API

### DirectX 12 Only
- **Pros**: Widely used in games
- **Cons**: Windows-only, vendor lock-in
- **Rejected**: Want cross-platform

### OpenGL Only
- **Pros**: Cross-platform, widely supported
- **Cons**: Outdated, no ray tracing
- **Rejected**: Vulkan is modern replacement

### Direct Hardware Access
- **Pros**: Maximum performance
- **Cons**: Impossible (vendor-specific)
- **Rejected**: Would require vendor-specific code

## Related Decisions

- **ADR-0008**: WebAssembly as primary application format
- **ADR-0013**: Ray tracing support

## References

- [Vulkan Specification](https://www.khronos.org/vulkan/)
- [Vulkan Tutorial](https://vulkan-tutorial.com/)
- [DXVK (DirectX to Vulkan)](https://github.com/doitsujin/dxvk)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [x] Implemented (partial)
- [ ] Tested

---

**Author**: VantisOS Team  
**Date**: 2024-06-15  
**Last Updated**: 2025-02-24