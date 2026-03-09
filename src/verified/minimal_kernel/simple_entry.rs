// Minimal Kernel Entry Point for VantisOS v0.5.0
// This is a simplified version to demonstrate multiboot header functionality

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

/// Multiboot header for bootloader compatibility
#[repr(C, packed)]
pub struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

/// Kernel stack size (8 MB)
const KERNEL_STACK_SIZE: usize = 8 * 1024 * 1024;

/// Kernel stack (aligned to 16 bytes)
#[link_section = ".bss"]
#[no_mangle]
static mut KERNEL_STACK: [u8; KERNEL_STACK_SIZE] = [0; KERNEL_STACK_SIZE];

/// Multiboot header (must be at the beginning of the kernel image)
#[link_section = ".multiboot"]
#[no_mangle]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0x1BADB002,
    flags: 0x00000003, // Page alignment + memory info
    checksum: 0xE4514FFF, // -(magic + flags)
};

/// Simple panic handler
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

/// Kernel halt function
fn kernel_halt() -> ! {
    loop {
        unsafe {
            asm!("cli", "hlt");
        }
    }
}

/// Kernel entry point called by bootloader
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Set up stack pointer
        asm!(
            "mov rsp, {stack_top}",
            stack_top = in(reg) KERNEL_STACK.as_ptr_range().end,
        );

        // Simple kernel main - just halt for now
        kernel_halt();
    }
}