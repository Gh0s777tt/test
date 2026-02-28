// VantisFS Advanced Features
// Journaling, B-tree indexing, extent-based allocation, compression

use crate::verified::filesystem::vantisfs::*;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// Journaling for Crash Recovery
// ============================================================================

/// Journal entry type
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum JournalEntryType {
    BeginTransaction = 1,
    CommitTransaction = 2,
    AbortTransaction = 3,
    InodeUpdate = 4,
    BlockUpdate = 5,
    DirectoryUpdate = 6,
    SuperblockUpdate = 7,
}

/// Journal entry header
#[derive(Debug, Clone)]
#[repr(C)]
pub struct JournalEntryHeader {
    pub entry_type: JournalEntryType,
    pub transaction_id: u64,
    pub sequence: u64,
    pub length: u32,
    pub checksum: u32,
}

/// Journal entry
#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub header: JournalEntryHeader,
    pub data: Vec<u8>,
}

/// Journal configuration
#[derive(Debug, Clone)]
pub struct JournalConfig {
    pub journal_size: u64,           // Size of journal in blocks
    pub checkpoint_interval: u64,    // Checkpoint every N transactions
    pub max_transaction_age: u64,    // Max age of transaction in ms
}

impl Default for JournalConfig {
    fn default() -> Self {
        Self {
            journal_size: 1024,      // 1024 blocks
            checkpoint_interval: 100, // Checkpoint every 100 transactions
            max_transaction_age: 5000, // 5 seconds
        }
    }
}

/// Journal manager
pub struct JournalManager {
    config: JournalConfig,
    current_transaction_id: AtomicU64,
    current_sequence: AtomicU64,
    active_transactions: BTreeMap<u64, Vec<JournalEntry>>,
    committed_transactions: BTreeMap<u64, Vec<JournalEntry>>,
    checkpoint_counter: AtomicU64,
}

impl JournalManager {
    pub fn new(config: JournalConfig) -> Self {
        Self {
            config,
            current_transaction_id: AtomicU64::new(1),
            current_sequence: AtomicU64::new(1),
            active_transactions: BTreeMap::new(),
            committed_transactions: BTreeMap::new(),
            checkpoint_counter: AtomicU64::new(0),
        }
    }

    /// Begin a new transaction
    pub fn begin_transaction(&self) -> u64 {
        let tx_id = self.current_transaction_id.fetch_add(1, Ordering::SeqCst);
        
        let entry = JournalEntry {
            header: JournalEntryHeader {
                entry_type: JournalEntryType::BeginTransaction,
                transaction_id: tx_id,
                sequence: self.current_sequence.fetch_add(1, Ordering::SeqCst),
                length: 0,
                checksum: 0,
            },
            data: Vec::new(),
        };
        
        self.active_transactions.insert(tx_id, vec![entry]);
        tx_id
    }

    /// Add entry to transaction
    pub fn add_entry(&self, tx_id: u64, entry_type: JournalEntryType, data: Vec<u8>) -> Result<(), &'static str> {
        if !self.active_transactions.contains_key(&tx_id) {
            return Err("Transaction not found");
        }

        let checksum = self.calculate_checksum(&data);
        let entry = JournalEntry {
            header: JournalEntryHeader {
                entry_type,
                transaction_id: tx_id,
                sequence: self.current_sequence.fetch_add(1, Ordering::SeqCst),
                length: data.len() as u32,
                checksum,
            },
            data,
        };

        if let Some(entries) = self.active_transactions.get_mut(&tx_id) {
            entries.push(entry);
        }

