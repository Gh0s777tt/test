# Post-Quantum Cryptography - VantisOS v1.5.0

## Overview

The VantisOS Post-Quantum Cryptography module provides quantum-resistant cryptographic algorithms that are secure against attacks from both classical and quantum computers. This module implements NIST-standardized algorithms and follows the latest cryptographic best practices.

## Architecture

```
src/verified/vault/
├── mod.rs              # Vault module exports
├── post_quantum.rs     # Main PQ crypto module
├── lattice.rs          # Lattice-based cryptography
├── hash_sig.rs         # Hash-based signatures
├── code_based.rs       # Code-based cryptography
└── multivariate.rs     # Multivariate cryptography
```

## Security Levels

VantisOS PQ crypto supports NIST security levels:

| Level | Classical Security | Quantum Security | Use Case |
|-------|-------------------|------------------|----------|
| Level 1 | 128 bits | 64 bits | Legacy systems, testing |
| Level 2 | 192 bits | 96 bits | Standard security |
| Level 3 | 256 bits | 128 bits | High security |
| Level 5 | 512 bits | 256 bits | Long-term security |

```rust
use vantis_vault::post_quantum::SecurityLevel;

let level = SecurityLevel::Level3;  // Recommended for most applications
```

## NIST Standardized Algorithms

### 1. Kyber (Key Encapsulation)

Kyber is a lattice-based key encapsulation mechanism (KEM) standardized by NIST for post-quantum key exchange.

#### Key Sizes

| Security Level | Public Key | Secret Key | Ciphertext |
|---------------|------------|------------|------------|
| Level 1 | 800 B | 1,632 B | 768 B |
| Level 2 | 1,184 B | 2,400 B | 1,088 B |
| Level 3 | 1,568 B | 3,168 B | 1,432 B |
| Level 5 | 2,400 B | 4,096 B | 2,080 B |

#### Usage

```rust
use vantis_vault::post_quantum::{Kyber, SecurityLevel, KeyEncapsulation};

// Generate key pair
let (public_key, secret_key) = Kyber::generate_keypair(SecurityLevel::Level3);

// Encapsulate shared secret
let (ciphertext, shared_secret) = Kyber::encapsulate(&public_key);

// Decapsulate shared secret
let recovered_secret = Kyber::decapsulate(&secret_key, &ciphertext);

assert_eq!(shared_secret, recovered_secret);
```

### 2. Dilithium (Digital Signatures)

Dilithium is a lattice-based digital signature scheme standardized by NIST.

#### Key and Signature Sizes

| Security Level | Public Key | Secret Key | Signature |
|---------------|------------|------------|-----------|
| Level 1 | 1,312 B | 2,528 B | 2,420 B |
| Level 2 | 1,952 B | 4,000 B | 3,293 B |
| Level 3 | 2,592 B | 4,864 B | 4,595 B |
| Level 5 | 3,968 B | 6,464 B | 8,163 B |

#### Usage

```rust
use vantis_vault::post_quantum::{Dilithium, SecurityLevel, DigitalSignature};

// Generate key pair
let (public_key, secret_key) = Dilithium::generate_keypair(SecurityLevel::Level3);

// Sign message
let message = b"Important document";
let signature = Dilithium::sign(&secret_key, message);

// Verify signature
let valid = Dilithium::verify(&public_key, message, &signature);
assert!(valid);
```

## Additional Algorithms

### 3. SPHINCS+ (Hash-Based Signatures)

SPHINCS+ is a stateless hash-based signature scheme providing:
- Stateless operation (no tracking required)
- Based solely on hash function security
- Larger signatures but simpler security assumptions

```rust
use vantis_vault::post_quantum::{SPHINCSPlus, DigitalSignature};

let (pk, sk) = SPHINCSPlus::generate_keypair(SecurityLevel::Level3);
let signature = SPHINCSPlus::sign(&sk, message);
let valid = SPHINCSPlus::verify(&pk, message, &signature);
```

