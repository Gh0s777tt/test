//! Interrupt handling for VantisOS
//! 
//! This module provides:
//! - Interrupt Descriptor Table (IDT)
//! - Exception handlers
//! - Hardware interrupt handlers (IRQ)
//! - PIC/APIC configuration

use x86_64::structures::idt::{
    InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode,
    InterruptStackFrameValue,
};
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;

use crate::serial_println;
use crate::serial_print;

/// PIC ports
const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

/// IRQ base vectors
const PIC1_BASE: u8 = 32;
const PIC2_BASE: u8 = 40;

/// APIC registers
const APIC_BASE_MSR: u32 = 0x1B;
const APIC_SPURIOUS: u32 = 0xF0;
const APIC_TPR: u32 = 0x80;
const APIC_EOI: u32 = 0xB0;
const APIC_LVT_TIMER: u32 = 0x320;
const APIC_TIMER_INIT: u32 = 0x380;
const APIC_TIMER_CURRENT: u32 = 0x390;
const APIC_TIMER_DIVIDE: u32 = 0x3E0;

/// Interrupt vectors
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum InterruptVector {
    /// Divide by zero
    DivideError = 0,
    /// Debug exception
    Debug = 1,
    /// Non-maskable interrupt
    Nmi = 2,
    /// Breakpoint
    Breakpoint = 3,
    /// Overflow
    Overflow = 4,
    /// Bound range exceeded
    BoundRange = 5,
    /// Invalid opcode
    InvalidOpcode = 6,
    /// Device not available
    DeviceNotAvailable = 7,
    /// Double fault
    DoubleFault = 8,
    /// Coprocessor segment overrun
    CoprocessorOverrun = 9,
    /// Invalid TSS
    InvalidTss = 10,
    /// Segment not present
    SegmentNotPresent = 11,
    /// Stack segment fault
    StackSegmentFault = 12,
    /// General protection fault
    GeneralProtectionFault = 13,
    /// Page fault
    PageFault = 14,
    /// Reserved
    Reserved15 = 15,
    /// x87 FPU error
    X87FpuError = 16,
    /// Alignment check
    AlignmentCheck = 17,
    /// Machine check
    MachineCheck = 18,
    /// SIMD exception
    SimdException = 19,
    /// Virtualization exception
    VirtualizationException = 20,
    /// Control protection exception
    ControlProtectionException = 21,
    
    // IRQ vectors (32+)
    /// Timer IRQ
    Timer = 32,
    /// Keyboard IRQ
    Keyboard = 33,
    /// Cascade IRQ
    Cascade = 34,
    /// COM2 IRQ
    Com2 = 35,
    /// COM1 IRQ
    Com1 = 36,
    /// LPT2 IRQ
    Lpt2 = 37,
    /// Floppy IRQ
    Floppy = 38,
    /// LPT1 IRQ
    Lpt1 = 39,
    /// RTC IRQ
    Rtc = 40,
    /// PCI IRQ
    Pci = 41,
    /// Mouse IRQ
    Mouse = 44,
    /// FPU IRQ
    Fpu = 45,
    /// Primary ATA IRQ
    PrimaryAta = 46,
    /// Secondary ATA IRQ
    SecondaryAta = 47,
    
    // Custom vectors (48+)
    /// Scheduler tick
    SchedulerTick = 48,
    /// System call
    SystemCall = 128,
}

/// Exception info for error messages
struct ExceptionInfo {
    name: &'static str,
    has_error_code: bool,
    is_trap: bool,
}

