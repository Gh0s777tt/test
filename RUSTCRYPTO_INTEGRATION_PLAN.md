# 🔐 RustCrypto Integration Plan
## Production-Grade Cryptography for VANTIS OS

---

## 🎯 Objective

Replace demo/placeholder cryptographic implementations with production-grade RustCrypto libraries to achieve FIPS 140-3 compliance and production readiness.

---

## 📋 Current State

### Existing Modules
1. **vault_aes.rs** - Placeholder AES-256-CBC implementation
2. **vault_twofish.rs** - Placeholder Twofish-256-CBC implementation
3. **vault_serpent.rs** - Placeholder Serpent-256-CBC implementation
4. **vault_cascade.rs** - Cascade encryption framework (uses placeholders)
5. **vault_production_example.rs** - Example code with RustCrypto (commented out)

### Dependencies Already Added
```toml
aes = { version = "0.8", features = ["cbc"] }
twofish = { version = "0.7" }
serpent = { version = "0.5" }
cipher = { version = "0.4", features = ["block-padding"] }
rand = { version = "0.8", features = ["std", "std_rng"] }
```

---

## 🔧 Integration Strategy

### Phase 1: AES-256-CBC Integration (1 hour)
**Goal**: Replace placeholder AES with real RustCrypto implementation

**Tasks**:
1. Update vault_aes.rs with real AES-256-CBC
2. Implement proper IV generation with rand
3. Add PKCS#7 padding
4. Enable AES-NI hardware acceleration
5. Update tests with real test vectors
6. Verify with Verus + Kani

**Expected Result**: Production-ready AES-256-CBC with hardware acceleration

---

### Phase 2: Twofish-256-CBC Integration (45 minutes)
**Goal**: Replace placeholder Twofish with real RustCrypto implementation

**Tasks**:
1. Update vault_twofish.rs with real Twofish-256-CBC
2. Implement proper IV generation
3. Add PKCS#7 padding
4. Update tests with real test vectors
5. Verify with Verus + Kani

**Expected Result**: Production-ready Twofish-256-CBC

---

### Phase 3: Serpent-256-CBC Integration (45 minutes)
**Goal**: Replace placeholder Serpent with real RustCrypto implementation

**Tasks**:
1. Update vault_serpent.rs with real Serpent-256-CBC
2. Implement proper IV generation
3. Add PKCS#7 padding
4. Update tests with real test vectors
5. Verify with Verus + Kani

**Expected Result**: Production-ready Serpent-256-CBC

---

### Phase 4: Cascade Integration Update (30 minutes)
**Goal**: Update cascade encryption to use real implementations

**Tasks**:
1. Update vault_cascade.rs to use real crypto modules
2. Verify three-layer encryption works correctly
3. Update tests with real test vectors
4. Benchmark performance with hardware acceleration
5. Verify with Verus + Kani

**Expected Result**: Production-ready cascade encryption

---

### Phase 5: FIPS 140-3 Preparation (30 minutes)
**Goal**: Prepare for FIPS 140-3 certification

**Tasks**:
1. Add self-tests for all algorithms
2. Implement known-answer tests (KAT)
3. Add continuous random number generator tests
4. Document FIPS 140-3 compliance
5. Create certification checklist

**Expected Result**: FIPS 140-3 ready cryptographic module

---

## 📊 Implementation Details

### AES-256-CBC with RustCrypto

```rust
use aes::Aes256;
use cipher::{
    BlockEncryptMut, BlockDecryptMut, KeyIvInit,
    block_padding::Pkcs7,
};
use rand::RngCore;

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

pub fn encrypt_aes256_cbc(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    // Generate random IV
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    
    // Create encryptor
    let encryptor = Aes256CbcEnc::new(key.into(), &iv.into());
    
    // Encrypt with PKCS#7 padding
    let ciphertext = encryptor.encrypt_padded_vec_mut::<Pkcs7>(plaintext);
    
    // Prepend IV to ciphertext
    let mut result = Vec::with_capacity(16 + ciphertext.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

pub fn decrypt_aes256_cbc(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if data.len() < 16 {
        return Err(CryptoError::InvalidLength);
    }
    
    // Extract IV and ciphertext
    let (iv, ciphertext) = data.split_at(16);
    
    // Create decryptor
    let decryptor = Aes256CbcDec::new(key.into(), iv.into());
    
    // Decrypt and remove padding
    let plaintext = decryptor.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed)?;
    
    Ok(plaintext)
}
```

### Hardware Acceleration

```rust
// Enable AES-NI when available
#[cfg(target_feature = "aes")]
use aes::Aes256;

// Fallback to software implementation
#[cfg(not(target_feature = "aes"))]
use aes::soft::Aes256;
```

### FIPS 140-3 Self-Tests

```rust
pub fn fips_self_test() -> Result<(), CryptoError> {
    // Known-Answer Test (KAT) for AES
    let key = hex::decode("603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4")?;
    let plaintext = hex::decode("6bc1bee22e409f96e93d7e117393172a")?;
    let expected = hex::decode("f58c4c04d6e5f1ba779eabfb5f7bfbd6")?;
    
    let ciphertext = encrypt_aes256_cbc(&key, &plaintext)?;
    assert_eq!(&ciphertext[16..], &expected[..]);
    
    // Test all algorithms
    test_twofish_kat()?;
    test_serpent_kat()?;
    test_cascade_kat()?;
    
    Ok(())
}
```

---

## 🔬 Verification Strategy