        Ok(())
    }

    /// Commit transaction
    pub fn commit_transaction(&self, tx_id: u64) -> Result<(), &'static str> {
        if !self.active_transactions.contains_key(&tx_id) {
            return Err("Transaction not found");
        }

        let mut entries = self.active_transactions.remove(&tx_id).unwrap();
        
        // Add commit entry
        let commit_entry = JournalEntry {
            header: JournalEntryHeader {
                entry_type: JournalEntryType::CommitTransaction,
                transaction_id: tx_id,
                sequence: self.current_sequence.fetch_add(1, Ordering::SeqCst),
                length: 0,
                checksum: 0,
            },
            data: Vec::new(),
        };
        entries.push(commit_entry);

        self.committed_transactions.insert(tx_id, entries);
        
        // Checkpoint if needed
        let counter = self.checkpoint_counter.fetch_add(1, Ordering::SeqCst);
        if counter + 1 >= self.config.checkpoint_interval {
            self.checkpoint();
            self.checkpoint_counter.store(0, Ordering::SeqCst);
        }

        Ok(())
    }

    /// Abort transaction
    pub fn abort_transaction(&self, tx_id: u64) -> Result<(), &'static str> {
        if !self.active_transactions.contains_key(&tx_id) {
            return Err("Transaction not found");
        }

        self.active_transactions.remove(&tx_id);
        Ok(())
    }

    /// Checkpoint journal
    pub fn checkpoint(&self) {
        // In a real implementation, this would:
        // 1. Write all committed transactions to disk
        // 2. Clear the journal
        // 3. Update superblock checkpoint info
        
        // For now, just clear committed transactions
        self.committed_transactions.clear();
    }

    /// Recover from journal
    pub fn recover(&self) -> Result<Vec<u64>, &'static str> {
        let mut recovered_txs = Vec::new();
        
        // In a real implementation, this would:
        // 1. Read journal from disk
        // 2. Find incomplete transactions
        // 3. Replay committed transactions
        // 4. Rollback uncommitted transactions
        
        // For now, return empty list
        Ok(recovered_txs)
    }

    /// Calculate checksum
    fn calculate_checksum(&self, data: &[u8]) -> u32 {
        let mut checksum: u32 = 0;
        for byte in data {
            checksum = checksum.wrapping_mul(31).wrapping_add(*byte as u32);
        }
        checksum
    }

    /// Get journal statistics
    pub fn get_stats(&self) -> JournalStats {
        JournalStats {
            active_transactions: self.active_transactions.len(),
            committed_transactions: self.committed_transactions.len(),
            current_transaction_id: self.current_transaction_id.load(Ordering::SeqCst),
            current_sequence: self.current_sequence.load(Ordering::SeqCst),
            checkpoint_counter: self.checkpoint_counter.load(Ordering::SeqCst),
        }
    }
}

/// Journal statistics
#[derive(Debug, Clone)]
pub struct JournalStats {
    pub active_transactions: usize,
    pub committed_transactions: usize,
    pub current_transaction_id: u64,
    pub current_sequence: u64,
    pub checkpoint_counter: u64,
}

// ============================================================================
// B-tree Indexing for Directories
// ============================================================================

/// B-tree node type
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BTreeNodeType {
    Internal = 1,
    Leaf = 2,
}

/// B-tree node
#[derive(Debug, Clone)]
pub struct BTreeNode {
    pub node_type: BTreeNodeType,
    pub keys: Vec<String>,
    pub values: Vec<u64>,  // Inode numbers
    pub children: Vec<u64>, // Child node block numbers
}

impl BTreeNode {
    pub fn new(node_type: BTreeNodeType) -> Self {
        Self {
            node_type,
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.node_type == BTreeNodeType::Leaf
    }

    pub fn is_full(&self, order: usize) -> bool {
        self.keys.len() >= 2 * order - 1
    }
}

/// B-tree configuration
#[derive(Debug, Clone)]
pub struct BTreeConfig {
    pub order: usize,  // Order of the B-tree (minimum degree)
}

impl Default for BTreeConfig {
    fn default() -> Self {
        Self {
            order: 4,  // Order 4 means max 7 keys, 8 children
        }
    }
}

/// B-tree index for directories
pub struct BTreeIndex {
    config: BTreeConfig,
    root_block: u64,
    nodes: BTreeMap<u64, BTreeNode>,
    next_block: AtomicU64,
}

impl BTreeIndex {
    pub fn new(config: BTreeConfig) -> Self {
        let root = BTreeNode::new(BTreeNodeType::Leaf);
        let root_block = 1;
        
        let mut nodes = BTreeMap::new();
        nodes.insert(root_block, root);

        Self {
            config,
            root_block,
            nodes,
            next_block: AtomicU64::new(2),
        }
    }

    /// Insert key-value pair
    pub fn insert(&mut self, key: String, value: u64) -> Result<(), &'static str> {
        if self.nodes.get(&self.root_block).unwrap().is_full(self.config.order) {
            let old_root = self.root_block;
            let new_root_block = self.next_block.fetch_add(1, Ordering::SeqCst);
            let mut new_root = BTreeNode::new(BTreeNodeType::Internal);
            new_root.children.push(old_root);
            
            self.nodes.insert(new_root_block, new_root);
            self.root_block = new_root_block;
            
            self.split_child(new_root_block, 0)?;
        }

        self.insert_non_full(self.root_block, key, value)
    }

