// TCP (Transmission Control Protocol) - VantisOS
//
// This module implements TCP for reliable, connection-oriented communication.

use alloc::vec::Vec;

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

/// Initialize TCP
pub fn init() {
    // TODO: Initialize TCP
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
}