//! # Page Allocator
//!
//! This module provides page-based memory allocation for the minimal kernel.

use super::{PhysAddr, AllocError};

/// Page allocator
pub struct PageAllocator {
    /// Free pages bitmap
    free_pages: Vec<u8>,
    /// Total number of pages
    total_pages: usize,
    /// Page size
    page_size: usize,
}

impl PageAllocator {
    /// Initialize page allocator
    pub fn init() {
        // This is a placeholder - actual implementation would:
        // 1. Detect available physical memory
        // 2. Initialize free pages bitmap
        // 3. Mark kernel pages as used
        // 4. Mark reserved pages as used
    }
    
    /// Allocate a page
    ///
    /// Returns the physical address of the allocated page.
    pub fn alloc_page(&mut self) -> Result<PhysAddr, AllocError> {
        // Find first free page
        for (byte_idx, byte) in self.free_pages.iter().enumerate() {
            if *byte != 0xFF {
                // Find first free bit in this byte
                for bit_idx in 0..8 {
                    if byte & (1 << bit_idx) == 0 {
                        // Mark page as used
                        self.free_pages[byte_idx] |= 1 << bit_idx;
                        
                        // Calculate physical address
                        let page_num = byte_idx * 8 + bit_idx;
                        let phys_addr = page_num * self.page_size;
                        
                        return Ok(phys_addr);
                    }
                }
            }
        }
        
        Err(AllocError::OutOfMemory)
    }
    
    /// Free a page
    ///
    /// Frees the page at the given physical address.
    pub fn free_page(&mut self, addr: PhysAddr) -> Result<(), AllocError> {
        // Check alignment
        if addr % self.page_size != 0 {
            return Err(AllocError::AlignmentError);
        }
        
        // Calculate page number
        let page_num = addr / self.page_size;
        
        // Check if page is within range
        if page_num >= self.total_pages {
            return Err(AllocError::InvalidAddress);
        }
        
        // Calculate byte and bit indices
        let byte_idx = page_num / 8;
        let bit_idx = page_num % 8;
        
        // Mark page as free
        self.free_pages[byte_idx] &= !(1 << bit_idx);
        
        Ok(())
    }
    
    /// Get number of free pages
    pub fn free_pages_count(&self) -> usize {
        let mut count = 0;
        for byte in &self.free_pages {
            count += byte.count_zeros() as usize;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alloc_free_page() {
        let mut allocator = PageAllocator {
            free_pages: vec![0x00; 1],
            total_pages: 8,
            page_size: 4096,
        };
        
        // Allocate a page
        let addr = allocator.alloc_page().unwrap();
        assert_eq!(addr, 0);
        
        // Free the page
        allocator.free_page(addr).unwrap();
        
        // Allocate again - should get the same page
        let addr2 = allocator.alloc_page().unwrap();
        assert_eq!(addr2, 0);
    }
    
    #[test]
    fn test_out_of_memory() {
        let mut allocator = PageAllocator {
            free_pages: vec![0xFF; 1],
            total_pages: 8,
            page_size: 4096,
        };
        
        // Try to allocate when no pages are free
        let result = allocator.alloc_page();
        assert!(matches!(result, Err(AllocError::OutOfMemory)));
    }
    
    #[test]
    fn test_alignment_error() {
        let mut allocator = PageAllocator {
            free_pages: vec![0x00; 1],
            total_pages: 8,
            page_size: 4096,
        };
        
        // Try to free misaligned address
        let result = allocator.free_page(1234);
        assert!(matches!(result, Err(AllocError::AlignmentError)));
    }
}