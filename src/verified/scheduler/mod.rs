//! # Scheduler Module
//! 
//! Implementuje planista zadań dla środowiska wielordzeniowego z różnymi
/// strategiami planowania i load balancing.

pub mod policy;
pub mod load_balancer;
pub mod affinity;
pub mod priority;
pub mod queue;

pub use policy::{SchedulingPolicy, SchedulingClass};
pub use load_balancer::{LoadBalancer, LoadBalancingStrategy};
pub use affinity::{CpuAffinity, set_task_affinity};
pub use priority::{Priority, PriorityLevel};
pub use queue::{TaskQueue, TaskQueueType};

use core::sync::atomic::{AtomicU32, Ordering};

/// Liczba aktywnych planistów
static ACTIVE_SCHEDULERS: AtomicU32 = AtomicU32::new(1);

/// Inicjalizuje system planowania
pub fn init() -> Result<(), SchedulerError> {
    // Inicjalizuj polityki planowania
    policy::init()?;
    
    // Inicjalizuj load balancer
    load_balancer::init()?;
    
    // Inicjalizuj przynależność CPU
    affinity::init()?;
    
    // Inicjalizuj priorytety
    priority::init()?;
    
    // Inicjalizuj kolejki zadań
    queue::init()?;
    
    Ok(())
}

/// Zwraca liczbę aktywnych planistów
pub fn scheduler_count() -> usize {
    ACTIVE_SCHEDULERS.load(Ordering::Acquire) as usize
}

/// Błędy planisty
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerError {
    /// Nieprawidłowa polityka planowania
    InvalidPolicy,
    /// Nieprawidłowa przynależność CPU
    InvalidAffinity,
    /// Nieprawidłowy priorytet
    InvalidPriority,
    /// Kolejka zadań pełna
    QueueFull,
    /// Kolejka zadań pusta
    QueueEmpty,
    /// Błąd load balancing
    LoadBalancingError,
}

impl core::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SchedulerError::InvalidPolicy => write!(f, "Invalid scheduling policy"),
            SchedulerError::InvalidAffinity => write!(f, "Invalid CPU affinity"),
            SchedulerError::InvalidPriority => write!(f, "Invalid priority"),
            SchedulerError::QueueFull => write!(f, "Task queue full"),
            SchedulerError::QueueEmpty => write!(f, "Task queue empty"),
            SchedulerError::LoadBalancingError => write!(f, "Load balancing error"),
        }
    }
}

impl core::error::Error for SchedulerError {}