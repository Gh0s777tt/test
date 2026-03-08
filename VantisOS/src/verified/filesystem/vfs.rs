// Virtual File System (VFS) - VantisOS
//
// This module implements the Virtual File System layer for VantisOS,
// providing a unified interface for different file systems.

use alloc::vec::Vec;
use alloc::sync::Arc;
use alloc::string::String;
use spin::Mutex;

/// File types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FileType {
    RegularFile = 0,
    Directory = 1,
    CharacterDevice = 2,
    BlockDevice = 3,
    NamedPipe = 4,
    SymbolicLink = 5,
    Socket = 6,
}

/// File permissions (Unix-style)
#[derive(Debug, Clone, Copy)]
pub struct FilePermissions {
    pub user_read: bool,
    pub user_write: bool,
    pub user_execute: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,
}

impl FilePermissions {
    /// Create a new file permissions
    pub fn new() -> Self {
        Self {
            user_read: true,
            user_write: true,
            user_execute: false,
            group_read: true,
            group_write: false,
            group_execute: false,
            other_read: true,
            other_write: false,
            other_execute: false,
        }
    }
    
    /// Convert to u16 (Unix mode)
    pub fn to_mode(&self) -> u16 {
        let mut mode = 0u16;
        
        if self.user_read { mode |= 0o400; }
        if self.user_write { mode |= 0o200; }
        if self.user_execute { mode |= 0o100; }
        if self.group_read { mode |= 0o040; }
        if self.group_write { mode |= 0o020; }
        if self.group_execute { mode |= 0o010; }
        if self.other_read { mode |= 0o004; }
        if self.other_write { mode |= 0o002; }
        if self.other_execute { mode |= 0o001; }
        
        mode
    }
    
    /// Parse from u16 (Unix mode)
    pub fn from_mode(mode: u16) -> Self {
        Self {
            user_read: mode & 0o400 != 0,
            user_write: mode & 0o200 != 0,
            user_execute: mode & 0o100 != 0,
            group_read: mode & 0o040 != 0,
            group_write: mode & 0o020 != 0,
            group_execute: mode & 0o010 != 0,
            other_read: mode & 0o004 != 0,
            other_write: mode & 0o002 != 0,
            other_execute: mode & 0o001 != 0,
        }
    }
}

/// File attributes
#[derive(Debug, Clone)]
pub struct FileAttributes {
    pub file_type: FileType,
    pub permissions: FilePermissions,
    pub size: u64,
    pub uid: u32,
    pub gid: u32,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
}

impl FileAttributes {
    /// Create a new file attributes
    pub fn new(file_type: FileType) -> Self {
        Self {
            file_type,
            permissions: FilePermissions::new(),
            size: 0,
            uid: 0,
            gid: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        }
    }
}

/// VFS inode
#[derive(Debug, Clone)]
pub struct VfsInode {
    pub inode_number: u64,
    pub attributes: FileAttributes,
    pub data: Vec<u8>,
}

impl VfsInode {
    /// Create a new VFS inode
    pub fn new(inode_number: u64, file_type: FileType) -> Self {
        Self {
            inode_number,
            attributes: FileAttributes::new(file_type),
            data: Vec::new(),
        }
    }
}

/// VFS file descriptor
#[derive(Debug, Clone)]
pub struct VfsFileDescriptor {
    pub fd: i32,
    pub inode_number: u64,
    pub offset: u64,
    pub flags: u32,
}

impl VfsFileDescriptor {
    /// Create a new VFS file descriptor
    pub fn new(fd: i32, inode_number: u64, flags: u32) -> Self {
        Self {
            fd,
            inode_number,
            offset: 0,
            flags,
        }
    }
}

/// VFS operations trait
pub trait VfsOperations: Send + Sync {
    /// Open a file
    fn open(&self, path: &str, flags: u32) -> Result<i32, ()>;
    
    /// Close a file
    fn close(&self, fd: i32) -> Result<(), ()>;
    
    /// Read from a file
    fn read(&self, fd: i32, buffer: &mut [u8]) -> Result<usize, ()>;
    
    /// Write to a file
    fn write(&self, fd: i32, buffer: &[u8]) -> Result<usize, ()>;
    
    /// Seek in a file
    fn seek(&self, fd: i32, offset: i64, whence: u32) -> Result<u64, ()>;
    
    /// Get file attributes
    fn getattr(&self, path: &str) -> Result<FileAttributes, ()>;
    
    /// Set file attributes
    fn setattr(&self, path: &str, attrs: &FileAttributes) -> Result<(), ()>;
    
