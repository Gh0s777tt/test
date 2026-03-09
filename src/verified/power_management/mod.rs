//! Power Management Module
//! 
//! This module provides comprehensive power management for VantisOS,
//! including advanced CPU frequency scaling, power profiles, and battery optimization.

mod cpu;
mod gpu;
mod screen;
mod battery;
mod profile;
mod scheduler;

pub use cpu::CpuPowerManager;
pub use gpu::GpuPowerManager;
pub use screen::ScreenPowerManager;
pub use battery::BatteryManager;
pub use profile::{PowerProfile, PowerProfileManager};
pub use scheduler::PowerScheduler;

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

/// Power management manager
#[derive(Debug)]
pub struct PowerManager {
    state: Arc<AtomicU32>,
    cpu: CpuPowerManager,
    gpu: GpuPowerManager,
    screen: ScreenPowerManager,
    battery: BatteryManager,
    profile: PowerProfileManager,
    scheduler: PowerScheduler,
}

impl PowerManager {
    pub fn new() -> Result<Self, PowerError> {
        Ok(Self {
            state: Arc::new(AtomicU32::new(0)),
            cpu: CpuPowerManager::new()?,
            gpu: GpuPowerManager::new()?,
            screen: ScreenPowerManager::new()?,
            battery: BatteryManager::new()?,
            profile: PowerProfileManager::new()?,
            scheduler: PowerScheduler::new()?,
        })
    }

    pub fn initialize(&mut self) -> Result<(), PowerError> {
        self.cpu.initialize()?;
        self.gpu.initialize()?;
        self.screen.initialize()?;
        self.battery.initialize()?;
        self.profile.initialize()?;
        self.scheduler.initialize()?;
        
        self.state.store(1, Ordering::SeqCst);
        Ok(())
    }

    pub fn set_profile(&mut self, profile: PowerProfile) -> Result<(), PowerError> {
        self.profile.set_profile(profile)?;
        
        // Apply profile settings
        match profile {
            PowerProfile::Performance => {
                self.cpu.set_governor(CpuGovernor::Performance)?;
                self.gpu.set_performance_mode(GpuPerformance::Max)?;
                self.screen.set_brightness(100)?;
            }
            PowerProfile::Balanced => {
                self.cpu.set_governor(CpuGovernor::OnDemand)?;
                self.gpu.set_performance_mode(GpuPerformance::Balanced)?;
                self.screen.set_brightness(50)?;
            }
            PowerProfile::PowerSaver => {
                self.cpu.set_governor(CpuGovernor::PowerSave)?;
                self.gpu.set_performance_mode(GpuPerformance::Min)?;
                self.screen.set_brightness(30)?;
            }
        }
        
        Ok(())
    }

    pub fn get_profile(&self) -> PowerProfile {
        self.profile.get_profile()
    }

    pub fn get_battery_info(&self) -> &BatteryInfo {
        self.battery.get_info()
    }

    pub fn suspend(&mut self) -> Result<(), PowerError> {
        self.scheduler.suspend()
    }

    pub fn hibernate(&mut self) -> Result<(), PowerError> {
        self.scheduler.hibernate()
    }
}

impl Default for PowerManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to create PowerManager"))
    }
}

/// Power errors
#[derive(Debug, thiserror::Error)]
pub enum PowerError {
    #[error("CPU power error: {0}")]
    CpuError(String),
    #[error("GPU power error: {0}")]
    GpuError(String),
    #[error("Screen power error: {0}")]
    ScreenError(String),
    #[error("Battery error: {0}")]
    BatteryError(String),
    #[error("Profile error: {0}")]
    ProfileError(String),
    #[error("Scheduler error: {0}")]
    SchedulerError(String),
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Operation not supported")]
    NotSupported,
}

/// CPU governor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuGovernor {
    Performance,
    OnDemand,
    Conservative,
    PowerSave,
    Userspace,
    Schedutil,
}

/// GPU performance mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuPerformance {
    Min,
    Balanced,
    Max,
}

/// Battery information
#[derive(Debug, Clone)]
pub struct BatteryInfo {
    pub capacity: u32,        // mAh
    pub voltage: u32,         // mV
    pub current: i32,         // mA (positive = charging, negative = discharging)
    pub temperature: i32,     // Celsius
    pub cycle_count: u32,
    pub health: u32,          // Percentage
    pub charge_level: u32,    // Percentage
    pub status: BatteryStatus,
}

/// Battery status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    Unknown,
}
