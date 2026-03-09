// Ethernet Driver (RTL8139) - VantisOS
//
// This module implements Ethernet functionality for VantisOS,
// including RTL8139 driver support and Ethernet frame handling.

use alloc::vec::Vec;
use core::mem;

/// Ethernet MTU (Maximum Transmission Unit)
pub const ETHERNET_MTU: u16 = 1500;

/// Ethernet header size
pub const ETHERNET_HEADER_SIZE: usize = 14;

/// Ethernet frame minimum size
pub const ETHERNET_MIN_FRAME_SIZE: usize = 64;

/// Ethernet frame maximum size
pub const ETHERNET_MAX_FRAME_SIZE: usize = 1518;

/// Ethernet address (MAC address)
pub type EthernetAddress = [u8; 6];

/// Broadcast MAC address
pub const ETHERNET_BROADCAST: EthernetAddress = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

/// Ethernet frame types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum EthernetFrameType {
    Ipv4 = 0x0800,
    Arp = 0x0806,
    Ipv6 = 0x86DD,
    Vlan = 0x8100,
}

impl EthernetFrameType {
    /// Parse frame type from u16
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0x0800 => Some(EthernetFrameType::Ipv4),
            0x0806 => Some(EthernetFrameType::Arp),
            0x86DD => Some(EthernetFrameType::Ipv6),
            0x8100 => Some(EthernetFrameType::Vlan),
            _ => None,
        }
    }
}

/// Ethernet frame structure
#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub destination: EthernetAddress,
    pub source: EthernetAddress,
    pub frame_type: EthernetFrameType,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    /// Create a new Ethernet frame
    pub fn new(
        destination: EthernetAddress,
        source: EthernetAddress,
        frame_type: EthernetFrameType,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            destination,
            source,
            frame_type,
            payload,
        }
    }
    
    /// Parse Ethernet frame from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < ETHERNET_HEADER_SIZE {
            return None;
        }
        
        let destination: EthernetAddress = bytes[0..6].try_into().ok()?;
        let source: EthernetAddress = bytes[6..12].try_into().ok()?;
        let frame_type_raw = u16::from_be_bytes([bytes[12], bytes[13]]);
        let frame_type = EthernetFrameType::from_u16(frame_type_raw)?;
        let payload = bytes[ETHERNET_HEADER_SIZE..].to_vec();
        
        Some(Self {
            destination,
            source,
            frame_type,
            payload,
        })
    }
    
    /// Serialize Ethernet frame to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(ETHERNET_HEADER_SIZE + self.payload.len());
        
        bytes.extend_from_slice(&self.destination);
        bytes.extend_from_slice(&self.source);
        bytes.extend_from_slice(&(self.frame_type as u16).to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        
        // Pad to minimum frame size
        while bytes.len() < ETHERNET_MIN_FRAME_SIZE {
            bytes.push(0);
        }
        
        bytes
    }
    
    /// Get frame size
    pub fn size(&self) -> usize {
        ETHERNET_HEADER_SIZE + self.payload.len()
    }
    
    /// Check if frame is broadcast
    pub fn is_broadcast(&self) -> bool {
        self.destination == ETHERNET_BROADCAST
    }
    
    /// Check if frame is multicast
    pub fn is_multicast(&self) -> bool {
        self.destination[0] & 0x01 == 0x01
    }
}

/// RTL8139 Ethernet driver
pub struct EthernetDriver {
    name: String,
    mac_address: EthernetAddress,
    mtu: u16,
    is_up: bool,
    promiscuous: bool,
    multicast: bool,
    stats: super::ndi::NetworkDeviceStats,
}

impl EthernetDriver {
    /// Create a new Ethernet driver
    pub fn new(name: String, mac_address: EthernetAddress) -> Self {
        Self {
            name,
            mac_address,
            mtu: ETHERNET_MTU,
            is_up: false,
            promiscuous: false,
            multicast: false,
            stats: super::ndi::NetworkDeviceStats::default(),
        }
    }
    
    /// Initialize the driver
    pub fn init(&mut self) -> Result<(), ()> {
        // TODO: Initialize RTL8139 hardware
        // This is a placeholder for actual hardware initialization
        Ok(())
    }
    
    /// Get MAC address
    pub fn get_mac_address(&self) -> EthernetAddress {
        self.mac_address
    }
    
    /// Set MAC address
    pub fn set_mac_address(&mut self, mac_address: EthernetAddress) {
        self.mac_address = mac_address;
    }
}

