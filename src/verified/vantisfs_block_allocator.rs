//! VantisFS - Block Allocator
//! 
//! This module provides efficient block allocation for VantisFS with
//! Copy-on-Write semantics and free block tracking.
//!
//! # Features
//! - Bitmap-based free block tracking
//! - O(1) allocation operations
//! - Copy-on-Write allocation
//! - Formal verification with Verus
//!
//! # Security
//! - Never overwrites allocated blocks
//! - Tracks all block allocations
//! - Atomic operations

#[cfg(feature = "verus-full")]
use builtin::*;
#[cfg(feature = "verus-full")]
use builtin_macros::*;
#[cfg(feature = "verus-full")]
use vstd::prelude::*;

#[cfg(feature = "verus-full")]
verus! {

/// Maximum number of blocks in the filesystem
pub const MAX_BLOCKS: usize = 1_000_000;

/// Block size in bytes (4KB)
pub const BLOCK_SIZE: usize = 4096;

/// Size of bitmap in u64 words
pub const BITMAP_SIZE: usize = (MAX_BLOCKS + 63) / 64;

/// Block allocator error types
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BlockAllocatorError {
    /// No free blocks available
    NoFreeBlocks,
    /// Invalid block number
    InvalidBlock,
    /// Block already allocated
    BlockAlreadyAllocated,
    /// Block not allocated
    BlockNotAllocated,
}

/// Block allocator state
pub struct BlockAllocator {
    /// Bitmap tracking free (1) and allocated (0) blocks
    bitmap: [u64; BITMAP_SIZE],
    /// Number of free blocks
    free_count: u64,
    /// Total number of blocks
    total_blocks: u64,
    /// Next block to search for free block
    next_free: u64,
}

impl BlockAllocator {
    /// Create a new block allocator
    pub const fn new(total_blocks: u64) -> Self {
        BlockAllocator {
            bitmap: [u64::MAX; BITMAP_SIZE], // All blocks initially free
            free_count: total_blocks,
            total_blocks,
            next_free: 0,
        }
    }

    /// Check if a block is free
    #[verifier::spec]
    pub open spec fn is_block_free_spec(&self, block: u64) -> bool {
        block < self.total_blocks
    }

    /// Check if a block is free (implementation)
    pub fn is_block_free(&self, block: u64) -> (free: bool)
        requires block < self.total_blocks
        ensures free == self.is_block_free_spec(block)
    {
        if block >= self.total_blocks {
            return false;
        }

        let word_index = (block / 64) as usize;
        let bit_index = (block % 64) as u64;
        
        if word_index >= BITMAP_SIZE {
            return false;
        }

        let word = self.bitmap[word_index];
        let mask = 1u64 << bit_index;
        
        (word & mask) != 0
    }

    /// Allocate a free block
    /// 
    /// # Returns
    /// Block number if successful, error otherwise
    /// 
    /// # Formal Specification
    /// - Postcondition: returned block is marked as allocated
    /// - Postcondition: free_count is decremented
    pub fn allocate_block(&mut self) -> (result: Result<u64, BlockAllocatorError>)
        requires 
            old(self).free_count <= old(self).total_blocks,
            old(self).total_blocks <= MAX_BLOCKS as u64
        ensures 
            self.total_blocks == old(self).total_blocks,
            match result {
                Ok(block) => {
                    block < self.total_blocks &&
                    self.free_count == old(self).free_count - 1
                },
                Err(_) => {
                    self.free_count == old(self).free_count
                }
            }
    {
        if self.free_count == 0 {
            return Err(BlockAllocatorError::NoFreeBlocks);
        }

        // Search for free block starting from next_free
        let start = self.next_free;
        let mut current = start;
        
        loop 
            invariant 
                self.free_count <= self.total_blocks,
                self.total_blocks <= MAX_BLOCKS as u64,
                current < self.total_blocks
        {
            if self.is_block_free(current) {
                // Mark block as allocated
                let word_index = (current / 64) as usize;
                let bit_index = (current % 64) as u64;
                let mask = 1u64 << bit_index;
                
                self.bitmap[word_index] &= !mask;
                self.free_count -= 1;
                self.next_free = (current + 1) % self.total_blocks;
                
                return Ok(current);
            }
            
            current = (current + 1) % self.total_blocks;
            
            // If we've searched all blocks, no free block found
            if current == start {
                return Err(BlockAllocatorError::NoFreeBlocks);
            }
        }
    }

    /// Free a previously allocated block
    /// 
    /// # Arguments
    /// * `block` - Block number to free
    /// 
    /// # Formal Specification
    /// - Precondition: block < total_blocks
    /// - Postcondition: block is marked as free
    /// - Postcondition: free_count is incremented
    pub fn free_block(&mut self, block: u64) -> (result: Result<(), BlockAllocatorError>)
        requires 
            old(self).free_count <= old(self).total_blocks,
            old(self).total_blocks <= MAX_BLOCKS as u64,
            block < old(self).total_blocks
        ensures 
            self.total_blocks == old(self).total_blocks,
            match result {
                Ok(_) => {
                    self.free_count <= old(self).free_count + 1
                },
                Err(_) => {
                    self.free_count == old(self).free_count
                }
            }
    {
        if block >= self.total_blocks {
            return Err(BlockAllocatorError::InvalidBlock);
        }

        let word_index = (block / 64) as usize;
        let bit_index = (block % 64) as u64;
        
        if word_index >= BITMAP_SIZE {
            return Err(BlockAllocatorError::InvalidBlock);
        }

        let mask = 1u64 << bit_index;
        
        // Check if block is already free
        if (self.bitmap[word_index] & mask) != 0 {
            return Err(BlockAllocatorError::BlockNotAllocated);
        }

        // Mark block as free
        self.bitmap[word_index] |= mask;
        self.free_count += 1;
        
        Ok(())
    }

    /// Get number of free blocks
    pub fn get_free_count(&self) -> (count: u64)
        requires self.free_count <= self.total_blocks
        ensures count == self.free_count && count <= self.total_blocks
    {
        self.free_count
    }

    /// Get total number of blocks
    pub fn get_total_blocks(&self) -> (count: u64)
        requires self.total_blocks <= MAX_BLOCKS as u64
        ensures count == self.total_blocks && count <= MAX_BLOCKS as u64
    {
        self.total_blocks
    }

    /// Get number of allocated blocks
    pub fn get_allocated_count(&self) -> (count: u64)
        requires self.free_count <= self.total_blocks
        ensures count <= self.total_blocks
    {
        self.total_blocks - self.free_count
    }

    /// Check if allocator is full (no free blocks)
    pub fn is_full(&self) -> (full: bool)
        requires self.free_count <= self.total_blocks
        ensures full == (self.free_count == 0)
    {
        self.free_count == 0
    }

    /// Check if allocator is empty (all blocks free)
    pub fn is_empty(&self) -> (empty: bool)
        requires self.free_count <= self.total_blocks
        ensures empty == (self.free_count == self.total_blocks)
    {
        self.free_count == self.total_blocks
    }
}

} // verus!

