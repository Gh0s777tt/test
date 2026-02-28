// Interrupt Handling
//
// This module provides interrupt handling for VantisOS, including:
// - Interrupt Descriptor Table (IDT) setup
// - Interrupt Service Routines (ISRs)
// - Exception handlers
// - IRQ handlers
// - System call interface

#![no_std]

use core::arch::asm;

/// Number of interrupt vectors (256)
pub const IDT_ENTRIES: usize = 256;

/// Exception vectors (0-31)
pub const EXCEPTION_DIVIDE_ERROR: u8 = 0;
pub const EXCEPTION_DEBUG: u8 = 1;
pub const EXCEPTION_NON_MASKABLE_INTERRUPT: u8 = 2;
pub const EXCEPTION_BREAKPOINT: u8 = 3;
pub const EXCEPTION_OVERFLOW: u8 = 4;
pub const EXCEPTION_BOUND_RANGE_EXCEEDED: u8 = 5;
pub const EXCEPTION_INVALID_OPCODE: u8 = 6;
pub const EXCEPTION_DEVICE_NOT_AVAILABLE: u8 = 7;
pub const EXCEPTION_DOUBLE_FAULT: u8 = 8;
pub const EXCEPTION_INVALID_TSS: u8 = 10;
pub const EXCEPTION_SEGMENT_NOT_PRESENT: u8 = 11;
pub const EXCEPTION_STACK_SEGMENT_FAULT: u8 = 12;
pub const EXCEPTION_GENERAL_PROTECTION_FAULT: u8 = 13;
pub const EXCEPTION_PAGE_FAULT: u8 = 14;
pub const EXCEPTION_X87_FPU_ERROR: u8 = 16;
pub const EXCEPTION_ALIGNMENT_CHECK: u8 = 17;
pub const EXCEPTION_MACHINE_CHECK: u8 = 18;
pub const EXCEPTION_SIMD_FLOATING_POINT: u8 = 19;
pub const EXCEPTION_VIRTUALIZATION: u8 = 20;
pub const EXCEPTION_CONTROL_PROTECTION: u8 = 21;
pub const EXCEPTION_SECURITY: u8 = 30;

/// IRQ vectors (32-47)
pub const IRQ_BASE: u8 = 32;
pub const IRQ_TIMER: u8 = 32;
pub const IRQ_KEYBOARD: u8 = 33;
pub const IRQ_CASCADE: u8 = 34;
pub const IRQ_COM2: u8 = 35;
pub const IRQ_COM1: u8 = 36;
pub const IRQ_LPT2: u8 = 37;
pub const IRQ_FLOPPY: u8 = 38;
pub const IRQ_LPT1: u8 = 39;
pub const IRQ_RTC: u8 = 40;
pub const IRQ_MOUSE: u8 = 44;
pub const IRQ_FPU: u8 = 45;
pub const IRQ_PRIMARY_ATA: u8 = 46;
pub const IRQ_SECONDARY_ATA: u8 = 47;

/// System call vector (128)
pub const SYSCALL_VECTOR: u8 = 128;

/// Interrupt Descriptor Table entry
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    /// Lower 16 bits of handler address
    offset_low: u16,
    /// Segment selector
    selector: u16,
    /// IST and type attributes
    ist: u8,
    /// Type and attributes
    type_attr: u8,
    /// Middle 16 bits of handler address
    offset_mid: u16,
    /// Upper 32 bits of handler address
    offset_high: u32,
    /// Reserved
    reserved: u32,
}

impl IdtEntry {
    /// Create a new IDT entry
    pub fn new(handler: u64, selector: u16, type_attr: u8) -> Self {
        IdtEntry {
            offset_low: (handler & 0xFFFF) as u16,
            selector,
            ist: 0,
            type_attr,
            offset_mid: ((handler >> 16) & 0xFFFF) as u16,
            offset_high: ((handler >> 32) & 0xFFFFFFFF) as u32,
            reserved: 0,
        }
    }

    /// Create an interrupt gate entry
    pub fn interrupt_gate(handler: u64, selector: u16, dpl: u8) -> Self {
        let type_attr = 0x8E | (dpl << 5); // Present, Ring 0/1/2/3, 32-bit interrupt gate
        Self::new(handler, selector, type_attr)
    }

