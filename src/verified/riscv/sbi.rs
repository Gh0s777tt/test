//! RISC-V SBI (Supervisor Binary Interface)
//! 
//! This module implements RISC-V SBI calls for interacting with the machine mode.

#![no_std]

use core::arch::asm;

/// SBI extension IDs
pub mod ext_id {
    /// Base extension
    pub const BASE: usize = 0x10;
    
    /// Timer extension
    pub const TIMER: usize = 0x54494D45;
    
    /// IPI extension
    pub const IPI: usize = 0x735049;
    
    /// RFENCE extension
    pub const RFENCE: usize = 0x52464E43;
    
    /// HSM extension
    pub const HSM: usize = 0x48534D;
    
    /// PMU extension
    pub const PMU: usize = 0x504D55;
    
    /// Console extension
    pub const CONSOLE: usize = 0x01;
    
    /// SRST extension
    pub const SRST: usize = 0x53525354;
}

/// SBI function IDs for BASE extension
pub mod base_fid {
    /// Get SBI specification version
    pub const GET_SBI_SPEC_VERSION: usize = 0;
    
    /// Get SBI implementation ID
    pub const GET_SBI_IMPL_ID: usize = 1;
    
    /// Get SBI implementation version
    pub const GET_SBI_IMPL_VERSION: usize = 2;
    
    /// Probe SBI extension
    pub const PROBE_EXTENSION: usize = 3;
    
    /// Get Mvendorid
    pub const GET_MVENDORID: usize = 4;
    
    /// Get Marchid
    pub const GET_MARCHID: usize = 5;
    
    /// Get Mimpid
    pub const GET_MIMPID: usize = 6;
}

/// SBI function IDs for TIMER extension
pub mod timer_fid {
    /// Set timer
    pub const SET_TIMER: usize = 0;
}

/// SBI function IDs for IPI extension
pub mod ipi_fid {
    /// Send IPI
    pub const SEND_IPI: usize = 0;
}

/// SBI function IDs for RFENCE extension
pub mod rfence_fid {
    /// Remote fence.i
    pub const REMOTE_FENCE_I: usize = 0;
    
    /// Remote sfence.vma
    pub const REMOTE_SFENCE_VMA: usize = 1;
    
    /// Remote sfence.vma.asid
    pub const REMOTE_SFENCE_VMA_ASID: usize = 2;
    
    /// Remote hfence.gvma
    pub const REMOTE_HFENCE_GVMA: usize = 3;
    
    /// Remote hfence.gvma.vmid
    pub const REMOTE_HFENCE_GVMA_VMID: usize = 4;
}

/// SBI function IDs for CONSOLE extension
pub mod console_fid {
    /// Write character
    pub const PUTCHAR: usize = 0;
    
    /// Read character
    pub const GETCHAR: usize = 1;
}

/// SBI function IDs for SRST extension
pub mod srst_fid {
    /// System reset
    pub const SYSTEM_RESET: usize = 0;
}

/// SBI error codes
pub mod error {
    /// Success
    pub const SUCCESS: isize = 0;
    
    /// Failed
    pub const FAILED: isize = -1;
    
    /// Not supported
    pub const NOT_SUPPORTED: isize = -2;
    
    /// Invalid param
    pub const INVALID_PARAM: isize = -3;
    
    /// Denied
    pub const DENIED: isize = -4;
    
    /// Invalid address
    pub const INVALID_ADDRESS: isize = -5;
    
    /// Already available
    pub const ALREADY_AVAILABLE: isize = -6;
    
    /// Already started
    pub const ALREADY_STARTED: isize = -7;
    
    /// Already stopped
    pub const ALREADY_STOPPED: isize = -8;
}

/// SBI specification version
#[derive(Debug, Clone, Copy)]
pub struct SbiSpecVersion {
    pub major: u32,
    pub minor: u32,
}

impl SbiSpecVersion {
    /// Create a new SBI specification version
    pub const fn new(major: u32, minor: u32) -> Self {
        SbiSpecVersion { major, minor }
    }
    
    /// Parse from raw value
    pub fn from_raw(value: usize) -> Self {
        let major = (value >> 24) as u32;
        let minor = (value & 0xFFFFFF) as u32;
        SbiSpecVersion::new(major, minor)
    }
}

/// SBI implementation information
#[derive(Debug, Clone, Copy)]
pub struct SbiImplInfo {
    pub impl_id: u32,
    pub impl_version: u32,
    pub mvendorid: u32,
    pub marchid: u32,
    pub mimpid: u32,
}

impl SbiImplInfo {
    /// Create a new SBI implementation information
    pub const fn new() -> Self {
        SbiImplInfo {
            impl_id: 0,
            impl_version: 0,
            mvendorid: 0,
            marchid: 0,
            mimpid: 0,
        }
    }
}

/// Make an SBI call
#[inline(always)]
pub fn sbi_call(
    ext: usize,
    fid: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> (isize, usize, usize, usize) {
    let mut error: isize;
    let mut value0: usize;
    let mut value1: usize;
    let mut value2: usize;
    
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => value0,
            inlateout("x11") arg1 => value1,
            inlateout("x12") arg2 => value2,
            inlateout("x17") ext => _,
            inlateout("x16") fid => _,
            in("x11") arg1,
            in("x12") arg2,
            in("x13") arg3,
            in("x14") arg4,
            in("x15") arg5,
            lateout("x10") error,
            lateout("x11") value0,
            lateout("x12") value1,
            lateout("x13") value2,
            clobber_abi("system"),
        );
    }
    
    (error, value0, value1, value2)
}

