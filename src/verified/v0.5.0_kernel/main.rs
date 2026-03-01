#![no_std]
#![no_main]

mod vga_console;

use vga_console::{init, write_string};

// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize VGA console
    init();
    
    // Print boot message
    write_string("VantisOS v0.5.0 - VGA Console Test\n");
    write_string("====================================\n\n");
    
    write_string("Kernel started successfully!\n");
    write_string("Architecture: x86_64\n");
    write_string("Version: 0.5.0\n\n");
    
    write_string("VGA Console initialized.\n");
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