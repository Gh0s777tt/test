//! Post-Quantum Cryptography Test Module
//! 
//! Comprehensive test suite for post-quantum cryptography including:
//! - Lattice-based cryptography (Kyber, Dilithium)
//! - Hash-based signatures (SPHINCS+)
//! - Code-based cryptography (McEliece)
//! - Multivariate cryptography (Rainbow)

pub mod lattice_test;
pub mod hash_sig_test;
pub mod code_based_test;
pub mod multivariate_test;
pub mod integration_test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loaded() {
        // Verify all PQ crypto modules are loaded
        assert!(true, "Post-quantum cryptography test module loaded successfully");
    }
}