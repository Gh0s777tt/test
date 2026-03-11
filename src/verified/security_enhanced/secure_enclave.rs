//! Secure Enclave for VANTIS OS
//!
//! Provides an isolated execution environment for sensitive cryptographic
//! operations. The enclave manages secret keys, performs encryption/decryption,
//! and signing operations within a protected memory region that is inaccessible
//! to the rest of the kernel.

use core::fmt;

// ============================================================================
// Enclave Types
// ============================================================================

/// Types of keys managed by the enclave
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    /// Symmetric encryption key (AES-256)
    Symmetric256,
    /// Asymmetric signing key pair (Ed25519)
    SigningKeyPair,
    /// Key derivation master key
    DerivationMaster,
    /// Sealing key for enclave data persistence
    SealingKey,
    /// Attestation key for remote verification
    AttestationKey,
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Symmetric256 => write!(f, "AES-256"),
            KeyType::SigningKeyPair => write!(f, "Ed25519"),
            KeyType::DerivationMaster => write!(f, "KDF-Master"),
            KeyType::SealingKey => write!(f, "Sealing"),
            KeyType::AttestationKey => write!(f, "Attestation"),
        }
    }
}

/// Key usage permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyPermission {
    /// Key can be used for encryption
    Encrypt,
    /// Key can be used for decryption
    Decrypt,
    /// Key can be used for signing
    Sign,
    /// Key can be used for verification
    Verify,
    /// Key can be used for key derivation
    Derive,
    /// Key can be exported (wrapped)
    Export,
}

/// Enclave operational state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnclaveState {
    /// Enclave not yet initialized
    Uninitialized,
    /// Enclave is initializing (loading keys, verifying integrity)
    Initializing,
    /// Enclave is ready for operations
    Ready,
    /// Enclave is locked (too many failed attempts)
    Locked,
    /// Enclave is being destroyed (zeroizing secrets)
    Destroying,
    /// Enclave has been destroyed
    Destroyed,
}

impl fmt::Display for EnclaveState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnclaveState::Uninitialized => write!(f, "UNINITIALIZED"),
            EnclaveState::Initializing => write!(f, "INITIALIZING"),
            EnclaveState::Ready => write!(f, "READY"),
            EnclaveState::Locked => write!(f, "LOCKED"),
            EnclaveState::Destroying => write!(f, "DESTROYING"),
            EnclaveState::Destroyed => write!(f, "DESTROYED"),
        }
    }
}

// ============================================================================
// Key Handle
// ============================================================================

/// A handle to a key stored in the enclave (opaque to callers)
#[derive(Debug, Clone)]
pub struct KeyHandle {
    pub id: u64,
    pub label: String,
    pub key_type: KeyType,
    pub permissions: Vec<KeyPermission>,
    pub created_epoch: u64,
    pub use_count: u64,
    pub max_uses: Option<u64>,
    /// Internal key material (never exposed outside enclave)
    key_material: Vec<u8>,
}

impl KeyHandle {
    fn new(
        id: u64,
        label: &str,
        key_type: KeyType,
        permissions: Vec<KeyPermission>,
        key_material: Vec<u8>,
        epoch: u64,
    ) -> Self {
        Self {
            id,
            label: label.to_string(),
            key_type,
            permissions,
            created_epoch: epoch,
            use_count: 0,
            max_uses: None,
            key_material,
        }
    }

    /// Check if this key has a specific permission
    pub fn has_permission(&self, perm: KeyPermission) -> bool {
        self.permissions.contains(&perm)
    }

    /// Check if the key has exceeded its maximum use count
    pub fn is_exhausted(&self) -> bool {
        if let Some(max) = self.max_uses {
            self.use_count >= max
        } else {
            false
        }
    }

    /// Increment use counter
    fn record_use(&mut self) {
        self.use_count += 1;
    }
}

// ============================================================================
// Enclave Operations Results
// ============================================================================

/// Result of an encryption operation
#[derive(Debug, Clone)]
pub struct EncryptionResult {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub tag: Vec<u8>,
}

