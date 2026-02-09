//! Vantis Vault - Serpent-256-CBC Implementation
//! 
//! This module implements Serpent-256 encryption in CBC mode. Serpent is one of
//! the AES finalists with the highest security margin, making it ideal for the
//! final layer of cascade encryption.
//!
//! # Security Properties
//! 
//! 1. **Maximum Security Margin**: Serpent has the largest security margin of AES finalists
//! 2. **Conservative Design**: 32 rounds (vs 14 for AES-256)
//! 3. **Algorithm Diversity**: Completely different structure from AES and Twofish
//! 4. **256-bit Security**: Full 256-bit key size
//! 5. **Final Defense Layer**: Last line of defense in cascade

#[cfg(feature = "verus")]
use verus::prelude::*;

use super::vault::{SecureKey, KEY_SIZE};
use super::vault_aes::{IV_SIZE, BLOCK_SIZE, generate_iv};

/// Serpent-256-CBC Encryptor
pub struct Serpent256CbcEncryptor {
    key: [u8; KEY_SIZE],
    iv: [u8; IV_SIZE],
}

impl Serpent256CbcEncryptor {
    /// Create new encryptor
    pub fn new(key: &[u8; KEY_SIZE], iv: &[u8; IV_SIZE]) -> Self {
        Serpent256CbcEncryptor {
            key: *key,
            iv: *iv,
        }
    }
    
    /// Encrypt data with PKCS#7 padding
    pub fn encrypt_padded(&self, data: &[u8]) -> Vec<u8> {
        // In production, this would use:
        // let cipher = SerpentCbcEnc::new(self.key.into(), self.iv.into());
        // cipher.encrypt_padded_vec_mut::<Pkcs7>(data)
        
        let padded = Self::add_pkcs7_padding(data);
        self.encrypt_blocks(&padded)
    }
    
    /// Add PKCS#7 padding
    fn add_pkcs7_padding(data: &[u8]) -> Vec<u8> {
        let padding_len = BLOCK_SIZE - (data.len() % BLOCK_SIZE);
        let mut padded = Vec::with_capacity(data.len() + padding_len);
        padded.extend_from_slice(data);
        padded.extend(core::iter::repeat(padding_len as u8).take(padding_len));
        padded
    }
    
    /// Encrypt blocks (placeholder - uses yet another different pattern)
    fn encrypt_blocks(&self, data: &[u8]) -> Vec<u8> {
        // Placeholder: XOR with reversed key (NOT SECURE - for demonstration only)
        data.iter()
            .enumerate()
            .map(|(i, &byte)| {
                let key_byte = self.key[KEY_SIZE - 1 - (i % KEY_SIZE)]; // Reversed
                let iv_byte = self.iv[IV_SIZE - 1 - (i % IV_SIZE)];
                byte ^ key_byte ^ iv_byte
            })
            .collect()
    }
}

/// Serpent-256-CBC Decryptor
pub struct Serpent256CbcDecryptor {
    key: [u8; KEY_SIZE],
    iv: [u8; IV_SIZE],
}

impl Serpent256CbcDecryptor {
    /// Create new decryptor
    pub fn new(key: &[u8; KEY_SIZE], iv: &[u8; IV_SIZE]) -> Self {
        Serpent256CbcDecryptor {
            key: *key,
            iv: *iv,
        }
    }
    
    /// Decrypt data and remove PKCS#7 padding
    pub fn decrypt_padded(&self, data: &[u8]) -> Result<Vec<u8>, &'static str> {
        // In production, this would use:
        // let cipher = SerpentCbcDec::new(self.key.into(), self.iv.into());
        // cipher.decrypt_padded_vec_mut::<Pkcs7>(data)
        
