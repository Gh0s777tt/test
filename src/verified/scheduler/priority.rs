//! # Priority Module
//! 
//! Definiuje poziomy priorytetów i zarządza nimi.

use core::sync::atomic::{AtomicU32, Ordering};

/// Poziom priorytetu
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityLevel {
    /// Priorytet czasu rzeczywistego
    RealTime = 0,
    /// Wysoki priorytet
    High = 1,
    /// Normalny priorytet
    Normal = 2,
    /// Niski priorytet
    Low = 3,
    /// Priorytet idle
    Idle = 4,
}

/// Priorytet zadania
#[derive(Debug, Clone, Copy)]
pub struct Priority {
    /// Poziom priorytetu
    pub level: PriorityLevel,
    /// Wartość nice (-20 do 19)
    pub nice: i32,
    /// Priorytet statyczny
    pub static_prio: u32,
    /// Priorytet dynamiczny
    pub dynamic_prio: u32,
}

impl Priority {
    /// Tworzy nowy priorytet
    pub fn new(level: PriorityLevel, nice: i32) -> Self {
        let static_prio = Self::nice_to_static_prio(nice);
        let dynamic_prio = static_prio;
        
        Self {
            level,
            nice,
            static_prio,
            dynamic_prio,
        }
    }
    
    /// Tworzy priorytet czasu rzeczywistego
    pub fn realtime(prio: u32) -> Self {
        Self {
            level: PriorityLevel::RealTime,
            nice: -20,
            static_prio: prio,
            dynamic_prio: prio,
        }
    }
    
    /// Tworzy normalny priorytet
    pub fn normal() -> Self {
        Self::new(PriorityLevel::Normal, 0)
    }
    
    /// Konwertuje nice na priorytet statyczny
    fn nice_to_static_prio(nice: i32) -> u32 {
        // nice: -20..19 -> static_prio: 100..139
        const PRIO_TO_NICE: i32 = 120;
        (PRIO_TO_NICE + nice) as u32
    }
    
    /// Konwertuje priorytet statyczny na nice
    pub fn static_prio_to_nice(static_prio: u32) -> i32 {
        const PRIO_TO_NICE: i32 = 120;
        (static_prio as i32) - PRIO_TO_NICE
    }
    
    /// Aktualizuje priorytet dynamiczny
    pub fn update_dynamic_prio(&mut self) {
        // Placeholder - aktualizacja priorytetu dynamicznego
        self.dynamic_prio = self.static_prio;
    }
    
    /// Zwraca czy jest to priorytet czasu rzeczywistego
    pub fn is_realtime(&self) -> bool {
        self.level == PriorityLevel::RealTime
    }
}

/// Domyślny priorytet
static DEFAULT_PRIORITY: AtomicU32 = AtomicU32::new(120); // nice 0

/// Inicjalizuje moduł priorytetów
pub fn init() -> Result<(), super::SchedulerError> {
    // Placeholder - inicjalizacja modułu
    Ok(())
}

/// Ustawia domyślny priorytet
pub fn set_default_priority(nice: i32) {
    let static_prio = Priority::nice_to_static_prio(nice);
    DEFAULT_PRIORITY.store(static_prio, Ordering::Release);
}

/// Zwraca domyślny priorytet
pub fn default_priority() -> Priority {
    let static_prio = DEFAULT_PRIORITY.load(Ordering::Acquire);
    let nice = Priority::static_prio_to_nice(static_prio);
    Priority::new(PriorityLevel::Normal, nice)
}

/// Zwraca priorytet dla zadania
pub fn get_task_priority(task_id: u64) -> Option<Priority> {
    // Placeholder - pobieranie priorytetu zadania
    let _ = task_id;
    None
}

/// Ustawia priorytet dla zadania
pub fn set_task_priority(task_id: u64, priority: Priority) -> Result<(), super::SchedulerError> {
    // Placeholder - ustawienie priorytetu zadania
    let _ = (task_id, priority);
    Ok(())
}

/// Zwraca priorytet dla bieżącego zadania
pub fn get_current_priority() -> Option<Priority> {
    // Placeholder - pobieranie priorytetu bieżącego zadania
    None
}

/// Ustawia priorytet dla bieżącego zadania
pub fn set_current_priority(priority: Priority) -> Result<(), super::SchedulerError> {
    // Placeholder - ustawienie priorytetu bieżącego zadania
    let _ = priority;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_priority_creation() {
        let priority = Priority::new(PriorityLevel::Normal, 0);
        assert_eq!(priority.level, PriorityLevel::Normal);
        assert_eq!(priority.nice, 0);
    }
    
    #[test]
    fn test_priority_realtime() {
        let priority = Priority::realtime(50);
        assert_eq!(priority.level, PriorityLevel::RealTime);
        assert_eq!(priority.static_prio, 50);
        assert!(priority.is_realtime());
    }
    
    #[test]
    fn test_nice_conversion() {
        let static_prio = Priority::nice_to_static_prio(0);
        assert_eq!(static_prio, 120);
        
        let nice = Priority::static_prio_to_nice(120);
        assert_eq!(nice, 0);
    }
}