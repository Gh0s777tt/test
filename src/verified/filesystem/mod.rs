// Filesystem Module - VantisOS
//
// This module provides filesystem functionality for VantisOS, including:
// - Virtual File System (VFS) layer
// - VantisFS implementation
// - File system utilities

pub mod vfs;

pub use vfs::{
    Vfs,
    VfsInode,
    VfsFileDescriptor,
    VfsMountPoint,
    VfsOperations,
    FileType,
    FilePermissions,
    FileAttributes,
};

/// Filesystem initialization
pub fn init() {
    vfs::init();
}