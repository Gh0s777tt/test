//! DEVFS - Device File System
//! Provides device nodes in /dev

use super::*;
use alloc::collections::BTreeMap;
use alloc::string::String;
use spin::Mutex;

/// Device types for DEVFS
#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    /// Block device (disk, partition)
    Block,
    /// Character device (tty, keyboard, mouse)
    Char,
    /// Pipe device
    Pipe,
    /// Socket device
    Socket,
}

/// Device information
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Device name
    pub name: String,
    /// Device type
    pub dev_type: DeviceType,
    /// Major device number
    pub major: u32,
    /// Minor device number
    pub minor: u32,
    /// Device operations
    pub read: Option<fn(&mut [u8]) -> Result<usize, FsError>>,
    pub write: Option<fn(&[u8]) -> Result<usize, FsError>>,
    /// Control operations
    pub ioctl: Option<fn(u32, usize) -> Result<(), FsError>>,
}

/// DEVFS inode
pub struct DevfsInode {
    /// Inode number
    pub ino: u64,
    /// Device info
    pub device: Option<DeviceInfo>,
    /// Directory entries
    pub entries: BTreeMap<String, u64>,
}

/// DEVFS superblock
pub struct DevfsSuperblock {
    /// Root inode
    pub root_ino: u64,
    /// Next inode number
    pub next_ino: u64,
    /// All inodes
    pub inodes: BTreeMap<u64, DevfsInode>,
    /// Device name to inode mapping
    pub devices: BTreeMap<String, u64>,
}

impl DevfsSuperblock {
    /// Create a new DEVFS instance
    pub fn new() -> Self {
        let mut sb = Self {
            root_ino: 1,
            next_ino: 2,
            inodes: BTreeMap::new(),
            devices: BTreeMap::new(),
        };
        
        // Create root directory
        let root = DevfsInode {
            ino: 1,
            device: None,
            entries: BTreeMap::new(),
        };
        sb.inodes.insert(1, root);
        
        // Create standard devices
        sb.create_device("null", DeviceType::Char, 1, 3, Some(Self::null_read), Some(Self::null_write), None);
        sb.create_device("zero", DeviceType::Char, 1, 5, Some(Self::zero_read), Some(Self::zero_write), None);
        sb.create_device("full", DeviceType::Char, 1, 7, Some(Self::full_read), Some(Self::full_write), None);
        sb.create_device("random", DeviceType::Char, 1, 8, Some(Self::random_read), None, None);
        sb.create_device("urandom", DeviceType::Char, 1, 9, Some(Self::random_read), None, None);
        sb.create_device("tty", DeviceType::Char, 5, 0, Some(Self::tty_read), Some(Self::tty_write), None);
        sb.create_device("console", DeviceType::Char, 5, 1, Some(Self::tty_read), Some(Self::tty_write), None);
        sb.create_device("stdin", DeviceType::Char, 5, 2, Some(Self::tty_read), None, None);
        sb.create_device("stdout", DeviceType::Char, 5, 3, None, Some(Self::tty_write), None);
        sb.create_device("stderr", DeviceType::Char, 5, 4, None, Some(Self::tty_write), None);
        sb.create_device("tty0", DeviceType::Char, 4, 0, Some(Self::tty_read), Some(Self::tty_write), None);
        sb.create_device("tty1", DeviceType::Char, 4, 1, Some(Self::tty_read), Some(Self::tty_write), None);
        
        // Block devices
        sb.create_device("sda", DeviceType::Block, 8, 0, None, None, None);
        sb.create_device("sda1", DeviceType::Block, 8, 1, None, None, None);
        sb.create_device("sda2", DeviceType::Block, 8, 2, None, None, None);
        sb.create_device("hda", DeviceType::Block, 3, 0, None, None, None);
        sb.create_device("hda1", DeviceType::Block, 3, 1, None, None, None);
        
        // Input devices
        sb.create_device("input/keyboard", DeviceType::Char, 13, 64, Some(Self::keyboard_read), None, None);
        sb.create_device("input/mouse0", DeviceType::Char, 13, 32, Some(Self::mouse_read), None, None);
        sb.create_device("input/event0", DeviceType::Char, 13, 64, None, None, None);
        
        sb
    }
    