const EXCEPTION_INFOS: [ExceptionInfo; 32] = [
    ExceptionInfo { name: "Divide Error", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Debug", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "NMI", has_error_code: false, is_trap: false },
    ExceptionInfo { name: "Breakpoint", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Overflow", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Bound Range Exceeded", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Invalid Opcode", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Device Not Available", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Double Fault", has_error_code: true, is_trap: false },
    ExceptionInfo { name: "Coprocessor Segment Overrun", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Invalid TSS", has_error_code: true, is_trap: true },
    ExceptionInfo { name: "Segment Not Present", has_error_code: true, is_trap: true },
    ExceptionInfo { name: "Stack Segment Fault", has_error_code: true, is_trap: true },
    ExceptionInfo { name: "General Protection Fault", has_error_code: true, is_trap: true },
    ExceptionInfo { name: "Page Fault", has_error_code: true, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "x87 FPU Error", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Alignment Check", has_error_code: true, is_trap: true },
    ExceptionInfo { name: "Machine Check", has_error_code: false, is_trap: false },
    ExceptionInfo { name: "SIMD Exception", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Virtualization Exception", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Control Protection", has_error_code: true, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
    ExceptionInfo { name: "Reserved", has_error_code: false, is_trap: true },
];

/// Global IDT
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        
        // CPU exceptions
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(nmi_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_fpu_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);
        
        // Hardware interrupts (IRQ)
        idt[InterruptVector::Timer as usize].set_handler_fn(timer_irq_handler);
        idt[InterruptVector::Keyboard as usize].set_handler_fn(keyboard_irq_handler);
        idt[InterruptVector::Mouse as usize].set_handler_fn(mouse_irq_handler);
        idt[InterruptVector::Com1 as usize].set_handler_fn(com1_irq_handler);
        idt[InterruptVector::PrimaryAta as usize].set_handler_fn(primary_ata_handler);
        idt[InterruptVector::SecondaryAta as usize].set_handler_fn(secondary_ata_handler);
        
        // Custom interrupts
        idt[InterruptVector::SchedulerTick as usize].set_handler_fn(scheduler_tick_handler);
        idt[InterruptVector::SystemCall as usize].set_handler_fn(syscall_handler);
        
        idt
    };
}

/// Initialize IDT
pub fn init_idt() {
    IDT.load();
    serial_println!("[IDT] Loaded interrupt descriptor table");
}

/// Remap and initialize PIC
pub fn remap_pic() {
    unsafe {
        // Initialize both PICs
        outb(PIC1_CMD, 0x11);    // ICW1: Initialize + ICW4 needed
        io_wait();
        outb(PIC2_CMD, 0x11);
        io_wait();
        
        // ICW2: Set vector offsets
        outb(PIC1_DATA, PIC1_BASE);  // Master: vectors 32-39
        io_wait();
        outb(PIC2_DATA, PIC2_BASE);  // Slave: vectors 40-47
        io_wait();
        
        // ICW3: Tell PICs about each other
        outb(PIC1_DATA, 0x04);   // Master has slave on IRQ2
        io_wait();
        outb(PIC2_DATA, 0x02);   // Slave is on IRQ2
        io_wait();
        
        // ICW4: 8086 mode
        outb(PIC1_DATA, 0x01);
        io_wait();
        outb(PIC2_DATA, 0x01);
        io_wait();
        
        // Mask all IRQs except keyboard and timer
        outb(PIC1_DATA, 0xFC);   // Enable timer (IRQ0) and keyboard (IRQ1)
        outb(PIC2_DATA, 0xFF);   // Mask all on slave
    }
    
    serial_println!("[PIC] Remapped to vectors {}-{}", PIC1_BASE, PIC2_BASE + 7);
}

/// Disable all IRQs on PIC
pub fn disable_all_irqs() {
    unsafe {
        outb(PIC1_DATA, 0xFF);
        outb(PIC2_DATA, 0xFF);
    }
}

/// Enable specific IRQ
pub fn enable_irq(irq: u8) {
    unsafe {
        if irq < 8 {
            let mask = inb(PIC1_DATA) & !(1 << irq);
            outb(PIC1_DATA, mask);
        } else {
            let mask = inb(PIC2_DATA) & !(1 << (irq - 8));
            outb(PIC2_DATA, mask);
        }
    }
}

/// Disable specific IRQ
pub fn disable_irq(irq: u8) {
    unsafe {
        if irq < 8 {
            let mask = inb(PIC1_DATA) | (1 << irq);
            outb(PIC1_DATA, mask);
        } else {
            let mask = inb(PIC2_DATA) | (1 << (irq - 8));
            outb(PIC2_DATA, mask);
        }
    }
}

/// Send End of Interrupt to PIC
pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_CMD, 0x20);  // Send EOI to slave
        }
        outb(PIC1_CMD, 0x20);      // Send EOI to master
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

/// I/O wait
fn io_wait() {
    unsafe { outb(0x80, 0); }
}

/// Port output (byte)
pub unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nostack, nomem));
}

/// Port input (byte)
pub unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!("in al, dx", out("al") val, in("dx") port, options(nostack, nomem));
    val
}

/// Port output (word)
pub unsafe fn outw(port: u16, val: u16) {
    core::arch::asm!("out dx, ax", in("dx") port, in("ax") val, options(nostack, nomem));
}

/// Port input (word)
pub unsafe fn inw(port: u16) -> u16 {
    let val: u16;
    core::arch::asm!("in ax, dx", out("ax") val, in("dx") port, options(nostack, nomem));
    val
}

/// Port output (dword)
pub unsafe fn outl(port: u16, val: u32) {
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") val, options(nostack, nomem));
}