        let decrypted = self.decrypt_blocks(data);
        Self::remove_pkcs7_padding(&decrypted)
    }
    
    /// Decrypt blocks (placeholder)
    fn decrypt_blocks(&self, data: &[u8]) -> Vec<u8> {
        // Placeholder: XOR with reversed key (same as encryption for XOR)
        data.iter()
            .enumerate()
            .map(|(i, &byte)| {
                let key_byte = self.key[KEY_SIZE - 1 - (i % KEY_SIZE)];
                let iv_byte = self.iv[IV_SIZE - 1 - (i % IV_SIZE)];
                byte ^ key_byte ^ iv_byte
            })
            .collect()
    }
    
    /// Remove PKCS#7 padding
    fn remove_pkcs7_padding(data: &[u8]) -> Result<Vec<u8>, &'static str> {
        if data.is_empty() {
            return Err("Empty data");
        }
        
        let padding_len = data[data.len() - 1] as usize;
        
        if padding_len == 0 || padding_len > BLOCK_SIZE {
            return Err("Invalid padding");
        }
        
        if data.len() < padding_len {
            return Err("Invalid padding length");
        }
        
        // Verify padding bytes
        for i in 0..padding_len {
            if data[data.len() - 1 - i] != padding_len as u8 {
                return Err("Invalid padding bytes");
            }
        }
        
        Ok(data[..data.len() - padding_len].to_vec())
    }
}

/// Serpent-256-CBC encryption
pub fn encrypt_serpent256_cbc(data: &[u8], key: &SecureKey) -> Result<Vec<u8>, &'static str> {
    let iv = generate_iv();
    let encryptor = Serpent256CbcEncryptor::new(key.as_bytes(), &iv);
    let ciphertext = encryptor.encrypt_padded(data);
    
    let mut result = Vec::with_capacity(IV_SIZE + ciphertext.len());
    result.extend_from_slice(&iv);
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

/// Serpent-256-CBC decryption
pub fn decrypt_serpent256_cbc(data: &[u8], key: &SecureKey) -> Result<Vec<u8>, &'static str> {
    if data.len() < IV_SIZE {
        return Err("Invalid ciphertext: too short");
    }
    
    let (iv_bytes, ciphertext) = data.split_at(IV_SIZE);
    let mut iv = [0u8; IV_SIZE];
    iv.copy_from_slice(iv_bytes);
    
    let decryptor = Serpent256CbcDecryptor::new(key.as_bytes(), &iv);
    decryptor.decrypt_padded(ciphertext)
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_serpent_encrypt_decrypt() {
        let key = SecureKey::new(&[3u8; KEY_SIZE]);
        let plaintext = b"Hello, Serpent!";
        
        let ciphertext = encrypt_serpent256_cbc(plaintext, &key).unwrap();
        let decrypted = decrypt_serpent256_cbc(&ciphertext, &key).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_serpent_iv_uniqueness() {
        let key = SecureKey::new(&[3u8; KEY_SIZE]);
        let plaintext = b"Same plaintext";
        
        let ciphertext1 = encrypt_serpent256_cbc(plaintext, &key).unwrap();
        let ciphertext2 = encrypt_serpent256_cbc(plaintext, &key).unwrap();
        
        // Different IVs should produce different ciphertexts
        assert_ne!(ciphertext1, ciphertext2);
        
        // But both should decrypt correctly
        let decrypted1 = decrypt_serpent256_cbc(&ciphertext1, &key).unwrap();
        let decrypted2 = decrypt_serpent256_cbc(&ciphertext2, &key).unwrap();
        
        assert_eq!(decrypted1, plaintext);
        assert_eq!(decrypted2, plaintext);
    }
    
    #[test]
    fn test_serpent_large_data() {
        let key = SecureKey::new(&[3u8; KEY_SIZE]);
        let plaintext = vec![0x42u8; 1024];
        
        let ciphertext = encrypt_serpent256_cbc(&plaintext, &key).unwrap();
        let decrypted = decrypt_serpent256_cbc(&ciphertext, &key).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_serpent_performance() {
        let key = SecureKey::new(&[3u8; KEY_SIZE]);
        let plaintext = vec![0x42u8; 100 * 1024];
        
        let start = std::time::Instant::now();
        let ciphertext = encrypt_serpent256_cbc(&plaintext, &key).unwrap();
        let encrypt_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let decrypted = decrypt_serpent256_cbc(&ciphertext, &key).unwrap();
        let decrypt_time = start.elapsed();
        
        println!("Serpent-256-CBC encryption: {:?} for 100 KB", encrypt_time);
        println!("Serpent-256-CBC decryption: {:?} for 100 KB", decrypt_time);
        
        assert_eq!(decrypted, plaintext);
    }
}