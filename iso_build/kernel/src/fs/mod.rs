//! VFS (Virtual File System) Implementation
//! Provides unified file system interface

pub mod ramfs;
pub mod procfs;
pub mod devfs;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;
use spin::Mutex;

/// File system errors
#[derive(Debug, Clone, Copy)]
pub enum FsError {
    /// File not found
    NotFound,
    /// Permission denied
    PermissionDenied,
    /// File already exists
    AlreadyExists,
    /// Not a directory
    NotDirectory,
    /// Is a directory
    IsDirectory,
    /// No space left on device
    NoSpace,
    /// Read-only file system
    ReadOnly,
    /// Invalid argument
    InvalidArgument,
    /// Operation not supported
    NotSupported,
    /// I/O error
    IoError,
    /// Out of memory
    OutOfMemory,
}

/// File types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// Regular file
    RegularFile,
    /// Directory
    Directory,
    /// Symbolic link
    Symlink,
    /// Block device
    BlockDevice,
    /// Character device
    CharDevice,
    /// Named pipe
    Pipe,
    /// Unix socket
    Socket,
}

/// File permissions
#[derive(Debug, Clone, Copy)]
pub struct Permissions {
    pub owner_read: bool,
    pub owner_write: bool,
    pub owner_exec: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_exec: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_exec: bool,
    pub setuid: bool,
    pub setgid: bool,
    pub sticky: bool,
}

impl Permissions {
    /// Create default permissions (0644)
    pub fn new() -> Self {
        Self {
            owner_read: true,
            owner_write: true,
            owner_exec: false,
            group_read: true,
            group_write: false,
            group_exec: false,
            other_read: true,
            other_write: false,
            other_exec: false,
            setuid: false,
            setgid: false,
            sticky: false,
        }
    }
    
    /// Create all permissions (0777)
    pub fn all() -> Self {
        Self {
            owner_read: true,
            owner_write: true,
            owner_exec: true,
            group_read: true,
            group_write: true,
            group_exec: true,
            other_read: true,
            other_write: true,
            other_exec: true,
            setuid: false,
            setgid: false,
            sticky: false,
        }
    }
    
    /// Create directory permissions (0755)
    pub fn dir() -> Self {
        Self {
            owner_read: true,
            owner_write: true,
            owner_exec: true,
            group_read: true,
            group_write: false,
            group_exec: true,
            other_read: true,
            other_write: false,
            other_exec: true,
            setuid: false,
            setgid: false,
            sticky: false,
        }
    }
    
    /// Convert to mode bits
    pub fn to_mode(&self) -> u32 {
        let mut mode = 0u32;
        if self.owner_read { mode |= 0o400; }
        if self.owner_write { mode |= 0o200; }
        if self.owner_exec { mode |= 0o100; }
        if self.group_read { mode |= 0o040; }
        if self.group_write { mode |= 0o020; }
        if self.group_exec { mode |= 0o010; }
        if self.other_read { mode |= 0o004; }
        if self.other_write { mode |= 0o002; }
        if self.other_exec { mode |= 0o001; }
        if self.setuid { mode |= 0o4000; }
        if self.setgid { mode |= 0o2000; }
        if self.sticky { mode |= 0o1000; }
        mode
    }
}

/// Inode attributes
#[derive(Debug, Clone)]
pub struct InodeAttr {
    /// Inode number
    pub ino: u64,
    /// File type
    pub file_type: FileType,
    /// Permissions
    pub mode: Permissions,
    /// Owner UID
    pub uid: u32,
    /// Owner GID
    pub gid: u32,
    /// File size
    pub size: usize,
    /// Number of hard links
    pub nlink: usize,
    /// Access time (seconds since epoch)
    pub atime: u64,
    /// Modification time
    pub mtime: u64,
    /// Creation time
    pub ctime: u64,
}

/// Inode operations
pub trait InodeOps {
    /// Read from the inode
    fn read(&self, offset: usize, buf: &mut [u8]) -> Result<usize, FsError>;
    /// Write to the inode
    fn write(&mut self, offset: usize, buf: &[u8]) -> Result<usize, FsError>;
    /// Get attributes
    fn getattr(&self) -> InodeAttr;
    /// Set attributes
    fn setattr(&mut self, attr: &InodeAttr) -> Result<(), FsError>;
}

