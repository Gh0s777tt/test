// VantisOS v0.6.0 ARM64 Kernel Library Entry Point

#![no_std]
#![no_main]

pub mod arm64 {
    pub mod boot;
    pub mod memory;
    pub mod interrupt;
}

// Re-export main entry point
pub use arm64::boot::arm64_kernel_entry;

// Entry point - ARM64 boot parameters
#[no_mangle]
pub extern "C" fn _start(dtb_ptr: u64, dtb_size: u64, x0: u64, x1: u64, x2: u64, x3: u64) -> ! {
    // Call ARM64 boot process
    arm64::boot::arm64_kernel_entry(dtb_ptr, dtb_size, x0, x1, x2, x3);
    
    loop {}
}