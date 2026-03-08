//! Light Sensor Driver
//! 
//! This module provides light sensor driver support for IoT devices.

use core::sync::atomic::{AtomicU32, Ordering};

/// Light sensor error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightError {
    SensorNotFound,
    ReadFailed,
    InvalidValue,
    Timeout,
}

/// Light sensor type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightSensorType {
    Photoresistor,
    BH1750,
    TSL2561,
    VEML6070,
    MAX44009,
    Custom,
}

/// Light sensor configuration
#[derive(Debug, Clone, Copy)]
pub struct LightSensorConfig {
    pub sensor_type: LightSensorType,
    pub i2c_address: Option<u8>,
    pub pin: Option<u8>,
    pub update_interval_ms: u32,
}

/// Light sensor
pub struct LightSensor {
    pub id: u8,
    config: LightSensorConfig,
    last_value: f32,
    last_update: u64,
}

impl LightSensor {
    /// Create a new light sensor
    pub fn new(id: u8, config: LightSensorConfig) -> Self {
        Self {
            id,
            config,
            last_value: 0.0,
            last_update: 0,
        }
    }
    
    /// Initialize the light sensor
    pub fn init(&mut self) {
        // Implementation depends on sensor type
        // This is a placeholder for hardware-specific code
    }
    
    /// Read light intensity in lux
    pub fn read_lux(&mut self) -> Result<f32, LightError> {
        // Implementation depends on sensor type
        // This is a placeholder for hardware-specific code
        Ok(100.0)
    }
    
    /// Read light intensity as percentage (0-100)
    pub fn read_percentage(&mut self) -> Result<f32, LightError> {
        let lux = self.read_lux()?;
        // Assuming max lux is 1000 for percentage calculation
        Ok((lux / 1000.0) * 100.0)
    }
    
    /// Read raw ADC value
    pub fn read_raw(&mut self) -> Result<u16, LightError> {
        // Implementation depends on sensor type
        // This is a placeholder for hardware-specific code
        Ok(0)
    }
    
    /// Get last read value
    pub fn get_last_value(&self) -> f32 {
        self.last_value
    }
    
    /// Get last update time
    pub fn get_last_update(&self) -> u64 {
        self.last_update
    }
    
    /// Check if update is needed
    pub fn needs_update(&self, current_time: u64) -> bool {
        current_time - self.last_update >= self.config.update_interval_ms as u64
    }
    
    /// Update sensor reading
    pub fn update(&mut self, current_time: u64) -> Result<(), LightError> {
        if self.needs_update(current_time) {
            self.last_value = self.read_lux()?;
            self.last_update = current_time;
        }
        Ok(())
    }
}

/// Light sensor driver state
static LIGHT_INITIALIZED: AtomicU32 = AtomicU32::new(0);

/// Initialize light sensor driver
pub fn init() {
    if LIGHT_INITIALIZED.load(Ordering::SeqCst) == 0 {
        // Initialize hardware-specific light sensors
        // This is a placeholder for hardware-specific code
        
        LIGHT_INITIALIZED.store(1, Ordering::SeqCst);
    }
}

/// Check if light sensor driver is initialized
pub fn is_initialized() -> bool {
    LIGHT_INITIALIZED.load(Ordering::SeqCst) != 0
}

/// Get light sensor driver version
pub fn get_version() -> &'static str {
    "Light Sensor Driver v0.7.0"
}

/// Default light sensor configuration
impl Default for LightSensorConfig {
    fn default() -> Self {
        Self {
            sensor_type: LightSensorType::Custom,
            i2c_address: None,
            pin: None,
            update_interval_ms: 500,
        }
    }
}