    /// Create a trap gate entry
    pub fn trap_gate(handler: u64, selector: u16, dpl: u8) -> Self {
        let type_attr = 0x8F | (dpl << 5); // Present, Ring 0/1/2/3, 32-bit trap gate
        Self::new(handler, selector, type_attr)
    }

    /// Create a task gate entry
    pub fn task_gate(tss_selector: u16, dpl: u8) -> Self {
        let type_attr = 0x85 | (dpl << 5); // Present, Ring 0/1/2/3, task gate
        IdtEntry {
            offset_low: 0,
            selector: tss_selector,
            ist: 0,
            type_attr,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
}

/// Interrupt Descriptor Table pointer
#[repr(C, packed)]
pub struct IdtPointer {
    /// Size of IDT - 1
    limit: u16,
    /// Base address of IDT
    base: u64,
}

impl IdtPointer {
    pub fn new(idt: &Idt) -> Self {
        IdtPointer {
            limit: (core::mem::size_of::<Idt>() - 1) as u16,
            base: idt as *const Idt as u64,
        }
    }
}

/// Interrupt Descriptor Table
#[repr(C, align(16))]
pub struct Idt {
    entries: [IdtEntry; IDT_ENTRIES],
}

impl Idt {
    /// Create a new IDT
    pub const fn new() -> Self {
        Idt {
            entries: [IdtEntry {
                offset_low: 0,
                selector: 0,
                ist: 0,
                type_attr: 0,
                offset_mid: 0,
                offset_high: 0,
                reserved: 0,
            }; IDT_ENTRIES],
        }
    }

    /// Set an IDT entry
    pub fn set_entry(&mut self, index: usize, entry: IdtEntry) {
        if index < IDT_ENTRIES {
            self.entries[index] = entry;
        }
    }

    /// Load the IDT
    pub unsafe fn load(&self) {
        let idt_ptr = IdtPointer::new(self);
        asm!("lidt [{}]", in(reg) &idt_ptr, options(readonly, nostack));
    }
}

/// Interrupt stack frame pushed by CPU
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InterruptStackFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
    pub vector: u64,
    pub error_code: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

/// Exception handler type
pub type ExceptionHandler = extern "x86-interrupt" fn(&mut InterruptStackFrame);
pub type ExceptionHandlerWithError = extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u64);

/// IRQ handler type
pub type IrqHandler = extern "x86-interrupt" fn(&mut InterruptStackFrame);

/// System call handler type
pub type SyscallHandler = extern "x86-interrupt" fn(&mut InterruptStackFrame);

/// Exception handlers
static mut EXCEPTION_HANDLERS: [Option<ExceptionHandler>; 32] = [None; 32];
static mut EXCEPTION_HANDLERS_WITH_ERROR: [Option<ExceptionHandlerWithError>; 32] = [None; 32];

/// IRQ handlers
static mut IRQ_HANDLERS: [Option<IrqHandler>; 16] = [None; 16];

/// System call handler
static mut SYSCALL_HANDLER: Option<SyscallHandler> = None;

/// Set exception handler
pub fn set_exception_handler(vector: u8, handler: ExceptionHandler) {
    unsafe {
        if vector < 32 {
            EXCEPTION_HANDLERS[vector as usize] = Some(handler);
        }
    }
}

/// Set exception handler with error code
pub fn set_exception_handler_with_error(vector: u8, handler: ExceptionHandlerWithError) {
    unsafe {
        if vector < 32 {
            EXCEPTION_HANDLERS_WITH_ERROR[vector as usize] = Some(handler);
        }
    }
}

/// Set IRQ handler
pub fn set_irq_handler(irq: u8, handler: IrqHandler) {
    unsafe {
        if irq < 16 {
            IRQ_HANDLERS[irq as usize] = Some(handler);
        }
    }
}

/// Set system call handler
pub fn set_syscall_handler(handler: SyscallHandler) {
    unsafe {
        SYSCALL_HANDLER = Some(handler);
    }
}

/// Default exception handler
extern "x86-interrupt" fn default_exception_handler(frame: &mut InterruptStackFrame) {
    // TODO: Print exception information and halt
    // For now, just halt
    loop {
        unsafe { asm!("hlt"); }
    }
}

/// Default exception handler with error code
extern "x86-interrupt" fn default_exception_handler_with_error(frame: &mut InterruptStackFrame, error_code: u64) {
    // TODO: Print exception information and error code, then halt
    // For now, just halt
    loop {
        unsafe { asm!("hlt"); }
    }
}

/// Default IRQ handler
extern "x86-interrupt" fn default_irq_handler(frame: &mut InterruptStackFrame) {
    // TODO: Send EOI to PIC
    // For now, just return
}

/// Initialize interrupt handling
pub fn init() {
    let mut idt = Idt::new();

    // Set up exception handlers (0-31)
    for i in 0..32 {
        let handler = match i {
            EXCEPTION_DOUBLE_FAULT => double_fault_handler as u64,
            EXCEPTION_PAGE_FAULT => page_fault_handler as u64,
            _ => exception_handler as u64,
        };

        idt.set_entry(
            i,
            IdtEntry::interrupt_gate(handler, 0x08, 0),
        );
    }

    // Set up IRQ handlers (32-47)
    for i in 0..16 {
        let handler = irq_handler as u64;
        idt.set_entry(
            (IRQ_BASE + i) as usize,
            IdtEntry::interrupt_gate(handler, 0x08, 0),
        );
    }

    // Set up system call handler (128)
    idt.set_entry(
        SYSCALL_VECTOR as usize,
        IdtEntry::trap_gate(syscall_handler as u64, 0x08, 3), // Ring 3
    );

    // Load IDT
    unsafe {
        idt.load();
    }
}

/// Exception handler wrapper
extern "x86-interrupt" fn exception_handler(frame: &mut InterruptStackFrame) {
    let vector = frame.vector as u8;
    
    unsafe {
        if let Some(handler) = EXCEPTION_HANDLERS[vector as usize] {
            handler(frame);
        } else {
            default_exception_handler(frame);
        }
    }
}

/// Exception handler with error code wrapper
extern "x86-interrupt" fn exception_handler_with_error(frame: &mut InterruptStackFrame, error_code: u64) {
    let vector = frame.vector as u8;
    
    unsafe {
        if let Some(handler) = EXCEPTION_HANDLERS_WITH_ERROR[vector as usize] {
            handler(frame, error_code);
        } else {
            default_exception_handler_with_error(frame, error_code);
        }
    }
}

/// Double fault handler
extern "x86-interrupt" fn double_fault_handler(frame: &mut InterruptStackFrame, error_code: u64) {
    // TODO: Handle double fault
    // This is a critical error, usually indicates stack corruption
    loop {
        unsafe { asm!("hlt"); }
    }
}

/// Page fault handler
extern "x86-interrupt" fn page_fault_handler(frame: &mut InterruptStackFrame, error_code: u64) {
    // TODO: Handle page fault
    // Read CR2 to get faulting address
    let faulting_address: u64;
    unsafe {
        asm!("mov {}, cr2", out(reg) faulting_address);
    }
    
    // TODO: Implement page fault handling
    // - Check error code flags
    // - Handle user vs kernel page faults
    // - Implement demand paging if needed
    
    loop {
        unsafe { asm!("hlt"); }
    }
}

/// IRQ handler wrapper
extern "x86-interrupt" fn irq_handler(frame: &mut InterruptStackFrame) {
    let vector = frame.vector as u8;
    let irq = vector - IRQ_BASE;
    
    // Call IRQ handler if registered
    unsafe {
        if let Some(handler) = IRQ_HANDLERS[irq as usize] {
            handler(frame);
        } else {
            default_irq_handler(frame);
        }
    }
    
    // Send EOI to PIC
    // TODO: Implement PIC EOI
}

/// System call handler
extern "x86-interrupt" fn syscall_handler(frame: &mut InterruptStackFrame) {
    // TODO: Implement system call dispatch
    // - Read system call number from RAX
    // - Read arguments from RDI, RSI, RDX, R10, R8, R9
    // - Dispatch to appropriate handler
    // - Return result in RAX
    
    unsafe {
        if let Some(handler) = SYSCALL_HANDLER {
            handler(frame);
        }
    }
}

/// Enable interrupts
pub fn enable_interrupts() {
    unsafe {
        asm!("sti");
    }
}

/// Disable interrupts
pub fn disable_interrupts() {
    unsafe {
        asm!("cli");
    }
}

/// Check if interrupts are enabled
pub fn are_interrupts_enabled() -> bool {
    let rflags: u64;
    unsafe {
        asm!("pushf; pop {}", out(reg) rflags);
    }
    (rflags & (1 << 9)) != 0
}

/// Halt CPU until next interrupt
pub fn halt() {
    unsafe {
        asm!("hlt");
    }
}

/// 8259 PIC (Programmable Interrupt Controller)
pub mod pic {
    use core::arch::asm;

