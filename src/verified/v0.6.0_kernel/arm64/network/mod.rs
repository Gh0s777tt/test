// ARM64 Network Drivers for VantisOS v0.6.0
// Mobile network support

pub mod wifi;
pub mod bluetooth;
pub mod cellular;
pub mod gps;

pub use wifi::{WifiController, WifiNetwork, WifiSecurity, WifiStatus, WifiStats};
pub use bluetooth::{BluetoothController, BluetoothDevice, BluetoothVersion, BluetoothStatus, BluetoothStats};
pub use cellular::{CellularController, CellularNetwork, NetworkType, NetworkStatus, CellularStats};
pub use gps::{GpsController, GpsPosition, GpsFixType, GpsSatelliteInfo, GpsStats};