// Security Tests for VantisOS v0.6.0 ARM64 kernel
// Phase 4: Testing and Documentation - Security Tests

pub mod security_test;
pub mod memory_protection_test;
pub mod access_control_test;
pub mod sandbox_test;

// Re-export test functions
pub use security_test::run_security_tests;
pub use memory_protection_test::run_memory_protection_tests;
pub use access_control_test::run_access_control_tests;
pub use sandbox_test::run_sandbox_tests;
