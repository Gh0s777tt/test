//! Legacy Integration Module
//! 
//! This module provides legacy system integration capabilities for VantisOS
//! including Windows compatibility, Linux compatibility, POSIX compatibility,
//! legacy API support, and migration tools.

pub mod windows;
pub mod linux;
pub mod posix;
pub mod api;
pub mod migration;

use alloc::sync::Arc;
use spin::Mutex;

/// Legacy system type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacySystemType {
    Windows,
    Linux,
    MacOS,
    Unix,
    VMS,
    Mainframe,
}

/// Compatibility level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityLevel {
    None,
    Partial,
    Full,
}

/// Legacy manager that coordinates all legacy integration features
pub struct LegacyManager {
    windows_compatibility: Arc<Mutex<CompatibilityLevel>>,
    linux_compatibility: Arc<Mutex<CompatibilityLevel>>,
    posix_compatibility: Arc<Mutex<CompatibilityLevel>>,
}

impl LegacyManager {
    /// Create a new legacy manager
    pub fn new() -> Self {
        Self {
            windows_compatibility: Arc::new(Mutex::new(CompatibilityLevel::None)),
            linux_compatibility: Arc::new(Mutex::new(CompatibilityLevel::None)),
            posix_compatibility: Arc::new(Mutex::new(CompatibilityLevel::None)),
        }
    }

    /// Set Windows compatibility level
    pub fn set_windows_compatibility(&self, level: CompatibilityLevel) {
        *self.windows_compatibility.lock() = level;
    }

    /// Set Linux compatibility level
    pub fn set_linux_compatibility(&self, level: CompatibilityLevel) {
        *self.linux_compatibility.lock() = level;
    }

    /// Set POSIX compatibility level
    pub fn set_posix_compatibility(&self, level: CompatibilityLevel) {
        *self.posix_compatibility.lock() = level;
    }

    /// Get Windows compatibility level
    pub fn windows_compatibility(&self) -> CompatibilityLevel {
        *self.windows_compatibility.lock()
    }

    /// Get Linux compatibility level
    pub fn linux_compatibility(&self) -> CompatibilityLevel {
        *self.linux_compatibility.lock()
    }

    /// Get POSIX compatibility level
    pub fn posix_compatibility(&self) -> CompatibilityLevel {
        *self.posix_compatibility.lock()
    }
}

impl Default for LegacyManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global legacy manager instance
static LEGACY_MANAGER: spin::Once<LegacyManager> = spin::Once::new();

/// Get the global legacy manager
pub fn legacy_manager() -> &'static LegacyManager {
    LEGACY_MANAGER.call_once(|| LegacyManager::new())
}

/// Set Windows compatibility level
pub fn set_windows_compatibility(level: CompatibilityLevel) {
    legacy_manager().set_windows_compatibility(level);
}

/// Set Linux compatibility level
pub fn set_linux_compatibility(level: CompatibilityLevel) {
    legacy_manager().set_linux_compatibility(level);
}

/// Set POSIX compatibility level
pub fn set_posix_compatibility(level: CompatibilityLevel) {
    legacy_manager().set_posix_compatibility(level);
}