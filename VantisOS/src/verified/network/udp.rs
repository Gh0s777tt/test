// UDP (User Datagram Protocol) - VantisOS
//
// This module implements UDP for connectionless, unreliable communication.

use alloc::vec::Vec;

/// UDP datagram structure
#[derive(Debug, Clone)]
pub struct UdpDatagram {
    pub source_port: u16,
    pub destination_port: u16,
    pub length: u16,
    pub checksum: u16,
    pub payload: Vec<u8>,
}

impl UdpDatagram {
    /// Create a new UDP datagram
    pub fn new(source_port: u16, destination_port: u16, payload: Vec<u8>) -> Self {
        let length = (8 + payload.len()) as u16;
        
        Self {
            source_port,
            destination_port,
            length,
            checksum: 0,
            payload,
        }
    }
    
    /// Parse UDP datagram from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        
        let source_port = u16::from_be_bytes([bytes[0], bytes[1]]);
        let destination_port = u16::from_be_bytes([bytes[2], bytes[3]]);
        let length = u16::from_be_bytes([bytes[4], bytes[5]]);
        let checksum = u16::from_be_bytes([bytes[6], bytes[7]]);
        let payload = bytes[8..].to_vec();
        
        Some(Self {
            source_port,
            destination_port,
            length,
            checksum,
            payload,
        })
    }
    
    /// Serialize UDP datagram to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.payload.len());
        
        bytes.extend_from_slice(&self.source_port.to_be_bytes());
        bytes.extend_from_slice(&self.destination_port.to_be_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        
        bytes
    }
}

/// Initialize UDP
pub fn init() {
    // TODO: Initialize UDP
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_udp_datagram_creation() {
        let datagram = UdpDatagram::new(12345, 53, vec![0x41, 0x42, 0x43]);
        
        assert_eq!(datagram.source_port, 12345);
        assert_eq!(datagram.destination_port, 53);
        assert_eq!(datagram.length, 11);
    }
    
    #[test]
    fn test_udp_datagram_serialization() {
        let datagram = UdpDatagram::new(12345, 53, vec![0x41, 0x42, 0x43]);
        
        let bytes = datagram.to_bytes();
        let parsed = UdpDatagram::from_bytes(&bytes);
        
        assert!(parsed.is_some());
        let parsed = parsed.unwrap();
        assert_eq!(parsed.source_port, datagram.source_port);
        assert_eq!(parsed.destination_port, datagram.destination_port);
    }
}