//! VantisOS Kernel v1.5.0 "Quantum Ready"
//! 
//! A modern, quantum-ready operating system kernel
//! 
//! ## Features
//! - x86_64 architecture support
//! - Preemptive multitasking with priority scheduling
//! - Virtual memory with 4-level paging
//! - Hardware interrupt handling
//! - Quantum computing simulation
//! - Post-quantum cryptography

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(str_as_str)]
#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(ambiguous_glob_reexports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(clippy::empty_line_after_doc_comments)]
// Clippy allows for kernel development
#![allow(clippy::collapsible_match)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::new_without_default)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::map_identity)]
#![allow(clippy::iter_kv_map)]
#![allow(clippy::unnecessary_sort_by)]
#![allow(clippy::manual_clamp)]
#![allow(clippy::fn_to_numeric_cast)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(clippy::for_kv_map)]
#![allow(clippy::iter_cloned_collect)]
#![allow(static_mut_refs)]
#![allow(dead_code)]

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
pub mod update;
pub mod archive;
pub mod shell;

// GUI subsystem for desktop environment
pub mod gui;

// Installation wizard
pub mod installer;

// Desktop applications
pub mod apps;

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
#[derive(Debug, Clone, Copy)]
pub struct BootInfo {
    pub total_memory: u64,
    pub cmdline: [u8; 256],
    pub cmdline_len: usize,
}

impl Default for BootInfo {
    fn default() -> Self {
        Self {
            total_memory: 0,
            cmdline: [0; 256],
            cmdline_len: 0,
        }
    }
}

/// Global boot information
pub static mut BOOT_INFO: BootInfo = BootInfo {
    total_memory: 0,
    cmdline: [0; 256],
    cmdline_len: 0,
};

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start(multiboot_info: usize) -> ! {
    // Initialize serial for debugging
    arch::serial::init();
    
    serial_println!();
    serial_println!("╔══════════════════════════════════════════════════════════════╗");
    serial_println!("║     VantisOS v{} &quot;{}&quot;               ║", VERSION, CODENAME);
    serial_println!("║     Quantum-Ready Operating System                           ║");
    serial_println!("╚══════════════════════════════════════════════════════════════╝");
    serial_println!();
    
    // Parse multiboot info
    parse_multiboot_info(multiboot_info);
    
    // Phase 1: CPU Configuration
    serial_println!("\n[Phase 1] CPU Configuration");
    serial_println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    serial_println!("  [→] Initializing GDT...");
    arch::gdt::init();
    serial_println!("      └─ GDT loaded with TSS");
    
    serial_println!("  [→] Initializing IDT...");
    interrupts::init_idt();
    serial_println!("      └─ IDT loaded with exception handlers");
    
    serial_println!("  [→] Remapping PIC...");
    interrupts::remap_pic();
    serial_println!("      └─ PIC remapped to vectors 32-47");
    
    // Phase 2: Memory Management
    serial_println!("\n[Phase 2] Memory Management");
    serial_println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    serial_println!("  [→] Initializing memory management...");
    memory::init();
    
    serial_println!("  [→] Initializing heap...");
    unsafe {
        let heap_start = 0xFFFF800000200000usize;
        ALLOCATOR.lock().init(heap_start as *mut u8, HEAP_SIZE);
    }
    serial_println!("      └─ Heap: 16 MB at {:#x}", 0xFFFF800000200000u64);
    
    // Phase 3: Process Management
    serial_println!("\n[Phase 3] Process Management");
    serial_println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    serial_println!("  [→] Initializing process manager...");
    process::init();
    
    // Phase 4: Device Drivers
    serial_println!("\n[Phase 4] Device Drivers");
    serial_println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    serial_println!("  [→] Initializing VGA...");
    drivers::vga::init();
    
    serial_println!("  [→] Initializing keyboard...");
    drivers::keyboard::init();
    
    serial_println!("  [→] Initializing timer (100 Hz)...");
    drivers::timer::init(100);
    
    // Phase 5: Subsystems
    serial_println!("\n[Phase 5] Subsystems");
    serial_println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    serial_println!("  [→] Initializing VFS...");
    fs::init();
    
    serial_println!("  [→] Initializing IPC...");
    ipc::init();
    
    serial_println!("  [→] Initializing security...");
    security::init();
    
    serial_println!("  [→] Initializing syscalls...");
    syscall::init();
    
    serial_println!("  [→] Initializing quantum module...");
    quantum::init();
    
    // Enable interrupts
    serial_println!("\n[Boot] Enabling interrupts...");
    interrupts::enable();
    
    // Print welcome on VGA
    serial_println!("\n[Boot] Initializing VGA console...");
    
    {
        let mut vga = drivers::vga::WRITER.lock();
        vga.clear_screen();
        vga.set_color(drivers::vga::Color::LightCyan, drivers::vga::Color::Blue);
        vga.write_string("╔══════════════════════════════════════════════════════════════════════════════╗\n");
        vga.write_string("║                      VantisOS v1.5.0 &quot;Quantum Ready&quot;                        ║\n");
        vga.write_string("╚══════════════════════════════════════════════════════════════════════════════╝\n\n");
        vga.set_color(drivers::vga::Color::White, drivers::vga::Color::Blue);
        vga.write_string("  System initialized successfully!\n\n");
        vga.write_string("  Features:\n");
        vga.set_color(drivers::vga::Color::LightGreen, drivers::vga::Color::Blue);
        vga.write_string("    ✓ x86_64 kernel with preemptive multitasking\n");
        vga.write_string("    ✓ Virtual memory with 4-level paging\n");
        vga.write_string("    ✓ Hardware interrupt handling\n");
        vga.write_string("    ✓ Quantum computing simulation\n");
        vga.write_string("    ✓ Post-quantum cryptography\n\n");
        vga.set_color(drivers::vga::Color::Yellow, drivers::vga::Color::Blue);
        vga.write_string("  Type 'help' for available commands.\n\n");
        vga.set_color(drivers::vga::Color::White, drivers::vga::Color::Blue);
        vga.write_string("vantis> ");
    }
    
    serial_println!("\n╔══════════════════════════════════════════════════════════════╗");
    serial_println!("║              SYSTEM READY - BOOT COMPLETE                     ║");
    serial_println!("╚══════════════════════════════════════════════════════════════╝");
    
    // Main loop
    kernel_main()
}

