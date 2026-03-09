// Enhanced UDP (User Datagram Protocol) - VantisOS
//
// This module implements enhanced UDP with checksum calculation
// and socket management.

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;

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
    
    /// Calculate UDP checksum
    pub fn calculate_checksum(&self, source_ip: [u8; 4], destination_ip: [u8; 4]) -> u16 {
        let mut sum: u32 = 0;
        
        // Add pseudo-header
        sum += (source_ip[0] as u32) << 8 | source_ip[1] as u32;
        sum += (source_ip[2] as u32) << 8 | source_ip[3] as u32;
        sum += (destination_ip[0] as u32) << 8 | destination_ip[1] as u32;
        sum += (destination_ip[2] as u32) << 8 | destination_ip[3] as u32;
        sum += 17; // UDP protocol
        sum += self.length as u32;
        
        // Add UDP header
        sum += self.source_port as u32;
        sum += self.destination_port as u32;
        sum += self.length as u32;
        
        // Add payload
        let mut i = 0;
        while i < self.payload.len() {
            if i + 1 < self.payload.len() {
                sum += (self.payload[i] as u32) << 8;
                sum += self.payload[i + 1] as u32;
            } else {
                sum += (self.payload[i] as u32) << 8;
            }
            i += 2;
        }
        
        // Fold 32-bit sum to 16 bits
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        
        // One's complement
        !sum as u16
    }
    
    /// Verify checksum
    pub fn verify_checksum(&self, source_ip: [u8; 4], destination_ip: [u8; 4]) -> bool {
        let calculated = self.calculate_checksum(source_ip, destination_ip);
        calculated == self.checksum
    }
}

/// UDP socket structure
pub struct UdpSocket {
    pub local_port: u16,
    pub local_ip: [u8; 4],
    pub receive_buffer: Mutex<Vec<u8>>,
    pub send_buffer: Mutex<Vec<u8>>,
}

impl UdpSocket {
    /// Create a new UDP socket
    pub fn new(local_port: u16, local_ip: [u8; 4]) -> Self {
        Self {
            local_port,
            local_ip,
            receive_buffer: Mutex::new(Vec::new()),
            send_buffer: Mutex::new(Vec::new()),
        }
    }
    
    /// Send data
    pub fn send(&self, data: &[u8]) -> Result<usize, ()> {
        self.send_buffer.lock().extend_from_slice(data);
        Ok(data.len())
    }
    
    /// Receive data
    pub fn receive(&self, buffer: &mut [u8]) -> Result<usize, ()> {
        let recv_buffer = self.receive_buffer.lock();
        let len = recv_buffer.len().min(buffer.len());
        buffer[..len].copy_from_slice(&recv_buffer[..len]);
        Ok(len)
    }
}

/// UDP socket manager
pub struct UdpSocketManager {
    sockets: BTreeMap<u16, UdpSocket>,
}

impl UdpSocketManager {
    /// Create a new UDP socket manager
    pub fn new() -> Self {
        Self {
            sockets: BTreeMap::new(),
        }
    }
    
    /// Create a socket
    pub fn create_socket(&mut self, port: u16, local_ip: [u8; 4]) -> Result<(), ()> {
        if self.sockets.contains_key(&port) {
            return Err(());
        }
        
        let socket = UdpSocket::new(port, local_ip);
        self.sockets.insert(port, socket);
        Ok(())
    }
    
    /// Get a socket by port
    pub fn get_socket(&self, port: u16) -> Option<&UdpSocket> {
        self.sockets.get(&port)
    }
    
    /// Remove a socket
    pub fn remove_socket(&mut self, port: u16) -> Option<UdpSocket> {
        self.sockets.remove(&port)
    }
}

/// Initialize enhanced UDP
pub fn init() {
    // TODO: Initialize enhanced UDP
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
    
    #[test]
    fn test_udp_checksum() {
        let datagram = UdpDatagram::new(12345, 53, vec![0x41, 0x42, 0x43]);
        
        let checksum = datagram.calculate_checksum(
            [192, 168, 1, 100],
            [192, 168, 1, 1],
        );
        
        assert!(checksum != 0);
    }
    
    #[test]
    fn test_udp_socket() {
        let socket = UdpSocket::new(12345, [192, 168, 1, 100]);
        
        assert_eq!(socket.local_port, 12345);
        assert_eq!(socket.local_ip, [192, 168, 1, 100]);
    }
    
    #[test]
    fn test_udp_socket_manager() {
        let mut manager = UdpSocketManager::new();
        
        assert!(manager.create_socket(12345, [192, 168, 1, 100]).is_ok());
        assert!(manager.get_socket(12345).is_some());
    }
}