//! System call interface for VantisOS

use spin::Mutex;

/// Syscall numbers
#[repr(usize)]
pub enum SyscallNumber {
    Exit = 0,
    Write = 1,
    Read = 2,
    Open = 3,
    Close = 4,
    GetPid = 5,
}

/// Initialize syscalls
pub fn init() {
    // Setup syscall MSRs
}