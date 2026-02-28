// System Call Interface
// System call dispatcher, table, handler registration, validation

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// System Call Numbers
// ============================================================================

/// System call numbers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum SyscallNumber {
    // Process system calls
    Exit = 1,
    Fork = 2,
    Exec = 3,
    Wait = 4,
    GetPid = 5,
    GetPpid = 6,
    
    // File system system calls
    Open = 10,
    Close = 11,
    Read = 12,
    Write = 13,
    Seek = 14,
    Stat = 15,
    Fstat = 16,
    Lstat = 17,
    Mkdir = 18,
    Rmdir = 19,
    Unlink = 20,
    Rename = 21,
    Chmod = 22,
    Chown = 23,
    
    // Memory system calls
    Mmap = 30,
    Munmap = 31,
    Brk = 32,
    Mprotect = 33,
    
    // Network system calls
    Socket = 40,
    Bind = 41,
    Listen = 42,
    Accept = 43,
    Connect = 44,
    Send = 45,
    Recv = 46,
    Sendto = 47,
    Recvfrom = 48,
    
    // IPC system calls
    Pipe = 50,
    Pipe2 = 51,
    Msgget = 52,
    Msgsnd = 53,
    Msgrcv = 54,
    Semget = 55,
    Semop = 56,
    Shmget = 57,
    Shmat = 58,
    Shmdt = 59,
    
    // Advanced system calls
    Ioctl = 60,
    Fcntl = 61,
    Poll = 62,
    Select = 63,
    EpollCreate = 64,
    EpollCtl = 65,
    EpollWait = 66,
    
    // Time system calls
    GetTimeOfDay = 70,
    ClockGetTime = 71,
    Nanosleep = 72,
    
    // Signal system calls
    Signal = 80,
    Sigaction = 81,
    Sigprocmask = 82,
    Kill = 83,
    
    // Information system calls
    Uname = 90,
    Sysinfo = 91,
    Getrlimit = 92,
    Setrlimit = 93,
}

impl SyscallNumber {
    pub fn from_u64(num: u64) -> Option<Self> {
        match num {
            1 => Some(SyscallNumber::Exit),
            2 => Some(SyscallNumber::Fork),
            3 => Some(SyscallNumber::Exec),
            4 => Some(SyscallNumber::Wait),
            5 => Some(SyscallNumber::GetPid),
            6 => Some(SyscallNumber::GetPpid),
            10 => Some(SyscallNumber::Open),
            11 => Some(SyscallNumber::Close),
            12 => Some(SyscallNumber::Read),
            13 => Some(SyscallNumber::Write),
            14 => Some(SyscallNumber::Seek),
            15 => Some(SyscallNumber::Stat),
            16 => Some(SyscallNumber::Fstat),
            17 => Some(SyscallNumber::Lstat),
            18 => Some(SyscallNumber::Mkdir),
            19 => Some(SyscallNumber::Rmdir),
            20 => Some(SyscallNumber::Unlink),
            21 => Some(SyscallNumber::Rename),
            22 => Some(SyscallNumber::Chmod),
            23 => Some(SyscallNumber::Chown),
            30 => Some(SyscallNumber::Mmap),
            31 => Some(SyscallNumber::Munmap),
            32 => Some(SyscallNumber::Brk),
            33 => Some(SyscallNumber::Mprotect),
            40 => Some(SyscallNumber::Socket),
            41 => Some(SyscallNumber::Bind),
            42 => Some(SyscallNumber::Listen),
            43 => Some(SyscallNumber::Accept),
            44 => Some(SyscallNumber::Connect),
            45 => Some(SyscallNumber::Send),
            46 => Some(SyscallNumber::Recv),
            47 => Some(SyscallNumber::Sendto),
            48 => Some(SyscallNumber::Recvfrom),
            50 => Some(SyscallNumber::Pipe),
            51 => Some(SyscallNumber::Pipe2),
            52 => Some(SyscallNumber::Msgget),
            53 => Some(SyscallNumber::Msgsnd),
            54 => Some(SyscallNumber::Msgrcv),
            55 => Some(SyscallNumber::Semget),
            56 => Some(SyscallNumber::Semop),
            57 => Some(SyscallNumber::Shmget),
            58 => Some(SyscallNumber::Shmat),
            59 => Some(SyscallNumber::Shmdt),
            60 => Some(SyscallNumber::Ioctl),
            61 => Some(SyscallNumber::Fcntl),
            62 => Some(SyscallNumber::Poll),
            63 => Some(SyscallNumber::Select),
            64 => Some(SyscallNumber::EpollCreate),
            65 => Some(SyscallNumber::EpollCtl),
            66 => Some(SyscallNumber::EpollWait),
            70 => Some(SyscallNumber::GetTimeOfDay),
            71 => Some(SyscallNumber::ClockGetTime),
            72 => Some(SyscallNumber::Nanosleep),
            80 => Some(SyscallNumber::Signal),
            81 => Some(SyscallNumber::Sigaction),
            82 => Some(SyscallNumber::Sigprocmask),
            83 => Some(SyscallNumber::Kill),
            90 => Some(SyscallNumber::Uname),
            91 => Some(SyscallNumber::Sysinfo),
            92 => Some(SyscallNumber::Getrlimit),
            93 => Some(SyscallNumber::Setrlimit),
            _ => None,
        }
    }
}