    /// Create a directory
    fn mkdir(&self, path: &str, mode: u16) -> Result<(), ()>;
    
    /// Remove a directory
    fn rmdir(&self, path: &str) -> Result<(), ()>;
    
    /// List directory entries
    fn readdir(&self, path: &str) -> Result<Vec<String>, ()>;
    
    /// Create a file
    fn create(&self, path: &str, mode: u16) -> Result<i32, ()>;
    
    /// Delete a file
    fn unlink(&self, path: &str) -> Result<(), ()>;
    
    /// Rename a file
    fn rename(&self, old_path: &str, new_path: &str) -> Result<(), ()>;
}

/// VFS mount point
#[derive(Debug, Clone)]
pub struct VfsMountPoint {
    pub mount_point: String,
    pub device: String,
    pub filesystem_type: String,
    pub ops: Arc<dyn VfsOperations>,
}

impl VfsMountPoint {
    /// Create a new VFS mount point
    pub fn new(
        mount_point: String,
        device: String,
        filesystem_type: String,
        ops: Arc<dyn VfsOperations>,
    ) -> Self {
        Self {
            mount_point,
            device,
            filesystem_type,
            ops,
        }
    }
}

/// VFS
pub struct Vfs {
    mount_points: Vec<VfsMountPoint>,
    file_descriptors: Mutex<Vec<Option<VfsFileDescriptor>>>,
    next_fd: Mutex<i32>,
    inodes: Mutex<Vec<VfsInode>>,
    next_inode: Mutex<u64>,
}

impl Vfs {
    /// Create a new VFS
    pub fn new() -> Self {
        Self {
            mount_points: Vec::new(),
            file_descriptors: Mutex::new(Vec::new()),
            next_fd: Mutex::new(3), // Start from 3 (0=stdin, 1=stdout, 2=stderr)
            inodes: Mutex::new(Vec::new()),
            next_inode: Mutex::new(1),
        }
    }
    
    /// Mount a filesystem
    pub fn mount(&mut self, mount_point: String, mount: VfsMountPoint) -> Result<(), ()> {
        self.mount_points.push(mount);
        Ok(())
    }
    
    /// Unmount a filesystem
    pub fn unmount(&mut self, mount_point: &str) -> Result<(), ()> {
        self.mount_points.retain(|m| m.mount_point != mount_point);
        Ok(())
    }
    
    /// Resolve path to mount point
    fn resolve_path(&self, path: &str) -> Option<(Arc<dyn VfsOperations>, String)> {
        // Find the best matching mount point
        let mut best_match: Option<(Arc<dyn VfsOperations>, String)> = None;
        let mut best_match_len = 0;
        
        for mount in &self.mount_points {
            if path.starts_with(&mount.mount_point) {
                let len = mount.mount_point.len();
                if len > best_match_len {
                    best_match = Some((mount.ops.clone(), path[len..].to_string()));
                    best_match_len = len;
                }
            }
        }
        
        best_match
    }
    
    /// Allocate a file descriptor
    fn allocate_fd(&self) -> Result<i32, ()> {
        let mut next_fd = self.next_fd.lock();
        let mut fds = self.file_descriptors.lock();
        
        // Find a free slot
        while *next_fd < fds.len() as i32 {
            if fds[*next_fd as usize].is_none() {
                let fd = *next_fd;
                *next_fd += 1;
                return Ok(fd);
            }
            *next_fd += 1;
        }
        
        // Allocate new slot
        let fd = *next_fd;
        fds.push(None);
        *next_fd += 1;
        
        Ok(fd)
    }
    
    /// Free a file descriptor
    fn free_fd(&self, fd: i32) {
        let mut fds = self.file_descriptors.lock();
        if fd >= 0 && (fd as usize) < fds.len() {
            fds[fd as usize] = None;
        }
    }
    
    /// Allocate an inode
    fn allocate_inode(&self) -> u64 {
        let mut next_inode = self.next_inode.lock();
        let inode = *next_inode;
        *next_inode += 1;
        inode
    }
    
    /// Open a file
    pub fn open(&self, path: &str, flags: u32) -> Result<i32, ()> {
        let (ops, relative_path) = self.resolve_path(path).ok_or(())?;
        let fd = ops.open(&relative_path, flags)?;
        
        let mut fds = self.file_descriptors.lock();
        if fd as usize >= fds.len() {
            fds.resize(fd as usize + 1, None);
        }
        
        fds[fd as usize] = Some(VfsFileDescriptor::new(fd, 0, flags));
        
        Ok(fd)
    }
    
