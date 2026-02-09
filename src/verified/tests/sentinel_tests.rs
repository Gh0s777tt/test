//! Sentinel Integration Tests
//! 
//! Comprehensive tests for the Sentinel Hardware Abstraction Layer

#[cfg(test)]
mod sentinel_integration_tests {
    use crate::sentinel::*;
    use crate::sentinel_sandbox::*;
    use crate::sentinel_lifecycle::*;
    use crate::sentinel_recovery::*;
    use crate::sentinel_fingerprint::*;
    use crate::sentinel_api::*;

    #[test]
    fn test_complete_driver_lifecycle() {
        // Initialize all components
        let mut sentinel = Sentinel::new();
        let mut sandbox_mgr = SandboxManager::new();
        let mut lifecycle_mgr = LifecycleManager::new();
        let mut recovery_mgr = RecoveryManager::new();
        let mut api = DriverApi::new();

        sentinel.init().unwrap();
        sandbox_mgr.init().unwrap();
        lifecycle_mgr.init().unwrap();
        recovery_mgr.init().unwrap();
        api.init().unwrap();

        // Register driver
        let driver_id = sentinel.register_driver(
            String::from("test_driver"),
            String::from("1.0.0"),
            SecurityPolicy::Standard,
        ).unwrap();

        // Create sandbox
        let limits = ResourceLimits::default();
        let sandbox_id = sandbox_mgr.create_sandbox(driver_id, limits).unwrap();

        // Grant capabilities
        sandbox_mgr.grant_capability(sandbox_id, Capability::Memory).unwrap();
        sandbox_mgr.grant_capability(sandbox_id, Capability::Interrupt).unwrap();

        // Load driver
        let version = Version::new(1, 0, 0);
        lifecycle_mgr.load_driver(
            driver_id,
            String::from("test_driver"),
            String::from("/drivers/test.ko"),
            version,
        ).unwrap();

        // Start watchdog
        recovery_mgr.start_watchdog(driver_id, 1000).unwrap();

        // Start driver
        lifecycle_mgr.start_driver(driver_id).unwrap();

        // Initialize driver API
        api.driver_init(driver_id).unwrap();

        // Verify everything is running
        assert_eq!(lifecycle_mgr.get_driver_state(driver_id), Some(DriverState::Running));
        assert_eq!(recovery_mgr.health_check(driver_id), HealthStatus::Healthy);

        // Cleanup
        api.driver_shutdown(driver_id).unwrap();
        lifecycle_mgr.stop_driver(driver_id).unwrap();
        recovery_mgr.stop_watchdog(driver_id).unwrap();
        sandbox_mgr.destroy_sandbox(sandbox_id).unwrap();
        sentinel.unregister_driver(driver_id).unwrap();
    }

    #[test]
    fn test_driver_fault_recovery() {
        let mut sentinel = Sentinel::new();
        let mut recovery_mgr = RecoveryManager::new();
        let mut lifecycle_mgr = LifecycleManager::new();

        sentinel.init().unwrap();
        recovery_mgr.init().unwrap();
        lifecycle_mgr.init().unwrap();

        // Register and start driver
        let driver_id = sentinel.register_driver(
            String::from("test_driver"),
            String::from("1.0.0"),
            SecurityPolicy::Standard,
        ).unwrap();

        let version = Version::new(1, 0, 0);
        lifecycle_mgr.load_driver(
            driver_id,
            String::from("test_driver"),
            String::from("/drivers/test.ko"),
            version,
        ).unwrap();

        lifecycle_mgr.start_driver(driver_id).unwrap();
        recovery_mgr.start_watchdog(driver_id, 1000).unwrap();

        // Simulate fault
        recovery_mgr.detect_fault(driver_id, FaultType::Crash).unwrap();

        // Recover driver
        recovery_mgr.recover_driver(driver_id).unwrap();

        // Verify recovery
        let stats = recovery_mgr.get_recovery_stats();
        assert_eq!(stats.total_faults, 1);
        assert_eq!(stats.successful_recoveries, 1);
        assert!(stats.avg_recovery_time_us < 500_000); // < 500ms
    }

    #[test]
    fn test_hardware_detection_and_driver_matching() {
        let mut scanner = HardwareScanner::new();
        scanner.init().unwrap();

        // Scan hardware
        scanner.scan_hardware().unwrap();

        let fingerprint = scanner.get_fingerprint().unwrap();

        // Verify CPU detected
        assert!(!fingerprint.cpu.vendor.is_empty());
        assert!(fingerprint.cpu.cores > 0);

        // Verify GPU detected
        assert!(!fingerprint.gpus.is_empty());
        assert!(fingerprint.gpus[0].memory_mb > 0);

        // Register driver match
        scanner.register_driver_match(
            String::from("10de:2684"),
            String::from("nvidia_driver"),
        );

        // Match driver
        let driver = scanner.match_driver("10de:2684");
        assert_eq!(driver, Some(String::from("nvidia_driver")));
    }

