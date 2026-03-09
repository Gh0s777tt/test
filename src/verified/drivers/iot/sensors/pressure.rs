//! Pressure Sensor Driver
//! 
//! This module provides pressure sensor driver support for IoT devices.

use core::sync::atomic::{AtomicU32, Ordering};

/// Pressure sensor error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureError {
    SensorNotFound,
    ReadFailed,
    InvalidValue,
    Timeout,
}

/// Pressure sensor type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressureSensorType {
    BMP180,
    BMP280,
    BME280,
    MPL3115A2,
    MS5611,
    Custom,
}

/// Pressure sensor configuration
#[derive(Debug, Clone, Copy)]
pub struct PressureSensorConfig {
    pub sensor_type: PressureSensorType,
    pub i2c_address: Option<u8>,
    pub update_interval_ms: u32,
}

/// Pressure sensor
pub struct PressureSensor {
    pub id: u8,
    config: PressureSensorConfig,
    last_value: f32,
    last_update: u64,
}

impl PressureSensor {
    /// Create a new pressure sensor
    pub fn new(id: u8, config: PressureSensorConfig) -> Self {
        Self {
            id,
            config,
            last_value: 0.0,
            last_update: 0,
        }
    }
    
    /// Initialize the pressure sensor
    pub fn init(&mut self) {
        // Implementation depends on sensor type
        // This is a placeholder for hardware-specific code
    }
    
    /// Read pressure in Pascals (Pa)
    pub fn read_pascals(&mut self) -> Result<f32, PressureError> {
        // Implementation depends on sensor type
        // This is a placeholder for hardware-specific code
        Ok(101325.0)
    }
    
    /// Read pressure in hectopascals (hPa)
    pub fn read_hectopascals(&mut self) -> Result<f32, PressureError> {
        let pascals = self.read_pascals()?;
        Ok(pascals / 100.0)
    }
    
    /// Read pressure in millibars (mbar)
    pub fn read_millibars(&mut self) -> Result<f32, PressureError> {
        self.read_hectopascals()
    }
    
    /// Read pressure in atmospheres (atm)
    pub fn read_atmospheres(&mut self) -> Result<f32, PressureError> {
        let pascals = self.read_pascals()?;
        Ok(pascals / 101325.0)
    }
    
    /// Read pressure in mmHg
    pub fn read_mmhg(&mut self) -> Result<f32, PressureError> {
        let pascals = self.read_pascals()?;
        Ok(pascals * 0.00750062)
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
    pub fn update(&mut self, current_time: u64) -> Result<(), PressureError> {
        if self.needs_update(current_time) {
            self.last_value = self.read_pascals()?;
            self.last_update = current_time;
        }
        Ok(())
    }
}

/// Pressure sensor driver state
static PRESSURE_INITIALIZED: AtomicU32 = AtomicU32::new(0);

/// Initialize pressure sensor driver
pub fn init() {
    if PRESSURE_INITIALIZED.load(Ordering::SeqCst) == 0 {
        // Initialize hardware-specific pressure sensors
        // This is a placeholder for hardware-specific code
        
        PRESSURE_INITIALIZED.store(1, Ordering::SeqCst);
    }
}

/// Check if pressure sensor driver is initialized
pub fn is_initialized() -> bool {
    PRESSURE_INITIALIZED.load(Ordering::SeqCst) != 0
}

/// Get pressure sensor driver version
pub fn get_version() -> &'static str {
    "Pressure Sensor Driver v0.7.0"
}

/// Default pressure sensor configuration
impl Default for PressureSensorConfig {
    fn default() -> Self {
        Self {
            sensor_type: PressureSensorType::Custom,
            i2c_address: None,
            update_interval_ms: 1000,
        }
    }
}