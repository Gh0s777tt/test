//! # Barrier Synchronization Module
//! 
//! Implementuje bariery synchronizacyjne dla koordynacji między CPU.

use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

/// Bariery synchronizacyjne
pub struct Barrier {
    /// Liczba CPU, które muszą dotrzeć do bariery
    count: AtomicUsize,
    /// Liczba CPU, które dotarły do bariery
    arrived: AtomicU32,
    /// Liczba cykli bariery
    generation: AtomicU32,
}

impl Barrier {
    /// Tworzy nową barierę
    pub const fn new(count: usize) -> Self {
        Self {
            count: AtomicUsize::new(count),
            arrived: AtomicU32::new(0),
            generation: AtomicU32::new(0),
        }
    }
    
    /// Czeka aż wszystkie CPU dotrą do bariery
    pub fn wait(&self) {
        let current_gen = self.generation.load(Ordering::Acquire);
        
        // Zwiększ licznik dotarłych CPU
        let arrived = self.arrived.fetch_add(1, Ordering::AcqRel) + 1;
        
        let count = self.count.load(Ordering::Acquire);
        
        if arrived as usize == count {
            // Ostatnie CPU dotarło - resetuj barierę
            self.arrived.store(0, Ordering::Release);
            self.generation.fetch_add(1, Ordering::AcqRel);
        } else {
            // Czekaj aż inne CPU dotrą
            while self.generation.load(Ordering::Acquire) == current_gen {
                core::hint::spin_loop();
            }
        }
    }
}

/// Inicjalizuje system bariery
pub fn init() -> Result<(), crate::verified::smp::SmpError> {
    // Placeholder - inicjalizacja systemu bariery
    Ok(())
}

/// Funkcja pomocnicza do czekania na barierę
pub fn barrier_wait(barrier: &Barrier) {
    barrier.wait();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_barrier_creation() {
        let barrier = Barrier::new(4);
        assert_eq!(barrier.count.load(Ordering::Acquire), 4);
        assert_eq!(barrier.arrived.load(Ordering::Acquire), 0);
    }
    
    #[test]
    fn test_barrier_single_cpu() {
        let barrier = Barrier::new(1);
        barrier.wait();
        assert_eq!(barrier.generation.load(Ordering::Acquire), 1);
    }
}