    #[test]
    fn test_dma_and_interrupt_handling() {
        let mut api = DriverApi::new();
        api.init().unwrap();

        let driver_id = 1;

        // Allocate DMA buffer
        let dma_handle = api.allocate_dma(driver_id, 4096).unwrap();
        let buffer = api.get_dma_buffer(dma_handle).unwrap();
        assert_eq!(buffer.size, 4096);

        // Map memory
        let vaddr = api.map_memory(driver_id, 0x1000, 4096).unwrap();
        let region = api.get_memory_region(vaddr).unwrap();
        assert_eq!(region.size, 4096);

        // Register interrupt
        api.register_interrupt(driver_id, 10).unwrap();

        // Handle interrupt
        let result = api.handle_interrupt(driver_id, 10);
        assert_eq!(result, InterruptResult::Handled);

        // Cleanup
        api.unregister_interrupt(driver_id, 10).unwrap();
        api.unmap_memory(vaddr).unwrap();
        api.free_dma(dma_handle).unwrap();
    }

    #[test]
    fn test_sandbox_resource_limits() {
        let mut sandbox_mgr = SandboxManager::new();
        sandbox_mgr.init().unwrap();

        let limits = ResourceLimits {
            max_memory: 1024,
            max_cpu_time: 1_000_000,
            max_io_ops: 10_000,
        };

        let sandbox_id = sandbox_mgr.create_sandbox(1, limits).unwrap();

        // Allocate within limits
        sandbox_mgr.isolate_memory(sandbox_id, 0x1000, 512).unwrap();
        assert!(sandbox_mgr.enforce_limits(sandbox_id).is_ok());

        // Exceed limits
        sandbox_mgr.isolate_memory(sandbox_id, 0x2000, 1024).ok();
        assert!(sandbox_mgr.enforce_limits(sandbox_id).is_err());
    }

    #[test]
    fn test_hot_reload() {
        let mut lifecycle_mgr = LifecycleManager::new();
        lifecycle_mgr.init().unwrap();

        let version = Version::new(1, 0, 0);
        lifecycle_mgr.load_driver(
            1,
            String::from("test_driver"),
            String::from("/drivers/test_v1.ko"),
            version,
        ).unwrap();

        lifecycle_mgr.start_driver(1).unwrap();

        // Hot reload with new version
        lifecycle_mgr.hot_reload_driver(1, String::from("/drivers/test_v2.ko")).unwrap();

        // Verify still running
        assert_eq!(lifecycle_mgr.get_driver_state(1), Some(DriverState::Running));
    }

    #[test]
    fn test_state_preservation_and_restoration() {
        let mut recovery_mgr = RecoveryManager::new();
        recovery_mgr.init().unwrap();

        let state_data = vec![1, 2, 3, 4, 5];
        recovery_mgr.preserve_state(1, state_data.clone()).unwrap();

        let restored = recovery_mgr.restore_state(1);
        assert_eq!(restored, Some(state_data));
    }

    #[test]
    fn test_capability_based_security() {
        let mut sandbox_mgr = SandboxManager::new();
        sandbox_mgr.init().unwrap();

        let limits = ResourceLimits::default();
        let sandbox_id = sandbox_mgr.create_sandbox(1, limits).unwrap();

        // Initially no capabilities
        assert!(!sandbox_mgr.check_capability(sandbox_id, Capability::Memory));
        assert!(!sandbox_mgr.check_capability(sandbox_id, Capability::Interrupt));

        // Grant capabilities
        sandbox_mgr.grant_capability(sandbox_id, Capability::Memory).unwrap();
        assert!(sandbox_mgr.check_capability(sandbox_id, Capability::Memory));
        assert!(!sandbox_mgr.check_capability(sandbox_id, Capability::Interrupt));

        // Revoke capability
        sandbox_mgr.revoke_capability(sandbox_id, Capability::Memory).unwrap();
        assert!(!sandbox_mgr.check_capability(sandbox_id, Capability::Memory));
    }

    #[test]
    fn test_ipc_communication() {
        let mut sandbox_mgr = SandboxManager::new();
        sandbox_mgr.init().unwrap();

        let limits = ResourceLimits::default();
        let sender_id = sandbox_mgr.create_sandbox(1, limits).unwrap();
        let receiver_id = sandbox_mgr.create_sandbox(2, limits).unwrap();

        // Setup IPC channels
        sandbox_mgr.setup_ipc_channel(sender_id).unwrap();
        sandbox_mgr.setup_ipc_channel(receiver_id).unwrap();

        // Send message
        let data = vec![1, 2, 3, 4, 5];
        sandbox_mgr.send_message(sender_id, receiver_id, data).unwrap();

        // Verify stats
        let sender_stats = sandbox_mgr.get_sandbox_stats(sender_id).unwrap();
        assert_eq!(sender_stats.messages_sent, 1);

        let receiver_stats = sandbox_mgr.get_sandbox_stats(receiver_id).unwrap();
        assert_eq!(receiver_stats.messages_received, 1);
    }

    #[test]
    fn test_watchdog_timeout_detection() {
        let mut recovery_mgr = RecoveryManager::new();
        recovery_mgr.init().unwrap();

        recovery_mgr.start_watchdog(1, 1000).unwrap();

        // Initially healthy
        assert_eq!(recovery_mgr.health_check(1), HealthStatus::Healthy);

        // Advance time past timeout
        recovery_mgr.advance_time(1500);

        // Should be failed
        assert_eq!(recovery_mgr.health_check(1), HealthStatus::Failed);
    }
}