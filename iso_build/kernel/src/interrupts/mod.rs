//! Interrupt handling for VantisOS

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use spin::Mutex;

/// PIC ports
const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

/// Global IDT
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[32].set_handler_fn(timer_handler);
        idt[33].set_handler_fn(keyboard_handler);
        idt
    };
}

/// Initialize IDT
pub fn init_idt() {
    IDT.load();
}

/// Remap PIC
pub fn remap_pic() {
    unsafe {
        outb(PIC1_CMD, 0x11);
        outb(PIC2_CMD, 0x11);
        outb(PIC1_DATA, 0x20);
        outb(PIC2_DATA, 0x28);
        outb(PIC1_DATA, 0x04);
        outb(PIC2_DATA, 0x02);
        outb(PIC1_DATA, 0x01);
        outb(PIC2_DATA, 0x01);
        outb(PIC1_DATA, 0x00);
        outb(PIC2_DATA, 0x00);
    }
}

/// Enable interrupts
pub fn enable() {
    unsafe { core::arch::asm!("sti", options(nostack, nomem)); }
}

/// Disable interrupts
pub fn disable() {
    unsafe { core::arch::asm!("cli", options(nostack, nomem)); }
}

/// Halt CPU
pub fn hlt() {
    unsafe { core::arch::asm!("hlt", options(nostack, nomem)); }
}

/// Port output
pub unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nostack, nomem));
}

/// Port input
pub unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!("in al, dx", out("al") val, in("dx") port, options(nostack, nomem));
    val
}

// Handlers
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::arch::serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, _error_code: x86_64::structures::idt::PageFaultErrorCode) {
    panic!("EXCEPTION: PAGE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    unsafe { outb(PIC1_CMD, 0x20); }
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    unsafe { outb(PIC1_CMD, 0x20); }
}