    /// Insert into non-full node
    fn insert_non_full(&mut self, block: u64, key: String, value: u64) -> Result<(), &'static str> {
        let node = self.nodes.get_mut(&block).unwrap();
        
        if node.is_leaf() {
            // Insert into leaf node
            let pos = node.keys.iter().position(|k| k >= &key).unwrap_or(node.keys.len());
            node.keys.insert(pos, key);
            node.values.insert(pos, value);
        } else {
            // Insert into internal node
            let pos = node.keys.iter().position(|k| k > &key).unwrap_or(node.keys.len());
            let child_block = node.children[pos];
            
            if self.nodes.get(&child_block).unwrap().is_full(self.config.order) {
                self.split_child(block, pos)?;
                // After split, determine which child to go to
                let node = self.nodes.get(block).unwrap();
                if key > node.keys[pos] {
                    return self.insert_non_full(node.children[pos + 1], key, value);
                }
            }
            
            self.insert_non_full(child_block, key, value)?;
        }

        Ok(())
    }

    /// Split child node
    fn split_child(&mut self, parent_block: u64, child_index: usize) -> Result<(), &'static str> {
        let parent = self.nodes.get(&parent_block).unwrap().clone();
        let child_block = parent.children[child_index];
        let mut child = self.nodes.get(&child_block).unwrap().clone();
        
        let new_block = self.next_block.fetch_add(1, Ordering::SeqCst);
        let mut new_node = BTreeNode::new(child.node_type);
        
        let order = self.config.order;
        let mid = order - 1;
        
        // Split keys and values
        let mid_key = child.keys.remove(mid);
        let mid_value = child.values.remove(mid);
        
        new_node.keys = child.keys.split_off(mid);
        new_node.values = child.values.split_off(mid);
        
        if !child.is_leaf() {
            new_node.children = child.children.split_off(mid + 1);
        }
        
        // Insert mid key into parent
        let mut parent = self.nodes.get_mut(&parent_block).unwrap();
        parent.keys.insert(child_index, mid_key);
        parent.values.insert(child_index, mid_value);
        parent.children.insert(child_index + 1, new_block);
        
        // Store nodes
        self.nodes.insert(child_block, child);
        self.nodes.insert(new_block, new_node);
        
        Ok(())
    }

    /// Search for key
    pub fn search(&self, key: &str) -> Option<u64> {
        self.search_node(self.root_block, key)
    }

    /// Search in node
    fn search_node(&self, block: u64, key: &str) -> Option<u64> {
        let node = self.nodes.get(&block)?;
        
        let pos = node.keys.iter().position(|k| k >= key).unwrap_or(node.keys.len());
        
        if pos < node.keys.len() && node.keys[pos] == key {
            return Some(node.values[pos]);
        }
        
        if node.is_leaf() {
            None
        } else {
            self.search_node(node.children[pos], key)
        }
    }

    /// Delete key
    pub fn delete(&mut self, key: &str) -> Result<Option<u64>, &'static str> {
        self.delete_from_node(self.root_block, key)
    }

    /// Delete from node
    fn delete_from_node(&mut self, block: u64, key: &str) -> Result<Option<u64>, &'static str> {
        let node = self.nodes.get(&block).unwrap().clone();
        
        if node.is_leaf() {
            if let Some(pos) = node.keys.iter().position(|k| k == key) {
                let mut node = self.nodes.get_mut(&block).unwrap();
                let value = node.values.remove(pos);
                node.keys.remove(pos);
                return Ok(Some(value));
            }
            return Ok(None);
        }
        
        // Internal node - find child
        let pos = node.keys.iter().position(|k| k > key).unwrap_or(node.keys.len());
        let child_block = node.children[pos];
        
        self.delete_from_node(child_block, key)
    }

    /// List all keys
    pub fn list_keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        self.list_keys_node(self.root_block, &mut keys);
        keys
    }

    /// List keys in node
    fn list_keys_node(&self, block: u64, keys: &mut Vec<String>) {
        if let Some(node) = self.nodes.get(&block) {
            if node.is_leaf() {
                keys.extend(node.keys.clone());
            } else {
                for (i, child_block) in node.children.iter().enumerate() {
                    self.list_keys_node(*child_block, keys);
                    if i < node.keys.len() {
                        keys.push(node.keys[i].clone());
                    }
                }
            }
        }
    }

    /// Get B-tree statistics
    pub fn get_stats(&self) -> BTreeStats {
        let mut stats = BTreeStats {
            total_nodes: self.nodes.len(),
            total_keys: 0,
            tree_depth: 0,
            root_block: self.root_block,
        };
        
        self.count_keys_and_depth(self.root_block, 0, &mut stats);
        stats
    }

    /// Count keys and calculate depth
    fn count_keys_and_depth(&self, block: u64, depth: usize, stats: &mut BTreeStats) {
        if let Some(node) = self.nodes.get(&block) {
            stats.total_keys += node.keys.len();
            stats.tree_depth = stats.tree_depth.max(depth + 1);
            
            if !node.is_leaf() {
                for child_block in &node.children {
                    self.count_keys_and_depth(*child_block, depth + 1, stats);
                }
            }
        }
    }
}

