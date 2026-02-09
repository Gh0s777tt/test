//! Comprehensive tests for Vantis Aegis kernel masquerade system

#[cfg(test)]
mod vantis_aegis_integration_tests {
    use crate::vantis_aegis_nt_api::*;
    use crate::vantis_aegis_registry::*;
    use crate::vantis_aegis_syscall::*;
    
    // ========================================================================
    // NT API TESTS
    // ========================================================================
    
    #[test]
    fn test_nt_api_system_information_consistency() {
        let emulator = NtApiEmulator::new();
        
        // Query multiple times - should be consistent
        let info1 = emulator.query_system_basic_information().unwrap();
        let info2 = emulator.query_system_basic_information().unwrap();
        
        assert_eq!(info1.page_size, info2.page_size);
        assert_eq!(info1.number_of_processors, info2.number_of_processors);
        assert_eq!(info1.number_of_physical_pages, info2.number_of_physical_pages);
    }
    
    #[test]
    fn test_nt_api_version_information() {
        let emulator = NtApiEmulator::new();
        let version = emulator.get_version();
        
        // Should report Windows 11
        assert_eq!(version.major_version, 10);
        assert_eq!(version.minor_version, 0);
        assert_eq!(version.build_number, 22631);
        assert_eq!(version.platform_id, 2); // VER_PLATFORM_WIN32_NT
    }
    
    #[test]
    fn test_nt_api_processor_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_system_processor_information().unwrap();
        
