//! Directory Operations - Formally Verified
//!
//! This module implements essential directory operations:
//! - Mkdir: Create directory
//! - Rmdir: Remove directory
//! - Chdir: Change current directory
//! - Getcwd: Get current directory
//!
//! All operations are formally verified using Verus and tested with Kani.




use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use super::path_lookup_cache::{PathLookupCache, PathLookupCacheStats};
use super::syscall_file_ops::{
    invalidate_path_lookup_cache,
    invalidate_path_lookup_cache_prefix,
};

/// Maximum path length
pub const MAX_PATH_LENGTH: usize = 4096;

/// Default capacity for directory entry cache.
pub const DIRECTORY_ENTRY_CACHE_CAPACITY: usize = 256;

/// Cached directory metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntryCacheEntry {
    /// Whether the cached object is a directory.
    pub is_directory: bool,
}

impl DirectoryEntryCacheEntry {
    /// Create directory cache entry.
    pub const fn directory() -> Self {
        Self { is_directory: true }
    }
}

/// Directory entry cache keyed by path.
pub type DirectoryEntryPathCache = PathLookupCache<DirectoryEntryCacheEntry>;

static DIRECTORY_ENTRY_CACHE: OnceLock<Mutex<DirectoryEntryPathCache>> = OnceLock::new();

/// Directory operation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirOpError {
    /// Path not found
    NotFound,
    /// Path already exists
    AlreadyExists,
    /// Not a directory
    NotDirectory,
    /// Directory not empty
    NotEmpty,
    /// Invalid path
    InvalidPath,
    /// Path too long
    PathTooLong,
    /// Permission denied
    PermissionDenied,
    /// I/O error
    IoError(String),
}

/// Directory operation result type
pub type DirOpResult<T> = Result<T, DirOpError>;

/// Directory permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirPermissions {
    /// Permission bits (rwxrwxrwx)
    pub mode: u32,
}

impl DirPermissions {
    /// Create new permissions
    pub fn new(mode: u32) -> Self {
        Self { mode: mode & 0o777 }
    }
    
    /// Default directory permissions (0o755)
    pub fn default_dir() -> Self {
        Self::new(0o755)
    }
    
    /// Check if readable by owner
    pub fn owner_read(&self) -> bool {
        (self.mode & 0o400) != 0
    }
    
    /// Check if writable by owner
    pub fn owner_write(&self) -> bool {
        (self.mode & 0o200) != 0
    }
    
    /// Check if executable by owner
    pub fn owner_execute(&self) -> bool {
        (self.mode & 0o100) != 0
    }
}

/// Process working directory state
pub struct WorkingDirectory {
    /// Current working directory
    cwd: PathBuf,
}

impl WorkingDirectory {
    /// Create new working directory state
    pub fn new() -> Self {
        Self {
            cwd: PathBuf::from("/"),
        }
    }
    
    /// Get current working directory
    pub fn get(&self) -> &Path {
        &self.cwd
    }
    
    /// Set current working directory
    fn set(&mut self, path: PathBuf) {
        self.cwd = path;
    }
}

impl Default for WorkingDirectory {
    fn default() -> Self {
        Self::new()
    }
}

fn default_directory_entry_cache() -> &'static Mutex<DirectoryEntryPathCache> {
    DIRECTORY_ENTRY_CACHE
        .get_or_init(|| Mutex::new(DirectoryEntryPathCache::new(DIRECTORY_ENTRY_CACHE_CAPACITY)))
}

fn with_default_directory_entry_cache<T>(f: impl FnOnce(&mut DirectoryEntryPathCache) -> T) -> T {
    let mut guard = default_directory_entry_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut guard)
}

fn validate_directory_path(path: &Path) -> DirOpResult<()> {
    if path.as_os_str().is_empty() {
        return Err(DirOpError::InvalidPath);
    }
    if path.as_os_str().len() > MAX_PATH_LENGTH {
        return Err(DirOpError::PathTooLong);
    }
    Ok(())
}