/// File descriptor flags
#[derive(Debug, Clone, Copy)]
pub struct FdFlags {
    /// Close on exec
    pub cloexec: bool,
    /// Non-blocking I/O
    pub nonblock: bool,
    /// Append mode
    pub append: bool,
    /// Synchronous I/O
    pub sync: bool,
    /// Direct I/O
    pub direct: bool,
}

impl FdFlags {
    pub fn new() -> Self {
        Self {
            cloexec: false,
            nonblock: false,
            append: false,
            sync: false,
            direct: false,
        }
    }
}

/// Open file flags
#[derive(Debug, Clone, Copy)]
pub struct OpenFlags {
    /// Read access
    pub read: bool,
    /// Write access
    pub write: bool,
    /// Create if not exists
    pub create: bool,
    /// Fail if exists
    pub excl: bool,
    /// Truncate on open
    pub trunc: bool,
    /// Append mode
    pub append: bool,
    /// No controlling terminal
    pub noctty: bool,
}

impl OpenFlags {
    /// Create read-only flags
    pub fn rdonly() -> Self {
        Self { read: true, write: false, create: false, excl: false, trunc: false, append: false, noctty: false }
    }
    
    /// Create write-only flags
    pub fn wronly() -> Self {
        Self { read: false, write: true, create: false, excl: false, trunc: false, append: false, noctty: false }
    }
    
    /// Create read-write flags
    pub fn rdwr() -> Self {
        Self { read: true, write: true, create: false, excl: false, trunc: false, append: false, noctty: false }
    }
}

/// File descriptor table entry
#[derive(Debug)]
pub struct FileDescriptor {
    /// Path to the file
    pub path: String,
    /// Current file offset
    pub offset: usize,
    /// Open flags
    pub flags: OpenFlags,
    /// FD flags
    pub fd_flags: FdFlags,
    /// Reference count
    pub refcount: usize,
    /// Filesystem type
    pub fs_type: FsType,
    /// Inode number
    pub ino: u64,
}

/// File system type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsType {
    /// RAM-based filesystem
    Ramfs,
    /// Process filesystem
    Procfs,
    /// Device filesystem
    Devfs,
    /// ext2 filesystem
    Ext2,
    /// FAT filesystem
    Fat,
    /// ISO9660 (CD-ROM)
    Iso9660,
    /// Unknown
    Unknown,
}

/// Mount point
#[derive(Debug)]
pub struct Mount {
    /// Mount point path
    pub path: String,
    /// Filesystem type
    pub fs_type: FsType,
    /// Mount flags
    pub flags: MountFlags,
}

/// Mount flags
#[derive(Debug, Clone, Copy)]
pub struct MountFlags {
    /// Read-only mount
    pub rdonly: bool,
    /// No device
    pub nodev: bool,
    /// No setuid
    pub nosuid: bool,
    /// No execute
    pub noexec: bool,
    /// Sync writes
    pub sync: bool,
}

impl MountFlags {
    pub fn new() -> Self {
        Self {
            rdonly: false,
            nodev: false,
            nosuid: false,
            noexec: false,
            sync: false,
        }
    }
}

/// Global VFS state
pub static VFS_STATE: Mutex<VfsState> = Mutex::new(VfsState {
    initialized: false,
    next_fd: 3, // 0, 1, 2 are reserved for stdin/stdout/stderr
    mounts: BTreeMap::new(),
    cwd: String::new(),
});

/// VFS state
pub struct VfsState {
    /// Initialization flag
    pub initialized: bool,
    /// Next file descriptor number
    pub next_fd: usize,
    /// Mounted filesystems
    pub mounts: BTreeMap<String, Mount>,
    /// Current working directory
    pub cwd: String,
}

