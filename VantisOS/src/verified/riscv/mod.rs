//! RISC-V Architecture Support Module
//! 
//! This module provides complete RISC-V architecture support for VantisOS.

pub mod boot;
pub mod mmu;
pub mod interrupt;
pub mod context;
pub mod sbi;

#[cfg(test)]
pub mod tests;

pub use boot::*;
pub use mmu::*;
pub use interrupt::*;
pub use context::*;
pub use sbi::*;

/// RISC-V architecture version
pub const RISCV_VERSION: &str = "RV64GC";

/// RISC-V ISA extensions
pub const RISCV_EXTENSIONS: &str = "IMAFDC";

/// Initialize RISC-V architecture
pub fn init() {
    // Initialize boot process
    boot::init();
    
    // Initialize MMU
    mmu::init();
    
    // Initialize interrupt handling
    interrupt::init();
    
    // Initialize SBI
    sbi::init();
}

/// Get RISC-V architecture information
pub fn get_info() -> &'static str {
    "RISC-V 64-bit (RV64GC) with IMAFDC extensions"
}

/// Check if running on RISC-V
pub fn is_riscv() -> bool {
    true
}