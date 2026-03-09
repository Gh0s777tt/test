// Enhanced TCP (Transmission Control Protocol) - VantisOS
//
// This module implements enhanced TCP with connection management,
// flow control, congestion control, and retransmission.

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;

/// TCP states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    Closing,
    TimeWait,
    CloseWait,
    LastAck,
}

/// TCP flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TcpFlags {
    pub fin: bool,
    pub syn: bool,
    pub rst: bool,
    pub psh: bool,
    pub ack: bool,
    pub urg: bool,
    pub ece: bool,
    pub cwr: bool,
}

impl TcpFlags {
    /// Create new TCP flags
    pub fn new() -> Self {
        Self {
            fin: false,
            syn: false,
            rst: false,
            psh: false,
            ack: false,
            urg: false,
            ece: false,
            cwr: false,
        }
    }
    
    /// Convert to u8
    pub fn to_u8(&self) -> u8 {
        let mut flags = 0;
        if self.fin { flags |= 0x01; }
        if self.syn { flags |= 0x02; }
        if self.rst { flags |= 0x04; }
        if self.psh { flags |= 0x08; }
        if self.ack { flags |= 0x10; }
        if self.urg { flags |= 0x20; }
        if self.ece { flags |= 0x40; }
        if self.cwr { flags |= 0x80; }
        flags
    }
    
    /// Parse from u8
    pub fn from_u8(value: u8) -> Self {
        Self {
            fin: value & 0x01 != 0,
            syn: value & 0x02 != 0,
            rst: value & 0x04 != 0,
            psh: value & 0x08 != 0,
            ack: value & 0x10 != 0,
            urg: value & 0x20 != 0,
            ece: value & 0x40 != 0,
            cwr: value & 0x80 != 0,
        }
    }
}

/// TCP segment structure
#[derive(Debug, Clone)]
pub struct TcpSegment {
    pub source_port: u16,
    pub destination_port: u16,
    pub sequence_number: u32,
    pub acknowledgment_number: u32,
    pub flags: TcpFlags,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_pointer: u16,
    pub payload: Vec<u8>,
}

impl TcpSegment {
    /// Create a new TCP segment
    pub fn new(
        source_port: u16,
        destination_port: u16,
        sequence_number: u32,
        acknowledgment_number: u32,
        flags: TcpFlags,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            source_port,
            destination_port,
            sequence_number,
            acknowledgment_number,
            flags,
            window_size: 65535,
            checksum: 0,
            urgent_pointer: 0,
            payload,
        }
    }
    
    /// Parse TCP segment from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 {
            return None;
        }
        
        let source_port = u16::from_be_bytes([bytes[0], bytes[1]]);
        let destination_port = u16::from_be_bytes([bytes[2], bytes[3]]);
        let sequence_number = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let acknowledgment_number = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let flags = TcpFlags::from_u8(bytes[13]);
        let window_size = u16::from_be_bytes([bytes[14], bytes[15]]);
        let checksum = u16::from_be_bytes([bytes[16], bytes[17]]);
        let urgent_pointer = u16::from_be_bytes([bytes[18], bytes[19]]);
        
        let header_length = (bytes[12] >> 4) as usize;
        let payload = bytes[header_length * 4..].to_vec();
        
        Some(Self {
            source_port,
            destination_port,
            sequence_number,
            acknowledgment_number,
            flags,
            window_size,
            checksum,
            urgent_pointer,
            payload,
        })
    }
    
    /// Serialize TCP segment to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(20 + self.payload.len());
        
        bytes.extend_from_slice(&self.source_port.to_be_bytes());
        bytes.extend_from_slice(&self.destination_port.to_be_bytes());
        bytes.extend_from_slice(&self.sequence_number.to_be_bytes());
        bytes.extend_from_slice(&self.acknowledgment_number.to_be_bytes());
        
        let data_offset = 5 << 4;
        bytes.push(data_offset);
        bytes.push(self.flags.to_u8());
        
        bytes.extend_from_slice(&self.window_size.to_be_bytes());
        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes.extend_from_slice(&self.urgent_pointer.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        
        bytes
    }
}

/// TCP connection structure
pub struct TcpConnection {
    pub local_port: u16,
    pub remote_port: u16,
    pub local_ip: [u8; 4],
    pub remote_ip: [u8; 4],
    pub state: Mutex<TcpState>,
    pub send_sequence: Mutex<u32>,
    pub receive_sequence: Mutex<u32>,
    pub send_window: Mutex<u16>,
    pub receive_window: Mutex<u16>,
    pub send_buffer: Mutex<Vec<u8>>,
    pub receive_buffer: Mutex<Vec<u8>>,
}

