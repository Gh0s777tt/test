// Memory Statistics and Tracking
//
// This module provides memory statistics and tracking for VantisOS, including:
// - Memory allocation tracking
// - Memory usage statistics
// - Memory leak detection
// - Memory performance metrics

#![no_std]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Memory allocation statistics
pub struct AllocationStats {
    /// Total allocations
    total_allocations: AtomicU64,
    /// Total deallocations
    total_deallocations: AtomicU64,
    /// Current allocations
    current_allocations: AtomicU64,
    /// Peak allocations
    peak_allocations: AtomicU64,
    /// Total bytes allocated
    total_bytes_allocated: AtomicU64,
    /// Total bytes deallocated
    total_bytes_deallocated: AtomicU64,
    /// Current bytes allocated
    current_bytes_allocated: AtomicU64,
    /// Peak bytes allocated
    peak_bytes_allocated: AtomicU64,
}

impl AllocationStats {
    /// Create new allocation statistics
    pub const fn new() -> Self {
        AllocationStats {
            total_allocations: AtomicU64::new(0),
            total_deallocations: AtomicU64::new(0),
            current_allocations: AtomicU64::new(0),
            peak_allocations: AtomicU64::new(0),
            total_bytes_allocated: AtomicU64::new(0),
            total_bytes_deallocated: AtomicU64::new(0),
            current_bytes_allocated: AtomicU64::new(0),
            peak_bytes_allocated: AtomicU64::new(0),
        }
    }

    /// Record allocation
    pub fn record_allocation(&self, size: u64) {
        self.total_allocations.fetch_add(1, Ordering::SeqCst);
        self.total_bytes_allocated.fetch_add(size, Ordering::SeqCst);
        
        let current = self.current_allocations.fetch_add(1, Ordering::SeqCst) + 1;
        let peak = self.peak_allocations.load(Ordering::SeqCst);
        if current > peak {
            self.peak_allocations.store(current, Ordering::SeqCst);
        }
        
        let current_bytes = self.current_bytes_allocated.fetch_add(size, Ordering::SeqCst) + size;
        let peak_bytes = self.peak_bytes_allocated.load(Ordering::SeqCst);
        if current_bytes > peak_bytes {
            self.peak_bytes_allocated.store(current_bytes, Ordering::SeqCst);
        }
    }

    /// Record deallocation
    pub fn record_deallocation(&self, size: u64) {
        self.total_deallocations.fetch_add(1, Ordering::SeqCst);
        self.total_bytes_deallocated.fetch_add(size, Ordering::SeqCst);
        self.current_allocations.fetch_sub(1, Ordering::SeqCst);
        self.current_bytes_allocated.fetch_sub(size, Ordering::SeqCst);
    }

    /// Get total allocations
    pub fn get_total_allocations(&self) -> u64 {
        self.total_allocations.load(Ordering::SeqCst)
    }

    /// Get total deallocations
    pub fn get_total_deallocations(&self) -> u64 {
        self.total_deallocations.load(Ordering::SeqCst)
    }

    /// Get current allocations
    pub fn get_current_allocations(&self) -> u64 {
        self.current_allocations.load(Ordering::SeqCst)
    }

    /// Get peak allocations
    pub fn get_peak_allocations(&self) -> u64 {
        self.peak_allocations.load(Ordering::SeqCst)
    }

    /// Get total bytes allocated
    pub fn get_total_bytes_allocated(&self) -> u64 {
        self.total_bytes_allocated.load(Ordering::SeqCst)
    }

    /// Get total bytes deallocated
    pub fn get_total_bytes_deallocated(&self) -> u64 {
        self.total_bytes_deallocated.load(Ordering::SeqCst)
    }

    /// Get current bytes allocated
    pub fn get_current_bytes_allocated(&self) -> u64 {
        self.current_bytes_allocated.load(Ordering::SeqCst)
    }

    /// Get peak bytes allocated
    pub fn get_peak_bytes_allocated(&self) -> u64 {
        self.peak_bytes_allocated.load(Ordering::SeqCst)
    }

    /// Check for memory leak
    pub fn has_memory_leak(&self) -> bool {
        self.current_allocations.load(Ordering::SeqCst) > 0
    }

    /// Get leak size
    pub fn get_leak_size(&self) -> u64 {
        self.current_bytes_allocated.load(Ordering::SeqCst)
    }

    /// Reset statistics
    pub fn reset(&self) {
        self.total_allocations.store(0, Ordering::SeqCst);
        self.total_deallocations.store(0, Ordering::SeqCst);
        self.current_allocations.store(0, Ordering::SeqCst);
        self.peak_allocations.store(0, Ordering::SeqCst);
        self.total_bytes_allocated.store(0, Ordering::SeqCst);
        self.total_bytes_deallocated.store(0, Ordering::SeqCst);
        self.current_bytes_allocated.store(0, Ordering::SeqCst);
        self.peak_bytes_allocated.store(0, Ordering::SeqCst);
    }
}

