# ADR-0016: IOMMU Implementation for DMA Attack Prevention

## Status

**Accepted**

## Context

Direct Memory Access (DMA) allows devices to access system memory directly:
- **High performance**: No CPU involvement in data transfer
- **Essential for GPUs, NICs, storage**: Performance-critical

**Security problem**:
- **DMA attacks**: Malicious device can read/write arbitrary memory
- **Thunderbolt/USB4**: Hot-plug devices can attack via DMA
- **PCIe devices**: Can bypass OS memory protection

**Protection needed**:
- Restrict device memory access
- Prevent DMA from accessing kernel memory
- Enforce memory isolation per device

## Decision

VantisOS will implement **IOMMU (Input-Output Memory Management Unit)** for DMA protection:

**IOMMU Architecture**:
1. **Per-device IOMMU**: Each device has isolated IOMMU domain
2. **Memory isolation**: Devices can only access their allocated memory
3. **Kernel protection**: DMA cannot access kernel memory
4. **Hot-plug protection**: Thunderbolt/USB4 devices get isolated IOMMU
5. **Capability-based**: Device capabilities control IOMMU mappings

**Implementation**:
- **VT-d/AMD-Vi**: Use hardware IOMMU when available
- **Software fallback**: Software IOMMU for systems without hardware support
- **Page-level isolation**: IOMMU mappings at page granularity
- **Dynamic mapping**: Update mappings as needed


- **Secure boot integration**: IOMMU enabled from boot

**Supported Features**:
- **DMA remapping**: Restrict device memory access
- **Interrupt remapping**: Isolate device interrupts
- **Hot-plug**: Dynamic IOMMU for hot-plug devices
- **Passthrough**: Selective passthrough for trusted devices

**Verification**:
- **Formal verification**: Verify IOMMU isolation properties
- **Testing**: DMA attack simulation testing
- **Compliance**: Meets IOMMU security requirements

## Consequences

### Positive
- **DMA protection**: Prevents DMA attacks
- **Kernel isolation**: Devices cannot read kernel memory
- **Hot-plug security**: Thunderbolt/USB4 devices isolated
- **Compliance**: Meets security requirements for high-security industries

I'll refine the security implications, focusing on potential challenges and trade-offs of implementing IOMMU. The approach introduces complexity in device interaction, potentially increasing development overhead. Some legacy hardware might face compatibility issues, requiring careful mapping strategies. Debugging memory access becomes more intricate with IOMMU's isolation mechanisms, potentially complicating troubleshooting efforts for system developers.

The implementation impacts multiple system layers: device drivers must adapt to IOMMU constraints, memory management requires precise mapping, and the security model gains robust DMA protection. Hot-plug scenarios become more secure but demand more sophisticated handling, especially for Thunderbolt and USB4 interfaces. Comprehensive testing will be crucial to validate both performance and security characteristics.

Exploring alternative strategies reveals significant challenges. While complete IOMMU removal maximizes performance, it creates critical security vulnerabilities. Selective IOMMU implementation offers partial protection but fails to isolate all devices effectively. Software-only IOMMU provides universal compatibility but introduces substantial performance penalties. Disabling hot-plug functionality represents an extreme approach that would severely limit system flexibility and user experience. I'll continue the list by adding the last entry for AMD-Vi and IOMMU Specification:

- [AMD-Vi IOMMU](https://developer.amd.com/resources/what-is-amd-vi/)
- [IOMMU Specification](https://www.pcisig.com/specifications/iov)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [ ] Implemented
- [ ] Tested

---

**Author**: VantisOS Team  
**Date**: 2024-08-15  
**Last Updated**: 2025-02-24