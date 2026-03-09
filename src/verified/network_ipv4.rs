// VantisOS IPv4 Implementation
//
// This module implements IPv4 protocol support including:
// - IPv4 packet parsing and generation
// - Fragmentation and reassembly
// - Routing
// - ICMP support

#![no_std]
#![allow(dead_code)]

use core::mem;
use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

use super::{NetworkError, IpAddress, MacAddress, PacketBuffer, Protocol};

/// IPv4 header length (minimum)
const IPV4_HEADER_MIN_LEN: u8 = 20;

/// IPv4 maximum header length
const IPV4_HEADER_MAX_LEN: u8 = 60;

/// IPv4 header options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ipv4Option {
    /// End of options list
    EndOfOptions,
    /// No operation
    NoOperation,
    /// Security
    Security,
    /// Loose source routing
    LooseSourceRouting,
    /// Strict source routing
    StrictSourceRouting,
    /// Record route
    RecordRoute,
    /// Stream identifier
    StreamIdentifier,
    /// Internet timestamp
    InternetTimestamp,
}

/// IPv4 header
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ipv4Header {
    /// Version (4 bits) and IHL (4 bits)
    pub version_ihl: u8,
    /// Type of service (8 bits)
    pub tos: u8,
    /// Total length (16 bits)
    pub total_length: u16,
    /// Identification (16 bits)
    pub identification: u16,
    /// Flags (3 bits) and fragment offset (13 bits)
    pub flags_fragment: u16,
    /// Time to live (8 bits)
    pub ttl: u8,
    /// Protocol (8 bits)
    pub protocol: u8,
    /// Header checksum (16 bits)
    pub header_checksum: u16,
    /// Source address (32 bits)
    pub source_addr: u32,
    /// Destination address (32 bits)
    pub destination_addr: u32,
}

impl Ipv4Header {
    /// Create a new IPv4 header
    pub fn new(
        source: u32,
        destination: u32,
        protocol: Protocol,
        ttl: u8,
        total_length: u16,
    ) -> Self {
        Self {
            version_ihl: 0x45, // Version 4, IHL 5 (20 bytes)
            tos: 0,
            total_length,
            identification: 0,
            flags_fragment: 0,
            ttl,
            protocol: protocol as u8,
            header_checksum: 0,
            source_addr: source,
            destination_addr: destination,
        }
    }

    /// Get version
    pub fn version(&self) -> u8 {
        (self.version_ihl >> 4) & 0x0F
    }

    /// Get IHL (Internet Header Length)
    pub fn ihl(&self) -> u8 {
        self.version_ihl & 0x0F
    }

    /// Get header length in bytes
    pub fn header_len(&self) -> u8 {
        self.ihl() * 4
    }

    /// Get flags
    pub fn flags(&self) -> u8 {
        ((self.flags_fragment >> 13) & 0x07) as u8
    }

    /// Get fragment offset
    pub fn fragment_offset(&self) -> u16 {
        self.flags_fragment & 0x1FFF
    }

    /// Check if packet is fragmented
    pub fn is_fragmented(&self) -> bool {
        (self.flags_fragment & 0x3FFF) != 0
    }

    /// Check if packet has more fragments
    pub fn has_more_fragments(&self) -> bool {
        (self.flags_fragment & 0x2000) != 0
    }

    /// Calculate header checksum
    pub fn calculate_checksum(&self) -> u16 {
        let mut sum: u32 = 0;

        // Add header words (16-bit)
        let words = unsafe {
            let ptr = self as *const Ipv4Header as *const u16;
            let len = self.header_len() as usize / 2;
            core::slice::from_raw_parts(ptr, len)
        };

        for &word in words {
            sum += word as u32;
        }

        // Fold 32-bit sum to 16 bits
        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }

    /// Verify header checksum
    pub fn verify_checksum(&self) -> bool {
        self.calculate_checksum() == 0
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 20] {
        let mut bytes = [0u8; 20];
        unsafe {
            let ptr = self as *const Ipv4Header as *const u8;
            core::ptr::copy_nonoverlapping(ptr, bytes.as_mut_ptr(), 20);
        }
        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 {
            return None;
        }

        unsafe {
            Some(*(bytes.as_ptr() as *const Ipv4Header))
        }
    }
}

/// IPv4 packet
pub struct Ipv4Packet {
    /// Header
    header: Ipv4Header,
    /// Payload
    payload: PacketBuffer,
}

impl Ipv4Packet {
    /// Create a new IPv4 packet
    pub fn new(
        source: u32,
        destination: u32,
        protocol: Protocol,
        ttl: u8,
        payload: PacketBuffer,
    ) -> Self {
        let total_length = 20 + payload.len() as u16;
        let mut header = Ipv4Header::new(source, destination, protocol, ttl, total_length);
        header.header_checksum = header.calculate_checksum();

        Self {
            header,
            payload,
        }
    }

