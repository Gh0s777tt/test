//! Crash Recovery Module
//! 
//! This module provides crash detection and recovery mechanisms
//! including crash dumps, stack trace collection, and recovery procedures.

use alloc::sync::Arc;
use spin::Mutex;

/// Crash type
#[derive(Debug, Clone, Copy)]
pub enum CrashType {
    Panic,
    KernelPanic,
    AssertionFailure,
    MemoryAccessViolation,
    IllegalInstruction,
    Unknown,
}

/// Crash severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrashSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Crash information
#[derive(Debug, Clone)]
pub struct CrashInfo {
    pub crash_type: CrashType,
    pub severity: CrashSeverity,
    pub thread_id: usize,
    pub instruction_pointer: usize,
    pub stack_pointer: usize,
    pub stack_trace: Vec<usize>,
    pub registers: [usize; 16],
    pub timestamp: u64,
    pub message: Option<alloc::string::String>,
}

/// Crash dump
#[derive(Debug, Clone)]
pub struct CrashDump {
    pub crash_info: CrashInfo,
    pub memory_regions: Vec<MemoryRegion>,
    pub open_files: Vec<FileInfo>,
    pub network_connections: Vec<ConnectionInfo>,
}

/// Memory region snapshot
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start_address: usize,
    pub size: usize,
    pub permissions: u8,
    pub data: Vec<u8>,
}

/// File information
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub file_descriptor: usize,
    pub path: Option<alloc::string::String>,
    pub mode: u32,
    pub offset: usize,
}

/// Network connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub local_address: [u8; 4],
    pub local_port: u16,
    pub remote_address: [u8; 4],
    pub remote_port: u16,
    pub state: ConnectionState,
}

/// Connection state
#[derive(Debug, Clone, Copy)]
pub enum ConnectionState {
    Established,
    Listening,
    Closed,
    Unknown,
}

/// Crash recovery action
#[derive(Debug, Clone, Copy)]
pub enum RecoveryAction {
    RestartThread,
    RestartProcess,
    RebootSystem,
    None,
}

/// Crash recovery manager
pub struct CrashRecoveryManager {
    crash_history: Arc<Mutex<Vec<CrashInfo>>>,
    recovery_enabled: Arc<Mutex<bool>>,
    auto_recovery_enabled: Arc<Mutex<bool>>,
}

impl CrashRecoveryManager {
    /// Create a new crash recovery manager
    pub fn new() -> Self {
        Self {
            crash_history: Arc::new(Mutex::new(Vec::new())),
            recovery_enabled: Arc::new(Mutex::new(true)),
            auto_recovery_enabled: Arc::new(Mutex::new(false)),
        }
    }

    /// Enable or disable crash recovery
    pub fn set_recovery_enabled(&self, enabled: bool) {
        *self.recovery_enabled.lock() = enabled;
    }

    /// Enable or disable auto recovery
    pub fn set_auto_recovery_enabled(&self, enabled: bool) {
        *self.auto_recovery_enabled.lock() = enabled;
    }

    /// Handle a crash
    pub fn handle_crash(&self, crash_info: CrashInfo) -> RecoveryAction {
        let mut history = self.crash_history.lock();
        history.push(crash_info.clone());
        
        // Determine recovery action based on crash severity
        let action = if *self.auto_recovery_enabled.lock() {
            match crash_info.severity {
                CrashSeverity::Low => RecoveryAction::RestartThread,
                CrashSeverity::Medium => RecoveryAction::RestartProcess,
                CrashSeverity::High => RecoveryAction::RestartProcess,
                CrashSeverity::Critical => RecoveryAction::RebootSystem,
            }
        } else {
            RecoveryAction::None
        };
        
        action
    }

    /// Create a crash dump
    pub fn create_crash_dump(&self, crash_info: &CrashInfo) -> CrashDump {
        // In a real implementation, this would collect:
        // 1. Stack trace
        // 2. Memory regions
        // 3. Register values
        // 4. Open files
        // 5. Network connections
        // 6. Thread states
        
        CrashDump {
            crash_info: crash_info.clone(),
            memory_regions: Vec::new(),
            open_files: Vec::new(),
            network_connections: Vec::new(),
        }
    }

    /// Get crash history
    pub fn crash_history(&self) -> Vec<CrashInfo> {
        self.crash_history.lock().clone()
    }

    /// Get crash statistics
    pub fn statistics(&self) -> CrashStatistics {
        let history = self.crash_history.lock();
        
        let mut low = 0;
        let mut medium = 0;
        let mut high = 0;
        let mut critical = 0;
        
        for crash in history.iter() {
            match crash.severity {
                CrashSeverity::Low => low += 1,
                CrashSeverity::Medium => medium += 1,
                CrashSeverity::High => high += 1,
                CrashSeverity::Critical => critical += 1,
            }
        }
        
        CrashStatistics {
            total_crashes: history.len(),
            low_severity: low,
            medium_severity: medium,
            high_severity: high,
            critical_severity: critical,
        }
    }

    /// Clear crash history
    pub fn clear_history(&self) {
        self.crash_history.lock().clear();
    }
}

impl Default for CrashRecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Crash statistics
#[derive(Debug, Clone, Copy)]
pub struct CrashStatistics {
    pub total_crashes: usize,
    pub low_severity: usize,
    pub medium_severity: usize,
    pub high_severity: usize,
    pub critical_severity: usize,
}

/// Crash handler configuration
#[derive(Debug, Clone)]
pub struct CrashHandlerConfig {
    pub create_dump: bool,
    pub save_history: bool,
    pub auto_reboot_on_critical: bool,
    pub max_history_size: usize,
}

impl Default for CrashHandlerConfig {
    fn default() -> Self {
        Self {
            create_dump: true,
            save_history: true,
            auto_reboot_on_critical: false,
            max_history_size: 1000,
        }
    }
}

/// Global crash recovery manager
static CRASH_RECOVERY_MANAGER: spin::Once<CrashRecoveryManager> = spin::Once::new();

/// Get the global crash recovery manager
pub fn crash_recovery_manager() -> &'static CrashRecoveryManager {
    CRASH_RECOVERY_MANAGER.call_once(|| CrashRecoveryManager::new())
}

/// Handle a crash
pub fn handle_crash(crash_info: CrashInfo) -> RecoveryAction {
    crash_recovery_manager().handle_crash(crash_info)
}

/// Create a crash dump
pub fn create_crash_dump(crash_info: &CrashInfo) -> CrashDump {
    crash_recovery_manager().create_crash_dump(crash_info)
}

/// Get crash statistics
pub fn crash_statistics() -> CrashStatistics {
    crash_recovery_manager().statistics()
}