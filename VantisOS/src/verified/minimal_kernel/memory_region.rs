// Memory Region Management
//
// This module provides memory region management for VantisOS, including:
// - Memory region tracking
// - Memory region allocation
// - Memory region protection
// - Memory region statistics

#![no_std]

use core::sync::atomic::{AtomicU64, Ordering};

/// Memory region types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    /// Available memory
    Available,
    /// Reserved memory
    Reserved,
    /// Kernel code
    KernelCode,
    /// Kernel data
    KernelData,
    /// Kernel stack
    KernelStack,
    /// User code
    UserCode,
    /// User data
    UserData,
    /// User stack
    UserStack,
    /// Device memory
    DeviceMemory,
    /// ACPI memory
    AcpiMemory,
    /// ACPI NVS memory
    AcpiNvsMemory,
    /// Unusable memory
    Unusable,
}

/// Memory region flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegionFlags {
    /// Readable
    pub readable: bool,
    /// Writable
    pub writable: bool,
    /// Executable
    pub executable: bool,
    /// Cacheable
    pub cacheable: bool,
    /// User accessible
    pub user_accessible: bool,
}

impl Default for MemoryRegionFlags {
    fn default() -> Self {
        MemoryRegionFlags {
            readable: true,
            writable: true,
            executable: false,
            cacheable: true,
            user_accessible: false,
        }
    }
}

/// Memory region
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    /// Start address
    pub start: u64,
    /// End address
    pub end: u64,
    /// Region type
    pub region_type: MemoryRegionType,
    /// Region flags
    pub flags: MemoryRegionFlags,
    /// Region name (for debugging)
    pub name: &'static str,
}

impl MemoryRegion {
    /// Create a new memory region
    pub fn new(start: u64, end: u64, region_type: MemoryRegionType, name: &'static str) -> Self {
        MemoryRegion {
            start,
            end,
            region_type,
            flags: MemoryRegionFlags::default(),
            name,
        }
    }

    /// Create a new memory region with flags
    pub fn with_flags(
        start: u64,
        end: u64,
        region_type: MemoryRegionType,
        flags: MemoryRegionFlags,
        name: &'static str,
    ) -> Self {
        MemoryRegion {
            start,
            end,
            region_type,
            flags,
            name,
        }
    }

    /// Get region size
    pub fn size(&self) -> u64 {
        self.end - self.start
    }

    /// Check if region contains address
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.start && addr < self.end
    }

    /// Check if region overlaps with another region
    pub fn overlaps(&self, other: &MemoryRegion) -> bool {
        self.start < other.end && self.end > other.start
    }

    /// Check if region is available
    pub fn is_available(&self) -> bool {
        self.region_type == MemoryRegionType::Available
    }

    /// Check if region is reserved
    pub fn is_reserved(&self) -> bool {
        self.region_type == MemoryRegionType::Reserved
    }

    /// Check if region is kernel memory
    pub fn is_kernel(&self) -> bool {
        matches!(
            self.region_type,
            MemoryRegionType::KernelCode
                | MemoryRegionType::KernelData
                | MemoryRegionType::KernelStack
        )
    }

    /// Check if region is user memory
    pub fn is_user(&self) -> bool {
        matches!(
            self.region_type,
            MemoryRegionType::UserCode
                | MemoryRegionType::UserData
                | MemoryRegionType::UserStack
        )
    }
}

/// Memory region manager
pub struct MemoryRegionManager {
    /// Memory regions
    regions: [Option<MemoryRegion>; 256],
    /// Number of regions
    region_count: usize,
    /// Total available memory
    total_available: AtomicU64,
    /// Total reserved memory
    total_reserved: AtomicU64,
    /// Total kernel memory
    total_kernel: AtomicU64,
    /// Total user memory
    total_user: AtomicU64,
}

impl MemoryRegionManager {
    /// Create a new memory region manager
    pub fn new() -> Self {
        MemoryRegionManager {
            regions: [None; 256],
            region_count: 0,
            total_available: AtomicU64::new(0),
            total_reserved: AtomicU64::new(0),
            total_kernel: AtomicU64::new(0),
            total_user: AtomicU64::new(0),
        }
    }

