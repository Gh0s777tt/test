// VantisOS IPv6 Implementation
//
// This module implements IPv6 protocol support including:
// - IPv6 packet parsing and generation
// - Extension headers
// - Routing
// - ICMPv6 support

#![no_std]
#![allow(dead_code)]

use core::mem;
use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

use super::{NetworkError, IpAddress, PacketBuffer, Protocol};

/// IPv6 header length
const IPV6_HEADER_LEN: usize = 40;

/// IPv6 next header types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Ipv6NextHeader {
    /// Hop-by-hop options
    HopByHopOptions = 0,
    /// ICMPv6
    IcmpV6 = 58,
    /// No next header
    NoNextHeader = 59,
    /// Destination options
    DestinationOptions = 60,
    /// Routing header
    Routing = 43,
    /// Fragment header
    Fragment = 44,
    /// Authentication header
    Authentication = 51,
    /// Encapsulating security payload
    EncapsulatingSecurityPayload = 50,
    /// TCP
    Tcp = 6,
    /// UDP
    Udp = 17,
}

impl From<Protocol> for Ipv6NextHeader {
    fn from(protocol: Protocol) -> Self {
        match protocol {
            Protocol::Icmp => Ipv6NextHeader::IcmpV6,
            Protocol::IcmpV6 => Ipv6NextHeader::IcmpV6,
            Protocol::Tcp => Ipv6NextHeader::Tcp,
            Protocol::Udp => Ipv6NextHeader::Udp,
        }
    }
}

/// IPv6 header
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ipv6Header {
    /// Version (4 bits), traffic class (8 bits), flow label (20 bits)
    pub version_tc_fl: u32,
    /// Payload length (16 bits)
    pub payload_length: u16,
    /// Next header (8 bits)
    pub next_header: u8,
    /// Hop limit (8 bits)
    pub hop_limit: u8,
    /// Source address (128 bits)
    pub source_addr: [u8; 16],
    /// Destination address (128 bits)
    pub destination_addr: [u8; 16],
}

impl Ipv6Header {
    /// Create a new IPv6 header
    pub fn new(
        source: [u8; 16],
        destination: [u8; 16],
        next_header: Ipv6NextHeader,
        hop_limit: u8,
        payload_length: u16,
    ) -> Self {
        Self {
            version_tc_fl: 0x60000000, // Version 6
            payload_length,
            next_header: next_header as u8,
            hop_limit,
            source_addr: source,
            destination_addr: destination,
        }
    }

    /// Get version
    pub fn version(&self) -> u8 {
        ((self.version_tc_fl >> 28) & 0x0F) as u8
    }

    /// Get traffic class
    pub fn traffic_class(&self) -> u8 {
        ((self.version_tc_fl >> 20) & 0xFF) as u8
    }

    /// Get flow label
    pub fn flow_label(&self) -> u32 {
        self.version_tc_fl & 0x000FFFFF
    }

    /// Set traffic class
    pub fn set_traffic_class(&mut self, tc: u8) {
        self.version_tc_fl = (self.version_tc_fl & 0xF00FFFFF) | ((tc as u32) << 20);
    }

    /// Set flow label
    pub fn set_flow_label(&mut self, fl: u32) {
        self.version_tc_fl = (self.version_tc_fl & 0xFFF00000) | (fl & 0x000FFFFF);
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> [u8; 40] {
        let mut bytes = [0u8; 40];
        unsafe {
            let ptr = self as *const Ipv6Header as *const u8;
            core::ptr::copy_nonoverlapping(ptr, bytes.as_mut_ptr(), 40);
        }
        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 40 {
            return None;
        }

        unsafe {
            Some(*(bytes.as_ptr() as *const Ipv6Header))
        }
    }
}

/// IPv6 extension header
#[derive(Debug, Clone, Copy)]
pub enum Ipv6ExtensionHeader {
    /// Hop-by-hop options
    HopByHopOptions {
        next_header: Ipv6NextHeader,
        options: [u8; 8],
    },
    /// Destination options
    DestinationOptions {
        next_header: Ipv6NextHeader,
        options: [u8; 8],
    },
    /// Routing header
    Routing {
        next_header: Ipv6NextHeader,
        routing_type: u8,
        segments_left: u8,
        addresses: [[u8; 16]; 2],
    },
    /// Fragment header
    Fragment {
        next_header: Ipv6NextHeader,
        fragment_offset: u16,
        more_fragments: bool,
        identification: u32,
    },
}

/// IPv6 packet
pub struct Ipv6Packet {
    /// Header
    header: Ipv6Header,
    /// Extension headers
    extension_headers: Vec<Ipv6ExtensionHeader>,
    /// Payload
    payload: PacketBuffer,
}

impl Ipv6Packet {
    /// Create a new IPv6 packet
    pub fn new(
        source: [u8; 16],
        destination: [u8; 16],
        next_header: Ipv6NextHeader,
        hop_limit: u8,
        payload: PacketBuffer,
    ) -> Self {
        let payload_length = payload.len() as u16;
        let header = Ipv6Header::new(source, destination, next_header, hop_limit, payload_length);

        Self {
            header,
            extension_headers: Vec::new(),
            payload,
        }
    }