// ============================================================================
// System Call Handler
// ============================================================================

/// System call handler function type
pub type SyscallHandler = fn(args: &[u64]) -> Result<u64, &'static str>;

/// System call entry
#[derive(Debug, Clone)]
pub struct SyscallEntry {
    pub number: SyscallNumber,
    pub name: &'static str,
    pub handler: SyscallHandler,
    pub num_args: usize,
}

// ============================================================================
// System Call Table
// ============================================================================

/// System call table
pub struct SyscallTable {
    entries: BTreeMap<u64, SyscallEntry>,
    next_id: AtomicU64,
}

impl SyscallTable {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            next_id: AtomicU64::new(1),
        }
    }

    /// Register system call
    pub fn register(&mut self, entry: SyscallEntry) -> Result<(), &'static str> {
        let num = entry.number as u64;
        if self.entries.contains_key(&num) {
            return Err("System call already registered");
        }
        self.entries.insert(num, entry);
        Ok(())
    }

    /// Get system call entry
    pub fn get(&self, number: u64) -> Option<&SyscallEntry> {
        self.entries.get(&number)
    }

    /// List all system calls
    pub fn list(&self) -> Vec<&SyscallEntry> {
        self.entries.values().collect()
    }

    /// Get system call count
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

// ============================================================================
// System Call Dispatcher
// ============================================================================

/// System call dispatcher
pub struct SyscallDispatcher {
    table: SyscallTable,
    call_count: AtomicU64,
    error_count: AtomicU64,
}

impl SyscallDispatcher {
    pub fn new(table: SyscallTable) -> Self {
        Self {
            table,
            call_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
        }
    }

    /// Dispatch system call
    pub fn dispatch(&self, number: u64, args: &[u64]) -> Result<u64, &'static str> {
        self.call_count.fetch_add(1, Ordering::SeqCst);

        // Validate system call number
        let syscall_num = SyscallNumber::from_u64(number)
            .ok_or("Invalid system call number")?;

        // Get system call entry
        let entry = self.table.get(number)
            .ok_or("System call not found")?;

        // Validate argument count
        if args.len() != entry.num_args {
            self.error_count.fetch_add(1, Ordering::SeqCst);
            return Err("Invalid argument count");
        }

        // Call handler
        (entry.handler)(args)
    }

    /// Get statistics
    pub fn get_stats(&self) -> SyscallStats {
        SyscallStats {
            total_calls: self.call_count.load(Ordering::SeqCst),
            total_errors: self.error_count.load(Ordering::SeqCst),
            registered_syscalls: self.table.count(),
        }
    }
}

