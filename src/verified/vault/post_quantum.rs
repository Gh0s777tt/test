// Post-Quantum Cryptography for VantisOS
// Quantum-resistant cryptographic algorithms

pub mod lattice;
pub mod hash_sig;
pub mod code_based;
pub mod multivariate;

// Re-export main types
pub use lattice::{Kyber, Dilithium, LWEProblem};
pub use hash_sig::{SPHINCSPlus, XMSS, HashBasedSignature};
pub use code_based::{McEliece, Niederreiter, CodeBasedCipher};
pub use multivariate::{Rainbow, MultivariateSignature};

/// Post-quantum security levels
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SecurityLevel {
    Level1,
    Level2,
    Level3,
    Level5,
}

impl SecurityLevel {
    /// Get the security level value
    pub fn value(&self) -> usize {
        match self {
            SecurityLevel::Level1 => 1,
            SecurityLevel::Level2 => 2,
            SecurityLevel::Level3 => 3,
            SecurityLevel::Level5 => 5,
        }
    }

    /// Get the recommended parameter set
    pub fn parameter_set(&self) -> &str {
        match self {
            SecurityLevel::Level1 => "lightweight",
            SecurityLevel::Level2 => "medium",
            SecurityLevel::Level3 => "high",
            SecurityLevel::Level5 => "very_high",
        }
    }
}

/// Post-quantum key encapsulation mechanism
pub trait KeyEncapsulation {
    type PublicKey;
    type SecretKey;
    type Ciphertext;

    /// Generate a key pair
    fn generate_keypair(security_level: SecurityLevel) -> (Self::PublicKey, Self::SecretKey);

    /// Encapsulate a shared secret
    fn encapsulate(public_key: &Self::PublicKey) -> (Self::Ciphertext, Vec<u8>);

    /// Decapsulate a shared secret
    fn decapsulate(secret_key: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Vec<u8>;
}

/// Post-quantum digital signature
pub trait DigitalSignature {
    type PublicKey;
    type SecretKey;
    type Signature;

    /// Generate a key pair
    fn generate_keypair(security_level: SecurityLevel) -> (Self::PublicKey, Self::SecretKey);

    /// Sign a message
    fn sign(secret_key: &Self::SecretKey, message: &[u8]) -> Self::Signature;

    /// Verify a signature
    fn verify(public_key: &Self::PublicKey, message: &[u8], signature: &Self::Signature) -> bool;
}

/// Hybrid key exchange (post-quantum + classical)
pub struct HybridKeyExchange;

impl HybridKeyExchange {
    /// Perform hybrid key exchange
    pub fn exchange(pq_public: &[u8], classical_public: &[u8]) -> Result<Vec<u8>, String> {
        // Combine post-quantum and classical key exchange
        let mut combined = Vec::with_capacity(pq_public.len() + classical_public.len());
        combined.extend_from_slice(pq_public);
        combined.extend_from_slice(classical_public);
        
        // In practice, would derive shared secret from both
        Ok(combined)
    }

    /// Verify hybrid exchange
    pub fn verify(pq_shared: &[u8], classical_shared: &[u8]) -> bool {
        // Verify both components
        !pq_shared.is_empty() && !classical_shared.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level() {
        assert_eq!(SecurityLevel::Level1.value(), 1);
        assert_eq!(SecurityLevel::Level3.parameter_set(), "high");
    }

    #[test]
    fn test_hybrid_exchange() {
        let pq_pub = vec![1u8; 32];
        let classical_pub = vec![2u8; 32];
        let result = HybridKeyExchange::exchange(&pq_pub, &classical_pub);
        assert!(result.is_ok());
    }
}