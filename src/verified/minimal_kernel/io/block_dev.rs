//! # Block Device I/O
//!
//! This module provides block device I/O for the minimal kernel.

use super::IoError;

/// Block device trait
pub trait BlockDevice {
    /// Read block from device
    fn read_block(&mut self, block: u64, buf: &mut [u8]) -> Result<(), IoError>;
    
    /// Write block to device
    fn write_block(&mut self, block: u64, buf: &[u8]) -> Result<(), IoError>;
    
    /// Flush device
    fn flush(&mut self) -> Result<(), IoError>;
}

/// RAM disk device
pub struct RamDisk {
    /// Disk data
    data: Vec<u8>,
    /// Block size
    block_size: usize,
}

impl RamDisk {
    /// Create new RAM disk
    pub fn new(size: usize, block_size: usize) -> Self {
        Self {
            data: vec![0; size],
            block_size,
        }
    }
    
    /// Initialize RAM disk
    pub fn init() {
        // This is a placeholder - actual implementation would:
        // 1. Allocate memory for disk
        // 2. Initialize disk structures
        // 3. Register device
    }
}

impl BlockDevice for RamDisk {
    fn read_block(&mut self, block: u64, buf: &mut [u8]) -> Result<(), IoError> {
        // Check buffer size
        if buf.len() != self.block_size {
            return Err(IoError::InvalidRequest);
        }
        
        // Calculate offset
        let offset = (block as usize) * self.block_size;
        
        // Check if block is within range
        if offset + self.block_size > self.data.len() {
            return Err(IoError::InvalidRequest);
        }
        
        // Read block
        buf.copy_from_slice(&self.data[offset..offset + self.block_size]);
        
        Ok(())
    }
    
    fn write_block(&mut self, block: u64, buf: &[u8]) -> Result<(), IoError> {
        // Check buffer size
        if buf.len() != self.block_size {
            return Err(IoError::InvalidRequest);
        }
        
        // Calculate offset
        let offset = (block as usize) * self.block_size;
        
        // Check if block is within range
        if offset + self.block_size > self.data.len() {
            return Err(IoError::InvalidRequest);
        }
        
        // Write block
        self.data[offset..offset + self.block_size].copy_from_slice(buf);
        
        Ok(())
    }
    
    fn flush(&mut self) -> Result<(), IoError> {
        // RAM disk doesn't need flushing
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ram_disk() {
        let mut disk = RamDisk::new(4096, 512);
        let mut buf = vec![0u8; 512];
        
        // Write block
        buf[0] = 0x42;
        disk.write_block(0, &buf).unwrap();
        
        // Read block
        let mut read_buf = vec![0u8; 512];
        disk.read_block(0, &mut read_buf).unwrap();
        
        assert_eq!(read_buf[0], 0x42);
    }
}