    /// Add a memory region
    pub fn add_region(&mut self, region: MemoryRegion) -> bool {
        if self.region_count >= 256 {
            return false;
        }

        // Check for overlaps
        for i in 0..self.region_count {
            if let Some(existing) = self.regions[i] {
                if region.overlaps(&existing) {
                    return false;
                }
            }
        }

        // Add region
        self.regions[self.region_count] = Some(region);
        self.region_count += 1;

        // Update statistics
        self.update_statistics();

        true
    }

    /// Remove a memory region
    pub fn remove_region(&mut self, start: u64) -> bool {
        for i in 0..self.region_count {
            if let Some(region) = self.regions[i] {
                if region.start == start {
                    // Shift remaining regions
                    for j in i..self.region_count - 1 {
                        self.regions[j] = self.regions[j + 1];
                    }
                    self.regions[self.region_count - 1] = None;
                    self.region_count -= 1;

                    // Update statistics
                    self.update_statistics();

                    return true;
                }
            }
        }
        false
    }

    /// Find region containing address
    pub fn find_region(&self, addr: u64) -> Option<MemoryRegion> {
        for i in 0..self.region_count {
            if let Some(region) = self.regions[i] {
                if region.contains(addr) {
                    return Some(region);
                }
            }
        }
        None
    }

    /// Find available region of at least given size
    pub fn find_available_region(&self, size: u64) -> Option<MemoryRegion> {
        for i in 0..self.region_count {
            if let Some(region) = self.regions[i] {
                if region.is_available() && region.size() >= size {
                    return Some(region);
                }
            }
        }
        None
    }

    /// Allocate region from available memory
    pub fn allocate_region(&mut self, size: u64, region_type: MemoryRegionType, name: &'static str) -> Option<u64> {
        // Find available region
        let region = self.find_available_region(size)?;
        let start = region.start;

        // Remove old region
        self.remove_region(start);

        // Create new allocated region
        let allocated = MemoryRegion::new(start, start + size, region_type, name);
        self.add_region(allocated);

        // Create remaining available region
        if region.size() > size {
            let remaining = MemoryRegion::new(
                start + size,
                region.end,
                MemoryRegionType::Available,
                "Available",
            );
            self.add_region(remaining);
        }

        Some(start)
    }

    /// Free region
    pub fn free_region(&mut self, start: u64) -> bool {
        for i in 0..self.region_count {
            if let Some(region) = self.regions[i] {
                if region.start == start && !region.is_available() {
                    // Remove old region
                    self.remove_region(start);

                    // Create new available region
                    let available = MemoryRegion::new(
                        region.start,
                        region.end,
                        MemoryRegionType::Available,
                        "Available",
                    );
                    self.add_region(available);

                    // Try to merge with adjacent available regions
                    self.merge_available_regions();

                    return true;
                }
            }
        }
        false
    }

    /// Merge adjacent available regions
    fn merge_available_regions(&mut self) {
        let mut merged = true;
        while merged {
            merged = false;
            for i in 0..self.region_count {
                for j in i + 1..self.region_count {
                    if let (Some(region1), Some(region2)) = (self.regions[i], self.regions[j]) {
                        if region1.is_available() && region2.is_available() {
                            if region1.end == region2.start {
                                // Merge region1 and region2
                                let merged_region = MemoryRegion::new(
                                    region1.start,
                                    region2.end,
                                    MemoryRegionType::Available,
                                    "Available",
                                );
                                self.remove_region(region1.start);
                                self.remove_region(region2.start);
                                self.add_region(merged_region);
                                merged = true;
                                break;
                            } else if region2.end == region1.start {
                                // Merge region2 and region1
                                let merged_region = MemoryRegion::new(
                                    region2.start,
                                    region1.end,
                                    MemoryRegionType::Available,
                                    "Available",
                                );
                                self.remove_region(region2.start);
                                self.remove_region(region1.start);
                                self.add_region(merged_region);
                                merged = true;
                                break;
                            }
                        }
                    }
                    if merged {
                        break;
                    }
                }
                if merged {
                    break;
                }
            }
        }
    }

