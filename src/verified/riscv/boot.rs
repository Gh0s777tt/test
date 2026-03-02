//! RISC-V Boot Process
//! 
//! This module handles the RISC-V boot process, including:
//! - Bootloader initialization
//! - Memory setup
//! - Stack configuration
//! - BSS clearing
//! - Kernel entry point

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Clear BSS section
    clear_bss();
    
    // 2. Setup stack
    setup_stack();
    
    // 3. Initialize hardware
    init_hardware();
    
    // 4. Setup MMU
    setup_mmu();
    
    // 5. Jump to kernel main
    kernel_main();
    
    // Should never reach here
    loop {}
}

/// Clear BSS section
fn clear_bss() {
    extern "C" {
        static mut _bss_start: u8;
        static mut _bss_end: u8;
    }
    
    unsafe {
        let start = &_bss_start as *mut u8;
        let end = &_bss_end as *mut u8;
        let size = end as usize - start as usize;
        
        core::ptr::write_bytes(start, 0, size);
    }
}

/// Setup stack
fn setup_stack() {
    extern "C" {
        static mut _stack_top: u8;
    }
    
    unsafe {
        // Stack pointer is set by bootloader
        // Just validate it's properly aligned
        let sp = &_stack_top as *const u8 as usize;
        assert!(sp & 0xF == 0, "Stack not 16-byte aligned");
    }
}

/// Initialize hardware
fn init_hardware() {
    // Initialize UART for debug output
    init_uart();
    
    // Initialize timer
    init_timer();
    
    // Initialize interrupt controller
    init_plic();
}

/// Initialize UART
fn init_uart() {
    // UART base address (QEMU virt machine)
    const UART_BASE: usize = 0x10000000;
    
    unsafe {
        // Set baud rate divisor (115200)
        let uart = UART_BASE as *mut u8;
        
        // Disable interrupts
        uart.add(1).write_volatile(0x00);
        
        // Set DLAB
        uart.add(3).write_volatile(0x80);
        
        // Set divisor
        uart.add(0).write_volatile(0x03);
        uart.add(1).write_volatile(0x00);
        
        // Clear DLAB, 8N1
        uart.add(3).write_volatile(0x03);
        
        // Enable FIFO
        uart.add(2).write_volatile(0x01);
    }
}

/// Initialize timer
fn init_timer() {
    // Timer is handled by SBI
    // Just set initial timecmp
    extern "C" {
        fn sbi_set_timer(stime_value: u64);
    }
    
    unsafe {
        // Set timer to fire in 1 second
        const MTIME_FREQ: u64 = 10_000_000; // 10 MHz
        sbi_set_timer(MTIME_FREQ);
    }
}

/// Initialize PLIC (Platform-Level Interrupt Controller)
fn init_plic() {
    // PLIC base address (QEMU virt machine)
    const PLIC_BASE: usize = 0x0c000000;
    
    unsafe {
        let plic = PLIC_BASE as *mut u32;
        
        // Disable all interrupts
        for i in 0..32 {
            plic.add(0x0020 + i * 4).write_volatile(0);
        }
        
        // Set priority thresholds
        plic.add(0x200000).write_volatile(0); // M-mode threshold
        plic.add(0x201000).write_volatile(0); // S-mode threshold
    }
}

/// Setup MMU
fn setup_mmu() {
    // MMU setup is handled in mmu.rs
    // Just call the initialization function
    extern "C" {
        fn riscv_mmu_init();
    }
    
    unsafe {
        riscv_mmu_init();
    }
}

/// Kernel main function
fn kernel_main() -> ! {
    // Print boot message
    print_boot_message();
    
    // Initialize kernel subsystems
    init_kernel();
    
    // Start scheduler
    start_scheduler();
    
    // Should never reach here
    loop {}
}

/// Print boot message
fn print_boot_message() {
    const BOOT_MSG: &[u8] = b"VantisOS v0.7.0 - IoT Ready\nRISC-V Architecture Support\nBooting...\n";
    
    unsafe {
        const UART_BASE: usize = 0x10000000;
        let uart = UART_BASE as *mut u8;
        
        for &byte in BOOT_MSG {
            // Wait for transmit buffer to be empty
            while uart.add(5).read_volatile() & 0x20 == 0 {}
            
            // Write byte
            uart.write_volatile(byte);
        }
    }
}

/// Initialize kernel subsystems
fn init_kernel() {
    // Initialize memory allocator
    extern "C" {
        fn riscv_allocator_init();
    }
    
    unsafe {
        riscv_allocator_init();
    }
    
    // Initialize interrupt handling
    extern "C" {
        fn riscv_interrupt_init();
    }
    
    unsafe {
        riscv_interrupt_init();
    }
    
    // Initialize scheduler
    extern "C" {
        fn riscv_scheduler_init();
    }
    
    unsafe {
        riscv_scheduler_init();
    }
}

/// Start scheduler
fn start_scheduler() -> ! {
    extern "C" {
        fn riscv_scheduler_start() -> !;
    }
    
    unsafe {
        riscv_scheduler_start();
    }
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    const PANIC_MSG: &[u8] = b"\n\nKERNEL PANIC!\nSystem halted.\n";
    
    unsafe {
        const UART_BASE: usize = 0x10000000;
        let uart = UART_BASE as *mut u8;
        
        for &byte in PANIC_MSG {
            while uart.add(5).read_volatile() & 0x20 == 0 {}
            uart.write_volatile(byte);
        }
    }
    
    loop {}
}