impl Default for AllocationStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory allocation record
#[derive(Debug, Clone, Copy)]
pub struct AllocationRecord {
    /// Address
    pub address: u64,
    /// Size
    pub size: u64,
    /// Allocation time (ticks)
    pub allocation_time: u64,
    /// Allocation type
    pub allocation_type: AllocationType,
}

/// Allocation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationType {
    /// Page allocation
    Page,
    /// Slab allocation
    Slab,
    /// Kernel heap allocation
    KernelHeap,
    /// User heap allocation
    UserHeap,
    /// Stack allocation
    Stack,
}

/// Memory tracker
pub struct MemoryTracker {
    /// Allocation records
    records: [Option<AllocationRecord>; 1024],
    /// Record count
    record_count: AtomicUsize,
    /// Statistics
    stats: AllocationStats,
}

impl MemoryTracker {
    /// Create new memory tracker
    pub fn new() -> Self {
        MemoryTracker {
            records: [None; 1024],
            record_count: AtomicUsize::new(0),
            stats: AllocationStats::new(),
        }
    }

    /// Track allocation
    pub fn track_allocation(&mut self, address: u64, size: u64, allocation_type: AllocationType, time: u64) -> bool {
        if self.record_count.load(Ordering::SeqCst) >= 1024 {
            return false;
        }

        let record = AllocationRecord {
            address,
            size,
            allocation_time: time,
            allocation_type,
        };

        self.records[self.record_count.load(Ordering::SeqCst)] = Some(record);
        self.record_count.fetch_add(1, Ordering::SeqCst);
        self.stats.record_allocation(size);

        true
    }

    /// Track deallocation
    pub fn track_deallocation(&mut self, address: u64) -> bool {
        for i in 0..self.record_count.load(Ordering::SeqCst) {
            if let Some(record) = self.records[i] {
                if record.address == address {
                    self.stats.record_deallocation(record.size);
                    
                    // Remove record
                    for j in i..self.record_count.load(Ordering::SeqCst) - 1 {
                        self.records[j] = self.records[j + 1];
                    }
                    self.records[self.record_count.load(Ordering::SeqCst) - 1] = None;
                    self.record_count.fetch_sub(1, Ordering::SeqCst);
                    
                    return true;
                }
            }
        }
        false
    }

    /// Find allocation record
    pub fn find_allocation(&self, address: u64) -> Option<AllocationRecord> {
        for i in 0..self.record_count.load(Ordering::SeqCst) {
            if let Some(record) = self.records[i] {
                if record.address == address {
                    return Some(record);
                }
            }
        }
        None
    }

    /// Get statistics
    pub fn get_stats(&self) -> &AllocationStats {
        &self.stats
    }

    /// Get record count
    pub fn get_record_count(&self) -> usize {
        self.record_count.load(Ordering::SeqCst)
    }

    /// Get all records
    pub fn get_records(&self) -> &[Option<AllocationRecord>; 1024] {
        &self.records
    }

    /// Check for memory leak
    pub fn has_memory_leak(&self) -> bool {
        self.stats.has_memory_leak()
    }

    /// Get leak size
    pub fn get_leak_size(&self) -> u64 {
        self.stats.get_leak_size()
    }

    /// Print allocation records
    pub fn print_records(&self) {
        // TODO: Implement printing when console is available
        // For now, this is a placeholder
    }

