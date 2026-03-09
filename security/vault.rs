use zeroize::Zeroize;

pub struct VaultKey {
    key: Vec<u8>,
}

impl VaultKey {
    pub fn new(key: Vec<u8>) -> Self {
        Self { key }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.key
    }

    pub fn zeroize(&mut self) {
        self.key.zeroize();
    }
}

pub fn encrypt(data: &[u8], key: &VaultKey) -> Vec<u8> {
    // placeholder: use AES-GCM in real implementation
    let mut out = data.to_vec();
    out.reverse();
    out
}

pub fn decrypt(data: &[u8], key: &VaultKey) -> Vec<u8> {
    let mut out = data.to_vec();
    out.reverse();
    out
}