impl super::ndi::NetworkDeviceOps for EthernetDriver {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_type(&self) -> super::ndi::NetworkDeviceType {
        super::ndi::NetworkDeviceType::Ethernet
    }
    
    fn get_mac_address(&self) -> [u8; 6] {
        self.mac_address
    }
    
    fn get_mtu(&self) -> u16 {
        self.mtu
    }
    
    fn send_packet(&self, _packet: &[u8]) -> Result<(), ()> {
        // TODO: Send packet via RTL8139
        // This is a placeholder for actual hardware operation
        Ok(())
    }
    
    fn receive_packet(&self, _buffer: &mut [u8]) -> Result<usize, ()> {
        // TODO: Receive packet from RTL8139
        // This is a placeholder for actual hardware operation
        Err(())
    }
    
    fn get_stats(&self) -> super::ndi::NetworkDeviceStats {
        self.stats
    }
    
    fn set_promiscuous(&self, _enabled: bool) -> Result<(), ()> {
        // TODO: Set promiscuous mode
        Ok(())
    }
    
    fn set_multicast(&self, _enabled: bool) -> Result<(), ()> {
        // TODO: Set multicast mode
        Ok(())
    }
    
    fn is_up(&self) -> bool {
        self.is_up
    }
    
    fn bring_up(&self) -> Result<(), ()> {
        // TODO: Bring up interface
        Ok(())
    }
    
    fn bring_down(&self) -> Result<(), ()> {
        // TODO: Bring down interface
        Ok(())
    }
}

/// Initialize Ethernet driver
pub fn init() {
    // TODO: Initialize Ethernet driver
    // This will be called during system boot
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ethernet_frame_creation() {
        let frame = EthernetFrame::new(
            ETHERNET_BROADCAST,
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            EthernetFrameType::Ipv4,
            vec![0x45, 0x00, 0x00, 0x1c],
        );
        
        assert_eq!(frame.destination, ETHERNET_BROADCAST);
        assert_eq!(frame.source, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(frame.frame_type, EthernetFrameType::Ipv4);
        assert_eq!(frame.payload.len(), 4);
    }
    
    #[test]
    fn test_ethernet_frame_serialization() {
        let frame = EthernetFrame::new(
            ETHERNET_BROADCAST,
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            EthernetFrameType::Ipv4,
            vec![0x45, 0x00, 0x00, 0x1c],
        );
        
        let bytes = frame.to_bytes();
        assert!(bytes.len() >= ETHERNET_MIN_FRAME_SIZE);
        
        let parsed = EthernetFrame::from_bytes(&bytes);
        assert!(parsed.is_some());
        
        let parsed = parsed.unwrap();
        assert_eq!(parsed.destination, frame.destination);
        assert_eq!(parsed.source, frame.source);
        assert_eq!(parsed.frame_type, frame.frame_type);
    }
    
    #[test]
    fn test_ethernet_frame_type_parsing() {
        assert_eq!(EthernetFrameType::from_u16(0x0800), Some(EthernetFrameType::Ipv4));
        assert_eq!(EthernetFrameType::from_u16(0x0806), Some(EthernetFrameType::Arp));
        assert_eq!(EthernetFrameType::from_u16(0x86DD), Some(EthernetFrameType::Ipv6));
        assert_eq!(EthernetFrameType::from_u16(0x8100), Some(EthernetFrameType::Vlan));
        assert_eq!(EthernetFrameType::from_u16(0xFFFF), None);
    }
    
    #[test]
    fn test_ethernet_driver_creation() {
        let driver = EthernetDriver::new(
            "eth0".to_string(),
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
        );
        
        assert_eq!(driver.get_name(), "eth0");
        assert_eq!(driver.get_mac_address(), [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(driver.get_mtu(), ETHERNET_MTU);
    }
    
    #[test]
    fn test_broadcast_detection() {
        let frame = EthernetFrame::new(
            ETHERNET_BROADCAST,
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            EthernetFrameType::Ipv4,
            vec![],
        );
        
        assert!(frame.is_broadcast());
    }
    
    #[test]
    fn test_multicast_detection() {
        let frame = EthernetFrame::new(
            [0x01, 0x00, 0x5E, 0x00, 0x00, 0x01],
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            EthernetFrameType::Ipv4,
            vec![],
        );
        
        assert!(frame.is_multicast());
    }
}