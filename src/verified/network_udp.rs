// VantisOS UDP Implementation
//
// This module implements UDP (User Datagram Protocol) support including:
// - UDP packet parsing and generation
// - Checksum calculation
// - Socket management
// - Multicast support

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

use super::{NetworkError, IpAddress, SocketAddress, PacketBuffer, Port};

/// UDP header length
const UDP_HEADER_LEN: usize = 8;

/// UDP header
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UdpHeader {
    /// Source port (16 bits)
    pub source_port: u16,
    /// Destination port (16 bits)
    pub destination_port: u16,
    /// Length (16 bits)
    pub length: u16,
    /// Checksum (16 bits)
    pub checksum: u16,
}

impl UdpHeader {
    /// Create a new UDP header
    pub fn new(source_port: Port, destination_port: Port, length: u16) -> Self {
        Self {
            source_port,
            destination_port,
            length,
            checksum: 0,
        }
    }

    /// Calculate checksum
    pub fn calculate_checksum(&self, source_ip: &IpAddress, destination_ip: &IpAddress, payload: &[u8]) -> u16 {
        let mut sum: u32 = 0;

        // Pseudo-header
        match (source_ip, destination_ip) {
            (IpAddress::V4(src), IpAddress::V4(dst)) => {
                sum += *src;
                sum += *dst;
                sum += 17; // UDP protocol
                sum += self.length as u32;
            }
            (IpAddress::V6(src), IpAddress::V6(dst)) => {
                for i in (0..16).step_by(4) {
                    let word = u32::from_be_bytes([src[i], src[i+1], src[i+2], src[i+3]]);
                    sum += word;
                }
                for i in (0..16).step_by(4) {
                    let word = u32::from_be_bytes([dst[i], dst[i+1], dst[i+2], dst[i+3]]);
                    sum += word;
                }
                sum += 17; // UDP protocol
                sum += self.length as u32;
            }
            _ => {}
        }

        // UDP header
        let words = unsafe {
            let ptr = self as *const UdpHeader as *const u16;
            core::slice::from_raw_parts(ptr, 4)
        };

        for &word in words {
            sum += word as u32;
        }

        // Payload
        for i in (0..payload.len()).step_by(2) {
            let word = if i + 1 < payload.len() {
                ((payload[i] as u16) << 8) | (payload[i + 1] as u16)
            } else {
                (payload[i] as u16) << 8
            };
            sum += word as u32;
        }

        // Fold 32-bit sum to 16 bits
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        unsafe {
            let ptr = self as *const UdpHeader as *const u8;
            core::ptr::copy_nonoverlapping(ptr, bytes.as_mut_ptr(), 8);
        }
        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 8 {
            return None;
        }

        unsafe {
            Some(*(bytes.as_ptr() as *const UdpHeader))
        }
    }
}

/// UDP packet
pub struct UdpPacket {
    /// Header
    header: UdpHeader,
    /// Payload
    payload: PacketBuffer,
}

impl UdpPacket {
    /// Create a new UDP packet
    pub fn new(source_port: Port, destination_port: Port, payload: PacketBuffer) -> Self {
        let length = (8 + payload.len()) as u16;
        let header = UdpHeader::new(source_port, destination_port, length);

        Self {
            header,
            payload,
        }
    }

    /// Get header
    pub fn header(&self) -> &UdpHeader {
        &self.header
    }

    /// Get payload
    pub fn payload(&self) -> &PacketBuffer {
        &self.payload
    }

    /// Get payload as mutable
    pub fn payload_mut(&mut self) -> &mut PacketBuffer {
        &mut self.payload
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.header.length as usize);
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(self.payload.as_slice());
        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, NetworkError> {
        if bytes.len() < 8 {
            return Err(NetworkError::InvalidPacket);
        }

        let header = UdpHeader::from_bytes(bytes).ok_or(NetworkError::InvalidPacket)?;

        let payload_len = (header.length as usize).saturating_sub(8);
        if bytes.len() < 8 + payload_len {
            return Err(NetworkError::InvalidPacket);
        }

        let mut payload = PacketBuffer::new(payload_len).map_err(|_| NetworkError::OutOfMemory)?;
        payload.append(&bytes[8..8 + payload_len]).map_err(|_| NetworkError::BufferOverflow)?;

        Ok(Self {
            header,
            payload,
        })
    }

