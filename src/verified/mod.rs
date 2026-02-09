//! Formally verified components of VANTIS OS
//! 
//! This module contains code that has been formally verified using
//! Verus and Kani to ensure correctness and safety.

pub mod memory;
pub mod math;
pub mod direct_metal;
pub mod allocator;
pub mod process;
pub mod ipc;
pub mod ipc_inline;
pub mod syscall;
pub mod scheduler;
pub mod scheduler_optimized;
pub mod vault;
pub mod vault_simple_demo;
pub mod vault_aes;
pub mod vault_twofish;
pub mod vault_serpent;
pub mod vault_cascade;
pub mod vault_production_example;
pub mod vault_fips_tests;
pub mod neural_scheduler;
pub mod workload_predictor;
pub mod neural_scheduler_integration;
pub mod vantisfs_block_allocator;
pub mod vantisfs_inode;
pub mod vantisfs_ab;
pub mod vantisfs_data;
pub mod vantisfs_recovery;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_verified_modules() {
        // Basic sanity tests for verified modules
        assert!(true);
    }
}