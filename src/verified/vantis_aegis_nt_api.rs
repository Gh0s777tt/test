//! Vantis Aegis - NT API Emulation Layer
//! 
//! This module provides emulation of Windows NT kernel APIs to make VantisOS
//! appear as Windows to anti-cheat systems and other software that queries
//! the operating system.
//!
//! # Design Philosophy
//! 
//! - Return plausible Windows-like values based on actual system state
//! - Maintain consistency across multiple queries
//! - Minimize overhead and performance impact
//! - Log unknown queries for future implementation
//!
//! # Legal Notice
//! 
//! This implementation uses only publicly documented Windows APIs from
//! official Microsoft documentation. No reverse engineering or proprietary
//! information is used. This is a clean-room implementation for compatibility
//! purposes only.

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// NT API error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NtError {
    /// Invalid parameter
    InvalidParameter,
    /// Buffer too small
    BufferTooSmall,
    /// Access denied
    AccessDenied,
    /// Object not found
    ObjectNotFound,
    /// Not implemented
    NotImplemented,
    /// Internal error
    InternalError(String),
}

/// NT API result type
pub type NtResult<T> = Result<T, NtError>;

/// System basic information structure
/// Corresponds to SYSTEM_BASIC_INFORMATION in Windows
#[derive(Debug, Clone)]
pub struct SystemBasicInformation {
    pub reserved: u32,
    pub timer_resolution: u32,
    pub page_size: u32,
    pub number_of_physical_pages: u32,
    pub lowest_physical_page_number: u32,
    pub highest_physical_page_number: u32,
    pub allocation_granularity: u32,
    pub minimum_user_mode_address: u64,
    pub maximum_user_mode_address: u64,
    pub active_processors_affinity_mask: u64,
    pub number_of_processors: u8,
}

/// System processor information structure
#[derive(Debug, Clone)]
pub struct SystemProcessorInformation {
    pub processor_architecture: u16,
    pub processor_level: u16,
    pub processor_revision: u16,
    pub maximum_processors: u16,
    pub processor_feature_bits: u32,
}

/// System performance information structure
#[derive(Debug, Clone)]
pub struct SystemPerformanceInformation {
    pub idle_process_time: u64,
    pub io_read_transfer_count: u64,
    pub io_write_transfer_count: u64,
    pub io_other_transfer_count: u64,
    pub io_read_operation_count: u32,
    pub io_write_operation_count: u32,
    pub io_other_operation_count: u32,
    pub available_pages: u32,
    pub committed_pages: u32,
    pub commit_limit: u32,
    pub peak_commitment: u32,
}

/// System time information structure
#[derive(Debug, Clone)]
pub struct SystemTimeInformation {
    pub boot_time: u64,
    pub current_time: u64,
    pub time_zone_bias: i64,
    pub time_zone_id: u32,
}

/// Process basic information structure
#[derive(Debug, Clone)]
pub struct ProcessBasicInformation {
    pub exit_status: u32,
    pub peb_base_address: u64,
    pub affinity_mask: u64,
    pub base_priority: i32,
    pub unique_process_id: u64,
    pub inherited_from_unique_process_id: u64,
}

/// Thread basic information structure
#[derive(Debug, Clone)]
pub struct ThreadBasicInformation {
    pub exit_status: u32,
    pub teb_base_address: u64,
    pub client_id_process: u64,
    pub client_id_thread: u64,
    pub affinity_mask: u64,
    pub priority: i32,
    pub base_priority: i32,
}

/// Windows version information
#[derive(Debug, Clone)]
pub struct NtVersion {
    pub major_version: u32,
    pub minor_version: u32,
    pub build_number: u32,
    pub platform_id: u32,
    pub service_pack: String,
}

/// NT API emulation layer
pub struct NtApiEmulator {
    /// Cached system information
    system_info: Arc<Mutex<SystemBasicInformation>>,
    /// Cached processor information
    processor_info: Arc<Mutex<SystemProcessorInformation>>,
    /// Boot time
    boot_time: u64,
    /// Version information
    version: NtVersion,
}

impl NtApiEmulator {
    /// Create a new NT API emulator
    pub fn new() -> Self {
        let boot_time = Self::get_boot_time();
        
        Self {
            system_info: Arc::new(Mutex::new(Self::create_system_basic_info())),
            processor_info: Arc::new(Mutex::new(Self::create_processor_info())),
            boot_time,
            version: Self::create_version_info(),
        }
    }
    
    /// Get the global NT API emulator instance
    pub fn instance() -> &'static NtApiEmulator {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<NtApiEmulator> = OnceLock::new();
        