        assert_eq!(info.processor_architecture, 9); // AMD64
        assert!(info.maximum_processors > 0);
        assert!(info.maximum_processors <= 256); // Reasonable limit
    }
    
    #[test]
    fn test_nt_api_performance_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_system_performance_information().unwrap();
        
        assert!(info.available_pages > 0);
        assert!(info.committed_pages > 0);
        assert!(info.commit_limit > 0);
        assert!(info.available_pages <= info.commit_limit);
    }
    
    #[test]
    fn test_nt_api_time_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_system_time_information().unwrap();
        
        assert!(info.boot_time > 0);
        assert!(info.current_time > info.boot_time);
    }
    
    #[test]
    fn test_nt_api_no_debugger() {
        let emulator = NtApiEmulator::new();
        let has_debugger = emulator.query_system_kernel_debugger_information().unwrap();
        
        assert!(!has_debugger);
    }
    
    #[test]
    fn test_nt_api_process_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_process_basic_information(1).unwrap();
        
        assert_eq!(info.unique_process_id, 1);
        assert!(info.affinity_mask > 0);
        assert_eq!(info.base_priority, 8); // NORMAL_PRIORITY_CLASS
    }
    
    #[test]
    fn test_nt_api_thread_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_thread_basic_information(1).unwrap();
        
        assert_eq!(info.client_id_thread, 1);
        assert!(info.affinity_mask > 0);
        assert_eq!(info.priority, 8); // THREAD_PRIORITY_NORMAL
    }
    
    // ========================================================================
    // REGISTRY TESTS
    // ========================================================================
    
    #[test]
    fn test_registry_windows_version() {
        let registry = RegistryEmulator::new();
        
        let product_name = registry.query_value(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "ProductName"
        ).unwrap();
        
        match product_name {
            RegistryValue::String(s) => {
                assert!(s.contains("Windows"));
                assert!(s.contains("11"));
            }
            _ => panic!("Expected string value"),
        }
    }
    
    #[test]
    fn test_registry_build_number() {
        let registry = RegistryEmulator::new();
        
        let build = registry.query_value(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "CurrentBuild"
        ).unwrap();
        
        match build {
            RegistryValue::String(s) => assert_eq!(s, "22631"),
            _ => panic!("Expected string value"),
        }
    }
    
    #[test]
    fn test_registry_version_numbers() {
        let registry = RegistryEmulator::new();
        
        let major = registry.query_value(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "CurrentMajorVersionNumber"
        ).unwrap();
        
        let minor = registry.query_value(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "CurrentMinorVersionNumber"
        ).unwrap();
        
        match (major, minor) {
            (RegistryValue::DWord(maj), RegistryValue::DWord(min)) => {
                assert_eq!(maj, 10);
                assert_eq!(min, 0);
            }
            _ => panic!("Expected DWord values"),
        }
    }
    
    #[test]
    fn test_registry_cpu_information() {
        let registry = RegistryEmulator::new();
        
        let cpu_name = registry.query_value(
            "HKLM\\HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0",
            "ProcessorNameString"
        ).unwrap();
        
        match cpu_name {
            RegistryValue::String(s) => assert!(!s.is_empty()),
            _ => panic!("Expected string value"),
        }
    }
    
    #[test]
    fn test_registry_enumerate_subkeys() {
        let registry = RegistryEmulator::new();
        
        let subkeys = registry.enumerate_subkeys("HKLM\\SOFTWARE\\Microsoft").unwrap();
        assert!(subkeys.contains(&"Windows NT".to_string()));
    }
    
    #[test]
    fn test_registry_enumerate_values() {
        let registry = RegistryEmulator::new();
        
        let values = registry.enumerate_values(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion"
        ).unwrap();
        
        assert!(values.contains(&"ProductName".to_string()));
        assert!(values.contains(&"CurrentBuild".to_string()));
        assert!(values.contains(&"CurrentMajorVersionNumber".to_string()));
    }
    
    // ========================================================================
    // SYSCALL TRANSLATION TESTS
    // ========================================================================
    
    #[test]
    fn test_syscall_query_system_information() {
        let translator = SyscallTranslator::new();
        let args = SyscallArgs::new(vec![0]); // SystemBasicInformation
        
        let result = translator.translate_syscall(0x0018, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // STATUS_SUCCESS
    }
    
    #[test]
    fn test_syscall_query_process_information() {
        let translator = SyscallTranslator::new();
        let args = SyscallArgs::new(vec![0xFFFFFFFF, 0]); // Current process, ProcessBasicInformation
        
        let result = translator.translate_syscall(0x0019, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // STATUS_SUCCESS
    }
    
    #[test]
    fn test_syscall_query_thread_information() {
        let translator = SyscallTranslator::new();
        let args = SyscallArgs::new(vec![0xFFFFFFFE, 0]); // Current thread, ThreadBasicInformation
        
        let result = translator.translate_syscall(0x0025, &args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // STATUS_SUCCESS
    }
    
    #[test]
    fn test_syscall_registry_operations() {
        let translator = SyscallTranslator::new();
        
        // Open key
        let open_args = SyscallArgs::new(vec![0, 0, 0]);
        let handle = translator.translate_syscall(0x000F, &open_args).unwrap();
        assert!(handle > 0);
        
        // Query value
        let query_args = SyscallArgs::new(vec![handle, 0, 0, 0, 0]);
        let result = translator.translate_syscall(0x0017, &query_args);
        assert!(result.is_ok());
        
        // Close key
        let close_args = SyscallArgs::new(vec![handle]);
        let result = translator.translate_syscall(0x000C, &close_args);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_syscall_file_operations() {
        let translator = SyscallTranslator::new();
        
        // Create file
        let create_args = SyscallArgs::new(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let handle = translator.translate_syscall(0x0033, &create_args).unwrap();
        assert!(handle > 0);
        
        // Read file
        let read_args = SyscallArgs::new(vec![handle, 0, 0, 0, 0, 0, 0, 0, 0]);
        let result = translator.translate_syscall(0x0003, &read_args);
        assert!(result.is_ok());
        
        // Write file
        let write_args = SyscallArgs::new(vec![handle, 0, 0, 0, 0, 0, 0, 0, 0]);
        let result = translator.translate_syscall(0x0008, &write_args);
        assert!(result.is_ok());
        
        // Close file
        let close_args = SyscallArgs::new(vec![handle]);
        let result = translator.translate_syscall(0x000C, &close_args);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_syscall_name_lookup() {
        let translator = SyscallTranslator::new();
        
        assert_eq!(translator.get_syscall_name(0x0018), Some("NtQuerySystemInformation"));
        assert_eq!(translator.get_syscall_name(0x0019), Some("NtQueryInformationProcess"));
        assert_eq!(translator.get_syscall_name(0x0025), Some("NtQueryInformationThread"));
        assert_eq!(translator.get_syscall_name(0xFFFF), None);
    }
    
    // ========================================================================
    // INTEGRATION TESTS
    // ========================================================================
    
    #[test]
    fn test_anti_cheat_detection_simulation() {
        // Simulate common anti-cheat queries
        let emulator = NtApiEmulator::new();
        let registry = RegistryEmulator::new();
        
        // 1. Check Windows version
        let version = emulator.get_version();
        assert_eq!(version.major_version, 10);
        assert_eq!(version.build_number, 22631);
        
        // 2. Check product name
        let product_name = registry.query_value(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "ProductName"
        ).unwrap();
        match product_name {
            RegistryValue::String(s) => assert!(s.contains("Windows")),
            _ => panic!("Expected string value"),
        }
        
        // 3. Check for debugger
        let has_debugger = emulator.query_system_kernel_debugger_information().unwrap();
        assert!(!has_debugger);
        
        // 4. Check process info
        let proc_info = emulator.query_process_basic_information(1).unwrap();
        assert_eq!(proc_info.unique_process_id, 1);
        
        // 5. Check system info
        let sys_info = emulator.query_system_basic_information().unwrap();
        assert_eq!(sys_info.page_size, 4096);
        assert!(sys_info.number_of_processors > 0);
    }
    
    #[test]
    fn test_consistency_across_modules() {
        let emulator = NtApiEmulator::new();
        let registry = RegistryEmulator::new();
        
        // Version should match between NT API and Registry
        let nt_version = emulator.get_version();
        let reg_build = registry.query_value(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "CurrentBuild"
        ).unwrap();
        
        match reg_build {
            RegistryValue::String(s) => {
                assert_eq!(s, nt_version.build_number.to_string());
            }
            _ => panic!("Expected string value"),
        }
    }
    
    #[test]
    fn test_performance_overhead() {
        use std::time::Instant;
        
        let emulator = NtApiEmulator::new();
        let iterations = 1000;
        
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = emulator.query_system_basic_information();
        }
        let duration = start.elapsed();
        
        let avg_time = duration.as_micros() / iterations;
        
        // Should be fast (< 100 microseconds per query)
        assert!(avg_time < 100, "Query too slow: {} microseconds", avg_time);
    }
}