| Security Level | Public Key | Signature Size |
|---------------|------------|----------------|
| Level 1 | 32 B | 7,856 B |
| Level 2 | 48 B | 16,224 B |
| Level 3 | 64 B | 29,792 B |
| Level 5 | 96 B | 49,216 B |

### 4. McEliece (Code-Based Encryption)

McEliece is a code-based public-key encryption scheme:
- Based on the difficulty of decoding linear codes
- Large public keys but fast operations
- Resistant to quantum attacks

```rust
use vantis_vault::post_quantum::{McEliece, KeyEncapsulation};

let (pk, sk) = McEliece::generate_keypair(SecurityLevel::Level3);
let (ciphertext, shared_secret) = McEliece::encapsulate(&pk);
let recovered = McEliece::decapsulate(&sk, &ciphertext);
```

| Security Level | Public Key | Secret Key |
|---------------|------------|------------|
| Level 1 | 261 KB | 6.4 KB |
| Level 2 | 524 KB | 13.7 KB |
| Level 3 | 1,043 KB | 20.5 KB |

### 5. XMSS (Hash-Based Signatures)

XMSS is a stateful hash-based signature scheme:
- Smaller signatures than SPHINCS+
- Limited number of signatures per key
- Suitable for specific use cases

```rust
use vantis_vault::hash_sig::{XMSS, WOTSPlus};

let xmss = XMSS::new(32, 10);  // 32-byte hash, height 10
let tree = xmss.generate_tree();

// Maximum signatures = 2^height
println!("Max signatures: {}", xmss.max_signatures());
```

### 6. Rainbow (Multivariate Signatures)

Rainbow is a multivariate polynomial signature scheme:
- Based on MQ problem hardness
- Small signatures
- Note: Some variants have been broken; use with caution

```rust
use vantis_vault::post_quantum::{Rainbow, DigitalSignature};

let (pk, sk) = Rainbow::generate_keypair(SecurityLevel::Level1);
let signature = Rainbow::sign(&sk, message);
```

## Hybrid Key Exchange

For transition security, combine classical and post-quantum algorithms:

```rust
use vantis_vault::post_quantum::{HybridKeyExchange, Kyber};

// Generate classical key pair (e.g., X25519)
let classical_pk = generate_classical_public_key();

// Generate PQ key pair
let (pq_pk, pq_sk) = Kyber::generate_keypair(SecurityLevel::Level3);

// Perform hybrid exchange
let combined_secret = HybridKeyExchange::exchange(&pq_pk, &classical_pk)?;
```

## Integration Patterns

### TLS Integration

```rust
use vantis_vault::post_quantum::{Kyber, SecurityLevel};

// Server-side: Generate PQ certificate
let (server_pk, server_sk) = Kyber::generate_keypair(SecurityLevel::Level3);

// Client-side: Perform PQ key exchange
let (ciphertext, shared_secret) = Kyber::encapsulate(&server_pk);

// Use shared_secret for TLS session keys
```

### VPN Integration

```rust
// Generate long-term PQ keys for VPN tunnel
let (pk, sk) = Kyber::generate_keypair(SecurityLevel::Level5);

// Establish secure tunnel with post-quantum forward secrecy
let tunnel = establish_pq_vpn_tunnel(pk, sk)?;
```

### Blockchain Integration

```rust
use vantis_vault::post_quantum::{Dilithium, DigitalSignature};

// Create quantum-resistant blockchain transaction
let tx = create_transaction();
let signature = Dilithium::sign(&wallet_sk, &tx.serialize());

// Verify on all nodes
let valid = Dilithium::verify(&wallet_pk, &tx.serialize(), &signature);
```

## Security Best Practices

### 1. Key Management

```rust
// Always use appropriate security level
let level = SecurityLevel::Level3;  // Recommended minimum

// Store secret keys securely
use vantis_vault::secure_storage;

secure_storage::store("kyber_sk", &secret_key)?;

// Rotate keys regularly
let (new_pk, new_sk) = Kyber::generate_keypair(level);
```

