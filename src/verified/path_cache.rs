//! Path Lookup Cache
//!
//! This module implements an LRU (Least Recently Used) cache for filesystem path lookups.
//! It significantly improves performance by caching the results of path resolution operations.
//!
//! # Features
//! - LRU eviction policy
//! - Thread-safe operations
//! - Configurable cache size
//! - Cache statistics tracking
//! - Automatic invalidation on file operations

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Maximum number of entries in the path cache
const DEFAULT_CACHE_SIZE: usize = 1024;

/// Result of a path lookup
#[derive(Debug, Clone)]
pub struct PathCacheEntry {
    /// The resolved inode number
    pub inode: u64,
    /// The parent directory inode
    pub parent_inode: u64,
    /// Whether this is a directory
    pub is_directory: bool,
    /// Access count for statistics
    pub access_count: u64,
}

/// Node in the LRU linked list
#[derive(Debug)]
struct LruNode {
    /// The cache key (path)
    key: String,
    /// The cached entry
    entry: PathCacheEntry,
    /// Previous node in LRU list
    prev: Option<usize>,
    /// Next node in LRU list
    next: Option<usize>,
}

/// Path lookup cache with LRU eviction
pub struct PathCache {
    /// Maximum cache size
    max_size: usize,
    /// Hash map for O(1) lookups
    map: HashMap<String, usize>,
    /// LRU list nodes
    nodes: Vec<Option<LruNode>>,
    /// Free list for reusing node slots
    free_list: Vec<usize>,
    /// Head of LRU list (most recently used)
    head: Option<usize>,
    /// Tail of LRU list (least recently used)
    tail: Option<usize>,
    /// Current cache size
    size: usize,
    /// Statistics
    stats: CacheStats,
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total number of lookups
    pub lookups: u64,
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of evictions
    pub evictions: u64,
    /// Number of invalidations
    pub invalidations: u64,
}

