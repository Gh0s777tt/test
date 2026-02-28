//! # I/O System
//!
//! This module provides I/O functionality for the minimal kernel.

pub mod char_dev;
pub mod block_dev;
pub mod request;

use char_dev::CharDevice;
use block_dev::BlockDevice;

/// Initialize I/O system
pub fn init() {
    // Initialize character devices
    CharDevice::init();
    
    // Initialize block devices
    BlockDevice::init();
}

/// I/O error
#[derive(Debug, Clone, Copy)]
pub enum IoError {
    DeviceNotFound,
    InvalidRequest,
    Timeout,
    Busy,
}

/// I/O request
pub struct IoRequest {
    /// Request type
    pub request_type: IoRequestType,
    /// Device ID
    pub device_id: u32,
    /// Buffer
    pub buffer: *mut u8,
    /// Size
    pub size: usize,
    /// Offset
    pub offset: u64,
}

/// I/O request type
pub enum IoRequestType {
    Read,
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_io_request() {
        let request = IoRequest {
            request_type: IoRequestType::Read,
            device_id: 0,
            buffer: 0x1000 as *mut u8,
            size: 4096,
            offset: 0,
        };
        
        assert_eq!(request.device_id, 0);
        assert_eq!(request.size, 4096);
    }
}