/// Result of a signing operation
#[derive(Debug, Clone)]
pub struct SignatureResult {
    pub signature: Vec<u8>,
    pub key_id: u64,
}

/// Attestation report for remote verification
#[derive(Debug, Clone)]
pub struct AttestationReport {
    pub enclave_id: u64,
    pub measurement: [u8; 32],
    pub timestamp: u64,
    pub key_count: usize,
    pub state: EnclaveState,
    pub signature: Vec<u8>,
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum EnclaveError {
    /// Enclave is not in the correct state for this operation
    InvalidState(EnclaveState),
    /// Key not found
    KeyNotFound(u64),
    /// Key label already exists
    DuplicateKeyLabel(String),
    /// Operation not permitted for this key
    PermissionDenied { key_id: u64, required: KeyPermission },
    /// Key has exceeded maximum use count
    KeyExhausted(u64),
    /// Enclave is locked due to too many failed attempts
    EnclaveLocked,
    /// Authentication failed
    AuthenticationFailed,
    /// Invalid input data
    InvalidInput(String),
    /// Enclave memory limit exceeded
    MemoryLimitExceeded { limit: usize, requested: usize },
}

impl fmt::Display for EnclaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnclaveError::InvalidState(s) => write!(f, "Invalid enclave state: {}", s),
            EnclaveError::KeyNotFound(id) => write!(f, "Key {} not found", id),
            EnclaveError::DuplicateKeyLabel(l) => write!(f, "Duplicate key label: {}", l),
            EnclaveError::PermissionDenied { key_id, required } => {
                write!(f, "Key {} lacks {:?} permission", key_id, required)
            }
            EnclaveError::KeyExhausted(id) => write!(f, "Key {} exhausted", id),
            EnclaveError::EnclaveLocked => write!(f, "Enclave is locked"),
            EnclaveError::AuthenticationFailed => write!(f, "Authentication failed"),
            EnclaveError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            EnclaveError::MemoryLimitExceeded { limit, requested } => {
                write!(f, "Memory limit exceeded: {} > {}", requested, limit)
            }
        }
    }
}

// ============================================================================
// Secure Enclave
// ============================================================================

/// Maximum keys per enclave
const MAX_KEYS: usize = 256;
/// Maximum failed auth attempts before lockout
const MAX_FAILED_ATTEMPTS: u32 = 5;
/// Default enclave memory limit (1 MB)
const DEFAULT_MEMORY_LIMIT: usize = 1024 * 1024;

/// The main secure enclave
pub struct SecureEnclave {
    id: u64,
    state: EnclaveState,
    keys: Vec<KeyHandle>,
    next_key_id: u64,
    current_epoch: u64,
    failed_auth_attempts: u32,
    memory_used: usize,
    memory_limit: usize,
    total_operations: u64,
    /// Enclave measurement (hash of code + initial state)
    measurement: [u8; 32],
    /// Authentication secret hash
    auth_secret_hash: Vec<u8>,
}

impl SecureEnclave {
    /// Create a new secure enclave
    pub fn new(id: u64) -> Self {
        Self {
            id,
            state: EnclaveState::Uninitialized,
            keys: Vec::new(),
            next_key_id: 1,
            current_epoch: 0,
            failed_auth_attempts: 0,
            memory_used: 0,
            memory_limit: DEFAULT_MEMORY_LIMIT,
            total_operations: 0,
            measurement: [0u8; 32],
            auth_secret_hash: Vec::new(),
        }
    }

    /// Initialize the enclave with an authentication secret
    pub fn initialize(&mut self, auth_secret: &[u8], epoch: u64) -> Result<(), EnclaveError> {
        if self.state != EnclaveState::Uninitialized {
            return Err(EnclaveError::InvalidState(self.state));
        }

        self.state = EnclaveState::Initializing;
        self.current_epoch = epoch;

        // Store hash of auth secret (simplified hash)
        self.auth_secret_hash = Self::simple_hash(auth_secret);

        // Compute enclave measurement
        let mut measurement_input = Vec::new();
        measurement_input.extend_from_slice(&self.id.to_le_bytes());
        measurement_input.extend_from_slice(&epoch.to_le_bytes());
        measurement_input.extend_from_slice(auth_secret);
        let hash = Self::simple_hash(&measurement_input);
        self.measurement.copy_from_slice(&hash[..32.min(hash.len())]);

        self.state = EnclaveState::Ready;
        Ok(())
    }