### Formal Verification with Verus

```rust
verus! {

#[verifier::spec]
pub open spec fn encrypt_decrypt_roundtrip_spec(key: &[u8; 32], plaintext: &[u8]) -> bool {
    let ciphertext = encrypt_aes256_cbc(key, plaintext);
    let decrypted = decrypt_aes256_cbc(key, &ciphertext);
    decrypted == plaintext
}

pub fn verify_roundtrip(key: &[u8; 32], plaintext: &[u8]) -> (success: bool)
    ensures success == encrypt_decrypt_roundtrip_spec(key, plaintext)
{
    let ciphertext = encrypt_aes256_cbc(key, plaintext).unwrap();
    let decrypted = decrypt_aes256_cbc(key, &ciphertext).unwrap();
    decrypted == plaintext
}

}
```

### Kani Verification

```rust
#[cfg(kani)]
#[kani::proof]
fn verify_aes_roundtrip() {
    let key: [u8; 32] = kani::any();
    let plaintext: Vec<u8> = kani::any();
    kani::assume(plaintext.len() <= 1024);
    
    let ciphertext = encrypt_aes256_cbc(&key, &plaintext).unwrap();
    let decrypted = decrypt_aes256_cbc(&key, &ciphertext).unwrap();
    
    assert_eq!(plaintext, decrypted);
}
```

---

## 📈 Expected Performance

### With Hardware Acceleration (AES-NI)

| Operation | Size | Time | Throughput |
|-----------|------|------|------------|
| AES Encrypt | 1KB | ~1μs | ~1 GB/s |
| AES Decrypt | 1KB | ~1μs | ~1 GB/s |
| Twofish Encrypt | 1KB | ~5μs | ~200 MB/s |
| Twofish Decrypt | 1KB | ~5μs | ~200 MB/s |
| Serpent Encrypt | 1KB | ~8μs | ~125 MB/s |
| Serpent Decrypt | 1KB | ~8μs | ~125 MB/s |
| Cascade Encrypt | 1KB | ~14μs | ~71 MB/s |
| Cascade Decrypt | 1KB | ~14μs | ~71 MB/s |

### Without Hardware Acceleration

| Operation | Size | Time | Throughput |
|-----------|------|------|------------|
| AES Encrypt | 1KB | ~10μs | ~100 MB/s |
| AES Decrypt | 1KB | ~10μs | ~100 MB/s |
| Cascade Encrypt | 1KB | ~23μs | ~43 MB/s |
| Cascade Decrypt | 1KB | ~23μs | ~43 MB/s |

---

## ✅ Success Criteria

### Functional Requirements
- ✅ All algorithms encrypt/decrypt correctly
- ✅ Roundtrip tests pass for all algorithms
- ✅ IV generation is cryptographically secure
- ✅ Padding is correct (PKCS#7)
- ✅ Cascade encryption works end-to-end

### Security Requirements
- ✅ No key material leakage
- ✅ Secure IV generation (unique per encryption)
- ✅ Proper padding to prevent oracle attacks
- ✅ Constant-time operations where possible
- ✅ Secure memory zeroization

### Performance Requirements
- ✅ AES with AES-NI: >500 MB/s
- ✅ Cascade encryption: >50 MB/s
- ✅ Encryption overhead: <20μs per KB
- ✅ Memory usage: <1KB per operation

### Verification Requirements
- ✅ All functions formally verified with Verus
- ✅ All edge cases tested with Kani
- ✅ 100% test coverage maintained
- ✅ Zero unsafe code (except zeroization)
- ✅ FIPS 140-3 self-tests pass

---

## 🎯 Deliverables

### Code
1. Updated vault_aes.rs with RustCrypto
2. Updated vault_twofish.rs with RustCrypto
3. Updated vault_serpent.rs with RustCrypto
4. Updated vault_cascade.rs with real crypto
5. FIPS 140-3 self-test module

### Tests
1. Real test vectors for all algorithms
2. Roundtrip tests
3. Known-answer tests (KAT)
4. Performance benchmarks
5. Kani verification harnesses

### Documentation
1. Implementation guide
2. FIPS 140-3 compliance documentation
3. Performance benchmarks
4. API documentation
5. Security considerations

---

## 📊 Project Impact

### Before Integration
```
Vantis Vault:        Framework complete, demo crypto
FIPS 140-3:         Not ready
Production Ready:    No
Hardware Accel:      No
```

### After Integration
```
Vantis Vault:        Production-ready with RustCrypto
FIPS 140-3:         Ready for certification
Production Ready:    Yes
Hardware Accel:      Yes (AES-NI)
```

### Functions Added
- ~5-8 new verified functions
- ~400-600 lines of production code
- ~20-30 new tests
- ~5,000 words of documentation

---

## ⏱️ Timeline

| Phase | Duration | Cumulative |
|-------|----------|------------|
| Phase 1: AES | 1 hour | 1 hour |
| Phase 2: Twofish | 45 min | 1h 45m |
| Phase 3: Serpent | 45 min | 2h 30m |
| Phase 4: Cascade | 30 min | 3h |
| Phase 5: FIPS | 30 min | 3h 30m |
| **Total** | **3.5 hours** | **3.5 hours** |

---

## 🚀 Let's Begin!

Starting with Phase 1: AES-256-CBC Integration

---

**Status**: Ready to implement  
**Estimated Time**: 3-4 hours  
**Expected Functions**: 5-8 verified functions  
**Expected Impact**: FIPS 140-3 ready, production-grade security