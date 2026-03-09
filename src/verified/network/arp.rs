// ARP (Address Resolution Protocol) - VantisOS
//
// This module implements ARP for mapping IP addresses to MAC addresses.

use alloc::vec::Vec;

/// ARP hardware types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ArpHardwareType {
    Ethernet = 1,
}

impl ArpHardwareType {
    /// Parse hardware type from u16
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(ArpHardwareType::Ethernet),
            _ => None,
        }
    }
}

/// ARP protocol types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ArpProtocolType {
    Ipv4 = 0x0800,
}

impl ArpProtocolType {
    /// Parse protocol type from u16
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            0x0800 => Some(ArpProtocolType::Ipv4),
            _ => None,
        }
    }
}

/// ARP operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ArpOperation {
    Request = 1,
    Reply = 2,
}

impl ArpOperation {
    /// Parse operation from u16
    pub fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(ArpOperation::Request),
            2 => Some(ArpOperation::Reply),
            _ => None,
        }
    }
}

/// ARP packet structure
#[derive(Debug, Clone)]
pub struct ArpPacket {
    pub hardware_type: ArpHardwareType,
    pub protocol_type: ArpProtocolType,
    pub hardware_size: u8,
    pub protocol_size: u8,
    pub operation: ArpOperation,
    pub sender_mac: [u8; 6],
    pub sender_ip: [u8; 4],
    pub target_mac: [u8; 6],
    pub target_ip: [u8; 4],
}

impl ArpPacket {
    /// Create a new ARP packet
    pub fn new(
        hardware_type: ArpHardwareType,
        protocol_type: ArpProtocolType,
        operation: ArpOperation,
        sender_mac: [u8; 6],
        sender_ip: [u8; 4],
        target_mac: [u8; 6],
        target_ip: [u8; 4],
    ) -> Self {
        Self {
            hardware_type,
            protocol_type,
            hardware_size: 6,
            protocol_size: 4,
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        }
    }
    
    /// Create an ARP request
    pub fn request(sender_mac: [u8; 6], sender_ip: [u8; 4], target_ip: [u8; 4]) -> Self {
        Self::new(
            ArpHardwareType::Ethernet,
            ArpProtocolType::Ipv4,
            ArpOperation::Request,
            sender_mac,
            sender_ip,
            [0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            target_ip,
        )
    }
    
    /// Create an ARP reply
    pub fn reply(
        sender_mac: [u8; 6],
        sender_ip: [u8; 4],
        target_mac: [u8; 6],
        target_ip: [u8; 4],
    ) -> Self {
        Self::new(
            ArpHardwareType::Ethernet,
            ArpProtocolType::Ipv4,
            ArpOperation::Reply,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        )
    }
    
    /// Parse ARP packet from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 28 {
            return None;
        }
        
        let hardware_type = ArpHardwareType::from_u16(u16::from_be_bytes([bytes[0], bytes[1]]))?;
        let protocol_type = ArpProtocolType::from_u16(u16::from_be_bytes([bytes[2], bytes[3]]))?;
        let hardware_size = bytes[4];
        let protocol_size = bytes[5];
        let operation = ArpOperation::from_u16(u16::from_be_bytes([bytes[6], bytes[7]]))?;
        let sender_mac: [u8; 6] = bytes[8..14].try_into().ok()?;
        let sender_ip: [u8; 4] = bytes[14..18].try_into().ok()?;
        let target_mac: [u8; 6] = bytes[18..24].try_into().ok()?;
        let target_ip: [u8; 4] = bytes[24..28].try_into().ok()?;
        
        Some(Self {
            hardware_type,
            protocol_type,
            hardware_size,
            protocol_size,
            operation,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        })
    }
    
    /// Serialize ARP packet to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(28);
        
        bytes.extend_from_slice(&(self.hardware_type as u16).to_be_bytes());
        bytes.extend_from_slice(&(self.protocol_type as u16).to_be_bytes());
        bytes.push(self.hardware_size);
        bytes.push(self.protocol_size);
        bytes.extend_from_slice(&(self.operation as u16).to_be_bytes());
        bytes.extend_from_slice(&self.sender_mac);
        bytes.extend_from_slice(&self.sender_ip);
        bytes.extend_from_slice(&self.target_mac);
        bytes.extend_from_slice(&self.target_ip);
        
        bytes
    }
    
    /// Get packet size
    pub fn size(&self) -> usize {
        28
    }
}

