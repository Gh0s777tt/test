//! RISC-V Memory Management Unit
//! 
//! This module implements RISC-V MMU support including:
//! - Page table setup
//! - Virtual memory mapping
//! - Memory protection
//! - TLB management

#![no_std]

use core::ptr;

/// Page size (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Page shift (log2 of PAGE_SIZE)
pub const PAGE_SHIFT: usize = 12;

/// Virtual address bits (Sv39)
pub const VA_BITS: usize = 39;

/// Physical address bits
pub const PA_BITS: usize = 56;

/// Page table entry bits
pub const PTE_BITS: usize = 64;

/// Page table levels (Sv39)
pub const PT_LEVELS: usize = 3;

/// Page table entry flags
pub mod pte_flags {
    /// Valid bit
    pub const V: u64 = 1 << 0;
    
    /// Read bit
    pub const R: u64 = 1 << 1;
    
    /// Write bit
    pub const W: u64 = 1 << 2;
    
    /// Execute bit
    pub const X: u64 = 1 << 3;
    
    /// User bit
    pub const U: u64 = 1 << 4;
    
    /// Global bit
    pub const G: u64 = 1 << 5;
    
    /// Accessed bit
    pub const A: u64 = 1 << 6;
    
    /// Dirty bit
    pub const D: u64 = 1 << 7;
    
    /// Read/write/execute permissions
    pub const RWX: u64 = Self::R | Self::W | Self::X;
    
    /// User accessible
    pub const USER: u64 = Self::U;
    
    /// Kernel only
    pub const KERNEL: u64 = 0;
}

/// Page table entry
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    /// Create a new page table entry
    pub fn new(ppn: u64, flags: u64) -> Self {
        PageTableEntry {
            entry: (ppn << 10) | flags,
        }
    }
    
    /// Check if entry is valid
    pub fn is_valid(&self) -> bool {
        self.entry & pte_flags::V != 0
    }
    
    /// Get physical page number
    pub fn ppn(&self) -> u64 {
        self.entry >> 10
    }
    
    /// Get flags
    pub fn flags(&self) -> u64 {
        self.entry & 0x3FF
    }
    
    /// Set flags
    pub fn set_flags(&mut self, flags: u64) {
        self.entry = (self.entry & !0x3FF) | flags;
    }
    
    /// Get physical address
    pub fn pa(&self) -> u64 {
        self.ppn() << PAGE_SHIFT
    }
}

/// Page table
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    /// Create a new zeroed page table
    pub const fn new() -> Self {
        PageTable {
            entries: [PageTableEntry { entry: 0 }; 512],
        }
    }
    
    /// Get entry at index
    pub fn get(&self, index: usize) -> PageTableEntry {
        self.entries[index]
    }
    
    /// Set entry at index
    pub fn set(&mut self, index: usize, entry: PageTableEntry) {
        self.entries[index] = entry;
    }
    
    /// Clear all entries
    pub fn clear(&mut self) {
        for entry in &mut self.entries {
            entry.entry = 0;
        }
    }
}

/// Memory region
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

impl MemoryRegion {
    /// Create a new memory region
    pub fn new(start: usize, end: usize, flags: u64) -> Self {
        MemoryRegion { start, end, flags }
    }
    
    /// Check if region is valid
    pub fn is_valid(&self) -> bool {
        self.start < self.end && self.start % PAGE_SIZE == 0 && self.end % PAGE_SIZE == 0
    }
}

/// MMU state
pub struct MMU {
    root_page_table: *mut PageTable,
    kernel_regions: [MemoryRegion; 16],
    user_regions: [MemoryRegion; 16],
    kernel_region_count: usize,
    user_region_count: usize,
}

impl MMU {
    /// Create a new MMU instance
    pub fn new() -> Self {
        MMU {
            root_page_table: ptr::null_mut(),
            kernel_regions: [MemoryRegion::new(0, 0, 0); 16],
            user_regions: [MemoryRegion::new(0, 0, 0); 16],
            kernel_region_count: 0,
            user_region_count: 0,
        }
    }
    
    /// Initialize MMU
    pub fn init(&mut self) {
        // Allocate root page table
        self.root_page_table = self.alloc_page_table();
        
        // Map kernel regions
        self.map_kernel_regions();
        
        // Enable MMU
        self.enable_mmu();
    }
    
