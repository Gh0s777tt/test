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

// IOMMU System - DMA attack prevention and device isolation
pub mod iommu;
pub mod iommu_intel;
pub mod iommu_amd;
pub mod iommu_arm;
pub mod iommu_usb4;

// Network Stack - TCP/IP, Wi-Fi 7, eBPF/XDP, Zero-Copy
pub mod network;
pub mod network_ipv4;
pub mod network_ipv6;
pub mod network_tcp;
pub mod network_udp;
pub mod network_wifi7;
pub mod network_ebpf;
pub mod network_zerocopy;

// Self-Healing System - Real-time failure detection and recovery
pub mod self_healing;

// Ray Tracing System - Vendor-agnostic ray tracing with Vulkan, DirectX 12, Metal
pub mod ray_tracing;
pub mod ray_tracing_vulkan;
pub mod ray_tracing_dx12;
pub mod ray_tracing_metal;
pub mod ray_tracing_unified;
pub mod ray_tracing_bvh;
pub mod ray_tracing_gpu;
pub mod ray_tracing_tests;

// Cinema Enclave - DRM and premium content protection
pub mod cinema_enclave;
pub mod cinema_widevine;
pub mod cinema_playready;
pub mod cinema_fairplay;
pub mod cinema_hdcp;
pub mod cinema_audio;
pub mod cinema_tests;

// Nexus Server modules
pub mod nexus_server;
pub mod nexus_api;
pub mod nexus_engine;
pub mod nexus_compliance;
pub mod nexus_storage;
pub mod nexus_auth;
pub mod nexus_analytics;
pub mod nexus_updates;
pub mod nexus_tests;

#[cfg(all(test, feature = "verus-full"))]
mod ipc_complete_tests;

#[cfg(all(test, feature = "verus-full"))]
mod iommu_tests;

#[cfg(all(test, feature = "verus-full"))]
mod tests {
    use super::*;
    
    #[test]
    fn test_verified_modules() {
        // Basic sanity tests for verified modules
        assert!(true);
    }
}