        INSTANCE.get_or_init(NtApiEmulator::new)
    }
    
    /// Create system basic information
    fn create_system_basic_info() -> SystemBasicInformation {
        let total_memory = Self::get_total_memory();
        let page_size = 4096u32;
        let num_pages = (total_memory / page_size as u64) as u32;
        let num_cpus = Self::get_cpu_count();
        
        SystemBasicInformation {
            reserved: 0,
            timer_resolution: 156250, // 15.625ms (typical Windows)
            page_size,
            number_of_physical_pages: num_pages,
            lowest_physical_page_number: 1,
            highest_physical_page_number: num_pages,
            allocation_granularity: 65536, // 64KB
            minimum_user_mode_address: 0x10000,
            maximum_user_mode_address: 0x7FFFFFFF0000,
            active_processors_affinity_mask: (1u64 << num_cpus) - 1,
            number_of_processors: num_cpus as u8,
        }
    }
    
    /// Create processor information
    fn create_processor_info() -> SystemProcessorInformation {
        SystemProcessorInformation {
            processor_architecture: 9, // PROCESSOR_ARCHITECTURE_AMD64
            processor_level: 6, // Intel/AMD x64
            processor_revision: 0x9E0D, // Typical modern CPU
            maximum_processors: Self::get_cpu_count() as u16,
            processor_feature_bits: 0xFFFFFFFF, // All features supported
        }
    }
    
    /// Create version information (Windows 11)
    fn create_version_info() -> NtVersion {
        NtVersion {
            major_version: 10,
            minor_version: 0,
            build_number: 22631, // Windows 11 22H2
            platform_id: 2, // VER_PLATFORM_WIN32_NT
            service_pack: String::new(),
        }
    }
    
    /// Get total system memory in bytes
    fn get_total_memory() -> u64 {
        // Try to read from /proc/meminfo
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            return kb * 1024; // Convert KB to bytes
                        }
                    }
                }
            }
        }
        
        // Fallback: 16GB
        16 * 1024 * 1024 * 1024
    }
    
    /// Get CPU count
    fn get_cpu_count() -> usize {
        num_cpus::get()
    }
    
    /// Get boot time in Windows FILETIME format
    fn get_boot_time() -> u64 {
        // Try to read from /proc/uptime
        if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
            if let Some(uptime_str) = content.split_whitespace().next() {
                if let Ok(uptime_secs) = uptime_str.parse::<f64>() {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let boot_time = now - uptime_secs as u64;
                    
                    // Convert Unix timestamp to Windows FILETIME
                    // FILETIME is 100-nanosecond intervals since 1601-01-01
                    // Unix epoch is 1970-01-01, which is 11644473600 seconds after 1601-01-01
                    return (boot_time + 11644473600) * 10000000;
                }
            }
        }
        
        // Fallback: current time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        (now + 11644473600) * 10000000
    }
    
    /// Get current time in Windows FILETIME format
    fn get_current_time() -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        (now + 11644473600) * 10000000
    }
    
    // ========================================================================
    // PUBLIC API FUNCTIONS
    // ========================================================================
    
    /// Query system basic information
    /// 
    /// Returns basic system information including memory, processors, and page size.
    pub fn query_system_basic_information(&self) -> NtResult<SystemBasicInformation> {
        let info = self.system_info.lock().unwrap();
        Ok(info.clone())
    }
    
    /// Query system processor information
    /// 
    /// Returns processor architecture and feature information.
    pub fn query_system_processor_information(&self) -> NtResult<SystemProcessorInformation> {
        let info = self.processor_info.lock().unwrap();
        Ok(info.clone())
    }
    
    /// Query system performance information
    /// 
    /// Returns performance metrics including I/O statistics and memory usage.
    pub fn query_system_performance_information(&self) -> NtResult<SystemPerformanceInformation> {
        let total_memory = Self::get_total_memory();
        let page_size = 4096u64;
        let total_pages = (total_memory / page_size) as u32;
        
        // Simulate reasonable values
        Ok(SystemPerformanceInformation {
            idle_process_time: 0,
            io_read_transfer_count: 1024 * 1024 * 1024, // 1GB
            io_write_transfer_count: 512 * 1024 * 1024, // 512MB
            io_other_transfer_count: 256 * 1024 * 1024, // 256MB
            io_read_operation_count: 10000,
            io_write_operation_count: 5000,
            io_other_operation_count: 2500,
            available_pages: (total_pages as f64 * 0.6) as u32, // 60% available
            committed_pages: (total_pages as f64 * 0.3) as u32, // 30% committed
            commit_limit: total_pages,
            peak_commitment: (total_pages as f64 * 0.4) as u32, // 40% peak
        })
    }
    
    /// Query system time information
    /// 
    /// Returns boot time, current time, and timezone information.
    pub fn query_system_time_information(&self) -> NtResult<SystemTimeInformation> {
        Ok(SystemTimeInformation {
            boot_time: self.boot_time,
            current_time: Self::get_current_time(),
            time_zone_bias: 0, // UTC
            time_zone_id: 0, // Unknown
        })
    }
    
    /// Query system kernel debugger information
    /// 
    /// Returns whether a kernel debugger is present.
    pub fn query_system_kernel_debugger_information(&self) -> NtResult<bool> {
        // Always return false (no debugger)
        Ok(false)
    }
    
    /// Query process basic information
    /// 
    /// Returns basic information about a process.
    pub fn query_process_basic_information(&self, pid: u32) -> NtResult<ProcessBasicInformation> {
        // Try to get actual process info from /proc
        let ppid = self.get_parent_pid(pid).unwrap_or(1);
        
        Ok(ProcessBasicInformation {
            exit_status: 0x103, // STATUS_PENDING
            peb_base_address: 0x7FFDF000, // Typical PEB address
            affinity_mask: (1u64 << Self::get_cpu_count()) - 1,
            base_priority: 8, // NORMAL_PRIORITY_CLASS
            unique_process_id: pid as u64,
            inherited_from_unique_process_id: ppid as u64,
        })
    }
    
    /// Query process image name
    /// 
    /// Returns the full path to the process executable.
    pub fn query_process_image_name(&self, pid: u32) -> NtResult<String> {
        // Try to read from /proc/[pid]/exe
        let exe_path = format!("/proc/{}/exe", pid);
        if let Ok(path) = std::fs::read_link(&exe_path) {
            // Convert Linux path to Windows-style path
            let windows_path = self.linux_to_windows_path(path.to_string_lossy().as_ref());
            return Ok(windows_path);
        }
        
        Err(NtError::ObjectNotFound)
    }
    
    /// Query process debug port
    /// 
    /// Returns the debug port handle (0 if not being debugged).
    pub fn query_process_debug_port(&self, _pid: u32) -> NtResult<u64> {
        // Always return 0 (not being debugged)
        Ok(0)
    }
    
    /// Query process WOW64 information
    /// 
    /// Returns whether the process is running under WOW64 (32-bit on 64-bit).
    pub fn query_process_wow64_information(&self, _pid: u32) -> NtResult<bool> {
        // Always return false (native 64-bit)
        Ok(false)
    }
    
    /// Query thread basic information
    /// 
    /// Returns basic information about a thread.
    pub fn query_thread_basic_information(&self, tid: u32) -> NtResult<ThreadBasicInformation> {
        Ok(ThreadBasicInformation {
            exit_status: 0x103, // STATUS_PENDING
            teb_base_address: 0x7FFDE000, // Typical TEB address
            client_id_process: 0, // Unknown
            client_id_thread: tid as u64,
            affinity_mask: (1u64 << Self::get_cpu_count()) - 1,
            priority: 8, // THREAD_PRIORITY_NORMAL
            base_priority: 8,
        })
    }
    
    /// Query thread times
    /// 
    /// Returns CPU time used by a thread.
    pub fn query_thread_times(&self, _tid: u32) -> NtResult<(u64, u64, u64, u64)> {
        // Return (creation_time, exit_time, kernel_time, user_time)
        let current_time = Self::get_current_time();
        Ok((
            current_time - 1000000000, // Created 100 seconds ago
            0, // Not exited
            500000000, // 50 seconds kernel time
            500000000, // 50 seconds user time
        ))
    }
    
    /// Get Windows version information
    /// 
    /// Returns Windows 11 version information.
    pub fn get_version(&self) -> NtVersion {
        self.version.clone()
    }
    
    /// Get Windows build number
    /// 
    /// Returns the Windows build number (22631 for Windows 11 22H2).
    pub fn get_build_number(&self) -> u32 {
        self.version.build_number
    }
    
    /// Get Windows product type
    /// 
    /// Returns 1 for Workstation, 2 for Domain Controller, 3 for Server.
    pub fn get_product_type(&self) -> u32 {
        1 // VER_NT_WORKSTATION
    }
    
    // ========================================================================
    // HELPER FUNCTIONS
    // ========================================================================
    
    /// Get parent process ID from /proc
    fn get_parent_pid(&self, pid: u32) -> Option<u32> {
        let stat_path = format!("/proc/{}/stat", pid);
        if let Ok(content) = std::fs::read_to_string(&stat_path) {
            // Parse /proc/[pid]/stat format
            // Format: pid (comm) state ppid ...
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 4 {
                if let Ok(ppid) = parts[3].parse::<u32>() {
                    return Some(ppid);
                }
            }
        }
        None
    }
    
    /// Convert Linux path to Windows-style path
    fn linux_to_windows_path(&self, linux_path: &str) -> String {
        // Simple conversion: /path/to/file -> C:\path\to\file
        let windows_path = linux_path.replace('/', "\\");
        format!("C:{}", windows_path)
    }
}