/// Port input (dword)
pub unsafe fn inl(port: u16) -> u32 {
    let val: u32;
    core::arch::asm!("in eax, dx", out("eax") val, in("dx") port, options(nostack, nomem));
    val
}

/// Print exception info
fn print_exception_info(vector: u8, stack_frame: &InterruptStackFrame, error_code: Option<u64>) {
    let info = &EXCEPTION_INFOS[vector as usize];
    
    serial_println!("\n!!! EXCEPTION: {} !!!", info.name);
    serial_println!("Vector: {}", vector);
    
    if let Some(code) = error_code {
        serial_println!("Error Code: {:#x}", code);
    }
    
    serial_println!("\nStack Frame:");
    serial_println!("  RIP: {:#018x}", stack_frame.instruction_pointer.as_u64());
    serial_println!("  CS:  {:#06x}", stack_frame.code_segment);
    serial_println!("  RSP: {:#018x}", stack_frame.stack_pointer.as_u64());
    serial_println!("  SS:  {:#06x}", stack_frame.stack_segment);
    serial_println!("  RFLAGS: {:#018x}", stack_frame.cpu_flags);
}

// ========== Exception Handlers ==========

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    print_exception_info(0, &stack_frame, None);
    panic!("Divide by zero exception");
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    serial_println!("[DEBUG] Breakpoint at {:#x}", 
        stack_frame.instruction_pointer.as_u64());
}

extern "x86-interrupt" fn nmi_handler(stack_frame: InterruptStackFrame) {
    serial_println!("\n!!! NMI !!!");
    serial_println!("RIP: {:#x}", stack_frame.instruction_pointer.as_u64());
    panic!("Non-maskable interrupt");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    serial_println!("[BREAKPOINT] at {:#x}", 
        stack_frame.instruction_pointer.as_u64());
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    print_exception_info(4, &stack_frame, None);
    panic!("Overflow exception");
}

extern "x86-interrupt" fn bound_range_handler(stack_frame: InterruptStackFrame) {
    print_exception_info(5, &stack_frame, None);
    panic!("Bound range exceeded");
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    print_exception_info(6, &stack_frame, None);
    panic!("Invalid opcode");
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    print_exception_info(7, &stack_frame, None);
    panic!("Device not available");
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    print_exception_info(8, &stack_frame, Some(error_code));
    panic!("DOUBLE FAULT - System halted");
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    print_exception_info(10, &stack_frame, Some(error_code));
    panic!("Invalid TSS");
}

extern "x86-interrupt" fn segment_not_present_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    print_exception_info(11, &stack_frame, Some(error_code));
    panic!("Segment not present");
}

extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    print_exception_info(12, &stack_frame, Some(error_code));
    panic!("Stack segment fault");
}

extern "x86-interrupt" fn general_protection_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    print_exception_info(13, &stack_frame, Some(error_code));
    
    // Decode error code
    let table = match error_code & 0x3 {
        0 => "GDT",
        1 => "IDT",
        2 => "LDT",
        3 => "IDT",
        _ => "Unknown",
    };
    let index = (error_code >> 3) & 0x1FFF;
    let external = (error_code & 0x01) != 0;
    let table_idx = (error_code >> 1) & 0x03;
    
    serial_println!("\nGPF Details:");
    serial_println!("  Table: {} (bits: {})", table, table_idx);
    serial_println!("  Index: {}", index);
    serial_println!("  External: {}", external);
    
    panic!("General Protection Fault");
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    
    let fault_addr = Cr2::read();
    
    serial_println!("\n!!! PAGE FAULT !!!");
    serial_println!("Fault Address: {:#018x}", fault_addr.as_u64());
    serial_println!("IP: {:#018x}", stack_frame.instruction_pointer.as_u64());
    serial_println!("Error: {:?}", error_code);
    
    // Decode error
    if error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION) {
        serial_println!("  - Page present but protection violated");
    } else {
        serial_println!("  - Page not present");
    }
    
    if error_code.contains(PageFaultErrorCode::CAUSED_BY_WRITE) {
        serial_println!("  - Write access");
    } else {
        serial_println!("  - Read access");
    }
    
    if error_code.contains(PageFaultErrorCode::USER_MODE) {
        serial_println!("  - User mode access");
    } else {
        serial_println!("  - Kernel mode access");
    }
    
    panic!("Page Fault");
}

extern "x86-interrupt" fn x87_fpu_handler(stack_frame: InterruptStackFrame) {
    print_exception_info(16, &stack_frame, None);
    panic!("x87 FPU error");
}

