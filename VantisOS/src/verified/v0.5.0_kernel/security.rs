// VantisOS v0.5.0 - Security Hardening
// Day 14: Security Hardening

#![allow(unused_unsafe)]

use crate::vga_console::{write_string, write_dec, write_hex32};

// Stack canary value
static mut STACK_CANARY: u64 = 0;

// Security flags
#[derive(Clone, Copy)]
pub struct SecurityFlags {
    pub stack_canary_enabled: bool,
    pub memory_protection_enabled: bool,
    pub secure_boot_enabled: bool,
    pub aslr_enabled: bool,
}

static mut SECURITY_FLAGS: SecurityFlags = SecurityFlags {
    stack_canary_enabled: false,
    memory_protection_enabled: false,
    secure_boot_enabled: false,
    aslr_enabled: false,
};

// Initialize security subsystem
pub fn init_security() {
    unsafe {
        // Generate random stack canary
        STACK_CANARY = generate_random_canary();
        
        // Enable security features
        SECURITY_FLAGS.stack_canary_enabled = true;
        SECURITY_FLAGS.memory_protection_enabled = true;
        SECURITY_FLAGS.secure_boot_enabled = false; // Requires secure boot infrastructure
        SECURITY_FLAGS.aslr_enabled = false; // Requires ASLR infrastructure
        
        write_string("Security subsystem initialized\n");
        write_string("  Stack canary: ");
        write_hex32(STACK_CANARY as u32);
        write_string("\n");
    }
}

// Generate random stack canary
fn generate_random_canary() -> u64 {
    // Use RDTSC as a simple random source
    // In production, use a proper CSPRNG
    unsafe {
        let mut high: u32;
        let mut low: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") low,
            out("edx") high,
            options(nomem, nostack)
        );
        
        // Mix with some constants
        let canary = ((high as u64) << 32) | (low as u64);
        canary ^ 0xDEADBEEFCAFEBABE
    }
}

// Get stack canary value
pub fn get_stack_canary() -> u64 {
    unsafe { STACK_CANARY }
}

// Verify stack canary
pub fn verify_stack_canary(canary: u64) -> bool {
    unsafe {
        if canary != STACK_CANARY {
            write_string("\n!!! STACK CANARY CORRUPTION DETECTED !!!\n");
            write_string("Expected: ");
            write_hex32(STACK_CANARY as u32);
            write_string("\n");
            write_string("Found: ");
            write_hex32(canary as u32);
            write_string("\n");
            write_string("Kernel panic: Stack buffer overflow detected!\n");
            kernel_panic();
        }
        true
    }
}

// Get security flags
pub fn get_security_flags() -> SecurityFlags {
    unsafe { SECURITY_FLAGS }
}

// Enable memory protection
pub fn enable_memory_protection() {
    unsafe {
        SECURITY_FLAGS.memory_protection_enabled = true;
        write_string("Memory protection enabled\n");
    }
}

// Disable memory protection (for debugging only)
pub fn disable_memory_protection() {
    unsafe {
        SECURITY_FLAGS.memory_protection_enabled = false;
        write_string("Memory protection disabled (DEBUG MODE)\n");
    }
}

// Check if memory protection is enabled
pub fn is_memory_protection_enabled() -> bool {
    unsafe { SECURITY_FLAGS.memory_protection_enabled }
}

// Kernel panic function
pub fn kernel_panic() -> ! {
    write_string("\n!!! KERNEL PANIC !!!\n");
    write_string("System halted.\n");
    
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

// Security check function
pub fn security_check(condition: bool, message: &str) {
    if !condition {
        write_string("\n!!! SECURITY CHECK FAILED !!!\n");
        write_string(message);
        write_string("\n");
        kernel_panic();
    }
}

// Memory access check
pub fn check_memory_access(address: u64, size: u64, write: bool) -> bool {
    unsafe {
        if !SECURITY_FLAGS.memory_protection_enabled {
            return true;
        }
        
        // Check if address is in kernel space
        if address >= 0xC0000000 {
            // Kernel space - always allow
            return true;
        }
        
        // User space - check permissions
        // For now, allow all user space access
        // In production, implement proper access control
        true
    }
}

// Validate pointer
pub fn validate_pointer<T>(ptr: *const T) -> bool {
    let addr = ptr as u64;
    
    // Check for null pointer
    if addr == 0 {
        return false;
    }
    
    // Check for kernel space
    if addr >= 0xC0000000 {
        return true;
    }
    
    // Check for user space
    if addr < 0xC0000000 {
        return true;
    }
    
    false
}

// Security statistics
#[derive(Clone, Copy)]
pub struct SecurityStats {
    pub stack_canary_checks: u64,
    pub memory_access_checks: u64,
    pub security_checks: u64,
}

static mut SECURITY_STATS: SecurityStats = SecurityStats {
    stack_canary_checks: 0,
    memory_access_checks: 0,
    security_checks: 0,
};

// Record stack canary check
pub fn record_stack_canary_check() {
    unsafe {
        SECURITY_STATS.stack_canary_checks += 1;
    }
}

// Record memory access check
pub fn record_memory_access_check() {
    unsafe {
        SECURITY_STATS.memory_access_checks += 1;
    }
}

// Record security check
pub fn record_security_check() {
    unsafe {
        SECURITY_STATS.security_checks += 1;
    }
}

// Display security statistics
pub fn display_security_stats() {
    unsafe {
        let stats = SECURITY_STATS;
        
        write_string("\n=== Security Statistics ===\n");
        
        write_string("Stack Canary Checks: ");
        write_dec(stats.stack_canary_checks);
        write_string("\n");
        
        write_string("Memory Access Checks: ");
        write_dec(stats.memory_access_checks);
        write_string("\n");
        
        write_string("Security Checks: ");
        write_dec(stats.security_checks);
        write_string("\n");
        
        write_string("\n");
    }
}