/// Parse multiboot info
fn parse_multiboot_info(addr: usize) {
    unsafe {
        let info = &*(addr as *const MultibootInfo);
        BOOT_INFO.total_memory = (info.mem_upper as u64) * 1024;
        
        serial_println!("\n[Boot] Multiboot Information:");
        serial_println!("  Memory: {} MB lower, {} MB upper", 
            info.mem_lower / 1024, info.mem_upper / 1024);
        serial_println!("  Total: {} MB", BOOT_INFO.total_memory / 1024 / 1024);
        
        // Parse command line if present
        if info.flags & 0x04 != 0 {
            let cmdline_ptr = info.cmdline as *const u8;
            let mut i = 0;
            while i < 256 {
                let byte = *cmdline_ptr.add(i);
                if byte == 0 {
                    break;
                }
                BOOT_INFO.cmdline[i] = byte;
                i += 1;
            }
            BOOT_INFO.cmdline_len = i;
            
            let cmdline = core::str::from_utf8(&BOOT_INFO.cmdline[..i]);
            if let Ok(cmd) = cmdline {
                serial_println!("  Command line: {}", cmd);
            }
        }
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
        // Check for keyboard input
        if drivers::keyboard::has_key() {
            if let Some(event) = drivers::keyboard::read_key() {
                if event.state == drivers::keyboard::KeyState::Pressed {
                    if let Some(c) = event.key {
                        // Echo to VGA
                        let mut vga = drivers::vga::WRITER.lock();
                        vga.write_byte(c as u8);
                    }
                }
            }
        }
        
        interrupts::hlt();
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("\n");
    serial_println!("╔══════════════════════════════════════════════════════════════╗");
    serial_println!("║                    !!! KERNEL PANIC !!!                       ║");
    serial_println!("╚══════════════════════════════════════════════════════════════╝");
    serial_println!("\n{}", info);
    
    // Also print to VGA
    {
        let mut writer = drivers::vga::WRITER.lock();
        writer.set_color(drivers::vga::Color::White, drivers::vga::Color::Red);
        writer.write_string("\n!!! KERNEL PANIC !!!\n");
        writer.write_string("The system has encountered a fatal error.\n\n");
    }
    
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