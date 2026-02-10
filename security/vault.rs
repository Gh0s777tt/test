use core::fmt;
use vantis_verified::vault_aes::{decrypt_aes256_cbc, encrypt_aes256_cbc};
use zeroize::Zeroize;

/// Maximum payload size accepted by the lightweight security crate.
pub const MAX_DATA_SIZE: usize = 1024 * 1024;

/// Fixed-size key wrapper with automatic zeroization.
pub struct VaultKey {
    key: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultError {
    InvalidKeyLength,
    DataTooLarge,
    EncryptFailed,
    DecryptFailed,
}

impl VaultKey {
    pub fn new(key_bytes: &[u8]) -> Result<Self, VaultError> {
        if key_bytes.len() != 32 {
            return Err(VaultError::InvalidKeyLength);
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(key_bytes);
        Ok(Self { key })
    }

    pub const fn from_array(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub const fn as_array(&self) -> &[u8; 32] {
        &self.key
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.key
    }

    pub fn zeroize(&mut self) {
        self.key.zeroize();
    }
}

impl Drop for VaultKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

impl fmt::Debug for VaultKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VaultKey([REDACTED])")
    }
}

/// Encrypts data with AES-256-CBC and random IV.
pub fn encrypt(data: &[u8], key: &VaultKey) -> Result<Vec<u8>, VaultError> {
    if data.len() > MAX_DATA_SIZE {
        return Err(VaultError::DataTooLarge);
    }
    encrypt_aes256_cbc(key.as_array(), data).map_err(|_| VaultError::EncryptFailed)
}

/// Decrypts data produced by [`encrypt`].
pub fn decrypt(data: &[u8], key: &VaultKey) -> Result<Vec<u8>, VaultError> {
    decrypt_aes256_cbc(key.as_array(), data).map_err(|_| VaultError::DecryptFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_validation() {
        assert!(VaultKey::new(&[1u8; 31]).is_err());
        assert!(VaultKey::new(&[1u8; 32]).is_ok());
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = VaultKey::from_array([0x42; 32]);
        let plaintext = b"Security crate vault test";

        let ciphertext = encrypt(plaintext, &key).unwrap();
        assert_ne!(ciphertext, plaintext);

        let decrypted = decrypt(&ciphertext, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