    /// Get header
    pub fn header(&self) -> &Ipv6Header {
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

    /// Add extension header
    pub fn add_extension_header(&mut self, ext: Ipv6ExtensionHeader) {
        self.extension_headers.push(ext);
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(IPV6_HEADER_LEN + self.payload.len());

        // Add header
        bytes.extend_from_slice(&self.header.to_bytes());

        // Add extension headers
        for ext in &self.extension_headers {
            match ext {
                Ipv6ExtensionHeader::HopByHopOptions { next_header, options } => {
                    bytes.push(*next_header as u8);
                    bytes.push(0); // Hdr Ext Len
                    bytes.extend_from_slice(options);
                }
                Ipv6ExtensionHeader::DestinationOptions { next_header, options } => {
                    bytes.push(*next_header as u8);
                    bytes.push(0); // Hdr Ext Len
                    bytes.extend_from_slice(options);
                }
                Ipv6ExtensionHeader::Routing { next_header, routing_type, segments_left, addresses } => {
                    bytes.push(*next_header as u8);
                    bytes.push(4); // Hdr Ext Len (4 * 8 = 32 bytes)
                    bytes.push(*routing_type);
                    bytes.push(*segments_left);
                    bytes.extend_from_slice(&[0u8; 4]); // Reserved
                    for addr in addresses {
                        bytes.extend_from_slice(addr);
                    }
                }
                Ipv6ExtensionHeader::Fragment { next_header, fragment_offset, more_fragments, identification } => {
                    bytes.push(*next_header as u8);
                    bytes.push(0); // Reserved
                    let offset = (fragment_offset >> 3) as u8;
                    let flags = if *more_fragments { 0x01 } else { 0x00 };
                    bytes.push(offset);
                    bytes.push(flags);
                    bytes.extend_from_slice(&identification.to_be_bytes());
                }
            }
        }

        // Add payload
        bytes.extend_from_slice(self.payload.as_slice());

        bytes
    }

    /// Parse from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, NetworkError> {
        if bytes.len() < IPV6_HEADER_LEN {
            return Err(NetworkError::InvalidPacket);
        }

        let header = Ipv6Header::from_bytes(bytes).ok_or(NetworkError::InvalidPacket)?;

        if header.version() != 6 {
            return Err(NetworkError::InvalidPacket);
        }

        let payload_len = header.payload_length as usize;
        if bytes.len() < IPV6_HEADER_LEN + payload_len {
            return Err(NetworkError::InvalidPacket);
        }

        let mut payload = PacketBuffer::new(payload_len).map_err(|_| NetworkError::OutOfMemory)?;
        payload.append(&bytes[IPV6_HEADER_LEN..IPV6_HEADER_LEN + payload_len])
            .map_err(|_| NetworkError::BufferOverflow)?;

        Ok(Self {
            header,
            extension_headers: Vec::new(),
            payload,
        })
    }

    /// Get source address
    pub fn source(&self) -> IpAddress {
        IpAddress::V6(self.header.source_addr)
    }

    /// Get destination address
    pub fn destination(&self) -> IpAddress {
        IpAddress::V6(self.header.destination_addr)
    }

