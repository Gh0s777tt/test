//! VANTIS OS Verified Components Library
//! 
//! This library contains all formally verified components of VANTIS OS.

// Verus shim for conditional compilation
#[cfg(not(feature = "verus"))]
#[macro_use]
pub mod verus_shim;

pub mod math;
pub mod memory;
pub mod allocator;
pub mod process;
pub mod ipc;
pub mod ipc_inline;
pub mod syscall;
pub mod syscall_file_ops;
pub mod syscall_dir_ops;
pub mod syscall_advanced_ops;
pub mod syscall_time_ops;
pub mod scheduler;
pub mod scheduler_optimized;
pub mod neural_scheduler;
pub mod neural_scheduler_integration;
pub mod workload_predictor;
pub mod vantisfs_block_allocator;
pub mod vantisfs_inode;
pub mod vantisfs_ab;
pub mod vantisfs_data;
pub mod vantisfs_recovery;
pub mod vault;
pub mod vault_simple_demo;
pub mod vault_aes;
pub mod vault_twofish;
pub mod vault_serpent;
pub mod vault_cascade;
pub mod vault_fips_tests;
pub mod vault_production_example;
pub mod direct_metal;
pub mod direct_metal_backend;
pub mod direct_metal_vulkan;
pub mod direct_metal_metal;
pub mod flux_engine;
pub mod flux_wayland;
pub mod flux_window;
pub mod flux_compositor;
pub mod flux_hdr;
pub mod flux_gaming;
pub mod horizon_profiles;
pub mod horizon_gamer;
pub mod horizon_wraith;
pub mod horizon_creator;
pub mod horizon_enterprise;
pub mod sentinel;
pub mod sentinel_sandbox;
pub mod sentinel_lifecycle;
pub mod sentinel_recovery;
pub mod sentinel_fingerprint;
pub mod sentinel_api;
// Temporarily disabled due to string escaping issues
// pub mod vantis_aegis;
// pub mod vantis_aegis_nt_api;
// pub mod vantis_aegis_registry;
// pub mod vantis_aegis_syscall;
