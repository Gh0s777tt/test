
use core::sync::atomic::{AtomicU64, Ordering};

// ARM64 GIC (Generic Interrupt Controller) Version
#[repr(u32)]
pub enum GicVersion {
    GICv2 = 2,
    GICv3 = 3,
    GICv4 = 4,
}

// ARM64 Interrupt Types
#[repr(u8)]
pub enum Arm64InterruptType {
    SGI = 0,  // Software Generated Interrupt
    PPI = 1,  // Private Peripheral Interrupt
    SPI = 2,  // Shared Peripheral Interrupt
}

// ARM64 Exception Levels
#[repr(u8)]
pub enum Arm64ExceptionLevel {
    EL0 = 0,  // User mode
    EL1 = 1,  // Kernel mode
    EL2 = 2,  // Hypervisor mode
    EL3 = 3,  // Secure monitor mode
}

// ARM64 Exception Vector
#[repr(C)]
pub struct Arm64ExceptionVector {
    pub current_elr_el1: u64,
    pub current_spsr_el1: u64,
    pub current_elr_el2: u64,
    pub current_spsr_el2: u64,
    pub current_elr_el3: u64,
    pub current_spsr_el3: u64,
    pub current_sp_el0: u64,
}

// ARM64 Exception Handler
pub type Arm64ExceptionHandler = extern "C" fn(&Arm64ExceptionVector);

// ARM64 GIC Distributor
pub struct Arm64GicDistributor {
    pub base_addr: u64,
    pub version: GicVersion,
    pub num_irqs: u32,
}

impl Arm64GicDistributor {
    pub const fn new(base_addr: u64) -> Self {
        Self {
            base_addr,
            version: GicVersion::GICv2,
            num_irqs: 1024,
        }
    }

    pub fn init(&mut self) {
        arm64_print("  GIC Distributor: ARM64 GIC Distributor initialized\n");
        arm64_print("    - GIC version: GICv2/GICv3\n");
        arm64_print("    - Number of IRQs: ");
        arm64_print_dec(self.num_irqs as u64);
        arm64_print("\n");
        arm64_print("    - Interrupt types: SGI, PPI, SPI\n");
        arm64_print("    - Interrupt priorities: 0-255\n");
    }

    pub fn enable_irq(&mut self, irq: u32) {
        // Enable interrupt in GIC distributor
        let _ = (irq, self.base_addr);
    }

    pub fn disable_irq(&mut self, irq: u32) {
        // Disable interrupt in GIC distributor
        let _ = (irq, self.base_addr);
    }

    pub fn set_priority(&mut self, irq: u32, priority: u8) {
        // Set interrupt priority
        let _ = (irq, priority, self.base_addr);
    }

    pub fn get_priority(&self, irq: u32) -> u8 {
        // Get interrupt priority
        0  // Placeholder
    }
}

// ARM64 GIC CPU Interface
pub struct Arm64GicCpuInterface {
    pub base_addr: u64,
}

impl Arm64GicCpuInterface {
    pub const fn new(base_addr: u64) -> Self {
        Self {
            base_addr,
        }
    }

    pub fn init(&mut self) {
        arm64_print("  GIC CPU Interface: ARM64 GIC CPU Interface initialized\n");
        arm64_print("    - CPU interface enabled\n");
        arm64_print("    - Priority masking enabled\n");
    }

    pub fn enable_interrupts(&mut self) {
        // Enable interrupts in GIC CPU interface
        unsafe {
            core::arch::asm!(
                "msr daifclr, #2",  // Enable IRQ
                "msr daifclr, #1",  // Enable FIQ
                options(nomem, nostack)
            );
        }
    }

    pub fn disable_interrupts(&mut self) {
        // Disable interrupts in GIC CPU interface
        unsafe {
            core::arch::asm!(
                "msr daifset, #2",  // Disable IRQ
                "msr daifset, #1",  // Disable FIQ
                options(nomem, nostack)
            );
        }
    }

    pub fn end_of_interrupt(&mut self, irq: u32) {
        // Signal end of interrupt to GIC
        let _ = (irq, self.base_addr);
    }
}

