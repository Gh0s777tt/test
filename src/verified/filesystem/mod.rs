// Filesystem Module - VantisOS
//
// This module provides filesystem functionality for VantisOS, including:
// - Virtual File System (VFS) layer
// - VantisFS implementation
// - VantisFS features
// - File system utilities

pub mod vfs;
pub mod vantisfs;
pub mod vantisfs_features;
pub mod vantisfs_advanced;

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

pub use vantisfs::{
    Vantisfs,
    VantisfsSuperblock,
    VantisfsInode,
    VantisfsDirectoryEntry,
    VantisfsBlockAllocator,
    VantisfsInodeAllocator,
};

pub use vantisfs_features::{
    VantisfsSymlink,
    VantisfsHardlink,
    VantisfsExtendedAttributes,
    VantisfsPermissionsManager,
    VantisfsLinkManager,
    VantisfsAttributesManager,
    VantisfsFeatures,
};

/// Filesystem initialization
pub fn init() {
    vfs::init();
    vantisfs::init();
    vantisfs_features::init();
}