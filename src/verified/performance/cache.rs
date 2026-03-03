//! Cache Optimization Module
//! 
//! This module provides cache optimization capabilities including
//! cache-friendly data structures, prefetching, and cache tuning.

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;

/// Cache level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheLevel {
    L1,
    L2,
    L3,
}

/// Cache type
#[derive(Debug, Clone, Copy)]
pub enum CacheType {
    Instruction,
    Data,
    Unified,
}

/// Cache information
#[derive(Debug, Clone, Copy)]
pub struct CacheInfo {
    pub level: CacheLevel,
    pub cache_type: CacheType,
    pub size_kb: u32,
    pub line_size: u32,
    pub associativity: u32,
}

/// Cache optimization strategy
#[derive(Debug, Clone, Copy)]
pub enum CacheOptimizationStrategy {
    Auto,
    Spatial,
    Temporal,
    Both,
}

/// Prefetch type
#[derive(Debug, Clone, Copy)]
pub enum PrefetchType {
    None,
    Spatial,
    Temporal,
}

/// Cache optimizer
pub struct CacheOptimizer {
    strategy: CacheOptimizationStrategy,
    cache_info: Vec<CacheInfo>,
    prefetch_type: PrefetchType,
    prefetch_distance: usize,
}

impl CacheOptimizer {
    /// Create a new cache optimizer
    pub fn new(cache_info: Vec<CacheInfo>) -> Self {
        Self {
            strategy: CacheOptimizationStrategy::Auto,
            cache_info,
            prefetch_type: PrefetchType::Temporal,
            prefetch_distance: 64,
        }
    }

    /// Set the optimization strategy
    pub fn set_strategy(&mut self, strategy: CacheOptimizationStrategy) {
        self.strategy = strategy;
    }

    /// Set prefetch type
    pub fn set_prefetch_type(&mut self, prefetch_type: PrefetchType) {
        self.prefetch_type = prefetch_type;
    }

    /// Set prefetch distance
    pub fn set_prefetch_distance(&mut self, distance: usize) {
        self.prefetch_distance = distance;
    }

    /// Prefetch data at address
    pub fn prefetch(&self, address: usize) {
        match self.prefetch_type {
            PrefetchType::None => {}
            PrefetchType::Spatial => {
                for i in 1..=4 {
                    let addr = address + (i * self.prefetch_distance);
                    self.prefetch_address(addr);
                }
            }
            PrefetchType::Temporal => {
                self.prefetch_address(address);
            }
        }
    }

    /// Prefetch a specific address
    fn prefetch_address(&self, _address: usize) {
        // In a real implementation, this would use CPU prefetch instructions
        // like PREFETCHT0, PREFETCHT1, PREFETCHT2, or PREFETCHNTA
    }

    /// Get cache information
    pub fn cache_info(&self) -> &[CacheInfo] {
        &self.cache_info
    }

    /// Get L1 cache line size
    pub fn l1_line_size(&self) -> u32 {
        for info in &self.cache_info {
            if info.level == CacheLevel::L1 {
                return info.line_size;
            }
        }
        64 // Default cache line size
    }

    /// Get total cache size
    pub fn total_cache_size_kb(&self) -> u32 {
        self.cache_info.iter().map(|info| info.size_kb).sum()
    }

    /// Align address to cache line
    pub fn align_to_cache_line(&self, address: usize) -> usize {
        let line_size = self.l1_line_size() as usize;
        address & !(line_size - 1)
    }

    /// Check if address is cache line aligned
    pub fn is_cache_line_aligned(&self, address: usize) -> usize {
        let line_size = self.l1_line_size() as usize;
        address % line_size
    }
}

impl Default for CacheOptimizer {
    fn default() -> Self {
        Self::new(vec![
            CacheInfo {
                level: CacheLevel::L1,
                cache_type: CacheType::Data,
                size_kb: 32,
                line_size: 64,
                associativity: 8,
            },
            CacheInfo {
                level: CacheLevel::L2,
                cache_type: CacheType::Unified,
                size_kb: 256,
                line_size: 64,
                associativity: 8,
            },
            CacheInfo {
                level: CacheLevel::L3,
                cache_type: CacheType::Unified,
                size_kb: 8192,
                line_size: 64,
                associativity: 16,
            },
        ])
    }
}

/// Cache-friendly buffer
pub struct CacheBuffer<T> {
    data: VecDeque<T>,
    capacity: usize,
    line_size: usize,
}

impl<T> CacheBuffer<T> {
    /// Create a new cache buffer
    pub fn new(capacity: usize, line_size: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            capacity,
            line_size,
        }
    }

    /// Push an item to the buffer
    pub fn push(&mut self, item: T) {
        if self.data.len() >= self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(item);
    }

    /// Pop an item from the buffer
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    /// Get buffer length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Default for CacheBuffer<T> {
    fn default() -> Self {
        Self::new(1024, 64)
    }
}

/// Cache statistics
#[derive(Debug, Clone, Copy)]
pub struct CacheStatistics {
    pub total_accesses: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub hit_ratio: f64,
}

impl CacheStatistics {
    /// Calculate hit ratio
    pub fn calculate_hit_ratio(total: u64, hits: u64) -> f64 {
        if total == 0 {
            0.0
        } else {
            (hits as f64) / (total as f64)
        }
    }
}

/// Global cache optimizer
static CACHE_OPTIMIZER: spin::Once<CacheOptimizer> = spin::Once::new();

/// Get the global cache optimizer
pub fn cache_optimizer() -> &'static CacheOptimizer {
    CACHE_OPTIMIZER.call_once(|| CacheOptimizer::default())
}

/// Prefetch data at address
pub fn prefetch(address: usize) {
    cache_optimizer().prefetch(address)
}

/// Align address to cache line
pub fn align_to_cache_line(address: usize) -> usize {
    cache_optimizer().align_to_cache_line(address)
}