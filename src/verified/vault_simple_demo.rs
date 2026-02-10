//! Vantis Vault - Simple Demo Implementation
//! 
//! This is a simplified demonstration implementation showing the cascade
//! encryption concept. For production use, this should be replaced with
//! actual cryptographic implementations using RustCrypto libraries.
//!
//! # Security Warning
//! 
//! This implementation uses XOR-based "encryption" for demonstration purposes.
//! It is NOT cryptographically secure and should NOT be used in production.
//! 
//! For production, use:
//! - AES-256 from `aes` crate
//! - Twofish-256 from `twofish` crate
//! - Serpent-256 from `serpent` crate

#[cfg(feature = "verus")]
use verus::prelude::*;

use super::vault::{CascadeKeys, KEY_SIZE, MAX_DATA_SIZE};

/// Simple XOR-based encryption (DEMO ONLY - NOT SECURE)
/// 
/// This demonstrates the cascade concept without requiring crypto libraries.
/// In production, replace with actual AES-256-CBC implementation.
fn xor_encrypt_demo(data: &[u8], key: &[u8; KEY_SIZE]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % KEY_SIZE])
        .collect()
}

/// Simple XOR-based decryption (DEMO ONLY - NOT SECURE)
/// 
/// XOR is its own inverse, so encryption and decryption are the same.
fn xor_decrypt_demo(data: &[u8], key: &[u8; KEY_SIZE]) -> Vec<u8> {
    xor_encrypt_demo(data, key)
}

/// Demo implementation of Vantis Vault
pub struct VantisVaultDemo {
    keys: Option<CascadeKeys>,
    panic_mode: bool,
}

impl VantisVaultDemo {
    /// Create new vault
    pub fn new() -> Self {
        VantisVaultDemo {
            keys: None,
            panic_mode: false,
        }
    }
    
    /// Initialize vault with keys
    pub fn initialize(&mut self, keys: CascadeKeys) {
        self.keys = Some(keys);
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.keys.is_some()
    }
    
    /// Encrypt data using cascade encryption (DEMO)
    /// 
    /// # Security Warning
    /// This is a demonstration using XOR. NOT cryptographically secure!
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, &'static str> {
        if !self.is_initialized() {
            return Err("Vault not initialized");
        }
        
        if data.len() > MAX_DATA_SIZE {
            return Err("Data too large");
        }
        
        let keys = self.keys.as_ref().unwrap();
        
        // Layer 1: "AES" (XOR demo)
        let mut encrypted = xor_encrypt_demo(data, keys.aes_key().as_bytes());
        
        // Layer 2: "Twofish" (XOR demo)
        encrypted = xor_encrypt_demo(&encrypted, keys.twofish_key().as_bytes());
        
        // Layer 3: "Serpent" (XOR demo)
        encrypted = xor_encrypt_demo(&encrypted, keys.serpent_key().as_bytes());
        
        Ok(encrypted)
    }
    
    /// Decrypt data using cascade decryption (DEMO)
    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, &'static str> {
        if !self.is_initialized() {
            return Err("Vault not initialized");
        }
        
        let keys = self.keys.as_ref().unwrap();
        
        // Layer 3: "Serpent" (XOR demo - reverse order)
        let mut decrypted = xor_decrypt_demo(data, keys.serpent_key().as_bytes());
        
        // Layer 2: "Twofish" (XOR demo)
        decrypted = xor_decrypt_demo(&decrypted, keys.twofish_key().as_bytes());
        
        // Layer 1: "AES" (XOR demo)
        decrypted = xor_decrypt_demo(&decrypted, keys.aes_key().as_bytes());
        
        Ok(decrypted)
    }
    
    /// Activate panic mode
    pub fn panic(&mut self) {
        if let Some(mut keys) = self.keys.take() {
            keys.zeroize();
        }
        self.panic_mode = true;
    }
    
    /// Check if panic mode is active
    pub fn is_panic_mode(&self) -> bool {
        self.panic_mode
    }
}

impl Default for VantisVaultDemo {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::vault::CascadeKeys;
    
