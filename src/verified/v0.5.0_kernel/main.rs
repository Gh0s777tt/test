#![no_std]
#![no_main]

mod vga_console;
mod memory;
mod interrupt;
mod integration;
mod performance;
mod security;

use vga_console::{init as console_init, write_string, write_dec, write_hex32};
use memory::{init as memory_init, get_stats, MemoryStats};
use interrupt::{init_idt, load_idt, enable_interrupts};
use integration::{kernel_init, display_kernel_status, test_all_components};
use performance::{rdtsc, cycles_to_ms, record_boot_time, display_performance_stats};
use security::{init_security, display_security_stats};

// Multiboot header
#[repr(C, packed)]
struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

#[link_section = ".multiboot"]
#[no_mangle]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0x1BADB002,
    flags: 0,
    checksum: 0xE4524FFE,
};

// Boot information structure
#[repr(C, packed)]
struct BootInfo {
    magic: u32,
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
    drives_length: u32,
    drives_addr: u32,
    config_table: u32,
    boot_loader_name: u32,
    apm_table: u32,
    vbe_control_info: u32,
    vbe_mode_info: u32,
    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
}

// Memory map entry
#[repr(C, packed)]
struct MemoryMapEntry {
    size: u32,
    base: u64,
    length: u64,
    region_type: u32,
}

// Kernel entry point
#[no_mangle]
pub extern "C" fn _start(multiboot_info: *const BootInfo) -> ! {
    // Record boot start time
    let boot_start = rdtsc();
    
    // Print boot message
    write_string("VantisOS v0.5.0 - System Integration Test\n");
    write_string("======================================\n\n");
    
    // Print boot information
    write_string("Boot Information:\n");
    write_string("  Magic: ");
    unsafe {
        write_hex32((*multiboot_info).magic);
    }
    write_string("\n");
    
    write_string("  Lower Memory: ");
    unsafe {
        write_dec((*multiboot_info).mem_lower as u64);
    }
    write_string(" KB\n");
    
    write_string("  Upper Memory: ");
    unsafe {
        write_dec((*multiboot_info).mem_upper as u64);
    }
    write_string(" KB\n");
    
    // Parse memory map
    write_string("\nParsing Memory Map...\n");
    let mut total_memory = 0u64;
    let mut available_memory = 0u64;
    
    unsafe {
        let mmap_addr = (*multiboot_info).mmap_addr;
        let mmap_length = (*multiboot_info).mmap_length;
        
        let mut offset = 0u32;
        while offset < mmap_length {
            let entry = mmap_addr as *const MemoryMapEntry;
            let entry = &*entry.add((offset / 4) as usize);
            
            let _base = entry.base;
            let length = entry.length;
            let region_type = entry.region_type;
            
            total_memory += length;
            
            if region_type == 1 {
                // Available memory
                available_memory += length;
            }
            
            offset += entry.size + 4;
        }
    }
    
    write_string("  Total Memory: ");
    write_dec(total_memory / 1024);
    write_string(" KB\n");
    
    write_string("  Available Memory: ");
    write_dec(available_memory / 1024);
    write_string(" KB\n");
    
    // Initialize memory manager
    write_string("\nInitializing Memory Manager...\n");
    
    // Create simple memory map for initialization
    let memory_regions = [
        memory::MemoryRegion {
            base: 0,
            length: 0x100000, // First 1MB reserved
            region_type: memory::MemoryRegionType::Reserved as u32,
            acpi_attrs: 0,
        },
        memory::MemoryRegion {
            base: 0x100000,
            length: available_memory - 0x100000,
            region_type: memory::MemoryRegionType::Available as u32,
            acpi_attrs: 0,
        },
    ];
    
    memory_init(&memory_regions);
    write_string("  [OK] Memory manager initialized\n");
    
    // Initialize security subsystem
    write_string("\nInitializing Security Subsystem...\n");
    init_security();
    
    // Unified kernel initialization
    write_string("\n");
    kernel_init();
    
    // Display kernel status
    display_kernel_status();
    
    // Test all components
    test_all_components();
    
    // Record boot end time
    let boot_end = rdtsc();
    let boot_time = cycles_to_ms(boot_end - boot_start);
    record_boot_time(boot_time);
    
    write_string("\nBoot Time: ");
    write_dec(boot_time);
    write_string(" ms\n");
    
    // Display performance statistics
    display_performance_stats();
    
    // Display security statistics
    display_security_stats();
    
    write_string("\nSystem Integration Test Complete!\n");
    write_string("System halted.\n");
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    write_string("\nKERNEL PANIC!\n");
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}