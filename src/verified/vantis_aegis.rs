//! Vantis Aegis - Main Integration Module
//! 
//! This module provides the main interface for the Vantis Aegis kernel masquerade
//! system, integrating NT API emulation, registry emulation, and syscall translation.
//!
//! # Usage
//! 
//! ```rust
//! use vantis_aegis::VantisAegis;
//! 
//! // Initialize Vantis Aegis
//! let aegis = VantisAegis::initialize();
//! 
//! // Check if system appears as Windows
//! assert!(aegis.is_windows_mode());
//! 
//! // Get Windows version
//! let version = aegis.get_windows_version();
//! println!("Appearing as: {}", version);
//! ```

use crate::vantis_aegis_nt_api::{NtApiEmulator, NtVersion};
use crate::vantis_aegis_registry::RegistryEmulator;
use crate::vantis_aegis_syscall::SyscallTranslator;

/// Vantis Aegis main interface
pub struct VantisAegis {
    /// NT API emulator
    nt_api: &'static NtApiEmulator,
    /// Registry emulator
    registry: &'static RegistryEmulator,
    /// Syscall translator
    syscall: &'static SyscallTranslator,
    /// Enabled state
    enabled: bool,
}

/// Vantis Aegis configuration
#[derive(Debug, Clone)]
pub struct AegisConfig {
    /// Enable NT API emulation
    pub enable_nt_api: bool,
    /// Enable registry emulation
    pub enable_registry: bool,
    /// Enable syscall translation
    pub enable_syscall: bool,
    /// Verbose logging
    pub verbose: bool,
}

impl Default for AegisConfig {
    fn default() -> Self {
        Self {
            enable_nt_api: true,
            enable_registry: true,
            enable_syscall: true,
            verbose: false,
        }
    }
}

/// Vantis Aegis statistics
#[derive(Debug, Clone)]
pub struct AegisStats {
    /// Number of NT API calls
    pub nt_api_calls: u64,
    /// Number of registry queries
    pub registry_queries: u64,
    /// Number of syscalls translated
    pub syscalls_translated: u64,
    /// Total time spent (microseconds)
    pub total_time_us: u64,
}

impl VantisAegis {
    /// Initialize Vantis Aegis
    /// 
    /// This initializes all components of the Vantis Aegis system.
    pub fn initialize() -> &'static Self {
        Self::instance()
    }
    
    /// Get the global Vantis Aegis instance
    pub fn instance() -> &'static Self {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<VantisAegis> = OnceLock::new();
        
        INSTANCE.get_or_init(|| {
            VantisAegis {
                nt_api: NtApiEmulator::instance(),
                registry: RegistryEmulator::instance(),
                syscall: SyscallTranslator::instance(),
                enabled: true,
            }
        })
    }
    
    /// Check if Vantis Aegis is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Check if system is in Windows mode
    /// 
    /// Returns true if Vantis Aegis is successfully masquerading as Windows.
    pub fn is_windows_mode(&self) -> bool {
        self.enabled
    }
    
    /// Get Windows version string
    /// 
    /// Returns a human-readable Windows version string.
    pub fn get_windows_version(&self) -> String {
        let version = self.nt_api.get_version();
        format!("Windows {} Build {}", 
                if version.major_version == 10 { "11" } else { "10" },
                version.build_number)
    }
    
    /// Get detailed Windows version information
    pub fn get_version_info(&self) -> NtVersion {
        self.nt_api.get_version()
    }
    
    /// Get NT API emulator
    pub fn nt_api(&self) -> &'static NtApiEmulator {
        self.nt_api
    }
    
    /// Get registry emulator
    pub fn registry(&self) -> &'static RegistryEmulator {
        self.registry
    }
    
    /// Get syscall translator
    pub fn syscall(&self) -> &'static SyscallTranslator {
        self.syscall
    }
    
    /// Perform system health check
    /// 
    /// Verifies that all Vantis Aegis components are functioning correctly.
    pub fn health_check(&self) -> Result<(), String> {
        // Check NT API
        self.nt_api.query_system_basic_information()
            .map_err(|e| format!("NT API check failed: {:?}", e))?;
        
        // Check Registry
        self.registry.query_value(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "ProductName"
        ).map_err(|e| format!("Registry check failed: {:?}", e))?;
        
        Ok(())
    }
    
    /// Get system information summary
    /// 
    /// Returns a summary of the emulated Windows system.
    pub fn get_system_summary(&self) -> String {
        let version = self.get_windows_version();
        let sys_info = self.nt_api.query_system_basic_information().unwrap();
        
        format!(
            "{}\nProcessors: {}\nMemory: {} MB\nPage Size: {} KB",
            version,
            sys_info.number_of_processors,
            (sys_info.number_of_physical_pages as u64 * sys_info.page_size as u64) / (1024 * 1024),
            sys_info.page_size / 1024
        )
    }
    
    /// Validate Windows compatibility
    /// 
    /// Checks if the system appears sufficiently Windows-like.
    pub fn validate_compatibility(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Check version
        let version = self.nt_api.get_version();
        if version.major_version != 10 {
            errors.push("Invalid Windows version".to_string());
        }
        
        // Check registry
        if let Err(e) = self.registry.query_value(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "ProductName"
        ) {
            errors.push(format!("Registry check failed: {:?}", e));
        }
        
        // Check system info
        if let Err(e) = self.nt_api.query_system_basic_information() {
            errors.push(format!("System info check failed: {:?}", e));
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(all(test, feature = "verus"))]
mod tests {
    use super::*;
    
    #[test]
    fn test_vantis_aegis_initialization() {
        let aegis = VantisAegis::initialize();
        assert!(aegis.is_enabled());
    }
    
    #[test]
    fn test_is_windows_mode() {
        let aegis = VantisAegis::instance();
        assert!(aegis.is_windows_mode());
    }
    
    #[test]
    fn test_get_windows_version() {
        let aegis = VantisAegis::instance();
        let version = aegis.get_windows_version();
        assert!(version.contains("Windows"));
        assert!(version.contains("11"));
    }
    
    #[test]
    fn test_get_version_info() {
        let aegis = VantisAegis::instance();
        let version = aegis.get_version_info();
        assert_eq!(version.major_version, 10);
        assert_eq!(version.build_number, 22631);
    }
    
    #[test]
    fn test_health_check() {
        let aegis = VantisAegis::instance();
        let result = aegis.health_check();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_get_system_summary() {
        let aegis = VantisAegis::instance();
        let summary = aegis.get_system_summary();
        assert!(summary.contains("Windows"));
        assert!(summary.contains("Processors"));
        assert!(summary.contains("Memory"));
    }
    
    #[test]
    fn test_validate_compatibility() {
        let aegis = VantisAegis::instance();
        let result = aegis.validate_compatibility();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_nt_api_access() {
        let aegis = VantisAegis::instance();
        let nt_api = aegis.nt_api();
        let version = nt_api.get_version();
        assert_eq!(version.major_version, 10);
    }
    
    #[test]
    fn test_registry_access() {
        let aegis = VantisAegis::instance();
        let registry = aegis.registry();
        let result = registry.query_value(
            "HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion",
            "ProductName"
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_syscall_access() {
        let aegis = VantisAegis::instance();
        let syscall = aegis.syscall();
        let name = syscall.get_syscall_name(0x0018);
        assert_eq!(name, Some("NtQuerySystemInformation"));
    }
}