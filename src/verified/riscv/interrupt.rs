//! RISC-V Interrupt Handling
//! 
//! This module implements RISC-V interrupt handling including:
//! - Trap vector setup
//! - Exception handling
//! - Interrupt controller (PLIC)
//! - Timer interrupts

#![no_std]

use core::arch::asm;

/// Trap cause codes
pub mod trap_cause {
    /// Instruction address misaligned
    pub const INSTRUCTION_MISALIGNED: usize = 0;
    
    /// Instruction access fault
    pub const INSTRUCTION_ACCESS_FAULT: usize = 1;
    
    /// Illegal instruction
    pub const ILLEGAL_INSTRUCTION: usize = 2;
    
    /// Breakpoint
    pub const BREAKPOINT: usize = 3;
    
    /// Load address misaligned
    pub const LOAD_MISALIGNED: usize = 4;
    
    /// Load access fault
    pub const LOAD_ACCESS_FAULT: usize = 5;
    
    /// Store/AMO address misaligned
    pub const STORE_MISALIGNED: usize = 6;
    
    /// Store/AMO access fault
    pub const STORE_ACCESS_FAULT: usize = 7;
    
    /// Environment call from U-mode
    pub const ECALL_U: usize = 8;
    
    /// Environment call from S-mode
    pub const ECALL_S: usize = 9;
    
    /// Environment call from M-mode
    pub const ECALL_M: usize = 11;
    
    /// Instruction page fault
    pub const INSTRUCTION_PAGE_FAULT: usize = 12;
    
    /// Load page fault
    pub const LOAD_PAGE_FAULT: usize = 13;
    
    /// Store/AMO page fault
    pub const STORE_PAGE_FAULT: usize = 15;
    
    /// User software interrupt
    pub const USER_SOFTWARE_INTERRUPT: usize = 0;
    
    /// Supervisor software interrupt
    pub const SUPERVISOR_SOFTWARE_INTERRUPT: usize = 1;
    
    /// Machine software interrupt
    pub const MACHINE_SOFTWARE_INTERRUPT: usize = 3;
    
    /// User timer interrupt
    pub const USER_TIMER_INTERRUPT: usize = 4;
    
    /// Supervisor timer interrupt
    pub const SUPERVISOR_TIMER_INTERRUPT: usize = 5;
    
    /// Machine timer interrupt
    pub const MACHINE_TIMER_INTERRUPT: usize = 7;
    
    /// User external interrupt
    pub const USER_EXTERNAL_INTERRUPT: usize = 8;
    
    /// Supervisor external interrupt
    pub const SUPERVISOR_EXTERNAL_INTERRUPT: usize = 9;
    
    /// Machine external interrupt
    pub const MACHINE_EXTERNAL_INTERRUPT: usize = 11;
}

/// Trap frame
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame {
    /// General-purpose registers
    pub x: [usize; 32],
    
    /// Floating-point registers
    pub f: [u64; 32],
    
    /// Program counter
    pub pc: usize,
    
    /// Status register
    pub status: usize,
    
    /// Cause register
    pub cause: usize,
    
    /// Value register
    pub tval: usize,
}

impl TrapFrame {
    /// Create a new trap frame
    pub const fn new() -> Self {
        TrapFrame {
            x: [0; 32],
            f: [0; 32],
            pc: 0,
            status: 0,
            cause: 0,
            tval: 0,
        }
    }
}

/// Interrupt handler type
pub type InterruptHandler = fn(&mut TrapFrame);

/// Exception handler type
pub type ExceptionHandler = fn(&mut TrapFrame);

/// Interrupt and exception handlers
pub struct Handlers {
    /// Software interrupt handlers
    pub software_interrupts: [Option<InterruptHandler>; 4],
    
    /// Timer interrupt handlers
    pub timer_interrupts: [Option<InterruptHandler>; 4],
    
    /// External interrupt handlers
    pub external_interrupts: [Option<InterruptHandler>; 4],
    