    /// Get source port
    pub fn source_port(&self) -> Port {
        self.header.source_port
    }

    /// Get destination port
    pub fn destination_port(&self) -> Port {
        self.header.destination_port
    }

    /// Get length
    pub fn length(&self) -> u16 {
        self.header.length
    }
}

/// UDP socket
pub struct UdpSocket {
    /// Local address
    local_addr: SocketAddress,
    /// Bound flag
    bound: bool,
    /// Multicast groups
    multicast_groups: core::cell::UnsafeCell<core::mem::MaybeUninit<[Option<IpAddress>; 16]>>,
    /// Multicast group count
    multicast_count: AtomicU32,
}

impl UdpSocket {
    /// Create a new UDP socket
    pub fn new(local_addr: SocketAddress) -> Self {
        Self {
            local_addr,
            bound: false,
            multicast_groups: core::cell::UnsafeCell::new(core::mem::MaybeUninit::uninit()),
            multicast_count: AtomicU32::new(0),
        }
    }

    /// Get local address
    pub fn local_addr(&self) -> SocketAddress {
        self.local_addr
    }

    /// Check if socket is bound
    pub fn is_bound(&self) -> bool {
        self.bound
    }

    /// Bind socket
    pub fn bind(&mut self) -> Result<(), NetworkError> {
        self.bound = true;
        Ok(())
    }

    /// Join multicast group
    pub fn join_multicast(&self, group: IpAddress) -> Result<(), NetworkError> {
        unsafe {
            let groups = &mut *self.multicast_groups.get();
            for i in 0..16 {
                if groups.assume_init_ref()[i].is_none() {
                    groups.assume_init_mut()[i] = Some(group);
                    self.multicast_count.fetch_add(1, Ordering::AcqRel);
                    return Ok(());
                }
            }
        }
        Err(NetworkError::OutOfMemory)
    }

    /// Leave multicast group
    pub fn leave_multicast(&self, group: IpAddress) -> Result<(), NetworkError> {
        unsafe {
            let groups = &mut *self.multicast_groups.get();
            for i in 0..16 {
                if groups.assume_init_ref()[i] == Some(group) {
                    groups.assume_init_mut()[i] = None;
                    self.multicast_count.fetch_sub(1, Ordering::AcqRel);
                    return Ok(());
                }
            }
        }
        Err(NetworkError::AddressNotAvailable)
    }

    /// Get multicast group count
    pub fn multicast_count(&self) -> u32 {
        self.multicast_count.load(Ordering::Acquire)
    }
}

/// UDP statistics
pub struct UdpStats {
    /// Datagrams received
    pub datagrams_received: AtomicU64,
    /// Datagrams sent
    pub datagrams_sent: AtomicU64,
    /// Bytes received
    pub bytes_received: AtomicU64,
    /// Bytes sent
    pub bytes_sent: AtomicU64,
    /// Checksum errors
    pub checksum_errors: AtomicU64,
    /// No port errors
    pub no_port_errors: AtomicU64,
}

impl Default for UdpStats {
    fn default() -> Self {
        Self {
            datagrams_received: AtomicU64::new(0),
            datagrams_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),
            checksum_errors: AtomicU64::new(0),
            no_port_errors: AtomicU64::new(0),
        }
    }
}

/// UDP implementation
pub struct Udp {
    /// Sockets
    sockets: core::cell::UnsafeCell<core::mem::MaybeUninit<[Option<UdpSocket>; 65536]>>,
    /// Statistics
    stats: UdpStats,
}

impl Udp {
    /// Create a new UDP implementation
    pub fn new() -> Self {
        Self {
            sockets: core::cell::UnsafeCell::new(core::mem::MaybeUninit::uninit()),
            stats: UdpStats::default(),
        }
    }

    /// Get statistics
    pub fn stats(&self) -> &UdpStats {
        &self.stats
    }

