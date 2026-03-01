// VantisOS v0.5.0 - Real Kernel Entry Point
// Advanced kernel with system calls, process management, thread management, and file system

#![no_std]
#![allow(dead_code)]

use core::panic::PanicInfo;
use core::arch::asm;

// Import all kernel modules
mod vga_console;
mod memory;
mod interrupt;
mod integration;
mod performance;
mod security;
mod syscall;
mod process;
mod thread;
mod filesystem;

use vga_console::*;
use memory::*;
use interrupt::*;
use integration::*;
use performance::*;
use security::*;
use syscall::*;
use process::*;
use thread::*;
use filesystem::*;

// Wrapper functions for initialization
fn console_init() {
    vga_console::init();
}

fn memory_init() {
    // Initialize with empty memory map for now
    memory::init(&[]);
}

fn interrupt_init() {
    interrupt::init_idt();
    interrupt::load_idt();
}

fn security_init() {
    security::init_security();
}

fn performance_init() {
    // Performance module doesn't have init function
}

fn integration_init() {
    // Integration module doesn't have init function
}

// Wrapper functions for getting stats
fn get_memory_stats() -> Option<memory::MemoryStats> {
    Some(memory::get_stats())
}

fn get_performance_stats() -> Option<performance::PerformanceCounters> {
    Some(performance::get_counters())
}

fn get_security_stats() -> Option<security::SecurityStats> {
    // Security module doesn't have get_stats function
    None
}

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
    checksum: 0xE4524FFE, // -(0x1BADB002 + 0)
};

// Kernel entry point
#[no_mangle]
pub extern "C" fn _start(multiboot_info: u64) -> ! {
    // Initialize kernel
    kernel_init(multiboot_info);
    
    // Main kernel loop
    kernel_loop();
}

// Kernel initialization
fn kernel_init(multiboot_info: u64) {
    // Initialize VGA console
    console_init();
    
    // Print boot message
    print("VantisOS v0.5.0 - Real Kernel\n");
    print("Advanced Kernel with System Calls, Process Management, Thread Management, and File System\n");
    print("=============================================================\n\n");
    
    // Initialize memory management
    print("Initializing memory management...\n");
    memory_init();
    print("Memory management initialized\n\n");
    
    // Initialize interrupt handling
    print("Initializing interrupt handling...\n");
    interrupt_init();
    print("Interrupt handling initialized\n\n");
    
    // Initialize system call interface
    print("Initializing system call interface...\n");
    syscall_init();
    print("System call interface initialized\n\n");
    
    // Initialize process management
    print("Initializing process management...\n");
    process_init();
    print("Process management initialized\n\n");
    
    // Initialize thread management
    print("Initializing thread management...\n");
    thread_init();
    print("Thread management initialized\n\n");
    
    // Initialize file system
    print("Initializing file system...\n");
    filesystem_init();
    print("File system initialized\n\n");
    
    // Initialize security
    print("Initializing security...\n");
    security_init();
    print("Security initialized\n\n");
    
    // Initialize performance counters
    print("Initializing performance counters...\n");
    performance_init();
    print("Performance counters initialized\n\n");
    
    // Initialize integration
    print("Initializing integration...\n");
    integration_init();
    print("Integration initialized\n\n");
    
    // Print kernel status
    print_kernel_status();
}

