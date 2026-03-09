//! IoT Device Drivers Module
//! 
//! This module provides device drivers for IoT devices including sensors,
//! GPIO, I2C, SPI, UART, PWM, and ADC.

pub mod sensors;
pub mod gpio;
pub mod i2c;
pub mod spi;
pub mod uart;
pub mod pwm;
pub mod adc;

pub use sensors::*;
pub use gpio::*;
pub use i2c::*;
pub use spi::*;
pub use uart::*;
pub use pwm::*;
pub use adc::*;

/// Initialize all IoT drivers
pub fn init() {
    // Initialize GPIO
    gpio::init();
    
    // Initialize I2C
    i2c::init();
    
    // Initialize SPI
    spi::init();
    
    // Initialize UART
    uart::init();
    
    // Initialize PWM
    pwm::init();
    
    // Initialize ADC
    adc::init();
    
    // Initialize sensors
    sensors::init();
}

/// Get IoT drivers information
pub fn get_info() -> &'static str {
    "IoT Device Drivers v0.7.0"
}