/// System call statistics
#[derive(Debug, Clone)]
pub struct SyscallStats {
    pub total_calls: u64,
    pub total_errors: u64,
    pub registered_syscalls: usize,
}

// ============================================================================
// System Call Validation
// ============================================================================

/// System call validator
pub struct SyscallValidator {
    max_args: usize,
    max_string_length: usize,
    max_buffer_size: usize,
}

impl Default for SyscallValidator {
    fn default() -> Self {
        Self {
            max_args: 6,
            max_string_length: 4096,
            max_buffer_size: 1024 * 1024, // 1MB
        }
    }
}

impl SyscallValidator {
    pub fn new(max_args: usize, max_string_length: usize, max_buffer_size: usize) -> Self {
        Self {
            max_args,
            max_string_length,
            max_buffer_size,
        }
    }

    /// Validate system call number
    pub fn validate_number(&self, number: u64) -> Result<(), &'static str> {
        if SyscallNumber::from_u64(number).is_none() {
            return Err("Invalid system call number");
        }
        Ok(())
    }

    /// Validate argument count
    pub fn validate_arg_count(&self, count: usize) -> Result<(), &'static str> {
        if count > self.max_args {
            return Err("Too many arguments");
        }
        Ok(())
    }

    /// Validate string argument
    pub fn validate_string(&self, _str: &str) -> Result<(), &'static str> {
        // In real implementation, check string length
        Ok(())
    }

    /// Validate buffer size
    pub fn validate_buffer_size(&self, size: usize) -> Result<(), &'static str> {
        if size > self.max_buffer_size {
            return Err("Buffer too large");
        }
        Ok(())
    }

    /// Validate pointer
    pub fn validate_pointer(&self, ptr: u64) -> Result<(), &'static str> {
        if ptr == 0 {
            return Err("Null pointer");
        }
        Ok(())
    }

    /// Validate file descriptor
    pub fn validate_fd(&self, fd: u64) -> Result<(), &'static str> {
        if fd < 3 {
            return Err("Invalid file descriptor");
        }
        Ok(())
    }

    /// Validate permissions
    pub fn validate_permissions(&self, perms: u32) -> Result<(), &'static str> {
        if perms > 0o777 {
            return Err("Invalid permissions");
        }
        Ok(())
    }

    /// Validate flags
    pub fn validate_flags(&self, flags: u64) -> Result<(), &'static str> {
        // In real implementation, validate specific flags
        Ok(())
    }
}

// ============================================================================
// Default System Call Handlers (Placeholders)
// ============================================================================

