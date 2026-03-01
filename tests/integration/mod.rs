// Integration tests for VantisOS vantisOS v0.6.0 ARM64 kernel
// Phase 4: Testing and Documentation - Integration Tests

pub mod kernel_integration_test;
pub mod driver_integration_test;
pub mod ui_integration_test;
pub mod app_integration_test;

// Re-export test functions
pub use kernel_integration_test::run_kernel_integration_tests;
pub use driver_integration_test::run_driver_integration_tests;
pub use ui_integration_test::run_ui_integration_tests;
pub use app_integration_test::run_app_integration_tests;
