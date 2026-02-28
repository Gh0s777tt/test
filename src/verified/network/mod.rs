// Network Module - VantisOS
// 
// This module provides network functionality for VantisOS, including:
// - Network Device Interface (NDI)
// - Ethernet driver (RTL8139)
// - Network protocols (ARP, ICMP, IP, TCP, UDP)
// - Socket interface

pub mod ndi;
pub mod ethernet;
pub mod arp;
pub mod icmp;
pub mod ip;
pub mod ip_enhanced;
pub mod tcp;
pub mod tcp_enhanced;
pub mod udp;
pub mod udp_enhanced;
pub mod socket;
pub mod socket_enhanced;

pub use ndi::{NetworkDevice, NetworkDeviceOps, NetworkDeviceType};
pub use ethernet::{EthernetDriver, EthernetFrame};
pub use arp::{ArpPacket, ArpOperation};
pub use icmp::{IcmpPacket, IcmpType};
pub use ip::{IpPacket, IpProtocol};
pub use ip_enhanced::{IpPacket as IpPacketEnhanced, IpRoutingTable, IpRouteEntry};
pub use tcp::{TcpSegment, TcpState};
pub use tcp_enhanced::{TcpConnection, TcpConnectionManager};
pub use udp::{UdpDatagram};
pub use udp_enhanced::{UdpSocket, UdpSocketManager};
pub use socket::{Socket, SocketType, SocketState};
pub use socket_enhanced::{SocketAddress, SocketManager, SocketOption};

/// Network module initialization
pub fn init() {
    ndi::init();
    ethernet::init();
    arp::init();
    icmp::init();
    ip::init();
    tcp::init();
    udp::init();
    socket::init();
}