//! Security and Cryptography Module
//! Provides security features for VantisOS

pub mod crypto;
pub mod acl;

use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

/// Security context for a process
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// User ID
    pub uid: u32,
    /// Group ID
    pub gid: u32,
    /// Supplementary groups
    pub groups: Vec<u32>,
    /// Capabilities
    pub capabilities: u64,
    /// SELinux context
    pub selinux_context: String,
}

impl SecurityContext {
    /// Create a new security context for root
    pub fn root() -> Self {
        Self {
            uid: 0,
            gid: 0,
            groups: Vec::new(),
            capabilities: 0xFFFFFFFFFFFFFFFF, // All capabilities
            selinux_context: String::from("kernel:kernel:kernel_t"),
        }
    }
    
    /// Create a new security context for a user
    pub fn user(uid: u32, gid: u32) -> Self {
        Self {
            uid,
            gid,
            groups: Vec::new(),
            capabilities: 0,
            selinux_context: String::from("user:user:user_t"),
        }
    }
    
    /// Check if this context has a capability
    pub fn has_capability(&self, cap: Capability) -> bool {
        (self.capabilities & (1 << cap as u64)) != 0
    }
}

/// POSIX capabilities
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum Capability {
    /// Set UID/GID
    SetUid = 0,
    /// Set GID
    SetGid = 1,
    /// Sysadmin operations
    SysAdmin = 2,
    /// Network operations
    NetAdmin = 3,
    /// Raw I/O
    SysRawio = 4,
    /// Chown files
    Chown = 5,
    /// DAC override
    DacOverride = 6,
    /// DAC read search
    DacReadSearch = 7,
    /// Kill processes
    Kill = 8,
    /// Nice
    SysNice = 9,
    /// Module operations
    SysModule = 10,
    /// Reboot
    SysBoot = 11,
    /// Time operations
    SysTime = 12,
    /// Nice
    SysResource = 13,
}

/// Initialize security subsystem
pub fn init() {
    crypto::init();
}

/// Global security state
pub static SECURITY_STATE: Mutex<SecurityState> = Mutex::new(SecurityState {
    initialized: false,
    enforcing: true,
});

/// Security state
pub struct SecurityState {
    pub initialized: bool,
    pub enforcing: bool,
}