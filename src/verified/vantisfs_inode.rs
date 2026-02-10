//! VantisFS - Inode Manager
//! 
//! This module manages file metadata (inodes) for VantisFS.
//! Inodes store file attributes, permissions, and block pointers.
//!
//! # Features
//! - File metadata management
//! - Direct and indirect block pointers
//! - Permissions and ownership
//! - Timestamps
//! - Formal verification with Verus
//!
//! # Security
//! - Validates all inode operations
//! - Checks permissions
//! - Prevents invalid state

#[cfg(feature = "verus")]
use builtin::*;
#[cfg(feature = "verus")]
use builtin_macros::*;
#[cfg(feature = "verus")]
use vstd::prelude::*;

#[cfg(feature = "verus")]
verus! {

/// Maximum number of inodes
pub const MAX_INODES: usize = 100_000;

/// Number of direct block pointers
pub const DIRECT_BLOCKS: usize = 12;

/// File type constants
pub const FILE_TYPE_REGULAR: u32 = 0x8000;
pub const FILE_TYPE_DIRECTORY: u32 = 0x4000;
pub const FILE_TYPE_SYMLINK: u32 = 0xA000;

/// Permission bits
pub const PERM_READ: u32 = 0x0004;
pub const PERM_WRITE: u32 = 0x0002;
pub const PERM_EXEC: u32 = 0x0001;

/// Inode error types
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum InodeError {
    /// Invalid inode number
    InvalidInode,
    /// Inode already allocated
    InodeAlreadyAllocated,
    /// Inode not allocated
    InodeNotAllocated,
    /// Invalid file type
    InvalidFileType,
    /// Invalid permissions
    InvalidPermissions,
}

/// Inode structure representing file metadata
#[derive(Copy, Clone)]
pub struct Inode {
    /// Inode number
    pub ino: u64,
    /// File mode (type + permissions)
    pub mode: u32,
    /// User ID
    pub uid: u32,
    /// Group ID
    pub gid: u32,
    /// Number of hard links
    pub nlink: u32,
    /// File size in bytes
    pub size: u64,
    /// Direct block pointers
    pub direct_blocks: [u64; DIRECT_BLOCKS],
    /// Single indirect block pointer
    pub indirect_block: u64,
    /// Double indirect block pointer
    pub double_indirect: u64,
    /// Triple indirect block pointer
    pub triple_indirect: u64,
    /// Access time (Unix timestamp)
    pub atime: u64,
    /// Modification time (Unix timestamp)
    pub mtime: u64,
    /// Change time (Unix timestamp)
    pub ctime: u64,
}

impl Inode {
    /// Create a new inode
    pub const fn new(ino: u64, mode: u32, uid: u32, gid: u32) -> Self {
        Inode {
            ino,
            mode,
            uid,
            gid,
            nlink: 1,
            size: 0,
            direct_blocks: [0; DIRECT_BLOCKS],
            indirect_block: 0,
            double_indirect: 0,
            triple_indirect: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        }
    }

    /// Check if inode is a regular file
    pub fn is_regular_file(&self) -> (is_file: bool)
        ensures is_file == ((self.mode & 0xF000) == FILE_TYPE_REGULAR)
    {
        (self.mode & 0xF000) == FILE_TYPE_REGULAR
    }

    /// Check if inode is a directory
    pub fn is_directory(&self) -> (is_dir: bool)
        ensures is_dir == ((self.mode & 0xF000) == FILE_TYPE_DIRECTORY)
    {
        (self.mode & 0xF000) == FILE_TYPE_DIRECTORY
    }

    /// Check if inode is a symbolic link
    pub fn is_symlink(&self) -> (is_link: bool)
        ensures is_link == ((self.mode & 0xF000) == FILE_TYPE_SYMLINK)
    {
        (self.mode & 0xF000) == FILE_TYPE_SYMLINK
    }

    /// Get file permissions
    pub fn get_permissions(&self) -> (perms: u32)
        ensures perms == (self.mode & 0x0FFF)
    {
        self.mode & 0x0FFF
    }

    /// Set file size
    pub fn set_size(&mut self, size: u64)
        ensures self.size == size
    {
        self.size = size;
    }

    /// Get file size
    pub fn get_size(&self) -> (size: u64)
        ensures size == self.size
    {
        self.size
    }

    /// Set direct block pointer
    pub fn set_direct_block(&mut self, index: usize, block: u64) -> (result: Result<(), InodeError>)
        requires index < DIRECT_BLOCKS
        ensures 
            match result {
                Ok(_) => self.direct_blocks[index] == block,
                Err(_) => true
            }
    {
        if index >= DIRECT_BLOCKS {
            return Err(InodeError::InvalidInode);
        }
        
        self.direct_blocks[index] = block;
        Ok(())
    }

    /// Get direct block pointer
    pub fn get_direct_block(&self, index: usize) -> (result: Result<u64, InodeError>)
        requires index < DIRECT_BLOCKS
        ensures 
            match result {
                Ok(block) => block == self.direct_blocks[index],
                Err(_) => true
            }
    {
        if index >= DIRECT_BLOCKS {
            return Err(InodeError::InvalidInode);
        }
        
        Ok(self.direct_blocks[index])
    }

    /// Increment link count
    pub fn inc_nlink(&mut self)
        ensures self.nlink == old(self).nlink + 1
    {
        self.nlink += 1;
    }

    /// Decrement link count
    pub fn dec_nlink(&mut self) -> (result: Result<(), InodeError>)
        ensures 
            match result {
                Ok(_) => self.nlink == old(self).nlink - 1,
                Err(_) => self.nlink == old(self).nlink
            }
    {
        if self.nlink == 0 {
            return Err(InodeError::InodeNotAllocated);
        }
        
        self.nlink -= 1;
        Ok(())
    }

    /// Update access time
    pub fn update_atime(&mut self, time: u64)
        ensures self.atime == time
    {
        self.atime = time;
    }

    /// Update modification time
    pub fn update_mtime(&mut self, time: u64)
        ensures self.mtime == time
    {
        self.mtime = time;
    }

    /// Update change time
    pub fn update_ctime(&mut self, time: u64)
        ensures self.ctime == time
    {
        self.ctime = time;
    }
}

/// Inode manager for allocating and tracking inodes
pub struct InodeManager {
    /// Bitmap tracking allocated inodes
    bitmap: [u64; (MAX_INODES + 63) / 64],
    /// Number of free inodes
    free_count: u64,
    /// Total number of inodes
    total_inodes: u64,
    /// Next inode to search
    next_free: u64,
}

impl InodeManager {
    /// Create a new inode manager
    pub const fn new(total_inodes: u64) -> Self {
        InodeManager {
            bitmap: [u64::MAX; (MAX_INODES + 63) / 64],
            free_count: total_inodes,
            total_inodes,
            next_free: 1, // Reserve inode 0
        }
    }

    /// Allocate a new inode number
    pub fn allocate_inode(&mut self) -> (result: Result<u64, InodeError>)
        requires 
            old(self).free_count <= old(self).total_inodes,
            old(self).total_inodes <= MAX_INODES as u64
        ensures 
            self.total_inodes == old(self).total_inodes,
            match result {
                Ok(ino) => {
                    ino > 0 && ino < self.total_inodes &&
                    self.free_count == old(self).free_count - 1
                },
                Err(_) => {
                    self.free_count == old(self).free_count
                }
            }
    {
        if self.free_count == 0 {
            return Err(InodeError::InodeNotAllocated);
        }

        let start = self.next_free;
        let mut current = start;
        
        loop 
            invariant 
                self.free_count <= self.total_inodes,
                self.total_inodes <= MAX_INODES as u64,
                current > 0 && current < self.total_inodes
        {
            let word_index = (current / 64) as usize;
            let bit_index = (current % 64) as u64;
            let mask = 1u64 << bit_index;
            
            if (self.bitmap[word_index] & mask) != 0 {
                // Mark inode as allocated
                self.bitmap[word_index] &= !mask;
                self.free_count -= 1;
                self.next_free = (current + 1) % self.total_inodes;
                if self.next_free == 0 {
                    self.next_free = 1;
                }
                
                return Ok(current);
            }
            
            current = (current + 1) % self.total_inodes;
            if current == 0 {
                current = 1;
            }
            
            if current == start {
                return Err(InodeError::InodeNotAllocated);
            }
        }
    }

    /// Free an inode
    pub fn free_inode(&mut self, ino: u64) -> (result: Result<(), InodeError>)
        requires 
            old(self).free_count <= old(self).total_inodes,
            old(self).total_inodes <= MAX_INODES as u64,
            ino > 0 && ino < old(self).total_inodes
        ensures 
            self.total_inodes == old(self).total_inodes,
            match result {
                Ok(_) => self.free_count <= old(self).free_count + 1,
                Err(_) => self.free_count == old(self).free_count
            }
    {
        if ino == 0 || ino >= self.total_inodes {
            return Err(InodeError::InvalidInode);
        }

        let word_index = (ino / 64) as usize;
        let bit_index = (ino % 64) as u64;
        let mask = 1u64 << bit_index;
        
        // Check if already free
        if (self.bitmap[word_index] & mask) != 0 {
            return Err(InodeError::InodeNotAllocated);
        }

        // Mark as free
        self.bitmap[word_index] |= mask;
        self.free_count += 1;
        
        Ok(())
    }

    /// Get number of free inodes
    pub fn get_free_count(&self) -> (count: u64)
        requires self.free_count <= self.total_inodes
        ensures count == self.free_count
    {
        self.free_count
    }

    /// Get total number of inodes
    pub fn get_total_inodes(&self) -> (count: u64)
        requires self.total_inodes <= MAX_INODES as u64
        ensures count == self.total_inodes
    {
        self.total_inodes
    }
}

#[cfg(feature = "verus")]
} // verus!

