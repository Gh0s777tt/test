use semver::Version;
use serde::Deserialize;
use sha2::{Digest, Sha256};

const DIGEST_SIZE: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PackagePermissions {
    pub filesystem: bool,
    pub network: bool,
    pub devices: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub hash: String,
    pub permissions: PackagePermissions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreVerifyError {
    InvalidPacket,
    ManifestParse(String),
    InvalidManifest(String),
    HashMismatch,
}

impl std::fmt::Display for StoreVerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreVerifyError::InvalidPacket => write!(f, "invalid package packet format"),
            StoreVerifyError::ManifestParse(msg) => write!(f, "manifest parse error: {msg}"),
            StoreVerifyError::InvalidManifest(msg) => write!(f, "invalid manifest: {msg}"),
            StoreVerifyError::HashMismatch => write!(f, "manifest hash does not match payload"),
        }
    }
}

impl std::error::Error for StoreVerifyError {}

fn split_packet(data: &[u8]) -> Result<(&[u8], &[u8]), StoreVerifyError> {
    if data.len() < DIGEST_SIZE {
        return Err(StoreVerifyError::InvalidPacket);
    }
    let split = data.len() - DIGEST_SIZE;
    Ok(data.split_at(split))
}

fn compute_payload_digest(payload: &[u8]) -> [u8; DIGEST_SIZE] {
    let digest = Sha256::digest(payload);
    let mut out = [0u8; DIGEST_SIZE];
    out.copy_from_slice(&digest);
    out
}

fn digest_to_lower_hex(digest: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(digest.len() * 2);
    for &byte in digest {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn is_hex_digest(hash: &str) -> bool {
    hash.len() == DIGEST_SIZE * 2 && hash.bytes().all(|b| b.is_ascii_hexdigit())
}

pub fn validate_manifest(manifest: &PackageManifest) -> Result<(), StoreVerifyError> {
    if manifest.name.trim().is_empty() {
        return Err(StoreVerifyError::InvalidManifest(
            "name must not be empty".to_string(),
        ));
    }
    if manifest.name.len() > 128 {
        return Err(StoreVerifyError::InvalidManifest(
            "name exceeds 128 characters".to_string(),
        ));
    }

    Version::parse(&manifest.version).map_err(|err| {
        StoreVerifyError::InvalidManifest(format!("version is not valid semver: {err}"))
    })?;

    if !is_hex_digest(&manifest.hash) {
        return Err(StoreVerifyError::InvalidManifest(
            "hash must be 64 hexadecimal characters".to_string(),
        ));
    }

    Ok(())
}

pub fn parse_manifest(manifest_json: &str) -> Result<PackageManifest, StoreVerifyError> {
    let manifest: PackageManifest = serde_json::from_str(manifest_json)
        .map_err(|err| StoreVerifyError::ManifestParse(err.to_string()))?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

/// Verifies payload integrity encoded as:
/// `payload || sha256(payload)` (32-byte digest suffix).
pub fn verify_signature(data: &[u8]) -> bool {
    let Ok((payload, expected_digest)) = split_packet(data) else {
        return false;
    };
    let computed = compute_payload_digest(payload);
    computed.as_ref() == expected_digest
}

/// Validates package payload integrity and manifest consistency.
///
/// Packet format:
/// - bytes `[0..n-32]`: package payload
/// - bytes `[n-32..n]`: raw SHA-256 digest of payload
pub fn verify_package(
    packet: &[u8],
    manifest_json: &str,
) -> Result<PackageManifest, StoreVerifyError> {
    let manifest = parse_manifest(manifest_json)?;
    let (payload, expected_digest) = split_packet(packet)?;
    let computed = compute_payload_digest(payload);

    if computed.as_ref() != expected_digest {
        return Err(StoreVerifyError::InvalidPacket);
    }

    let computed_hex = digest_to_lower_hex(&computed);
    if !manifest.hash.eq_ignore_ascii_case(&computed_hex) {
        return Err(StoreVerifyError::HashMismatch);
    }

    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::{
        digest_to_lower_hex, verify_package, verify_signature, StoreVerifyError, DIGEST_SIZE,
    };
    use sha2::{Digest, Sha256};

    fn build_packet(payload: &[u8]) -> Vec<u8> {
        let mut packet = payload.to_vec();
        let digest = Sha256::digest(payload);
        packet.extend_from_slice(&digest);
        packet
    }

    fn manifest_json(hash: &str) -> String {
        format!(
            r#"{{
  "name": "vantis.demo",
  "version": "1.2.3",
  "hash": "{hash}",
  "permissions": {{
    "filesystem": false,
    "network": true,
    "devices": false
  }}
}}"#
        )
    }

    #[test]
    fn test_verify_signature_valid_suffix_digest() {
        let packet = build_packet(b"vantis package payload");
        assert!(verify_signature(&packet));
    }

    #[test]
    fn test_verify_signature_invalid_data() {
        assert!(!verify_signature(b"tiny"));
        let mut bad = b"payload".to_vec();
        bad.extend_from_slice(&[0u8; DIGEST_SIZE]);
        assert!(!verify_signature(&bad));
    }

    #[test]
    fn test_verify_package_success() {
        let payload = b"vantis package payload";
        let digest = Sha256::digest(payload);
        let hash_hex = digest_to_lower_hex(&digest);
        let manifest = manifest_json(&hash_hex);
        let packet = build_packet(payload);

        let parsed = verify_package(&packet, &manifest).expect("package should verify");
        assert_eq!(parsed.name, "vantis.demo");
        assert_eq!(parsed.version, "1.2.3");
    }

    #[test]
    fn test_verify_package_rejects_hash_mismatch() {
        let payload = b"vantis package payload";
        let packet = build_packet(payload);
        let manifest = manifest_json("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

        let err = verify_package(&packet, &manifest).expect_err("manifest hash should mismatch");
        assert_eq!(err, StoreVerifyError::HashMismatch);
    }

    #[test]
    fn test_verify_package_rejects_invalid_manifest_version() {
        let payload = b"vantis package payload";
        let digest = Sha256::digest(payload);
        let hash_hex = digest_to_lower_hex(&digest);
        let manifest = format!(
            r#"{{
  "name": "vantis.demo",
  "version": "invalid-version",
  "hash": "{hash_hex}",
  "permissions": {{
    "filesystem": false,
    "network": true,
    "devices": false
  }}
}}"#
        );
        let packet = build_packet(payload);

        let err = verify_package(&packet, &manifest).expect_err("invalid version must fail");
        match err {
            StoreVerifyError::InvalidManifest(msg) => {
                assert!(msg.contains("semver"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn test_verify_package_rejects_tampered_packet() {
        let payload = b"vantis package payload";
        let digest = Sha256::digest(payload);
        let hash_hex = digest_to_lower_hex(&digest);
        let manifest = manifest_json(&hash_hex);

        let mut packet = build_packet(payload);
        packet[0] ^= 0xFF;

        let err = verify_package(&packet, &manifest).expect_err("tampered packet must fail");
        assert_eq!(err, StoreVerifyError::InvalidPacket);
    }
}
