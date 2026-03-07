//! System Call Interface
//! Provides POSIX-like system calls for userspace

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::fmt::Write;
use crate::fs;
use crate::process;

/// Maximum number of file descriptors per process
pub const MAX_FDS: usize = 256;

/// System call numbers
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum SyscallNum {
    /// Read from a file descriptor
    Read = 0,
    /// Write to a file descriptor
    Write = 1,
    /// Open a file
    Open = 2,
    /// Close a file descriptor
    Close = 3,
    /// Get process statistics
    Stat = 4,
    /// Get file statistics
    Fstat = 5,
    /// Get link statistics
    Lstat = 6,
    /// Set current file position
    Lseek = 8,
    /// Map files or devices into memory
    Mmap = 9,
    /// Unmap memory
    Munmap = 11,
    /// Create a file control record
    Ioctl = 16,
    /// Create a directory
    Mkdir = 39,
    /// Remove a directory
    Rmdir = 40,
    /// Create a directory entry
    Link = 86,
    /// Remove a directory entry
    Unlink = 87,
    /// Read value of a symbolic link
    Readlink = 89,
    /// Change current working directory
    Chdir = 12,
    /// Get current working directory
    Getcwd = 183,
    /// Change current root directory
    Chroot = 161,
    /// Duplicate an open file descriptor
    Dup = 32,
    /// Duplicate2
    Dup2 = 33,
    /// Get process group ID
    Getpgrp = 111,
    /// Get process ID
    Getpid = 20,
    /// Get parent process ID
    Getppid = 64,
    /// Get user ID
    Getuid = 24,
    /// Get effective user ID
    Geteuid = 107,
    /// Get group ID
    Getgid = 47,
    /// Get effective group ID
    Getegid = 108,
    /// Create a new process
    Fork = 57,
    /// Execute a program
    Execve = 59,
    /// Terminate a process
    Exit = 60,
    /// Wait for process termination
    Wait4 = 61,
    /// Send a signal
    Kill = 62,
    /// Get process priority
    Getpriority = 140,
    /// Set process priority
    Setpriority = 141,
    /// Sleep for specified time
    Nanosleep = 35,
    /// Get time of day
    Gettimeofday = 96,
    /// Get system information
    Sysinfo = 99,
    /// Create a pipe
    Pipe = 22,
    /// Mount a filesystem
    Mount = 165,
    /// Unmount a filesystem
    Umount2 = 166,
    /// Get system name
    Uname = 63,
    /// Reserved
    _Reserved = 164,
    /// Sync filesystem caches
    Sync = 162,
    /// Reboot system
    Reboot = 169,
    /// Create a symbolic link
    Symlink = 88,
    /// Read directory entries
    Getdents = 78,
    /// Set file mode creation mask
    Umask = 95,
    /// Change file permissions
    Chmod = 90,
    /// Change file ownership
    Chown = 92,
    /// Socket operations
    Socket = 340,
    /// Connect socket
    Connect = 342,
    /// Accept connection
    Accept = 343,
    /// Bind socket
    Bind = 341,
    /// Listen for connections
    Listen = 344,
}

/// Stat structure for file information
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
}

impl Stat {
    pub fn new() -> Self {
        Self {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 1,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 4096,
            st_blocks: 0,
            st_atime: 0,
            st_atime_nsec: 0,
            st_mtime: 0,
            st_mtime_nsec: 0,
            st_ctime: 0,
            st_ctime_nsec: 0,
        }
    }
}

/// UTS name structure for uname
#[derive(Debug, Clone)]
#[repr(C)]
pub struct UtsName {
    pub sysname: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
}

impl UtsName {
    pub fn new() -> Self {
        let mut name = Self {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
        };
        
        // Set system name
        let sysname = b"VantisOS";
        name.sysname[..sysname.len()].copy_from_slice(sysname);
        
        // Set node name
        let nodename = b"vantis";
        name.nodename[..nodename.len()].copy_from_slice(nodename);
        
        // Set release version
        let release = b"1.5.0";
        name.release[..release.len()].copy_from_slice(release);
        
        // Set version
        let version = b"Quantum Ready";
        name.version[..version.len()].copy_from_slice(version);
        
        // Set machine
        let machine = b"x86_64";
        name.machine[..machine.len()].copy_from_slice(machine);
        
        name
    }
}