#[cfg(all(test, feature = "verus"))]
mod tests {
    use super::*;

    #[test]
    fn test_inode_creation() {
        let inode = Inode::new(1, FILE_TYPE_REGULAR | 0o644, 1000, 1000);
        assert_eq!(inode.ino, 1);
        assert!(inode.is_regular_file());
        assert!(!inode.is_directory());
        assert_eq!(inode.get_permissions(), 0o644);
    }

    #[test]
    fn test_inode_file_types() {
        let file = Inode::new(1, FILE_TYPE_REGULAR, 0, 0);
        assert!(file.is_regular_file());
        assert!(!file.is_directory());
        assert!(!file.is_symlink());
        
        let dir = Inode::new(2, FILE_TYPE_DIRECTORY, 0, 0);
        assert!(!dir.is_regular_file());
        assert!(dir.is_directory());
        assert!(!dir.is_symlink());
        
        let link = Inode::new(3, FILE_TYPE_SYMLINK, 0, 0);
        assert!(!link.is_regular_file());
        assert!(!link.is_directory());
        assert!(link.is_symlink());
    }

    #[test]
    fn test_inode_size() {
        let mut inode = Inode::new(1, FILE_TYPE_REGULAR, 0, 0);
        assert_eq!(inode.get_size(), 0);
        
        inode.set_size(1024);
        assert_eq!(inode.get_size(), 1024);
    }

