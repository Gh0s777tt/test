//! IoT Sensors Module
//! 
//! This module provides sensor drivers for IoT devices including temperature,
//! humidity, pressure, motion, and light sensors.

pub mod temperature;
pub mod humidity;
pub mod pressure;
pub mod motion;
pub mod light;

pub use temperature::*;
pub use humidity::*;
pub use pressure::*;
pub use motion::*;
pub use light::*;

/// Initialize all sensors
pub fn init() {
    // Initialize temperature sensors
    temperature::init();
    
    // Initialize humidity sensors
    humidity::init();
    
    // Initialize pressure sensors
    pressure::init();
    
    // Initialize motion sensors
    motion::init();
    
    // Initialize light sensors
    light::init();
}

/// Get sensors information
pub fn get_info() -> &'static str {
    "IoT Sensors v0.7.0"
}