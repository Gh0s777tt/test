//! # Symmetric Multi-Processing (SMP) Module
//! 
//! Ten moduł zapewnia wsparcie dla wielordzeniowych procesorów z pamięcią współdzieloną.
//! Implementuje SMP (Symmetric Multi-Processing) dla równoległego wykonywania zadań.

pub mod core;
pub mod boot;
pub mod ipi;
pub mod barrier;
pub mod spinlock;

pub use core::{CpuId, CpuInfo, CpuState};
pub use boot::{smp_init, smp_boot_secondary_cpus};
pub use ipi::{IpiType, send_ipi, send_ipi_to_others};
pub use barrier::{Barrier, barrier_wait};
pub use spinlock::{SpinLock, SpinLockGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use core::sync::atomic::{AtomicU32, Ordering};

/// Liczba aktywnych CPU w systemie
static ACTIVE_CPUS: AtomicU32 = AtomicU32::new(1);

/// Maksymalna liczba obsługiwanych CPU
pub const MAX_CPUS: usize = 256;

/// Inicjalizuje moduł SMP
pub fn init() -> Result<(), SmpError> {
    // Inicjalizacja podstawowych struktur SMP
    core::init()?;
    ipi::init()?;
    barrier::init()?;
    
    Ok(())
}

/// Zwraca liczbę aktywnych CPU
pub fn cpu_count() -> usize {
    ACTIVE_CPUS.load(Ordering::Acquire) as usize
}

/// Zwraca ID bieżącego CPU
pub fn current_cpu() -> CpuId {
    core::current_cpu()
}

/// Błędy związane z SMP
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmpError {
    /// CPU nie istnieje
    InvalidCpuId,
    /// CPU jest już aktywne
    CpuAlreadyActive,
    /// CPU nie może zostać uruchomione
    CpuBootFailed,
    /// Błąd wysyłania IPI
    IpiFailed,
    /// Błąd bariery
    BarrierError,
    /// Błąd synchronizacji
    SyncError,
}

impl core::fmt::Display for SmpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SmpError::InvalidCpuId => write!(f, "Invalid CPU ID"),
            SmpError::CpuAlreadyActive => write!(f, "CPU already active"),
            SmpError::CpuBootFailed => write!(f, "CPU boot failed"),
            SmpError::IpiFailed => write!(f, "IPI send failed"),
            SmpError::BarrierError => write!(f, "Barrier error"),
            SmpError::SyncError => write!(f, "Synchronization error"),
        }
    }
}

impl core::error::Error for SmpError {}