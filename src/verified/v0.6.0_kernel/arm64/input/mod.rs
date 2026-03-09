// ARM64 Input Drivers for VantisOS v0.6.0
// Mobile input support

pub mod accelerometer;
pub mod gyroscope;
pub mod magnetometer;

pub use accelerometer::{AccelerometerController, AccelerometerData, AccelerometerStats};
pub use gyroscope::{GyroscopeController, GyroscopeData, GyroscopeStats};
pub use magnetometer::{MagnetometerController, MagnetometerData, MagnetometerStats};