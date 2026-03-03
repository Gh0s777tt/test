//! Power Management Module
//! 
//! This module provides power management functionality for VantisOS including
//! power saving modes, power monitoring, dynamic frequency scaling, and wake-up mechanisms.

pub mod management;
pub mod modes;
pub mod monitoring;
pub mod frequency;
pub mod wakeup;

pub use management::*;
pub use modes::*;
pub use monitoring::*;
pub use frequency::*;
pub use wakeup::*;

/// Initialize power management
pub fn init() {
    // Initialize power management
    management::init();
    
    // Initialize power monitoring
    monitoring::init();
    
    // Initialize frequency scaling
    frequency::init();
    
    // Initialize wake-up mechanisms
    wakeup::init();
}

/// Get power management information
pub fn get_info() -> &'static str {
    "Power Management v0.7.0"
}