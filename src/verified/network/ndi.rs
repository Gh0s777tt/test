// Network Device Interface (NDI) - VantisOS
//
// This module provides a generic interface for network devices,
// allowing different network drivers to be used interchangeably.

use alloc::sync::Arc;
use spin::Mutex;

/// Maximum number of network devices
const MAX_NETWORK_DEVICES: usize = 16;

/// Network device types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkDeviceType {
    Ethernet,
    WiFi,
    Loopback,
    Virtual,
}

/// Network device statistics
#[derive(Debug, Default)]
pub struct NetworkDeviceStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors: u64,
    pub drops: u64,
}

/// Network device operations trait
pub trait NetworkDeviceOps: Send + Sync {
    /// Get device name
    fn get_name(&self) -> &str;
    
    /// Get device type
    fn get_type(&self) -> NetworkDeviceType;
    
    /// Get MAC address
    fn get_mac_address(&self) -> [u8; 6];
    
    /// Get MTU (Maximum Transmission Unit)
    fn get_mtu(&self) -> u16;
    
    /// Send a packet
    fn send_packet(&self, packet: &[u8]) -> Result<(), ()>;
    
    /// Receive a packet (non-blocking)
    fn receive_packet(&self, buffer: &mut [u8]) -> Result<usize, ()>;
    
    /// Get device statistics
    fn get_stats(&self) -> NetworkDeviceStats;
    
    /// Set promiscuous mode
    fn set_promiscuous(&self, enabled: bool) -> Result<(), ()>;
    
    /// Set multicast mode
    fn set_multicast(&self, enabled: bool) -> Result<(), ()>;
    
    /// Get device status
    fn is_up(&self) -> bool;
    
    /// Bring device up
    fn bring_up(&self) -> Result<(), ()>;
    
    /// Bring device down
    fn bring_down(&self) -> Result<(), ()>;
}

/// Network device structure
pub struct NetworkDevice {
    name: String,
    device_type: NetworkDeviceType,
    ops: Arc<dyn NetworkDeviceOps>,
    stats: Mutex<NetworkDeviceStats>,
    is_up: Mutex<bool>,
}

impl NetworkDevice {
    /// Create a new network device
    pub fn new(
        name: String,
        device_type: NetworkDeviceType,
        ops: Arc<dyn NetworkDeviceOps>,
    ) -> Self {
        Self {
            name,
            device_type,
            ops,
            stats: Mutex::new(NetworkDeviceStats::default()),
            is_up: Mutex::new(false),
        }
    }
    
    /// Get device name
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    /// Get device type
    pub fn get_type(&self) -> NetworkDeviceType {
        self.device_type
    }
    
    /// Get MAC address
    pub fn get_mac_address(&self) -> [u8; 6] {
        self.ops.get_mac_address()
    }
    
    /// Get MTU
    pub fn get_mtu(&self) -> u16 {
        self.ops.get_mtu()
    }
    
    /// Send a packet
    pub fn send_packet(&self, packet: &[u8]) -> Result<(), ()> {
        if !*self.is_up.lock() {
            return Err(());
        }
        
        let len = packet.len();
        self.ops.send_packet(packet)?;
        
        let mut stats = self.stats.lock();
        stats.bytes_sent += len as u64;
        stats.packets_sent += 1;
        
        Ok(())
    }
    
    /// Receive a packet
    pub fn receive_packet(&self, buffer: &mut [u8]) -> Result<usize, ()> {
        if !*self.is_up.lock() {
            return Err(());
        }
        
        let len = self.ops.receive_packet(buffer)?;
        
        let mut stats = self.stats.lock();
        stats.bytes_received += len as u64;
        stats.packets_received += 1;
        
        Ok(len)
    }
    
    /// Get device statistics
    pub fn get_stats(&self) -> NetworkDeviceStats {
        *self.stats.lock()
    }
    
    /// Set promiscuous mode
    pub fn set_promiscuous(&self, enabled: bool) -> Result<(), ()> {
        self.ops.set_promiscuous(enabled)
    }
    
    /// Set multicast mode
    pub fn set_multicast(&self, enabled: bool) -> Result<(), ()> {
        self.ops.set_multicast(enabled)
    }
    
    /// Check if device is up
    pub fn is_up(&self) -> bool {
        *self.is_up.lock()
    }
    