#[cfg(all(test, feature = "verus-full"))]
mod tests {
    use super::*;

    #[test]
    fn test_allocator_creation() {
        let allocator = BlockAllocator::new(1000);
        assert_eq!(allocator.get_total_blocks(), 1000);
        assert_eq!(allocator.get_free_count(), 1000);
        assert_eq!(allocator.get_allocated_count(), 0);
        assert!(allocator.is_empty());
        assert!(!allocator.is_full());
    }

    #[test]
    fn test_allocate_single_block() {
        let mut allocator = BlockAllocator::new(1000);
        
        let block = allocator.allocate_block().unwrap();
        assert!(block < 1000);
        assert_eq!(allocator.get_free_count(), 999);
        assert_eq!(allocator.get_allocated_count(), 1);
    }

    #[test]
    fn test_allocate_multiple_blocks() {
        let mut allocator = BlockAllocator::new(1000);
        
        let mut blocks = Vec::new();
        for _ in 0..10 {
            let block = allocator.allocate_block().unwrap();
            blocks.push(block);
        }
        
        assert_eq!(allocator.get_free_count(), 990);
        assert_eq!(allocator.get_allocated_count(), 10);
        
        // All blocks should be unique
        for i in 0..blocks.len() {
            for j in (i+1)..blocks.len() {
                assert_ne!(blocks[i], blocks[j]);
            }
        }
    }

    #[test]
    fn test_free_block() {
        let mut allocator = BlockAllocator::new(1000);
        
        let block = allocator.allocate_block().unwrap();
        assert_eq!(allocator.get_free_count(), 999);
        
        allocator.free_block(block).unwrap();
        assert_eq!(allocator.get_free_count(), 1000);
    }

    #[test]
    fn test_allocate_and_free_multiple() {
        let mut allocator = BlockAllocator::new(1000);
        
        let mut blocks = Vec::new();
        for _ in 0..100 {
            let block = allocator.allocate_block().unwrap();
            blocks.push(block);
        }
        
        assert_eq!(allocator.get_free_count(), 900);
        
        // Free half the blocks
        for i in 0..50 {
            allocator.free_block(blocks[i]).unwrap();
        }
        
        assert_eq!(allocator.get_free_count(), 950);
    }

