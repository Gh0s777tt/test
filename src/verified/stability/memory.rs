//! Memory Leak Detection Module
//! 
//! This module provides comprehensive memory leak detection and tracking
//! capabilities for identifying memory management issues.

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use spin::Mutex;

/// Memory allocation tracker
pub struct MemoryTracker {
    allocations: Arc<Mutex<BTreeMap<usize, AllocationInfo>>>,
    total_allocated: Arc<Mutex<u64>>,
    total_freed: Arc<Mutex<u64>>,
    leaks_detected: Arc<Mutex<u64>>,
}

/// Allocation information
#[derive(Debug, Clone)]
pub struct AllocationInfo {
    pub address: usize,
    pub size: usize,
    pub timestamp: u64,
    pub caller: Option<usize>,
    pub freed: bool,
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        Self {
            allocations: Arc::new(Mutex::new(BTreeMap::new())),
            total_allocated: Arc::new(Mutex::new(0)),
            total_freed: Arc::new(Mutex::new(0)),
            leaks_detected: Arc::new(Mutex::new(0)),
        }
    }

    /// Track a memory allocation
    pub fn track_allocation(&self, address: usize, size: usize, caller: Option<usize>) {
        let mut allocs = self.allocations.lock();
        let mut total = self.total_allocated.lock();
        
        let info = AllocationInfo {
            address,
            size,
            timestamp: self.current_timestamp(),
            caller,
            freed: false,
        };
        
        allocs.insert(address, info);
        *total += size as u64;
    }

    /// Track a memory deallocation
    pub fn track_deallocation(&self, address: usize) {
        let mut allocs = self.allocations.lock();
        let mut total = self.total_freed.lock();
        
        if let Some(mut info) = allocs.get_mut(&address) {
            if !info.freed {
                info.freed = true;
                *total += info.size as u64;
            } else {
                // Double free detected
            }
        }
    }

    /// Detect memory leaks
    pub fn detect_leaks(&self) -> Vec<AllocationInfo> {
        let allocs = self.allocations.lock();
        let mut leaks = Vec::new();
        let mut leaks_count = self.leaks_detected.lock();
        
        for (_, info) in allocs.iter() {
            if !info.freed {
                leaks.push(info.clone());
                *leaks_count += 1;
            }
        }
        
        leaks
    }

    /// Get memory statistics
    pub fn statistics(&self) -> MemoryStatistics {
        let allocs = self.allocations.lock();
        let total_alloc = *self.total_allocated.lock();
        let total_free = *self.total_freed.lock();
        let leaks = *self.leaks_detected.lock();
        
        let active_allocations = allocs.values().filter(|info| !info.freed).count();
        let total_active_memory: u64 = allocs.values()
            .filter(|info| !info.freed)
            .map(|info| info.size as u64)
            .sum();
        
        MemoryStatistics {
            total_allocated,
            total_freed,
            active_allocations,
            total_active_memory,
            leaks_detected: leaks,
        }
    }

    /// Clear all tracked allocations
    pub fn clear(&self) {
        self.allocations.lock().clear();
        *self.total_allocated.lock() = 0;
        *self.total_freed.lock() = 0;
        *self.leaks_detected.lock() = 0;
    }

    /// Get current timestamp (placeholder)
    fn current_timestamp(&self) -> u64 {
        // In a real implementation, this would return actual time
        0
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory statistics
#[derive(Debug, Clone, Copy)]
pub struct MemoryStatistics {
    pub total_allocated: u64,
    pub total_freed: u64,
    pub active_allocations: usize,
    pub total_active_memory: u64,
    pub leaks_detected: u64,
}

/// Memory leak detection configuration
#[derive(Debug, Clone)]
pub struct MemoryLeakDetectionConfig {
    pub enabled: bool,
    pub check_interval_seconds: u64,
    pub leak_threshold_mb: u64,
    pub auto_cleanup: bool,
}

impl Default for MemoryLeakDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval_seconds: 60,
            leak_threshold_mb: 100,
            auto_cleanup: false,
        }
    }
}

/// Memory leak detector
pub struct MemoryLeakDetector {
    tracker: MemoryTracker,
    config: MemoryLeakDetectionConfig,
}

impl MemoryLeakDetector {
    /// Create a new memory leak detector
    pub fn new(config: MemoryLeakDetectionConfig) -> Self {
        Self {
            tracker: MemoryTracker::new(),
            config,
        }
    }

    /// Check for memory leaks
    pub fn check_for_leaks(&self) -> Vec<AllocationInfo> {
        self.tracker.detect_leaks()
    }

    /// Get memory statistics
    pub fn statistics(&self) -> MemoryStatistics {
        self.tracker.statistics()
    }

    /// Get the tracker
    pub fn tracker(&self) -> &MemoryTracker {
        &self.tracker
    }
}

impl Default for MemoryLeakDetector {
    fn default() -> Self {
        Self::new(MemoryLeakDetectionConfig::default())
    }
}

/// Global memory tracker
static MEMORY_TRACKER: spin::Once<MemoryTracker> = spin::Once::new();

/// Get the global memory tracker
pub fn memory_tracker() -> &'static MemoryTracker {
    MEMORY_TRACKER.call_once(|| MemoryTracker::new())
}

/// Track a memory allocation
pub fn track_allocation(address: usize, size: usize, caller: Option<usize>) {
    memory_tracker().track_allocation(address, size, caller);
}

/// Track a memory deallocation
pub fn track_deallocation(address: usize) {
    memory_tracker().track_deallocation(address);
}

/// Detect memory leaks
pub fn detect_leaks() -> Vec<AllocationInfo> {
    memory_tracker().detect_leaks()
}

/// Get memory statistics
pub fn memory_statistics() -> MemoryStatistics {
    memory_tracker().statistics()
}