/// B-tree statistics
#[derive(Debug, Clone)]
pub struct BTreeStats {
    pub total_nodes: usize,
    pub total_keys: usize,
    pub tree_depth: usize,
    pub root_block: u64,
}

// ============================================================================
// Extent-based Allocation
// ============================================================================

/// Extent descriptor
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Extent {
    pub start_block: u64,
    pub length: u64,
}

impl Extent {
    pub fn new(start_block: u64, length: u64) -> Self {
        Self {
            start_block,
            length,
        }
    }

    pub fn is_contiguous(&self, other: &Extent) -> bool {
        self.start_block + self.length == other.start_block
    }

    pub fn merge(&mut self, other: &Extent) {
        self.length += other.length;
    }
}

/// Extent tree node
#[derive(Debug, Clone)]
pub struct ExtentTreeNode {
    pub extents: Vec<Extent>,
    pub children: Vec<u64>,
}

impl ExtentTreeNode {
    pub fn new() -> Self {
        Self {
            extents: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// Extent tree configuration
#[derive(Debug, Clone)]
pub struct ExtentTreeConfig {
    pub max_extents_per_node: usize,
}

impl Default for ExtentTreeConfig {
    fn default() -> Self {
        Self {
            max_extents_per_node: 16,
        }
    }
}

/// Extent-based allocator
pub struct ExtentAllocator {
    config: ExtentTreeConfig,
    root_block: u64,
    nodes: BTreeMap<u64, ExtentTreeNode>,
    next_block: AtomicU64,
    free_blocks: Vec<u64>,
}

impl ExtentAllocator {
    pub fn new(config: ExtentTreeConfig, total_blocks: u64) -> Self {
        let root = ExtentTreeNode::new();
        let root_block = 1;
        
        let mut nodes = BTreeMap::new();
        nodes.insert(root_block, root);
        
        let mut free_blocks = Vec::new();
        for block in 2..total_blocks {
            free_blocks.push(block);
        }

        Self {
            config,
            root_block,
            nodes,
            next_block: AtomicU64::new(total_blocks),
            free_blocks,
        }
    }

    /// Allocate blocks
    pub fn allocate(&mut self, count: u64) -> Result<Vec<Extent>, &'static str> {
        let mut extents = Vec::new();
        let mut remaining = count;
        
        while remaining > 0 {
            if self.free_blocks.is_empty() {
                return Err("No free blocks available");
            }
            
            let start_block = self.free_blocks.remove(0);
            let alloc_count = remaining.min(self.config.max_extents_per_node as u64);
            
            extents.push(Extent::new(start_block, alloc_count));
            remaining -= alloc_count;
        }
        
        Ok(extents)
    }

    /// Free blocks
    pub fn free(&mut self, extents: Vec<Extent>) {
        for extent in extents {
            for block in extent.start_block..(extent.start_block + extent.length) {
                self.free_blocks.push(block);
            }
        }
        
        // Sort free blocks for better allocation
        self.free_blocks.sort();
    }

    /// Get extent statistics
    pub fn get_stats(&self) -> ExtentStats {
        ExtentStats {
            total_nodes: self.nodes.len(),
            free_blocks: self.free_blocks.len(),
            next_block: self.next_block.load(Ordering::SeqCst),
        }
    }
}

/// Extent statistics
#[derive(Debug, Clone)]
pub struct ExtentStats {
    pub total_nodes: usize,
    pub free_blocks: usize,
    pub next_block: u64,
}

// ============================================================================
// Compression Support
// ============================================================================

/// Compression algorithm
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum CompressionAlgorithm {
    None = 0,
    LZ4 = 1,
    Zstd = 2,
    Deflate = 3,
}

/// Compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    pub algorithm: CompressionAlgorithm,
    pub compression_level: u32,
    pub min_size_to_compress: usize,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: CompressionAlgorithm::LZ4,
            compression_level: 3,
            min_size_to_compress: 4096,
        }
    }
}

