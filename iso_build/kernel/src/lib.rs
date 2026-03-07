//! VantisOS Kernel v1.5.0 "Quantum Ready"
//! 
//! A modern, quantum-ready operating system kernel

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(asm_sym)]
#![feature(naked_functions)]
#![feature(new_uninit)]

extern crate alloc;

use core::panic::PanicInfo;

pub mod arch;
pub mod memory;
pub mod interrupts;
pub mod process;
pub mod syscall;
pub mod drivers;
pub mod fs;
pub mod ipc;
pub mod security;
pub mod quantum;

#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

/// Kernel version
pub const VERSION: &str = "1.5.0";

/// Kernel name
pub const NAME: &str = "VantisOS";

/// Kernel codename
pub const CODENAME: &str = "Quantum Ready";

/// Heap size (16 MB)
const HEAP_SIZE: usize = 16 * 1024 * 1024;

/// Kernel boot information
#[derive(Debug, Clone, Copy, Default)]
pub struct BootInfo {
    pub total_memory: u64,
    pub cmdline: [u8; 256],
    pub cmdline_len: usize,
}

/// Global boot information
pub static mut BOOT_INFO: BootInfo = BootInfo::default();

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start(multiboot_info: usize) -> ! {
    // Initialize serial for debugging
    arch::serial::init();
    
    arch::serial_println!();
    arch::serial_println!("========================================");
    arch::serial_println!("  VantisOS v{} &quot;{}&quot;", VERSION, CODENAME);
    arch::serial_println!("  Quantum-Ready Operating System");
    arch::serial_println!("========================================");
    arch::serial_println!();
    
    // Parse multiboot info
    parse_multiboot_info(multiboot_info);
    
    // Initialize GDT
    arch::serial_println!("[*] Initializing GDT...");
    arch::gdt::init();
    
    // Initialize IDT
    arch::serial_println!("[*] Initializing IDT...");
    interrupts::init_idt();
    
    // Remap PIC
    arch::serial_println!("[*] Remapping PIC...");
    interrupts::remap_pic();
    
    // Initialize memory
    arch::serial_println!("[*] Initializing memory...");
    memory::init();
    
    // Initialize heap
    arch::serial_println!("[*] Initializing heap...");
    unsafe {
        let heap_start = 0x200000usize;
        ALLOCATOR.lock().init(heap_start as *mut u8, HEAP_SIZE);
    }
    
    // Initialize subsystems
    arch::serial_println!("[*] Initializing processes...");
    process::init();
    
    arch::serial_println!("[*] Initializing VFS...");
    fs::init();
    
    arch::serial_println!("[*] Initializing IPC...");
    ipc::init();
    
    arch::serial_println!("[*] Initializing security...");
    security::init();
    
    arch::serial_println!("[*] Initializing syscalls...");
    syscall::init();
    
    arch::serial_println!("[*] Initializing quantum...");
    quantum::init();
    
    arch::serial_println!("[*] Initializing VGA...");
    drivers::vga::init();
    
    // Print welcome
    drivers::vga::WRITER.lock().set_color(drivers::vga::Color::LightCyan, drivers::vga::Color::Black);
    let _ = drivers::vga::WRITER.lock().write_str("VantisOS v1.5.0 &quot;Quantum Ready&quot;\n\n");
    drivers::vga::WRITER.lock().set_color(drivers::vga::Color::White, drivers::vga::Color::Black);
    let _ = drivers::vga::WRITER.lock().write_str("System ready. Welcome to VantisOS!\n\nvantis> ");
    
    arch::serial_println!("[OK] All subsystems initialized!");
    
    // Enable interrupts
    interrupts::enable();
    
    // Main loop
    kernel_main()
}

/// Parse multiboot info
fn parse_multiboot_info(addr: usize) {
    unsafe {
        let info = &*(addr as *const MultibootInfo);
        BOOT_INFO.total_memory = (info.mem_upper as u64) * 1024;
        arch::serial_println!("[*] Memory: {} MB", BOOT_INFO.total_memory / 1024 / 1024);
    }
}

/// Multiboot1 info
#[repr(C, packed)]
struct MultibootInfo {
    flags: u32,
    mem_lower: u32,
    mem_upper: u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms: [u32; 4],
    mmap_length: u32,
    mmap_addr: u32,
}

/// Main kernel loop
fn kernel_main() -> ! {
    loop {
        interrupts::hlt();
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    arch::serial_println!("\nKERNEL PANIC: {}", info);
    
    let mut writer = drivers::vga::WRITER.lock();
    writer.set_color(drivers::vga::Color::Red, drivers::vga::Color::Black);
    let _ = writer.write_str("\n!!! KERNEL PANIC !!!\n");
    
    loop {
        interrupts::disable();
        interrupts::hlt();
    }
}

/// Allocation error handler
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}