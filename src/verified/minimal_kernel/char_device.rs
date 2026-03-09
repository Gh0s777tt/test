// Character Device Driver
//
// This module provides character device driver support for VantisOS, including:
// - Character device registration
// - Character device operations
// - Character device I/O
// - Character device management

#![no_std]

use crate::verified::minimal_kernel::io::request::IoRequest;
use core::sync::atomic::{AtomicU32, Ordering};

/// Character device ID type
pub type CharDeviceId = u32;

/// Invalid character device ID
pub const INVALID_CHAR_DEVICE_ID: CharDeviceId = 0;

/// Maximum number of character devices
pub const MAX_CHAR_DEVICES: usize = 256;

/// Character device operations
pub trait CharDeviceOps {
    /// Read from device
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, CharDeviceError>;
    
    /// Write to device
    fn write(&mut self, buffer: &[u8]) -> Result<usize, CharDeviceError>;
    
    /// Flush device
    fn flush(&mut self) -> Result<(), CharDeviceError>;
    
    /// Get device status
    fn get_status(&self) -> CharDeviceStatus;
}

/// Character device error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharDeviceError {
    /// No error
    None,
    /// Device not found
    NotFound,
    /// Device busy
    Busy,
    /// Invalid operation
    InvalidOperation,
    /// I/O error
    IoError,
    /// Buffer too small
    BufferTooSmall,
    /// Timeout
    Timeout,
}

/// Character device status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharDeviceStatus {
    /// Device is ready
    pub ready: bool,
    /// Device is busy
    pub busy: bool,
    /// Device has data to read
    pub has_data: bool,
    /// Device can accept data
    pub can_write: bool,
    /// Device error
    pub error: bool,
}

impl CharDeviceStatus {
    pub fn new() -> Self {
        CharDeviceStatus {
            ready: true,
            busy: false,
            has_data: false,
            can_write: true,
            error: false,
        }
    }
}

impl Default for CharDeviceStatus {
    fn default() -> Self {
        Self::new()
    }
}

/// Character device
pub struct CharDevice {
    /// Device ID
    id: CharDeviceId,
    /// Device name
    name: &'static str,
    /// Device type
    device_type: CharDeviceType,
    /// Device operations
    ops: Option<&'static dyn CharDeviceOps>,
    /// Device flags
    flags: CharDeviceFlags,
    /// Reference count
    ref_count: AtomicU32,
}

/// Character device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharDeviceType {
    /// Console device
    Console,
    /// Serial port
    Serial,
    /// Keyboard
    Keyboard,
    /// Mouse
    Mouse,
    /// TTY device
    Tty,
    /// PTY device
    Pty,
    /// Pipe
    Pipe,
    /// Socket
    Socket,
    /// Custom device
    Custom,
}

/// Character device flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharDeviceFlags {
    /// Readable
    pub readable: bool,
    /// Writable
    pub writable: bool,
    /// Non-blocking
    pub non_blocking: bool,
    /// Append only
    pub append_only: bool,
}

impl CharDeviceFlags {
    pub fn new() -> Self {
        CharDeviceFlags {
            readable: true,
            writable: true,
            non_blocking: false,
            append_only: false,
        }
    }
}

impl Default for CharDeviceFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl CharDevice {
    /// Create a new character device
    pub fn new(
        id: CharDeviceId,
        name: &'static str,
        device_type: CharDeviceType,
        flags: CharDeviceFlags,
    ) -> Self {
        CharDevice {
            id,
            name,
            device_type,
            ops: None,
            flags,
            ref_count: AtomicU32::new(0),
        }
    }

    /// Set device operations
    pub fn set_ops(&mut self, ops: &'static dyn CharDeviceOps) {
        self.ops = Some(ops);
    }

    /// Get device ID
    pub fn get_id(&self) -> CharDeviceId {
        self.id
    }

    /// Get device name
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Get device type
    pub fn get_type(&self) -> CharDeviceType {
        self.device_type
    }

    /// Get device flags
    pub fn get_flags(&self) -> CharDeviceFlags {
        self.flags
    }

    /// Get reference count
    pub fn get_ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    /// Increment reference count
    pub fn inc_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Decrement reference count
    pub fn dec_ref(&self) {
        self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }

    /// Read from device
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, CharDeviceError> {
        if !self.flags.readable {
            return Err(CharDeviceError::InvalidOperation);
        }

        if let Some(ops) = self.ops {
            ops.read(buffer)
        } else {
            Err(CharDeviceError::InvalidOperation)
        }
    }

    /// Write to device
    pub fn write(&mut self, buffer: &[u8]) -> Result<usize, CharDeviceError> {
        if !self.flags.writable {
            return Err(CharDeviceError::InvalidOperation);
        }

        if let Some(ops) = self.ops {
            ops.write(buffer)
        } else {
            Err(CharDeviceError::InvalidOperation)
        }
    }

