//! File Systems Module
//! 
//! This module provides file system support for VantisOS including ext4, FAT32,
//! exFAT, journaling, and recovery mechanisms.

pub mod ext4;
pub mod fat32;
pub mod exfat;
pub mod journaling;
pub mod recovery;

pub use ext4::*;
pub use fat32::*;
pub use exfat::*;
pub use journaling::*;
pub use recovery::*;

/// Initialize file systems
pub fn init() {
    // Initialize ext4
    ext4::init();
    
    // Initialize FAT32
    fat32::init();
    
    // Initialize exFAT
    exfat::init();
    
    // Initialize journaling
    journaling::init();
    
    // Initialize recovery
    recovery::init();
}

/// Get file systems information
pub fn get_info() -> &'static str {
    "File Systems v0.7.0"
}