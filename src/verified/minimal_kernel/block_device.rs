// Block Device Driver
//
// This module provides block device driver support for VantisOS, including:
// - Block device registration
// - Block device operations
// - Block device I/O
// - Block device management

#![no_std]

use crate::verified::minimal_kernel::io::request::IoRequest;
use core::sync::atomic::{AtomicU32, Ordering};

/// Block device ID type
pub type BlockDeviceId = u32;

/// Invalid block device ID
pub const INVALID_BLOCK_DEVICE_ID: BlockDeviceId = 0;

/// Maximum number of block devices
pub const MAX_BLOCK_DEVICES: usize = 32;

/// Block size (512 bytes)
pub const BLOCK_SIZE: usize = 512;

/// Block device operations
pub trait BlockDeviceOps {
    /// Read blocks from device
    fn read_blocks(&mut self, block_num: u64, buffer: &mut [u8]) -> Result<usize, BlockDeviceError>;
    
    /// Write blocks to device
    fn write_blocks(&mut self, block_num: u64, buffer: &[u8]) -> Result<usize, BlockDeviceError>;
    
    /// Flush device
    fn flush(&mut self) -> Result<(), BlockDeviceError>;
    
    /// Get device status
    fn get_status(&self) -> BlockDeviceStatus;
    
    /// Get device size (in blocks)
    fn get_size(&self) -> u64;
}

/// Block device error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockDeviceError {
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
    /// Invalid block number
    InvalidBlock,
    /// Read-only device
    ReadOnly,
    /// Timeout
    Timeout,
}

/// Block device status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeviceStatus {
    /// Device is ready
    pub ready: bool,
    /// Device is busy
    pub busy: bool,
    /// Device has media
    pub has_media: bool,
    /// Device is write-protected
    pub write_protected: bool,
    /// Device error
    pub error: bool,
}

impl BlockDeviceStatus {
    pub fn new() -> Self {
        BlockDeviceStatus {
            ready: true,
            busy: false,
            has_media: true,
            write_protected: false,
            error: false,
        }
    }
}

impl Default for BlockDeviceStatus {
    fn default() -> Self {
        Self::new()
    }
}

/// Block device
pub struct BlockDevice {
    /// Device ID
    id: BlockDeviceId,
    /// Device name
    name: &'static str,
    /// Device type
    device_type: BlockDeviceType,
    /// Device operations
    ops: Option<&'static dyn BlockDeviceOps>,
    /// Device flags
    flags: BlockDeviceFlags,
    /// Block size
    block_size: usize,
    /// Device size (in blocks)
    size: u64,
    /// Reference count
    ref_count: AtomicU32,
}

/// Block device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockDeviceType {
    /// Hard disk
    HardDisk,
    /// SSD
    Ssd,
    /// CD-ROM
    CdRom,
    /// DVD
    Dvd,
    /// USB drive
    UsbDrive,
    /// RAM disk
    RamDisk,
    /// Loop device
    LoopDevice,
    /// Custom device
    Custom,
}

/// Block device flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockDeviceFlags {
    /// Readable
    pub readable: bool,
    /// Writable
    pub writable: bool,
    /// Removable media
    pub removable: bool,
    /// Cache enabled
    pub cache_enabled: bool,
}

impl BlockDeviceFlags {
    pub fn new() -> Self {
        BlockDeviceFlags {
            readable: true,
            writable: true,
            removable: false,
            cache_enabled: true,
        }
    }
}

impl Default for BlockDeviceFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockDevice {
    /// Create a new block device
    pub fn new(
        id: BlockDeviceId,
        name: &'static str,
        device_type: BlockDeviceType,
        flags: BlockDeviceFlags,
        block_size: usize,
        size: u64,
    ) -> Self {
        BlockDevice {
            id,
            name,
            device_type,
            ops: None,
            flags,
            block_size,
            size,
            ref_count: AtomicU32::new(0),
        }
    }

    /// Set device operations
    pub fn set_ops(&mut self, ops: &'static dyn BlockDeviceOps) {
        self.ops = Some(ops);
    }

    /// Get device ID
    pub fn get_id(&self) -> BlockDeviceId {
        self.id
    }

    /// Get device name
    pub fn get_name(&self) -> &'static str {
        self.name
    }

    /// Get device type
    pub fn get_type(&self) -> BlockDeviceType {
        self.device_type
    }

    /// Get device flags
    pub fn get_flags(&self) -> BlockDeviceFlags {
        self.flags
    }

    /// Get block size
    pub fn get_block_size(&self) -> usize {
        self.block_size
    }

    /// Get device size (in blocks)
    pub fn get_size(&self) -> u64 {
        self.size
    }

    /// Get device size (in bytes)
    pub fn get_size_bytes(&self) -> u64 {
        self.size * self.block_size as u64
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

    /// Read blocks from device
    pub fn read_blocks(&mut self, block_num: u64, buffer: &mut [u8]) -> Result<usize, BlockDeviceError> {
        if !self.flags.readable {
            return Err(BlockDeviceError::InvalidOperation);
        }

        if block_num >= self.size {
            return Err(BlockDeviceError::InvalidBlock);
        }

        if buffer.len() % self.block_size != 0 {
            return Err(BlockDeviceError::InvalidOperation);
        }

        if let Some(ops) = self.ops {
            ops.read_blocks(block_num, buffer)
        } else {
            Err(BlockDeviceError::InvalidOperation)
        }
    }

    /// Write blocks to device
    pub fn write_blocks(&mut self, block_num: u64, buffer: &[u8]) -> Result<usize, BlockDeviceError> {
        if !self.flags.writable {
            return Err(BlockDeviceError::ReadOnly);
        }

        if block_num >= self.size {
            return Err(BlockDeviceError::InvalidBlock);
        }

        if buffer.len() % self.block_size != 0 {
            return Err(BlockDeviceError::InvalidOperation);
        }

        if let Some(ops) = self.ops {
            ops.write_blocks(block_num, buffer)
        } else {
            Err(BlockDeviceError::InvalidOperation)
        }
    }

    /// Flush device
    pub fn flush(&mut self) -> Result<(), BlockDeviceError> {
        if let Some(ops) = self.ops {
            ops.flush()
        } else {
            Err(BlockDeviceError::InvalidOperation)
        }
    }

    /// Get device status
    pub fn get_status(&self) -> BlockDeviceStatus {
        if let Some(ops) = self.ops {
            ops.get_status()
        } else {
            BlockDeviceStatus::new()
        }
    }
}