    /// Bring device up
    pub fn bring_up(&self) -> Result<(), ()> {
        self.ops.bring_up()?;
        *self.is_up.lock() = true;
        Ok(())
    }
    
    /// Bring device down
    pub fn bring_down(&self) -> Result<(), ()> {
        self.ops.bring_down()?;
        *self.is_up.lock() = false;
        Ok(())
    }
}

/// Network device manager
pub struct NetworkDeviceManager {
    devices: [Option<Arc<NetworkDevice>>; MAX_NETWORK_DEVICES],
    default_device: Mutex<Option<usize>>,
}

impl NetworkDeviceManager {
    /// Create a new network device manager
    pub const fn new() -> Self {
        Self {
            devices: [const { None }; MAX_NETWORK_DEVICES],
            default_device: Mutex::new(None),
        }
    }
    
    /// Register a network device
    pub fn register_device(&mut self, device: Arc<NetworkDevice>) -> Result<usize, ()> {
        for i in 0..MAX_NETWORK_DEVICES {
            if self.devices[i].is_none() {
                self.devices[i] = Some(device);
                
                // Set as default if first device
                if self.default_device.lock().is_none() {
                    *self.default_device.lock() = Some(i);
                }
                
                return Ok(i);
            }
        }
        
        Err(())
    }
    
    /// Unregister a network device
    pub fn unregister_device(&mut self, index: usize) -> Result<(), ()> {
        if index >= MAX_NETWORK_DEVICES || self.devices[index].is_none() {
            return Err(());
        }
        
        self.devices[index] = None;
        
        // Update default device if needed
        let mut default = self.default_device.lock();
        if *default == Some(index) {
            *default = self.devices.iter().position(|d| d.is_some());
        }
        
        Ok(())
    }
    
    /// Get a network device by index
    pub fn get_device(&self, index: usize) -> Option<Arc<NetworkDevice>> {
        if index >= MAX_NETWORK_DEVICES {
            return None;
        }
        
        self.devices[index].clone()
    }
    
    /// Get default network device
    pub fn get_default_device(&self) -> Option<Arc<NetworkDevice>> {
        let default = *self.default_device.lock();
        default.and_then(|index| self.get_device(index))
    }
    
    /// Set default network device
    pub fn set_default_device(&self, index: usize) -> Result<(), ()> {
        if index >= MAX_NETWORK_DEVICES || self.devices[index].is_none() {
            return Err(());
        }
        
        *self.default_device.lock() = Some(index);
        Ok(())
    }
    
    /// Get all network devices
    pub fn get_all_devices(&self) -> Vec<Arc<NetworkDevice>> {
        self.devices
            .iter()
            .filter_map(|d| d.clone())
            .collect()
    }
    
    /// Get number of registered devices
    pub fn get_device_count(&self) -> usize {
        self.devices.iter().filter(|d| d.is_some()).count()
    }
}

/// Global network device manager instance
static mut NETWORK_DEVICE_MANAGER: Option<NetworkDeviceManager> = None;

/// Initialize the network device manager
pub fn init() {
    unsafe {
        NETWORK_DEVICE_MANAGER = Some(NetworkDeviceManager::new());
    }
}

/// Get the global network device manager
pub fn get_manager() -> &'static mut NetworkDeviceManager {
    unsafe {
        NETWORK_DEVICE_MANAGER.as_mut().expect("Network device manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_network_device_stats() {
        let stats = NetworkDeviceStats::default();
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.bytes_received, 0);
        assert_eq!(stats.packets_sent, 0);
        assert_eq!(stats.packets_received, 0);
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.drops, 0);
    }
    
    #[test]
    fn test_network_device_manager() {
        let mut manager = NetworkDeviceManager::new();
        assert_eq!(manager.get_device_count(), 0);
        assert!(manager.get_default_device().is_none());
    }
    
    #[test]
    fn test_network_device_type() {
        assert_eq!(NetworkDeviceType::Ethernet, NetworkDeviceType::Ethernet);
        assert_eq!(NetworkDeviceType::WiFi, NetworkDeviceType::WiFi);
        assert_eq!(NetworkDeviceType::Loopback, NetworkDeviceType::Loopback);
        assert_eq!(NetworkDeviceType::Virtual, NetworkDeviceType::Virtual);
    }
}