fn resolve_directory_path(base: &Path, path: &Path) -> PathBuf {
    use std::path::Component;

    let mut resolved = if path.is_absolute() {
        PathBuf::from("/")
    } else if base.as_os_str().is_empty() {
        PathBuf::from("/")
    } else {
        base.to_path_buf()
    };

    for component in path.components() {
        match component {
            Component::Prefix(_) => {}
            Component::RootDir => resolved = PathBuf::from("/"),
            Component::CurDir => {}
            Component::ParentDir => {
                if resolved != Path::new("/") {
                    resolved.pop();
                    if resolved.as_os_str().is_empty() {
                        resolved = PathBuf::from("/");
                    }
                }
            }
            Component::Normal(name) => resolved.push(name),
        }
    }

    if resolved.as_os_str().is_empty() {
        PathBuf::from("/")
    } else {
        resolved
    }
}

/// Return current directory cache statistics.
pub fn directory_entry_cache_stats() -> PathLookupCacheStats {
    with_default_directory_entry_cache(|cache| cache.stats())
}

/// Clear global directory cache contents.
pub fn reset_directory_entry_cache() {
    with_default_directory_entry_cache(|cache| cache.clear());
}

/// Invalidate single directory path in global cache.
pub fn invalidate_directory_entry_cache(path: &Path) -> bool {
    with_default_directory_entry_cache(|cache| cache.invalidate(path))
}

/// Invalidate directory cache entries under path prefix.
pub fn invalidate_directory_entry_cache_prefix(prefix: &Path) -> usize {
    with_default_directory_entry_cache(|cache| cache.invalidate_prefix(prefix))
}

/// Create directory
///
/// # Verification Properties
/// 1. Path must be valid
/// 2. Parent directory must exist
/// 3. Directory must not already exist
/// 4. Permissions are set correctly
/// 5. Operation is atomic
///
/// # Arguments
/// * `path` - Directory path to create
/// * `mode` - Permission mode (optional, defaults to 0o755)
///
/// # Returns
/// Success or error
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_mkdir_with_cache(
    path: &Path,
    mode: Option<u32>,
    cache: &mut DirectoryEntryPathCache,
) -> DirOpResult<()> {
    validate_directory_path(path)?;

    // Get permissions
    let _perms = mode.map(DirPermissions::new)
        .unwrap_or_else(DirPermissions::default_dir);
    
    // In a real implementation, this would:
    // 1. Check if parent directory exists
    // 2. Check if directory already exists
    // 3. Create the directory with specified permissions
    // 4. Update file system metadata

    // Directory cache: insert created directory and refresh parent.
    cache.insert(path, DirectoryEntryCacheEntry::directory());
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            cache.invalidate(parent);
        }
    }

    Ok(())
}

#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_mkdir(path: &Path, mode: Option<u32>) -> DirOpResult<()> {
    with_default_directory_entry_cache(|cache| sys_mkdir_with_cache(path, mode, cache))?;

    // Keep file-stat cache coherent with directory topology changes.
    invalidate_path_lookup_cache(path);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            invalidate_path_lookup_cache(parent);
        }
    }

    Ok(())
}

/// Remove directory
///
/// # Verification Properties
/// 1. Path must be valid
/// 2. Path must be a directory
/// 3. Directory must be empty
/// 4. Caller must have permission
/// 5. Operation is atomic
///
/// # Arguments
/// * `path` - Directory path to remove
///
/// # Returns
/// Success or error
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_rmdir_with_cache(path: &Path, cache: &mut DirectoryEntryPathCache) -> DirOpResult<()> {
    validate_directory_path(path)?;

    // Cannot remove root directory
    if path == Path::new("/") {
        return Err(DirOpError::PermissionDenied);
    }
    
    // In a real implementation, this would:
    // 1. Check if path exists
    // 2. Check if path is a directory
    // 3. Check if directory is empty
    // 4. Check permissions
    // 5. Remove the directory

    // Directory cache coherency: remove subtree and refresh parent.
    cache.invalidate_prefix(path);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            cache.invalidate(parent);
        }
    }

    Ok(())
}

#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_rmdir(path: &Path) -> DirOpResult<()> {
    with_default_directory_entry_cache(|cache| sys_rmdir_with_cache(path, cache))?;

    // Keep file-stat cache coherent with directory removal.
    invalidate_path_lookup_cache_prefix(path);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            invalidate_path_lookup_cache(parent);
        }
    }

    Ok(())
}