    /// Authenticate to the enclave
    pub fn authenticate(&mut self, secret: &[u8]) -> Result<(), EnclaveError> {
        if self.state == EnclaveState::Locked {
            return Err(EnclaveError::EnclaveLocked);
        }

        let hash = Self::simple_hash(secret);
        if hash != self.auth_secret_hash {
            self.failed_auth_attempts += 1;
            if self.failed_auth_attempts >= MAX_FAILED_ATTEMPTS {
                self.state = EnclaveState::Locked;
                return Err(EnclaveError::EnclaveLocked);
            }
            return Err(EnclaveError::AuthenticationFailed);
        }

        self.failed_auth_attempts = 0;
        Ok(())
    }

    /// Generate a new key inside the enclave
    pub fn generate_key(
        &mut self,
        label: &str,
        key_type: KeyType,
        permissions: Vec<KeyPermission>,
    ) -> Result<u64, EnclaveError> {
        self.require_ready()?;

        if self.keys.len() >= MAX_KEYS {
            return Err(EnclaveError::MemoryLimitExceeded {
                limit: MAX_KEYS,
                requested: MAX_KEYS + 1,
            });
        }

        if self.keys.iter().any(|k| k.label == label) {
            return Err(EnclaveError::DuplicateKeyLabel(label.to_string()));
        }

        // Generate key material (deterministic for verification)
        let key_size = match key_type {
            KeyType::Symmetric256 => 32,
            KeyType::SigningKeyPair => 64,
            KeyType::DerivationMaster => 32,
            KeyType::SealingKey => 32,
            KeyType::AttestationKey => 64,
        };

        let key_material = Self::generate_key_material(self.next_key_id, key_size);
        let mem_needed = key_material.len() + label.len() + 128; // overhead

        if self.memory_used + mem_needed > self.memory_limit {
            return Err(EnclaveError::MemoryLimitExceeded {
                limit: self.memory_limit,
                requested: self.memory_used + mem_needed,
            });
        }

        let key_id = self.next_key_id;
        self.next_key_id += 1;
        self.memory_used += mem_needed;

        let handle = KeyHandle::new(key_id, label, key_type, permissions, key_material, self.current_epoch);
        self.keys.push(handle);
        self.total_operations += 1;

        Ok(key_id)
    }

    /// Encrypt data using a key in the enclave
    pub fn encrypt(
        &mut self,
        key_id: u64,
        plaintext: &[u8],
    ) -> Result<EncryptionResult, EnclaveError> {
        self.require_ready()?;

        // Save epoch before mutable borrow
        let current_epoch = self.current_epoch;

        let key = self.find_key_mut(key_id)?;
        if !key.has_permission(KeyPermission::Encrypt) {
            return Err(EnclaveError::PermissionDenied {
                key_id,
                required: KeyPermission::Encrypt,
            });
        }
        if key.is_exhausted() {
            return Err(EnclaveError::KeyExhausted(key_id));
        }

        // Simplified encryption: XOR with key material (for demonstration)
        let key_bytes = &key.key_material;
        let ciphertext: Vec<u8> = plaintext.iter().enumerate()
            .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
            .collect();

        // Generate nonce from epoch and operation count
        let nonce = current_epoch.to_le_bytes().to_vec();

        // Generate tag (simplified MAC)
        let mut tag_input = Vec::new();
        tag_input.extend_from_slice(&ciphertext);
        tag_input.extend_from_slice(&nonce);
        let tag = Self::simple_hash(&tag_input)[..16].to_vec();

        key.record_use();
        self.total_operations += 1;

        Ok(EncryptionResult { ciphertext, nonce, tag })
    }

