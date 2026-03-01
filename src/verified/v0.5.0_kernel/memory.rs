// VantisOS v0.5.0 - Memory Management
// Day 9: Set Up Memory Management

#![allow(unused_unsafe)]

// Memory constants
const PAGE_SIZE: usize = 4096;
const KERNEL_HEAP_START: usize = 0x200000; // 2MB
const KERNEL_HEAP_SIZE: usize = 0x100000; // 1MB

// Memory region types
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum MemoryRegionType {
    Available = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNvs = 4,
    Unusable = 5,
}

// Memory region structure
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct MemoryRegion {
    pub base: u64,
    pub length: u64,
    pub region_type: u32,
    pub acpi_attrs: u32,
}

// Page allocator
pub struct PageAllocator {
    bitmap: *mut u8,
    total_pages: usize,
    used_pages: usize,
    start_addr: usize,
}

impl PageAllocator {
    pub fn new(start_addr: usize, total_pages: usize) -> PageAllocator {
        let bitmap_size = (total_pages + 7) / 8;
        let bitmap = start_addr as *mut u8;
        
        // Initialize bitmap to all zeros (all pages free)
        for i in 0..bitmap_size {
            unsafe {
                *bitmap.add(i) = 0;
            }
        }
        
        PageAllocator {
            bitmap,
            total_pages,
            used_pages: 0,
            start_addr,
        }
    }
    
    pub fn allocate(&mut self) -> Option<usize> {
        for page in 0..self.total_pages {
            let byte_index = page / 8;
            let bit_index = page % 8;
            
            unsafe {
                let byte = *self.bitmap.add(byte_index);
                if byte & (1 << bit_index) == 0 {
                    // Page is free, mark as used
                    *self.bitmap.add(byte_index) |= 1 << bit_index;
                    self.used_pages += 1;
                    return Some(self.start_addr + page * PAGE_SIZE);
                }
            }
        }
        
        None // No free pages
    }
    
    pub fn free(&mut self, addr: usize) {
        let page = (addr - self.start_addr) / PAGE_SIZE;
        
        if page < self.total_pages {
            let byte_index = page / 8;
            let bit_index = page % 8;
            
            unsafe {
                *self.bitmap.add(byte_index) &= !(1 << bit_index);
            }
            self.used_pages -= 1;
        }
    }
    
    pub fn free_pages(&self) -> usize {
        self.total_pages - self.used_pages
    }
    
    pub fn used_pages(&self) -> usize {
        self.used_pages
    }
}

// Simple heap allocator
pub struct HeapAllocator {
    start: usize,
    size: usize,
    used: usize,
}

impl HeapAllocator {
    pub fn new(start: usize, size: usize) -> HeapAllocator {
        HeapAllocator {
            start,
            size,
            used: 0,
        }
    }
    
    pub fn allocate(&mut self, size: usize, align: usize) -> Option<usize> {
        let aligned_size = (size + align - 1) & !(align - 1);
        
        if self.used + aligned_size > self.size {
            return None;
        }
        
        let addr = self.start + self.used;
        self.used += aligned_size;
        
        Some(addr)
    }
    
    pub fn free(&mut self, _addr: usize) {
        // Simple heap doesn't support deallocation
    }
    
    pub fn free_space(&self) -> usize {
        self.size - self.used
    }
    
    pub fn used_space(&self) -> usize {
        self.used
    }
}

// Memory manager
pub struct MemoryManager {
    page_allocator: Option<PageAllocator>,
    heap_allocator: Option<HeapAllocator>,
    total_memory: u64,
    available_memory: u64,
}

impl MemoryManager {
    pub fn new() -> MemoryManager {
        MemoryManager {
            page_allocator: None,
            heap_allocator: None,
            total_memory: 0,
            available_memory: 0,
        }
    }
    
