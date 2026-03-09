// Memory Protection
//
// This module provides memory protection for VantisOS, including:
// - Page table entry flags
// - Memory protection bits
// - Access control
// - Memory permissions

#![no_std]

/// Page table entry flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFlags {
    /// Present bit
    pub present: bool,
    /// Writable bit
    pub writable: bool,
    /// User accessible bit
    pub user_accessible: bool,
    /// Write-through cache
    pub write_through: bool,
    /// Cache disable
    pub cache_disable: bool,
    /// Accessed bit
    pub accessed: bool,
    /// Dirty bit
    pub dirty: bool,
    /// Global bit
    pub global: bool,
    /// No-execute bit
    pub no_execute: bool,
}

impl PageFlags {
    /// Create new page flags (default: present, writable, kernel)
    pub fn new() -> Self {
        PageFlags {
            present: true,
            writable: true,
            user_accessible: false,
            write_through: false,
            cache_disable: false,
            accessed: false,
            dirty: false,
            global: false,
            no_execute: false,
        }
    }

    /// Create kernel code page flags
    pub fn kernel_code() -> Self {
        PageFlags {
            present: true,
            writable: false,
            user_accessible: false,
            write_through: false,
            cache_disable: false,
            accessed: false,
            dirty: false,
            global: true,
            no_execute: false,
        }
    }

    /// Create kernel data page flags
    pub fn kernel_data() -> Self {
        PageFlags {
            present: true,
            writable: true,
            user_accessible: false,
            write_through: false,
            cache_disable: false,
            accessed: false,
            dirty: false,
            global: true,
            no_execute: true,
        }
    }

    /// Create user code page flags
    pub fn user_code() -> Self {
        PageFlags {
            present: true,
            writable: false,
            user_accessible: true,
            write_through: false,
            cache_disable: false,
            accessed: false,
            dirty: false,
            global: false,
            no_execute: false,
        }
    }

    /// Create user data page flags
    pub fn user_data() -> Self {
        PageFlags {
            present: true,
            writable: true,
            user_accessible: true,
            write_through: false,
            cache_disable: false,
            accessed: false,
            dirty: false,
            global: false,
            no_execute: true,
        }
    }

    /// Create read-only page flags
    pub fn read_only() -> Self {
        PageFlags {
            present: true,
            writable: false,
            user_accessible: false,
            write_through: false,
            cache_disable: false,
            accessed: false,
            dirty: false,
            global: false,
            no_execute: false,
        }
    }

    /// Create device memory page flags
    pub fn device_memory() -> Self {
        PageFlags {
            present: true,
            writable: true,
            user_accessible: false,
            write_through: true,
            cache_disable: true,
            accessed: false,
            dirty: false,
            global: false,
            no_execute: true,
        }
    }

    /// Convert to u64 for page table entry
    pub fn to_u64(&self) -> u64 {
        let mut flags = 0u64;

        if self.present {
            flags |= 1 << 0;
        }
        if self.writable {
            flags |= 1 << 1;
        }
        if self.user_accessible {
            flags |= 1 << 2;
        }
        if self.write_through {
            flags |= 1 << 3;
        }
        if self.cache_disable {
            flags |= 1 << 4;
        }
        if self.accessed {
            flags |= 1 << 5;
        }
        if self.dirty {
            flags |= 1 << 6;
        }
        if self.global {
            flags |= 1 << 8;
        }
        if self.no_execute {
            flags |= 1u64 << 63;
        }

        flags
    }

    /// Convert from u64
    pub fn from_u64(flags: u64) -> Self {
        PageFlags {
            present: (flags & (1 << 0)) != 0,
            writable: (flags & (1 << 1)) != 0,
            user_accessible: (flags & (1 << 2)) != 0,
            write_through: (flags & (1 << 3)) != 0,
            cache_disable: (flags & (1 << 4)) != 0,
            accessed: (flags & (1 << 5)) != 0,
            dirty: (flags & (1 << 6)) != 0,
            global: (flags & (1 << 8)) != 0,
            no_execute: (flags & (1u64 << 63)) != 0,
        }
    }

    /// Check if page is executable
    pub fn is_executable(&self) -> bool {
        !self.no_execute
    }

    /// Check if page is readable
    pub fn is_readable(&self) -> bool {
        self.present
    }

    /// Check if page is writable
    pub fn is_writable(&self) -> bool {
        self.writable
    }

    /// Check if page is user accessible
    pub fn is_user_accessible(&self) -> bool {
        self.user_accessible
    }
}

