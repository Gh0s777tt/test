// Kernel Entry Point
// 
// This module provides the kernel entry point and boot process for VantisOS.
// It handles the transition from bootloader to kernel execution.

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

/// Multiboot information structure passed by bootloader
#[repr(C)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u32; 4],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
}

/// Memory map entry
#[repr(C)]
pub struct MemoryMapEntry {
    pub size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub entry_type: u32,
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

/// External kernel initialization functions
extern "Rust" {
    fn kernel_init(boot_info: &MultibootInfo);
    fn kernel_main() -> !;
}

/// Kernel entry point called by bootloader
///
/// This is the first function executed by the kernel after the bootloader
/// transfers control. It performs minimal setup and calls kernel_init.
#[no_mangle]
#[naked]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Set up stack pointer
        asm!(
            "mov rsp, {stack_top}",
            stack_top = in(reg) KERNEL_STACK.as_ptr_range().end,
        );

        // Save multiboot info pointer in RDI (first argument)
        // The bootloader passes the multiboot info in EBX, move it to RDI
        asm!(
            "mov rdi, rbx",
        );

        // Call kernel_init
        asm!(
            "call {kernel_init}",
            kernel_init = sym kernel_init,
        );

        // Call kernel_main (should never return)
        asm!(
            "call {kernel_main}",
            kernel_main = sym kernel_main,
        );

        // If kernel_main returns, halt the CPU
        kernel_halt();
    }
}

/// Kernel halt function - stops CPU execution
#[inline(never)]
fn kernel_halt() -> ! {
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}

/// Panic handler for the kernel
///
/// This function is called when a panic occurs. It prints the panic information
/// and halts the CPU.
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // In a real implementation, we would print the panic information
    // to the screen or serial port. For now, we just halt.
    
    // TODO: Implement proper panic output
    // - Print panic location
    // - Print panic message
    // - Print backtrace if available
    
    kernel_halt();
}

/// Early boot helper functions

/// Check if we're in long mode (64-bit)
pub fn is_long_mode() -> bool {
    let mut eax: u32;
    let mut edx: u32;
    
    unsafe {
        asm!(
            "cpuid",
            inlateout("eax") 0x80000001 => eax,
            lateout("edx") edx,
        );
    }
    
    (edx & (1 << 29)) != 0
}

/// Get CPU vendor string
pub fn get_cpu_vendor() -> [u8; 12] {
    let mut vendor = [0u8; 12];
    let mut eax: u32;
    let mut ebx: u32;
    let mut ecx: u32;
    let mut edx: u32;
    
    unsafe {
        asm!(
            "cpuid",
            inlateout("eax") 0 => eax,
            lateout("ebx") ebx,
            lateout("ecx") ecx,
            lateout("edx") edx,
        );
    }
    
    // Vendor string is EBX, EDX, ECX
    unsafe {
        core::ptr::copy_nonoverlapping(
            &ebx as *const u32 as *const u8,
            vendor.as_mut_ptr(),
            4,
        );
        core::ptr::copy_nonoverlapping(
            &edx as *const u32 as *const u8,
            vendor.as_mut_ptr().add(4),
            4,
        );
        core::ptr::copy_nonoverlapping(
            &ecx as *const u32 as *const u8,
            vendor.as_mut_ptr().add(8),
            4,
        );
    }
    
    vendor
}

/// Get CPU features
pub fn get_cpu_features() -> u32 {
    let mut eax: u32;
    let mut ebx: u32;
    let mut ecx: u32;
    let mut edx: u32;
    
    unsafe {
        asm!(
            "cpuid",
            inlateout("eax") 1 => eax,
            lateout("ebx") ebx,
            lateout("ecx") ecx,
            lateout("edx") edx,
        );
    }
    
    // Return EDX which contains feature flags
    edx
}

/// Check if CPU supports SSE
pub fn has_sse() -> bool {
    (get_cpu_features() & (1 << 25)) != 0
}

/// Check if CPU supports SSE2
pub fn has_sse2() -> bool {
    (get_cpu_features() & (1 << 26)) != 0
}

/// Check if CPU supports AVX
pub fn has_avx() -> bool {
    let mut ecx: u32;
    
    unsafe {
        asm!(
            "cpuid",
            inlateout("eax") 1 => _,
            lateout("ecx") ecx,
        );
    }
    
    (ecx & (1 << 28)) != 0
}

/// Memory map iteration helper
pub struct MemoryMapIter<'a> {
    current: *const MemoryMapEntry,
    end: *const u8,
}

impl<'a> MemoryMapIter<'a> {
    pub fn new(mmap_addr: u32, mmap_length: u32) -> Self {
        let start = mmap_addr as *const MemoryMapEntry;
        let end = (mmap_addr + mmap_length) as *const u8;
        
        MemoryMapIter {
            current: start,
            end,
        }
    }
}