    #[test]
    fn test_demo_vault_encrypt_decrypt() {
        let mut vault = VantisVaultDemo::new();
        
        let keys = CascadeKeys::new(
            &[1u8; KEY_SIZE],
            &[2u8; KEY_SIZE],
            &[3u8; KEY_SIZE]
        );
        
        vault.initialize(keys);
        
        let plaintext = b"Hello, VANTIS OS! This is a demo.";
        let ciphertext = vault.encrypt(plaintext).unwrap();
        let decrypted = vault.decrypt(&ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_demo_vault_different_keys_different_output() {
        let mut vault1 = VantisVaultDemo::new();
        let mut vault2 = VantisVaultDemo::new();
        
        let keys1 = CascadeKeys::new(
            &[1u8; KEY_SIZE],
            &[2u8; KEY_SIZE],
            &[3u8; KEY_SIZE]
        );
        
        let keys2 = CascadeKeys::new(
            &[4u8; KEY_SIZE],
            &[5u8; KEY_SIZE],
            &[6u8; KEY_SIZE]
        );
        
        vault1.initialize(keys1);
        vault2.initialize(keys2);
        
        let plaintext = b"Same plaintext";
        let ciphertext1 = vault1.encrypt(plaintext).unwrap();
        let ciphertext2 = vault2.encrypt(plaintext).unwrap();
        
        // Different keys should produce different ciphertexts
        assert_ne!(ciphertext1, ciphertext2);
    }
    
    #[test]
    fn test_demo_vault_cascade_layers() {
        let mut vault = VantisVaultDemo::new();
        
        let keys = CascadeKeys::new(
            &[1u8; KEY_SIZE],
            &[2u8; KEY_SIZE],
            &[4u8; KEY_SIZE]
        );
        
        vault.initialize(keys);
        
        let plaintext = b"Test cascade";
        
        // Full cascade encryption
        let ciphertext = vault.encrypt(plaintext).unwrap();
        
        // Ciphertext should be different from plaintext
        assert_ne!(ciphertext.as_slice(), plaintext);
        
        // Decryption should recover plaintext
        let decrypted = vault.decrypt(&ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_demo_vault_panic_mode() {
        let mut vault = VantisVaultDemo::new();
        
        let keys = CascadeKeys::new(
            &[1u8; KEY_SIZE],
            &[2u8; KEY_SIZE],
            &[3u8; KEY_SIZE]
        );
        
        vault.initialize(keys);
        assert!(vault.is_initialized());
        assert!(!vault.is_panic_mode());
        
        vault.panic();
        assert!(!vault.is_initialized());
        assert!(vault.is_panic_mode());
    }
    
    #[test]
    fn test_demo_vault_large_data() {
        let mut vault = VantisVaultDemo::new();
        
        let keys = CascadeKeys::new(
            &[1u8; KEY_SIZE],
            &[2u8; KEY_SIZE],
            &[3u8; KEY_SIZE]
        );
        
        vault.initialize(keys);
        
        // Test with 1 KB of data
        let plaintext = vec![0x42u8; 1024];
        let ciphertext = vault.encrypt(&plaintext).unwrap();
        let decrypted = vault.decrypt(&ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_demo_vault_empty_data() {
        let mut vault = VantisVaultDemo::new();
        
        let keys = CascadeKeys::new(
            &[1u8; KEY_SIZE],
            &[2u8; KEY_SIZE],
            &[3u8; KEY_SIZE]
        );
        
        vault.initialize(keys);
        
        let plaintext = b"";
        let ciphertext = vault.encrypt(plaintext).unwrap();
        let decrypted = vault.decrypt(&ciphertext).unwrap();
        
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_demo_vault_uninitialized() {
        let vault = VantisVaultDemo::new();
        
        let plaintext = b"test";
        let result = vault.encrypt(plaintext);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Vault not initialized");
    }
    
    #[test]
    fn test_demo_vault_performance() {
        let mut vault = VantisVaultDemo::new();
        
        let keys = CascadeKeys::new(
            &[1u8; KEY_SIZE],
            &[2u8; KEY_SIZE],
            &[3u8; KEY_SIZE]
        );
        
        vault.initialize(keys);
        
        // Test with 100 KB
        let plaintext = vec![0x42u8; 100 * 1024];
        
        let start = std::time::Instant::now();
        let ciphertext = vault.encrypt(&plaintext).unwrap();
        let encrypt_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let decrypted = vault.decrypt(&ciphertext).unwrap();
        let decrypt_time = start.elapsed();
        
        println!("Demo encryption: {:?} for 100 KB", encrypt_time);
        println!("Demo decryption: {:?} for 100 KB", decrypt_time);
        
        assert_eq!(decrypted, plaintext);
    }
}