    /// Flush device
    pub fn flush(&mut self) -> Result<(), CharDeviceError> {
        if let Some(ops) = self.ops {
            ops.flush()
        } else {
            Err(CharDeviceError::InvalidOperation)
        }
    }

    /// Get device status
    pub fn get_status(&self) -> CharDeviceStatus {
        if let Some(ops) = self.ops {
            ops.get_status()
        } else {
            CharDeviceStatus::new()
        }
    }
}

/// Character device manager
pub struct CharDeviceManager {
    /// Device table
    devices: [Option<CharDevice>; MAX_CHAR_DEVICES],
    /// Next device ID
    next_id: AtomicU32,
    /// Device count
    device_count: AtomicU32,
}

impl CharDeviceManager {
    /// Create a new character device manager
    pub fn new() -> Self {
        CharDeviceManager {
            devices: [None; MAX_CHAR_DEVICES],
            next_id: AtomicU32::new(1),
            device_count: AtomicU32::new(0),
        }
    }

    /// Register a character device
    pub fn register_device(&mut self, device: CharDevice) -> Result<CharDeviceId, CharDeviceError> {
        if self.device_count.load(Ordering::SeqCst) >= MAX_CHAR_DEVICES {
            return Err(CharDeviceError::Busy);
        }

        // Find free slot
        let slot = self.find_free_slot().ok_or(CharDeviceError::Busy)?;

        // Add to device table
        self.devices[slot] = Some(device);
        self.device_count.fetch_add(1, Ordering::SeqCst);

        Ok(self.devices[slot].as_ref().unwrap().get_id())
    }

    /// Unregister a character device
    pub fn unregister_device(&mut self, id: CharDeviceId) -> Result<(), CharDeviceError> {
        let slot = self.find_device_slot(id).ok_or(CharDeviceError::NotFound)?;

        // Check reference count
        let device = self.devices[slot].as_ref().unwrap();
        if device.get_ref_count() > 0 {
            return Err(CharDeviceError::Busy);
        }

        // Remove from device table
        self.devices[slot] = None;
        self.device_count.fetch_sub(1, Ordering::SeqCst);

        Ok(())
    }

    /// Get device by ID
    pub fn get_device(&self, id: CharDeviceId) -> Option<&CharDevice> {
        let slot = self.find_device_slot(id)?;
        self.devices[slot].as_ref()
    }

    /// Get device by ID (mutable)
    pub fn get_device_mut(&mut self, id: CharDeviceId) -> Option<&mut CharDevice> {
        let slot = self.find_device_slot(id)?;
        self.devices[slot].as_mut()
    }

    /// Get device by name
    pub fn get_device_by_name(&self, name: &str) -> Option<CharDeviceId> {
        for i in 0..MAX_CHAR_DEVICES {
            if let Some(device) = &self.devices[i] {
                if device.get_name() == name {
                    return Some(device.get_id());
                }
            }
        }
        None
    }

    /// Get device count
    pub fn get_device_count(&self) -> usize {
        self.device_count.load(Ordering::SeqCst)
    }

    /// Get all devices
    pub fn get_devices(&self) -> &[Option<CharDevice>; MAX_CHAR_DEVICES] {
        &self.devices
    }

    /// Find free slot in device table
    fn find_free_slot(&self) -> Option<usize> {
        for i in 0..MAX_CHAR_DEVICES {
            if self.devices[i].is_none() {
                return Some(i);
            }
        }
        None
    }

    /// Find device slot by ID
    fn find_device_slot(&self, id: CharDeviceId) -> Option<usize> {
        for i in 0..MAX_CHAR_DEVICES {
            if let Some(device) = &self.devices[i] {
                if device.get_id() == id {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Print device list
    pub fn print_device_list(&self) {
        // TODO: Implement printing when console is available
        // For now, this is a placeholder
    }
}

impl Default for CharDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_device_creation() {
        let device = CharDevice::new(
            1,
            "test",
            CharDeviceType::Console,
            CharDeviceFlags::new(),
        );
        
        assert_eq!(device.get_id(), 1);
        assert_eq!(device.get_name(), "test");
        assert_eq!(device.get_type(), CharDeviceType::Console);
    }

    #[test]
    fn test_char_device_manager() {
        let mut manager = CharDeviceManager::new();
        
        let device = CharDevice::new(
            1,
            "test",
            CharDeviceType::Console,
            CharDeviceFlags::new(),
        );
        
        assert!(manager.register_device(device).is_ok());
        assert_eq!(manager.get_device_count(), 1);
    }

    #[test]
    fn test_char_device_flags() {
        let flags = CharDeviceFlags::new();
        assert!(flags.readable);
        assert!(flags.writable);
        assert!(!flags.non_blocking);
    }

    #[test]
    fn test_char_device_status() {
        let status = CharDeviceStatus::new();
        assert!(status.ready);
        assert!(!status.busy);
    }
}