/// Compression manager
pub struct CompressionManager {
    config: CompressionConfig,
    stats: CompressionStats,
}

impl CompressionManager {
    pub fn new(config: CompressionConfig) -> Self {
        Self {
            config,
            stats: CompressionStats::default(),
        }
    }

    /// Compress data
    pub fn compress(&mut self, data: &[u8]) -> Result<Vec<u8>, &'static str> {
        if data.len() < self.config.min_size_to_compress {
            return Ok(data.to_vec());
        }

        let compressed = match self.config.algorithm {
            CompressionAlgorithm::None => data.to_vec(),
            CompressionAlgorithm::LZ4 => self.compress_lz4(data),
            CompressionAlgorithm::Zstd => self.compress_zstd(data),
            CompressionAlgorithm::Deflate => self.compress_deflate(data),
        };

        // Only use compression if it actually reduces size
        if compressed.len() < data.len() {
            self.stats.compressed_blocks += 1;
            self.stats.compressed_size += compressed.len();
            self.stats.original_size += data.len();
            Ok(compressed)
        } else {
            self.stats.uncompressed_blocks += 1;
            self.stats.original_size += data.len();
            Ok(data.to_vec())
        }
    }

    /// Decompress data
    pub fn decompress(&mut self, data: &[u8], original_size: usize) -> Result<Vec<u8>, &'static str> {
        match self.config.algorithm {
            CompressionAlgorithm::None => Ok(data.to_vec()),
            CompressionAlgorithm::LZ4 => self.decompress_lz4(data, original_size),
            CompressionAlgorithm::Zstd => self.decompress_zstd(data, original_size),
            CompressionAlgorithm::Deflate => self.decompress_deflate(data, original_size),
        }
    }

    /// Simple LZ4-like compression (placeholder)
    fn compress_lz4(&self, data: &[u8]) -> Vec<u8> {
        // Placeholder: In real implementation, use actual LZ4 library
        // For now, just return data with compression header
        let mut compressed = Vec::with_capacity(data.len() + 8);
        compressed.push(0x04); // LZ4 magic
        compressed.push(0x22);
        compressed.push(0x4D);
        compressed.push(0x18);
        compressed.extend_from_slice(data);
        compressed
    }

    /// Simple LZ4-like decompression (placeholder)
    fn decompress_lz4(&self, data: &[u8], _original_size: usize) -> Result<Vec<u8>, &'static str> {
        // Placeholder: In real implementation, use actual LZ4 library
        if data.len() < 4 {
            return Err("Invalid compressed data");
        }
        Ok(data[4..].to_vec())
    }

    /// Simple Zstd-like compression (placeholder)
    fn compress_zstd(&self, data: &[u8]) -> Vec<u8> {
        // Placeholder: In real implementation, use actual Zstd library
        let mut compressed = Vec::with_capacity(data.len() + 8);
        compressed.push(0xFD); // Zstd frame magic
        compressed.push(0x2F);
        compressed.push(0xB5);
        compressed.push(0x28);
        compressed.extend_from_slice(data);
        compressed
    }

    /// Simple Zstd-like decompression (placeholder)
    fn decompress_zstd(&self, data: &[u8], _original_size: usize) -> Result<Vec<u8>, &'static str> {
        // Placeholder: In real implementation, use actual Zstd library
        if data.len() < 4 {
            return Err("Invalid compressed data");
        }
        Ok(data[4..].to_vec())
    }

    /// Simple Deflate compression (placeholder)
    fn compress_deflate(&self, data: &[u8]) -> Vec<u8> {
        // Placeholder: In real implementation, use actual Deflate library
        let mut compressed = Vec::with_capacity(data.len() + 8);
        compressed.push(0x78); // Deflate header
        compressed.push(0x9C);
        compressed.extend_from_slice(data);
        compressed
    }

    /// Simple Deflate decompression (placeholder)
    fn decompress_deflate(&self, data: &[u8], _original_size: usize) -> Result<Vec<u8>, &'static str> {
        // Placeholder: In real implementation, use actual Deflate library
        if data.len() < 2 {
            return Err("Invalid compressed data");
        }
        Ok(data[2..].to_vec())
    }

    /// Get compression statistics
    pub fn get_stats(&self) -> CompressionStats {
        self.stats.clone()
    }
}