    /// Exception handlers
    pub exceptions: [Option<ExceptionHandler>; 16],
}

impl Handlers {
    /// Create a new handlers structure
    pub const fn new() -> Self {
        Handlers {
            software_interrupts: [None; 4],
            timer_interrupts: [None; 4],
            external_interrupts: [None; 4],
            exceptions: [None; 16],
        }
    }
    
    /// Set software interrupt handler
    pub fn set_software_interrupt(&mut self, mode: usize, handler: InterruptHandler) {
        self.software_interrupts[mode] = Some(handler);
    }
    
    /// Set timer interrupt handler
    pub fn set_timer_interrupt(&mut self, mode: usize, handler: InterruptHandler) {
        self.timer_interrupts[mode] = Some(handler);
    }
    
    /// Set external interrupt handler
    pub fn set_external_interrupt(&mut self, mode: usize, handler: InterruptHandler) {
        self.external_interrupts[mode] = Some(handler);
    }
    
    /// Set exception handler
    pub fn set_exception(&mut self, cause: usize, handler: ExceptionHandler) {
        self.exceptions[cause] = Some(handler);
    }
}

/// Global interrupt handlers
static mut HANDLERS: Handlers = Handlers::new();

/// Initialize interrupt handling
pub fn init() {
    unsafe {
        // Set trap vector
        set_trap_vector(trap_entry as usize);
        
        // Enable supervisor interrupts
        enable_interrupts();
        
        // Setup default handlers
        setup_default_handlers();
    }
}

/// Set trap vector
fn set_trap_vector(vector: usize) {
    unsafe {
        asm!(
            "csrw stvec, {}",
            in(reg) vector,
        );
    }
}

/// Enable interrupts
pub fn enable_interrupts() {
    unsafe {
        asm!(
            "csrsi sstatus, 0x2", // Set SIE bit
        );
    }
}

/// Disable interrupts
pub fn disable_interrupts() {
    unsafe {
        asm!(
            "csrci sstatus, 0x2", // Clear SIE bit
        );
    }
}

/// Check if interrupts are enabled
pub fn interrupts_enabled() -> bool {
    let enabled: usize;
    unsafe {
        asm!(
            "csrr {}, sstatus",
            out(reg) enabled,
        );
    }
    enabled & 0x2 != 0
}

/// Setup default handlers
fn setup_default_handlers() {
    unsafe {
        // Timer interrupt handler
        HANDLERS.set_timer_interrupt(1, timer_interrupt_handler);
        
        // External interrupt handler
        HANDLERS.set_external_interrupt(1, external_interrupt_handler);
        
        // Exception handlers
        HANDLERS.set_exception(trap_cause::ILLEGAL_INSTRUCTION, illegal_instruction_handler);
        HANDLERS.set_exception(trap_cause::BREAKPOINT, breakpoint_handler);
        HANDLERS.set_exception(trap_cause::ECALL_S, syscall_handler);
        HANDLERS.set_exception(trap_cause::INSTRUCTION_PAGE_FAULT, page_fault_handler);
        HANDLERS.set_exception(trap_cause::LOAD_PAGE_FAULT, page_fault_handler);
        HANDLERS.set_exception(trap_cause::STORE_PAGE_FAULT, page_fault_handler);
    }
}

/// Trap entry point
#[no_mangle]
pub extern "C" fn trap_entry() -> ! {
    let mut frame = TrapFrame::new();
    
    unsafe {
        // Save trap frame
        save_trap_frame(&mut frame);
        
        // Get trap cause
        let cause = frame.cause;
        let is_interrupt = cause & (1 << 63) != 0;
        let cause_num = cause & !(1 << 63);
        
        if is_interrupt {
            // Handle interrupt
            handle_interrupt(&mut frame, cause_num);
        } else {
            // Handle exception
            handle_exception(&mut frame, cause_num);
        }
        
        // Restore trap frame
        restore_trap_frame(&frame);
    }
    
    // Return from trap
    unsafe {
        asm!(
            "sret",
        );
    }
    
    unreachable!()
}

