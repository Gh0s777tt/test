//! Integration of Path Cache with Filesystem Syscalls
//!
//! This module provides integration between the path lookup cache and
//! filesystem syscalls to improve performance of path resolution operations.

use crate::path_cache::{PathCacheEntry, SharedPathCache};
use crate::syscall_file_ops::FileOpError;

/// Filesystem operations with path caching
pub struct CachedFilesystem {
    /// The path cache
    cache: SharedPathCache,
}

impl CachedFilesystem {
    /// Create a new cached filesystem
    pub fn new() -> Self {
        CachedFilesystem {
            cache: SharedPathCache::new(),
        }
    }

    /// Create a new cached filesystem with specified cache capacity
    pub fn with_cache_capacity(capacity: usize) -> Self {
        CachedFilesystem {
            cache: SharedPathCache::with_capacity(capacity),
        }
    }

    /// Look up a path with caching
    ///
    /// This function first checks the cache for the path. If found, it returns
    /// the cached inode. Otherwise, it performs the actual path lookup and
    /// caches the result.
    pub fn lookup_path(&self, path: &str) -> Result<u64, FileOpError> {
        // Check cache first
        if let Some(entry) = self.cache.lookup(path) {
            return Ok(entry.inode);
        }

        // Cache miss - perform actual lookup
        // In a real implementation, this would call the actual filesystem lookup
        let inode = self.perform_path_lookup(path)?;
        
        // Determine parent inode and whether it's a directory
        let (parent_inode, is_directory) = self.get_path_metadata(path)?;

        // Cache the result
        let entry = PathCacheEntry {
            inode,
            parent_inode,
            is_directory,
            access_count: 1,
        };
        self.cache.insert(path.to_string(), entry);

        Ok(inode)
    }

    /// Invalidate cache entries when a file is created
    pub fn on_file_created(&self, path: &str) {
        // Invalidate the parent directory
        if let Some(parent) = Self::get_parent_path(path) {
            self.cache.invalidate_directory(parent);
        }
    }

    /// Invalidate cache entries when a file is deleted
    pub fn on_file_deleted(&self, path: &str) {
        // Invalidate the file itself
        self.cache.invalidate(path);
        
        // Invalidate the parent directory
        if let Some(parent) = Self::get_parent_path(path) {
            self.cache.invalidate_directory(parent);
        }
    }

    /// Invalidate cache entries when a file is renamed
    pub fn on_file_renamed(&self, old_path: &str, new_path: &str) {
        // Invalidate both old and new paths
        self.cache.invalidate(old_path);
        self.cache.invalidate(new_path);
        
        // Invalidate parent directories
        if let Some(old_parent) = Self::get_parent_path(old_path) {
            self.cache.invalidate_directory(old_parent);
        }
        if let Some(new_parent) = Self::get_parent_path(new_path) {
            self.cache.invalidate_directory(new_parent);
        }
    }

    /// Invalidate cache entries when a directory is modified
    pub fn on_directory_modified(&self, dir_path: &str) {
        self.cache.invalidate_directory(dir_path);
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> crate::path_cache::CacheStats {
        self.cache.stats()
    }

    /// Get cache hit rate
    pub fn cache_hit_rate(&self) -> f64 {
        self.cache.hit_rate()
    }

    /// Clear the cache
    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Perform the actual path lookup (placeholder)
    ///
    /// In a real implementation, this would call the actual filesystem
    /// path resolution logic.
    fn perform_path_lookup(&self, _path: &str) -> Result<u64, FileOpError> {
        // Placeholder - in real implementation, this would:
        // 1. Parse the path
        // 2. Walk the directory tree
        // 3. Resolve symbolic links
        // 4. Return the inode number
        
        // For now, return a dummy inode
        Ok(1000)
    }

    /// Get metadata for a path (placeholder)
    fn get_path_metadata(&self, _path: &str) -> Result<(u64, bool), FileOpError> {
        // Placeholder - in real implementation, this would:
        // 1. Get the parent directory inode
        // 2. Determine if the path is a directory
        
        // For now, return dummy values
        Ok((1, false))
    }

    /// Extract the parent path from a full path
    fn get_parent_path(path: &str) -> Option<&str> {
        path.rfind('/').map(|pos| &path[..pos])
    }
}

impl Default for CachedFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Global cached filesystem instance
use std::sync::OnceLock;
static CACHED_FS: OnceLock<CachedFilesystem> = OnceLock::new();

/// Initialize the global cached filesystem
pub fn init_cached_filesystem() {
    CACHED_FS.get_or_init(CachedFilesystem::new);
}

/// Get the global cached filesystem instance
pub fn get_cached_filesystem() -> Option<&'static CachedFilesystem> {
    CACHED_FS.get()
}

#[cfg(all(test, feature = "verus"))]
mod tests {
    use super::*;

    #[test]
    fn test_cached_lookup() {
        let fs = CachedFilesystem::new();
        
        // First lookup - cache miss
        let inode1 = fs.lookup_path("/test/file").unwrap();
        assert_eq!(fs.cache_stats().misses, 1);
        
        // Second lookup - cache hit
        let inode2 = fs.lookup_path("/test/file").unwrap();
        assert_eq!(inode1, inode2);
        assert_eq!(fs.cache_stats().hits, 1);
    }

    #[test]
    fn test_invalidation_on_create() {
        let fs = CachedFilesystem::new();
        
        // Populate cache
        fs.lookup_path("/dir/file1").unwrap();
        
        // Create a new file
        fs.on_file_created("/dir/file2");
        
        // Directory should be invalidated
        assert!(fs.cache_stats().invalidations > 0);
    }

    #[test]
    fn test_invalidation_on_delete() {
        let fs = CachedFilesystem::new();
        
        // Populate cache
        fs.lookup_path("/dir/file").unwrap();
        
        // Delete the file
        fs.on_file_deleted("/dir/file");
        
        // File should be invalidated
        assert!(fs.cache_stats().invalidations > 0);
    }

    #[test]
    fn test_parent_path_extraction() {
        assert_eq!(
            CachedFilesystem::get_parent_path("/dir/subdir/file"),
            Some("/dir/subdir")
        );
        assert_eq!(
            CachedFilesystem::get_parent_path("/file"),
            Some("")
        );
        assert_eq!(
            CachedFilesystem::get_parent_path("file"),
            None
        );
    }
}
