// Enhanced Socket Interface - VantisOS
//
// This module implements enhanced socket interface with
// connection management, buffering, and options.

use alloc::sync::Arc;
use alloc::collections::BTreeMap;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        
        let ip_parts: Vec<&str> = parts[0].split('.').collect();
        if ip_parts.len() != 4 {
            return None;
        }
        
        let ip = [
            ip_parts[0].parse().ok()?,
            ip_parts[1].parse().ok()?,
            ip_parts[2].parse().ok()?,
            ip_parts[3].parse().ok()?,
        ];
        
        let port = parts[1].parse().ok()?;
        
        Some(Self { ip, port })
    }
    
    /// Convert to string
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}.{}:{}", self.ip[0], self.ip[1], self.ip[2], self.ip[3], self.port)
    }
}

/// Socket options
#[derive(Debug, Clone, Copy)]
pub enum SocketOption {
    ReuseAddr(bool),
    KeepAlive(bool),
    TcpNoDelay(bool),
    ReceiveBufferSize(u32),
    SendBufferSize(u32),
    ReceiveTimeout(u32),
    SendTimeout(u32),
}

/// Socket structure
pub struct Socket {
    socket_type: SocketType,
    state: Mutex<SocketState>,
    local_address: Mutex<Option<SocketAddress>>,
    remote_address: Mutex<Option<SocketAddress>>,
    receive_buffer: Mutex<Vec<u8>>,
    send_buffer: Mutex<Vec<u8>>,
    options: Mutex<Vec<SocketOption>>,
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
            options: Mutex::new(Vec::new()),
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
    
    /// Set socket state
    pub fn set_state(&self, state: SocketState) {
        *self.state.lock() = state;
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
    
    /// Set socket option
    pub fn set_option(&self, option: SocketOption) -> Result<(), ()> {
        self.options.lock().push(option);
        Ok(())
    }
    
    /// Get socket option
    pub fn get_option(&self, option_type: SocketOption) -> Option<SocketOption> {
        self.options.lock().iter().find(|opt| {
            match (opt, &option_type) {
                (SocketOption::ReuseAddr(_), SocketOption::ReuseAddr(_)) => true,
                (SocketOption::KeepAlive(_), SocketOption::KeepAlive(_)) => true,
                (SocketOption::TcpNoDelay(_), SocketOption::TcpNoDelay(_)) => true,
                (SocketOption::ReceiveBufferSize(_), SocketOption::ReceiveBufferSize(_)) => true,
                (SocketOption::SendBufferSize(_), SocketOption::SendBufferSize(_)) => true,
                (SocketOption::ReceiveTimeout(_), SocketOption::ReceiveTimeout(_)) => true,
                (SocketOption::SendTimeout(_), SocketOption::SendTimeout(_)) => true,
                _ => false,
            }
        }).copied()
    }
}

/// Socket manager
pub struct SocketManager {
    sockets: BTreeMap<u32, Arc<Socket>>,
    next_socket_id: u32,
}

impl SocketManager {
    /// Create a new socket manager
    pub fn new() -> Self {
        Self {
            sockets: BTreeMap::new(),
            next_socket_id: 1,
        }
    }
    
    /// Create a socket
    pub fn create_socket(&mut self, socket_type: SocketType) -> u32 {
        let socket_id = self.next_socket_id;
        self.next_socket_id += 1;
        
        let socket = Arc::new(Socket::new(socket_type));
        self.sockets.insert(socket_id, socket);
        socket_id
    }
    
    /// Get a socket by ID
    pub fn get_socket(&self, socket_id: u32) -> Option<Arc<Socket>> {
        self.sockets.get(&socket_id).cloned()
    }
    
    /// Remove a socket
    pub fn remove_socket(&mut self, socket_id: u32) -> Option<Arc<Socket>> {
        self.sockets.remove(&socket_id)
    }
    
    /// Get all sockets
    pub fn get_all_sockets(&self) -> Vec<(u32, Arc<Socket>)> {
        self.sockets.iter().map(|(id, socket)| (*id, socket.clone())).collect()
    }
}

/// Initialize enhanced socket interface
pub fn init() {
    // TODO: Initialize enhanced socket interface
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_socket_address_creation() {
        let address = SocketAddress::new([192, 168, 1, 100], 8080);
        
        assert_eq!(address.ip, [192, 168, 1, 100]);
        assert_eq!(address.port, 8080);
    }
    
    #[test]
    fn test_socket_address_from_string() {
        let address = SocketAddress::from_str("192.168.1.100:8080");
        
        assert!(address.is_some());
        let address = address.unwrap();
        assert_eq!(address.ip, [192, 168, 1, 100]);
        assert_eq!(address.port, 8080);
    }
    
    #[test]
    fn test_socket_address_to_string() {
        let address = SocketAddress::new([192, 168, 1, 100], 8080);
        let string = address.to_string();
        
        assert_eq!(string, "192.168.1.100:8080");
    }
    
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
    
    #[test]
    fn test_socket_options() {
        let socket = Socket::new(SocketType::Stream);
        
        assert!(socket.set_option(SocketOption::ReuseAddr(true)).is_ok());
        assert!(socket.set_option(SocketOption::KeepAlive(true)).is_ok());
        
        let reuse_addr = socket.get_option(SocketOption::ReuseAddr(false));
        assert!(reuse_addr.is_some());
    }
    
    #[test]
    fn test_socket_manager() {
        let mut manager = SocketManager::new();
        
        let socket_id = manager.create_socket(SocketType::Stream);
        assert!(manager.get_socket(socket_id).is_some());
    }
}