//! iOS Support Module
//! 
//! This module provides iOS-specific support for VantisOS including
//! platform integration, notifications, and iOS-specific features.

use alloc::string::String;

/// iOS version
#[derive(Debug, Clone)]
pub struct IosVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl IosVersion {
    /// Create a new iOS version
    pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Get version as string
    pub fn as_string(&self) -> String {
        alloc::format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// iOS device model
#[derive(Debug, Clone, Copy)]
pub enum IosDeviceModel {
    IPhone6,
    IPhone6S,
    IPhone7,
    IPhone8,
    IPhoneX,
    IPhoneXS,
    IPhoneXR,
    IPhone11,
    IPhone12,
    IPhone13,
    IPhone14,
    IPhone15,
    IPhoneSE,
    IPadAir,
    IPadPro,
    IPadMini,
    IPodTouch,
}

/// iOS notification type
#[derive(Debug, Clone, Copy)]
pub enum IosNotificationType {
    Alert,
    Badge,
    Sound,
}

/// iOS permission
#[derive(Debug, Clone, Copy)]
pub enum IosPermission {
    Camera,
    Microphone,
    Location,
    Notifications,
    Contacts,
    Photos,
    Calendar,
    Reminders,
}

/// iOS support manager
pub struct IosSupportManager {
    version: IosVersion,
    device_model: IosDeviceModel,
    granted_permissions: alloc::vec::Vec<IosPermission>,
}

impl IosSupportManager {
    /// Create a new iOS support manager
    pub fn new(version: IosVersion, device_model: IosDeviceModel) -> Self {
        Self {
            version,
            device_model,
            granted_permissions: Vec::new(),
        }
    }

    /// Request a permission
    pub fn request_permission(&mut self, permission: IosPermission) -> Result<bool, IosError> {
        // In a real implementation, this would request the permission from iOS
        self.granted_permissions.push(permission);
        Ok(true)
    }

    /// Check if a permission is granted
    pub fn has_permission(&self, permission: IosPermission) -> bool {
        self.granted_permissions.contains(&permission)
    }

    /// Send a notification
    pub fn send_notification(
        &self,
        title: impl Into<String>,
        body: impl Into<String>,
        notification_types: alloc::vec::Vec<IosNotificationType>,
    ) -> Result<(), IosError> {
        // In a real implementation, this would send a notification via iOS
        Ok(())
    }

    /// Get iOS version
    pub fn version(&self) -> &IosVersion {
        &self.version
    }

    /// Get device model
    pub fn device_model(&self) -> IosDeviceModel {
        self.device_model
    }

    /// Check if device is iPhone
    pub fn is_iphone(&self) -> bool {
        matches!(self.device_model, 
            IosDeviceModel::IPhone6 | IosDeviceModel::IPhone6S | 
            IosDeviceModel::IPhone7 | IosDeviceModel::IPhone8 |
            IosDeviceModel::IPhoneX | IosDeviceModel::IPhoneXS |
            IosDeviceModel::IPhoneXR | IosDeviceModel::IPhone11 |
            IosDeviceModel::IPhone12 | IosDeviceModel::IPhone13 |
            IosDeviceModel::IPhone14 | IosDeviceModel::IPhone15 |
            IosDeviceModel::IPhoneSE)
    }

    /// Check if device is iPad
    pub fn is_ipad(&self) -> bool {
        matches!(self.device_model,
            IosDeviceModel::IPadAir | IosDeviceModel::IPadPro | IosDeviceModel::IPadMini)
    }

    /// Get screen dimensions
    pub fn screen_dimensions(&self) -> (u32, u32) {
        match self.device_model {
            IosDeviceModel::IPhone6 | IosDeviceModel::IPhone6S => (750, 1334),
            IosDeviceModel::IPhone7 | IosDeviceModel::IPhone8 => (750, 1334),
            IosDeviceModel::IPhoneX | IosDeviceModel::IPhoneXS => (1125, 2436),
            IosDeviceModel::IPhoneXR => (828, 1792),
            IosDeviceModel::IPhone11 => (828, 1792),
            IosDeviceModel::IPhone12 => (1170, 2532),
            IosDeviceModel::IPhone13 => (1170, 2532),
            IosDeviceModel::IPhone14 => (1170, 2532),
            IosDeviceModel::IPhone15 => (1179, 2556),
            IosDeviceModel::IPhoneSE => (750, 1334),
            IosDeviceModel::IPadAir => (1640, 2360),
            IosDeviceModel::IPadPro => (2048, 2732),
            IosDeviceModel::IPadMini => (1536, 2048),
            IosDeviceModel::IPodTouch => (750, 1334),
        }
    }
}

/// iOS error types
#[derive(Debug, Clone, Copy)]
pub enum IosError {
    PermissionDenied,
    FeatureNotSupported,
    DeviceNotSupported,
    SystemError,
}

impl core::fmt::Display for IosError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::FeatureNotSupported => write!(f, "Feature not supported"),
            Self::DeviceNotSupported => write!(f, "Device not supported"),
            Self::SystemError => write!(f, "System error"),
        }
    }
}

/// Global iOS support manager
static IOS_SUPPORT_MANAGER: spin::Once<IosSupportManager> = spin::Once::new();

/// Initialize iOS support
pub fn init_ios_support(version: IosVersion, device_model: IosDeviceModel) {
    IOS_SUPPORT_MANAGER.call_once(|| IosSupportManager::new(version, device_model));
}

/// Get the iOS support manager
pub fn ios_support_manager() -> &'static IosSupportManager {
    IOS_SUPPORT_MANAGER.call_once(|| IosSupportManager::new(
        IosVersion::new(17, 0, 0),
        IosDeviceModel::IPhone15,
    ))
}

/// Request an iOS permission
pub fn request_ios_permission(permission: IosPermission) -> Result<bool, IosError> {
    // Note: This would need mutable access in a real implementation
    Ok(true)
}

/// Send an iOS notification
pub fn send_ios_notification(
    title: impl Into<String>,
    body: impl Into<String>,
) -> Result<(), IosError> {
    ios_support_manager().send_notification(title, body, alloc::vec![IosNotificationType::Alert])
}

/// Get iOS screen dimensions
pub fn get_ios_screen_dimensions() -> (u32, u32) {
    ios_support_manager().screen_dimensions()
}