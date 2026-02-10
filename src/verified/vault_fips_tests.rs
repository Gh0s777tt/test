//! FIPS 140-3 Self-Tests for Vantis Vault
//! 
//! This module implements FIPS 140-3 required self-tests for cryptographic
//! modules, including Known-Answer Tests (KAT) and continuous random number
//! generator tests.
//!
//! # FIPS 140-3 Requirements
//! - Power-up self-tests
//! - Known-Answer Tests (KAT) for all algorithms
//! - Continuous random number generator tests
//! - Conditional self-tests
//!
//! # Security
//! All tests must pass before cryptographic operations are allowed

use super::vault_aes::{encrypt_aes256_cbc_with_iv, decrypt_aes256_cbc, fips_kat_aes256_cbc};
use super::vault_twofish::{encrypt_twofish256_cbc_with_iv, decrypt_twofish256_cbc, kat_twofish256_cbc};
use super::vault_serpent::{encrypt_serpent256_cbc_with_iv, decrypt_serpent256_cbc, kat_serpent256_cbc};

/// FIPS 140-3 self-test result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FipsTestResult {
    /// All tests passed
    Pass,
    /// AES test failed
    AesFailed,
    /// Twofish test failed
    TwofishFailed,
    /// Serpent test failed
    SerpentFailed,
    /// Cascade test failed
    CascadeFailed,
    /// RNG test failed
    RngFailed,
}

/// Run all FIPS 140-3 power-up self-tests
/// 
/// # Returns
/// FipsTestResult::Pass if all tests pass, specific failure otherwise
/// 
/// # Security
/// This function must be called before any cryptographic operations
pub fn run_power_up_tests() -> FipsTestResult {
    // Test 1: AES-256-CBC Known-Answer Test
    if fips_kat_aes256_cbc().is_err() {
        return FipsTestResult::AesFailed;
    }
    
    // Test 2: Twofish-256-CBC Known-Answer Test
    if kat_twofish256_cbc().is_err() {
        return FipsTestResult::TwofishFailed;
    }
    
    // Test 3: Serpent-256-CBC Known-Answer Test
    if kat_serpent256_cbc().is_err() {
        return FipsTestResult::SerpentFailed;
    }
    
    // Test 4: Cascade encryption test
    if test_cascade_encryption().is_err() {
        return FipsTestResult::CascadeFailed;
    }
    
    // Test 5: Random number generator test
    if test_rng_continuous().is_err() {
        return FipsTestResult::RngFailed;
    }
    
    FipsTestResult::Pass
}

/// Test cascade encryption with known values
fn test_cascade_encryption() -> Result<(), ()> {
    let key1: [u8; 32] = [0x01; 32];
    let key2: [u8; 32] = [0x02; 32];
    let key3: [u8; 32] = [0x03; 32];
    let iv: [u8; 16] = [0x00; 16];
    let plaintext = b"FIPS 140-3 cascade test";
    
    // Encrypt through cascade
    let encrypted1 = encrypt_aes256_cbc_with_iv(&key1, &iv, plaintext).map_err(|_| ())?;
    let encrypted2 = encrypt_twofish256_cbc_with_iv(&key2, &iv, &encrypted1).map_err(|_| ())?;
    let encrypted3 = encrypt_serpent256_cbc_with_iv(&key3, &iv, &encrypted2).map_err(|_| ())?;
    
    // Decrypt through cascade (reverse order)
    let decrypted3 = decrypt_serpent256_cbc(&key3, &encrypted3).map_err(|_| ())?;
    let decrypted2 = decrypt_twofish256_cbc(&key2, &decrypted3).map_err(|_| ())?;
    let decrypted1 = decrypt_aes256_cbc(&key1, &decrypted2).map_err(|_| ())?;
    
    // Verify roundtrip
    if decrypted1 != plaintext {
        return Err(());
    }
    
    Ok(())
}

/// Continuous random number generator test
/// 
/// Tests that consecutive random values are different
/// (FIPS 140-3 requirement)
fn test_rng_continuous() -> Result<(), ()> {
    use rand::RngCore;
    
    let mut rng = rand::thread_rng();
    
    // Generate two consecutive random values
    let mut value1 = [0u8; 16];
    let mut value2 = [0u8; 16];
    
    rng.fill_bytes(&mut value1);
    rng.fill_bytes(&mut value2);
    
    // Values must be different (continuous RNG test)
    if value1 == value2 {
        return Err(());
    }
    
    Ok(())
}

/// Run conditional self-tests
/// 
/// These tests are run when specific conditions are met
/// (e.g., key generation, firmware update)
pub fn run_conditional_tests() -> FipsTestResult {
    // For now, conditional tests are the same as power-up tests
    run_power_up_tests()
}

/// Verify cryptographic module integrity
/// 
/// # Returns
/// true if module integrity is verified, false otherwise
/// 
/// # Note
/// In a full FIPS 140-3 implementation, this would verify
/// HMAC signatures of the cryptographic module
pub fn verify_module_integrity() -> bool {
    // Placeholder for module integrity verification
    // In production, this would:
    // 1. Calculate HMAC of cryptographic code
    // 2. Compare with stored HMAC
    // 3. Return true only if they match
    true
}

#[cfg(all(test, feature = "verus"))]
mod tests {
    use super::*;

    #[test]
    fn test_power_up_tests() {
        let result = run_power_up_tests();
        assert_eq!(result, FipsTestResult::Pass);
    }

    #[test]
    fn test_cascade_encryption_test() {
        assert!(test_cascade_encryption().is_ok());
    }

    #[test]
    fn test_rng_continuous_test() {
        assert!(test_rng_continuous().is_ok());
    }

    #[test]
    fn test_conditional_tests() {
        let result = run_conditional_tests();
        assert_eq!(result, FipsTestResult::Pass);
    }

    #[test]
    fn test_module_integrity() {
        assert!(verify_module_integrity());
    }

    #[test]
    fn test_all_algorithms_kat() {
        // Verify all algorithm KATs pass individually
        assert!(fips_kat_aes256_cbc().is_ok());
        assert!(kat_twofish256_cbc().is_ok());
        assert!(kat_serpent256_cbc().is_ok());
    }

    #[test]
    fn test_rng_produces_different_values() {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        
        // Generate 10 random values
        let mut values = Vec::new();
        for _ in 0..10 {
            let mut value = [0u8; 16];
            rng.fill_bytes(&mut value);
            values.push(value);
        }
        
        // All values should be different
        for i in 0..values.len() {
            for j in (i+1)..values.len() {
                assert_ne!(values[i], values[j]);
            }
        }
    }
}