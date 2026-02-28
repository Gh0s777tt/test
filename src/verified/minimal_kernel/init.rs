// Kernel Initialization
//
// This module provides kernel initialization for VantisOS, including:
// - Boot information parsing
// - Kernel subsystem initialization
// - Early boot setup
// - Kernel main loop

#![no_std]

use crate::verified::minimal_kernel::entry::{BootInfo, parse_boot_info, MultibootInfo};
use crate::verified::minimal_kernel::memory::{PageAllocator, VirtualMemory, memory_region::MemoryRegionManager, memory_protection::MemoryProtectionManager, memory_stats::init_global_stats};
use crate::verified::minimal_kernel::interrupt::{init as init_interrupts, enable_interrupts};
use crate::verified::minimal_kernel::timer::{init as init_timer, set_frequency};
use crate::verified::minimal_kernel::keyboard::init as init_keyboard;
use crate::verified::minimal_kernel::serial::{init_default, set_log_level, LogLevel, info, error, warn};
use crate::verified::minimal_kernel::process::ProcessManager;
use crate::verified::minimal_kernel::thread::{ThreadManager, Scheduler};
use crate::verified::minimal_kernel::io::{CharDeviceManager, BlockDeviceManager};

/// Kernel version
pub const KERNEL_VERSION: &str = "0.4.1";
pub const KERNEL_CODENAME: &str = "Cytadela Complete";

/// Boot information
static mut BOOT_INFO: Option<BootInfo> = None;

/// Page allocator
static mut PAGE_ALLOCATOR: Option<PageAllocator> = None;

/// Virtual memory manager
static mut VIRTUAL_MEMORY: Option<VirtualMemory> = None;

/// Process manager
static mut PROCESS_MANAGER: Option<ProcessManager> = None;

/// Thread manager
static mut THREAD_MANAGER: Option<ThreadManager> = None;

/// Scheduler
static mut SCHEDULER: Option<Scheduler> = None;

/// Character device manager
static mut CHAR_DEVICE_MANAGER: Option<CharDeviceManager> = None;

/// Block device manager
static mut BLOCK_DEVICE_MANAGER: Option<BlockDeviceManager> = None;

/// Kernel initialization
///
/// This function is called from the entry point after the bootloader
/// transfers control to the kernel. It performs all necessary initialization.
#[no_mangle]
pub extern "C" fn kernel_init(boot_info: &MultibootInfo) {
    // Parse boot information
    let info = parse_boot_info(boot_info);
    unsafe {
        BOOT_INFO = Some(info);
    }

    // Initialize serial port for early logging
    init_default();
    set_log_level(LogLevel::Info);

    info!("VantisOS {} ({}) initializing...", KERNEL_VERSION, KERNEL_CODENAME);
    info!("Memory size: {} MB", get_memory_size_mb());

    // Initialize memory management
    info!("Initializing memory management...");
    init_memory();

    // Initialize interrupt handling
    info!("Initializing interrupt handling...");
    init_interrupts();

    // Initialize timer
    info!("Initializing timer...");
    init_timer();
    set_frequency(1000); // 1000 Hz = 1 ms per tick

    // Initialize keyboard
    info!("Initializing keyboard...");
    init_keyboard();

    // Initialize process and thread management
    info!("Initializing process and thread management...");
    init_process_thread();

    // Initialize I/O subsystem
    info!("Initializing I/O subsystem...");
    init_io();

    // Enable interrupts
    info!("Enabling interrupts...");
    enable_interrupts();

    info!("Kernel initialization complete!");
    info!("System ready.");
}

/// Initialize memory management
fn init_memory() {
    unsafe {
        let boot_info = BOOT_INFO.as_ref().unwrap();
        let memory_size = boot_info.memory_size;

        // Create page allocator
        PAGE_ALLOCATOR = Some(PageAllocator::new(memory_size));
        info!("Page allocator initialized: {} MB", memory_size / (1024 * 1024));

        // Create virtual memory manager
        VIRTUAL_MEMORY = Some(VirtualMemory::new());
        info!("Virtual memory manager initialized");

        // Initialize memory region manager
        let mut region_manager = MemoryRegionManager::new();
        info!("Memory region manager initialized");

        // Initialize memory protection manager
        let protection_manager = MemoryProtectionManager::new();
        info!("Memory protection manager initialized");

        // Initialize global memory statistics
        init_global_stats();
        info!("Global memory statistics initialized");
    }
}