    /// Reset tracker
    pub fn reset(&mut self) {
        self.records = [None; 1024];
        self.record_count.store(0, Ordering::SeqCst);
        self.stats.reset();
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory performance metrics
pub struct MemoryPerformanceMetrics {
    /// Allocation time (ticks)
    total_allocation_time: AtomicU64,
    /// Deallocation time (ticks)
    total_deallocation_time: AtomicU64,
    /// Allocation count
    allocation_count: AtomicU64,
    /// Deallocation count
    deallocation_count: AtomicU64,
}

impl MemoryPerformanceMetrics {
    /// Create new memory performance metrics
    pub const fn new() -> Self {
        MemoryPerformanceMetrics {
            total_allocation_time: AtomicU64::new(0),
            total_deallocation_time: AtomicU64::new(0),
            allocation_count: AtomicU64::new(0),
            deallocation_count: AtomicU64::new(0),
        }
    }

    /// Record allocation time
    pub fn record_allocation_time(&self, time: u64) {
        self.total_allocation_time.fetch_add(time, Ordering::SeqCst);
        self.allocation_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Record deallocation time
    pub fn record_deallocation_time(&self, time: u64) {
        self.total_deallocation_time.fetch_add(time, Ordering::SeqCst);
        self.deallocation_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Get average allocation time
    pub fn get_average_allocation_time(&self) -> u64 {
        let count = self.allocation_count.load(Ordering::SeqCst);
        if count == 0 {
            return 0;
        }
        self.total_allocation_time.load(Ordering::SeqCst) / count
    }

    /// Get average deallocation time
    pub fn get_average_deallocation_time(&self) -> u64 {
        let count = self.deallocation_count.load(Ordering::SeqCst);
        if count == 0 {
            return 0;
        }
        self.total_deallocation_time.load(Ordering::SeqCst) / count
    }

    /// Get total allocation time
    pub fn get_total_allocation_time(&self) -> u64 {
        self.total_allocation_time.load(Ordering::SeqCst)
    }

    /// Get total deallocation time
    pub fn get_total_deallocation_time(&self) -> u64 {
        self.total_deallocation_time.load(Ordering::SeqCst)
    }

    /// Get allocation count
    pub fn get_allocation_count(&self) -> u64 {
        self.allocation_count.load(Ordering::SeqCst)
    }

    /// Get deallocation count
    pub fn get_deallocation_count(&self) -> u64 {
        self.deallocation_count.load(Ordering::SeqCst)
    }

    /// Reset metrics
    pub fn reset(&self) {
        self.total_allocation_time.store(0, Ordering::SeqCst);
        self.total_deallocation_time.store(0, Ordering::SeqCst);
        self.allocation_count.store(0, Ordering::SeqCst);
        self.deallocation_count.store(0, Ordering::SeqCst);
    }
}

impl Default for MemoryPerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Global memory statistics
static mut GLOBAL_ALLOCATION_STATS: Option<AllocationStats> = None;
static mut GLOBAL_MEMORY_TRACKER: Option<MemoryTracker> = None;
static mut GLOBAL_PERFORMANCE_METRICS: Option<MemoryPerformanceMetrics> = None;

/// Initialize global memory statistics
pub fn init_global_stats() {
    unsafe {
        GLOBAL_ALLOCATION_STATS = Some(AllocationStats::new());
        GLOBAL_MEMORY_TRACKER = Some(MemoryTracker::new());
        GLOBAL_PERFORMANCE_METRICS = Some(MemoryPerformanceMetrics::new());
    }
}

/// Get global allocation statistics
pub fn get_global_stats() -> Option<&'static AllocationStats> {
    unsafe { GLOBAL_ALLOCATION_STATS.as_ref() }
}

/// Get global memory tracker
pub fn get_global_tracker() -> Option<&'static mut MemoryTracker> {
    unsafe { GLOBAL_MEMORY_TRACKER.as_mut() }
}

/// Get global performance metrics
pub fn get_global_metrics() -> Option<&'static MemoryPerformanceMetrics> {
    unsafe { GLOBAL_PERFORMANCE_METRICS.as_ref() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_stats() {
        let stats = AllocationStats::new();
        
        stats.record_allocation(1024);
        stats.record_allocation(2048);
        stats.record_deallocation(1024);
        
        assert_eq!(stats.get_total_allocations(), 2);
        assert_eq!(stats.get_total_deallocations(), 1);
        assert_eq!(stats.get_current_allocations(), 1);
        assert_eq!(stats.get_current_bytes_allocated(), 2048);
    }

    #[test]
    fn test_memory_tracker() {
        let mut tracker = MemoryTracker::new();
        
        assert!(tracker.track_allocation(0x1000, 4096, AllocationType::Page, 100));
        assert!(tracker.track_allocation(0x2000, 4096, AllocationType::Page, 200));
        
        assert_eq!(tracker.get_record_count(), 2);
        assert!(tracker.find_allocation(0x1000).is_some());
        
        assert!(tracker.track_deallocation(0x1000));
        assert_eq!(tracker.get_record_count(), 1);
    }

    #[test]
    fn test_memory_performance_metrics() {
        let metrics = MemoryPerformanceMetrics::new();
        
        metrics.record_allocation_time(100);
        metrics.record_allocation_time(200);
        metrics.record_deallocation_time(50);
        
        assert_eq!(metrics.get_average_allocation_time(), 150);
        assert_eq!(metrics.get_average_deallocation_time(), 50);
        assert_eq!(metrics.get_allocation_count(), 2);
    }

    #[test]
    fn test_memory_leak_detection() {
        let stats = AllocationStats::new();
        
        stats.record_allocation(1024);
        stats.record_allocation(2048);
        
        assert!(stats.has_memory_leak());
        assert_eq!(stats.get_leak_size(), 3072);
        
        stats.record_deallocation(1024);
        stats.record_deallocation(2048);
        
        assert!(!stats.has_memory_leak());
        assert_eq!(stats.get_leak_size(), 0);
    }
}