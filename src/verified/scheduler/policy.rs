//! # Scheduling Policy Module
//! 
//! Definiuje różne polityki planowania zadań.

use core::sync::atomic::{AtomicU32, Ordering};

/// Polityka planowania
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingPolicy {
    /// Completely Fair Scheduler (CFS)
    Cfs,
    /// Real-time scheduler
    RealTime,
    /// Round-robin
    RoundRobin,
    /// First-come, first-served
    Fcfs,
    /// Shortest job first
    Sjf,
    /// Priority-based
    Priority,
    /// Deadline scheduler
    Deadline,
}

/// Klasa planowania
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingClass {
    /// Klasa czasu rzeczywistego
    RealTime,
    /// Klasa normalna
    Normal,
    /// Klasa tła
    Batch,
    /// Klasa idle
    Idle,
}

/// Domyślna polityka planowania
static DEFAULT_POLICY: AtomicU32 = AtomicU32::new(0); // 0 = CFS

/// Inicjalizuje polityki planowania
pub fn init() -> Result<(), super::SchedulerError> {
    // Placeholder - inicjalizacja polityk
    Ok(())
}

/// Ustawia domyślną politykę planowania
pub fn set_default_policy(policy: SchedulingPolicy) {
    DEFAULT_POLICY.store(policy as u32, Ordering::Release);
}

/// Zwraca domyślną politykę planowania
pub fn default_policy() -> SchedulingPolicy {
    match DEFAULT_POLICY.load(Ordering::Acquire) {
        0 => SchedulingPolicy::Cfs,
        1 => SchedulingPolicy::RealTime,
        2 => SchedulingPolicy::RoundRobin,
        3 => SchedulingPolicy::Fcfs,
        4 => SchedulingPolicy::Sjf,
        5 => SchedulingPolicy::Priority,
        6 => SchedulingPolicy::Deadline,
        _ => SchedulingPolicy::Cfs,
    }
}

/// Zwraca klasę planowania dla priorytetu
pub fn priority_to_class(priority: super::priority::Priority) -> SchedulingClass {
    match priority.level {
        super::priority::PriorityLevel::RealTime => SchedulingClass::RealTime,
        super::priority::PriorityLevel::High => SchedulingClass::Normal,
        super::priority::PriorityLevel::Normal => SchedulingClass::Normal,
        super::priority::PriorityLevel::Low => SchedulingClass::Batch,
        super::priority::PriorityLevel::Idle => SchedulingClass::Idle,
    }
}

/// Oblicza kwant czasu dla zadania
pub fn calculate_time_slice(policy: SchedulingPolicy, priority: super::priority::Priority) -> u64 {
    match policy {
        SchedulingPolicy::Cfs => {
            // CFS używa dynamicznych kwantów czasu
            match priority.level {
                super::priority::PriorityLevel::RealTime => 10000, // 10ms
                super::priority::PriorityLevel::High => 20000,     // 20ms
                super::priority::PriorityLevel::Normal => 30000,   // 30ms
                super::priority::PriorityLevel::Low => 50000,      // 50ms
                super::priority::PriorityLevel::Idle => 100000,    // 100ms
            }
        }
        SchedulingPolicy::RoundRobin => {
            // Round-robin używa stałych kwantów czasu
            20000 // 20ms
        }
        SchedulingPolicy::RealTime => {
            // Real-time ma bardzo krótkie kwanty
            5000 // 5ms
        }
        _ => 30000, // Domyślnie 30ms
    }
}

/// Oblicza vruntime dla CFS
pub fn calculate_vruntime(exec_time: u64, weight: u32) -> u64 {
    // vruntime = exec_time * (1024 / weight)
    const NICE_0_WEIGHT: u32 = 1024;
    (exec_time * NICE_0_WEIGHT as u64) / weight as u64
}

/// Oblicza wagę zadania
pub fn calculate_weight(priority: super::priority::Priority) -> u32 {
    match priority.level {
        super::priority::PriorityLevel::RealTime => 88761,
        super::priority::PriorityLevel::High => 31215,
        super::priority::PriorityLevel::Normal => 1024,
        super::priority::PriorityLevel::Low => 351,
        super::priority::PriorityLevel::Idle => 102,
    }
}