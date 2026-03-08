//! # Memory Management
//!
//! This module provides memory management functionality for the minimal kernel.
//!
//! It includes:
//! - Page allocator
//! - Slab allocator
//! - Virtual memory management
//! - Memory protection

pub mod page_alloc;
pub mod slab_alloc;
pub mod vmem;
pub mod memory_region;
pub mod memory_protection;
pub mod memory_stats;

use page_alloc::PageAllocator;
use slab_alloc::SlabAllocator;
use vmem::VirtualMemory;

/// Initialize memory management
pub fn init() {
    // Initialize page allocator
    PageAllocator::init();
    
    // Initialize slab allocator
    SlabAllocator::init();
    
    // Initialize virtual memory
    VirtualMemory::init();
}

/// Physical address type
pub type PhysAddr = usize;

/// Virtual address type
pub type VirtAddr = usize;

/// Memory allocation error
#[derive(Debug, Clone, Copy)]
pub enum AllocError {
    OutOfMemory,
    InvalidAddress,
    AlignmentError,
}

/// Memory protection flags
#[derive(Debug, Clone, Copy)]
pub struct ProtectionFlags {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

impl Default for ProtectionFlags {
    fn default() -> Self {
        Self {
            readable: true,
            writable: false,
            executable: false,
        }
    }
}

impl ProtectionFlags {
    /// Create read-only protection
    pub fn read_only() -> Self {
        Self {
            readable: true,
            writable: false,
            executable: false,
        }
    }
    
    /// Create read-write protection
    pub fn read_write() -> Self {
        Self {
            readable: true,
            writable: true,
            executable: false,
        }
    }
    
    /// Create read-execute protection
    pub fn read_execute() -> Self {
        Self {
            readable: true,
            writable: false,
            executable: true,
        }
    }
    
    /// Create read-write-execute protection
    pub fn read_write_execute() -> Self {
        Self {
            readable: true,
            writable: true,
            executable: true,
        }
    }
}