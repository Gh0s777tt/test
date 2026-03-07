//! Memory management for VantisOS
//! 
//! This module provides:
//! - Physical frame allocation
//! - Virtual memory management with 4-level paging
//! - Heap allocation
//! - Memory mapping utilities

use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, 
    PhysFrame, Size4KiB, Translate
};
use x86_64::{PhysAddr, VirtAddr};
use spin::Mutex;
use alloc::alloc::{alloc, dealloc, Layout};
use core::sync::atomic::{AtomicU64, Ordering};

use crate::serial_println;

extern crate alloc;

/// Physical memory offset for virtual memory mapping
pub const PHYS_MEM_OFFSET: u64 = 0xFFFF800000000000;

/// Kernel heap start address (2 MB)
pub const HEAP_START: u64 = 0xFFFF800000200000;

/// Kernel heap size (16 MB)
pub const HEAP_SIZE: usize = 16 * 1024 * 1024;

/// Page size (4 KB)
pub const PAGE_SIZE: usize = 4096;

/// Total physical memory (detected at runtime)
pub static TOTAL_MEMORY: AtomicU64 = AtomicU64::new(0);

/// Available physical memory
pub static AVAILABLE_MEMORY: AtomicU64 = AtomicU64::new(0);

/// Global frame allocator
pub static FRAME_ALLOCATOR: Mutex<Option<BootFrameAllocator>> = Mutex::new(None);

/// Global page table mapper
pub static MAPPER: Mutex<Option<OffsetPageTable<'static>>> = Mutex::new(None);

/// Initialize memory management
pub fn init() {
    // Get total memory from boot info
    let total_mem = unsafe { crate::BOOT_INFO.total_memory };
    TOTAL_MEMORY.store(total_mem, Ordering::SeqCst);
    
    serial_println!("[MEM] Total memory: {} MB", total_mem / 1024 / 1024);
    
    // Initialize frame allocator
    init_frame_allocator();
    
    // Initialize paging
    init_paging();
    
    serial_println!("[OK] Memory management initialized");
}

/// Initialize the frame allocator
fn init_frame_allocator() {
    let total_mem = TOTAL_MEMORY.load(Ordering::SeqCst);
    let frame_count = (total_mem / 4096) as usize;
    
    let allocator = BootFrameAllocator::new(frame_count);
    *FRAME_ALLOCATOR.lock() = Some(allocator);
    
    serial_println!("[MEM] Frame allocator initialized: {} frames", frame_count);
}

/// Initialize paging and create new page tables
fn init_paging() {
    // We'll use the bootloader-provided page tables initially
    // In a more advanced implementation, we'd create our own
    
    serial_println!("[MEM] Using bootloader page tables");
}

/// Boot frame allocator
/// 
/// A simple bump allocator that tracks used frames using a bitmap
pub struct BootFrameAllocator {
    /// Total number of frames
    total_frames: usize,
    /// Next frame to allocate
    next_frame: usize,
    /// Bitmap of used frames (1 = used, 0 = free)
    bitmap: &'static mut [u8],
}

impl BootFrameAllocator {
    /// Create a new frame allocator
    pub fn new(total_frames: usize) -> Self {
        // Calculate bitmap size (1 bit per frame, rounded up to bytes)
        let bitmap_size = (total_frames + 7) / 8;
        
        // Place bitmap at a fixed location after kernel
        // In a real OS, this would be dynamically allocated
        let bitmap_start = 0x400000usize; // 4 MB mark
        let bitmap = unsafe {
            core::slice::from_raw_parts_mut(
                bitmap_start as *mut u8,
                bitmap_size
            )
        };
        
        // Mark first frames as used (kernel + bitmap)
        let used_frames = (0x400000 + bitmap_size) / 4096 + 1;
        for i in 0..used_frames.min(total_frames) {
            let byte_idx = i / 8;
            let bit_idx = i % 8;
            bitmap[byte_idx] |= 1 << bit_idx;
        }
        
        BootFrameAllocator {
            total_frames,
            next_frame: used_frames,
            bitmap,
        }
    }
    
    /// Allocate a single frame
    pub fn allocate_frame(&mut self) -> Option<PhysFrame> {
        if self.next_frame >= self.total_frames {
            return None;
        }
        
        // Find next free frame
        while self.next_frame < self.total_frames {
            let byte_idx = self.next_frame / 8;
            let bit_idx = self.next_frame % 8;
            
            if self.bitmap[byte_idx] & (1 << bit_idx) == 0 {
                // Mark as used
                self.bitmap[byte_idx] |= 1 << bit_idx;
                
                let frame = PhysFrame::containing_address(
                    PhysAddr::new((self.next_frame as u64) * 4096)
                );
                self.next_frame += 1;
                return Some(frame);
            }
            self.next_frame += 1;
        }
        
        None
    }
    
    /// Deallocate a frame
    pub fn deallocate_frame(&mut self, frame: PhysFrame) {
        let frame_idx = frame.start_address().as_u64() / 4096;
        if frame_idx < self.total_frames as u64 {
            let byte_idx = (frame_idx as usize) / 8;
            let bit_idx = (frame_idx as usize) % 8;
            self.bitmap[byte_idx] &= !(1 << bit_idx);
            
            // Update next_frame if this is earlier
            if (frame_idx as usize) < self.next_frame {
                self.next_frame = frame_idx as usize;
            }
        }
    }
    