    /// Update statistics
    fn update_statistics(&self) {
        let mut available = 0u64;
        let mut reserved = 0u64;
        let mut kernel = 0u64;
        let mut user = 0u64;

        for i in 0..self.region_count {
            if let Some(region) = self.regions[i] {
                let size = region.size();
                match region.region_type {
                    MemoryRegionType::Available => available += size,
                    MemoryRegionType::Reserved => reserved += size,
                    MemoryRegionType::KernelCode
                    | MemoryRegionType::KernelData
                    | MemoryRegionType::KernelStack => kernel += size,
                    MemoryRegionType::UserCode
                    | MemoryRegionType::UserData
                    | MemoryRegionType::UserStack => user += size,
                    _ => {}
                }
            }
        }

        self.total_available.store(available, Ordering::SeqCst);
        self.total_reserved.store(reserved, Ordering::SeqCst);
        self.total_kernel.store(kernel, Ordering::SeqCst);
        self.total_user.store(user, Ordering::SeqCst);
    }

    /// Get total available memory
    pub fn get_total_available(&self) -> u64 {
        self.total_available.load(Ordering::SeqCst)
    }

    /// Get total reserved memory
    pub fn get_total_reserved(&self) -> u64 {
        self.total_reserved.load(Ordering::SeqCst)
    }

    /// Get total kernel memory
    pub fn get_total_kernel(&self) -> u64 {
        self.total_kernel.load(Ordering::SeqCst)
    }

    /// Get total user memory
    pub fn get_total_user(&self) -> u64 {
        self.total_user.load(Ordering::SeqCst)
    }

    /// Get number of regions
    pub fn get_region_count(&self) -> usize {
        self.region_count
    }

    /// Get region at index
    pub fn get_region(&self, index: usize) -> Option<MemoryRegion> {
        if index < self.region_count {
            self.regions[index]
        } else {
            None
        }
    }

    /// Get all regions
    pub fn get_regions(&self) -> &[Option<MemoryRegion>; 256] {
        &self.regions
    }

    /// Print memory region map
    pub fn print_region_map(&self) {
        // TODO: Implement printing when console is available
        // For now, this is a placeholder
    }
}

impl Default for MemoryRegionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_region_creation() {
        let region = MemoryRegion::new(0x1000, 0x2000, MemoryRegionType::Available, "Test");
        assert_eq!(region.start, 0x1000);
        assert_eq!(region.end, 0x2000);
        assert_eq!(region.size(), 0x1000);
    }

    #[test]
    fn test_memory_region_contains() {
        let region = MemoryRegion::new(0x1000, 0x2000, MemoryRegionType::Available, "Test");
        assert!(region.contains(0x1000));
        assert!(region.contains(0x1500));
        assert!(!region.contains(0x2000));
        assert!(!region.contains(0x0FFF));
    }

    #[test]
    fn test_memory_region_overlaps() {
        let region1 = MemoryRegion::new(0x1000, 0x2000, MemoryRegionType::Available, "Test1");
        let region2 = MemoryRegion::new(0x1500, 0x2500, MemoryRegionType::Available, "Test2");
        let region3 = MemoryRegion::new(0x2000, 0x3000, MemoryRegionType::Available, "Test3");
        
        assert!(region1.overlaps(&region2));
        assert!(!region1.overlaps(&region3));
    }

    #[test]
    fn test_memory_region_manager() {
        let mut manager = MemoryRegionManager::new();
        
        let region1 = MemoryRegion::new(0x1000, 0x2000, MemoryRegionType::Available, "Test1");
        let region2 = MemoryRegion::new(0x2000, 0x3000, MemoryRegionType::Available, "Test2");
        
        assert!(manager.add_region(region1));
        assert!(manager.add_region(region2));
        assert_eq!(manager.get_region_count(), 2);
    }

    #[test]
    fn test_memory_region_allocation() {
        let mut manager = MemoryRegionManager::new();
        
        let region = MemoryRegion::new(0x1000, 0x3000, MemoryRegionType::Available, "Available");
        manager.add_region(region);
        
        let addr = manager.allocate_region(0x1000, MemoryRegionType::UserCode, "User");
        assert_eq!(addr, Some(0x1000));
        assert_eq!(manager.get_region_count(), 2); // Allocated + remaining
    }

    #[test]
    fn test_memory_region_free() {
        let mut manager = MemoryRegionManager::new();
        
        let region = MemoryRegion::new(0x1000, 0x2000, MemoryRegionType::UserCode, "User");
        manager.add_region(region);
        
        assert!(manager.free_region(0x1000));
        assert_eq!(manager.get_region_count(), 1);
        assert!(manager.get_region(0).unwrap().unwrap().is_available());
    }
}