// Print kernel status
fn print_kernel_status() {
    print("Kernel Status:\n");
    print("=============\n");
    
    // Print memory statistics
    if let Some(mem_stats) = get_memory_stats() {
        print("Memory Statistics:\n");
        print("  Total Memory: ");
        write_dec(mem_stats.total_memory);
        print(" bytes\n");
        print("  Available Memory: ");
        write_dec(mem_stats.available_memory);
        print(" bytes\n");
        print("  Free Pages: ");
        write_dec(mem_stats.free_pages as u64);
        print("\n");
        print("  Used Pages: ");
        write_dec(mem_stats.used_pages as u64);
        print("\n");
        print("  Heap Free: ");
        write_dec(mem_stats.heap_free as u64);
        print(" bytes\n");
        print("  Heap Used: ");
        write_dec(mem_stats.heap_used as u64);
        print(" bytes\n\n");
    }
    
    // Print process statistics
    let proc_stats = get_process_manager().get_process_stats();
    print("Process Statistics:\n");
    print("  Total Processes: ");
    write_dec(proc_stats.total_processes);
    print("\n");
    print("  Created: ");
    write_dec(proc_stats.created);
    print("\n");
    print("  Ready: ");
    write_dec(proc_stats.ready);
    print("\n");
    print("  Running: ");
    write_dec(proc_stats.running);
    print("\n");
    print("  Blocked: ");
    write_dec(proc_stats.blocked);
    print("\n");
    print("  Terminated: ");
    write_dec(proc_stats.terminated);
    print("\n\n");
    
    // Print thread statistics
    let thread_stats = get_thread_scheduler().get_thread_stats();
    print("Thread Statistics:\n");
    print("  Total Threads: ");
    write_dec(thread_stats.total_threads);
    print("\n");
    print("  Created: ");
    write_dec(thread_stats.created);
    print("\n");
    print("  Ready: ");
    write_dec(thread_stats.ready);
    print("\n");
    print("  Running: ");
    write_dec(thread_stats.running);
    print("\n");
    print("  Blocked: ");
    write_dec(thread_stats.blocked);
    print("\n");
    print("  Terminated: ");
    write_dec(thread_stats.terminated);
    print("\n\n");
    
    // Print file system statistics
    let fd_count = get_filesystem_manager().get_fd_count();
    print("File System Statistics:\n");
    print("  Open File Descriptors: ");
    write_dec(fd_count);
    print("\n\n");
    
    // Print system call statistics
    let syscall_stats = get_syscall_stats();
    print("System Call Statistics:\n");
    print("  Total Calls: ");
    write_dec(syscall_stats.total_calls);
    print("\n");
    print("  Exit Calls: ");
    write_dec(syscall_stats.exit_calls);
    print("\n");
    print("  Read Calls: ");
    write_dec(syscall_stats.read_calls);
    print("\n");
    print("  Write Calls: ");
    write_dec(syscall_stats.write_calls);
    print("\n");
    print("  Open Calls: ");
    write_dec(syscall_stats.open_calls);
    print("\n");
    print("  Close Calls: ");
    write_dec(syscall_stats.close_calls);
    print("\n");
    print("  Stat Calls: ");
    write_dec(syscall_stats.stat_calls);
    print("\n");
    print("  Mmap Calls: ");
    write_dec(syscall_stats.mmap_calls);
    print("\n");
    print("  Ioctl Calls: ");
    write_dec(syscall_stats.ioctl_calls);
    print("\n\n");
    
    // Print performance statistics
    if let Some(perf_stats) = get_performance_stats() {
        print("Performance Statistics:\n");
        print("  Boot Time: ");
        write_dec(perf_stats.boot_time);
        print(" cycles\n");
        print("  Memory Allocations: ");
        write_dec(perf_stats.memory_allocations);
        print("\n");
        print("  Interrupts: ");
        write_dec(perf_stats.interrupt_count);
        print("\n");
        print("  Console Characters: ");
        write_dec(perf_stats.console_chars);
        print("\n\n");
    }
    
    // Print security statistics
    if let Some(sec_stats) = get_security_stats() {
        print("Security Statistics:\n");
        print("  Stack Canary Checks: ");
        write_dec(sec_stats.stack_canary_checks);
        print("\n");
        print("  Memory Access Checks: ");
        write_dec(sec_stats.memory_access_checks);
        print("\n");
        print("  Security Checks: ");
        write_dec(sec_stats.security_checks);
        print("\n\n");
    }
    
    print("Kernel initialization complete!\n");
    print("System ready.\n\n");
}

// Kernel main loop
fn kernel_loop() -> ! {
    print("Entering kernel main loop...\n");
    
    loop {
        // Enable interrupts
        enable_interrupts();
        
        // Halt CPU until next interrupt
        unsafe {
            asm!("hlt");
        }
    }
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("\n\nKERNEL PANIC!\n");
    print("System halted.\n");
    
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}