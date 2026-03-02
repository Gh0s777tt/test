//! # NUMA Allocation Module
//! 
//! Implementuje alokator pamięci NUMA z różnymi flagami i strategiami.

use core::sync::atomic::{AtomicUsize, Ordering};

/// Flagi alokacji NUMA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationFlags {
    /// Alokuj w lokalnym węźle
    pub local: bool,
    /// Alokuj w konkretnym węźle
    pub bind: bool,
    /// Alokuj z polityką interleave
    pub interleave: bool,
    /// Alokuj w preferowanym węźle
    pub preferred: bool,
    /// Zeruj pamięć
    pub zero: bool,
    /// Nie blokuj
    pub non_blocking: bool,
}

impl AllocationFlags {
    /// Tworzy domyślne flagi
    pub const fn default() -> Self {
        Self {
            local: true,
            bind: false,
            interleave: false,
            preferred: false,
            zero: false,
            non_blocking: false,
        }
    }
}

/// Aloktor pamięci NUMA
pub struct NumaAllocator {
    /// Całkowita zaalokowana pamięć
    total_allocated: AtomicUsize,
    /// Liczba alokacji
    allocation_count: AtomicUsize,
}

impl NumaAllocator {
    /// Tworzy nowy alokator NUMA
    pub const fn new() -> Self {
        Self {
            total_allocated: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
        }
    }
    
    /// Alokuj pamięć z flagami
    pub fn allocate(&self, size: usize, flags: AllocationFlags) -> Result<super::memory::NumaMemory, super::NumaError> {
        let memory = if flags.local {
            super::memory::alloc_local(size)?
        } else if flags.bind {
            // Placeholder - potrzebujemy ID węzła
            super::memory::alloc_local(size)?
        } else if flags.interleave {
            super::memory::alloc_interleave(size)?
        } else if flags.preferred {
            // Placeholder - potrzebujemy preferowanego węzła
            super::memory::alloc_local(size)?
        } else {
            super::memory::alloc_local(size)?
        };
        
        // Zeruj pamięć jeśli wymagane
        if flags.zero {
            self.zero_memory(&memory)?;
        }
        
        // Aktualizuj statystyki
        self.total_allocated.fetch_add(size, Ordering::Release);
        self.allocation_count.fetch_add(1, Ordering::Release);
        
        Ok(memory)
    }
    
    /// Zwalnia pamięć
    pub fn deallocate(&self, memory: super::memory::NumaMemory) -> Result<(), super::NumaError> {
        // Aktualizuj statystyki
        self.total_allocated.fetch_sub(memory.size, Ordering::Release);
        self.allocation_count.fetch_sub(1, Ordering::Release);
        
        // Zwolnij pamięć
        super::memory::free(memory)
    }
    
    /// Zeruje pamięć
    fn zero_memory(&self, memory: &super::memory::NumaMemory) -> Result<(), super::NumaError> {
        // Placeholder - zerowanie pamięci
        let _ = memory;
        Ok(())
    }
    
    /// Zwraca statystyki alokacji
    pub fn stats(&self) -> AllocationStats {
        AllocationStats {
            total_allocated: self.total_allocated.load(Ordering::Acquire),
            allocation_count: self.allocation_count.load(Ordering::Acquire),
        }
    }
}

/// Statystyki alokacji
#[derive(Debug, Clone)]
pub struct AllocationStats {
    /// Całkowita zaalokowana pamięć
    pub total_allocated: usize,
    /// Liczba alokacji
    pub allocation_count: usize,
}

/// Globalny alokator NUMA
static NUMA_ALLOCATOR: NumaAllocator = NumaAllocator::new();

/// Inicjalizuje alokator NUMA
pub fn init() -> Result<(), super::NumaError> {
    // Placeholder - inicjalizacja alokatora
    Ok(())
}

/// Alokuj pamięć NUMA
pub fn numa_alloc(size: usize, flags: AllocationFlags) -> Result<super::memory::NumaMemory, super::NumaError> {
    NUMA_ALLOCATOR.allocate(size, flags)
}

/// Zwalnij pamięć NUMA
pub fn numa_free(memory: super::memory::NumaMemory) -> Result<(), super::NumaError> {
    NUMA_ALLOCATOR.deallocate(memory)
}

/// Zwraca statystyki alokacji
pub fn allocation_stats() -> AllocationStats {
    NUMA_ALLOCATOR.stats()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_allocation_flags_default() {
        let flags = AllocationFlags::default();
        assert!(flags.local);
        assert!(!flags.bind);
        assert!(!flags.interleave);
    }
    
    #[test]
    fn test_numa_allocator() {
        let allocator = NumaAllocator::new();
        let flags = AllocationFlags::default();
        
        let memory = allocator.allocate(4096, flags).unwrap();
        assert_eq!(memory.size, 4096);
        
        allocator.deallocate(memory).unwrap();
    }
}