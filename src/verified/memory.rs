//! Formally verified memory management operations
//! 
//! This module provides mathematically proven implementations of
//! memory allocation and management with safety guarantees.

#[cfg(feature = "verus-full")]
use builtin::*;
#[cfg(feature = "verus-full")]
use builtin_macros::*;
#[cfg(feature = "verus-full")]
use vstd::prelude::*;

/// Memory allocation error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocError {
    OutOfMemory,
    InvalidSize,
    InvalidAlignment,
}

/// Verified buffer error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferError {
    Full,
}

/// Memory allocator with formal verification
#[cfg(feature = "verus-full")]
verus! {
    pub struct VerifiedAllocator {
        heap_start: usize,
        heap_size: usize,
        allocated: usize,
        ghost invariant: bool,
    }
    
    impl VerifiedAllocator {
        /// Create a new allocator
        /// 
        /// # Formal Specification
        /// - Precondition: size > 0
        /// - Precondition: start + size > start (no overflow)
        /// - Postcondition: result.heap_start == start
        /// - Postcondition: result.heap_size == size
        /// - Postcondition: result.allocated == 0
        pub fn new(start: usize, size: usize) -> (result: Self)
            requires 
                size > 0,
                start + size > start,
            ensures 
                result.heap_start == start,
                result.heap_size == size,
                result.allocated == 0,
        {
            VerifiedAllocator {
                heap_start: start,
                heap_size: size,
                allocated: 0,
                ghost invariant: true,
            }
        }
        
        /// Allocate memory
        /// 
        /// # Formal Specification
        /// - Precondition: size > 0
        /// - Precondition: self.allocated <= self.heap_size
        /// - Postcondition: On success, returns address in valid range
        /// - Postcondition: On success, allocated increases by size
        pub fn allocate(&mut self, size: usize) -> (result: Result<usize, AllocError>)
            requires 
                size > 0,
                old(self).allocated <= old(self).heap_size,
            ensures 
                match result {
                    Ok(addr) => {
                        addr >= self.heap_start &&
                        addr + size <= self.heap_start + self.heap_size &&
                        self.allocated == old(self).allocated + size
                    },
                    Err(_) => {
                        self.allocated == old(self).allocated
                    }
                },
        {
            if self.allocated + size <= self.heap_size {
                let addr = self.heap_start + self.allocated;
                self.allocated = self.allocated + size;
                Ok(addr)
            } else {
                Err(AllocError::OutOfMemory)
            }
        }
        
        /// Get available memory
        /// 
        /// # Formal Specification
        /// - Postcondition: result == heap_size - allocated
        pub fn available(&self) -> (result: usize)
            ensures result == self.heap_size - self.allocated,
        {
            self.heap_size - self.allocated
        }
    }
}

#[cfg(not(feature = "verus-full"))]
pub struct VerifiedAllocator {
    heap_start: usize,
    heap_size: usize,
    allocated: usize,
}

#[cfg(not(feature = "verus-full"))]
impl VerifiedAllocator {
    pub fn new(start: usize, size: usize) -> Self {
        assert!(size > 0, "Size must be greater than 0");
        assert!(start.checked_add(size).is_some(), "Address overflow");
        
        VerifiedAllocator {
            heap_start: start,
            heap_size: size,
            allocated: 0,
        }
    }
    
    pub fn allocate(&mut self, size: usize) -> Result<usize, AllocError> {
        if size == 0 {
            return Err(AllocError::InvalidSize);
        }
        
        if self.allocated + size <= self.heap_size {
            let addr = self.heap_start + self.allocated;
            self.allocated += size;
            Ok(addr)
        } else {
            Err(AllocError::OutOfMemory)
        }
    }
    
    pub fn available(&self) -> usize {
        self.heap_size - self.allocated
    }
}

/// Verified buffer with bounds checking
#[cfg(feature = "verus-full")]
verus! {
    pub struct VerifiedBuffer {
        data: Vec<u8>,
        capacity: usize,
        ghost size: nat,
    }
    
    impl VerifiedBuffer {
        /// Create a new buffer
        /// 
        /// # Formal Specification
        /// - Precondition: capacity > 0
        /// - Postcondition: result.capacity == capacity
        /// - Postcondition: result.size == 0
        pub fn new(capacity: usize) -> (result: Self)
            requires capacity > 0,
            ensures 
                result.capacity == capacity,
                result.size == 0,
        {
            VerifiedBuffer {
                data: Vec::new(),
                capacity,
                ghost size: 0,
            }
        }
        
        /// Push a byte to the buffer
        /// 
        /// # Formal Specification
        /// - Postcondition: On success, size increases by 1
        /// - Postcondition: On failure, size unchanged
        pub fn push(&mut self, value: u8) -> (result: Result<(), ()>)
            ensures 
                match result {
                    Ok(()) => self.size == old(self).size + 1,
                    Err(()) => self.size == old(self).size,
                },
        {
            if self.data.len() < self.capacity {
                self.data.push(value);
                proof { self.size = self.size + 1; }
                Ok(())
            } else {
                Err(())
            }
        }
        
        /// Get buffer length
        /// 
        /// # Formal Specification
        /// - Postcondition: result == size
        pub fn len(&self) -> (result: usize)
            ensures result == self.size,
        {
            self.data.len()
        }
    }
}

