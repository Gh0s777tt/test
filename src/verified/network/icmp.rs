// ICMP (Internet Control Message Protocol) - VantisOS
//
// This module implements ICMP for network diagnostics and error reporting.

use alloc::vec::Vec;

/// ICMP message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IcmpType {
    EchoReply = 0,
    DestinationUnreachable = 3,
    SourceQuench = 4,
    Redirect = 5,
    EchoRequest = 8,
    TimeExceeded = 11,
    ParameterProblem = 12,
    TimestampRequest = 13,
    TimestampReply = 14,
}

impl IcmpType {
    /// Parse ICMP type from u8
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(IcmpType::EchoReply),
            3 => Some(IcmpType::DestinationUnreachable),
            4 => Some(IcmpType::SourceQuench),
            5 => Some(IcmpType::Redirect),
            8 => Some(IcmpType::EchoRequest),
            11 => Some(IcmpType::TimeExceeded),
            12 => Some(IcmpType::ParameterProblem),
            13 => Some(IcmpType::TimestampRequest),
            14 => Some(IcmpType::TimestampReply),
            _ => None,
        }
    }
}

/// ICMP destination unreachable codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IcmpDestinationUnreachableCode {
    NetworkUnreachable = 0,
    HostUnreachable = 1,
    ProtocolUnreachable = 2,
    PortUnreachable = 3,
    FragmentationNeeded = 4,
    SourceRouteFailed = 5,
}

/// ICMP time exceeded codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum IcmpTimeExceededCode {
    TimeToLiveExceeded = 0,
    FragmentReassemblyTimeExceeded = 1,
}

/// ICMP packet structure
#[derive(Debug, Clone)]
pub struct IcmpPacket {
    pub icmp_type: IcmpType,
    pub code: u8,
    pub checksum: u16,
    pub payload: Vec<u8>,
}

impl IcmpPacket {
    /// Create a new ICMP packet
    pub fn new(icmp_type: IcmpType, code: u8, payload: Vec<u8>) -> Self {
        let mut packet = Self {
            icmp_type,
            code,
            checksum: 0,
            payload,
        };
        
        packet.checksum = packet.calculate_checksum();
        packet
    }
    
    /// Create an echo request (ping)
    pub fn echo_request(identifier: u16, sequence: u16, data: Vec<u8>) -> Self {
        let mut payload = Vec::with_capacity(4 + data.len());
        payload.extend_from_slice(&identifier.to_be_bytes());
        payload.extend_from_slice(&sequence.to_be_bytes());
        payload.extend_from_slice(&data);
        
        Self::new(IcmpType::EchoRequest, 0, payload)
    }
    
    /// Create an echo reply (pong)
    pub fn echo_reply(identifier: u16, sequence: u16, data: Vec<u8>) -> Self {
        let mut payload = Vec::with_capacity(4 + data.len());
        payload.extend_from_slice(&identifier.to_be_bytes());
        payload.extend_from_slice(&sequence.to_be_bytes());
        payload.extend_from_slice(&data);
        
        Self::new(IcmpType::EchoReply, 0, payload)
    }
    
    /// Create a destination unreachable message
    pub fn destination_unreachable(
        code: IcmpDestinationUnreachableCode,
        original_packet: &[u8],
    ) -> Self {
        let mut payload = Vec::with_capacity(8 + original_packet.len().min(8));
        payload.push(0); // unused
        payload.push(0); // unused
        payload.extend_from_slice(&0u16.to_be_bytes()); // next-hop MTU
        payload.extend_from_slice(&original_packet[..original_packet.len().min(8)]);
        
        Self::new(IcmpType::DestinationUnreachable, code as u8, payload)
    }
    
    /// Create a time exceeded message
    pub fn time_exceeded(code: IcmpTimeExceededCode, original_packet: &[u8]) -> Self {
        let mut payload = Vec::with_capacity(4 + original_packet.len().min(8));
        payload.push(0); // unused
        payload.push(0); // unused
        payload.push(0); // unused
        payload.push(0); // unused
        payload.extend_from_slice(&original_packet[..original_packet.len().min(8)]);
        
        Self::new(IcmpType::TimeExceeded, code as u8, payload)
    }
    
    /// Parse ICMP packet from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }
        
        let icmp_type = IcmpType::from_u8(bytes[0])?;
        let code = bytes[1];
        let checksum = u16::from_be_bytes([bytes[2], bytes[3]]);
        let payload = bytes[4..].to_vec();
        
