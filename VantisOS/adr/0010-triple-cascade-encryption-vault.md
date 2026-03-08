# ADR-0010: Triple Cascade Encryption for Vantis Vault

## Status

**Accepted**

## Context

Vantis Vault is the secure storage subsystem for:
- Encryption keys
- Passwords and credentials
- Security-sensitive data
- User secrets

Requirements:
- **Maximum security**: Vault must be impenetrable
- **Defense in depth**: Multiple layers of encryption
- **Resilience**: Survive compromise of one cipher
- **Performance**: Must still be reasonably fast

Single encryption algorithms have known weaknesses and potential future attacks:
- AES-256 may have side-channel attacks
- Twofish and Serpent are less widely analyzed
- Quantum computers may break current algorithms

## Decision

Vantis Vault will use **Triple Cascade Encryption**:

**Encryption Chain**:
1. **AES-256-GCM**: First layer (fast, widely analyzed)
2. **Twofish-256**: Second layer (different structure)
3. **Serpent-256**: Third layer (most conservative design)

**Why Cascade?**:
- **Redundancy**: Compromise of one cipher doesn't break all
- **Diversity**: Different cipher structures reduce common weaknesses
- **Future-proof**: Quantum-resistant design
- **Defense in depth**: Three independent layers

**Performance Optimization**:
- **Hardware acceleration**: Use AES-NI for AES layer
- **Parallelization**: Encrypt/decrypt in parallel where possible
- **Caching**: Cache derived keys for performance
- **Asynchronous**: Vault operations are non-blocking

**Key Management**:
- **Independent keys**: Each cipher uses unique key
- **Key derivation**: PBKDF2 with 100,000 iterations
- **Key separation**: Keys derived from master key via HKDF
- **Secure storage**: Keys stored in TPM 2.0 when available

## Consequences

### Positive
- **Maximum security**: Breaking vault requires breaking all three ciphers
- **Resilience**: Survives cryptanalysis of one cipher
- **Future-proof**: Protection against quantum computing advances
- **Confidence**: Highest possible security for sensitive data

### Negative
- **Performance**: 3x encryption overhead
- **Complexity**: More complex than single encryption
- **Key management**: Must manage three keys per encryption
- **Implementation**: More code = more potential bugs

### Affected Systems
- Vantis Vault (core encryption)
- Performance (3x slower encryption)
- Key management (triple keys)
- User experience (slower vault operations)

## Alternatives Considered

### Single AES-256
- **Pros**: Fast, widely analyzed, hardware accelerated
- **Cons**: Single point of failure, quantum-vulnerable
- **Rejected**: Not secure enough for vault

### Double Encryption
- **Pros**: Better than single, less overhead than triple
- **Cons**: Still not maximum security
- **Rejected**: Want maximum security for vault

### Post-Quantum Algorithms
- **Pros**: Quantum-resistant
- **Cons**: New, less analyzed, slower
- **Rejected**: Not yet mature enough for production

### Triple Encryption with Same Cipher
- **Pros**: Simpler key management
- **Cons**: Same weaknesses repeated
- **Rejected**: Different ciphers provide diversity

## Related Decisions

- **ADR-0009**: End-to-end encryption for IPC
- **ADR-0011**: Vantis Vault implementation

## References

- [Triple DES](https://en.wikipedia.org/wiki/Triple_DES)
- [Twofish Cipher](https://www.schneier.com/academic/twofish/)
- [Serpent Cipher](https://www.cl.cam.ac.uk/~rja14/serpent.html)
- [NIST Post-Quantum Cryptography](https://csrc.nist.gov/projects/post-quantum-cryptography)

## Implementation Status

- [x] Proposed
- [x] Accepted
- [x] Implemented
- [x] Tested

---

**Author**: VantisOS Team  
**Date**: 2024-05-15  
**Last Updated**: 2025-02-24