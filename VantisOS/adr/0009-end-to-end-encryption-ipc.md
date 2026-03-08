# ADR-0009: End-to-End Encryption for All IPC

## Status

**Accepted**

## Context

In traditional operating systems, IPC (Inter-Process Communication) is often unencrypted:
- Processes on the same machine assume trusted environment
- No confidentiality guarantees for inter-process communication
- Eavesdropping possible via compromised kernel
- Message tampering difficult to detect

VantisOS requires:
- Strong isolation between processes
- Protection even if kernel is compromised
- Verification of message integrity
- Authentication of message senders

## Decision

VantisOS will use **end-to-end encryption (E2EE) for all IPC**:

**Encryption Requirements**:
1. **All IPC encrypted**: Every message is encrypted end-to-end
2. **Per-pair keys**: Each communicating pair has unique keys
3. **Key rotation**: Keys rotated regularly for forward secrecy
4. **Message authentication**: All messages cryptographically signed
5. **Perfect forward secrecy**: Compromise of one key doesn't compromise past/future messages

**Cryptographic Choices**:
- **Encryption**: AES-256-GCM (authenticated encryption)
- **Key exchange**: X25519 (Curve25519) for ECDH
- **Signatures**: Ed25519 for message authentication
- **Hash**: SHA-256 for key derivation
- **RNG**: ChaCha20-Poly1305 CSPRNG for random values

**Key Management**:
- **Key derivation**: HKDF (HMAC-based Key Derivation Function)
- **Key storage**: Keys stored in kernel capability structures
- **Key rotation**: Every 24 hours or 10,000 messages
- **Secure deletion**: Keys securely erased when destroyed

## Consequences

### Positive
- **Strong confidentiality**: Kernel cannot read IPC messages
- **Message integrity**: Tampering detected immediately
- **Authentication**: Sender identity verified
- **Isolation**: Compromised kernel cannot decrypt IPC
- **Regulatory**: Meets requirements for high-security industries

### Negative
- **Performance overhead**: Encryption/decryption adds latency
- **CPU usage**: Crypto operations consume CPU cycles
- **Key complexity**: Key management adds complexity
- **Debugging**: Harder to debug encrypted IPC

### Affected Systems
- IPC system (all messaging)
- Performance (encryption overhead)
- Key management (capability system)
- Security model (E2EE guarantees)
- Scheduler (priority with encryption time)

## Alternatives Considered

### No Encryption
- **Pros**: Maximum performance, simple
- **Cons**: No confidentiality, no integrity
- **Rejected**: Security is non-negotiable

### Kernel-Level Encryption
- **Pros**: Simpler, kernel can decrypt
- **Cons**: Kernel is single point of compromise
- **Rejected**: Want E2EE, not just encryption in transit

### Application-Level Encryption
- **Pros**: Flexible, applications choose
- **Cons**: Inconsistent, mistakes possible
- **Rejected**: All IPC must be encrypted for consistency

### Hybrid (Critical Only)
- **Pros**: Better performance for non-critical IPC
- **Cons**: Complex, hard to categorize
- **Rejected**: Simpler to encrypt all IPC

## Related Decisions

- **ADR-0004**: Capability-based IPC system
- **ADR-0005**: Formal verification with Verus/Kani
- **ADR-0010**: Cryptographic module verification

## References

- [AES-GCM Specification](https://csrc.nist.gov/publications/detail/sp/800-38d/final)
- [Curve25519 for ECDH](https://cr.yp.to/ecdh.html)
- [Ed25519 Signatures](https://ed25519.cr.yp.to/)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [ ] Implemented
- [ ] Verified

---

**Author**: VantisOS Team  
**Date**: 2024-05-01  
**Last Updated**: 2025-02-24