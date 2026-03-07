//! Virtual filesystem for VantisOS

use spin::Mutex;

/// File types
#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
}

/// Initialize VFS
pub fn init() {
    // VFS initialization
}