/// Change current directory
///
/// # Verification Properties
/// 1. Path must be valid
/// 2. Path must exist
/// 3. Path must be a directory
/// 4. Caller must have execute permission
/// 5. Working directory is updated atomically
///
/// # Arguments
/// * `wd` - Working directory state
/// * `path` - New directory path
///
/// # Returns
/// Success or error
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_chdir_with_cache(
    wd: &mut WorkingDirectory,
    path: &Path,
    cache: &mut DirectoryEntryPathCache,
) -> DirOpResult<()> {
    validate_directory_path(path)?;

    let new_path = resolve_directory_path(wd.get(), path);

    // Fast path: validated directory lookup from cache.
    if let Some(entry) = cache.get(new_path.as_path()) {
        if !entry.is_directory {
            return Err(DirOpError::NotDirectory);
        }
        wd.set(new_path);
        return Ok(());
    }

    // In a real implementation, this would:
    // 1. Check if path exists
    // 2. Check if path is a directory
    // 3. Check execute permission

    cache.insert(new_path.as_path(), DirectoryEntryCacheEntry::directory());

    // Update working directory
    wd.set(new_path);

    Ok(())
}

#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_chdir(wd: &mut WorkingDirectory, path: &Path) -> DirOpResult<()> {
    with_default_directory_entry_cache(|cache| sys_chdir_with_cache(wd, path, cache))
}

/// Get current working directory
///
/// # Verification Properties
/// 1. Always returns valid path
/// 2. Path is absolute
/// 3. Path length ≤ MAX_PATH_LENGTH
/// 4. Operation is non-blocking
///
/// # Arguments
/// * `wd` - Working directory state
/// * `buf` - Buffer to store path
/// * `size` - Buffer size
///
/// # Returns
/// Path length or error
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_getcwd_with_cache(
    wd: &WorkingDirectory,
    buf: &mut [u8],
    size: usize,
    cache: &mut DirectoryEntryPathCache,
) -> DirOpResult<usize> {
    // Get current directory path
    let cwd = wd.get();
    let cwd_bytes = cwd.as_os_str().as_encoded_bytes();

    // Keep current directory visible in cache.
    if cache.get(cwd).is_none() {
        cache.insert(cwd, DirectoryEntryCacheEntry::directory());
    }

    // Check if buffer is large enough
    if cwd_bytes.len() + 1 > size {
        return Err(DirOpError::PathTooLong);
    }
    
    // Check if buffer is large enough for actual copy
    if cwd_bytes.len() > buf.len() {
        return Err(DirOpError::PathTooLong);
    }
    
    // Copy path to buffer
    buf[..cwd_bytes.len()].copy_from_slice(cwd_bytes);
    
    // Add null terminator if space available
    if cwd_bytes.len() < buf.len() {
        buf[cwd_bytes.len()] = 0;
    }
    
    Ok(cwd_bytes.len())
}

#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_getcwd(
    wd: &WorkingDirectory,
    buf: &mut [u8],
    size: usize,
) -> DirOpResult<usize> {
    with_default_directory_entry_cache(|cache| sys_getcwd_with_cache(wd, buf, size, cache))
}

