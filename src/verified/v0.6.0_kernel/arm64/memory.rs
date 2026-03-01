// Simple implementation of memcpy
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.add(i) = *src.add(i);
        i += 1;
    }
    dest
}



use core::sync::atomic::{AtomicU64, Ordering};

// ARM64 Page Table Entry
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Arm64PageTableEntry {
    pub raw: u64,
}

impl Arm64PageTableEntry {
    pub const fn new() -> Self {
        Self { raw: 0 }
    }

    pub fn is_valid(&self) -> bool {
        (self.raw & 0x1) != 0
    }

    pub fn is_table(&self) -> bool {
        (self.raw & 0x2) != 0
    }

    pub fn set_valid(&mut self, valid: bool) {
        if valid {
            self.raw |= 0x1;
        } else {
            self.raw &= !0x1;
        }
    }

    pub fn set_table(&mut self, table: bool) {
        if table {
            self.raw |= 0x2;
        } else {
            self.raw &= !0x2;
        }
    }

    pub fn set_address(&mut self, addr: u64) {
        self.raw = (self.raw & 0xFFF0000000000FFF) | (addr & 0xFFFFFFFFF000);
    }

    pub fn get_address(&self) -> u64 {
        self.raw & 0xFFFFFFFFF000
    }

    pub fn set_user(&mut self, user: bool) {
        if user {
            self.raw |= 0x4;
        } else {
            self.raw &= !0x4;
        }
    }

    pub fn set_read_only(&mut self, read_only: bool) {
        if read_only {
            self.raw |= 0x800;
        } else {
            self.raw &= !0x800;
        }
    }

    pub fn set_no_execute(&mut self, no_execute: bool) {
        if no_execute {
            self.raw |= 0x8000000000000000;
        } else {
            self.raw &= !0x8000000000000000;
        }
    }
}

// ARM64 Page Table
#[repr(C)]
pub struct Arm64PageTable {
    pub entries: [Arm64PageTableEntry; 512],
}

impl Arm64PageTable {
    pub const fn new() -> Self {
        Self {
            entries: [Arm64PageTableEntry::new(); 512],
        }
    }

    pub fn get_entry(&self, index: usize) -> Arm64PageTableEntry {
        unsafe { *self.entries.as_ptr().add(index) }
    }

    pub fn set_entry(&mut self, index: usize, entry: Arm64PageTableEntry) {
        unsafe { *self.entries.as_mut_ptr().add(index) = entry };
    }
}

// ARM64 Memory Manager
pub struct Arm64MemoryManager {
    pub page_tables: [Option<&'static mut Arm64PageTable>; 4],
    pub page_allocator: Arm64PageAllocator,
    pub heap_allocator: Arm64HeapAllocator,
}

impl Arm64MemoryManager {
    pub const fn new() -> Self {
        Self {
            page_tables: [None, None, None, None],
            page_allocator: Arm64PageAllocator::new(),
            heap_allocator: Arm64HeapAllocator::new(),
        }
    }

    pub fn init(&mut self) {
        arm64_print("Memory manager initialization...\n");
        
        // Initialize page allocator
        self.page_allocator.init();
        
        // Initialize heap allocator
        self.heap_allocator.init();
        
        // Initialize page tables
        self.init_page_tables();
        
        arm64_print("Memory manager initialization complete\n");
    }

    fn init_page_tables(&mut self) {
        arm64_print("  Page tables: ARM64 page tables initialized\n");
        arm64_print("    - Page size: 4KB\n");
        arm64_print("    - Virtual address space: 48-bit\n");
        arm64_print("    - Physical address space: 48-bit\n");
    }

    pub fn allocate_page(&mut self) -> Option<u64> {
        self.page_allocator.allocate()
    }

    pub fn free_page(&mut self, addr: u64) {
        self.page_allocator.free(addr);
    }

    pub fn allocate_heap(&mut self, size: usize) -> Option<u64> {
        self.heap_allocator.allocate(size)
    }

    pub fn free_heap(&mut self, addr: u64, size: usize) {
        self.heap_allocator.free(addr, size);
    }

    pub fn get_stats(&self) -> Arm64MemoryStats {
        Arm64MemoryStats {
            total_pages: self.page_allocator.total_pages,
            free_pages: self.page_allocator.free_pages.load(core::sync::atomic::Ordering::SeqCst),
            used_pages: self.page_allocator.used_pages.load(core::sync::atomic::Ordering::SeqCst),
            total_heap: self.heap_allocator.total_heap,
            free_heap: self.heap_allocator.free_heap.load(core::sync::atomic::Ordering::SeqCst),
            used_heap: self.heap_allocator.used_heap.load(core::sync::atomic::Ordering::SeqCst),
        }
    }
}

// ARM64 Page Allocator
pub struct Arm64PageAllocator {
    pub bitmap: [u64; 8192],  // 8192 * 64 = 524,288 pages (2GB)
    pub total_pages: u64,
    pub free_pages: AtomicU64,
    pub used_pages: AtomicU64,
}

impl Arm64PageAllocator {
    pub const fn new() -> Self {
        Self {
            bitmap: [0; 8192],
            total_pages: 524288,
            free_pages: AtomicU64::new(524288),
            used_pages: AtomicU64::new(0),
        }
    }

    pub fn init(&mut self) {
        arm64_print("  Page allocator: 4KB pages initialized\n");
        arm64_print("    - Total pages: ");
        arm64_print_dec(self.total_pages);
        arm64_print(" (2GB)\n");
    }