/// ARP cache entry
#[derive(Debug, Clone)]
pub struct ArpCacheEntry {
    pub ip_address: [u8; 4],
    pub mac_address: [u8; 6],
    pub timestamp: u64,
    pub is_static: bool,
}

impl ArpCacheEntry {
    /// Create a new ARP cache entry
    pub fn new(ip_address: [u8; 4], mac_address: [u8; 6]) -> Self {
        Self {
            ip_address,
            mac_address,
            timestamp: 0,
            is_static: false,
        }
    }
    
    /// Create a static ARP cache entry
    pub fn static_entry(ip_address: [u8; 4], mac_address: [u8; 6]) -> Self {
        Self {
            ip_address,
            mac_address,
            timestamp: 0,
            is_static: true,
        }
    }
}

/// ARP cache
pub struct ArpCache {
    entries: Vec<ArpCacheEntry>,
    max_entries: usize,
    timeout_seconds: u64,
}

impl ArpCache {
    /// Create a new ARP cache
    pub fn new(max_entries: usize, timeout_seconds: u64) -> Self {
        Self {
            entries: Vec::with_capacity(max_entries),
            max_entries,
            timeout_seconds,
        }
    }
    
    /// Add an entry to the cache
    pub fn add(&mut self, entry: ArpCacheEntry) {
        // Remove existing entry for same IP
        self.entries.retain(|e| e.ip_address != entry.ip_address);
        
        // Add new entry
        self.entries.push(entry);
        
        // Remove oldest entries if cache is full
        while self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }
    
    /// Look up an IP address in the cache
    pub fn lookup(&self, ip_address: [u8; 4]) -> Option<[u8; 6]> {
        self.entries
            .iter()
            .find(|e| e.ip_address == ip_address)
            .map(|e| e.mac_address)
    }
    
    /// Remove an entry from the cache
    pub fn remove(&mut self, ip_address: [u8; 4]) {
        self.entries.retain(|e| e.ip_address != ip_address);
    }
    
    /// Clear all entries from the cache
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    /// Get all entries
    pub fn get_entries(&self) -> &[ArpCacheEntry] {
        &self.entries
    }
    
    /// Get number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Initialize ARP
pub fn init() {
    // TODO: Initialize ARP cache
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_arp_packet_creation() {
        let packet = ArpPacket::request(
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            [192, 168, 1, 100],
            [192, 168, 1, 1],
        );
        
        assert_eq!(packet.operation, ArpOperation::Request);
        assert_eq!(packet.sender_mac, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(packet.sender_ip, [192, 168, 1, 100]);
        assert_eq!(packet.target_ip, [192, 168, 1, 1]);
    }
    
    #[test]
    fn test_arp_packet_serialization() {
        let packet = ArpPacket::request(
            [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            [192, 168, 1, 100],
            [192, 168, 1, 1],
        );
        
        let bytes = packet.to_bytes();
        assert_eq!(bytes.len(), 28);
        
        let parsed = ArpPacket::from_bytes(&bytes);
        assert!(parsed.is_some());
        
        let parsed = parsed.unwrap();
        assert_eq!(parsed.operation, packet.operation);
        assert_eq!(parsed.sender_mac, packet.sender_mac);
        assert_eq!(parsed.sender_ip, packet.sender_ip);
    }
    
    #[test]
    fn test_arp_cache() {
        let mut cache = ArpCache::new(100, 300);
        
        let entry = ArpCacheEntry::new([192, 168, 1, 100], [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        cache.add(entry);
        
        assert_eq!(cache.len(), 1);
        
        let mac = cache.lookup([192, 168, 1, 100]);
        assert!(mac.is_some());
        assert_eq!(mac.unwrap(), [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }
    
    #[test]
    fn test_arp_cache_remove() {
        let mut cache = ArpCache::new(100, 300);
        
        let entry = ArpCacheEntry::new([192, 168, 1, 100], [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        cache.add(entry);
        
        cache.remove([192, 168, 1, 100]);
        
        assert_eq!(cache.len(), 0);
        assert!(cache.lookup([192, 168, 1, 100]).is_none());
    }
    
    #[test]
    fn test_arp_operation_parsing() {
        assert_eq!(ArpOperation::from_u16(1), Some(ArpOperation::Request));
        assert_eq!(ArpOperation::from_u16(2), Some(ArpOperation::Reply));
        assert_eq!(ArpOperation::from_u16(3), None);
    }
}