/// Block device manager
pub struct BlockDeviceManager {
    /// Device table
    devices: [Option<BlockDevice>; MAX_BLOCK_DEVICES],
    /// Next device ID
    next_id: AtomicU32,
    /// Device count
    device_count: AtomicU32,
}

impl BlockDeviceManager {
    /// Create a new block device manager
    pub fn new() -> Self {
        BlockDeviceManager {
            devices: [None; MAX_BLOCK_DEVICES],
            next_id: AtomicU32::new(1),
            device_count: AtomicU32::new(0),
        }
    }

    /// Register a block device
    pub fn register_device(&mut self, device: BlockDevice) -> Result<BlockDeviceId, BlockDeviceError> {
        if self.device_count.load(Ordering::SeqCst) >= MAX_BLOCK_DEVICES {
            return Err(BlockDeviceError::Busy);
        }

        // Find free slot
        let slot = self.find_free_slot().ok_or(BlockDeviceError::Busy)?;

        // Add to device table
        self.devices[slot] = Some(device);
        self.device_count.fetch_add(1, Ordering::SeqCst);

        Ok(self.devices[slot].as_ref().unwrap().get_id())
    }

    /// Unregister a block device
    pub fn unregister_device(&mut self, id: BlockDeviceId) -> Result<(), BlockDeviceError> {
        let slot = self.find_device_slot(id).ok_or(BlockDeviceError::NotFound)?;

        // Check reference count
        let device = self.devices[slot].as_ref().unwrap();
        if device.get_ref_count() > 0 {
            return Err(BlockDeviceError::Busy);
        }

        // Remove from device table
        self.devices[slot] = None;
        self.device_count.fetch_sub(1, Ordering::SeqCst);

        Ok(())
    }

    /// Get device by ID
    pub fn get_device(&self, id: BlockDeviceId) -> Option<&BlockDevice> {
        let slot = self.find_device_slot(id)?;
        self.devices[slot].as_ref()
    }

    /// Get device by ID (mutable)
    pub fn get_device_mut(&mut self, id: BlockDeviceId) -> Option<&mut BlockDevice> {
        let slot = self.find_device_slot(id)?;
        self.devices[slot].as_mut()
    }

    /// Get device by name
    pub fn get_device_by_name(&self, name: &str) -> Option<BlockDeviceId> {
        for i in 0..MAX_BLOCK_DEVICES {
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
    pub fn get_devices(&self) -> &[Option<BlockDevice>; MAX_BLOCK_DEVICES] {
        &self.devices
    }

    /// Find free slot in device table
    fn find_free_slot(&self) -> Option<usize> {
        for i in 0..MAX_BLOCK_DEVICES {
            if self.devices[i].is_none() {
                return Some(i);
            }
        }
        None
    }

    /// Find device slot by ID
    fn find_device_slot(&self, id: BlockDeviceId) -> Option<usize> {
        for i in 0..MAX_BLOCK_DEVICES {
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

impl Default for BlockDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_device_creation() {
        let device = BlockDevice::new(
            1,
            "test",
            BlockDeviceType::HardDisk,
            BlockDeviceFlags::new(),
            BLOCK_SIZE,
            1024,
        );
        
        assert_eq!(device.get_id(), 1);
        assert_eq!(device.get_name(), "test");
        assert_eq!(device.get_type(), BlockDeviceType::HardDisk);
        assert_eq!(device.get_block_size(), BLOCK_SIZE);
        assert_eq!(device.get_size(), 1024);
    }

    #[test]
    fn test_block_device_manager() {
        let mut manager = BlockDeviceManager::new();
        
        let device = BlockDevice::new(
            1,
            "test",
            BlockDeviceType::HardDisk,
            BlockDeviceFlags::new(),
            BLOCK_SIZE,
            1024,
        );
        
        assert!(manager.register_device(device).is_ok());
        assert_eq!(manager.get_device_count(), 1);
    }

    #[test]
    fn test_block_device_flags() {
        let flags = BlockDeviceFlags::new();
        assert!(flags.readable);
        assert!(flags.writable);
        assert!(!flags.removable);
    }

    #[test]
    fn test_block_device_status() {
        let status = BlockDeviceStatus::new();
        assert!(status.ready);
        assert!(!status.busy);
    }
}