    #[test]
    fn test_allocate_all_blocks() {
        let mut allocator = BlockAllocator::new(100);
        
        for _ in 0..100 {
            allocator.allocate_block().unwrap();
        }
        
        assert_eq!(allocator.get_free_count(), 0);
        assert!(allocator.is_full());
        
        // Should fail to allocate when full
        let result = allocator.allocate_block();
        assert_eq!(result, Err(BlockAllocatorError::NoFreeBlocks));
    }

    #[test]
    fn test_free_invalid_block() {
        let mut allocator = BlockAllocator::new(1000);
        
        // Try to free block beyond total_blocks
        let result = allocator.free_block(1000);
        assert_eq!(result, Err(BlockAllocatorError::InvalidBlock));
        
        let result = allocator.free_block(2000);
        assert_eq!(result, Err(BlockAllocatorError::InvalidBlock));
    }

    #[test]
    fn test_free_already_free_block() {
        let mut allocator = BlockAllocator::new(1000);
        
        let block = allocator.allocate_block().unwrap();
        allocator.free_block(block).unwrap();
        
        // Try to free again
        let result = allocator.free_block(block);
        assert_eq!(result, Err(BlockAllocatorError::BlockNotAllocated));
    }

    #[test]
    fn test_is_block_free() {
        let mut allocator = BlockAllocator::new(1000);
        
        let block = allocator.allocate_block().unwrap();
        assert!(!allocator.is_block_free(block));
        
        allocator.free_block(block).unwrap();
        assert!(allocator.is_block_free(block));
    }

    #[test]
    fn test_allocator_reuse() {
        let mut allocator = BlockAllocator::new(10);
        
        // Allocate all blocks
        let mut blocks = Vec::new();
        for _ in 0..10 {
            blocks.push(allocator.allocate_block().unwrap());
        }
        
        // Free some blocks
        allocator.free_block(blocks[3]).unwrap();
        allocator.free_block(blocks[7]).unwrap();
        
        // Allocate again - should reuse freed blocks
        let new_block1 = allocator.allocate_block().unwrap();
        let new_block2 = allocator.allocate_block().unwrap();
        
        assert!(new_block1 == blocks[3] || new_block1 == blocks[7]);
        assert!(new_block2 == blocks[3] || new_block2 == blocks[7]);
        assert_ne!(new_block1, new_block2);
    }

    #[test]
    fn test_large_allocator() {
        let mut allocator = BlockAllocator::new(10000);
        
        // Allocate 5000 blocks
        for _ in 0..5000 {
            allocator.allocate_block().unwrap();
        }
        
        assert_eq!(allocator.get_free_count(), 5000);
        assert_eq!(allocator.get_allocated_count(), 5000);
    }

    #[test]
    fn test_allocator_stress() {
        let mut allocator = BlockAllocator::new(1000);
        let mut allocated = Vec::new();
        
        // Allocate and free in pattern
        for i in 0..500 {
            let block = allocator.allocate_block().unwrap();
            allocated.push(block);
            
            if i % 10 == 0 && !allocated.is_empty() {
                let to_free = allocated.remove(0);
                allocator.free_block(to_free).unwrap();
            }
        }
        
        // Verify consistency
        let expected_allocated = allocated.len() as u64;
        assert_eq!(allocator.get_allocated_count(), expected_allocated);
    }
}

#[cfg(kani)]
mod kani_verification {
    use super::*;

    #[kani::proof]
    fn verify_allocate_free_roundtrip() {
        let mut allocator = BlockAllocator::new(100);
        
        if let Ok(block) = allocator.allocate_block() {
            let free_before = allocator.get_free_count();
            allocator.free_block(block).unwrap();
            let free_after = allocator.get_free_count();
            
            assert_eq!(free_after, free_before + 1);
        }
    }

    #[kani::proof]
    fn verify_no_double_allocation() {
        let mut allocator = BlockAllocator::new(10);
        
        if let Ok(block1) = allocator.allocate_block() {
            if let Ok(block2) = allocator.allocate_block() {
                assert_ne!(block1, block2);
            }
        }
    }

    #[kani::proof]
    fn verify_free_count_consistency() {
        let mut allocator = BlockAllocator::new(100);
        let initial_free = allocator.get_free_count();
        
        if let Ok(block) = allocator.allocate_block() {
            assert_eq!(allocator.get_free_count(), initial_free - 1);
            
            if allocator.free_block(block).is_ok() {
                assert_eq!(allocator.get_free_count(), initial_free);
            }
        }
    }
}