    /// PIC command ports
    const PIC1_COMMAND: u16 = 0x20;
    const PIC1_DATA: u16 = 0x21;
    const PIC2_COMMAND: u16 = 0xA0;
    const PIC2_DATA: u16 = 0xA1;

    /// PIC commands
    const ICW1_INIT: u8 = 0x11;
    const ICW1_ICW4: u8 = 0x01;
    const ICW4_8086: u8 = 0x01;

    /// Initialize PIC
    pub unsafe fn init(offset1: u8, offset2: u8) {
        // Save original interrupt masks
        let mask1 = asm_inb(PIC1_DATA);
        let mask2 = asm_inb(PIC2_DATA);

        // Start initialization sequence
        asm_outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_wait();
        asm_outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
        io_wait();

        // Set vector offsets
        asm_outb(PIC1_DATA, offset1);
        io_wait();
        asm_outb(PIC2_DATA, offset2);
        io_wait();

        // Configure PIC2 as slave at IRQ2
        asm_outb(PIC1_DATA, 4);
        io_wait();
        asm_outb(PIC2_DATA, 2);
        io_wait();

        // Set 8086 mode
        asm_outb(PIC1_DATA, ICW4_8086);
        io_wait();
        asm_outb(PIC2_DATA, ICW4_8086);
        io_wait();

        // Restore interrupt masks
        asm_outb(PIC1_DATA, mask1);
        asm_outb(PIC2_DATA, mask2);
    }