    /// Close a file
    pub fn close(&self, fd: i32) -> Result<(), ()> {
        let (ops, _) = self.resolve_path("").ok_or(())?;
        ops.close(fd)?;
        self.free_fd(fd);
        Ok(())
    }
    
    /// Read from a file
    pub fn read(&self, fd: i32, buffer: &mut [u8]) -> Result<usize, ()> {
        let (ops, _) = self.resolve_path("").ok_or(())?;
        ops.read(fd, buffer)
    }
    
    /// Write to a file
    pub fn write(&self, fd: i32, buffer: &[u8]) -> Result<usize, ()> {
        let (ops, _) = self.resolve_path("").ok_or(())?;
        ops.write(fd, buffer)
    }
    
    /// Seek in a file
    pub fn seek(&self, fd: i32, offset: i64, whence: u32) -> Result<u64, ()> {
        let (ops, _) = self.resolve_path("").ok_or(())?;
        ops.seek(fd, offset, whence)
    }
    
    /// Get file attributes
    pub fn getattr(&self, path: &str) -> Result<FileAttributes, ()> {
        let (ops, relative_path) = self.resolve_path(path).ok_or(())?;
        ops.getattr(&relative_path)
    }
    
    /// Set file attributes
    pub fn setattr(&self, path: &str, attrs: &FileAttributes) -> Result<(), ()> {
        let (ops, relative_path) = self.resolve_path(path).ok_or(())?;
        ops.setattr(&relative_path, attrs)
    }
    
    /// Create a directory
    pub fn mkdir(&self, path: &str, mode: u16) -> Result<(), ()> {
        let (ops, relative_path) = self.resolve_path(path).ok_or(())?;
        ops.mkdir(&relative_path, mode)
    }
    
    /// Remove a directory
    pub fn rmdir(&self, path: &str) -> Result<(), ()> {
        let (ops, relative_path) = self.resolve_path(path).ok_or(())?;
        ops.rmdir(&relative_path)
    }
    
    /// List directory entries
    pub fn readdir(&self, path: &str) -> Result<Vec<String>, ()> {
        let (ops, relative_path) = self.resolve_path(path).ok_or(())?;
        ops.readdir(&relative_path)
    }
    
    /// Create a file
    pub fn create(&self, path: &str, mode: u16) -> Result<i32, ()> {
        let (ops, relative_path) = self.resolve_path(path).ok_or(())?;
        ops.create(&relative_path, mode)
    }
    
    /// Delete a file
    pub fn unlink(&self, path: &str) -> Result<(), ()> {
        let (ops, relative_path) = self.resolve_path(path).ok_or(())?;
        ops.unlink(&relative_path)
    }
    
    /// Rename a file
    pub fn rename(&self, old_path: &str, new_path: &str) -> Result<(), ()> {
        let (old_ops, old_relative) = self.resolve_path(old_path).ok_or(())?;
        let (new_ops, new_relative) = self.resolve_path(new_path).ok_or(())?;
        
        // Check if both paths are on the same filesystem
        if Arc::ptr_eq(&old_ops, &new_ops) {
            old_ops.rename(&old_relative, &new_relative)
        } else {
            Err(())
        }
    }
}

/// Global VFS instance
static mut GLOBAL_VFS: Option<Vfs> = None;

/// Initialize VFS
pub fn init() {
    unsafe {
        GLOBAL_VFS = Some(Vfs::new());
    }
}

/// Get global VFS instance
pub fn get_vfs() -> &'static Vfs {
    unsafe {
        GLOBAL_VFS.as_ref().expect("VFS not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_permissions() {
        let perms = FilePermissions::new();
        assert!(perms.user_read);
        assert!(perms.user_write);
        assert!(!perms.user_execute);
    }
    
    #[test]
    fn test_file_permissions_mode() {
        let perms = FilePermissions::new();
        let mode = perms.to_mode();
        assert_eq!(mode & 0o600, 0o600); // User rw
    }
    
    #[test]
    fn test_vfs_inode_creation() {
        let inode = VfsInode::new(1, FileType::RegularFile);
        assert_eq!(inode.inode_number, 1);
        assert_eq!(inode.attributes.file_type, FileType::RegularFile);
    }
    
    #[test]
    fn test_vfs_file_descriptor() {
        let fd = VfsFileDescriptor::new(3, 1, 0);
        assert_eq!(fd.fd, 3);
        assert_eq!(fd.inode_number, 1);
        assert_eq!(fd.offset, 0);
    }
    
    #[test]
    fn test_vfs_creation() {
        let vfs = Vfs::new();
        assert_eq!(vfs.mount_points.len(), 0);
    }
}