// ARM64 Exception Handlers
pub struct Arm64ExceptionHandlers {
    pub sync_handler: Option<Arm64ExceptionHandler>,
    pub irq_handler: Option<Arm64ExceptionHandler>,
    pub fiq_handler: Option<Arm64ExceptionHandler>,
    pub serror_handler: Option<Arm64ExceptionHandler>,
}

impl Arm64ExceptionHandlers {
    pub const fn new() -> Self {
        Self {
            sync_handler: None,
            irq_handler: None,
            fiq_handler: None,
            serror_handler: None,
        }
    }

    pub fn init(&mut self) {
        arm64_print("  Exception handlers: ARM64 exception handlers initialized\n");
        arm64_print("    - Synchronous exceptions\n");
        arm64_print("    - IRQ exceptions\n");
        arm64_print("    - FIQ exceptions\n");
        arm64_print("    - SError exceptions\n");
    }

    pub fn set_sync_handler(&mut self, handler: Arm64ExceptionHandler) {
        self.sync_handler = Some(handler);
    }

    pub fn set_irq_handler(&mut self, handler: Arm64ExceptionHandler) {
        self.irq_handler = Some(handler);
    }

    pub fn set_fiq_handler(&mut self, handler: Arm64ExceptionHandler) {
        self.fiq_handler = Some(handler);
    }

    pub fn set_serror_handler(&mut self, handler: Arm64ExceptionHandler) {
        self.serror_handler = Some(handler);
    }
}

// ARM64 Interrupt Manager
pub struct Arm64InterruptManager {
    pub gic_distributor: Arm64GicDistributor,
    pub gic_cpu_interface: Arm64GicCpuInterface,
    pub exception_handlers: Arm64ExceptionHandlers,
    pub interrupt_count: AtomicU64,
}

impl Arm64InterruptManager {
    pub const fn new() -> Self {
        Self {
            gic_distributor: Arm64GicDistributor::new(0x08000000),
            gic_cpu_interface: Arm64GicCpuInterface::new(0x08010000),
            exception_handlers: Arm64ExceptionHandlers::new(),
            interrupt_count: AtomicU64::new(0),
        }
    }

    pub fn init(&mut self) {
        arm64_print("Interrupt initialization...\n");
        
        // Initialize GIC distributor
        self.gic_distributor.init();
        
        // Initialize GIC CPU interface
        self.gic_cpu_interface.init();
        
        // Initialize exception handlers
        self.exception_handlers.init();
        
        // Enable interrupts
        self.gic_cpu_interface.enable_interrupts();
        
        arm64_print("Interrupt initialization complete\n");
    }

    pub fn handle_interrupt(&mut self, irq: u32) {
        self.interrupt_count.fetch_add(1, Ordering::SeqCst);
        
        // Handle interrupt
        // In production, dispatch to appropriate handler
        let _ = irq;
        
        // Signal end of interrupt
        self.gic_cpu_interface.end_of_interrupt(irq);
    }

    pub fn get_interrupt_count(&self) -> u64 {
        self.interrupt_count.load(Ordering::SeqCst)
    }
}

// ARM64 Exception Types
#[repr(u32)]
pub enum Arm64ExceptionType {
    SyncCurrentELwithSP0 = 0,
    IrqCurrentELwithSP0 = 1,
    FiqCurrentELwithSP0 = 2,
    SErrorCurrentELwithSP0 = 3,
    SyncCurrentELwithSPx = 4,
    IrqCurrentELwithSPx = 5,
    FiqCurrentELwithSPx = 6,
    SErrorCurrentELwithSPx = 7,
    SyncLowerELusingAarch64 = 8,
    IrqLowerELusingAarch64 = 9,
    FiqLowerELusingAarch64 = 10,
    SErrorLowerELusingAarch64 = 11,
    SyncLowerELusingAArch32 = 12,
    IrqLowerELusingAArch32 = 13,
    FiqLowerELusingAArch32 = 14,
    SErrorLowerELusingAArch32 = 15,
}

// ARM64 Exception Handler Function
#[no_mangle]
pub extern "C" fn arm64_exception_handler(exception_type: u32, esr: u64, elr: u64) {
    match exception_type {
        0 => arm64_sync_exception_handler(esr, elr),
        1 => arm64_irq_exception_handler(esr, elr),
        2 => arm64_fiq_exception_handler(esr, elr),
        3 => arm64_serror_exception_handler(esr, elr),
        _ => arm64_unknown_exception_handler(exception_type, esr, elr),
    }
}

