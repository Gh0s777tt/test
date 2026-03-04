//! Formal Verification Module for VantisOS AI
//! 
//! This module contains Verus-verified versions of AI components.
//! Each verified module includes:
//! - Memory safety proofs
//! - Correctness properties
//! - Bounded resource usage guarantees
//! - No undefined behavior
//! 
//! ## Verification Status
//! 
//! | Module | Verified | Properties |
//! |--------|----------|------------|
//! | Core | ✅ | Model bounds, resource limits |
//! | Scheduler | ⏳ | Q-learning invariants |
//! | Power Manager | ⏳ | Frequency bounds, thermal safety |
//! | Security | ⏳ | Feature safety, classification bounds |
//! | Load Balancer | ⏳ | Node selection, Thompson sampling |
//! 
//! ## Verification Approach
//! 
//! 1. **Memory Safety**: All array accesses bounds-checked, no null dereferences
//! 2. **Correctness**: Pre/postconditions for all public functions
//! 3. **Resource Bounds**: Explicit limits on memory, CPU, allocations
//! 4. **Invariants**: Ghost variables track key properties
//! 
//! ## Using Verified Modules
//! 
//! To use verified modules instead of unverified ones:
//! 
//! ```rust,no_run
//! use vantisos::ai::verification::core_verified::VerifiedAICore;
//! 
//! let core = VerifiedAICore::new(config)?;
//! ```

pub mod core_verified;
pub mod scheduler_verified;
pub mod power_manager_verified;

// Re-export commonly used verified types
pub use core_verified::VerifiedAICore;
pub use scheduler_verified::VerifiedMLScheduler;
pub use power_manager_verified::VerifiedPowerManager;

/// Verification configuration
#[derive(Debug, Clone)]
pub struct VerificationConfig {
    /// Enable runtime assertion checks
    pub enable_runtime_checks: bool,
    /// Enable verification proofs
    pub enable_proofs: bool,
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            enable_runtime_checks: true,
            enable_proofs: true,
        }
    }
}

/// Verification statistics
#[derive(Debug, Clone, Copy)]
pub struct VerificationStats {
    /// Number of verified functions
    pub verified_functions: usize,
    /// Number of proven lemmas
    pub proven_lemmas: usize,
    /// Verification time in milliseconds
    pub verification_time_ms: u64,
}

/// Verification result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationResult {
    /// Verification successful
    Success,
    /// Verification failed with error
    Failed(String),
    /// Verification timed out
    Timeout,
}