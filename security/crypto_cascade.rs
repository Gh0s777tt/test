use crate::security::vault::VaultKey;

pub fn cascade_encrypt(data: &[u8], key: &VaultKey) -> Vec<u8> {
    let a = crate::security::vault::encrypt(data, key);
    let b = crate::security::vault::encrypt(&a, key);
    crate::security::vault::encrypt(&b, key)
}

pub fn cascade_decrypt(data: &[u8], key: &VaultKey) -> Vec<u8> {
    let a = crate::security::vault::decrypt(data, key);
    let b = crate::security::vault::decrypt(&a, key);
    crate::security::vault::decrypt(&b, key)
}