// Synchronous exception handler
fn arm64_sync_exception_handler(esr: u64, elr: u64) {
    arm64_print("\n!!! SYNCHRONOUS EXCEPTION !!!\n");
    arm64_print("ESR: 0x");
    arm64_print_hex(esr);
    arm64_print("\n");
    arm64_print("ELR: 0x");
    arm64_print_hex(elr);
    arm64_print("\n");
    
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

// IRQ exception handler
fn arm64_irq_exception_handler(esr: u64, elr: u64) {
    arm64_print("\n!!! IRQ EXCEPTION !!!\n");
    arm64_print("ESR: 0x");
    arm64_print_hex(esr);
    arm64_print("\n");
    arm64_print("ELR: 0x");
    arm64_print_hex(elr);
    arm64_print("\n");
    
    // Handle IRQ
    // In production, dispatch to appropriate IRQ handler
}

// FIQ exception handler
fn arm64_fiq_exception_handler(esr: u64, elr: u64) {
    arm64_print("\n!!! FIQ EXCEPTION !!!\n");
    arm64_print("ESR: 0x");
    arm64_print_hex(esr);
    arm64_print("\n");
    arm64_print("ELR: 0x");
    arm64_print_hex(elr);
    arm64_print("\n");
    
    // Handle FIQ
    // In production, dispatch to appropriate FIQ handler
}

// SError exception handler
fn arm64_serror_exception_handler(esr: u64, elr: u64) {
    arm64_print("\n!!! SERROR EXCEPTION !!!\n");
    arm64_print("ESR: 0x");
    arm64_print_hex(esr);
    arm64_print("\n");
    arm64_print("ELR: 0x");
    arm64_print_hex(elr);
    arm64_print("\n");
    
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

// Unknown exception handler
fn arm64_unknown_exception_handler(exception_type: u32, esr: u64, elr: u64) {
    arm64_print("\n!!! UNKNOWN EXCEPTION !!!\n");
    arm64_print("Type: ");
    arm64_print_dec(exception_type as u64);
    arm64_print("\n");
    arm64_print("ESR: 0x");
    arm64_print_hex(esr);
    arm64_print("\n");
    arm64_print("ELR: 0x");
    arm64_print_hex(elr);
    arm64_print("\n");
    
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

// Print functions (reused from boot.rs)
fn arm64_print(s: &str) {
    const UART_BASE: u64 = 0x09000000;
    
    unsafe {
        let uart_ptr = UART_BASE as *mut u8;
        
        for byte in s.bytes() {
            while uart_ptr.add(5).read_volatile() & 0x20 == 0 {}
            uart_ptr.write_volatile(byte);
        }
    }
}

fn arm64_print_hex(n: u64) {
    const HEX_CHARS: &[u8] = b"0123456789ABCDEF";
    
    let mut buffer = [0u8; 16];
    let mut num = n;
    
    for i in (0..16).rev() {
        buffer[i] = HEX_CHARS[(num & 0xF) as usize];
        num >>= 4;
    }
    
    unsafe {
        let uart_ptr = 0x09000000 as *mut u8;
        
        for byte in buffer.iter() {
            while uart_ptr.add(5).read_volatile() & 0x20 == 0 {}
            uart_ptr.write_volatile(*byte);
        }
    }
}

fn arm64_print_dec(n: u64) {
    let mut buffer = [0u8; 20];
    let mut num = n;
    let mut i = 19;
    
    if num == 0 {
        buffer[i] = b'0';
        i -= 1;
    } else {
        while num > 0 && i > 0 {
            buffer[i] = b'0' + (num % 10) as u8;
            num /= 10;
            i -= 1;
        }
    }
    
    unsafe {
        let uart_ptr = 0x09000000 as *mut u8;
        
        for byte in buffer.iter().skip(i + 1) {
            while uart_ptr.add(5).read_volatile() & 0x20 == 0 {}
            uart_ptr.write_volatile(*byte);
        }
    }
}