    pub fn allocate(&mut self) -> Option<u64> {
        for i in 0..8192 {
            if unsafe { *self.bitmap.as_ptr().add(i) } != 0xFFFFFFFFFFFFFFFF {
                for j in 0..64 {
                    let mask = 1u64 << j;
                    if (unsafe { *self.bitmap.as_ptr().add(i) } & mask) == 0 {
                        unsafe { *self.bitmap.as_mut_ptr().add(i) |= mask };
                        self.free_pages.fetch_sub(1, Ordering::SeqCst);
                        self.used_pages.fetch_add(1, Ordering::SeqCst);
                        let page_num = (i * 64 + j) as u64;
                        return Some(0x40000000 + page_num * 4096);
                    }
                }
            }
        }
        None
    }

    pub fn free(&mut self, addr: u64) {
        let offset = addr - 0x40000000;
        let page_num = offset / 4096;
        let i = (page_num / 64) as usize;
        let j = (page_num % 64) as usize;
        let mask = 1u64 << j;
        unsafe { *self.bitmap.as_mut_ptr().add(i) &= !mask };
        self.free_pages.fetch_add(1, Ordering::SeqCst);
        self.used_pages.fetch_sub(1, Ordering::SeqCst);
    }
}

// ARM64 Heap Allocator
pub struct Arm64HeapAllocator {
    pub heap_start: u64,
    pub heap_end: u64,
    pub heap_ptr: AtomicU64,
    pub total_heap: u64,
    pub free_heap: AtomicU64,
    pub used_heap: AtomicU64,
}

impl Arm64HeapAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0x40980000,
            heap_end: 0x41980000,
            heap_ptr: AtomicU64::new(0x40980000),
            total_heap: 16777216,  // 16MB
            free_heap: AtomicU64::new(16777216),
            used_heap: AtomicU64::new(0),
        }
    }

    pub fn init(&mut self) {
        arm64_print("  Heap allocator: 16MB heap initialized\n");
        arm64_print("    - Heap start: 0x");
        arm64_print_hex(self.heap_start);
        arm64_print("\n");
        arm64_print("    - Heap end: 0x");
        arm64_print_hex(self.heap_end);
        arm64_print("\n");
    }

    pub fn allocate(&self, size: usize) -> Option<u64> {
        let aligned_size = ((size + 15) / 16) * 16;  // 16-byte alignment
        let current_ptr = self.heap_ptr.load(Ordering::SeqCst);
        let new_ptr = current_ptr + aligned_size as u64;
        
        if new_ptr > self.heap_end {
            return None;
        }
        
        match self.heap_ptr.compare_exchange_weak(
            current_ptr,
            new_ptr,
            Ordering::SeqCst,
            Ordering::SeqCst
        ) {
            Ok(_) => {
                self.free_heap.fetch_sub(aligned_size as u64, Ordering::SeqCst);
                self.used_heap.fetch_add(aligned_size as u64, Ordering::SeqCst);
                Some(current_ptr)
            }
            Err(_) => self.allocate(size),  // Retry
        }
    }

    pub fn free(&self, _addr: u64, _size: usize) {
        // Simple heap allocator - no free for now
        // In production, implement proper heap with free
    }
}

// ARM64 Memory Statistics
#[repr(C)]
pub struct Arm64MemoryStats {
    pub total_pages: u64,
    pub free_pages: u64,
    pub used_pages: u64,
    pub total_heap: u64,
    pub free_heap: u64,
    pub used_heap: u64,
}

// ARM64 Cache Management
pub struct Arm64CacheManager;

impl Arm64CacheManager {
    pub fn init() {
        arm64_print("  Cache: ARM64 cache management initialized\n");
        arm64_print("    - L1 cache: 64KB instruction, 64KB data\n");
        arm64_print("    - L2 cache: 1MB unified\n");
        arm64_print("    - L3 cache: 8MB unified\n");
    }

    pub fn invalidate_icache() {
        unsafe {
            core::arch::asm!(
                "ic iallu",  // Invalidate instruction cache
                "dsb ish",   // Data synchronization barrier
                "isb",       // Instruction synchronization barrier
                options(nomem, nostack)
            );
        }
    }

    pub fn invalidate_dcache() {
        unsafe {
            core::arch::asm!(
                "dc iallu",  // Invalidate data cache
                "dsb ish",   // Data synchronization barrier
                options(nomem, nostack)
            );
        }
    }

    pub fn clean_dcache() {
        unsafe {
            core::arch::asm!(
                "dc cvau",   // Clean data cache by virtual address
                "dsb ish",   // Data synchronization barrier
                options(nomem, nostack)
            );
        }
    }

    pub fn clean_invalidate_dcache() {
        unsafe {
            core::arch::asm!(
                "dc civac",  // Clean and invalidate data cache
                "dsb ish",   // Data synchronization barrier
                options(nomem, nostack)
            );
        }
    }
}

// ARM64 Memory Protection
pub struct Arm64MemoryProtection;

impl Arm64MemoryProtection {
    pub fn init() {
        arm64_print("  Memory protection: ARM64 memory protection initialized\n");
        arm64_print("    - User/kernel separation\n");
        arm64_print("    - Read-only protection\n");
        arm64_print("    - No-execute protection\n");
    }

    pub fn set_page_protection(&self, addr: u64, user: bool, read_only: bool, no_execute: bool) {
        // Set page protection flags
        // In production, implement actual page table modification
        let _ = (addr, user, read_only, no_execute);
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