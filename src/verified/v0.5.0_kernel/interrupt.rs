// VantisOS v0.5.0 - Interrupt Handling
// Day 10: Configure Interrupt Handling

#![allow(unused_unsafe)]

// Interrupt Descriptor Table entry
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct IdtEntry {
    pub offset_low: u16,
    pub selector: u16,
    pub ist: u8,
    pub type_attr: u8,
    pub offset_mid: u16,
    pub offset_high: u32,
    pub reserved: u32,
}

impl IdtEntry {
    pub const fn new(offset: u64, selector: u16, type_attr: u8) -> IdtEntry {
        IdtEntry {
            offset_low: (offset & 0xFFFF) as u16,
            selector,
            ist: 0,
            type_attr,
            offset_mid: ((offset >> 16) & 0xFFFF) as u16,
            offset_high: ((offset >> 32) & 0xFFFFFFFF) as u32,
            reserved: 0,
        }
    }
    
    pub const fn missing() -> IdtEntry {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
}

// IDT pointer
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u64,
}

impl IdtPointer {
    pub const fn new(base: u64, limit: u16) -> IdtPointer {
        IdtPointer { limit, base }
    }
}

// Interrupt Handler type
pub type InterruptHandler = extern "C" fn();

// Exception handlers
extern "C" fn divide_error_handler() {
    print("Exception: Divide Error\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn debug_handler() {
    print("Exception: Debug\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn nmi_handler() {
    print("Exception: Non-Maskable Interrupt\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn breakpoint_handler() {
    print("Exception: Breakpoint\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn overflow_handler() {
    print("Exception: Overflow\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn bound_range_exceeded_handler() {
    print("Exception: Bound Range Exceeded\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn invalid_opcode_handler() {
    print("Exception: Invalid Opcode\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn device_not_available_handler() {
    print("Exception: Device Not Available\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn double_fault_handler() {
    print("Exception: Double Fault\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn invalid_tss_handler() {
    print("Exception: Invalid TSS\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn segment_not_present_handler() {
    print("Exception: Segment Not Present\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn stack_segment_fault_handler() {
    print("Exception: Stack Segment Fault\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn general_protection_fault_handler() {
    print("Exception: General Protection Fault\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn page_fault_handler() {
    print("Exception: Page Fault\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn x87_fpu_error_handler() {
    print("Exception: x87 FPU Error\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn alignment_check_handler() {
    print("Exception: Alignment Check\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn machine_check_handler() {
    print("Exception: Machine Check\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn simd_floating_point_handler() {
    print("Exception: SIMD Floating Point\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn virtualization_handler() {
    print("Exception: Virtualization\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

extern "C" fn security_exception_handler() {
    print("Exception: Security Exception\n");
    loop { unsafe { core::arch::asm!("hlt"); } }
}

// IRQ handlers
extern "C" fn timer_handler() {
    print("IRQ: Timer\n");
}

extern "C" fn keyboard_handler() {
    print("IRQ: Keyboard\n");
}

extern "C" fn cascade_handler() {
    print("IRQ: Cascade\n");
}

extern "C" fn com2_handler() {
    print("IRQ: COM2\n");
}

extern "C" fn com1_handler() {
    print("IRQ: COM1\n");
}

extern "C" fn lpt2_handler() {
    print("IRQ: LPT2\n");
}

extern "C" fn floppy_handler() {
    print("IRQ: Floppy\n");
}

extern "C" fn lpt1_handler() {
    print("IRQ: LPT1\n");
}

extern "C" fn rtc_handler() {
    print("IRQ: RTC\n");
}

extern "C" fn free1_handler() {
    print("IRQ: Free1\n");
}

extern "C" fn free2_handler() {
    print("IRQ: Free2\n");
}

extern "C" fn mouse_handler() {
    print("IRQ: Mouse\n");
}

extern "C" fn fpu_handler() {
    print("IRQ: FPU\n");
}

extern "C" fn primary_ata_handler() {
    print("IRQ: Primary ATA\n");
}

extern "C" fn secondary_ata_handler() {
    print("IRQ: Secondary ATA\n");
}

// IDT
static mut IDT: [IdtEntry; 256] = [IdtEntry::missing(); 256];

// Initialize IDT
pub fn init_idt() {
    unsafe {
        // Exception handlers (0-31)
        IDT[0] = IdtEntry::new(divide_error_handler as u64, 0x08, 0x8E);
        IDT[1] = IdtEntry::new(debug_handler as u64, 0x08, 0x8E);
        IDT[2] = IdtEntry::new(nmi_handler as u64, 0x08, 0x8E);
        IDT[3] = IdtEntry::new(breakpoint_handler as u64, 0x08, 0x8E);
        IDT[4] = IdtEntry::new(overflow_handler as u64, 0x08, 0x8E);
        IDT[5] = IdtEntry::new(bound_range_exceeded_handler as u64, 0x08, 0x8E);
        IDT[6] = IdtEntry::new(invalid_opcode_handler as u64, 0x08, 0x8E);
        IDT[7] = IdtEntry::new(device_not_available_handler as u64, 0x08, 0x8E);
        IDT[8] = IdtEntry::new(double_fault_handler as u64, 0x08, 0x8E);
        IDT[10] = IdtEntry::new(invalid_tss_handler as u64, 0x08, 0x8E);
        IDT[11] = IdtEntry::new(segment_not_present_handler as u64, 0x08, 0x8E);
        IDT[12] = IdtEntry::new(stack_segment_fault_handler as u64, 0x08, 0x8E);
        IDT[13] = IdtEntry::new(general_protection_fault_handler as u64, 0x08, 0x8E);
        IDT[14] = IdtEntry::new(page_fault_handler as u64, 0x08, 0x8E);
        IDT[16] = IdtEntry::new(x87_fpu_error_handler as u64, 0x08, 0x8E);
        IDT[17] = IdtEntry::new(alignment_check_handler as u64, 0x08, 0x8E);
        IDT[18] = IdtEntry::new(machine_check_handler as u64, 0x08, 0x8E);
        IDT[19] = IdtEntry::new(simd_floating_point_handler as u64, 0x08, 0x8E);
        IDT[20] = IdtEntry::new(virtualization_handler as u64, 0x08, 0x8E);
        IDT[30] = IdtEntry::new(security_exception_handler as u64, 0x08, 0x8E);
        
        // IRQ handlers (32-47)
        IDT[32] = IdtEntry::new(timer_handler as u64, 0x08, 0x8E);
        IDT[33] = IdtEntry::new(keyboard_handler as u64, 0x08, 0x8E);
        IDT[34] = IdtEntry::new(cascade_handler as u64, 0x08, 0x8E);
        IDT[35] = IdtEntry::new(com2_handler as u64, 0x08, 0x8E);
        IDT[36] = IdtEntry::new(com1_handler as u64, 0x08, 0x8E);
        IDT[37] = IdtEntry::new(lpt2_handler as u64, 0x08, 0x8E);
        IDT[38] = IdtEntry::new(floppy_handler as u64, 0x08, 0x8E);
        IDT[39] = IdtEntry::new(lpt1_handler as u64, 0x08, 0x8E);
        IDT[40] = IdtEntry::new(rtc_handler as u64, 0x08, 0x8E);
        IDT[41] = IdtEntry::new(free1_handler as u64, 0x08, 0x8E);
        IDT[42] = IdtEntry::new(free2_handler as u64, 0x08, 0x8E);
        IDT[43] = IdtEntry::new(mouse_handler as u64, 0x08, 0x8E);
        IDT[44] = IdtEntry::new(fpu_handler as u64, 0x08, 0x8E);
        IDT[45] = IdtEntry::new(primary_ata_handler as u64, 0x08, 0x8E);
        IDT[46] = IdtEntry::new(secondary_ata_handler as u64, 0x08, 0x8E);
    }
}

// Load IDT
pub fn load_idt() {
    unsafe {
        let idt_pointer = IdtPointer::new(&IDT as *const _ as u64, 256 * 16 - 1);
        core::arch::asm!("lidt [{}]", in(reg) &idt_pointer, options(readonly, nostack));
    }
}

// Enable interrupts
pub fn enable_interrupts() {
    unsafe {
        core::arch::asm!("sti");
    }
}

// Disable interrupts
pub fn disable_interrupts() {
    unsafe {
        core::arch::asm!("cli");
    }
}

// Print function (uses VGA console)
fn print(s: &str) {
    use crate::vga_console::write_string;
    write_string(s);
}