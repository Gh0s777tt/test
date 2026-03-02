//! # CPU Affinity Module
//! 
//! Zarządza przypisaniem zadań do konkretnych CPU.

use core::sync::atomic::{AtomicU32, Ordering};

/// Przynależność CPU
pub struct CpuAffinity {
    /// Mask CPU
    pub cpu_mask: u64,
    /// Liczba CPU
    pub cpu_count: u32,
}

impl CpuAffinity {
    /// Tworzy nową przynależność CPU
    pub fn new(cpu_mask: u64, cpu_count: u32) -> Self {
        Self {
            cpu_mask,
            cpu_count,
        }
    }
    
    /// Tworzy przynależność dla pojedynczego CPU
    pub fn single(cpu_id: u32) -> Self {
        Self {
            cpu_mask: 1u64 << cpu_id,
            cpu_count: 1,
        }
    }
    
    /// Tworzy przynależność dla wszystkich CPU
    pub fn all(cpu_count: u32) -> Self {
        Self {
            cpu_mask: (1u64 << cpu_count) - 1,
            cpu_count,
        }
    }
    
    /// Sprawdza czy CPU jest w masce
    pub fn contains(&self, cpu_id: u32) -> bool {
        (self.cpu_mask & (1u64 << cpu_id)) != 0
    }
    
    /// Dodaje CPU do maski
    pub fn add(&mut self, cpu_id: u32) {
        self.cpu_mask |= 1u64 << cpu_id;
        self.cpu_count += 1;
    }
    
    /// Usuwa CPU z maski
    pub fn remove(&mut self, cpu_id: u32) {
        self.cpu_mask &= !(1u64 << cpu_id);
        self.cpu_count -= 1;
    }
    
    /// Zwraca listę CPU
    pub fn cpus(&self) -> Vec<u32> {
        let mut cpus = Vec::new();
        for i in 0..64 {
            if self.contains(i) {
                cpus.push(i);
            }
        }
        cpus
    }
}

/// Ustawia przynależność CPU dla zadania
pub fn set_task_affinity(task_id: u64, affinity: CpuAffinity) -> Result<(), super::SchedulerError> {
    // Placeholder - ustawienie przynależności zadania
    let _ = (task_id, affinity);
    Ok(())
}

/// Pobiera przynależność CPU dla zadania
pub fn get_task_affinity(task_id: u64) -> Option<CpuAffinity> {
    // Placeholder - pobieranie przynależności zadania
    let _ = task_id;
    None
}

/// Ustawia przynależność CPU dla bieżącego zadania
pub fn set_current_affinity(affinity: CpuAffinity) -> Result<(), super::SchedulerError> {
    // Placeholder - ustawienie przynależności bieżącego zadania
    let _ = affinity;
    Ok(())
}

/// Pobiera przynależność CPU dla bieżącego zadania
pub fn get_current_affinity() -> Option<CpuAffinity> {
    // Placeholder - pobieranie przynależności bieżącego zadania
    None
}

/// Inicjalizuje moduł przynależności CPU
pub fn init() -> Result<(), super::SchedulerError> {
    // Placeholder - inicjalizacja modułu
    Ok(())
}

/// Zwraca domyślną przynależność CPU
pub fn default_affinity() -> CpuAffinity {
    let cpu_count = crate::verified::smp::core::active_cpu_count() as u32;
    CpuAffinity::all(cpu_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cpu_affinity_single() {
        let affinity = CpuAffinity::single(0);
        assert!(affinity.contains(0));
        assert!(!affinity.contains(1));
        assert_eq!(affinity.cpu_count, 1);
    }
    
    #[test]
    fn test_cpu_affinity_all() {
        let affinity = CpuAffinity::all(4);
        assert!(affinity.contains(0));
        assert!(affinity.contains(1));
        assert!(affinity.contains(2));
        assert!(affinity.contains(3));
        assert!(!affinity.contains(4));
        assert_eq!(affinity.cpu_count, 4);
    }
    
    #[test]
    fn test_cpu_affinity_add_remove() {
        let mut affinity = CpuAffinity::single(0);
        affinity.add(1);
        assert!(affinity.contains(1));
        assert_eq!(affinity.cpu_count, 2);
        
        affinity.remove(0);
        assert!(!affinity.contains(0));
        assert_eq!(affinity.cpu_count, 1);
    }
}