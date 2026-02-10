//! Standalone Sentinel Tests
//! 
//! This file can be compiled independently to test Sentinel modules

#![allow(dead_code)]
#![allow(unused_imports)]

use alloc::string::String;
use alloc::vec::Vec;

// Import all Sentinel modules
mod sentinel;
mod sentinel_sandbox;
mod sentinel_lifecycle;
mod sentinel_recovery;
mod sentinel_fingerprint;
mod sentinel_api;

use sentinel::*;
use sentinel_sandbox::*;
use sentinel_lifecycle::*;
use sentinel_recovery::*;
use sentinel_fingerprint::*;
use sentinel_api::*;

#[cfg(all(test, feature = "verus"))]
mod tests {
    use super::*;

    #[test]
    fn test_all_sentinel_modules() {
        // Test Sentinel core
        let mut sentinel = Sentinel::new();
        assert!(sentinel.init().is_ok());

        // Test Sandbox
        let mut sandbox_mgr = SandboxManager::new();
        assert!(sandbox_mgr.init().is_ok());

        // Test Lifecycle
        let mut lifecycle_mgr = LifecycleManager::new();
        assert!(lifecycle_mgr.init().is_ok());

        // Test Recovery
        let mut recovery_mgr = RecoveryManager::new();
        assert!(recovery_mgr.init().is_ok());

        // Test Fingerprint
        let mut scanner = HardwareScanner::new();
        assert!(scanner.init().is_ok());

        // Test API
        let mut api = DriverApi::new();
        assert!(api.init().is_ok());

        println!("✅ All Sentinel modules initialized successfully!");
    }
}

fn main() {
    println!("Sentinel HAL - Standalone Test");
    println!("================================");
    
    // Run basic tests
    test_sentinel_core();
    test_sandbox_system();
    test_lifecycle_management();
    test_recovery_system();
    test_hardware_detection();
    test_driver_api();
    
    println!("\n✅ All tests passed!");
}

fn test_sentinel_core() {
    println!("\n📦 Testing Sentinel Core...");
    
    let mut sentinel = Sentinel::new();
    sentinel.init().unwrap();
    
    let id = sentinel.register_driver(
        String::from("test_driver"),
        String::from("1.0.0"),
        SecurityPolicy::Standard,
    ).unwrap();
    
    assert!(id > 0);
    println!("   ✓ Driver registration");
    
    sentinel.allocate_resources(id, ResourceType::Memory, 1024).unwrap();
    println!("   ✓ Resource allocation");
    
    sentinel.unregister_driver(id).unwrap();
    println!("   ✓ Driver unregistration");
}

fn test_sandbox_system() {
    println!("\n🔒 Testing Sandbox System...");
    
    let mut manager = SandboxManager::new();
    manager.init().unwrap();
    
    let limits = ResourceLimits::default();
    let id = manager.create_sandbox(1, limits).unwrap();
    println!("   ✓ Sandbox creation");
    
    manager.grant_capability(id, Capability::Memory).unwrap();
    assert!(manager.check_capability(id, Capability::Memory));
    println!("   ✓ Capability management");
    
    manager.isolate_memory(id, 0x1000, 4096).unwrap();
    println!("   ✓ Memory isolation");
    
    manager.destroy_sandbox(id).unwrap();
    println!("   ✓ Sandbox destruction");
}

fn test_lifecycle_management() {
    println!("\n🔄 Testing Lifecycle Management...");
    
    let mut manager = LifecycleManager::new();
    manager.init().unwrap();
    
    let version = Version::new(1, 0, 0);
    manager.load_driver(
        1,
        String::from("test_driver"),
        String::from("/drivers/test.ko"),
        version,
    ).unwrap();
    println!("   ✓ Driver loading");
    
    manager.start_driver(1).unwrap();
    println!("   ✓ Driver starting");
    
    manager.stop_driver(1).unwrap();
    println!("   ✓ Driver stopping");
    
    manager.unload_driver(1).unwrap();
    println!("   ✓ Driver unloading");
}

fn test_recovery_system() {
    println!("\n🚑 Testing Recovery System...");
    
    let mut manager = RecoveryManager::new();
    manager.init().unwrap();
    
    manager.start_watchdog(1, 1000).unwrap();
    println!("   ✓ Watchdog started");
    
    assert_eq!(manager.health_check(1), HealthStatus::Healthy);
    println!("   ✓ Health check");
    
    manager.detect_fault(1, FaultType::Crash).unwrap();
    println!("   ✓ Fault detection");
    
    manager.recover_driver(1).unwrap();
    let stats = manager.get_recovery_stats();
    assert!(stats.avg_recovery_time_us < 500_000);
    println!("   ✓ Driver recovery (<500ms)");
}

fn test_hardware_detection() {
    println!("\n🔍 Testing Hardware Detection...");
    
    let mut scanner = HardwareScanner::new();
    scanner.init().unwrap();
    
    scanner.scan_hardware().unwrap();
    println!("   ✓ Hardware scan");
    
    let fingerprint = scanner.get_fingerprint().unwrap();
    assert!(!fingerprint.cpu.vendor.is_empty());
    println!("   ✓ CPU detection: {}", fingerprint.cpu.vendor);
    
    assert!(!fingerprint.gpus.is_empty());
    println!("   ✓ GPU detection: {}", fingerprint.gpus[0].vendor);
    
    let device_id = scanner.get_device_id().unwrap();
    println!("   ✓ Device ID: {}", device_id);
}

fn test_driver_api() {
    println!("\n⚙️  Testing Driver API...");
    
    let mut api = DriverApi::new();
    api.init().unwrap();
    
    let handle = api.allocate_dma(1, 4096).unwrap();
    println!("   ✓ DMA allocation");
    
    let vaddr = api.map_memory(1, 0x1000, 4096).unwrap();
    println!("   ✓ Memory mapping");
    
    api.register_interrupt(1, 10).unwrap();
    println!("   ✓ Interrupt registration");
    
    let result = api.handle_interrupt(1, 10);
    assert_eq!(result, InterruptResult::Handled);
    println!("   ✓ Interrupt handling");
    
    api.unregister_interrupt(1, 10).unwrap();
    api.unmap_memory(vaddr).unwrap();
    api.free_dma(handle).unwrap();
    println!("   ✓ Resource cleanup");
}