/// Default exit handler
pub fn sys_exit(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default fork handler
pub fn sys_fork(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default exec handler
pub fn sys_exec(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default wait handler
pub fn sys_wait(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default getpid handler
pub fn sys_getpid(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(1)
}

/// Default getppid handler
pub fn sys_getppid(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default open handler
pub fn sys_open(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(3)
}

/// Default close handler
pub fn sys_close(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default read handler
pub fn sys_read(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default write handler
pub fn sys_write(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default seek handler
pub fn sys_seek(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default stat handler
pub fn sys_stat(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default fstat handler
pub fn sys_fstat(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default lstat handler
pub fn sys_lstat(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default mkdir handler
pub fn sys_mkdir(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default rmdir handler
pub fn sys_rmdir(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default unlink handler
pub fn sys_unlink(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default rename handler
pub fn sys_rename(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default chmod handler
pub fn sys_chmod(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default chown handler
pub fn sys_chown(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default mmap handler
pub fn sys_mmap(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default munmap handler
pub fn sys_munmap(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default brk handler
pub fn sys_brk(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default mprotect handler
pub fn sys_mprotect(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default socket handler
pub fn sys_socket(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default bind handler
pub fn sys_bind(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default listen handler
pub fn sys_listen(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default accept handler
pub fn sys_accept(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default connect handler
pub fn sys_connect(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default send handler
pub fn sys_send(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default recv handler
pub fn sys_recv(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default sendto handler
pub fn sys_sendto(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default recvfrom handler
pub fn sys_recvfrom(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default pipe handler
pub fn sys_pipe(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default pipe2 handler
pub fn sys_pipe2(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default msgget handler
pub fn sys_msgget(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default msgsnd handler
pub fn sys_msgsnd(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default msgrcv handler
pub fn sys_msgrcv(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default semget handler
pub fn sys_semget(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default semop handler
pub fn sys_semop(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default shmget handler
pub fn sys_shmget(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default shmat handler
pub fn sys_shmat(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default shmdt handler
pub fn sys_shmdt(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default ioctl handler
pub fn sys_ioctl(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default fcntl handler
pub fn sys_fcntl(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default poll handler
pub fn sys_poll(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default select handler
pub fn sys_select(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default epoll_create handler
pub fn sys_epoll_create(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default epoll_ctl handler
pub fn sys_epoll_ctl(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default epoll_wait handler
pub fn sys_epoll_wait(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default gettimeofday handler
pub fn sys_gettimeofday(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default clock_gettime handler
pub fn sys_clock_gettime(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default nanosleep handler
pub fn sys_nanosleep(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default signal handler
pub fn sys_signal(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default sigaction handler
pub fn sys_sigaction(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default sigprocmask handler
pub fn sys_sigprocmask(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default kill handler
pub fn sys_kill(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default uname handler
pub fn sys_uname(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default sysinfo handler
pub fn sys_sysinfo(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default getrlimit handler
pub fn sys_getrlimit(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

/// Default setrlimit handler
pub fn sys_setrlimit(_args: &[u64]) -> Result<u64, &'static str> {
    Ok(0)
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall_number_conversion() {
        assert_eq!(SyscallNumber::from_u64(1), Some(SyscallNumber::Exit));
        assert_eq!(SyscallNumber::from_u64(10), Some(SyscallNumber::Open));
        assert_eq!(SyscallNumber::from_u64(999), None);
    }

    #[test]
    fn test_syscall_table() {
        let mut table = SyscallTable::new();
        
        let entry = SyscallEntry {
            number: SyscallNumber::Exit,
            name: "exit",
            handler: sys_exit,
            num_args: 1,
        };
        
        table.register(entry).unwrap();
        assert_eq!(table.count(), 1);
        
        let retrieved = table.get(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "exit");
    }

    #[test]
    fn test_syscall_dispatcher() {
        let mut table = SyscallTable::new();
        
        table.register(SyscallEntry {
            number: SyscallNumber::GetPid,
            name: "getpid",
            handler: sys_getpid,
            num_args: 0,
        }).unwrap();
        
        let dispatcher = SyscallDispatcher::new(table);
        let result = dispatcher.dispatch(5, &[]);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_syscall_validator() {
        let validator = SyscallValidator::default();
        
        assert!(validator.validate_number(1).is_ok());
        assert!(validator.validate_number(999).is_err());
        assert!(validator.validate_arg_count(3).is_ok());
        assert!(validator.validate_arg_count(10).is_err());
        assert!(validator.validate_pointer(100).is_ok());
        assert!(validator.validate_pointer(0).is_err());
        assert!(validator.validate_fd(3).is_ok());
        assert!(validator.validate_fd(2).is_err());
        assert!(validator.validate_permissions(0o755).is_ok());
        assert!(validator.validate_permissions(0o7777).is_err());
    }

    #[test]
    fn test_syscall_stats() {
        let mut table = SyscallTable::new();
        
        table.register(SyscallEntry {
            number: SyscallNumber::GetPid,
            name: "getpid",
            handler: sys_getpid,
            num_args: 0,
        }).unwrap();
        
        let dispatcher = SyscallDispatcher::new(table);
        dispatcher.dispatch(5, &[]).unwrap();
        
        let stats = dispatcher.get_stats();
        assert_eq!(stats.total_calls, 1);
        assert_eq!(stats.total_errors, 0);
        assert_eq!(stats.registered_syscalls, 1);
    }
}