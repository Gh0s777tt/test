//! Path lookup cache with LRU eviction policy.
//!
//! This module provides a lightweight LRU cache keyed by `PathBuf`,
//! designed for syscall path-resolution acceleration.

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};

/// Runtime cache statistics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PathLookupCacheStats {
    /// Number of successful lookups.
    pub hits: u64,
    /// Number of failed lookups.
    pub misses: u64,
    /// Number of entries evicted due to capacity pressure.
    pub evictions: u64,
}

/// LRU path lookup cache.
///
/// `T` must be cloneable because cached values are returned by value.
pub struct PathLookupCache<T: Clone> {
    capacity: usize,
    entries: HashMap<PathBuf, T>,
    lru: VecDeque<PathBuf>,
    stats: PathLookupCacheStats,
}

impl<T: Clone> PathLookupCache<T> {
    /// Create a new cache.
    ///
    /// Capacity is clamped to at least 1.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            entries: HashMap::new(),
            lru: VecDeque::new(),
            stats: PathLookupCacheStats::default(),
        }
    }

    /// Maximum number of entries kept in cache.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Number of currently cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when the cache has no entries.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Current cache statistics.
    pub fn stats(&self) -> PathLookupCacheStats {
        self.stats
    }

    /// Remove all entries and LRU history.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.lru.clear();
    }

    /// Get cached value for path.
    ///
    /// Hit/miss counters are updated and LRU order is refreshed on hit.
    pub fn get(&mut self, path: &Path) -> Option<T> {
        if let Some(value) = self.entries.get(path).cloned() {
            self.stats.hits = self.stats.hits.saturating_add(1);
            self.touch(path);
            Some(value)
        } else {
            self.stats.misses = self.stats.misses.saturating_add(1);
            None
        }
    }

    /// Check if entry exists without affecting cache statistics.
    pub fn peek(&self, path: &Path) -> Option<&T> {
        self.entries.get(path)
    }

    /// Insert (or update) an entry.
    pub fn insert(&mut self, path: &Path, value: T) {
        let key = path.to_path_buf();
        let existed = self.entries.insert(key.clone(), value).is_some();
        self.touch(&key);
        if !existed {
            self.enforce_capacity();
        }
    }

    /// Remove and return an entry, if present.
    pub fn take(&mut self, path: &Path) -> Option<T> {
        let key = path.to_path_buf();
        self.lru.retain(|cached| cached != &key);
        self.entries.remove(&key)
    }

    /// Invalidate an exact path entry.
    pub fn invalidate(&mut self, path: &Path) -> bool {
        self.take(path).is_some()
    }

    /// Invalidate all entries under a path prefix.
    pub fn invalidate_prefix(&mut self, prefix: &Path) -> usize {
        let to_remove: Vec<PathBuf> = self
            .entries
            .keys()
            .filter(|cached_path| cached_path.starts_with(prefix))
            .cloned()
            .collect();

        for path in &to_remove {
            self.entries.remove(path);
            self.lru.retain(|cached| cached != path);
        }
        to_remove.len()
    }

    fn touch(&mut self, path: &Path) {
        self.lru.retain(|cached| cached != path);
        self.lru.push_back(path.to_path_buf());
    }

    fn enforce_capacity(&mut self) {
        while self.entries.len() > self.capacity {
            let Some(candidate) = self.lru.pop_front() else {
                break;
            };
            if self.entries.remove(&candidate).is_some() {
                self.stats.evictions = self.stats.evictions.saturating_add(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PathLookupCache;
    use std::path::Path;

    #[test]
    fn test_hit_and_miss_accounting() {
        let mut cache = PathLookupCache::new(4);
        cache.insert(Path::new("/a"), 10u64);

        assert_eq!(cache.get(Path::new("/a")), Some(10));
        assert_eq!(cache.get(Path::new("/b")), None);

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[test]
    fn test_lru_eviction() {
        let mut cache = PathLookupCache::new(2);
        cache.insert(Path::new("/a"), 1u64);
        cache.insert(Path::new("/b"), 2u64);
        assert_eq!(cache.get(Path::new("/a")), Some(1)); // mark /a as most-recently used
        cache.insert(Path::new("/c"), 3u64); // should evict /b

        assert!(cache.peek(Path::new("/b")).is_none());
        assert_eq!(cache.peek(Path::new("/a")), Some(&1));
        assert_eq!(cache.peek(Path::new("/c")), Some(&3));
        assert_eq!(cache.stats().evictions, 1);
    }

    #[test]
    fn test_take_and_invalidate() {
        let mut cache = PathLookupCache::new(2);
        cache.insert(Path::new("/a"), 1u64);

        assert_eq!(cache.take(Path::new("/a")), Some(1));
        assert!(cache.peek(Path::new("/a")).is_none());

        cache.insert(Path::new("/b"), 2u64);
        assert!(cache.invalidate(Path::new("/b")));
        assert!(!cache.invalidate(Path::new("/missing")));
    }

    #[test]
    fn test_invalidate_prefix() {
        let mut cache = PathLookupCache::new(8);
        cache.insert(Path::new("/usr/bin/a"), 1u64);
        cache.insert(Path::new("/usr/bin/b"), 2u64);
        cache.insert(Path::new("/var/log/c"), 3u64);

        let removed = cache.invalidate_prefix(Path::new("/usr"));
        assert_eq!(removed, 2);
        assert!(cache.peek(Path::new("/usr/bin/a")).is_none());
        assert!(cache.peek(Path::new("/usr/bin/b")).is_none());
        assert_eq!(cache.peek(Path::new("/var/log/c")), Some(&3));
    }
}