/// Initialize process and thread management
fn init_process_thread() {
    unsafe {
        // Create process manager
        PROCESS_MANAGER = Some(ProcessManager::new());
        info!("Process manager initialized");

        // Create thread manager
        THREAD_MANAGER = Some(ThreadManager::new());
        info!("Thread manager initialized");

        // Create scheduler
        SCHEDULER = Some(Scheduler::new());
        info!("Scheduler initialized");
    }
}

/// Initialize I/O subsystem
fn init_io() {
    unsafe {
        // Create character device manager
        CHAR_DEVICE_MANAGER = Some(CharDeviceManager::new());
        info!("Character device manager initialized");

        // Create block device manager
        BLOCK_DEVICE_MANAGER = Some(BlockDeviceManager::new());
        info!("Block device manager initialized");
    }
}

/// Kernel main function
///
/// This is the main kernel loop. It should never return.
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    info!("Entering kernel main loop...");

    // TODO: Start idle process
    // TODO: Start system services
    // TODO: Run scheduler

    // For now, just halt in a loop
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Get boot information
pub fn get_boot_info() -> Option<&'static BootInfo> {
    unsafe { BOOT_INFO.as_ref() }
}

/// Get memory size in MB
fn get_memory_size_mb() -> u64 {
    unsafe {
        BOOT_INFO.as_ref()
            .map(|info| info.memory_size / (1024 * 1024))
            .unwrap_or(0)
    }
}

/// Get page allocator
pub fn get_page_allocator() -> Option<&'static PageAllocator> {
    unsafe { PAGE_ALLOCATOR.as_ref() }
}

/// Get page allocator mutable
pub fn get_page_allocator_mut() -> Option<&'static mut PageAllocator> {
    unsafe { PAGE_ALLOCATOR.as_mut() }
}

/// Get virtual memory manager
pub fn get_virtual_memory() -> Option<&'static VirtualMemory> {
    unsafe { VIRTUAL_MEMORY.as_ref() }
}

/// Get virtual memory manager mutable
pub fn get_virtual_memory_mut() -> Option<&'static mut VirtualMemory> {
    unsafe { VIRTUAL_MEMORY.as_mut() }
}

/// Get process manager
pub fn get_process_manager() -> Option<&'static ProcessManager> {
    unsafe { PROCESS_MANAGER.as_ref() }
}

/// Get process manager mutable
pub fn get_process_manager_mut() -> Option<&'static mut ProcessManager> {
    unsafe { PROCESS_MANAGER.as_mut() }
}

/// Get thread manager
pub fn get_thread_manager() -> Option<&'static ThreadManager> {
    unsafe { THREAD_MANAGER.as_ref() }
}

/// Get thread manager mutable
pub fn get_thread_manager_mut() -> Option<&'static mut ThreadManager> {
    unsafe { THREAD_MANAGER.as_mut() }
}

/// Get scheduler
pub fn get_scheduler() -> Option<&'static Scheduler> {
    unsafe { SCHEDULER.as_ref() }
}

/// Get scheduler mutable
pub fn get_scheduler_mut() -> Option<&'static mut Scheduler> {
    unsafe { SCHEDULER.as_mut() }
}

/// Get character device manager
pub fn get_char_device_manager() -> Option<&'static CharDeviceManager> {
    unsafe { CHAR_DEVICE_MANAGER.as_ref() }
}

/// Get character device manager mutable
pub fn get_char_device_manager_mut() -> Option<&'static mut CharDeviceManager> {
    unsafe { CHAR_DEVICE_MANAGER.as_mut() }
}

/// Get block device manager
pub fn get_block_device_manager() -> Option<&'static BlockDeviceManager> {
    unsafe { BLOCK_DEVICE_MANAGER.as_ref() }
}

/// Get block device manager mutable
pub fn get_block_device_manager_mut() -> Option<&'static mut BlockDeviceManager> {
    unsafe { BLOCK_DEVICE_MANAGER.as_mut() }
}

/// Kernel panic handler
pub fn kernel_panic(message: &str) -> ! {
    error!("KERNEL PANIC: {}", message);
    
    // Disable interrupts
    unsafe {
        core::arch::asm!("cli");
    }
    
    // Halt CPU
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Kernel assertion
#[macro_export]
macro_rules! kernel_assert {
    ($condition:expr, $message:expr) => {
        if !$condition {
            $crate::verified::minimal_kernel::init::kernel_panic($message);
        }
    };
}

/// Kernel warning
pub fn kernel_warning(message: &str) {
    warn!("KERNEL WARNING: {}", message);
}

/// Kernel info
pub fn kernel_info(message: &str) {
    info!("KERNEL INFO: {}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_version() {
        assert_eq!(KERNEL_VERSION, "0.4.1");
        assert_eq!(KERNEL_CODENAME, "Cytadela Complete");
    }
}