    /// Get number of free frames
    pub fn free_frames(&self) -> usize {
        let mut count = 0;
        for i in 0..self.total_frames {
            let byte_idx = i / 8;
            let bit_idx = i % 8;
            if self.bitmap[byte_idx] & (1 << bit_idx) == 0 {
                count += 1;
            }
        }
        count
    }
    
    /// Get total frames
    pub fn total_frames(&self) -> usize {
        self.total_frames
    }
}

/// Implement the FrameAllocator trait for x86_64 crate
unsafe impl FrameAllocator<Size4KiB> for BootFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.allocate_frame()
    }
}

/// Page table management
pub struct PageTableManager {
    pml4: &'static mut PageTable,
    physical_offset: VirtAddr,
}

impl PageTableManager {
    /// Create a new page table manager from existing PML4
    pub unsafe fn new(pml4_addr: PhysAddr, physical_offset: VirtAddr) -> Self {
        let pml4 = &mut *(pml4_addr.as_u64() as *mut PageTable);
        PageTableManager {
            pml4,
            physical_offset,
        }
    }
    
    /// Get the physical address of the PML4
    pub fn pml4_physical(&self) -> PhysAddr {
        // This would be calculated based on where the PML4 is
        PhysAddr::new(self.pml4 as *const _ as u64 - self.physical_offset.as_u64())
    }
}

/// Map a virtual address to a physical frame
pub fn map_page(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    flags: PageTableFlags,
) -> Result<(), &'static str> {
    let mut mapper_lock = MAPPER.lock();
    let mapper = mapper_lock.as_mut().ok_or("Mapper not initialized")?;
    
    let frame: PhysFrame<Size4KiB> = PhysFrame::containing_address(physical_addr);
    let page: Page<Size4KiB> = Page::containing_address(virtual_addr);
    
    unsafe {
        mapper.map_to(page, frame, flags, &mut *FRAME_ALLOCATOR.lock().as_mut().unwrap())
            .map_err(|_| "Failed to map page")?
            .flush();
    }
    
    Ok(())
}

/// Allocate and map a new page
pub fn allocate_page(
    virtual_addr: VirtAddr,
    flags: PageTableFlags,
) -> Result<PhysAddr, &'static str> {
    let mut frame_alloc = FRAME_ALLOCATOR.lock();
    let frame_allocator = frame_alloc.as_mut().ok_or("Frame allocator not initialized")?;
    
    let frame = frame_allocator.allocate_frame()
        .ok_or("Out of memory")?;
    
    let mut mapper_lock = MAPPER.lock();
    let mapper = mapper_lock.as_mut().ok_or("Mapper not initialized")?;
    
    let page = Page::containing_address(virtual_addr);
    
    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
            .map_err(|_| "Failed to map page")?
            .flush();
    }
    
    Ok(frame.start_address())
}

/// Unmap a page
pub fn unmap_page(virtual_addr: VirtAddr) -> Result<PhysFrame, &'static str> {
    let mut mapper_lock = MAPPER.lock();
    let mapper = mapper_lock.as_mut().ok_or("Mapper not initialized")?;
    
    let page = Page::containing_address(virtual_addr);
    
    let (frame, flush) = mapper.unmap(page)
        .map_err(|_| "Failed to unmap page")?;
    
    flush.flush();
    
    Ok(frame)
}

/// Translate virtual address to physical
pub fn translate_addr(virtual_addr: VirtAddr) -> Option<PhysAddr> {
    let mapper_lock = MAPPER.lock();
    let mapper = mapper_lock.as_ref()?;
    
    mapper.translate_addr(virtual_addr)
}

/// Get memory statistics
pub fn get_memory_stats() -> MemoryStats {
    MemoryStats {
        total: TOTAL_MEMORY.load(Ordering::SeqCst),
        available: AVAILABLE_MEMORY.load(Ordering::SeqCst),
        used: TOTAL_MEMORY.load(Ordering::SeqCst) - AVAILABLE_MEMORY.load(Ordering::SeqCst),
    }
}

/// Memory statistics structure
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total: u64,
    pub available: u64,
    pub used: u64,
}

/// Memory region types (from BIOS/UEFI)
#[derive(Debug, Clone, Copy)]
pub enum MemoryRegionType {
    /// Usable RAM
    Usable,
    /// Reserved
    Reserved,
    /// ACPI Reclaimable
    AcpiReclaimable,
    /// ACPI NVS
    AcpiNvs,
    /// Unusable
    Unusable,
    /// Unknown
    Unknown,
}

/// Memory region descriptor
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub region_type: MemoryRegionType,
}

/// Heap allocator wrapper for debugging
pub struct HeapAllocator;

impl HeapAllocator {
    /// Print heap statistics
    pub fn print_stats() {
        serial_println!("[HEAP] Start: {:#x}", HEAP_START);
        serial_println!("[HEAP] Size: {} MB", HEAP_SIZE / 1024 / 1024);
    }
}