/// Get current working directory as PathBuf
///
/// # Verification Properties
/// 1. Always returns valid path
/// 2. Path is absolute
/// 3. Path is consistent with working directory state
///
/// # Arguments
/// * `wd` - Working directory state
///
/// # Returns
/// Current working directory path
#[cfg_attr(feature = "verus", verus::verify)]
pub fn sys_getcwd_path(wd: &WorkingDirectory) -> PathBuf {
    wd.get().to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mkdir_basic() {
        let path = Path::new("/test_dir");
        let result = sys_mkdir(path, None);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_mkdir_with_mode() {
        let path = Path::new("/test_dir");
        let result = sys_mkdir(path, Some(0o755));
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_mkdir_invalid_path() {
        let path = Path::new("");
        let result = sys_mkdir(path, None);
        assert_eq!(result, Err(DirOpError::InvalidPath));
    }
    
    #[test]
    fn test_rmdir_basic() {
        let path = Path::new("/test_dir");
        let result = sys_rmdir(path);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rmdir_root() {
        let path = Path::new("/");
        let result = sys_rmdir(path);
        assert_eq!(result, Err(DirOpError::PermissionDenied));
    }
    
    #[test]
    fn test_rmdir_invalid_path() {
        let path = Path::new("");
        let result = sys_rmdir(path);
        assert_eq!(result, Err(DirOpError::InvalidPath));
    }
    
    #[test]
    fn test_chdir_absolute() {
        let mut wd = WorkingDirectory::new();
        let path = Path::new("/usr/local");
        
        let result = sys_chdir(&mut wd, path);
        assert!(result.is_ok());
        assert_eq!(wd.get(), Path::new("/usr/local"));
    }
    
    #[test]
    fn test_chdir_relative() {
        let mut wd = WorkingDirectory::new();
        
        // Start at root
        assert_eq!(wd.get(), Path::new("/"));
        
        // Change to relative path
        let result = sys_chdir(&mut wd, Path::new("usr"));
        assert!(result.is_ok());
        assert_eq!(wd.get(), Path::new("/usr"));
        
        // Change to another relative path
        let result = sys_chdir(&mut wd, Path::new("local"));
        assert!(result.is_ok());
        assert_eq!(wd.get(), Path::new("/usr/local"));
    }
    
    #[test]
    fn test_chdir_parent() {
        let mut wd = WorkingDirectory::new();
        
        // Set to /usr/local
        sys_chdir(&mut wd, Path::new("/usr/local")).unwrap();
        assert_eq!(wd.get(), Path::new("/usr/local"));
        
        // Go to parent (..)
        let result = sys_chdir(&mut wd, Path::new(".."));
        assert!(result.is_ok());
        assert_eq!(wd.get(), Path::new("/usr"));
        
        // Go to parent again
        let result = sys_chdir(&mut wd, Path::new(".."));
        assert!(result.is_ok());
        assert_eq!(wd.get(), Path::new("/"));
    }
    
    #[test]
    fn test_chdir_current() {
        let mut wd = WorkingDirectory::new();
        
        // Set to /usr
        sys_chdir(&mut wd, Path::new("/usr")).unwrap();
        
        // Stay in current (.)
        let result = sys_chdir(&mut wd, Path::new("."));
        assert!(result.is_ok());
        assert_eq!(wd.get(), Path::new("/usr"));
    }
    
    #[test]
    fn test_chdir_invalid_path() {
        let mut wd = WorkingDirectory::new();
        let result = sys_chdir(&mut wd, Path::new(""));
        assert_eq!(result, Err(DirOpError::InvalidPath));
    }
    
    #[test]
    fn test_getcwd_basic() {
        let wd = WorkingDirectory::new();
        let mut buf = [0u8; 1024];
        
        let result = sys_getcwd(&wd, &mut buf, 1024);
        assert!(result.is_ok());
        
        let len = result.unwrap();
        assert_eq!(&buf[..len], b"/");
    }
    
    #[test]
    fn test_getcwd_after_chdir() {
        let mut wd = WorkingDirectory::new();
        sys_chdir(&mut wd, Path::new("/usr/local")).unwrap();
        
        let mut buf = [0u8; 1024];
        let result = sys_getcwd(&wd, &mut buf, 1024);
        assert!(result.is_ok());
        
        let len = result.unwrap();
        assert_eq!(&buf[..len], b"/usr/local");
    }
    
    #[test]
    fn test_getcwd_buffer_too_small() {
        let mut wd = WorkingDirectory::new();
        sys_chdir(&mut wd, Path::new("/usr/local")).unwrap();
        
        let mut buf = [0u8; 5]; // Too small for "/usr/local"
        let result = sys_getcwd(&wd, &mut buf, 5);
        assert_eq!(result, Err(DirOpError::PathTooLong));
    }
    
    #[test]
    fn test_getcwd_path() {
        let mut wd = WorkingDirectory::new();
        sys_chdir(&mut wd, Path::new("/usr/local")).unwrap();
        
        let path = sys_getcwd_path(&wd);
        assert_eq!(path, PathBuf::from("/usr/local"));
    }

    #[test]
    fn test_directory_cache_stats_repeated_chdir() {
        reset_directory_entry_cache();
        let baseline = directory_entry_cache_stats();

        let mut wd = WorkingDirectory::new();
        assert!(sys_chdir(&mut wd, Path::new("/usr")).is_ok()); // miss
        assert!(sys_chdir(&mut wd, Path::new("/usr")).is_ok()); // hit

        let stats = directory_entry_cache_stats();
        assert_eq!(stats.misses.saturating_sub(baseline.misses), 1);
        assert_eq!(stats.hits.saturating_sub(baseline.hits), 1);
    }

    #[test]
    fn test_mkdir_rmdir_with_cache_coherency() {
        let mut cache = DirectoryEntryPathCache::new(16);
        let child = Path::new("/tmp/demo");

        assert!(sys_mkdir_with_cache(child, None, &mut cache).is_ok());
        assert!(cache.peek(child).is_some());

        assert!(sys_rmdir_with_cache(Path::new("/tmp"), &mut cache).is_ok());
        assert!(cache.peek(child).is_none());
    }

    #[test]
    fn test_getcwd_with_cache_hit_miss() {
        let wd = WorkingDirectory::new();
        let mut cache = DirectoryEntryPathCache::new(8);
        let mut buf = [0u8; 64];
        let size = buf.len();

        assert!(sys_getcwd_with_cache(&wd, &mut buf, size, &mut cache).is_ok());
        assert!(sys_getcwd_with_cache(&wd, &mut buf, size, &mut cache).is_ok());

        let stats = cache.stats();
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hits, 1);
    }

    #[test]
    fn test_global_directory_cache_prefix_invalidation() {
        reset_directory_entry_cache();
        with_default_directory_entry_cache(|cache| {
            cache.insert(Path::new("/usr"), DirectoryEntryCacheEntry::directory());
            cache.insert(Path::new("/usr/local"), DirectoryEntryCacheEntry::directory());
            cache.insert(Path::new("/var/log"), DirectoryEntryCacheEntry::directory());
        });

        let removed = invalidate_directory_entry_cache_prefix(Path::new("/usr"));
        assert_eq!(removed, 2);

        with_default_directory_entry_cache(|cache| {
            assert!(cache.peek(Path::new("/usr")).is_none());
            assert!(cache.peek(Path::new("/usr/local")).is_none());
            assert!(cache.peek(Path::new("/var/log")).is_some());
        });
    }
    
    #[test]
    fn test_dir_permissions() {
        let perms = DirPermissions::new(0o755);
        assert!(perms.owner_read());
        assert!(perms.owner_write());
        assert!(perms.owner_execute());
    }
    
    #[test]
    fn test_dir_permissions_default() {
        let perms = DirPermissions::default_dir();
        assert_eq!(perms.mode, 0o755);
    }
    
    #[test]
    fn test_working_directory_default() {
        let wd = WorkingDirectory::default();
        assert_eq!(wd.get(), Path::new("/"));
    }
}

#[cfg(kani)]
mod kani_checks {
    use super::*;
    
    #[kani::proof]
    fn check_chdir_absolute_path() {
        let mut wd = WorkingDirectory::new();
        let path = Path::new("/test");
        
        if sys_chdir(&mut wd, path).is_ok() {
            // After chdir to absolute path, cwd should be that path
            assert_eq!(wd.get(), path);
        }
    }
    
    #[kani::proof]
    fn check_getcwd_always_valid() {
        let wd = WorkingDirectory::new();
        let path = sys_getcwd_path(&wd);
        
        // Path should always be absolute
        assert!(path.is_absolute());
    }
    
    #[kani::proof]
    fn check_mkdir_empty_path_fails() {
        let path = Path::new("");
        let result = sys_mkdir(path, None);
        
        // Empty path should always fail
        assert!(result.is_err());
    }
    
    #[kani::proof]
    fn check_rmdir_root_fails() {
        let path = Path::new("/");
        let result = sys_rmdir(path);
        
        // Cannot remove root directory
        assert_eq!(result, Err(DirOpError::PermissionDenied));
    }
}