    /// Get next header
    pub fn next_header(&self) -> Ipv6NextHeader {
        match self.header.next_header {
            0 => Ipv6NextHeader::HopByHopOptions,
            43 => Ipv6NextHeader::Routing,
            44 => Ipv6NextHeader::Fragment,
            50 => Ipv6NextHeader::EncapsulatingSecurityPayload,
            51 => Ipv6NextHeader::Authentication,
            58 => Ipv6NextHeader::IcmpV6,
            59 => Ipv6NextHeader::NoNextHeader,
            60 => Ipv6NextHeader::DestinationOptions,
            6 => Ipv6NextHeader::Tcp,
            17 => Ipv6NextHeader::Udp,
            _ => Ipv6NextHeader::NoNextHeader,
        }
    }
}

/// IPv6 routing table entry
#[derive(Debug, Clone, Copy)]
pub struct Ipv6RouteEntry {
    /// Destination prefix
    pub destination: [u8; 16],
    /// Prefix length
    pub prefix_length: u8,
    /// Gateway
    pub gateway: [u8; 16],
    /// Interface index
    pub interface_index: u32,
    /// Metric
    pub metric: u32,
}

impl Ipv6RouteEntry {
    /// Create a new IPv6 route entry
    pub fn new(
        destination: [u8; 16],
        prefix_length: u8,
        gateway: [u8; 16],
        interface_index: u32,
        metric: u32,
    ) -> Self {
        Self {
            destination,
            prefix_length,
            gateway,
            interface_index,
            metric,
        }
    }

    /// Check if address matches this route
    pub fn matches(&self, addr: &[u8; 16]) -> bool {
        let full_bytes = (self.prefix_length / 8) as usize;
        let remaining_bits = self.prefix_length % 8;

        // Compare full bytes
        for i in 0..full_bytes {
            if self.destination[i] != addr[i] {
                return false;
            }
        }

        // Compare remaining bits
        if remaining_bits > 0 && full_bytes < 16 {
            let mask = 0xFF << (8 - remaining_bits);
            if (self.destination[full_bytes] & mask) != (addr[full_bytes] & mask) {
                return false;
            }
        }

        true
    }
}

/// IPv6 routing table
pub struct Ipv6RoutingTable {
    /// Routes
    routes: core::cell::UnsafeCell<core::mem::MaybeUninit<[Option<Ipv6RouteEntry>; 256]>>,
    /// Number of routes
    route_count: AtomicU32,
}

impl Ipv6RoutingTable {
    /// Create a new IPv6 routing table
    pub fn new() -> Self {
        Self {
            routes: core::cell::UnsafeCell::new(core::mem::MaybeUninit::uninit()),
            route_count: AtomicU32::new(0),
        }
    }