        Some(Self {
            icmp_type,
            code,
            checksum,
            payload,
        })
    }
    
    /// Serialize ICMP packet to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + self.payload.len());
        
        bytes.push(self.icmp_type as u8);
        bytes.push(self.code);
        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        
        bytes
    }
    
    /// Calculate ICMP checksum
    fn calculate_checksum(&self) -> u16 {
        let mut sum: u32 = 0;
        
        // Add type and code
        sum += (self.icmp_type as u32) << 8;
        sum += self.code as u32;
        
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
    pub fn verify_checksum(&self) -> bool {
        let calculated = self.calculate_checksum();
        calculated == self.checksum
    }
    
    /// Get packet size
    pub fn size(&self) -> usize {
        4 + self.payload.len()
    }
    
    /// Get echo request identifier
    pub fn get_echo_identifier(&self) -> Option<u16> {
        if self.icmp_type == IcmpType::EchoRequest || self.icmp_type == IcmpType::EchoReply {
            if self.payload.len() >= 2 {
                Some(u16::from_be_bytes([self.payload[0], self.payload[1]]))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Get echo request sequence number
    pub fn get_echo_sequence(&self) -> Option<u16> {
        if self.icmp_type == IcmpType::EchoRequest || self.icmp_type == IcmpType::EchoReply {
            if self.payload.len() >= 4 {
                Some(u16::from_be_bytes([self.payload[2], self.payload[3]]))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Get echo request data
    pub fn get_echo_data(&self) -> Option<&[u8]> {
        if self.icmp_type == IcmpType::EchoRequest || self.icmp_type == IcmpType::EchoReply {
            if self.payload.len() >= 4 {
                Some(&self.payload[4..])
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// ICMP ping statistics
#[derive(Debug, Default)]
pub struct PingStatistics {
    pub packets_sent: u32,
    pub packets_received: u32,
    pub packets_lost: u32,
    pub round_trip_times: Vec<u64>,
}

impl PingStatistics {
    /// Create new ping statistics
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record a sent packet
    pub fn record_sent(&mut self) {
        self.packets_sent += 1;
    }
    
    /// Record a received packet
    pub fn record_received(&mut self, rtt: u64) {
        self.packets_received += 1;
        self.round_trip_times.push(rtt);
    }
    
    /// Calculate packet loss percentage
    pub fn packet_loss_percentage(&self) -> f64 {
        if self.packets_sent == 0 {
            0.0
        } else {
            (self.packets_lost as f64 / self.packets_sent as f64) * 100.0
        }
    }
    
    /// Calculate average round-trip time
    pub fn average_rtt(&self) -> Option<u64> {
        if self.round_trip_times.is_empty() {
            None
        } else {
            let sum: u64 = self.round_trip_times.iter().sum();
            Some(sum / self.round_trip_times.len() as u64)
        }
    }
    
    /// Calculate minimum round-trip time
    pub fn min_rtt(&self) -> Option<u64> {
        self.round_trip_times.iter().min().copied()
    }
    
    /// Calculate maximum round-trip time
    pub fn max_rtt(&self) -> Option<u64> {
        self.round_trip_times.iter().max().copied()
    }
}

/// Initialize ICMP
pub fn init() {
    // TODO: Initialize ICMP
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_icmp_echo_request() {
        let packet = IcmpPacket::echo_request(12345, 1, vec![0x41, 0x42, 0x43]);
        
        assert_eq!(packet.icmp_type, IcmpType::EchoRequest);
        assert_eq!(packet.code, 0);
        assert!(packet.verify_checksum());
    }
    
    #[test]
    fn test_icmp_echo_reply() {
        let packet = IcmpPacket::echo_reply(12345, 1, vec![0x41, 0x42, 0x43]);
        
        assert_eq!(packet.icmp_type, IcmpType::EchoReply);
        assert_eq!(packet.code, 0);
        assert!(packet.verify_checksum());
    }
    
    #[test]
    fn test_icmp_serialization() {
        let packet = IcmpPacket::echo_request(12345, 1, vec![0x41, 0x42, 0x43]);
        
        let bytes = packet.to_bytes();
        let parsed = IcmpPacket::from_bytes(&bytes);
        
        assert!(parsed.is_some());
        let parsed = parsed.unwrap();
        assert_eq!(parsed.icmp_type, packet.icmp_type);
        assert_eq!(parsed.code, packet.code);
        assert_eq!(parsed.payload, packet.payload);
    }
    
    #[test]
    fn test_icmp_echo_identifier() {
        let packet = IcmpPacket::echo_request(12345, 1, vec![0x41, 0x42, 0x43]);
        
        assert_eq!(packet.get_echo_identifier(), Some(12345));
        assert_eq!(packet.get_echo_sequence(), Some(1));
        assert_eq!(packet.get_echo_data(), Some(&[0x41, 0x42, 0x43][..]));
    }
    
    #[test]
    fn test_ping_statistics() {
        let mut stats = PingStatistics::new();
        
        stats.record_sent();
        stats.record_sent();
        stats.record_sent();
        
        stats.record_received(1000);
        stats.record_received(1500);
        
        assert_eq!(stats.packets_sent, 3);
        assert_eq!(stats.packets_received, 2);
        assert_eq!(stats.packets_lost, 1);
        assert_eq!(stats.packet_loss_percentage(), 33.333333333333336);
        assert_eq!(stats.average_rtt(), Some(1250));
        assert_eq!(stats.min_rtt(), Some(1000));
        assert_eq!(stats.max_rtt(), Some(1500));
    }
    
    #[test]
    fn test_icmp_type_parsing() {
        assert_eq!(IcmpType::from_u8(0), Some(IcmpType::EchoReply));
        assert_eq!(IcmpType::from_u8(8), Some(IcmpType::EchoRequest));
        assert_eq!(IcmpType::from_u8(11), Some(IcmpType::TimeExceeded));
        assert_eq!(IcmpType::from_u8(255), None);
    }
}