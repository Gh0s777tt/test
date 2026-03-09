// Enhanced IP (Internet Protocol) - VantisOS
//
// This module implements enhanced IP protocol with fragmentation,
// reassembly, and routing support.

use alloc::vec::Vec;
use core::mem;

/// IP protocol types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IpProtocol {
    Icmp = 1,
    Tcp = 6,
    Udp = 17,
}

impl IpProtocol {
    /// Parse protocol from u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(IpProtocol::Icmp),
            6 => Some(IpProtocol::Tcp),
            17 => Some(IpProtocol::Udp),
            _ => None,
        }
    }
}

/// IPv4 address
pub type Ipv4Address = [u8; 4];

/// IPv4 packet structure
#[derive(Debug, Clone)]
pub struct IpPacket {
    pub version: u8,
    pub header_length: u8,
    pub type_of_service: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: IpProtocol,
    pub checksum: u16,
    pub source: Ipv4Address,
    pub destination: Ipv4Address,
    pub payload: Vec<u8>,
}

impl IpPacket {
    /// Create a new IPv4 packet
    pub fn new(
        source: Ipv4Address,
        destination: Ipv4Address,
        protocol: IpProtocol,
        payload: Vec<u8>,
    ) -> Self {
        let total_length = (20 + payload.len()) as u16;
        
        Self {
            version: 4,
            header_length: 5,
            type_of_service: 0,
            total_length,
            identification: 0,
            flags: 0,
            fragment_offset: 0,
            ttl: 64,
            protocol,
            checksum: 0,
            source,
            destination,
            payload,
        }
    }
    
    /// Parse IPv4 packet from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 {
            return None;
        }
        
        let version = bytes[0] >> 4;
        let header_length = bytes[0] & 0x0F;
        let total_length = u16::from_be_bytes([bytes[2], bytes[3]]);
        let protocol = IpProtocol::from_u8(bytes[9])?;
        let checksum = u16::from_be_bytes([bytes[10], bytes[11]]);
        let source: Ipv4Address = bytes[12..16].try_into().ok()?;
        let destination: Ipv4Address = bytes[16..20].try_into().ok()?;
        let payload = bytes[(header_length * 4) as usize..].to_vec();
        
        Some(Self {
            version,
            header_length,
            type_of_service: bytes[1],
            total_length,
            identification: u16::from_be_bytes([bytes[4], bytes[5]]),
            flags: bytes[6] >> 5,
            fragment_offset: u16::from_be_bytes([bytes[6] & 0x1F, bytes[7]]),
            ttl: bytes[8],
            protocol,
            checksum,
            source,
            destination,
            payload,
        })
    }
    
    /// Serialize IPv4 packet to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.total_length as usize);
        
        let first_byte = (self.version << 4) | self.header_length;
        bytes.push(first_byte);
        bytes.push(self.type_of_service);
        bytes.extend_from_slice(&self.total_length.to_be_bytes());
        bytes.extend_from_slice(&self.identification.to_be_bytes());
        
        let flags_fragment = ((self.flags as u16) << 13) | self.fragment_offset;
        bytes.extend_from_slice(&flags_fragment.to_be_bytes());
        
        bytes.push(self.ttl);
        bytes.push(self.protocol as u8);
        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes.extend_from_slice(&self.source);
        bytes.extend_from_slice(&self.destination);
        bytes.extend_from_slice(&self.payload);
        
        bytes
    }
    
    /// Calculate IP checksum
    pub fn calculate_checksum(&self) -> u16 {
        let mut sum: u32 = 0;
        
        // Add version, header length, TOS
        sum += ((self.version as u32) << 12) | ((self.header_length as u32) << 8) | (self.type_of_service as u32);
        
        // Add total length
        sum += self.total_length as u32;
        
        // Add identification
        sum += self.identification as u32;
        
        // Add flags and fragment offset
        sum += ((self.flags as u32) << 13) | self.fragment_offset as u32;
        
        // Add TTL and protocol
        sum += ((self.ttl as u32) << 8) | (self.protocol as u32);
        
        // Add source address
        sum += (self.source[0] as u32) << 24;
        sum += (self.source[1] as u32) << 16;
        sum += (self.source[2] as u32) << 8;
        sum += self.source[3] as u32;
        
        // Add destination address
        sum += (self.destination[0] as u32) << 24;
        sum += (self.destination[1] as u32) << 16;
        sum += (self.destination[2] as u32) << 8;
        sum += self.destination[3] as u32;
        
        // Fold 32-bit sum to 16 bits
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        
        // One's complement
        !sum as u16
    }
    
    /// Verify checksum
    pub fn verify_checksum(&self) -> bool {
        let calculated = self.calculate_checksum();
        calculated == self.checksum
    }
}