/// System information structure
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SysInfo {
    pub uptime: i64,
    pub loads: [u64; 3],
    pub totalram: u64,
    pub freeram: u64,
    pub sharedram: u64,
    pub bufferram: u64,
    pub totalswap: u64,
    pub freeswap: u64,
    pub procs: u16,
    pub totalhigh: u64,
    pub freehigh: u64,
    pub mem_unit: u32,
    pub _pad: [u8; 8],
}

impl SysInfo {
    pub fn new() -> Self {
        Self {
            uptime: 0,
            loads: [0; 3],
            totalram: 16 * 1024 * 1024, // 16 MB
            freeram: 12 * 1024 * 1024,  // 12 MB free
            sharedram: 0,
            bufferram: 512 * 1024,      // 512 KB buffers
            totalswap: 0,
            freeswap: 0,
            procs: 1,
            totalhigh: 0,
            freehigh: 0,
            mem_unit: 1,
            _pad: [0; 8],
        }
    }
}

/// System call result
pub type SyscallResult = Result<usize, SyscallError>;

/// System call errors
#[derive(Debug, Clone, Copy)]
pub enum SyscallError {
    /// Operation not permitted
    Perm = 1,
    /// No such file or directory
    NoEnt = 2,
    /// No such process
    Srch = 3,
    /// Interrupted system call
    Intr = 4,
    /// I/O error
    Io = 5,
    /// No such device or address
    Nxio = 6,
    /// Argument list too long
    Big = 7,
    /// Exec format error
    NoExec = 8,
    /// Bad file number
    BadF = 9,
    /// No child processes
    Child = 10,
    /// Try again
    Again = 11,
    /// Out of memory
    NoMem = 12,
    /// Permission denied
    Acces = 13,
    /// Bad address
    Fault = 14,
    /// Block device required
    NotBlk = 15,
    /// Device or resource busy
    Busy = 16,
    /// File exists
    Exist = 17,
    /// Cross-device link
    XDev = 18,
    /// No such device
    NoDev = 19,
    /// Not a directory
    NotDir = 20,
    /// Is a directory
    IsDir = 21,
    /// Invalid argument
    Inval = 22,
    /// File table overflow
    NFile = 23,
    /// Too many open files
    MFile = 24,
    /// Not a typewriter
    NotTy = 25,
    /// Text file busy
    TxtBsy = 26,
    /// File too large
    Big2 = 27,
    /// No space left on device
    NoSpc = 28,
    /// Illegal seek
    Spipe = 29,
    /// Read-only file system
    RoFs = 30,
    /// Too many links
    MLink = 31,
    /// Broken pipe
    Pipe = 32,
    /// Math argument out of domain
    Dom = 33,
    /// Math result not representable
    Range = 34,
}

/// Dispatch a system call
pub fn dispatch(num: usize, args: [usize; 6]) -> SyscallResult {
    match num {
        0 => sys_read(args[0], args[1] as *mut u8, args[2]),
        1 => sys_write(args[0], args[1] as *const u8, args[2]),
        2 => sys_open(args[0] as *const i8, args[1], args[2]),
        3 => sys_close(args[0]),
        20 => sys_getpid(),
        39 => sys_mkdir(args[0] as *const i8, args[1]),
        42 => sys_pipe(args[0] as *mut i32),
        60 => sys_exit(args[0] as i32),
        63 => sys_uname(args[0] as *mut UtsName),
        99 => sys_sysinfo(args[0] as *mut SysInfo),
        165 => sys_mount(args[0] as *const i8, args[1] as *const i8, args[2] as *const i8, args[3], args[4] as *const u8),
        _ => {
            // Unknown syscall
            Ok(0)
        }
    }
}

/// Read system call
pub fn sys_read(fd: usize, buf: *mut u8, count: usize) -> SyscallResult {
    if buf.is_null() || count == 0 {
        return Err(SyscallError::Inval);
    }
    
    // For now, only support stdin (fd 0)
    if fd == 0 {
        // TODO: Read from keyboard
        return Ok(0);
    }
    
    // For fd >= 3, use VFS
    let buffer = unsafe { core::slice::from_raw_parts_mut(buf, count) };
    
    // Try to read from device files
    if fd >= 3 {
        // Check if this is a device file
        // For now, just return 0
        return Ok(0);
    }
    
    Ok(0)
}