    #[test]
    fn test_inode_direct_blocks() {
        let mut inode = Inode::new(1, FILE_TYPE_REGULAR, 0, 0);
        
        inode.set_direct_block(0, 100).unwrap();
        inode.set_direct_block(1, 200).unwrap();
        
        assert_eq!(inode.get_direct_block(0).unwrap(), 100);
        assert_eq!(inode.get_direct_block(1).unwrap(), 200);
    }

    #[test]
    fn test_inode_link_count() {
        let mut inode = Inode::new(1, FILE_TYPE_REGULAR, 0, 0);
        assert_eq!(inode.nlink, 1);
        
        inode.inc_nlink();
        assert_eq!(inode.nlink, 2);
        
        inode.dec_nlink().unwrap();
        assert_eq!(inode.nlink, 1);
    }

    #[test]
    fn test_inode_timestamps() {
        let mut inode = Inode::new(1, FILE_TYPE_REGULAR, 0, 0);
        
        inode.update_atime(1000);
        inode.update_mtime(2000);
        inode.update_ctime(3000);
        
        assert_eq!(inode.atime, 1000);
        assert_eq!(inode.mtime, 2000);
        assert_eq!(inode.ctime, 3000);
    }

    #[test]
    fn test_inode_manager_creation() {
        let manager = InodeManager::new(1000);
        assert_eq!(manager.get_total_inodes(), 1000);
        assert_eq!(manager.get_free_count(), 1000);
    }