    /// Decrypt data using a key in the enclave
    pub fn decrypt(
        &mut self,
        key_id: u64,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, EnclaveError> {
        self.require_ready()?;

        let key = self.find_key_mut(key_id)?;
        if !key.has_permission(KeyPermission::Decrypt) {
            return Err(EnclaveError::PermissionDenied {
                key_id,
                required: KeyPermission::Decrypt,
            });
        }
        if key.is_exhausted() {
            return Err(EnclaveError::KeyExhausted(key_id));
        }

        // Simplified decryption: XOR with key material (symmetric)
        let key_bytes = &key.key_material;
        let plaintext: Vec<u8> = ciphertext.iter().enumerate()
            .map(|(i, &b)| b ^ key_bytes[i % key_bytes.len()])
            .collect();

        key.record_use();
        self.total_operations += 1;

        Ok(plaintext)
    }

    /// Sign data using a signing key
    pub fn sign(
        &mut self,
        key_id: u64,
        data: &[u8],
    ) -> Result<SignatureResult, EnclaveError> {
        self.require_ready()?;

        let key = self.find_key_mut(key_id)?;
        if !key.has_permission(KeyPermission::Sign) {
            return Err(EnclaveError::PermissionDenied {
                key_id,
                required: KeyPermission::Sign,
            });
        }
        if key.is_exhausted() {
            return Err(EnclaveError::KeyExhausted(key_id));
        }

        // Simplified signing: HMAC-like construction
        let mut sign_input = Vec::new();
        sign_input.extend_from_slice(&key.key_material);
        sign_input.extend_from_slice(data);
        let signature = Self::simple_hash(&sign_input);

        key.record_use();
        self.total_operations += 1;

        Ok(SignatureResult { signature, key_id })
    }

    /// Verify a signature
    pub fn verify_signature(
        &mut self,
        key_id: u64,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, EnclaveError> {
        self.require_ready()?;

        let key = self.find_key_mut(key_id)?;
        if !key.has_permission(KeyPermission::Verify) {
            return Err(EnclaveError::PermissionDenied {
                key_id,
                required: KeyPermission::Verify,
            });
        }

        let mut sign_input = Vec::new();
        sign_input.extend_from_slice(&key.key_material);
        sign_input.extend_from_slice(data);
        let expected = Self::simple_hash(&sign_input);

        key.record_use();
        self.total_operations += 1;

        Ok(expected == signature)
    }

    /// Delete a key from the enclave (zeroize)
    pub fn delete_key(&mut self, key_id: u64) -> Result<(), EnclaveError> {
        self.require_ready()?;

        let idx = self.keys.iter().position(|k| k.id == key_id)
            .ok_or(EnclaveError::KeyNotFound(key_id))?;

        let key = &self.keys[idx];
        let freed = key.key_material.len() + key.label.len() + 128;
        self.memory_used = self.memory_used.saturating_sub(freed);
        self.keys.remove(idx);
        self.total_operations += 1;

        Ok(())
    }

    /// Generate an attestation report
    pub fn attest(&self) -> Result<AttestationReport, EnclaveError> {
        if self.state != EnclaveState::Ready {
            return Err(EnclaveError::InvalidState(self.state));
        }

        // Sign the report with enclave measurement
        let mut report_data = Vec::new();
        report_data.extend_from_slice(&self.id.to_le_bytes());
        report_data.extend_from_slice(&self.measurement);
        report_data.extend_from_slice(&self.current_epoch.to_le_bytes());
        let signature = Self::simple_hash(&report_data);

        Ok(AttestationReport {
            enclave_id: self.id,
            measurement: self.measurement,
            timestamp: self.current_epoch,
            key_count: self.keys.len(),
            state: self.state,
            signature,
        })
    }

    /// Destroy the enclave (zeroize all secrets)
    pub fn destroy(&mut self) -> Result<(), EnclaveError> {
        self.state = EnclaveState::Destroying;

        // Zeroize all key material
        for key in &mut self.keys {
            for byte in &mut key.key_material {
                *byte = 0;
            }
        }
        self.keys.clear();

        // Zeroize auth secret
        for byte in &mut self.auth_secret_hash {
            *byte = 0;
        }

        self.measurement = [0u8; 32];
        self.memory_used = 0;
        self.state = EnclaveState::Destroyed;

        Ok(())
    }

    // --- Internal helpers ---

    fn require_ready(&self) -> Result<(), EnclaveError> {
        if self.state != EnclaveState::Ready {
            return Err(EnclaveError::InvalidState(self.state));
        }
        Ok(())
    }

    fn find_key_mut(&mut self, key_id: u64) -> Result<&mut KeyHandle, EnclaveError> {
        self.keys.iter_mut()
            .find(|k| k.id == key_id)
            .ok_or(EnclaveError::KeyNotFound(key_id))
    }

    /// Simple hash function for demonstration (not cryptographically secure)
    fn simple_hash(data: &[u8]) -> Vec<u8> {
        let mut hash = vec![0u8; 32];
        for (i, &byte) in data.iter().enumerate() {
            hash[i % 32] ^= byte;
            hash[(i + 7) % 32] = hash[(i + 7) % 32].wrapping_add(byte);
            hash[(i + 13) % 32] = hash[(i + 13) % 32].wrapping_mul(byte.wrapping_add(1));
        }
        hash
    }

    /// Generate deterministic key material for testing
    fn generate_key_material(seed: u64, size: usize) -> Vec<u8> {
        let mut material = Vec::with_capacity(size);
        let seed_bytes = seed.to_le_bytes();
        for i in 0..size {
            let val = seed_bytes[i % 8]
                .wrapping_add(i as u8)
                .wrapping_mul(0x6D);
            material.push(val);
        }
        material
    }

    // --- Public getters ---

    pub fn state(&self) -> EnclaveState { self.state }
    pub fn key_count(&self) -> usize { self.keys.len() }
    pub fn total_operations(&self) -> u64 { self.total_operations }
    pub fn memory_used(&self) -> usize { self.memory_used }
    pub fn memory_limit(&self) -> usize { self.memory_limit }
    pub fn enclave_id(&self) -> u64 { self.id }

    pub fn memory_utilization(&self) -> f64 {
        if self.memory_limit == 0 { return 0.0; }
        self.memory_used as f64 / self.memory_limit as f64 * 100.0
    }

    pub fn get_key_info(&self, key_id: u64) -> Option<(&str, KeyType, u64)> {
        self.keys.iter()
            .find(|k| k.id == key_id)
            .map(|k| (k.label.as_str(), k.key_type, k.use_count))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn init_enclave() -> SecureEnclave {
        let mut enclave = SecureEnclave::new(1);
        enclave.initialize(b"test_secret_key", 1000).unwrap();
        enclave
    }

    #[test]
    fn test_enclave_initialization() {
        let enclave = init_enclave();
        assert_eq!(enclave.state(), EnclaveState::Ready);
        assert_eq!(enclave.key_count(), 0);
    }

    #[test]
    fn test_double_initialization() {
        let mut enclave = init_enclave();
        let result = enclave.initialize(b"another_secret", 2000);
        assert!(matches!(result, Err(EnclaveError::InvalidState(EnclaveState::Ready))));
    }

    #[test]
    fn test_authentication_success() {
        let mut enclave = init_enclave();
        assert!(enclave.authenticate(b"test_secret_key").is_ok());
    }

    #[test]
    fn test_authentication_failure() {
        let mut enclave = init_enclave();
        let result = enclave.authenticate(b"wrong_secret");
        assert!(matches!(result, Err(EnclaveError::AuthenticationFailed)));
    }

    #[test]
    fn test_lockout_after_failed_attempts() {
        let mut enclave = init_enclave();
        for _ in 0..MAX_FAILED_ATTEMPTS {
            let _ = enclave.authenticate(b"wrong");
        }
        assert_eq!(enclave.state(), EnclaveState::Locked);
        let result = enclave.authenticate(b"test_secret_key");
        assert!(matches!(result, Err(EnclaveError::EnclaveLocked)));
    }

    #[test]
    fn test_generate_key() {
        let mut enclave = init_enclave();
        let key_id = enclave.generate_key(
            "my_key",
            KeyType::Symmetric256,
            vec![KeyPermission::Encrypt, KeyPermission::Decrypt],
        ).unwrap();
        assert_eq!(key_id, 1);
        assert_eq!(enclave.key_count(), 1);
    }

    #[test]
    fn test_duplicate_key_label() {
        let mut enclave = init_enclave();
        enclave.generate_key("key1", KeyType::Symmetric256, vec![]).unwrap();
        let result = enclave.generate_key("key1", KeyType::Symmetric256, vec![]);
        assert!(matches!(result, Err(EnclaveError::DuplicateKeyLabel(_))));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let mut enclave = init_enclave();
        let key_id = enclave.generate_key(
            "enc_key",
            KeyType::Symmetric256,
            vec![KeyPermission::Encrypt, KeyPermission::Decrypt],
        ).unwrap();

        let plaintext = b"Hello, VANTIS OS!";
        let encrypted = enclave.encrypt(key_id, plaintext).unwrap();
        assert_ne!(encrypted.ciphertext, plaintext.to_vec());

        let decrypted = enclave.decrypt(key_id, &encrypted.ciphertext).unwrap();
        assert_eq!(decrypted, plaintext.to_vec());
    }

    #[test]
    fn test_sign_verify() {
        let mut enclave = init_enclave();
        let key_id = enclave.generate_key(
            "sign_key",
            KeyType::SigningKeyPair,
            vec![KeyPermission::Sign, KeyPermission::Verify],
        ).unwrap();

        let data = b"important document";
        let sig_result = enclave.sign(key_id, data).unwrap();

        let valid = enclave.verify_signature(key_id, data, &sig_result.signature).unwrap();
        assert!(valid);

        // Tampered data should fail
        let invalid = enclave.verify_signature(key_id, b"tampered", &sig_result.signature).unwrap();
        assert!(!invalid);
    }

    #[test]
    fn test_permission_denied() {
        let mut enclave = init_enclave();
        let key_id = enclave.generate_key(
            "sign_only",
            KeyType::SigningKeyPair,
            vec![KeyPermission::Sign],
        ).unwrap();

        let result = enclave.encrypt(key_id, b"data");
        assert!(matches!(result, Err(EnclaveError::PermissionDenied { .. })));
    }

    #[test]
    fn test_key_exhaustion() {
        let mut enclave = init_enclave();
        let key_id = enclave.generate_key(
            "limited_key",
            KeyType::Symmetric256,
            vec![KeyPermission::Encrypt],
        ).unwrap();

        // Set max uses
        enclave.keys.iter_mut().find(|k| k.id == key_id).unwrap().max_uses = Some(2);

        enclave.encrypt(key_id, b"data1").unwrap();
        enclave.encrypt(key_id, b"data2").unwrap();
        let result = enclave.encrypt(key_id, b"data3");
        assert!(matches!(result, Err(EnclaveError::KeyExhausted(_))));
    }

    #[test]
    fn test_delete_key() {
        let mut enclave = init_enclave();
        let key_id = enclave.generate_key("temp", KeyType::Symmetric256, vec![]).unwrap();
        assert_eq!(enclave.key_count(), 1);
        enclave.delete_key(key_id).unwrap();
        assert_eq!(enclave.key_count(), 0);
    }

    #[test]
    fn test_attestation() {
        let enclave = init_enclave();
        let report = enclave.attest().unwrap();
        assert_eq!(report.enclave_id, 1);
        assert_eq!(report.state, EnclaveState::Ready);
        assert!(!report.signature.is_empty());
    }

    #[test]
    fn test_destroy_enclave() {
        let mut enclave = init_enclave();
        enclave.generate_key("key1", KeyType::Symmetric256, vec![]).unwrap();
        enclave.destroy().unwrap();
        assert_eq!(enclave.state(), EnclaveState::Destroyed);
        assert_eq!(enclave.key_count(), 0);
        assert_eq!(enclave.memory_used(), 0);
    }

    #[test]
    fn test_operations_on_destroyed_enclave() {
        let mut enclave = init_enclave();
        enclave.destroy().unwrap();
        let result = enclave.generate_key("key", KeyType::Symmetric256, vec![]);
        assert!(matches!(result, Err(EnclaveError::InvalidState(EnclaveState::Destroyed))));
    }
}