    /// Allocate a page table
    fn alloc_page_table(&self) -> *mut PageTable {
        // For now, use a static allocation
        // In production, this should use the memory allocator
        extern "C" {
            static mut __riscv_root_page_table: PageTable;
        }
        
        unsafe {
            &mut __riscv_root_page_table as *mut PageTable
        }
    }
    
    /// Map kernel regions
    fn map_kernel_regions(&mut self) {
        // Kernel code region (0x80000000 - 0x80200000)
        self.map_region(
            0x80000000,
            0x80000000,
            0x80200000,
            pte_flags::V | pte_flags::R | pte_flags::X,
        );
        
        // Kernel data region (0x80200000 - 0x80400000)
        self.map_region(
            0x80200000,
            0x80200000,
            0x80400000,
            pte_flags::V | pte_flags::R | pte_flags::W,
        );
        
        // Kernel heap region (0x80400000 - 0x84000000)
        self.map_region(
            0x80400000,
            0x80400000,
            0x84000000,
            pte_flags::V | pte_flags::R | pte_flags::W,
        );
        
        // Device memory (0x10000000 - 0x12000000)
        self.map_region(
            0x10000000,
            0x10000000,
            0x12000000,
            pte_flags::V | pte_flags::R | pte_flags::W,
        );
    }
    
    /// Map a memory region
    fn map_region(&mut self, va: usize, pa: usize, end: usize, flags: u64) {
        let mut current_va = va;
        let mut current_pa = pa;
        
        while current_va < end {
            self.map_page(current_va, current_pa, flags);
            current_va += PAGE_SIZE;
            current_pa += PAGE_SIZE;
        }
    }
    
    /// Map a single page
    fn map_page(&mut self, va: usize, pa: usize, flags: u64) {
        let vpn = [va >> 30, (va >> 21) & 0x1FF, (va >> 12) & 0x1FF];
        let ppn = pa >> PAGE_SHIFT;
        
        unsafe {
            let mut pt = self.root_page_table;
            
            // Walk page table levels
            for level in 0..PT_LEVELS {
                let index = vpn[level];
                let entry = (*pt).get(index);
                
                if level == PT_LEVELS - 1 {
                    // Last level - create leaf entry
                    let pte = PageTableEntry::new(ppn, flags);
                    (*pt).set(index, pte);
                } else {
                    // Intermediate level - create or follow entry
                    if !entry.is_valid() {
                        // Allocate new page table
                        let new_pt = self.alloc_page_table();
                        let pte = PageTableEntry::new(
                            (new_pt as usize) >> PAGE_SHIFT,
                            pte_flags::V | pte_flags::W,
                        );
                        (*pt).set(index, pte);
                        pt = new_pt;
                    } else {
                        pt = (entry.pa()) as *mut PageTable;
                    }
                }
            }
        }
    }
    
    /// Enable MMU
    fn enable_mmu(&self) {
        unsafe {
            // Set SATP register
            let satp = (8usize << 60) | ((self.root_page_table as usize) >> PAGE_SHIFT);
            
            asm!(
                "csrw satp, {}",
                in(reg) satp,
            );
            
            // Flush TLB
            asm!("sfence.vma");
        }
    }
    
    /// Invalidate TLB entry
    pub fn tlb_invalidate(&self, va: usize) {
        unsafe {
            asm!(
                "sfence.vma {}, x0",
                in(reg) va,
            );
        }
    }
    
    /// Invalidate all TLB entries
    pub fn tlb_flush_all(&self) {
        unsafe {
            asm!("sfence.vma");
        }
    }
}

/// Initialize RISC-V MMU
#[no_mangle]
pub extern "C" fn riscv_mmu_init() {
    static mut MMU_INSTANCE: Option<MMU> = None;
    
    unsafe {
        if MMU_INSTANCE.is_none() {
            let mut mmu = MMU::new();
            mmu.init();
            MMU_INSTANCE = Some(mmu);
        }
    }
}

/// Static root page table allocation
#[link_section = ".bss"]
#[no_mangle]
static mut __riscv_root_page_table: PageTable = PageTable::new();

/// Additional page tables for intermediate levels
#[link_section = ".bss"]
#[no_mangle]
static mut __riscv_page_tables: [PageTable; 16] = [PageTable::new(); 16];