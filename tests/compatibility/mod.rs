// Compatibility Tests for VantisOS v0.6.0 ARM64 kernel
// Phase 4: Testing and Documentation - Compatibility Tests

pub mod arm64_compatibility_test;
pub mod driver_compatibility_test;
pub mod ui_compatibility_test;
pub mod app_compatibility_test;

// Re-export test functions
pub use arm64_compatibility_test::run_arm64_compatibility_tests;
pub use driver_compatibility_test::run_driver_compatibility_tests;
pub use ui_compatibility_test::run_ui_compatibility_tests;
pub use app_compatibility_test::run_app_compatibility_tests;
