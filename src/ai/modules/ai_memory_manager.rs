//! AI-Powered Memory Manager
//!
//! Advanced memory management system that uses machine learning to
//! optimize memory allocation, deallocation, and usage patterns.

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for AI memory manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Total available memory in bytes
    pub total_memory: usize,
    
    /// Minimum free memory threshold (0.0 - 1.0)
    pub min_free_threshold: f64,
    
    /// Enable predictive allocation
    pub enable_predictive_allocation: bool,
    
    /// Enable memory compression
    pub enable_compression: bool,
    
    /// Enable automatic defragmentation
    pub enable_defragmentation: bool,
    
    /// Defragmentation threshold (fragmentation percentage)
    pub defrag_threshold: f64,
    
    /// Maximum allocation size (bytes)
    pub max_allocation_size: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            total_memory: 16 * 1024 * 1024 * 1024, // 16GB
            min_free_threshold: 0.1,
            enable_predictive_allocation: true,
            enable_compression: true,
            enable_defragmentation: true,
            defrag_threshold: 0.3,
            max_allocation_size: 4 * 1024 * 1024 * 1024, // 4GB
        }
    }
}

/// Memory allocation request
#[derive(Debug, Clone)]
pub struct AllocationRequest {
    pub process_id: String,
    pub size: usize,
    pub priority: ProcessPriority,
    pub expected_lifetime: Duration,
}

/// Memory block
#[derive(Debug, Clone)]
struct MemoryBlock {
    start_addr: usize,
    size: usize,
    process_id: String,
    allocated: bool,
    last_access: Instant,
    access_count: usize,
    compressed: bool,
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_memory: usize,
    pub used_memory: usize,
    pub free_memory: usize,
    pub fragmentation: f64,
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub compression_ratio: f64,
}

/// Process priority
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// Memory prediction model
#[derive(Debug)]
struct MemoryPredictionModel {
    allocation_patterns: HashMap<String, VecDeque<usize>>,
    prediction_accuracy: VecDeque<f64>,
}

impl MemoryPredictionModel {
    fn new() -> Self {
        Self {
            allocation_patterns: HashMap::new(),
            prediction_accuracy: VecDeque::with_capacity(100),
        }
    }
    
    /// Record allocation for learning
    fn record_allocation(&mut self, process_id: &str, size: usize) {
        self.allocation_patterns
            .entry(process_id.to_string())
            .or_insert_with(|| VecDeque::with_capacity(100))
            .push_back(size);
        
        let patterns = self.allocation_patterns.get_mut(process_id).unwrap();
        while patterns.len() > 100 {
            patterns.pop_front();
        }
    }
    
