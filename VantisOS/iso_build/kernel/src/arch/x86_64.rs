//! x86_64 Architecture Support
//! Provides low-level CPU operations and structures

use core::arch::asm;

/// Halt the CPU
pub fn halt() {
    unsafe {
        asm!("hlt");
    }
}

/// Disable interrupts
pub fn cli() {
    unsafe {
        asm!("cli");
    }
}

/// Enable interrupts
pub fn sti() {
    unsafe {
        asm!("sti");
    }
}

/// Read from a CPU model-specific register
pub fn rdmsr(msr: u32) -> u64 {
    let (high, low): (u32, u32);
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            out("eax") low,
            out("edx") high,
        );
    }
    ((high as u64) << 32) | (low as u64)
}

/// Write to a CPU model-specific register
pub fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") low,
            in("edx") high,
        );
    }
}

/// Read CR0 register
pub fn read_cr0() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr0", out(reg) value);
    }
    value
}

/// Write CR0 register
pub fn write_cr0(value: u64) {
    unsafe {
        asm!("mov cr0, {}", in(reg) value);
    }
}

/// Read CR2 register (page fault address)
pub fn read_cr2() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr2", out(reg) value);
    }
    value
}

/// Read CR3 register (page table root)
pub fn read_cr3() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr3", out(reg) value);
    }
    value
}

/// Write CR3 register
pub fn write_cr3(value: u64) {
    unsafe {
        asm!("mov cr3, {}", in(reg) value);
    }
}

/// Read CR4 register
pub fn read_cr4() -> u64 {
    let value: u64;
    unsafe {
        asm!("mov {}, cr4", out(reg) value);
    }
    value
}

/// Write CR4 register
pub fn write_cr4(value: u64) {
    unsafe {
        asm!("mov cr4, {}", in(reg) value);
    }
}

/// Invalidate TLB entry
pub fn invlpg(addr: u64) {
    unsafe {
        asm!("invlpg [{0}]", in(reg) addr);
    }
}

/// CPUID instruction
pub fn cpuid(leaf: u32) -> (u32, u32, u32, u32) {
    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    unsafe {
        // Save rbx since it's reserved by LLVM
        // Use rdi as a temporary to hold ebx value
        asm!(
            "push rbx",
            "cpuid",
            "mov rdi, rbx",  // Copy ebx to rdi
            "pop rbx",       // Restore rbx
            inout("eax") leaf => eax,
            out("rdi") ebx,
            lateout("ecx") ecx,
            lateout("edx") edx,
        );
    }
    (eax, ebx, ecx, edx)
}

/// Get CPU vendor string
pub fn get_vendor() -> [u8; 12] {
    let (_, ebx, ecx, edx) = cpuid(0);
    let mut vendor = [0u8; 12];
    vendor[0..4].copy_from_slice(&ebx.to_le_bytes());
    vendor[4..8].copy_from_slice(&edx.to_le_bytes());
    vendor[8..12].copy_from_slice(&ecx.to_le_bytes());
    vendor
}

/// Get CPU brand string
pub fn get_brand() -> [u8; 48] {
    let mut brand = [0u8; 48];
    
    let (eax1, ebx1, ecx1, edx1) = cpuid(0x80000001);
    brand[0..4].copy_from_slice(&eax1.to_le_bytes());
    brand[4..8].copy_from_slice(&ebx1.to_le_bytes());
    brand[8..12].copy_from_slice(&ecx1.to_le_bytes());
    brand[12..16].copy_from_slice(&edx1.to_le_bytes());
    
    let (eax2, ebx2, ecx2, edx2) = cpuid(0x80000002);
    brand[16..20].copy_from_slice(&eax2.to_le_bytes());
    brand[20..24].copy_from_slice(&ebx2.to_le_bytes());
    brand[24..28].copy_from_slice(&ecx2.to_le_bytes());
    brand[28..32].copy_from_slice(&edx2.to_le_bytes());
    
    let (eax3, ebx3, ecx3, edx3) = cpuid(0x80000003);
    brand[32..36].copy_from_slice(&eax3.to_le_bytes());
    brand[36..40].copy_from_slice(&ebx3.to_le_bytes());
    brand[40..44].copy_from_slice(&ecx3.to_le_bytes());
    brand[44..48].copy_from_slice(&edx3.to_le_bytes());
    
    brand
}

/// Port I/O operations
pub mod port {
    use core::arch::asm;
    
    /// Read a byte from an I/O port
    pub fn read_u8(port: u16) -> u8 {
        let value: u8;
        unsafe {
            asm!("in al, dx", out("al") value, in("dx") port);
        }
        value
    }
    