impl Default for NtApiEmulator {
    fn default() -> Self {
        Self::new()
    }
}

// Dependency for CPU count
mod num_cpus {
    pub fn get() -> usize {
        // Try to read from /proc/cpuinfo
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            let count = content.lines()
                .filter(|line| line.starts_with("processor"))
                .count();
            if count > 0 {
                return count;
            }
        }
        
        // Fallback: 8 cores
        8
    }
}

#[cfg(all(test, feature = "verus"))]
mod tests {
    use super::*;
    
    #[test]
    fn test_nt_api_emulator_creation() {
        let emulator = NtApiEmulator::new();
        assert_eq!(emulator.version.major_version, 10);
        assert_eq!(emulator.version.minor_version, 0);
    }
    
    #[test]
    fn test_query_system_basic_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_system_basic_information().unwrap();
        
        assert_eq!(info.page_size, 4096);
        assert!(info.number_of_processors > 0);
        assert!(info.number_of_physical_pages > 0);
        assert_eq!(info.allocation_granularity, 65536);
    }
    
    #[test]
    fn test_query_system_processor_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_system_processor_information().unwrap();
        
        assert_eq!(info.processor_architecture, 9); // AMD64
        assert!(info.maximum_processors > 0);
    }
    
    #[test]
    fn test_query_system_performance_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_system_performance_information().unwrap();
        
        assert!(info.available_pages > 0);
        assert!(info.committed_pages > 0);
        assert!(info.commit_limit > 0);
    }
    
    #[test]
    fn test_query_system_time_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_system_time_information().unwrap();
        
        assert!(info.boot_time > 0);
        assert!(info.current_time > info.boot_time);
    }
    
    #[test]
    fn test_query_system_kernel_debugger_information() {
        let emulator = NtApiEmulator::new();
        let has_debugger = emulator.query_system_kernel_debugger_information().unwrap();
        
        assert!(!has_debugger);
    }
    
    #[test]
    fn test_query_process_basic_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_process_basic_information(1).unwrap();
        
        assert_eq!(info.unique_process_id, 1);
        assert!(info.affinity_mask > 0);
        assert_eq!(info.base_priority, 8);
    }
    
    #[test]
    fn test_query_process_debug_port() {
        let emulator = NtApiEmulator::new();
        let debug_port = emulator.query_process_debug_port(1).unwrap();
        
        assert_eq!(debug_port, 0);
    }
    
    #[test]
    fn test_query_process_wow64_information() {
        let emulator = NtApiEmulator::new();
        let is_wow64 = emulator.query_process_wow64_information(1).unwrap();
        
        assert!(!is_wow64);
    }
    
    #[test]
    fn test_query_thread_basic_information() {
        let emulator = NtApiEmulator::new();
        let info = emulator.query_thread_basic_information(1).unwrap();
        
        assert_eq!(info.client_id_thread, 1);
        assert!(info.affinity_mask > 0);
        assert_eq!(info.priority, 8);
    }
    
    #[test]
    fn test_get_version() {
        let emulator = NtApiEmulator::new();
        let version = emulator.get_version();
        
        assert_eq!(version.major_version, 10);
        assert_eq!(version.minor_version, 0);
        assert_eq!(version.build_number, 22631);
        assert_eq!(version.platform_id, 2);
    }
    
    #[test]
    fn test_get_build_number() {
        let emulator = NtApiEmulator::new();
        let build = emulator.get_build_number();
        
        assert_eq!(build, 22631);
    }
    
    #[test]
    fn test_get_product_type() {
        let emulator = NtApiEmulator::new();
        let product_type = emulator.get_product_type();
        
        assert_eq!(product_type, 1); // Workstation
    }
    
    #[test]
    fn test_linux_to_windows_path() {
        let emulator = NtApiEmulator::new();
        let windows_path = emulator.linux_to_windows_path("/usr/bin/bash");
        
        assert_eq!(windows_path, r"C:\usr\bin\bash");
    }
}