    pub fn init(&mut self, memory_map: &[MemoryRegion]) {
        // Find largest available region for page allocator
        let mut largest_region: Option<MemoryRegion> = None;
        let mut largest_size = 0u64;
        
        for region in memory_map {
            if region.region_type == MemoryRegionType::Available as u32 {
                if region.length > largest_size {
                    largest_size = region.length;
                    largest_region = Some(*region);
                }
            }
        }
        
        if let Some(region) = largest_region {
            let start_addr = region.base as usize;
            let total_pages = (region.length as usize) / PAGE_SIZE;
            
            // Initialize page allocator after kernel heap
            let page_allocator_start = KERNEL_HEAP_START + KERNEL_HEAP_SIZE;
            let page_allocator_pages = (total_pages * PAGE_SIZE - (page_allocator_start - start_addr)) / PAGE_SIZE;
            
            self.page_allocator = Some(PageAllocator::new(page_allocator_start, page_allocator_pages));
            
            // Initialize heap allocator
            self.heap_allocator = Some(HeapAllocator::new(KERNEL_HEAP_START, KERNEL_HEAP_SIZE));
            
            self.total_memory = region.length;
            self.available_memory = region.length;
        }
    }
    
    pub fn allocate_page(&mut self) -> Option<usize> {
        if let Some(ref mut allocator) = self.page_allocator {
            allocator.allocate()
        } else {
            None
        }
    }
    
    pub fn free_page(&mut self, addr: usize) {
        if let Some(ref mut allocator) = self.page_allocator {
            allocator.free(addr);
        }
    }
    
    pub fn allocate_heap(&mut self, size: usize, align: usize) -> Option<usize> {
        if let Some(ref mut allocator) = self.heap_allocator {
            allocator.allocate(size, align)
        } else {
            None
        }
    }
    
    pub fn free_heap(&mut self, addr: usize) {
        if let Some(ref mut allocator) = self.heap_allocator {
            allocator.free(addr);
        }
    }
    
    pub fn get_stats(&self) -> MemoryStats {
        let free_pages = self.page_allocator.as_ref().map(|a| a.free_pages()).unwrap_or(0);
        let used_pages = self.page_allocator.as_ref().map(|a| a.used_pages()).unwrap_or(0);
        let heap_free = self.heap_allocator.as_ref().map(|a| a.free_space()).unwrap_or(0);
        let heap_used = self.heap_allocator.as_ref().map(|a| a.used_space()).unwrap_or(0);
        
        MemoryStats {
            total_memory: self.total_memory,
            available_memory: self.available_memory,
            free_pages,
            used_pages,
            heap_free,
            heap_used,
        }
    }
}

// Memory statistics
#[derive(Clone, Copy, Debug)]
pub struct MemoryStats {
    pub total_memory: u64,
    pub available_memory: u64,
    pub free_pages: usize,
    pub used_pages: usize,
    pub heap_free: usize,
    pub heap_used: usize,
}

// Global memory manager
static mut MEMORY_MANAGER: Option<MemoryManager> = None;

pub fn init(memory_map: &[MemoryRegion]) {
    unsafe {
        if MEMORY_MANAGER.is_none() {
            MEMORY_MANAGER = Some(MemoryManager::new());
        }
        MEMORY_MANAGER.as_mut().unwrap().init(memory_map);
    }
}

pub fn allocate_page() -> Option<usize> {
    unsafe {
        if let Some(ref mut manager) = MEMORY_MANAGER {
            manager.allocate_page()
        } else {
            None
        }
    }
}

pub fn free_page(addr: usize) {
    unsafe {
        if let Some(ref mut manager) = MEMORY_MANAGER {
            manager.free_page(addr);
        }
    }
}

pub fn allocate_heap(size: usize, align: usize) -> Option<usize> {
    unsafe {
        if let Some(ref mut manager) = MEMORY_MANAGER {
            manager.allocate_heap(size, align)
        } else {
            None
        }
    }
}

pub fn free_heap(addr: usize) {
    unsafe {
        if let Some(ref mut manager) = MEMORY_MANAGER {
            manager.free_heap(addr);
        }
    }
}

pub fn get_stats() -> MemoryStats {
    unsafe {
        if let Some(ref manager) = MEMORY_MANAGER {
            manager.get_stats()
        } else {
            MemoryStats {
                total_memory: 0,
                available_memory: 0,
                free_pages: 0,
                used_pages: 0,
                heap_free: 0,
                heap_used: 0,
            }
        }
    }
}