/// Get SBI specification version
pub fn get_sbi_spec_version() -> SbiSpecVersion {
    let (error, value, _, _) = sbi_call(
        ext_id::BASE,
        base_fid::GET_SBI_SPEC_VERSION,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    
    if error == error::SUCCESS {
        SbiSpecVersion::from_raw(value)
    } else {
        SbiSpecVersion::new(0, 1) // Default to version 0.1
    }
}

/// Get SBI implementation information
pub fn get_sbi_impl_info() -> SbiImplInfo {
    let mut info = SbiImplInfo::new();
    
    // Get implementation ID
    let (error, value, _, _) = sbi_call(
        ext_id::BASE,
        base_fid::GET_SBI_IMPL_ID,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    if error == error::SUCCESS {
        info.impl_id = value as u32;
    }
    
    // Get implementation version
    let (error, value, _, _) = sbi_call(
        ext_id::BASE,
        base_fid::GET_SBI_IMPL_VERSION,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    if error == error::SUCCESS {
        info.impl_version = value as u32;
    }
    
    // Get mvendorid
    let (error, value, _, _) = sbi_call(
        ext_id::BASE,
        base_fid::GET_MVENDORID,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    if error == error::SUCCESS {
        info.mvendorid = value as u32;
    }
    
    // Get marchid
    let (error, value, _, _) = sbi_call(
        ext_id::BASE,
        base_fid::GET_MARCHID,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    if error == error::SUCCESS {
        info.marchid = value as u32;
    }
    
    // Get mimpid
    let (error, value, _, _) = sbi_call(
        ext_id::BASE,
        base_fid::GET_MIMPID,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    if error == error::SUCCESS {
        info.mimpid = value as u32;
    }
    
    info
}

/// Probe SBI extension
pub fn probe_extension(ext: usize) -> bool {
    let (error, value, _, _) = sbi_call(
        ext_id::BASE,
        base_fid::PROBE_EXTENSION,
        ext,
        0,
        0,
        0,
        0,
        0,
    );
    
    error == error::SUCCESS && value != 0
}

/// Set timer
pub fn set_timer(stime_value: u64) {
    let _ = sbi_call(
        ext_id::TIMER,
        timer_fid::SET_TIMER,
        stime_value as usize,
        (stime_value >> 32) as usize,
        0,
        0,
        0,
        0,
    );
}

/// Send IPI
pub fn send_ipi(hart_mask: usize) {
    let _ = sbi_call(
        ext_id::IPI,
        ipi_fid::SEND_IPI,
        hart_mask,
        0,
        0,
        0,
        0,
        0,
    );
}

/// Remote fence.i
pub fn remote_fence_i(hart_mask: usize) {
    let _ = sbi_call(
        ext_id::RFENCE,
        rfence_fid::REMOTE_FENCE_I,
        hart_mask,
        0,
        0,
        0,
        0,
        0,
    );
}

/// Remote sfence.vma
pub fn remote_sfence_vma(hart_mask: usize, start: usize, size: usize) {
    let _ = sbi_call(
        ext_id::RFENCE,
        rfence_fid::REMOTE_SFENCE_VMA,
        hart_mask,
        start,
        size,
        0,
        0,
        0,
    );
}

/// Put character to console
pub fn console_putchar(c: u8) {
    let _ = sbi_call(
        ext_id::CONSOLE,
        console_fid::PUTCHAR,
        c as usize,
        0,
        0,
        0,
        0,
        0,
    );
}

/// Get character from console
pub fn console_getchar() -> Option<u8> {
    let (error, value, _, _) = sbi_call(
        ext_id::CONSOLE,
        console_fid::GETCHAR,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    
    if error == error::SUCCESS {
        Some(value as u8)
    } else {
        None
    }
}

/// System reset
pub fn system_reset(reset_type: u32, reset_reason: u32) -> ! {
    let _ = sbi_call(
        ext_id::SRST,
        srst_fid::SYSTEM_RESET,
        reset_type as usize,
        reset_reason as usize,
        0,
        0,
        0,
        0,
    );
    
    // Should never reach here
    loop {}
}

/// Initialize SBI
pub fn init() {
    // Get SBI specification version
    let spec_version = get_sbi_spec_version();
    
    // Get SBI implementation information
    let impl_info = get_sbi_impl_info();
    
    // Print SBI information
    print_sbi_info(&spec_version, &impl_info);
}

/// Print SBI information
fn print_sbi_info(spec_version: &SbiSpecVersion, impl_info: &SbiImplInfo) {
    let msg = format!(
        "SBI v{}.{}\nImpl ID: 0x{:08x}\nImpl Version: 0x{:08x}\n",
        spec_version.major, spec_version.minor, impl_info.impl_id, impl_info.impl_version
    );
    
    for byte in msg.as_bytes() {
        console_putchar(*byte);
    }
}

/// Simple format function (no_std compatible)
fn format(args: core::fmt::Arguments<'_>) -> String {
    // For now, just return a simple string
    // In production, this should use a proper formatting library
    String::from("SBI Info\n")
}

/// Simple String type for no_std
struct String {
    data: [u8; 256],
    len: usize,
}

impl String {
    fn from(s: &str) -> Self {
        let mut string = String {
            data: [0; 256],
            len: 0,
        };
        
        for (i, byte) in s.as_bytes().iter().enumerate() {
            if i < 256 {
                string.data[i] = *byte;
                string.len = i + 1;
            }
        }
        
        string
    }
    
    fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }
}