extern "x86-interrupt" fn alignment_check_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    print_exception_info(17, &stack_frame, Some(error_code));
    panic!("Alignment check");
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    print_exception_info(18, &stack_frame, None);
    panic!("Machine check exception");
}

extern "x86-interrupt" fn simd_handler(stack_frame: InterruptStackFrame) {
    print_exception_info(19, &stack_frame, None);
    panic!("SIMD floating point exception");
}

extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    print_exception_info(20, &stack_frame, None);
    panic!("Virtualization exception");
}

extern "x86-interrupt" fn security_exception_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    print_exception_info(21, &stack_frame, Some(error_code));
    panic!("Security exception");
}

// ========== IRQ Handlers ==========

/// Timer tick counter
pub static TIMER_TICKS: Mutex<u64> = Mutex::new(0);

extern "x86-interrupt" fn timer_irq_handler(_stack_frame: InterruptStackFrame) {
    // Increment tick counter
    {
        let mut ticks = TIMER_TICKS.lock();
        *ticks += 1;
        
        // Print dot every second (assuming ~100 Hz timer)
        if *ticks % 100 == 0 {
            serial_print!(".");
        }
    }
    
    // Notify scheduler
    crate::process::timer_tick();
    
    // Send EOI
    send_eoi(0);
}

/// Keyboard buffer
pub static KEYBOARD_BUFFER: Mutex<[u8; 256]> = Mutex::new([0; 256]);
pub static KEYBOARD_HEAD: Mutex<usize> = Mutex::new(0);
pub static KEYBOARD_TAIL: Mutex<usize> = Mutex::new(0);

extern "x86-interrupt" fn keyboard_irq_handler(_stack_frame: InterruptStackFrame) {
    // Read scan code from keyboard
    let scan_code = unsafe { inb(0x60) };
    
    serial_println!("[KBD] Scan code: {:#x}", scan_code);
    
    // Store in buffer
    {
        let mut buf = KEYBOARD_BUFFER.lock();
        let mut tail = KEYBOARD_TAIL.lock();
        buf[*tail] = scan_code;
        *tail = (*tail + 1) % 256;
    }
    
    send_eoi(1);
}

extern "x86-interrupt" fn mouse_irq_handler(_stack_frame: InterruptStackFrame) {
    let data = unsafe { inb(0x60) };
    serial_println!("[MOUSE] Data: {:#x}", data);
    send_eoi(12);
}

extern "x86-interrupt" fn com1_irq_handler(_stack_frame: InterruptStackFrame) {
    let data = unsafe { inb(0x3F8) };
    serial_println!("[COM1] Data: {:#x}", data);
    send_eoi(4);
}

extern "x86-interrupt" fn primary_ata_handler(_stack_frame: InterruptStackFrame) {
    let status = unsafe { inb(0x1F7) };
    serial_println!("[ATA0] Status: {:#x}", status);
    send_eoi(14);
}

extern "x86-interrupt" fn secondary_ata_handler(_stack_frame: InterruptStackFrame) {
    let status = unsafe { inb(0x177) };
    serial_println!("[ATA1] Status: {:#x}", status);
    send_eoi(15);
}

// ========== Custom Handlers ==========

extern "x86-interrupt" fn scheduler_tick_handler(_stack_frame: InterruptStackFrame) {
    crate::process::timer_tick();
}

extern "x86-interrupt" fn syscall_handler(_stack_frame: InterruptStackFrame) {
    // System call handler - will be implemented in syscall module
    serial_println!("[SYSCALL] System call invoked");
}

// ========== APIC Functions ==========

/// Check if APIC is available
pub fn has_apic() -> bool {
    use x86_64::registers::model_specific::Msr;
    
    let cpuid = unsafe { core::arch::x86_64::__cpuid(1) };
    (cpuid.edx & (1 << 9)) != 0
}

/// Get APIC base address
pub fn get_apic_base() -> u64 {
    use x86_64::registers::model_specific::Msr;
    
    unsafe {
        let msr = Msr::new(APIC_BASE_MSR);
        msr.read() & 0xFFFF_F000
    }
}

/// Enable APIC
pub fn enable_apic() {
    if !has_apic() {
        serial_println!("[APIC] Not available, using PIC");
        return;
    }
    
    let base = get_apic_base();
    serial_println!("[APIC] Base address: {:#x}", base);
    
    // Enable APIC by setting spurious interrupt vector
    unsafe {
        let spurious = (base + APIC_SPURIOUS as u64) as *mut u32;
        let val = core::ptr::read_volatile(spurious);
        core::ptr::write_volatile(spurious, val | 0x100 | 0xFF);  // Enable + vector
    }
    
    serial_println!("[APIC] Enabled");
}