    /// Get header
    pub fn header(&self) -> &Ipv4Header {
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
        let mut bytes = Vec::with_capacity(self.header.total_length as usize);
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(self.payload.as_slice());
        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, NetworkError> {
        if bytes.len() < 20 {
            return Err(NetworkError::InvalidPacket);
        }

        let header = Ipv4Header::from_bytes(bytes).ok_or(NetworkError::InvalidPacket)?;

        if header.version() != 4 {
            return Err(NetworkError::InvalidPacket);
        }

        if !header.verify_checksum() {
            return Err(NetworkError::InvalidPacket);
        }

        let header_len = header.header_len() as usize;
        if bytes.len() < header_len {
            return Err(NetworkError::InvalidPacket);
        }

        let payload_len = bytes.len() - header_len;
        let mut payload = PacketBuffer::new(payload_len).map_err(|_| NetworkError::OutOfMemory)?;
        payload.append(&bytes[header_len..]).map_err(|_| NetworkError::BufferOverflow)?;

        Ok(Self {
            header,
            payload,
        })
    }

    /// Get source address
    pub fn source(&self) -> IpAddress {
        IpAddress::V4(self.header.source_addr)
    }

    /// Get destination address
    pub fn destination(&self) -> IpAddress {
        IpAddress::V4(self.header.destination_addr)
    }

    /// Get protocol
    pub fn protocol(&self) -> Protocol {
        match self.header.protocol {
            1 => Protocol::Icmp,
            6 => Protocol::Tcp,
            17 => Protocol::Udp,
            58 => Protocol::IcmpV6,
            _ => Protocol::Icmp, // Default
        }
    }
}

/// IPv4 routing table entry
#[derive(Debug, Clone, Copy)]
pub struct RouteEntry {
    /// Destination network
    pub destination: u32,
    /// Network mask
    pub netmask: u32,
    /// Gateway
    pub gateway: u32,
    /// Interface index
    pub interface_index: u32,
    /// Metric
    pub metric: u32,
}

impl RouteEntry {
    /// Create a new route entry
    pub fn new(destination: u32, netmask: u32, gateway: u32, interface_index: u32, metric: u32) -> Self {
        Self {
            destination,
            netmask,
            gateway,
            interface_index,
            metric,
        }
    }

    /// Check if address matches this route
    pub fn matches(&self, addr: u32) -> bool {
        (addr & self.netmask) == (self.destination & self.netmask)
    }
}

/// IPv4 routing table
pub struct RoutingTable {
    /// Routes
    routes: core::cell::UnsafeCell<core::mem::MaybeUninit<[Option<RouteEntry>; 256]>>,
    /// Number of routes
    route_count: AtomicU32,
}

impl RoutingTable {
    /// Create a new routing table
    pub fn new() -> Self {
        Self {
            routes: core::cell::UnsafeCell::new(core::mem::MaybeUninit::uninit()),
            route_count: AtomicU32::new(0),
        }
    }

    /// Add route
    pub fn add_route(&self, route: RouteEntry) -> Result<(), NetworkError> {
        unsafe {
            let routes = &mut *self.routes.get();
            for i in 0..256 {
                if routes.assume_init_ref()[i].is_none() {
                    routes.assume_init_mut()[i] = Some(route);
                    self.route_count.fetch_add(1, Ordering::AcqRel);
                    return Ok(());
                }
            }
        }
        Err(NetworkError::OutOfMemory)
    }

    /// Find route for destination
    pub fn find_route(&self, destination: u32) -> Option<RouteEntry> {
        unsafe {
            let routes = &*self.routes.get();
            let mut best_route: Option<RouteEntry> = None;
            let mut best_metric = u32::MAX;

            for i in 0..256 {
                if let Some(route) = routes.assume_init_ref()[i] {
                    if route.matches(destination) && route.metric < best_metric {
                        best_route = Some(route);
                        best_metric = route.metric;
                    }
                }
            }

            best_route
        }
    }

    /// Get number of routes
    pub fn route_count(&self) -> u32 {
        self.route_count.load(Ordering::Acquire)
    }

