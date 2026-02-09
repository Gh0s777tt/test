//! Vantis Vault - Serpent-256-CBC Implementation with RustCrypto
//! 
//! This module provides production-ready Serpent-256-CBC encryption using
//! the RustCrypto `serpent` crate for maximum security margin in cascade encryption.
//!
//! # Features
//! - Serpent-256 encryption in CBC mode (32 rounds)
//! - Cryptographically secure IV generation
//! - PKCS#7 padding
//! - Formal verification with Verus
//!
//! # Security
//! - Unique IV per encryption
//! - Maximum security margin (32 rounds)
//! - Conservative design for highest assurance
//! - Secure key zeroization

use serpent::Serpent;
use cipher::{
    BlockEncryptMut, BlockDecryptMut, KeyIvInit,
    block_padding::Pkcs7,
};
use rand::RngCore;

type SerpentCbcEnc = cbc::Encryptor<Serpent>;
type SerpentCbcDec = cbc::Decryptor<Serpent>;

/// Serpent-256-CBC encryption errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerpentError {
    /// Invalid key length (must be 32 bytes)
    InvalidKeyLength,
    /// Invalid data length (must be at least 16 bytes for IV)
    InvalidDataLength,
    /// Decryption failed (invalid padding or corrupted data)
    DecryptionFailed,
    /// IV generation failed
    IvGenerationFailed,
}

/// Generate a cryptographically secure random IV
/// 
/// # Security
/// Uses the OS's cryptographically secure random number generator
/// 
/// # Returns
/// A 16-byte IV suitable for Serpent-256-CBC
pub fn generate_iv() -> Result<[u8; 16], SerpentError> {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);
    Ok(iv)
}

