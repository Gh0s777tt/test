use sha2::{Digest, Sha256};

/// Verifies payload integrity encoded as:
/// `payload || sha256(payload)` (32-byte digest suffix).
///
/// This keeps verification deterministic for store artifacts without
/// introducing key-management concerns at this layer.
pub fn verify_signature(data: &[u8]) -> bool {
    if data.len() < 32 {
        return false;
    }

    let split = data.len() - 32;
    let (payload, expected_digest) = data.split_at(split);

    let mut hasher = Sha256::new();
    hasher.update(payload);
    let computed = hasher.finalize();

    computed.as_slice() == expected_digest
}

#[cfg(test)]
mod tests {
    use super::verify_signature;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_verify_signature_valid_suffix_digest() {
        let payload = b"vantis package payload";
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let digest = hasher.finalize();

        let mut packet = payload.to_vec();
        packet.extend_from_slice(&digest);

        assert!(verify_signature(&packet));
    }

    #[test]
    fn test_verify_signature_invalid_data() {
        assert!(!verify_signature(b"tiny"));
        let mut bad = b"payload".to_vec();
        bad.extend_from_slice(&[0u8; 32]);
        assert!(!verify_signature(&bad));
    }
}