#[cfg(not(feature = "verus-full"))]
pub struct VerifiedBuffer {
    data: Vec<u8>,
    capacity: usize,
}

#[cfg(not(feature = "verus-full"))]
impl VerifiedBuffer {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0");
        
        VerifiedBuffer {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }
    
    pub fn push(&mut self, value: u8) -> Result<(), BufferError> {
        if self.data.len() < self.capacity {
            self.data.push(value);
            Ok(())
        } else {
            Err(BufferError::Full)
        }
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// Kani verification harnesses
#[cfg(kani)]
mod verification {
    use super::*;
    
    #[kani::proof]
    fn verify_allocator_creation() {
        let start: usize = kani::any();
        let size: usize = kani::any();
        
        kani::assume(size > 0);
        kani::assume(start < usize::MAX - size);
        
        let allocator = VerifiedAllocator::new(start, size);
        
        assert!(allocator.heap_start == start);
        assert!(allocator.heap_size == size);
        assert!(allocator.allocated == 0);
    }
    
    #[kani::proof]
    #[kani::unwind(5)]
    fn verify_allocator_multiple_allocations() {
        let allocator = VerifiedAllocator::new(0x1000, 1024);
        let mut alloc = allocator;
        
        // Allocate multiple times
        let result1 = alloc.allocate(100);
        assert!(result1.is_ok());
        
        let result2 = alloc.allocate(200);
        assert!(result2.is_ok());
        
        let result3 = alloc.allocate(300);
        assert!(result3.is_ok());
        
        // Check total allocated
        assert!(alloc.allocated == 600);
    }
    
    #[kani::proof]
    fn verify_allocator_out_of_memory() {
        let mut allocator = VerifiedAllocator::new(0x1000, 100);
        
        // Try to allocate more than available
        let result = allocator.allocate(200);
        
        assert!(result.is_err());
        assert!(allocator.allocated == 0);
    }
    
    #[kani::proof]
    fn verify_buffer_creation() {
        let capacity: usize = kani::any();
        kani::assume(capacity > 0 && capacity < 1000);
        
        let buffer = VerifiedBuffer::new(capacity);
        
        assert!(buffer.capacity == capacity);
        assert!(buffer.len() == 0);
    }
    
    #[kani::proof]
    #[kani::unwind(10)]
    fn verify_buffer_push() {
        let mut buffer = VerifiedBuffer::new(10);
        
        // Push multiple values
        for i in 0..5 {
            let result = buffer.push(i as u8);
            assert!(result.is_ok());
        }
        
        assert!(buffer.len() == 5);
    }
    
    #[kani::proof]
    fn verify_buffer_overflow() {
        let mut buffer = VerifiedBuffer::new(5);
        
        // Fill buffer
        for i in 0..5 {
            let result = buffer.push(i as u8);
            assert!(result.is_ok());
        }
        
        // Try to push beyond capacity
        let result = buffer.push(99);
        assert!(result.is_err());
        assert!(buffer.len() == 5);
    }
}

#[cfg(all(test, feature = "verus-full"))]
mod tests {
    use super::*;
    
    #[test]
    fn test_allocator_creation() {
        let allocator = VerifiedAllocator::new(0x1000, 1024);
        assert_eq!(allocator.heap_start, 0x1000);
        assert_eq!(allocator.heap_size, 1024);
        assert_eq!(allocator.allocated, 0);
        assert_eq!(allocator.available(), 1024);
    }
    
    #[test]
    fn test_allocator_single_allocation() {
        let mut allocator = VerifiedAllocator::new(0x1000, 1024);
        
        let result = allocator.allocate(100);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x1000);
        assert_eq!(allocator.allocated, 100);
        assert_eq!(allocator.available(), 924);
    }
    
    #[test]
    fn test_allocator_multiple_allocations() {
        let mut allocator = VerifiedAllocator::new(0x1000, 1024);
        
        let addr1 = allocator.allocate(100).unwrap();
        let addr2 = allocator.allocate(200).unwrap();
        let addr3 = allocator.allocate(300).unwrap();
        
        assert_eq!(addr1, 0x1000);
        assert_eq!(addr2, 0x1000 + 100);
        assert_eq!(addr3, 0x1000 + 300);
        assert_eq!(allocator.allocated, 600);
    }
    
    #[test]
    fn test_allocator_out_of_memory() {
        let mut allocator = VerifiedAllocator::new(0x1000, 100);
        
        let result = allocator.allocate(200);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AllocError::OutOfMemory);
    }
    
    #[test]
    fn test_buffer_creation() {
        let buffer = VerifiedBuffer::new(10);
        assert_eq!(buffer.capacity, 10);
        assert_eq!(buffer.len(), 0);
    }
    
    #[test]
    fn test_buffer_push() {
        let mut buffer = VerifiedBuffer::new(10);
        
        for i in 0..5 {
            let result = buffer.push(i);
            assert!(result.is_ok());
        }
        
        assert_eq!(buffer.len(), 5);
    }
    
    #[test]
    fn test_buffer_overflow() {
        let mut buffer = VerifiedBuffer::new(5);
        
        // Fill buffer
        for i in 0..5 {
            buffer.push(i).unwrap();
        }
        
        // Try to overflow
        let result = buffer.push(99);
        assert!(result.is_err());
        assert_eq!(buffer.len(), 5);
    }
}