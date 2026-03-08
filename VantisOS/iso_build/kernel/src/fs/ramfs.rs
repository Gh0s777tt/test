//! RAMFS - In-Memory File System Implementation
//! Provides temporary file storage in RAM

use super::*;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

/// RAMFS inode implementation
pub struct RamfsInode {
    /// Inode number
    pub ino: u64,
    /// File type
    pub file_type: FileType,
    /// File permissions
    pub mode: Permissions,
    /// File size in bytes
    pub size: usize,
    /// File data
    pub data: Vec<u8>,
    /// Directory entries (if directory)
    pub entries: BTreeMap<String, u64>,
    /// Reference count
    pub refcount: usize,
}

impl RamfsInode {
    /// Create a new RAMFS inode
    pub fn new(ino: u64, file_type: FileType, mode: Permissions) -> Self {
        Self {
            ino,
            file_type,
            mode,
            size: 0,
            data: Vec::new(),
            entries: BTreeMap::new(),
            refcount: 1,
        }
    }
}

/// RAMFS superblock
pub struct RamfsSuperblock {
    /// Root inode number (always 1)
    pub root_ino: u64,
    /// Next available inode number
    pub next_ino: u64,
    /// All inodes
    pub inodes: BTreeMap<u64, RamfsInode>,
    /// Total size
    pub total_size: usize,
    /// Block size (4KB)
    pub block_size: usize,
}

impl RamfsSuperblock {
    /// Create a new RAMFS superblock
    pub fn new() -> Self {
        let mut sb = Self {
            root_ino: 1,
            next_ino: 2,
            inodes: BTreeMap::new(),
            total_size: 0,
            block_size: 4096,
        };
        
        // Create root directory
        let root = RamfsInode::new(1, FileType::Directory, Permissions::all());
        sb.inodes.insert(1, root);
        sb
    }
    
    /// Allocate a new inode number
    pub fn alloc_ino(&mut self) -> u64 {
        let ino = self.next_ino;
        self.next_ino += 1;
        ino
    }
    
    /// Create a new file
    pub fn create_file(&mut self, parent_ino: u64, name: &str, mode: Permissions) -> Result<u64, FsError> {
        // Check if parent exists and is a directory
        {
            let parent = self.inodes.get(&parent_ino)
                .ok_or(FsError::NotFound)?;
            
            if parent.file_type != FileType::Directory {
                return Err(FsError::NotDirectory);
            }
            
            // Check if name already exists
            if parent.entries.contains_key(name) {
                return Err(FsError::AlreadyExists);
            }
        }
        
        // Create new inode
        let ino = self.alloc_ino();
        let inode = RamfsInode::new(ino, FileType::RegularFile, mode);
        
        // Add to parent directory
        if let Some(parent) = self.inodes.get_mut(&parent_ino) {
            parent.entries.insert(String::from(name), ino);
        }
        
        // Store inode
        self.inodes.insert(ino, inode);
        
        Ok(ino)
    }
    
    /// Create a new directory
    pub fn create_dir(&mut self, parent_ino: u64, name: &str, mode: Permissions) -> Result<u64, FsError> {
        // Check if parent exists and is a directory
        {
            let parent = self.inodes.get(&parent_ino)
                .ok_or(FsError::NotFound)?;
            
            if parent.file_type != FileType::Directory {
                return Err(FsError::NotDirectory);
            }
            
            if parent.entries.contains_key(name) {
                return Err(FsError::AlreadyExists);
            }
        }
        
        let ino = self.alloc_ino();
        let mut inode = RamfsInode::new(ino, FileType::Directory, mode);
        
        // Add . and .. entries
        inode.entries.insert(String::from("."), ino);
        inode.entries.insert(String::from(".."), parent_ino);
        
        // Add to parent directory
        if let Some(parent) = self.inodes.get_mut(&parent_ino) {
            parent.entries.insert(String::from(name), ino);
        }
        
        self.inodes.insert(ino, inode);
        
        Ok(ino)
    }
    
    /// Read from an inode
    pub fn read(&self, ino: u64, offset: usize, buf: &mut [u8]) -> Result<usize, FsError> {
        let inode = self.inodes.get(&ino)
            .ok_or(FsError::NotFound)?;
        
        if offset >= inode.data.len() {
            return Ok(0);
        }
        
        let end = core::cmp::min(offset + buf.len(), inode.data.len());
        let count = end - offset;
        buf[..count].copy_from_slice(&inode.data[offset..end]);
        
        Ok(count)
    }
    
    /// Write to an inode
    pub fn write(&mut self, ino: u64, offset: usize, buf: &[u8]) -> Result<usize, FsError> {
        let inode = self.inodes.get_mut(&ino)
            .ok_or(FsError::NotFound)?;
        
        // Extend file if necessary
        let new_size = offset + buf.len();
        if new_size > inode.data.len() {
            inode.data.resize(new_size, 0);
        }
        
        inode.data[offset..new_size].copy_from_slice(buf);
        inode.size = inode.data.len();
        
        Ok(buf.len())
    }
    
    /// Delete a file
    pub fn unlink(&mut self, parent_ino: u64, name: &str) -> Result<(), FsError> {
        let parent = self.inodes.get_mut(&parent_ino)
            .ok_or(FsError::NotFound)?;
        
        let ino = parent.entries.remove(name)
            .ok_or(FsError::NotFound)?;
        
        self.inodes.remove(&ino);
        
        Ok(())
    }
    
    /// Look up a path
    pub fn lookup(&self, path: &str) -> Result<u64, FsError> {
        let mut current_ino = self.root_ino;
        
        for component in path.split('/').filter(|s| !s.is_empty()) {
            let inode = self.inodes.get(&current_ino)
                .ok_or(FsError::NotFound)?;
            
            if inode.file_type != FileType::Directory {
                return Err(FsError::NotDirectory);
            }
            
            current_ino = *inode.entries.get(component)
                .ok_or(FsError::NotFound)?;
        }
        
        Ok(current_ino)
    }
}

/// Global RAMFS instance
pub static RAMFS: Mutex<Option<RamfsSuperblock>> = Mutex::new(None);

/// Initialize RAMFS
pub fn init() {
    *RAMFS.lock() = Some(RamfsSuperblock::new());
}

/// Get the RAMFS instance
pub fn get_ramfs() -> &'static Mutex<Option<RamfsSuperblock>> {
    &RAMFS
}