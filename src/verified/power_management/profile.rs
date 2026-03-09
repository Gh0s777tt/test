//! Power Profile Management
//! 
//! This module provides power profile management.

use crate::verified::power_management::PowerError;

/// Power profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

impl PowerProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            PowerProfile::Performance => "Performance",
            PowerProfile::Balanced => "Balanced",
            PowerProfile::PowerSaver => "Power Saver",
        }
    }
}

impl Default for PowerProfile {
    fn default() -> Self {
        PowerProfile::Balanced
    }
}

/// Power profile manager
#[derive(Debug)]
pub struct PowerProfileManager {
    profile: PowerProfile,
    auto_switch: bool,
}

impl PowerProfileManager {
    pub fn new() -> Result<Self, PowerError> {
        Ok(Self {
            profile: PowerProfile::Balanced,
            auto_switch: true,
        })
    }

    pub fn initialize(&mut self) -> Result<(), PowerError> {
        Ok(())
    }

    pub fn set_profile(&mut self, profile: PowerProfile) -> Result<(), PowerError> {
        self.profile = profile;
        Ok(())
    }

    pub fn get_profile(&self) -> PowerProfile {
        self.profile
    }

    pub fn set_auto_switch(&mut self, enabled: bool) {
        self.auto_switch = enabled;
    }

    pub fn is_auto_switch(&self) -> bool {
        self.auto_switch
    }

    pub fn auto_detect_profile(&self, battery_level: u32, is_charging: bool) -> PowerProfile {
        if is_charging {
            if battery_level >= 80 {
                PowerProfile::Performance
            } else {
                PowerProfile::Balanced
            }
        } else {
            if battery_level <= 20 {
                PowerProfile::PowerSaver
            } else if battery_level <= 50 {
                PowerProfile::Balanced
            } else {
                PowerProfile::Balanced
            }
        }
    }
}

impl Default for PowerProfileManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to create PowerProfileManager"))
    }
}