/// Handle interrupt
fn handle_interrupt(frame: &mut TrapFrame, cause: usize) {
    unsafe {
        match cause {
            trap_cause::SUPERVISOR_SOFTWARE_INTERRUPT => {
                if let Some(handler) = HANDLERS.software_interrupts[1] {
                    handler(frame);
                }
            }
            trap_cause::SUPERVISOR_TIMER_INTERRUPT => {
                if let Some(handler) = HANDLERS.timer_interrupts[1] {
                    handler(frame);
                }
            }
            trap_cause::SUPERVISOR_EXTERNAL_INTERRUPT => {
                if let Some(handler) = HANDLERS.external_interrupts[1] {
                    handler(frame);
                }
            }
            _ => {
                // Unknown interrupt - ignore
            }
        }
    }
}

/// Handle exception
fn handle_exception(frame: &mut TrapFrame, cause: usize) {
    unsafe {
        if cause < HANDLERS.exceptions.len() {
            if let Some(handler) = HANDLERS.exceptions[cause] {
                handler(frame);
            } else {
                // Unhandled exception - panic
                panic!("Unhandled exception: {}", cause);
            }
        } else {
            panic!("Invalid exception cause: {}", cause);
        }
    }
}

/// Timer interrupt handler
fn timer_interrupt_handler(frame: &mut TrapFrame) {
    // Acknowledge timer interrupt
    extern "C" {
        fn sbi_set_timer(stime_value: u64);
    }
    
    unsafe {
        // Set next timer interrupt
        const MTIME_FREQ: u64 = 10_000_000; // 10 MHz
        let mtime: u64;
        asm!(
            "csrr {}, mtime",
            out(reg) mtime,
        );
        sbi_set_timer(mtime + MTIME_FREQ);
    }
    
    // Call scheduler
    extern "C" {
        fn riscv_scheduler_tick();
    }
    
    unsafe {
        riscv_scheduler_tick();
    }
}

/// External interrupt handler
fn external_interrupt_handler(frame: &mut TrapFrame) {
    // Handle PLIC interrupts
    handle_plic_interrupts();
}

/// Handle PLIC interrupts
fn handle_plic_interrupts() {
    const PLIC_BASE: usize = 0x0c000000;
    
    unsafe {
        let plic = PLIC_BASE as *mut u32;
        
        // Claim interrupt
        let claim = plic.add(0x200004).read_volatile();
        
        if claim != 0 {
            // Handle interrupt based on claim
            match claim {
                10 => {
                    // UART interrupt
                    handle_uart_interrupt();
                }
                _ => {
                    // Unknown interrupt
                }
            }
            
            // Complete interrupt
            plic.add(0x200004).write_volatile(claim);
        }
    }
}

/// Handle UART interrupt
fn handle_uart_interrupt() {
    // Read UART data
    const UART_BASE: usize = 0x10000000;
    
    unsafe {
        let uart = UART_BASE as *mut u8;
        
        // Read received byte
        let byte = uart.read_volatile();
        
        // Process byte (e.g., add to input buffer)
        // ...
    }
}

/// Illegal instruction handler
fn illegal_instruction_handler(frame: &mut TrapFrame) {
    panic!("Illegal instruction at 0x{:x}", frame.pc);
}

/// Breakpoint handler
fn breakpoint_handler(frame: &mut TrapFrame) {
    // Skip the breakpoint instruction
    frame.pc += 2;
}

/// System call handler
fn syscall_handler(frame: &mut TrapFrame) {
    // System call number is in a7
    let syscall_num = frame.x[17];
    
    // Handle system call
    extern "C" {
        fn riscv_syscall_dispatch(num: usize, args: &[usize]) -> usize;
    }
    
    let result = unsafe {
        riscv_syscall_dispatch(syscall_num, &frame.x[0..7])
    };
    
    // Return result in a0
    frame.x[10] = result;
    
    // Advance PC past ecall instruction
    frame.pc += 4;
}

