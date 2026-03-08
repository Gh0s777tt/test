//! Mobile Support Module
//! 
//! This module provides comprehensive mobile support capabilities for
//! VantisOS including iOS, Android, mobile UI framework, touch optimization,
//! battery optimization, and mobile security.

pub mod ios;
pub mod android;
pub mod ui;
pub mod touch;
pub mod battery;
pub mod security;

use alloc::sync::Arc;
use spin::Mutex;

/// Mobile platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobilePlatform {
    Ios,
    Android,
    CrossPlatform,
}

/// Mobile device type
#[derive(Debug, Clone, Copy)]
pub enum MobileDeviceType {
    Phone,
    Tablet,
    Phablet,
    Foldable,
    Wearable,
}

/// Mobile manager that coordinates all mobile features
pub struct MobileManager {
    platform: Arc<Mutex<MobilePlatform>>,
    device_type: Arc<Mutex<MobileDeviceType>>,
    initialized: Arc<Mutex<bool>>,
}

impl MobileManager {
    /// Create a new mobile manager
    pub fn new() -> Self {
        Self {
            platform: Arc::new(Mutex::new(MobilePlatform::CrossPlatform)),
            device_type: Arc::new(Mutex::new(MobileDeviceType::Phone)),
            initialized: Arc::new(Mutex::new(false)),
        }
    }

    /// Initialize the mobile manager
    pub fn init(&self, platform: MobilePlatform, device_type: MobileDeviceType) -> Result<(), MobileError> {
        *self.platform.lock() = platform;
        *self.device_type.lock() = device_type;
        *self.initialized.lock() = true;
        Ok(())
    }

    /// Get the current platform
    pub fn platform(&self) -> MobilePlatform {
        *self.platform.lock()
    }

    /// Get the device type
    pub fn device_type(&self) -> MobileDeviceType {
        *self.device_type.lock()
    }

    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        *self.initialized.lock()
    }
}

impl Default for MobileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Mobile error types
#[derive(Debug, Clone, Copy)]
pub enum MobileError {
    NotSupported,
    NotInitialized,
    InitializationFailed,
    HardwareNotAvailable,
}

impl core::fmt::Display for MobileError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotSupported => write!(f, "Feature not supported on this platform"),
            Self::NotInitialized => write!(f, "Mobile manager not initialized"),
            Self::InitializationFailed => write!(f, "Mobile manager initialization failed"),
            Self::HardwareNotAvailable => write!(f, "Hardware not available"),
        }
    }
}

/// Global mobile manager instance
static MOBILE_MANAGER: spin::Once<MobileManager> = spin::Once::new();

/// Get the global mobile manager
pub fn mobile_manager() -> &'static MobileManager {
    MOBILE_MANAGER.call_once(|| MobileManager::new())
}

/// Initialize the mobile subsystem
pub fn init(platform: MobilePlatform, device_type: MobileDeviceType) -> Result<(), MobileError> {
    mobile_manager().init(platform, device_type)
}

/// Get the current mobile platform
pub fn get_platform() -> MobilePlatform {
    mobile_manager().platform()
}

/// Get the device type
pub fn get_device_type() -> MobileDeviceType {
    mobile_manager().device_type()
}