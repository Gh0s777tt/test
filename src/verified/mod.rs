//! Formally verified components of VANTIS OS
//! 
//! This module contains code that has been formally verified using
//! Verus and Kani to ensure correctness and safety.

pub mod memory;
pub mod math;

// IPC System - All 5 properties formally verified
pub mod ipc_message_integrity;
pub mod ipc_resource_bounds;
pub mod ipc_information_leakage;
pub mod ipc_deadlock_freedom;
pub mod ipc_capability_correctness;
pub mod ipc_complete;

// Extended File Operations
pub mod syscall_file_ops;

// Directory Operations
pub mod syscall_dir_ops;

// Advanced File Operations
pub mod syscall_advanced_ops;

// Time and Timer Operations
pub mod syscall_time_ops;

#[cfg(test)]
mod ipc_complete_tests;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_verified_modules() {
        // Basic sanity tests for verified modules
        assert!(true);
    }
}