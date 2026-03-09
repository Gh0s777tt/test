# ADR-0019: Network Stack in User Space with eBPF/XDP

## Status

**Accepted**

## Context

Traditional OS network stacks:
- **In-kernel**: Network stack runs in kernel space
- **Complex**: Millions of lines of code in kernel
- **Vulnerable**: Kernel bugs compromise entire system
- **Hard to verify**: Too complex for formal verification
- **Slow to update**: Updating network stack requires kernel update

VantisOS requirements:
- **Memory safety**: Network stack must be formally verifiable
- **Security**: Vulnerabilities shouldn't compromise kernel
- **Performance**: Wire-speed packet processing
- **Flexibility**: Easy to update and extend
- **DDoS protection**: Built-in anti-DDoS capabilities

## Decision

VantisOS will implement a **Network Stack in User Space with eBPF/XDP**:

**Architecture**:
1. **XDP in kernel**: Minimal packet processing in kernel for performance
2. **Network stack in user space**: Full TCP/IP stack in user space
3. **eBPF/XDP filtering**: In-kernel packet filtering with eBPF
4. **Rust-native**: Network stack written in Rust
5. **Zero-copy**: Minimize packet copying for performance

**Components**:
- **XDP (eXpress Data Path)**: In-kernel packet filtering and forwarding
- **eBPF (Extended Berkeley Packet Filter)**: Programmable packet processing
- **TCP/IP stack**: Rust-native TCP/IP stack in user space
- **WiFi 7 support**: Modern wireless protocol support
- **eBPF/XDP anti-DDoS**: Built-in DDoS protection

**Performance Optimization**:
- **XDP fast path**: Common packets processed in kernel
- **Zero-copy**: Avoid packet copying where possible
- **Batching**: Process packets in batches
- **Hardware offload**: Offload to NIC when available

**Security**:
- **eBPF verification**: eBPF programs verified before loading
- **Capability-based**: Network capabilities control access
- **Isolation**: Network stack isolated from kernel
- **Formal verification**: Critical network components verified

## Consequences

### Positive
- **Security**: Network stack bugs don't compromise kernel
- **Formal verification**: User-space stack can be verified
- **Updatable**: Update network stack without kernel update
- **Flexible**: Easy to add new protocols
- **DDoS protection**: Built-in eBPF/XDP anti-DDoS

### Negative
- **Performance**: User-space stack has overhead
- **Complexity**: eBPF/XDP adds complexity
- **Context switches**: More context switches for networking
- **Kernel code**: XDP requires minimal kernel code

### Affected Systems
- Network stack (user space)
- Kernel (minimal XDP code)
- Security (eBPF verification)
- Performance (network performance)

## Alternatives Considered

### In-Kernel Network Stack
- **Pros**: Maximum performance, familiar
- **Cons**: Not verifiable, compromises kernel
- **Rejected**: Formal verification is non-negotiable

### Pure User-Space Stack
- **Pros**: No kernel code
- **Cons**: Poor performance
- **Rejected**: Performance is critical

### Kernel Bypass (DPDK)
- **Pros**: Maximum performance
- **Cons**: Bypasses kernel, not verifiable
- **Rejected**: Want verifiable networking

### Standard Linux Stack
- **Pros**: Mature, tested
- **Cons**: In kernel, not Rust, not verifiable
- **Rejected**: Want Rust and verification

## Related Decisions

- **ADR-0002**: Adopt microkernel architecture
- **ADR-0005**: Formal verification with Verus/Kani
- **ADR-0016**: IOMMU implementation

## References

- [XDP Documentation](https://www.kernel.org/doc/html/latest/networking/xdp-rx-metadata.html)
- [eBPF Documentation](https://ebpf.io/)
- [Rust Network Stack](https://github.com/smoltcp-rs/smoltcp)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [ ] Implemented
- [ ] Tested

---

**Author**: VantisOS Team  
**Date**: 2024-10-01  
**Last Updated**: 2025-02-24