impl Default for PageFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory protection level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtectionLevel {
    /// No access
    None = 0,
    /// Read-only
    Read = 1,
    /// Read-write
    ReadWrite = 2,
    /// Read-execute
    ReadExecute = 3,
    /// Read-write-execute
    ReadWriteExecute = 4,
}

impl ProtectionLevel {
    /// Convert to page flags
    pub fn to_page_flags(&self, user_accessible: bool) -> PageFlags {
        match self {
            ProtectionLevel::None => PageFlags {
                present: false,
                writable: false,
                user_accessible,
                write_through: false,
                cache_disable: false,
                accessed: false,
                dirty: false,
                global: false,
                no_execute: true,
            },
            ProtectionLevel::Read => PageFlags {
                present: true,
                writable: false,
                user_accessible,
                write_through: false,
                cache_disable: false,
                accessed: false,
                dirty: false,
                global: false,
                no_execute: true,
            },
            ProtectionLevel::ReadWrite => PageFlags {
                present: true,
                writable: true,
                user_accessible,
                write_through: false,
                cache_disable: false,
                accessed: false,
                dirty: false,
                global: false,
                no_execute: true,
            },
            ProtectionLevel::ReadExecute => PageFlags {
                present: true,
                writable: false,
                user_accessible,
                write_through: false,
                cache_disable: false,
                accessed: false,
                dirty: false,
                global: false,
                no_execute: false,
            },
            ProtectionLevel::ReadWriteExecute => PageFlags {
                present: true,
                writable: true,
                user_accessible,
                write_through: false,
                cache_disable: false,
                accessed: false,
                dirty: false,
                global: false,
                no_execute: false,
            },
        }
    }

    /// Check if read is allowed
    pub fn can_read(&self) -> bool {
        *self >= ProtectionLevel::Read
    }

    /// Check if write is allowed
    pub fn can_write(&self) -> bool {
        matches!(
            self,
            ProtectionLevel::ReadWrite | ProtectionLevel::ReadWriteExecute
        )
    }

    /// Check if execute is allowed
    pub fn can_execute(&self) -> bool {
        matches!(
            self,
            ProtectionLevel::ReadExecute | ProtectionLevel::ReadWriteExecute
        )
    }
}

/// Memory access control
pub struct MemoryAccessControl {
    /// Current protection level
    protection_level: ProtectionLevel,
    /// User accessible flag
    user_accessible: bool,
}

impl MemoryAccessControl {
    /// Create new memory access control
    pub fn new(protection_level: ProtectionLevel, user_accessible: bool) -> Self {
        MemoryAccessControl {
            protection_level,
            user_accessible,
        }
    }

    /// Get page flags
    pub fn get_page_flags(&self) -> PageFlags {
        self.protection_level.to_page_flags(self.user_accessible)
    }

    /// Check if read access is allowed
    pub fn can_read(&self) -> bool {
        self.protection_level.can_read()
    }

    /// Check if write access is allowed
    pub fn can_write(&self) -> bool {
        self.protection_level.can_write()
    }

    /// Check if execute access is allowed
    pub fn can_execute(&self) -> bool {
        self.protection_level.can_execute()
    }

    /// Set protection level
    pub fn set_protection_level(&mut self, level: ProtectionLevel) {
        self.protection_level = level;
    }

    /// Get protection level
    pub fn get_protection_level(&self) -> ProtectionLevel {
        self.protection_level
    }

    /// Set user accessible
    pub fn set_user_accessible(&mut self, user_accessible: bool) {
        self.user_accessible = user_accessible;
    }

    /// Get user accessible
    pub fn is_user_accessible(&self) -> bool {
        self.user_accessible
    }
}

impl Default for MemoryAccessControl {
    fn default() -> Self {
        MemoryAccessControl {
            protection_level: ProtectionLevel::ReadWrite,
            user_accessible: false,
        }
    }
}

/// Memory protection domain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectionDomain {
    /// Kernel domain
    Kernel,
    /// User domain
    User,
    /// Device domain
    Device,
}

impl ProtectionDomain {
    /// Get default page flags for domain
    pub fn get_default_flags(&self) -> PageFlags {
        match self {
            ProtectionDomain::Kernel => PageFlags::kernel_data(),
            ProtectionDomain::User => PageFlags::user_data(),
            ProtectionDomain::Device => PageFlags::device_memory(),
        }
    }

    /// Check if address is in domain
    pub fn is_in_domain(&self, addr: u64) -> bool {
        match self {
            ProtectionDomain::Kernel => addr < 0xFFFF_8000_0000_0000,
            ProtectionDomain::User => addr >= 0x0000_0000_0000_0000 && addr < 0x0000_8000_0000_0000,
            ProtectionDomain::Device => addr >= 0xFFFF_8000_0000_0000,
        }
    }
}