/// Write system call
pub fn sys_write(fd: usize, buf: *const u8, count: usize) -> SyscallResult {
    if buf.is_null() || count == 0 {
        return Err(SyscallError::Inval);
    }
    
    let buffer = unsafe { core::slice::from_raw_parts(buf, count) };
    
    // stdout (fd 1) or stderr (fd 2) - write to VGA
    if fd == 1 || fd == 2 {
        use crate::drivers::vga::WRITER;
        let mut writer = WRITER.lock();
        for &byte in buffer {
            writer.write_byte(byte);
        }
        return Ok(count);
    }
    
    // For fd >= 3, use VFS
    Ok(count)
}

/// Open system call
pub fn sys_open(path: *const i8, flags: usize, _mode: usize) -> SyscallResult {
    if path.is_null() {
        return Err(SyscallError::Fault);
    }
    
    // Convert C string to Rust string
    let path_str = unsafe {
        let len = (0..).take_while(|&i| *path.offset(i) != 0).count();
        let slice = core::slice::from_raw_parts(path as *const u8, len);
        core::str::from_utf8(slice).unwrap_or("")
    };
    
    // Parse flags
    let open_flags = fs::OpenFlags {
        read: (flags & 0o0) != 0 || (flags & 0o2) != 0,
        write: (flags & 0o1) != 0 || (flags & 0o2) != 0,
        create: (flags & 0o100) != 0,
        excl: (flags & 0o200) != 0,
        trunc: (flags & 0o1000) != 0,
        append: (flags & 0o2000) != 0,
        noctty: (flags & 0o4000) != 0,
    };
    
    // Try to open the file
    match fs::open(path_str, open_flags) {
        Ok(_fd) => {
            // Return file descriptor
            // For now, just return 3 (first available)
            Ok(3)
        }
        Err(_) => Err(SyscallError::NoEnt),
    }
}

/// Close system call
pub fn sys_close(_fd: usize) -> SyscallResult {
    Ok(0)
}

/// Get process ID
pub fn sys_getpid() -> SyscallResult {
    // For now, return PID 1
    Ok(1)
}

/// Exit system call
pub fn sys_exit(code: i32) -> SyscallResult {
    // Print exit message
    use crate::drivers::vga::WRITER;
    {
        let mut writer = WRITER.lock();
        writer.write_str("Process exited with code: ").unwrap();
        writer.write_str(&code.to_string()).unwrap();
        writer.write_str("\n").unwrap();
    }
    
    // Halt the CPU
    unsafe {
        core::arch::asm!("cli; hlt");
    }
    
    Ok(0)
}

/// Make directory
pub fn sys_mkdir(_path: *const i8, _mode: usize) -> SyscallResult {
    // TODO: Implement
    Ok(0)
}

/// Create pipe
pub fn sys_pipe(_pipefd: *mut i32) -> SyscallResult {
    // TODO: Implement
    Ok(0)
}

/// Get system name
pub fn sys_uname(buf: *mut UtsName) -> SyscallResult {
    if buf.is_null() {
        return Err(SyscallError::Fault);
    }
    
    unsafe {
        *buf = UtsName::new();
    }
    
    Ok(0)
}

/// Get system information
pub fn sys_sysinfo(info: *mut SysInfo) -> SyscallResult {
    if info.is_null() {
        return Err(SyscallError::Fault);
    }
    
    unsafe {
        *info = SysInfo::new();
    }
    
    Ok(0)
}

/// Mount filesystem
pub fn sys_mount(
    _source: *const i8,
    _target: *const i8,
    _fstype: *const i8,
    _flags: usize,
    _data: *const u8,
) -> SyscallResult {
    // TODO: Implement
    Ok(0)
}

/// Initialize syscall handler
pub fn init() {
    // Set up syscall handler (MSR or interrupt)
    // For now, we use interrupt-based syscalls
}