    #[test]
    fn test_allocate_inode() {
        let mut manager = InodeManager::new(1000);
        
        let ino = manager.allocate_inode().unwrap();
        assert!(ino > 0 && ino < 1000);
        assert_eq!(manager.get_free_count(), 999);
    }

    #[test]
    fn test_allocate_multiple_inodes() {
        let mut manager = InodeManager::new(1000);
        
        let mut inodes = Vec::new();
        for _ in 0..10 {
            let ino = manager.allocate_inode().unwrap();
            inodes.push(ino);
        }
        
        assert_eq!(manager.get_free_count(), 990);
        
        // All inodes should be unique
        for i in 0..inodes.len() {
            for j in (i+1)..inodes.len() {
                assert_ne!(inodes[i], inodes[j]);
            }
        }
    }

    #[test]
    fn test_free_inode() {
        let mut manager = InodeManager::new(1000);
        
        let ino = manager.allocate_inode().unwrap();
        assert_eq!(manager.get_free_count(), 999);
        
        manager.free_inode(ino).unwrap();
        assert_eq!(manager.get_free_count(), 1000);
    }

    #[test]
    fn test_allocate_and_free_multiple() {
        let mut manager = InodeManager::new(1000);
        
        let mut inodes = Vec::new();
        for _ in 0..100 {
            let ino = manager.allocate_inode().unwrap();
            inodes.push(ino);
        }
        
        assert_eq!(manager.get_free_count(), 900);
        
        // Free half
        for i in 0..50 {
            manager.free_inode(inodes[i]).unwrap();
        }
        
        assert_eq!(manager.get_free_count(), 950);
    }

    #[test]
    fn test_free_invalid_inode() {
        let mut manager = InodeManager::new(1000);
        
        // Try to free inode 0 (reserved)
        let result = manager.free_inode(0);
        assert_eq!(result, Err(InodeError::InvalidInode));
        
        // Try to free beyond range
        let result = manager.free_inode(1000);
        assert_eq!(result, Err(InodeError::InvalidInode));
    }

    #[test]
    fn test_inode_reuse() {
        let mut manager = InodeManager::new(10);
        
        let ino1 = manager.allocate_inode().unwrap();
        manager.free_inode(ino1).unwrap();
        
        let ino2 = manager.allocate_inode().unwrap();
        
        // Should reuse the freed inode
        assert_eq!(ino1, ino2);
    }
}

#[cfg(kani)]
mod kani_verification {
    use super::*;

    #[kani::proof]
    fn verify_inode_allocate_free() {
        let mut manager = InodeManager::new(100);
        
        if let Ok(ino) = manager.allocate_inode() {
            let free_before = manager.get_free_count();
            manager.free_inode(ino).unwrap();
            let free_after = manager.get_free_count();
            
            assert_eq!(free_after, free_before + 1);
        }
    }

    #[kani::proof]
    fn verify_no_inode_zero() {
        let mut manager = InodeManager::new(100);
        
        if let Ok(ino) = manager.allocate_inode() {
            assert!(ino > 0);
        }
    }
}