    /// Create a device
    fn create_device(
        &mut self,
        name: &str,
        dev_type: DeviceType,
        major: u32,
        minor: u32,
        read: Option<fn(&mut [u8]) -> Result<usize, FsError>>,
        write: Option<fn(&[u8]) -> Result<usize, FsError>>,
        ioctl: Option<fn(u32, usize) -> Result<(), FsError>>,
    ) {
        let ino = self.next_ino;
        self.next_ino += 1;
        
        let device = DeviceInfo {
            name: String::from(name),
            dev_type,
            major,
            minor,
            read,
            write,
            ioctl,
        };
        
        let inode = DevfsInode {
            ino,
            device: Some(device),
            entries: BTreeMap::new(),
        };
        
        self.inodes.insert(ino, inode);
        self.devices.insert(String::from(name), ino);
        
        // Add to root directory
        if let Some(root) = self.inodes.get_mut(&1) {
            root.entries.insert(String::from(name), ino);
        }
    }
    
    // Device implementations
    
    /// /dev/null - read returns nothing
    fn null_read(_buf: &mut [u8]) -> Result<usize, FsError> {
        Ok(0)
    }
    
    /// /dev/null - write discards everything
    fn null_write(buf: &[u8]) -> Result<usize, FsError> {
        Ok(buf.len())
    }
    
    /// /dev/zero - read returns zeros
    fn zero_read(buf: &mut [u8]) -> Result<usize, FsError> {
        for byte in buf.iter_mut() {
            *byte = 0;
        }
        Ok(buf.len())
    }
    
    /// /dev/zero - write discards everything
    fn zero_write(buf: &[u8]) -> Result<usize, FsError> {
        Ok(buf.len())
    }
    
    /// /dev/full - read returns nothing
    fn full_read(_buf: &mut [u8]) -> Result<usize, FsError> {
        Ok(0)
    }
    
    /// /dev/full - write returns ENOSPC
    fn full_write(_buf: &[u8]) -> Result<usize, FsError> {
        Err(FsError::NoSpace)
    }
    
    /// /dev/random - read returns random bytes
    fn random_read(buf: &mut [u8]) -> Result<usize, FsError> {
        // TODO: Use proper RNG
        let mut seed = 0x12345678u32;
        for (i, byte) in buf.iter_mut().enumerate() {
            // Simple LCG random number generator
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            *byte = ((seed >> 16) & 0xFF) as u8;
            seed = seed.wrapping_add(i as u32);
        }
        Ok(buf.len())
    }
    
    /// TTY read
    fn tty_read(buf: &mut [u8]) -> Result<usize, FsError> {
        // TODO: Read from keyboard buffer
        // For now, return nothing
        Ok(0)
    }
    
    /// TTY write
    fn tty_write(buf: &[u8]) -> Result<usize, FsError> {
        // Write to VGA buffer
        use crate::drivers::vga::WRITER;
        let mut writer = WRITER.lock();
        for &byte in buf {
            writer.write_byte(byte);
        }
        Ok(buf.len())
    }
    
    /// Keyboard read
    fn keyboard_read(_buf: &mut [u8]) -> Result<usize, FsError> {
        // TODO: Read from keyboard buffer
        Ok(0)
    }
    
    /// Mouse read
    fn mouse_read(_buf: &mut [u8]) -> Result<usize, FsError> {
        // TODO: Read from mouse buffer
        Ok(0)
    }
    
    /// Read from a device
    pub fn read(&self, ino: u64, buf: &mut [u8]) -> Result<usize, FsError> {
        let inode = self.inodes.get(&ino)
            .ok_or(FsError::NotFound)?;
        
        if let Some(ref device) = inode.device {
            if let Some(read_fn) = device.read {
                return read_fn(buf);
            }
        }
        
        Err(FsError::NotSupported)
    }
    
    /// Write to a device
    pub fn write(&self, ino: u64, buf: &[u8]) -> Result<usize, FsError> {
        let inode = self.inodes.get(&ino)
            .ok_or(FsError::NotFound)?;
        
        if let Some(ref device) = inode.device {
            if let Some(write_fn) = device.write {
                return write_fn(buf);
            }
        }
        
        Err(FsError::NotSupported)
    }
    
    /// Look up a device
    pub fn lookup(&self, path: &str) -> Result<u64, FsError> {
        let device_name = path.trim_start_matches('/');
        
        self.devices.get(device_name)
            .copied()
            .ok_or(FsError::NotFound)
    }
}

/// Global DEVFS instance
pub static DEVFS: Mutex<Option<DevfsSuperblock>> = Mutex::new(None);

/// Initialize DEVFS
pub fn init() {
    *DEVFS.lock() = Some(DevfsSuperblock::new());
}