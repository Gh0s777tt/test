// Performance tests for VantisOS v0.6.0 ARM64 kernel
// Phase 4: Testing and Documentation - Performance Tests

pub mod kernel_performance_test;
pub mod ui_performance_test;
pub mod benchmarks;

// Re-export test functions
pub use kernel_performance_test::run_kernel_performance_tests;
pub use ui_performance_test::run_ui_performance_tests;
pub use benchmarks::run_benchmarks;