    /// Send End of Interrupt (EOI)
    pub unsafe fn send_eoi(irq: u8) {
        if irq >= 8 {
            asm_outb(PIC2_COMMAND, 0x20);
        }
        asm_outb(PIC1_COMMAND, 0x20);
    }

    /// Disable PIC (for APIC)
    pub unsafe fn disable() {
        asm_outb(PIC1_DATA, 0xFF);
        asm_outb(PIC2_DATA, 0xFF);
    }

    /// I/O wait
    fn io_wait() {
        unsafe {
            asm_outb(0x80, 0);
        }
    }

    /// Read byte from port
    unsafe fn asm_inb(port: u16) -> u8 {
        let result: u8;
        asm!("in al, dx", inlateout("dx") port => result, options(nomem, nostack));
        result
    }

    /// Write byte to port
    unsafe fn asm_outb(port: u16, value: u8) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idt_entries() {
        assert_eq!(IDT_ENTRIES, 256);
    }

    #[test]
    fn test_exception_vectors() {
        assert_eq!(EXCEPTION_DIVIDE_ERROR, 0);
        assert_eq!(EXCEPTION_DEBUG, 1);
        assert_eq!(EXCEPTION_DOUBLE_FAULT, 8);
        assert_eq!(EXCEPTION_PAGE_FAULT, 14);
    }

    #[test]
    fn test_irq_vectors() {
        assert_eq!(IRQ_BASE, 32);
        assert_eq!(IRQ_TIMER, 32);
        assert_eq!(IRQ_KEYBOARD, 33);
        assert_eq!(IRQ_RTC, 40);
    }

    #[test]
    fn test_syscall_vector() {
        assert_eq!(SYSCALL_VECTOR, 128);
    }

    #[test]
    fn test_interrupt_stack_frame_size() {
        assert_eq!(core::mem::size_of::<InterruptStackFrame>(), 22 * 8);
    }
}