### 2. Hybrid Approach

```rust
// Use hybrid key exchange during transition period
let classical_secret = x25519_key_exchange();
let pq_secret = kyber_key_exchange();

// Combine both for maximum security
let combined = kdf(&classical_secret, &pq_secret);
```

### 3. Side-Channel Protection

```rust
// Use constant-time operations
use vantis_vault::constant_time;

// Verify signatures in constant time
let valid = constant_time::verify(signature, expected);
```

## Performance Considerations

### Algorithm Comparison

| Algorithm | KeyGen | Encap/Sign | Decap/Verify | Key Size | Sig Size |
|-----------|--------|------------|--------------|----------|----------|
| Kyber | Fast | Fast | Fast | Medium | Small |
| Dilithium | Fast | Medium | Fast | Medium | Medium |
| SPHINCS+ | Fast | Slow | Medium | Small | Large |
| McEliece | Slow | Fast | Fast | Large | Small |
| XMSS | Medium | Medium | Medium | Medium | Small |

### Optimization Tips

1. **Pre-compute keys**: Generate keys during initialization
2. **Batch operations**: Process multiple signatures efficiently
3. **Hardware acceleration**: Use AVX2/AVX-512 when available
4. **Caching**: Cache frequently used public keys

## Migration Guide

### From RSA to Dilithium

```rust
// Old RSA code
let rsa_key = RsaPrivateKey::new(&mut rng, 2048)?;
let signature = rsa_key.sign(padding, &hash)?;

// New Dilithium code
let (pk, sk) = Dilithium::generate_keypair(SecurityLevel::Level3);
let signature = Dilithium::sign(&sk, &hash);
```

### From ECDH to Kyber

```rust
// Old ECDH code
let shared_secret = ecdh(&private_key, &peer_public)?;

// New Kyber code
let (ciphertext, shared_secret) = Kyber::encapsulate(&peer_public);
// Send ciphertext to peer
let peer_secret = Kyber::decapsulate(&secret_key, &ciphertext);
```

## API Reference

### KeyEncapsulation Trait

```rust
pub trait KeyEncapsulation {
    type PublicKey;
    type SecretKey;
    type Ciphertext;

    fn generate_keypair(security_level: SecurityLevel) -> (Self::PublicKey, Self::SecretKey);
    fn encapsulate(public_key: &Self::PublicKey) -> (Self::Ciphertext, Vec<u8>);
    fn decapsulate(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Vec<u8>;
}
```

### DigitalSignature Trait

```rust
pub trait DigitalSignature {
    type PublicKey;
    type SecretKey;
    type Signature;

    fn generate_keypair(security_level: SecurityLevel) -> (Self::PublicKey, Self::SecretKey);
    fn sign(secret_key: &Self::SecretKey, message: &[u8]) -> Self::Signature;
    fn verify(public_key: &Self::PublicKey, message: &[u8], signature: &Self::Signature) -> bool;
}
```

## Testing

The PQ crypto module includes 250+ security tests:

```rust
#[test]
fn test_kyber_roundtrip() {
    let (pk, sk) = Kyber::generate_keypair(SecurityLevel::Level3);
    let (ct, ss1) = Kyber::encapsulate(&pk);
    let ss2 = Kyber::decapsulate(&sk, &ct);
    assert_eq!(ss1, ss2);
}

#[test]
fn test_dilithium_signature() {
    let (pk, sk) = Dilithium::generate_keypair(SecurityLevel::Level3);
    let message = b"test message";
    let sig = Dilithium::sign(&sk, message);
    assert!(Dilithium::verify(&pk, message, &sig));
}
```

## References

1. NIST Post-Quantum Cryptography Standardization (2024)
2. CRYSTALS-Kyber Specification
3. CRYSTALS-Dilithium Specification
4. SPHINCS+ Specification
5. Classic McEliece Specification

## License

Copyright (c) 2024 VantisOS Contributors