impl<'a> Iterator for MemoryMapIter<'a> {
    type Item = &'a MemoryMapEntry;
    
    fn next(&mut self) -> Option<Self::Item> {
        if (self.current as *const u8) >= self.end {
            return None;
        }
        
        unsafe {
            let entry = &*self.current;
            // Move to next entry (size + 4 bytes for size field itself)
            let entry_size = entry.size as usize + 4;
            self.current = (self.current as *const u8).add(entry_size) as *const MemoryMapEntry;
            Some(entry)
        }
    }
}

/// Memory entry types
pub const MEMORY_AVAILABLE: u32 = 1;
pub const MEMORY_RESERVED: u32 = 2;
pub const MEMORY_ACPI_RECLAIMABLE: u32 = 3;
pub const MEMORY_NVS: u32 = 4;
pub const MEMORY_UNUSABLE: u32 = 5;

/// Get total available memory from multiboot info
pub fn get_available_memory(boot_info: &MultibootInfo) -> u64 {
    let mut total: u64 = 0;
    
    if (boot_info.flags & (1 << 6)) != 0 {
        let iter = MemoryMapIter::new(boot_info.mmap_addr, boot_info.mmap_length);
        
        for entry in iter {
            if entry.entry_type == MEMORY_AVAILABLE {
                total += entry.length;
            }
        }
    } else {
        // Fall back to mem_lower and mem_upper
        total = (boot_info.mem_lower as u64) * 1024;
        total += (boot_info.mem_upper as u64) * 1024;
    }
    
    total
}

/// Boot information structure for kernel initialization
#[derive(Debug)]
pub struct BootInfo {
    pub memory_size: u64,
    pub command_line: Option<&'static str>,
    pub boot_loader_name: Option<&'static str>,
    pub modules: Vec<BootModule>,
}

#[derive(Debug)]
pub struct BootModule {
    pub start: u32,
    pub end: u32,
    pub string: Option<&'static str>,
}

/// Parse multiboot info into BootInfo structure
pub fn parse_boot_info(boot_info: &MultibootInfo) -> BootInfo {
    let memory_size = get_available_memory(boot_info);
    
    let command_line = if (boot_info.flags & (1 << 2)) != 0 {
        unsafe {
            let ptr = boot_info.cmdline as *const u8;
            let len = c_string_len(ptr);
            Some(core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len)))
        }
    } else {
        None
    };
    
    let boot_loader_name = if (boot_info.flags & (1 << 9)) != 0 {
        unsafe {
            let ptr = boot_info.boot_loader_name as *const u8;
            let len = c_string_len(ptr);
            Some(core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len)))
        }
    } else {
        None
    };
    
    let mut modules = Vec::new();
    if (boot_info.flags & (1 << 3)) != 0 && boot_info.mods_count > 0 {
        unsafe {
            let module_ptr = boot_info.mods_addr as *const u32;
            for i in 0..boot_info.mods_count {
                let start = *module_ptr.add((i * 2) as usize);
                let end = *module_ptr.add((i * 2 + 1) as usize);
                
                let string_ptr = *module_ptr.add((i * 2 + 2) as usize);
                let string = if string_ptr != 0 {
                    let ptr = string_ptr as *const u8;
                    let len = c_string_len(ptr);
                    Some(core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len)))
                } else {
                    None
                };
                
                modules.push(BootModule {
                    start,
                    end,
                    string,
                });
            }
        }
    }
    
    BootInfo {
        memory_size,
        command_line,
        boot_loader_name,
        modules,
    }
}

/// Get C string length
unsafe fn c_string_len(ptr: *const u8) -> usize {
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiboot_header_magic() {
        assert_eq!(MULTIBOOT_HEADER.magic, 0x1BADB002);
    }

    #[test]
    fn test_multiboot_header_checksum() {
        let sum = MULTIBOOT_HEADER.magic + MULTIBOOT_HEADER.flags + MULTIBOOT_HEADER.checksum;
        assert_eq!(sum & 0xFFFFFFFF, 0);
    }

    #[test]
    fn test_kernel_stack_size() {
        assert_eq!(KERNEL_STACK_SIZE, 8 * 1024 * 1024);
    }

    #[test]
    fn test_memory_constants() {
        assert_eq!(MEMORY_AVAILABLE, 1);
        assert_eq!(MEMORY_RESERVED, 2);
        assert_eq!(MEMORY_ACPI_RECLAIMABLE, 3);
        assert_eq!(MEMORY_NVS, 4);
        assert_eq!(MEMORY_UNUSABLE, 5);
    }
}