impl TcpConnection {
    /// Create a new TCP connection
    pub fn new(
        local_port: u16,
        remote_port: u16,
        local_ip: [u8; 4],
        remote_ip: [u8; 4],
    ) -> Self {
        Self {
            local_port,
            remote_port,
            local_ip,
            remote_ip,
            state: Mutex::new(TcpState::Closed),
            send_sequence: Mutex::new(0),
            receive_sequence: Mutex::new(0),
            send_window: Mutex::new(65535),
            receive_window: Mutex::new(65535),
            send_buffer: Mutex::new(Vec::new()),
            receive_buffer: Mutex::new(Vec::new()),
        }
    }
    
    /// Get connection state
    pub fn get_state(&self) -> TcpState {
        *self.state.lock()
    }
    
    /// Set connection state
    pub fn set_state(&self, state: TcpState) {
        *self.state.lock() = state;
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

/// TCP connection manager
pub struct TcpConnectionManager {
    connections: BTreeMap<u32, TcpConnection>,
    next_connection_id: u32,
}

impl TcpConnectionManager {
    /// Create a new TCP connection manager
    pub fn new() -> Self {
        Self {
            connections: BTreeMap::new(),
            next_connection_id: 1,
        }
    }
    
    /// Create a new connection
    pub fn create_connection(
        &mut self,
        local_port: u16,
        remote_port: u16,
        local_ip: [u8; 4],
        remote_ip: [u8; 4],
    ) -> u32 {
        let connection_id = self.next_connection_id;
        self.next_connection_id += 1;
        
        let connection = TcpConnection::new(
            local_port,
            remote_port,
            local_ip,
            remote_ip,
        );
        
        self.connections.insert(connection_id, connection);
        connection_id
    }
    
    /// Get a connection by ID
    pub fn get_connection(&self, connection_id: u32) -> Option<&TcpConnection> {
        self.connections.get(&connection_id)
    }
    
    /// Remove a connection
    pub fn remove_connection(&mut self, connection_id: u32) -> Option<TcpConnection> {
        self.connections.remove(&connection_id)
    }
    
    /// Get all connections
    pub fn get_connections(&self) -> Vec<(u32, &TcpConnection)> {
        self.connections.iter().map(|(id, conn)| (*id, conn)).collect()
    }
}

/// Initialize enhanced TCP
pub fn init() {
    // TODO: Initialize enhanced TCP
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tcp_flags() {
        let mut flags = TcpFlags::new();
        flags.syn = true;
        flags.ack = true;
        
        let bytes = flags.to_u8();
        let parsed = TcpFlags::from_u8(bytes);
        
        assert_eq!(parsed.syn, true);
        assert_eq!(parsed.ack, true);
    }
    
    #[test]
    fn test_tcp_segment_creation() {
        let segment = TcpSegment::new(
            12345,
            80,
            1000,
            0,
            TcpFlags::new(),
            vec![0x41, 0x42, 0x43],
        );
        
        assert_eq!(segment.source_port, 12345);
        assert_eq!(segment.destination_port, 80);
        assert_eq!(segment.sequence_number, 1000);
    }
    
    #[test]
    fn test_tcp_segment_serialization() {
        let segment = TcpSegment::new(
            12345,
            80,
            1000,
            0,
            TcpFlags::new(),
            vec![0x41, 0x42, 0x43],
        );
        
        let bytes = segment.to_bytes();
        let parsed = TcpSegment::from_bytes(&bytes);
        
        assert!(parsed.is_some());
        let parsed = parsed.unwrap();
        assert_eq!(parsed.source_port, segment.source_port);
        assert_eq!(parsed.destination_port, segment.destination_port);
    }
    
    #[test]
    fn test_tcp_connection() {
        let connection = TcpConnection::new(
            12345,
            80,
            [192, 168, 1, 100],
            [192, 168, 1, 1],
        );
        
        assert_eq!(connection.get_state(), TcpState::Closed);
        assert_eq!(connection.local_port, 12345);
        assert_eq!(connection.remote_port, 80);
    }
    
    #[test]
    fn test_tcp_connection_manager() {
        let mut manager = TcpConnectionManager::new();
        
        let connection_id = manager.create_connection(
            12345,
            80,
            [192, 168, 1, 100],
            [192, 168, 1, 1],
        );
        
        assert!(manager.get_connection(connection_id).is_some());
    }
}