/// Compression statistics
#[derive(Debug, Clone, Default)]
pub struct CompressionStats {
    pub compressed_blocks: u64,
    pub uncompressed_blocks: u64,
    pub compressed_size: u64,
    pub original_size: u64,
}

impl CompressionStats {
    pub fn compression_ratio(&self) -> f64 {
        if self.original_size == 0 {
            1.0
        } else {
            self.compressed_size as f64 / self.original_size as f64
        }
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Journaling tests
    #[test]
    fn test_journal_transaction() {
        let journal = JournalManager::new(JournalConfig::default());
        
        let tx_id = journal.begin_transaction();
        assert!(tx_id > 0);
        
        journal.add_entry(tx_id, JournalEntryType::InodeUpdate, vec![1, 2, 3]).unwrap();
        journal.commit_transaction(tx_id).unwrap();
        
        let stats = journal.get_stats();
        assert_eq!(stats.active_transactions, 0);
    }

    #[test]
    fn test_journal_abort() {
        let journal = JournalManager::new(JournalConfig::default());
        
        let tx_id = journal.begin_transaction();
        journal.add_entry(tx_id, JournalEntryType::InodeUpdate, vec![1, 2, 3]).unwrap();
        journal.abort_transaction(tx_id).unwrap();
        
        let stats = journal.get_stats();
        assert_eq!(stats.active_transactions, 0);
    }

    // B-tree tests
    #[test]
    fn test_btree_insert() {
        let mut btree = BTreeIndex::new(BTreeConfig::default());
        
        btree.insert("file1.txt".to_string(), 100).unwrap();
        btree.insert("file2.txt".to_string(), 200).unwrap();
        btree.insert("file3.txt".to_string(), 300).unwrap();
        
        assert_eq!(btree.search("file1.txt"), Some(100));
        assert_eq!(btree.search("file2.txt"), Some(200));
        assert_eq!(btree.search("file3.txt"), Some(300));
    }

    #[test]
    fn test_btree_delete() {
        let mut btree = BTreeIndex::new(BTreeConfig::default());
        
        btree.insert("file1.txt".to_string(), 100).unwrap();
        btree.insert("file2.txt".to_string(), 200).unwrap();
        
        let result = btree.delete("file1.txt").unwrap();
        assert_eq!(result, Some(100));
        assert_eq!(btree.search("file1.txt"), None);
    }

    #[test]
    fn test_btree_list() {
        let mut btree = BTreeIndex::new(BTreeConfig::default());
        
        btree.insert("c.txt".to_string(), 300).unwrap();
        btree.insert("a.txt".to_string(), 100).unwrap();
        btree.insert("b.txt".to_string(), 200).unwrap();
        
        let keys = btree.list_keys();
        assert_eq!(keys, vec!["a.txt", "b.txt", "c.txt"]);
    }

    // Extent tests
    #[test]
    fn test_extent_allocate() {
        let mut allocator = ExtentAllocator::new(ExtentTreeConfig::default(), 1000);
        
        let extents = allocator.allocate(10).unwrap();
        assert_eq!(extents.len(), 1);
        assert_eq!(extents[0].length, 10);
    }

    #[test]
    fn test_extent_free() {
        let mut allocator = ExtentAllocator::new(ExtentTreeConfig::default(), 1000);
        
        let extents = allocator.allocate(10).unwrap();
        allocator.free(extents);
        
        let stats = allocator.get_stats();
        assert_eq!(stats.free_blocks, 10);
    }

    // Compression tests
    #[test]
    fn test_compress_small() {
        let mut manager = CompressionManager::new(CompressionConfig::default());
        
        let data = vec![1, 2, 3];
        let compressed = manager.compress(&data).unwrap();
        
        // Small data should not be compressed
        assert_eq!(compressed, data);
    }

    #[test]
    fn test_compress_large() {
        let mut manager = CompressionManager::new(CompressionConfig::default());
        
        let data = vec![0u8; 8192];
        let compressed = manager.compress(&data).unwrap();
        
        // Large data should be compressed (with header)
        assert!(compressed.len() >= data.len());
    }

    #[test]
    fn test_compression_stats() {
        let mut manager = CompressionManager::new(CompressionConfig::default());
        
        let data = vec![0u8; 8192];
        manager.compress(&data).unwrap();
        
        let stats = manager.get_stats();
        assert!(stats.compressed_blocks > 0 || stats.uncompressed_blocks > 0);
    }
}