    /// Add route
    pub fn add_route(&self, route: Ipv6RouteEntry) -> Result<(), NetworkError> {
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
    pub fn find_route(&self, destination: &[u8; 16]) -> Option<Ipv6RouteEntry> {
        unsafe {
            let routes = &*self.routes.get();
            let mut best_route: Option<Ipv6RouteEntry> = None;
            let mut best_prefix_length = 0u8;
            let mut best_metric = u32::MAX;

            for i in 0..256 {
                if let Some(route) = routes.assume_init_ref()[i] {
                    if route.matches(destination) {
                        // Prefer longer prefix
                        if route.prefix_length > best_prefix_length ||
                           (route.prefix_length == best_prefix_length && route.metric < best_metric) {
                            best_route = Some(route);
                            best_prefix_length = route.prefix_length;
                            best_metric = route.metric;
                        }
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

/// IPv6 statistics
pub struct Ipv6Stats {
    /// Total packets received
    pub packets_received: AtomicU64,
    /// Total packets sent
    pub packets_sent: AtomicU64,
    /// Total bytes received
    pub bytes_received: AtomicU64,
    /// Total bytes sent
    pub bytes_sent: AtomicU64,
    /// Fragmented packets received
    pub fragmented_received: AtomicU64,
    /// Fragmented packets sent
    pub fragmented_sent: AtomicU64,
    /// Reassembly timeouts
    pub reassembly_timeouts: AtomicU64,
}

impl Default for Ipv6Stats {
    fn default() -> Self {
        Self {
            packets_received: AtomicU64::new(0),
            packets_sent: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),
            fragmented_received: AtomicU64::new(0),
            fragmented_sent: AtomicU64::new(0),
            reassembly_timeouts: AtomicU64::new(0),
        }
    }
}

/// IPv6 implementation
pub struct Ipv6 {
    /// Routing table
    routing_table: Ipv6RoutingTable,
    /// Statistics
    stats: Ipv6Stats,
}

impl Ipv6 {
    /// Create a new IPv6 implementation
    pub fn new() -> Self {
        Self {
            routing_table: Ipv6RoutingTable::new(),
            stats: Ipv6Stats::default(),
        }
    }

    /// Get routing table
    pub fn routing_table(&self) -> &Ipv6RoutingTable {
        &self.routing_table
    }

    /// Get statistics
    pub fn stats(&self) -> &Ipv6Stats {
        &self.stats
    }

    /// Send IPv6 packet
    pub fn send_packet(
        &self,
        source: [u8; 16],
        destination: [u8; 16],
        next_header: Ipv6NextHeader,
        hop_limit: u8,
        payload: PacketBuffer,
    ) -> Result<(), NetworkError> {
        let packet = Ipv6Packet::new(source, destination, next_header, hop_limit, payload);
        let bytes = packet.to_bytes();

        // Find route
        let route = self.routing_table.find_route(&destination)
            .ok_or(NetworkError::NoRouteToHost)?;

        // Send packet (placeholder)
        self.stats.packets_sent.fetch_add(1, Ordering::AcqRel);
        self.stats.bytes_sent.fetch_add(bytes.len() as u64, Ordering::AcqRel);

        Ok(())
    }

    /// Receive IPv6 packet
    pub fn receive_packet(&self, bytes: &[u8]) -> Result<Ipv6Packet, NetworkError> {
        let packet = Ipv6Packet::from_bytes(bytes)?;

        self.stats.packets_received.fetch_add(1, Ordering::AcqRel);
        self.stats.bytes_received.fetch_add(bytes.len() as u64, Ordering::AcqRel);

        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_header() {
        let source = [0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001];
        let source_bytes: [u8; 16] = unsafe { core::mem::transmute(source) };
        
        let destination = [0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0002];
        let destination_bytes: [u8; 16] = unsafe { core::mem::transmute(destination) };

        let header = Ipv6Header::new(source_bytes, destination_bytes, Ipv6NextHeader::Tcp, 64, 20);
        assert_eq!(header.version(), 6);
        assert_eq!(header.hop_limit, 64);
        assert_eq!(header.next_header, Ipv6NextHeader::Tcp as u8);
    }

    #[test]
    fn test_ipv6_packet() {
        let source = [0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001];
        let source_bytes: [u8; 16] = unsafe { core::mem::transmute(source) };
        
        let destination = [0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0002];
        let destination_bytes: [u8; 16] = unsafe { core::mem::transmute(destination) };

        let mut payload = PacketBuffer::new(20).unwrap();
        payload.append(&[1, 2, 3, 4]).unwrap();

        let packet = Ipv6Packet::new(source_bytes, destination_bytes, Ipv6NextHeader::Tcp, 64, payload);
        assert_eq!(packet.source(), IpAddress::V6(source_bytes));
        assert_eq!(packet.destination(), IpAddress::V6(destination_bytes));
    }

    #[test]
    fn test_ipv6_routing_table() {
        let table = Ipv6RoutingTable::new();

        let destination = [0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000];
        let gateway = [0xfe80, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001];
        let route = Ipv6RouteEntry::new(destination, 64, gateway, 0, 1);
        table.add_route(route).unwrap();

        assert_eq!(table.route_count(), 1);

        let test_addr = [0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001];
        let found = table.find_route(&test_addr);
        assert!(found.is_some());
    }

    #[test]
    fn test_ipv6() {
        let ipv6 = Ipv6::new();

        let source = [0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0001];
        let source_bytes: [u8; 16] = unsafe { core::mem::transmute(source) };
        
        let destination = [0x2001, 0x0db8, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0002];
        let destination_bytes: [u8; 16] = unsafe { core::mem::transmute(destination) };

        let mut payload = PacketBuffer::new(20).unwrap();
        payload.append(&[1, 2, 3, 4]).unwrap();

        let result = ipv6.send_packet(source_bytes, destination_bytes, Ipv6NextHeader::Tcp, 64, payload);
        assert!(result.is_ok());
    }
}