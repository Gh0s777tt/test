use vantis_verified::vault::{CascadeKeys, VantisVault, KEY_SIZE, MAX_DATA_SIZE};

fn sample_keys(seed: u8) -> CascadeKeys {
    let aes = [seed; KEY_SIZE];
    let twofish = [seed.wrapping_add(1); KEY_SIZE];
    let serpent = [seed.wrapping_add(2); KEY_SIZE];
    CascadeKeys::new(&aes, &twofish, &serpent)
}

#[test]
fn test_vault_roundtrip_encrypt_decrypt() {
    let mut vault = VantisVault::new();
    vault.initialize(sample_keys(0x11));

    let plaintext = b"VANTIS vault integration roundtrip";
    let ciphertext = vault.encrypt(plaintext).expect("encryption should succeed");
    let decrypted = vault
        .decrypt(&ciphertext)
        .expect("decryption should succeed");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_vault_ciphertext_differs_and_has_overhead() {
    let mut vault = VantisVault::new();
    vault.initialize(sample_keys(0x22));

    let plaintext = b"short-message";
    let ciphertext = vault.encrypt(plaintext).expect("encryption should succeed");

    assert_ne!(ciphertext, plaintext);
    assert!(ciphertext.len() > plaintext.len());
}

#[test]
fn test_vault_wrong_key_does_not_recover_plaintext() {
    let mut vault_a = VantisVault::new();
    let mut vault_b = VantisVault::new();
    vault_a.initialize(sample_keys(0x33));
    vault_b.initialize(sample_keys(0x44));

    let plaintext = b"wrong-key-check";
    let ciphertext = vault_a.encrypt(plaintext).expect("encryption should succeed");

    match vault_b.decrypt(&ciphertext) {
        Ok(decrypted) => assert_ne!(decrypted, plaintext),
        Err(_) => {}
    }
}

#[test]
fn test_vault_size_limits_are_enforced() {
    let mut vault = VantisVault::new();
    vault.initialize(sample_keys(0x55));

    let within_limit = vec![0xAB; MAX_DATA_SIZE];
    assert!(vault.encrypt(&within_limit).is_ok());

    let too_large = vec![0xCD; MAX_DATA_SIZE + 1];
    assert!(vault.encrypt(&too_large).is_err());
}

#[test]
fn test_vault_panic_mode_blocks_further_use() {
    let mut vault = VantisVault::new();
    vault.initialize(sample_keys(0x66));
    assert!(vault.encrypt(b"before-panic").is_ok());

    vault.panic();
    assert!(vault.is_panic_mode());
    assert!(!vault.is_initialized());
    assert!(vault.encrypt(b"after-panic").is_err());
}