/// Memory protection manager
pub struct MemoryProtectionManager {
    /// Current protection domain
    current_domain: ProtectionDomain,
}

impl MemoryProtectionManager {
    /// Create new memory protection manager
    pub fn new() -> Self {
        MemoryProtectionManager {
            current_domain: ProtectionDomain::Kernel,
        }
    }

    /// Set current protection domain
    pub fn set_domain(&mut self, domain: ProtectionDomain) {
        self.current_domain = domain;
    }

    /// Get current protection domain
    pub fn get_domain(&self) -> ProtectionDomain {
        self.current_domain
    }

    /// Check if access is allowed
    pub fn check_access(&self, addr: u64, access: ProtectionLevel) -> bool {
        let domain = self.get_domain_for_address(addr);
        
        match domain {
            ProtectionDomain::Kernel => {
                // Kernel can access everything
                true
            }
            ProtectionDomain::User => {
                // User can only access user memory
                if self.current_domain == ProtectionDomain::User {
                    access.can_read() || access.can_write() || access.can_execute()
                } else {
                    false
                }
            }
            ProtectionDomain::Device => {
                // Device memory requires kernel access
                self.current_domain == ProtectionDomain::Kernel
            }
        }
    }

    /// Get protection domain for address
    pub fn get_domain_for_address(&self, addr: u64) -> ProtectionDomain {
        if addr >= 0xFFFF_8000_0000_0000 {
            ProtectionDomain::Device
        } else if addr < 0x0000_8000_0000_0000 {
            ProtectionDomain::User
        } else {
            ProtectionDomain::Kernel
        }
    }

    /// Create page flags for address
    pub fn create_page_flags(&self, addr: u64, access: ProtectionLevel) -> PageFlags {
        let domain = self.get_domain_for_address(addr);
        let user_accessible = domain == ProtectionDomain::User;
        access.to_page_flags(user_accessible)
    }
}

impl Default for MemoryProtectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_flags_creation() {
        let flags = PageFlags::new();
        assert!(flags.present);
        assert!(flags.writable);
        assert!(!flags.user_accessible);
    }

    #[test]
    fn test_page_flags_kernel_code() {
        let flags = PageFlags::kernel_code();
        assert!(flags.present);
        assert!(!flags.writable);
        assert!(!flags.user_accessible);
        assert!(!flags.no_execute);
    }

    #[test]
    fn test_page_flags_user_data() {
        let flags = PageFlags::user_data();
        assert!(flags.present);
        assert!(flags.writable);
        assert!(flags.user_accessible);
        assert!(flags.no_execute);
    }

    #[test]
    fn test_page_flags_conversion() {
        let flags1 = PageFlags::user_data();
        let flags2 = PageFlags::from_u64(flags1.to_u64());
        assert_eq!(flags1.present, flags2.present);
        assert_eq!(flags1.writable, flags2.writable);
        assert_eq!(flags1.user_accessible, flags2.user_accessible);
    }

    #[test]
    fn test_protection_level() {
        assert!(ProtectionLevel::Read.can_read());
        assert!(!ProtectionLevel::Read.can_write());
        assert!(!ProtectionLevel::Read.can_execute());
        
        assert!(ProtectionLevel::ReadWrite.can_read());
        assert!(ProtectionLevel::ReadWrite.can_write());
        assert!(!ProtectionLevel::ReadWrite.can_execute());
        
        assert!(ProtectionLevel::ReadWriteExecute.can_read());
        assert!(ProtectionLevel::ReadWriteExecute.can_write());
        assert!(ProtectionLevel::ReadWriteExecute.can_execute());
    }

    #[test]
    fn test_memory_access_control() {
        let control = MemoryAccessControl::new(ProtectionLevel::ReadWrite, true);
        assert!(control.can_read());
        assert!(control.can_write());
        assert!(!control.can_execute());
        assert!(control.is_user_accessible());
    }

    #[test]
    fn test_protection_domain() {
        assert_eq!(
            ProtectionDomain::Kernel.get_default_flags().user_accessible,
            false
        );
        assert_eq!(
            ProtectionDomain::User.get_default_flags().user_accessible,
            true
        );
    }

    #[test]
    fn test_memory_protection_manager() {
        let manager = MemoryProtectionManager::new();
        assert_eq!(manager.get_domain(), ProtectionDomain::Kernel);
        
        let user_addr = 0x1000;
        let domain = manager.get_domain_for_address(user_addr);
        assert_eq!(domain, ProtectionDomain::User);
    }
}