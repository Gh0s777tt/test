// IP (Internet Protocol) - VantisOS
//
// This module implements IP protocol for network layer communication.

use alloc::vec::Vec;

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
}

/// Initialize IP
pub fn init() {
    // TODO: Initialize IP
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
    fn test_ip_protocol_parsing() {
        assert_eq!(IpProtocol::from_u8(1), Some(IpProtocol::Icmp));
        assert_eq!(IpProtocol::from_u8(6), Some(IpProtocol::Tcp));
        assert_eq!(IpProtocol::from_u8(17), Some(IpProtocol::Udp));
        assert_eq!(IpProtocol::from_u8(255), None);
    }
}