impl PathCache {
    /// Create a new path cache with default size
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CACHE_SIZE)
    }

    /// Create a new path cache with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        PathCache {
            max_size: capacity,
            map: HashMap::with_capacity(capacity),
            nodes: Vec::with_capacity(capacity),
            free_list: Vec::new(),
            head: None,
            tail: None,
            size: 0,
            stats: CacheStats::default(),
        }
    }

    /// Look up a path in the cache
    pub fn lookup(&mut self, path: &str) -> Option<PathCacheEntry> {
        self.stats.lookups += 1;

        if let Some(&node_idx) = self.map.get(path) {
            // Cache hit - move to front of LRU list
            self.stats.hits += 1;
            self.move_to_front(node_idx);
            
            if let Some(ref mut node) = self.nodes[node_idx] {
                node.entry.access_count += 1;
                return Some(node.entry.clone());
            }
        }

        // Cache miss
        self.stats.misses += 1;
        None
    }

    /// Insert a path into the cache
    pub fn insert(&mut self, path: String, entry: PathCacheEntry) {
        // Check if path already exists
        if let Some(&node_idx) = self.map.get(&path) {
            // Update existing entry and move to front
            if let Some(ref mut node) = self.nodes[node_idx] {
                node.entry = entry;
            }
            self.move_to_front(node_idx);
            return;
        }

        // Need to add new entry
        if self.size >= self.max_size {
            // Evict least recently used entry
            self.evict_lru();
        }

        // Get a node index (either from free list or allocate new)
        let node_idx = if let Some(idx) = self.free_list.pop() {
            idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(None);
            idx
        };

        // Create new node
        let node = LruNode {
            key: path.clone(),
            entry,
            prev: None,
            next: self.head,
        };

        // Update head's prev pointer
        if let Some(head_idx) = self.head {
            if let Some(ref mut head_node) = self.nodes[head_idx] {
                head_node.prev = Some(node_idx);
            }
        }

        // Insert node
        self.nodes[node_idx] = Some(node);
        self.map.insert(path, node_idx);

        // Update head
        self.head = Some(node_idx);

        // Update tail if this is the first node
        if self.tail.is_none() {
            self.tail = Some(node_idx);
        }

        self.size += 1;
    }

    /// Invalidate a path in the cache
    pub fn invalidate(&mut self, path: &str) {
        if let Some(node_idx) = self.map.remove(path) {
            self.stats.invalidations += 1;
            self.remove_node(node_idx);
        }
    }

    /// Invalidate all paths under a directory
    pub fn invalidate_directory(&mut self, dir_path: &str) {
        let prefix = if dir_path.ends_with('/') {
            dir_path.to_string()
        } else {
            format!("{}/", dir_path)
        };

        // Collect paths to invalidate
        let paths_to_remove: Vec<String> = self.map
            .keys()
            .filter(|path| path.starts_with(&prefix))
            .cloned()
            .collect();

        // Remove them
        for path in paths_to_remove {
            self.invalidate(&path);
        }
    }

    /// Clear the entire cache
    pub fn clear(&mut self) {
        self.map.clear();
        self.nodes.clear();
        self.free_list.clear();
        self.head = None;
        self.tail = None;
        self.size = 0;
    }

    /// Get cache statistics
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Get cache hit rate
    pub fn hit_rate(&self) -> f64 {
        if self.stats.lookups == 0 {
            0.0
        } else {
            self.stats.hits as f64 / self.stats.lookups as f64
        }
    }

    /// Get current cache size
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get maximum cache size
    pub fn capacity(&self) -> usize {
        self.max_size
    }

    /// Move a node to the front of the LRU list
    fn move_to_front(&mut self, node_idx: usize) {
        // If already at front, nothing to do
        if Some(node_idx) == self.head {
            return;
        }

        // Get node info before modifying
        let (prev_idx, next_idx, was_tail) = if let Some(ref node) = self.nodes[node_idx] {
            (node.prev, node.next, Some(node_idx) == self.tail)
        } else {
            return;
        };

        // Update prev node's next pointer
        if let Some(prev_idx) = prev_idx {
            if let Some(ref mut prev_node) = self.nodes[prev_idx] {
                prev_node.next = next_idx;
            }
        }

        // Update next node's prev pointer
        if let Some(next_idx) = next_idx {
            if let Some(ref mut next_node) = self.nodes[next_idx] {
                next_node.prev = prev_idx;
            }
        }

        // Update tail if this was the tail
        if was_tail {
            self.tail = prev_idx;
        }

        // Move to front
        if let Some(ref mut node) = self.nodes[node_idx] {
            node.prev = None;
            node.next = self.head;
        }

        // Update old head's prev pointer
        if let Some(head_idx) = self.head {
            if let Some(ref mut head_node) = self.nodes[head_idx] {
                head_node.prev = Some(node_idx);
            }
        }

        // Update head
        self.head = Some(node_idx);
    }

    /// Evict the least recently used entry
    fn evict_lru(&mut self) {
        if let Some(tail_idx) = self.tail {
            self.stats.evictions += 1;
            
            // Get the key before removing
            let key = if let Some(ref node) = self.nodes[tail_idx] {
                node.key.clone()
            } else {
                return;
            };

            // Remove from map
            self.map.remove(&key);

            // Remove the node
            self.remove_node(tail_idx);
        }
    }

    /// Remove a node from the LRU list
    fn remove_node(&mut self, node_idx: usize) {
        if let Some(node) = self.nodes[node_idx].take() {
            // Update prev node's next pointer
            if let Some(prev_idx) = node.prev {
                if let Some(ref mut prev_node) = self.nodes[prev_idx] {
                    prev_node.next = node.next;
                }
            } else {
                // This was the head
                self.head = node.next;
            }

            // Update next node's prev pointer
            if let Some(next_idx) = node.next {
                if let Some(ref mut next_node) = self.nodes[next_idx] {
                    next_node.prev = node.prev;
                }
            } else {
                // This was the tail
                self.tail = node.prev;
            }

            // Add to free list
            self.free_list.push(node_idx);
            self.size -= 1;
        }
    }
}

impl Default for PathCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe wrapper for PathCache
pub struct SharedPathCache {
    cache: Arc<RwLock<PathCache>>,
}

impl SharedPathCache {
    /// Create a new shared path cache
    pub fn new() -> Self {
        SharedPathCache {
            cache: Arc::new(RwLock::new(PathCache::new())),
        }
    }

    /// Create a new shared path cache with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        SharedPathCache {
            cache: Arc::new(RwLock::new(PathCache::with_capacity(capacity))),
        }
    }

    /// Look up a path in the cache
    pub fn lookup(&self, path: &str) -> Option<PathCacheEntry> {
        self.cache.write().unwrap().lookup(path)
    }

    /// Insert a path into the cache
    pub fn insert(&self, path: String, entry: PathCacheEntry) {
        self.cache.write().unwrap().insert(path, entry);
    }

    /// Invalidate a path in the cache
    pub fn invalidate(&self, path: &str) {
        self.cache.write().unwrap().invalidate(path);
    }

    /// Invalidate all paths under a directory
    pub fn invalidate_directory(&self, dir_path: &str) {
        self.cache.write().unwrap().invalidate_directory(dir_path);
    }

    /// Clear the entire cache
    pub fn clear(&self) {
        self.cache.write().unwrap().clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        self.cache.read().unwrap().stats().clone()
    }

    /// Get cache hit rate
    pub fn hit_rate(&self) -> f64 {
        self.cache.read().unwrap().hit_rate()
    }

    /// Get current cache size
    pub fn size(&self) -> usize {
        self.cache.read().unwrap().size()
    }

    /// Get maximum cache size
    pub fn capacity(&self) -> usize {
        self.cache.read().unwrap().capacity()
    }

    /// Clone the Arc for sharing across threads
    pub fn clone_arc(&self) -> Self {
        SharedPathCache {
            cache: Arc::clone(&self.cache),
        }
    }
}