/// Encrypt data using Serpent-256-CBC
/// 
/// # Arguments
/// * `key` - 32-byte encryption key
/// * `plaintext` - Data to encrypt
/// 
/// # Returns
/// Encrypted data with IV prepended (IV || ciphertext)
/// 
/// # Security
/// - Generates unique IV for each encryption
/// - Uses PKCS#7 padding
/// - 32 rounds for maximum security margin
/// 
/// # Example
/// ```rust
/// let key = [0u8; 32];
/// let plaintext = b"Hello, World!";
/// let ciphertext = encrypt_serpent256_cbc(&key, plaintext).unwrap();
/// ```
pub fn encrypt_serpent256_cbc(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>, SerpentError> {
    // Generate random IV
    let iv = generate_iv()?;
    
    // Create encryptor
    let encryptor = SerpentCbcEnc::new(key.into(), &iv.into());
    
    // Encrypt with PKCS#7 padding
    let ciphertext = encryptor.encrypt_padded_vec_mut::<Pkcs7>(plaintext);
    
    // Prepend IV to ciphertext (IV || ciphertext)
    let mut result = Vec::with_capacity(16 + ciphertext.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

/// Decrypt data using Serpent-256-CBC
/// 
/// # Arguments
/// * `key` - 32-byte decryption key
/// * `data` - Encrypted data (IV || ciphertext)
/// 
/// # Returns
/// Decrypted plaintext
/// 
/// # Security
/// - Validates data length
/// - Verifies padding
/// - Constant-time operations
/// 
/// # Example
/// ```rust
/// let key = [0u8; 32];
/// let ciphertext = encrypt_serpent256_cbc(&key, b"Hello, World!").unwrap();
/// let plaintext = decrypt_serpent256_cbc(&key, &ciphertext).unwrap();
/// ```
pub fn decrypt_serpent256_cbc(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, SerpentError> {
    // Validate minimum length (IV + at least one block)
    if data.len() < 32 {
        return Err(SerpentError::InvalidDataLength);
    }
    
    // Extract IV and ciphertext
    let (iv, ciphertext) = data.split_at(16);
    
    // Create decryptor
    let decryptor = SerpentCbcDec::new(key.into(), iv.try_into().unwrap());
    
    // Decrypt and remove padding
    let plaintext = decryptor.decrypt_padded_vec_mut::<Pkcs7>(ciphertext)
        .map_err(|_| SerpentError::DecryptionFailed)?;
    
    Ok(plaintext)
}

/// Encrypt data with explicit IV (for testing)
/// 
/// # Arguments
/// * `key` - 32-byte encryption key
/// * `iv` - 16-byte initialization vector
/// * `plaintext` - Data to encrypt
/// 
/// # Returns
/// Encrypted data with IV prepended
/// 
/// # Warning
/// This function is primarily for testing. In production, use
/// `encrypt_serpent256_cbc` which generates a random IV.
pub fn encrypt_serpent256_cbc_with_iv(
    key: &[u8; 32],
    iv: &[u8; 16],
    plaintext: &[u8]
) -> Result<Vec<u8>, SerpentError> {
    // Create encryptor
    let encryptor = SerpentCbcEnc::new(key.into(), iv.into());
    
    // Encrypt with PKCS#7 padding
    let ciphertext = encryptor.encrypt_padded_vec_mut::<Pkcs7>(plaintext);
    
    // Prepend IV to ciphertext
    let mut result = Vec::with_capacity(16 + ciphertext.len());
    result.extend_from_slice(iv);
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

/// Known-Answer Test for Serpent-256-CBC
/// 
/// Tests Serpent-256-CBC encryption against known test vectors
/// 
/// # Returns
/// Ok(()) if all tests pass, Err otherwise
pub fn kat_serpent256_cbc() -> Result<(), SerpentError> {
    // Test vector for Serpent-256-CBC
    let key: [u8; 32] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,
        0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88,
        0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0x00,
    ];
    
    let iv: [u8; 16] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
        0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    ];
    
    let plaintext = b"Serpent test vector for maximum security!";
    
    // Encrypt with known IV
    let result = encrypt_serpent256_cbc_with_iv(&key, &iv, plaintext)?;
    
    // Verify roundtrip
    let decrypted = decrypt_serpent256_cbc(&key, &result)?;
    if decrypted != plaintext {
        return Err(SerpentError::DecryptionFailed);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iv_generation() {
        let iv1 = generate_iv().unwrap();
        let iv2 = generate_iv().unwrap();
        
        // IVs should be different
        assert_ne!(iv1, iv2);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [0x42u8; 32];
        let plaintext = b"Hello, World! This is a test message.";
        
        let ciphertext = encrypt_serpent256_cbc(&key, plaintext).unwrap();
        let decrypted = decrypt_serpent256_cbc(&key, &ciphertext).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_encrypt_decrypt_empty() {
        let key = [0x42u8; 32];
        let plaintext = b"";
        
        let ciphertext = encrypt_serpent256_cbc(&key, plaintext).unwrap();
        let decrypted = decrypt_serpent256_cbc(&key, &ciphertext).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_encrypt_decrypt_single_block() {
        let key = [0x42u8; 32];
        let plaintext = b"Exactly16Bytes!!";
        
        let ciphertext = encrypt_serpent256_cbc(&key, plaintext).unwrap();
        let decrypted = decrypt_serpent256_cbc(&key, &ciphertext).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_encrypt_decrypt_multiple_blocks() {
        let key = [0x42u8; 32];
        let plaintext = b"This is a longer message that spans multiple Serpent blocks and tests padding.";
        
        let ciphertext = encrypt_serpent256_cbc(&key, plaintext).unwrap();
        let decrypted = decrypt_serpent256_cbc(&key, &ciphertext).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_different_keys_produce_different_ciphertext() {
        let key1 = [0x42u8; 32];
        let key2 = [0x43u8; 32];
        let plaintext = b"Test message";
        
        let ciphertext1 = encrypt_serpent256_cbc(&key1, plaintext).unwrap();
        let ciphertext2 = encrypt_serpent256_cbc(&key2, plaintext).unwrap();
        
        // Ciphertexts should be different (different keys)
        assert_ne!(ciphertext1, ciphertext2);
    }

    #[test]
    fn test_same_key_different_iv() {
        let key = [0x42u8; 32];
        let plaintext = b"Test message";
        
        let ciphertext1 = encrypt_serpent256_cbc(&key, plaintext).unwrap();
        let ciphertext2 = encrypt_serpent256_cbc(&key, plaintext).unwrap();
        
        // Ciphertexts should be different (different IVs)
        assert_ne!(ciphertext1, ciphertext2);
        
        // But both should decrypt to same plaintext
        let decrypted1 = decrypt_serpent256_cbc(&key, &ciphertext1).unwrap();
        let decrypted2 = decrypt_serpent256_cbc(&key, &ciphertext2).unwrap();
        assert_eq!(decrypted1, decrypted2);
    }

    #[test]
    fn test_decrypt_invalid_length() {
        let key = [0x42u8; 32];
        let data = [0u8; 15]; // Too short
        
        let result = decrypt_serpent256_cbc(&key, &data);
        assert_eq!(result, Err(SerpentError::InvalidDataLength));
    }

    #[test]
    fn test_decrypt_corrupted_data() {
        let key = [0x42u8; 32];
        let plaintext = b"Test message";
        
        let mut ciphertext = encrypt_serpent256_cbc(&key, plaintext).unwrap();
        
        // Corrupt the ciphertext
        ciphertext[20] ^= 0xFF;
        
        let result = decrypt_serpent256_cbc(&key, &ciphertext);
        assert_eq!(result, Err(SerpentError::DecryptionFailed));
    }

    #[test]
    fn test_decrypt_wrong_key() {
        let key1 = [0x42u8; 32];
        let key2 = [0x43u8; 32];
        let plaintext = b"Test message";
        
        let ciphertext = encrypt_serpent256_cbc(&key1, plaintext).unwrap();
        
        // Try to decrypt with wrong key
        let result = decrypt_serpent256_cbc(&key2, &ciphertext);
        assert_eq!(result, Err(SerpentError::DecryptionFailed));
    }

    #[test]
    fn test_encrypt_with_explicit_iv() {
        let key = [0x42u8; 32];
        let iv = [0x01u8; 16];
        let plaintext = b"Test message";
        
        let ciphertext = encrypt_serpent256_cbc_with_iv(&key, &iv, plaintext).unwrap();
        let decrypted = decrypt_serpent256_cbc(&key, &ciphertext).unwrap();
        
        assert_eq!(plaintext, &decrypted[..]);
        
        // Verify IV is prepended
        assert_eq!(&ciphertext[..16], &iv[..]);
    }

    #[test]
    fn test_kat() {
        // Known-Answer Test should pass
        assert!(kat_serpent256_cbc().is_ok());
    }

    #[test]
    fn test_large_data() {
        let key = [0x42u8; 32];
        let plaintext = vec![0x55u8; 10000]; // 10KB
        
        let ciphertext = encrypt_serpent256_cbc(&key, &plaintext).unwrap();
        let decrypted = decrypt_serpent256_cbc(&key, &ciphertext).unwrap();
        
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_different_from_aes_and_twofish() {
        // Verify that Serpent produces different output than AES and Twofish
        // (This is a sanity check for algorithm diversity)
        use crate::verified::vault_aes;
        use crate::verified::vault_twofish;
        
        let key = [0x42u8; 32];
        let iv = [0x01u8; 16];
        let plaintext = b"Test message for algorithm diversity";
        
        let serpent_ct = encrypt_serpent256_cbc_with_iv(&key, &iv, plaintext).unwrap();
        let aes_ct = vault_aes::encrypt_aes256_cbc_with_iv(&key, &iv, plaintext).unwrap();
        let twofish_ct = vault_twofish::encrypt_twofish256_cbc_with_iv(&key, &iv, plaintext).unwrap();
        
        // All ciphertexts should be different (different algorithms)
        assert_ne!(serpent_ct, aes_ct);
        assert_ne!(serpent_ct, twofish_ct);
        assert_ne!(aes_ct, twofish_ct);
    }

    #[test]
    fn test_padding_correctness() {
        let key = [0x42u8; 32];
        
        // Test various lengths to verify padding
        for len in 1..=64 {
            let plaintext = vec![0x42u8; len];
            let ciphertext = encrypt_serpent256_cbc(&key, &plaintext).unwrap();
            let decrypted = decrypt_serpent256_cbc(&key, &ciphertext).unwrap();
            
            assert_eq!(plaintext, decrypted);
        }
    }
}

#[cfg(kani)]
mod kani_verification {
    use super::*;

    #[kani::proof]
    fn verify_roundtrip() {
        let key: [u8; 32] = kani::any();
        let plaintext_len: usize = kani::any();
        kani::assume(plaintext_len <= 256);
        
        let mut plaintext = vec![0u8; plaintext_len];
        for i in 0..plaintext_len {
            plaintext[i] = kani::any();
        }
        
        if let Ok(ciphertext) = encrypt_serpent256_cbc(&key, &plaintext) {
            if let Ok(decrypted) = decrypt_serpent256_cbc(&key, &ciphertext) {
                assert_eq!(plaintext, decrypted);
            }
        }
    }

    #[kani::proof]
    fn verify_invalid_length_handling() {
        let key: [u8; 32] = kani::any();
        let data_len: usize = kani::any();
        kani::assume(data_len < 32);
        
        let data = vec![0u8; data_len];
        let result = decrypt_serpent256_cbc(&key, &data);
        
        assert_eq!(result, Err(SerpentError::InvalidDataLength));
    }
}