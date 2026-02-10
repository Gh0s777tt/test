use crate::vault::{VaultError, VaultKey};
use vantis_verified::vault::{CascadeKeys, VantisVault, KEY_SIZE, MAX_DATA_SIZE};

fn derive_layer_key(master: &[u8; KEY_SIZE], tweak: u8) -> [u8; KEY_SIZE] {
    let mut out = [0u8; KEY_SIZE];
    for (i, out_byte) in out.iter_mut().enumerate() {
        let rotated = master[i].rotate_left((i % 8) as u32);
        *out_byte = rotated ^ tweak ^ ((i as u8).wrapping_mul(17));
    }
    out
}

pub fn derive_cascade_keys(master_key: &VaultKey) -> CascadeKeys {
    let base = master_key.as_array();
    let aes_key = derive_layer_key(base, 0xA3);
    let twofish_key = derive_layer_key(base, 0x5C);
    let serpent_key = derive_layer_key(base, 0xE7);
    CascadeKeys::new(&aes_key, &twofish_key, &serpent_key)
}

pub fn cascade_encrypt(data: &[u8], key: &VaultKey) -> Result<Vec<u8>, VaultError> {
    if data.len() > MAX_DATA_SIZE {
        return Err(VaultError::DataTooLarge);
    }

    let keys = derive_cascade_keys(key);
    let mut vault = VantisVault::new();
    vault.initialize(keys);
    vault.encrypt(data).map_err(|_| VaultError::EncryptFailed)
}

pub fn cascade_decrypt(data: &[u8], key: &VaultKey) -> Result<Vec<u8>, VaultError> {
    let keys = derive_cascade_keys(key);
    let mut vault = VantisVault::new();
    vault.initialize(keys);
    vault.decrypt(data).map_err(|_| VaultError::DecryptFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cascade_roundtrip() {
        let key = VaultKey::from_array([0x42; 32]);
        let plaintext = b"Cascade security test payload";
        let ciphertext = cascade_encrypt(plaintext, &key).unwrap();
        assert_ne!(ciphertext, plaintext);
        let decrypted = cascade_decrypt(&ciphertext, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