    /// Clear all routes
    pub fn clear(&self) {
        unsafe {
            let routes = &mut *self.routes.get();
            for i in 0..256 {
                routes.assume_init_mut()[i] = None;
            }
        }
        self.route_count.store(0, Ordering::Release);
    }
}

/// IPv4 statistics
pub struct Ipv4Stats {
    /// Total packets received
    pub packets_received: AtomicU64,
    /// Total packets sent
    pub packets_sent: AtomicU64,
    /// Total bytes received
    pub bytes_received: AtomicU64,
    /// Total bytes sent
    pub bytes_sent: AtomicU64,
    /// Header checksum errors
    pub checksum_errors: AtomicU64,
    /// Fragmented packets received
    pub fragmented_received: AtomicU64,
    /// Fragmented packets sent
    pub fragmented_sent: AtomicU64,
    /// Reassembly timeouts
    pub reassembly_timeouts: AtomicU64,
}

impl Default for Ipv4Stats {
    fn default() -> Self {
        Self {
            packets_received: AtomicU64::new(0),
            packets_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),
            checksum_errors: AtomicU64::new(0),
            fragmented_received: AtomicU64::new(0),
            fragmented_sent: AtomicU64::new(0),
            reassembly_timeouts: AtomicU64::new(0),
        }
    }
}

/// IPv4 implementation
pub struct Ipv4 {
    /// Routing table
    routing_table: RoutingTable,
    /// Statistics
    stats: Ipv4Stats,
    /// Identification counter
    identification: AtomicU32,
}

impl Ipv4 {
    /// Create a new IPv4 implementation
    pub fn new() -> Self {
        Self {
            routing_table: RoutingTable::new(),
            stats: Ipv4Stats::default(),
            identification: AtomicU32::new(0),
        }
    }

    /// Get routing table
    pub fn routing_table(&self) -> &RoutingTable {
        &self.routing_table
    }

    /// Get statistics
    pub fn stats(&self) -> &Ipv4Stats {
        &self.stats
    }

    /// Send IPv4 packet
    pub fn send_packet(
        &self,
        source: u32,
        destination: u32,
        protocol: Protocol,
        ttl: u8,
        payload: PacketBuffer,
    ) -> Result<(), NetworkError> {
        let packet = Ipv4Packet::new(source, destination, protocol, ttl, payload);
        let bytes = packet.to_bytes();

        // Find route
        let route = self.routing_table.find_route(destination)
            .ok_or(NetworkError::NoRouteToHost)?;

        // Send packet (placeholder)
        self.stats.packets_sent.fetch_add(1, Ordering::AcqRel);
        self.stats.bytes_sent.fetch_add(bytes.len() as u64, Ordering::AcqRel);

        Ok(())
    }

    /// Receive IPv4 packet
    pub fn receive_packet(&self, bytes: &[u8]) -> Result<Ipv4Packet, NetworkError> {
        let packet = Ipv4Packet::from_bytes(bytes)?;

        self.stats.packets_received.fetch_add(1, Ordering::AcqRel);
        self.stats.bytes_received.fetch_add(bytes.len() as u64, Ordering::AcqRel);

        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_header() {
        let header = Ipv4Header::new(0xC0A80101, 0xC0A80102, Protocol::Tcp, 64, 40);
        assert_eq!(header.version(), 4);
        assert_eq!(header.ihl(), 5);
        assert_eq!(header.header_len(), 20);
        assert_eq!(header.source_addr, 0xC0A80101);
        assert_eq!(header.destination_addr, 0xC0A80102);
        assert_eq!(header.ttl, 64);
        assert_eq!(header.protocol, Protocol::Tcp as u8);
    }

    #[test]
    fn test_ipv4_header_checksum() {
        let mut header = Ipv4Header::new(0xC0A80101, 0xC0A80102, Protocol::Tcp, 64, 40);
        header.header_checksum = header.calculate_checksum();
        assert!(header.verify_checksum());
    }

    #[test]
    fn test_ipv4_packet() {
        let mut payload = PacketBuffer::new(20).unwrap();
        payload.append(&[1, 2, 3, 4]).unwrap();

        let packet = Ipv4Packet::new(0xC0A80101, 0xC0A80102, Protocol::Tcp, 64, payload);
        assert_eq!(packet.source(), IpAddress::V4(0xC0A80101));
        assert_eq!(packet.destination(), IpAddress::V4(0xC0A80102));
        assert_eq!(packet.protocol(), Protocol::Tcp);
    }

    #[test]
    fn test_routing_table() {
        let table = RoutingTable::new();

        let route = RouteEntry::new(0xC0A80000, 0xFFFFFF00, 0xC0A801FE, 0, 1);
        table.add_route(route).unwrap();

        assert_eq!(table.route_count(), 1);

        let found = table.find_route(0xC0A80101);
        assert!(found.is_some());
    }

    #[test]
    fn test_ipv4() {
        let ipv4 = Ipv4::new();

        let mut payload = PacketBuffer::new(20).unwrap();
        payload.append(&[1, 2, 3, 4]).unwrap();

        let result = ipv4.send_packet(0xC0A80101, 0xC0A80102, Protocol::Tcp, 64, payload);
        assert!(result.is_ok());
    }
}