    /// Create socket
    pub fn socket(&self, local_addr: SocketAddress) -> Result<(), NetworkError> {
        let socket = UdpSocket::new(local_addr);
        
        unsafe {
            let sockets = &mut *self.sockets.get();
            for i in 0..65536 {
                if sockets.assume_init_ref()[i].is_none() {
                    sockets.assume_init_mut()[i] = Some(socket);
                    return Ok(());
                }
            }
        }

        Err(NetworkError::OutOfMemory)
    }

    /// Bind socket
    pub fn bind(&self, local_addr: SocketAddress) -> Result<(), NetworkError> {
        unsafe {
            let sockets = &mut *self.sockets.get();
            for i in 0..65536 {
                if let Some(socket) = sockets.assume_init_mut()[i].as_mut() {
                    if socket.local_addr() == local_addr {
                        return socket.bind();
                    }
                }
            }
        }

        Err(NetworkError::AddressNotAvailable)
    }

    /// Send datagram
    pub fn send_to(&self, local_addr: SocketAddress, remote_addr: SocketAddress, data: &[u8]) 
        -> Result<(), NetworkError> {
        let mut payload = PacketBuffer::new(data.len()).map_err(|_| NetworkError::OutOfMemory)?;
        payload.append(data).map_err(|_| NetworkError::BufferOverflow)?;

        let packet = UdpPacket::new(local_addr.port, remote_addr.port, payload);
        let bytes = packet.to_bytes();

        // Send packet (placeholder)
        self.stats.datagrams_sent.fetch_add(1, Ordering::AcqRel);
        self.stats.bytes_sent.fetch_add(bytes.len() as u64, Ordering::AcqRel);

        Ok(())
    }

    /// Receive datagram
    pub fn recv_from(&self, local_addr: SocketAddress, data: &mut [u8]) 
        -> Result<(usize, SocketAddress), NetworkError> {
        // Placeholder implementation
        self.stats.datagrams_received.fetch_add(1, Ordering::AcqRel);
        Ok((0, SocketAddress::new(IpAddress::v4(0, 0, 0, 0), 0)))
    }

    /// Close socket
    pub fn close(&self, local_addr: SocketAddress) -> Result<(), NetworkError> {
        unsafe {
            let sockets = &mut *self.sockets.get();
            for i in 0..65536 {
                if sockets.assume_init_ref()[i].as_ref().map(|s| s.local_addr()) == Some(local_addr) {
                    sockets.assume_init_mut()[i] = None;
                    return Ok(());
                }
            }
        }

        Err(NetworkError::AddressNotAvailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udp_header() {
        let header = UdpHeader::new(1234, 5678, 20);
        assert_eq!(header.source_port, 1234);
        assert_eq!(header.destination_port, 5678);
        assert_eq!(header.length, 20);
    }

    #[test]
    fn test_udp_packet() {
        let mut payload = PacketBuffer::new(12).unwrap();
        payload.append(&[1, 2, 3, 4]).unwrap();

        let packet = UdpPacket::new(1234, 5678, payload);
        assert_eq!(packet.source_port(), 1234);
        assert_eq!(packet.destination_port(), 5678);
        assert_eq!(packet.length(), 20);
    }

    #[test]
    fn test_udp_socket() {
        let local = SocketAddress::v4(192, 168, 1, 1, 1234);
        let mut socket = UdpSocket::new(local);
        
        assert!(!socket.is_bound());
        socket.bind().unwrap();
        assert!(socket.is_bound());
    }

    #[test]
    fn test_udp_multicast() {
        let local = SocketAddress::v4(192, 168, 1, 1, 1234);
        let socket = UdpSocket::new(local);
        
        let group = IpAddress::v4(224, 0, 0, 1);
        socket.join_multicast(group).unwrap();
        assert_eq!(socket.multicast_count(), 1);
        
        socket.leave_multicast(group).unwrap();
        assert_eq!(socket.multicast_count(), 0);
    }

    #[test]
    fn test_udp() {
        let udp = Udp::new();
        
        let local = SocketAddress::v4(192, 168, 1, 1, 1234);
        let remote = SocketAddress::v4(192, 168, 1, 2, 5678);
        
        let result = udp.socket(local);
        assert!(result.is_ok());
    }
}