    /// Write a byte to an I/O port
    pub fn write_u8(port: u16, value: u8) {
        unsafe {
            asm!("out dx, al", in("dx") port, in("al") value);
        }
    }
    
    /// Read a word from an I/O port
    pub fn read_u16(port: u16) -> u16 {
        let value: u16;
        unsafe {
            asm!("in ax, dx", out("ax") value, in("dx") port);
        }
        value
    }
    
    /// Write a word to an I/O port
    pub fn write_u16(port: u16, value: u16) {
        unsafe {
            asm!("out dx, ax", in("dx") port, in("ax") value);
        }
    }
    
    /// Read a dword from an I/O port
    pub fn read_u32(port: u16) -> u32 {
        let value: u32;
        unsafe {
            asm!("in eax, dx", out("eax") value, in("dx") port);
        }
        value
    }
    
    /// Write a dword to an I/O port
    pub fn write_u32(port: u16, value: u32) {
        unsafe {
            asm!("out dx, eax", in("dx") port, in("eax") value);
        }
    }
    
    /// Wait for an I/O operation to complete
    pub fn wait() {
        // Write to port 0x80 (unused port) as a delay
        write_u8(0x80, 0);
    }
}

/// GDT (Global Descriptor Table)
pub mod gdt {
    use core::mem::size_of;
    
    /// GDT entry
    #[derive(Debug, Clone, Copy)]
    #[repr(C, packed)]
    pub struct GdtEntry {
        pub limit_low: u16,
        pub base_low: u16,
        pub base_middle: u8,
        pub access: u8,
        pub granularity: u8,
        pub base_high: u8,
    }
    
    impl GdtEntry {
        /// Create a new GDT entry
        pub fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
            Self {
                limit_low: (limit & 0xFFFF) as u16,
                base_low: (base & 0xFFFF) as u16,
                base_middle: ((base >> 16) & 0xFF) as u8,
                access,
                granularity: granularity | ((limit >> 16) & 0x0F) as u8,
                base_high: ((base >> 24) & 0xFF) as u8,
            }
        }
    }
    
    /// GDT pointer
    #[derive(Debug, Clone, Copy)]
    #[repr(C, packed)]
    pub struct GdtPointer {
        pub limit: u16,
        pub base: u64,
    }
    
    /// Number of GDT entries
    pub const GDT_ENTRIES: usize = 7;
    
    /// Null selector
    pub const NULL_SELECTOR: u16 = 0x00;
    /// Kernel code selector
    pub const KERNEL_CODE_SELECTOR: u16 = 0x08;
    /// Kernel data selector
    pub const KERNEL_DATA_SELECTOR: u16 = 0x10;
    /// User code selector
    pub const USER_CODE_SELECTOR: u16 = 0x18;
    /// User data selector
    pub const USER_DATA_SELECTOR: u16 = 0x20;
    /// TSS selector
    pub const TSS_SELECTOR: u16 = 0x28;
    
    /// Access byte constants
    pub const ACCESS_PRESENT: u8 = 0x80;
    pub const ACCESS_RING0: u8 = 0x00;
    pub const ACCESS_RING3: u8 = 0x60;
    pub const ACCESS_CODE: u8 = 0x18;
    pub const ACCESS_DATA: u8 = 0x10;
    pub const ACCESS_EXEC: u8 = 0x08;
    pub const ACCESS_READ: u8 = 0x02;
    pub const ACCESS_WRITE: u8 = 0x02;
    
    /// Granularity constants
    pub const GRANULARITY_4K: u8 = 0x80;
    pub const GRANULARITY_32BIT: u8 = 0x40;
}

/// TSS (Task State Segment)
pub mod tss {
    use core::arch::asm;
    
    /// Task State Segment
    #[derive(Debug, Clone, Copy)]
    #[repr(C, packed)]
    pub struct TaskStateSegment {
        pub reserved1: u32,
        pub rsp: [u64; 3],
        pub reserved2: u64,
        pub ist: [u64; 7],
        pub reserved3: u64,
        pub reserved4: u16,
        pub iomap_base: u16,
    }
    
    impl TaskStateSegment {
        /// Create a new TSS
        pub const fn new() -> Self {
            Self {
                reserved1: 0,
                rsp: [0; 3],
                reserved2: 0,
                ist: [0; 7],
                reserved3: 0,
                reserved4: 0,
                iomap_base: size_of::<TaskStateSegment>() as u16,
            }
        }
    }
    
    use core::mem::size_of;
    
    /// Load TSS
    pub fn load_tss(selector: u16) {
        unsafe {
            asm!("ltr {0:x}", in(reg) selector);
        }
    }
}