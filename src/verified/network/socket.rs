// Socket Interface - VantisOS
//
// This module provides a socket interface for network communication.

use alloc::sync::Arc;
use spin::Mutex;

/// Socket types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream,    // TCP
    Datagram,  // UDP
    Raw,       // Raw IP
}

/// Socket states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Unconnected,
    Connecting,
    Connected,
    Listening,
    Closing,
}

/// Socket address
#[derive(Debug, Clone, Copy)]
pub struct SocketAddress {
    pub ip: [u8; 4],
    pub port: u16,
}

impl SocketAddress {
    /// Create a new socket address
    pub fn new(ip: [u8; 4], port: u16) -> Self {
        Self { ip, port }
    }
    
    /// Create from string (e.g., "192.168.1.1:80")
    pub fn from_str(_s: &str) -> Option<Self> {
        // TODO: Parse string
        None
    }
}

/// Socket structure
pub struct Socket {
    socket_type: SocketType,
    state: Mutex<SocketState>,
    local_address: Mutex<Option<SocketAddress>>,
    remote_address: Mutex<Option<SocketAddress>>,
    receive_buffer: Mutex<Vec<u8>>,
    send_buffer: Mutex<Vec<u8>>,
}

impl Socket {
    /// Create a new socket
    pub fn new(socket_type: SocketType) -> Self {
        Self {
            socket_type,
            state: Mutex::new(SocketState::Unconnected),
            local_address: Mutex::new(None),
            remote_address: Mutex::new(None),
            receive_buffer: Mutex::new(Vec::new()),
            send_buffer: Mutex::new(Vec::new()),
        }
    }
    
    /// Get socket type
    pub fn get_type(&self) -> SocketType {
        self.socket_type
    }
    
    /// Get socket state
    pub fn get_state(&self) -> SocketState {
        *self.state.lock()
    }
    
    /// Bind socket to local address
    pub fn bind(&self, address: SocketAddress) -> Result<(), ()> {
        *self.local_address.lock() = Some(address);
        Ok(())
    }
    
    /// Connect socket to remote address
    pub fn connect(&self, address: SocketAddress) -> Result<(), ()> {
        *self.remote_address.lock() = Some(address);
        *self.state.lock() = SocketState::Connected;
        Ok(())
    }
    
    /// Listen for incoming connections
    pub fn listen(&self, _backlog: u32) -> Result<(), ()> {
        *self.state.lock() = SocketState::Listening;
        Ok(())
    }
    
    /// Accept incoming connection
    pub fn accept(&self) -> Result<Arc<Socket>, ()> {
        // TODO: Implement accept
        Err(())
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
    
    /// Close socket
    pub fn close(&self) {
        *self.state.lock() = SocketState::Closing;
    }
    
    /// Get local address
    pub fn get_local_address(&self) -> Option<SocketAddress> {
        *self.local_address.lock()
    }
    
    /// Get remote address
    pub fn get_remote_address(&self) -> Option<SocketAddress> {
        *self.remote_address.lock()
    }
}

/// Initialize socket interface
pub fn init() {
    // TODO: Initialize socket interface
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_socket_creation() {
        let socket = Socket::new(SocketType::Stream);
        
        assert_eq!(socket.get_type(), SocketType::Stream);
        assert_eq!(socket.get_state(), SocketState::Unconnected);
    }
    
    #[test]
    fn test_socket_bind() {
        let socket = Socket::new(SocketType::Stream);
        let address = SocketAddress::new([192, 168, 1, 100], 8080);
        
        assert!(socket.bind(address).is_ok());
        assert_eq!(socket.get_local_address(), Some(address));
    }
    
    #[test]
    fn test_socket_connect() {
        let socket = Socket::new(SocketType::Stream);
        let address = SocketAddress::new([192, 168, 1, 1], 80);
        
        assert!(socket.connect(address).is_ok());
        assert_eq!(socket.get_state(), SocketState::Connected);
        assert_eq!(socket.get_remote_address(), Some(address));
    }
    
    #[test]
    fn test_socket_send_receive() {
        let socket = Socket::new(SocketType::Stream);
        
        let data = b"Hello, World!";
        let sent = socket.send(data).unwrap();
        assert_eq!(sent, data.len());
        
        let mut buffer = [0u8; 1024];
        let received = socket.receive(&mut buffer).unwrap();
        assert_eq!(received, data.len());
    }
}