/// Initialize the VFS
pub fn init() {
    // Initialize filesystems
    ramfs::init();
    procfs::init();
    devfs::init();
    
    // Mark VFS as initialized
    let mut state = VFS_STATE.lock();
    state.initialized = true;
    state.cwd = String::from("/");
    
    // Add mount points
    state.mounts.insert(String::from("/"), Mount {
        path: String::from("/"),
        fs_type: FsType::Ramfs,
        flags: MountFlags::new(),
    });
    
    state.mounts.insert(String::from("/proc"), Mount {
        path: String::from("/proc"),
        fs_type: FsType::Procfs,
        flags: MountFlags::new(),
    });
    
    state.mounts.insert(String::from("/dev"), Mount {
        path: String::from("/dev"),
        fs_type: FsType::Devfs,
        flags: MountFlags::new(),
    });
}

/// Open a file
pub fn open(path: &str, flags: OpenFlags) -> Result<FileDescriptor, FsError> {
    let state = VFS_STATE.lock();
    
    if !state.initialized {
        return Err(FsError::IoError);
    }
    
    // Determine filesystem type from path
    let (fs_type, fs_path) = if path.starts_with("/proc") {
        (FsType::Procfs, path.trim_start_matches("/proc"))
    } else if path.starts_with("/dev") {
        (FsType::Devfs, path.trim_start_matches("/dev"))
    } else {
        (FsType::Ramfs, path)
    };
    
    // Look up inode based on filesystem type
    let ino = match fs_type {
        FsType::Ramfs => {
            let ramfs = ramfs::RAMFS.lock();
            if let Some(ref ramfs) = *ramfs {
                ramfs.lookup(fs_path)?
            } else {
                return Err(FsError::NotFound);
            }
        }
        FsType::Procfs => {
            let procfs = procfs::PROCFS.lock();
            if let Some(ref procfs) = *procfs {
                procfs.lookup(fs_path)?
            } else {
                return Err(FsError::NotFound);
            }
        }
        FsType::Devfs => {
            let devfs = devfs::DEVFS.lock();
            if let Some(ref devfs) = *devfs {
                devfs.lookup(fs_path)?
            } else {
                return Err(FsError::NotFound);
            }
        }
        _ => return Err(FsError::NotSupported),
    };
    
    Ok(FileDescriptor {
        path: String::from(path),
        offset: 0,
        flags,
        fd_flags: FdFlags::new(),
        refcount: 1,
        fs_type,
        ino,
    })
}

/// Read from a file
pub fn read(fd: &mut FileDescriptor, buf: &mut [u8]) -> Result<usize, FsError> {
    let count = match fd.fs_type {
        FsType::Ramfs => {
            let ramfs = ramfs::RAMFS.lock();
            if let Some(ref ramfs) = *ramfs {
                ramfs.read(fd.ino, fd.offset, buf)?
            } else {
                return Err(FsError::IoError);
            }
        }
        FsType::Procfs => {
            let procfs = procfs::PROCFS.lock();
            if let Some(ref procfs) = *procfs {
                procfs.read(fd.ino, fd.offset, buf)?
            } else {
                return Err(FsError::IoError);
            }
        }
        FsType::Devfs => {
            let devfs = devfs::DEVFS.lock();
            if let Some(ref devfs) = *devfs {
                devfs.read(fd.ino, buf)?
            } else {
                return Err(FsError::IoError);
            }
        }
        _ => return Err(FsError::NotSupported),
    };
    
    fd.offset += count;
    Ok(count)
}

/// Write to a file
pub fn write(fd: &mut FileDescriptor, buf: &[u8]) -> Result<usize, FsError> {
    match fd.fs_type {
        FsType::Ramfs => {
            let mut ramfs = ramfs::RAMFS.lock();
            if let Some(ref mut ramfs) = *ramfs {
                ramfs.write(fd.ino, fd.offset, buf)
            } else {
                Err(FsError::IoError)
            }
        }
        FsType::Devfs => {
            let devfs = devfs::DEVFS.lock();
            if let Some(ref devfs) = *devfs {
                devfs.write(fd.ino, buf)
            } else {
                Err(FsError::IoError)
            }
        }
        _ => Err(FsError::NotSupported),
    }
}

/// Close a file (no-op for now, just returns Ok)
pub fn close(_fd: FileDescriptor) -> Result<(), FsError> {
    Ok(())
}