/// Page fault handler
fn page_fault_handler(frame: &mut TrapFrame) {
    panic!("Page fault at 0x{:x}, tval=0x{:x}", frame.pc, frame.tval);
}

/// Save trap frame (assembly stub)
#[no_mangle]
extern "C" fn save_trap_frame(frame: &mut TrapFrame) {
    unsafe {
        asm!(
            // Save general-purpose registers
            "sd x1, 8(a0)",
            "sd x2, 16(a0)",
            "sd x3, 24(a0)",
            "sd x4, 32(a0)",
            "sd x5, 40(a0)",
            "sd x6, 48(a0)",
            "sd x7, 56(a0)",
            "sd x8, 64(a0)",
            "sd x9, 72(a0)",
            "sd x10, 80(a0)",
            "sd x11, 88(a0)",
            "sd x12, 96(a0)",
            "sd x13, 104(a0)",
            "sd x14, 112(a0)",
            "sd x15, 120(a0)",
            "sd x16, 128(a0)",
            "sd x17, 136(a0)",
            "sd x18, 144(a0)",
            "sd x19, 152(a0)",
            "sd x20, 160(a0)",
            "sd x21, 168(a0)",
            "sd x22, 176(a0)",
            "sd x23, 184(a0)",
            "sd x24, 192(a0)",
            "sd x25, 200(a0)",
            "sd x26, 208(a0)",
            "sd x27, 216(a0)",
            "sd x28, 224(a0)",
            "sd x29, 232(a0)",
            "sd x30, 240(a0)",
            "sd x31, 248(a0)",
            
            // Save PC, status, cause, tval
            "csrr t0, sepc",
            "sd t0, 256(a0)",
            "csrr t0, sstatus",
            "sd t0, 264(a0)",
            "csrr t0, scause",
            "sd t0, 272(a0)",
            "csrr t0, stval",
            "sd t0, 280(a0)",
        );
    }
}

/// Restore trap frame (assembly stub)
#[no_mangle]
extern "C" fn restore_trap_frame(frame: &TrapFrame) {
    unsafe {
        asm!(
            // Restore PC, status, cause, tval
            "ld t0, 256(a0)",
            "csrw sepc, t0",
            "ld t0, 264(a0)",
            "csrw sstatus, t0",
            "ld t0, 272(a0)",
            "csrw scause, t0",
            "ld t0, 280(a0)",
            "csrw stval, t0",
            
            // Restore general-purpose registers
            "ld x1, 8(a0)",
            "ld x2, 16(a0)",
            "ld x3, 24(a0)",
            "ld x4, 32(a0)",
            "ld x5, 40(a0)",
            "ld x6, 48(a0)",
            "ld x7, 56(a0)",
            "ld x8, 64(a0)",
            "ld x9, 72(a0)",
            "ld x10, 80(a0)",
            "ld x11, 88(a0)",
            "ld x12, 96(a0)",
            "ld x13, 104(a0)",
            "ld x14, 112(a0)",
            "ld x15, 120(a0)",
            "ld x16, 128(a0)",
            "ld x17, 136(a0)",
            "ld x18, 144(a0)",
            "ld x19, 152(a0)",
            "ld x20, 160(a0)",
            "ld x21, 168(a0)",
            "ld x22, 176(a0)",
            "ld x23, 184(a0)",
            "ld x24, 192(a0)",
            "ld x25, 200(a0)",
            "ld x26, 208(a0)",
            "ld x27, 216(a0)",
            "ld x28, 224(a0)",
            "ld x29, 232(a0)",
            "ld x30, 240(a0)",
            "ld x31, 248(a0)",
        );
    }
}

/// Initialize RISC-V interrupt handling
#[no_mangle]
pub extern "C" fn riscv_interrupt_init() {
    init();
}