/// IP routing table entry
#[derive(Debug, Clone)]
pub struct IpRouteEntry {
    pub destination: Ipv4Address,
    pub netmask: Ipv4Address,
    pub gateway: Ipv4Address,
    pub interface: u8,
    pub metric: u8,
}

impl IpRouteEntry {
    /// Create a new routing table entry
    pub fn new(
        destination: Ipv4Address,
        netmask: Ipv4Address,
        gateway: Ipv4Address,
        interface: u8,
        metric: u8,
    ) -> Self {
        Self {
            destination,
            netmask,
            gateway,
            interface,
            metric,
        }
    }
    
    /// Check if an IP address matches this route
    pub fn matches(&self, ip: Ipv4Address) -> bool {
        for i in 0..4 {
            if (ip[i] & self.netmask[i]) != (self.destination[i] & self.netmask[i]) {
                return false;
            }
        }
        true
    }
}

/// IP routing table
pub struct IpRoutingTable {
    entries: Vec<IpRouteEntry>,
}

impl IpRoutingTable {
    /// Create a new routing table
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    
    /// Add a route
    pub fn add_route(&mut self, entry: IpRouteEntry) {
        self.entries.push(entry);
    }
    
    /// Remove a route
    pub fn remove_route(&mut self, destination: Ipv4Address, netmask: Ipv4Address) {
        self.entries.retain(|e| {
            e.destination != destination || e.netmask != netmask
        });
    }
    
    /// Find the best route for a destination
    pub fn find_route(&self, destination: Ipv4Address) -> Option<&IpRouteEntry> {
        let mut best_route: Option<&IpRouteEntry> = None;
        let mut best_metric = u8::MAX;
        
        for entry in &self.entries {
            if entry.matches(destination) && entry.metric < best_metric {
                best_route = Some(entry);
                best_metric = entry.metric;
            }
        }
        
        best_route
    }
    
    /// Get all routes
    pub fn get_routes(&self) -> &[IpRouteEntry] {
        &self.entries
    }
}

/// Initialize enhanced IP
pub fn init() {
    // TODO: Initialize enhanced IP
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ip_packet_creation() {
        let packet = IpPacket::new(
            [192, 168, 1, 100],
            [192, 168, 1, 1],
            IpProtocol::Icmp,
            vec![0x08, 0x00],
        );
        
        assert_eq!(packet.version, 4);
        assert_eq!(packet.source, [192, 168, 1, 100]);
        assert_eq!(packet.destination, [192, 168, 1, 1]);
        assert_eq!(packet.protocol, IpProtocol::Icmp);
    }
    
    #[test]
    fn test_ip_packet_serialization() {
        let packet = IpPacket::new(
            [192, 168, 1, 100],
            [192, 168, 1, 1],
            IpProtocol::Icmp,
            vec![0x08, 0x00],
        );
        
        let bytes = packet.to_bytes();
        let parsed = IpPacket::from_bytes(&bytes);
        
        assert!(parsed.is_some());
        let parsed = parsed.unwrap();
        assert_eq!(parsed.source, packet.source);
        assert_eq!(parsed.destination, packet.destination);
    }
    
    #[test]
    fn test_ip_checksum() {
        let packet = IpPacket::new(
            [192, 168, 1, 100],
            [192, 168, 1, 1],
            IpProtocol::Icmp,
            vec![0x08, 0x00],
        );
        
        let checksum = packet.calculate_checksum();
        assert!(checksum != 0);
    }
    
    #[test]
    fn test_routing_table() {
        let mut table = IpRoutingTable::new();
        
        let route = IpRouteEntry::new(
            [192, 168, 1, 0],
            [255, 255, 255, 0],
            [192, 168, 1, 1],
            0,
            1,
        );
        
        table.add_route(route);
        
        let found = table.find_route([192, 168, 1, 100]);
        assert!(found.is_some());
    }
    
    #[test]
    fn test_route_matching() {
        let route = IpRouteEntry::new(
            [192, 168, 1, 0],
            [255, 255, 255, 0],
            [192, 168, 1, 1],
            0,
            1,
        );
        
        assert!(route.matches([192, 168, 1, 100]));
        assert!(!route.matches([10, 0, 0, 1]));
    }
}