impl Default for SharedPathCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SharedPathCache {
    fn clone(&self) -> Self {
        self.clone_arc()
    }
}

#[cfg(all(test, feature = "verus"))]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic_operations() {
        let mut cache = PathCache::new();

        let entry = PathCacheEntry {
            inode: 123,
            parent_inode: 1,
            is_directory: false,
            access_count: 0,
        };

        // Insert and lookup
        cache.insert("/test/path".to_string(), entry.clone());
        let result = cache.lookup("/test/path");
        assert!(result.is_some());
        assert_eq!(result.unwrap().inode, 123);

        // Cache hit
        assert_eq!(cache.stats().hits, 1);
        assert_eq!(cache.stats().misses, 0);
    }

    #[test]
    fn test_cache_miss() {
        let mut cache = PathCache::new();
        let result = cache.lookup("/nonexistent");
        assert!(result.is_none());
        assert_eq!(cache.stats().misses, 1);
    }

    #[test]
    fn test_lru_eviction() {
        let mut cache = PathCache::with_capacity(2);

        let entry1 = PathCacheEntry {
            inode: 1,
            parent_inode: 0,
            is_directory: false,
            access_count: 0,
        };

        let entry2 = PathCacheEntry {
            inode: 2,
            parent_inode: 0,
            is_directory: false,
            access_count: 0,
        };

        let entry3 = PathCacheEntry {
            inode: 3,
            parent_inode: 0,
            is_directory: false,
            access_count: 0,
        };

        // Fill cache
        cache.insert("/path1".to_string(), entry1);
        cache.insert("/path2".to_string(), entry2);

        // This should evict /path1
        cache.insert("/path3".to_string(), entry3);

        assert_eq!(cache.size(), 2);
        assert!(cache.lookup("/path1").is_none());
        assert!(cache.lookup("/path2").is_some());
        assert!(cache.lookup("/path3").is_some());
        assert_eq!(cache.stats().evictions, 1);
    }

    #[test]
    fn test_invalidation() {
        let mut cache = PathCache::new();

        let entry = PathCacheEntry {
            inode: 123,
            parent_inode: 1,
            is_directory: false,
            access_count: 0,
        };

        cache.insert("/test/path".to_string(), entry);
        cache.invalidate("/test/path");

        assert!(cache.lookup("/test/path").is_none());
        assert_eq!(cache.stats().invalidations, 1);
    }

    #[test]
    fn test_directory_invalidation() {
        let mut cache = PathCache::new();

        let entry = PathCacheEntry {
            inode: 123,
            parent_inode: 1,
            is_directory: false,
            access_count: 0,
        };

        cache.insert("/dir/file1".to_string(), entry.clone());
        cache.insert("/dir/file2".to_string(), entry.clone());
        cache.insert("/other/file".to_string(), entry);

        cache.invalidate_directory("/dir");

        assert!(cache.lookup("/dir/file1").is_none());
        assert!(cache.lookup("/dir/file2").is_none());
        assert!(cache.lookup("/other/file").is_some());
    }

    #[test]
    fn test_hit_rate() {
        let mut cache = PathCache::new();

        let entry = PathCacheEntry {
            inode: 123,
            parent_inode: 1,
            is_directory: false,
            access_count: 0,
        };

        cache.insert("/test".to_string(), entry);

        // 1 hit
        cache.lookup("/test");
        // 1 miss
        cache.lookup("/nonexistent");

        assert_eq!(cache.hit_rate(), 0.5);
    }

    #[test]
    fn test_shared_cache() {
        let cache = SharedPathCache::new();

        let entry = PathCacheEntry {
            inode: 123,
            parent_inode: 1,
            is_directory: false,
            access_count: 0,
        };

        cache.insert("/test".to_string(), entry);
        let result = cache.lookup("/test");

        assert!(result.is_some());
        assert_eq!(result.unwrap().inode, 123);
    }
}