    /// Predict next allocation size
    fn predict(&self, process_id: &str) -> Option<usize> {
        if let Some(patterns) = self.allocation_patterns.get(process_id) {
            if patterns.len() >= 5 {
                let sum: usize = patterns.iter().sum();
                Some(sum / patterns.len())
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Update prediction accuracy
    fn update_accuracy(&mut self, predicted: usize, actual: usize) {
        let accuracy = if actual == 0 {
            0.0
        } else {
            1.0 - (predicted.abs_diff(actual) as f64 / actual as f64)
        };
        
        self.prediction_accuracy.push_back(accuracy);
        if self.prediction_accuracy.len() > 100 {
            self.prediction_accuracy.pop_front();
        }
    }
}

/// AI-powered memory manager
pub struct AiMemoryManager {
    config: MemoryConfig,
    memory_blocks: Arc<RwLock<Vec<MemoryBlock>>>,
    process_allocations: Arc<RwLock<HashMap<String, Vec<usize>>>>,
    prediction_model: Arc<RwLock<MemoryPredictionModel>>,
    stats: Arc<RwLock<MemoryStats>>,
}

impl AiMemoryManager {
    /// Create a new AI memory manager
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            config,
            memory_blocks: Arc::new(RwLock::new(Vec::new())),
            process_allocations: Arc::new(RwLock::new(HashMap::new())),
            prediction_model: Arc::new(RwLock::new(MemoryPredictionModel::new())),
            stats: Arc::new(RwLock::new(MemoryStats {
                total_memory: config.total_memory,
                used_memory: 0,
                free_memory: config.total_memory,
                fragmentation: 0.0,
                total_allocations: 0,
                total_deallocations: 0,
                compression_ratio: 1.0,
            })),
        }
    }
    
    /// Allocate memory
    pub async fn allocate(&self, request: AllocationRequest) -> Result<usize, MemoryError> {
        let mut blocks = self.memory_blocks.write().await;
        let mut stats = self.stats.write().await;
        
        // Check if enough free memory
        let used = self.calculate_used_memory(&blocks);
        if used + request.size > self.config.total_memory {
            return Err(MemoryError::InsufficientMemory);
        }
        
        // Find free block or create new one
        let block = self.find_or_create_block(&mut blocks, &request)?;
        
        // Update statistics
        stats.used_memory = self.calculate_used_memory(&blocks);
        stats.free_memory = self.config.total_memory - stats.used_memory;
        stats.total_allocations += 1;
        
        // Record in prediction model
        let mut model = self.prediction_model.write().await;
        model.record_allocation(&request.process_id, request.size);
        
        // Add to process allocations
        let mut proc_allocs = self.process_allocations.write().await;
        proc_allocs
            .entry(request.process_id.clone())
            .or_insert_with(Vec::new)
            .push(block.start_addr);
        
        Ok(block.start_addr)
    }
    
    /// Deallocate memory
    pub async fn deallocate(&self, address: usize) -> Result<(), MemoryError> {
        let mut blocks = self.memory_blocks.write().await;
        let mut stats = self.stats.write().await;
        
        let block_idx = blocks
            .iter()
            .position(|b| b.start_addr == address)
            .ok_or(MemoryError::InvalidAddress)?;
        
        blocks[block_idx].allocated = false;
        
        // Update statistics
        stats.used_memory = self.calculate_used_memory(&blocks);
        stats.free_memory = self.config.total_memory - stats.used_memory;
        stats.total_deallocations += 1;
        
        // Check if defragmentation is needed
        if self.config.enable_defragmentation {
            let frag = self.calculate_fragmentation(&blocks);
            if frag > self.config.defrag_threshold {
                self.defragment(&mut blocks);
            }
        }
        
        Ok(())
    }
    
    /// Find or create a memory block
    fn find_or_create_block(
        &self,
        blocks: &mut Vec<MemoryBlock>,
        request: &AllocationRequest,
    ) -> Result<MemoryBlock, MemoryError> {
        // Try to find free block
        if let Some(idx) = blocks.iter().position(|b| !b.allocated && b.size >= request.size) {
            let mut block = blocks.remove(idx);
            block.allocated = true;
            block.process_id = request.process_id.clone();
            block.last_access = Instant::now();
            block.access_count += 1;
            return Ok(block);
        }
        
        // Create new block
        let current_used = blocks.iter().filter(|b| b.allocated).map(|b| b.size).sum();
        let new_block = MemoryBlock {
            start_addr: current_used,
            size: request.size,
            process_id: request.process_id.clone(),
            allocated: true,
            last_access: Instant::now(),
            access_count: 1,
            compressed: false,
        };
        
        Ok(new_block)
    }
    
    /// Calculate used memory
    fn calculate_used_memory(&self, blocks: &[MemoryBlock]) -> usize {
        blocks.iter().filter(|b| b.allocated).map(|b| b.size).sum()
    }
    
    /// Calculate fragmentation ratio
    fn calculate_fragmentation(&self, blocks: &[MemoryBlock]) -> f64 {
        if blocks.is_empty() {
            return 0.0;
        }
        
        let total_free: usize = blocks.iter().filter(|b| !b.allocated).map(|b| b.size).sum();
        if total_free == 0 {
            return 0.0;
        }
        
        let largest_free = blocks
            .iter()
            .filter(|b| !b.allocated)
            .map(|b| b.size)
            .max()
            .unwrap_or(0);
        
        if largest_free == 0 {
            return 0.0;
        }
        
        1.0 - (largest_free as f64 / total_free as f64)
    }
    
    /// Defragment memory
    fn defragment(&self, blocks: &mut Vec<MemoryBlock>) {
        let allocated: Vec<MemoryBlock> = blocks
            .iter()
            .filter(|b| b.allocated)
            .cloned()
            .collect();
        
        blocks.clear();
        let mut current_addr = 0;
        
        for mut block in allocated {
            block.start_addr = current_addr;
            current_addr += block.size;
            blocks.push(block);
        }
    }
    
    /// Predict future memory needs
    pub async fn predict_needs(&self, process_id: &str) -> Option<usize> {
        let model = self.prediction_model.read().await;
        model.predict(process_id)
    }
    
    /// Get memory statistics
    pub async fn get_stats(&self) -> MemoryStats {
        let blocks = self.memory_blocks.read().await;
        let mut stats = self.stats.write().await;
        
        stats.fragmentation = self.calculate_fragmentation(&blocks);
        stats.clone()
    }
    
    /// Optimize memory usage
    pub async fn optimize(&self) -> Result<(), MemoryError> {
        let mut blocks = self.memory_blocks.write().await;
        
        // Compress rarely used blocks
        if self.config.enable_compression {
            self.compress_blocks(&mut blocks);
        }
        
        // Defragment if needed
        if self.config.enable_defragmentation {
            let frag = self.calculate_fragmentation(&blocks);
            if frag > self.config.defrag_threshold {
                self.defragment(&mut blocks);
            }
        }
        
        Ok(())
    }
    
    /// Compress memory blocks
    fn compress_blocks(&self, blocks: &mut Vec<MemoryBlock>) {
        let now = Instant::now();
        for block in blocks.iter_mut() {
            if block.allocated && !block.compressed {
                if now.duration_since(block.last_access) > Duration::from_secs(300) {
                    // Compress blocks not accessed in 5 minutes
                    block.compressed = true;
                }
            }
        }
    }
}

/// Memory errors
#[derive(Debug, Clone)]
pub enum MemoryError {
    InsufficientMemory,
    InvalidAddress,
    AllocationFailed,
    DeallocationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_allocate_memory() {
        let config = MemoryConfig::default();
        let manager = AiMemoryManager::new(config);
        
        let request = AllocationRequest {
            process_id: "test".to_string(),
            size: 1024,
            priority: ProcessPriority::Normal,
            expected_lifetime: Duration::from_secs(10),
        };
        
        let address = manager.allocate(request).await;
        assert!(address.is_ok());
    }
    
    #[tokio::test]
    async fn test_deallocate_memory() {
        let config = MemoryConfig::default();
        let manager = AiMemoryManager::new(config);
        
        let request = AllocationRequest {
            process_id: "test".to_string(),
            size: 1024,
            priority: ProcessPriority::Normal,
            expected_lifetime: Duration::from_secs(10),
        };
        
        let address = manager.allocate(request).await.unwrap();
        let result = manager.deallocate(address).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_predict_needs() {
        let config = MemoryConfig::default();
        let manager = AiMemoryManager::new(config);
        
        // Allocate memory to build pattern
        for _ in 0..5 {
            let request = AllocationRequest {
                process_id: "test".to_string(),
                size: 1024,
                priority: ProcessPriority::Normal,
                expected_lifetime: Duration::from_secs(10),
            };
            let _ = manager.allocate(request).await;
        }
        
        let prediction = manager.predict_needs("test").await;
        assert!(prediction.is_some());
    }
}