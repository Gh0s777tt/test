//! # Virtualization Module
//! 
//! Ten moduł zawiera funkcje wirtualizacji dla uruchamiania maszyn wirtualnych.

pub mod hypervisor;
pub mod vm;
pub mod passthrough;
pub mod migration;
pub mod snapshot;

pub use hypervisor::{Hypervisor, HypervisorType, HypervisorConfig};
pub use vm::{VirtualMachine, VmConfig, VmState};
pub use passthrough::{DevicePassthrough, PassthroughDevice};
pub use migration::{LiveMigration, MigrationConfig};
pub use snapshot::{VmSnapshot, SnapshotConfig};

use core::sync::atomic::{AtomicU32, Ordering};

/// Liczba aktywnych VM
static ACTIVE_VMS: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł wirtualizacji
pub fn init() -> Result<(), VirtualizationError> {
    // Inicjalizuj hypervisor
    hypervisor::init()?;
    
    // Inicjalizuj zarządzanie VM
    vm::init()?;
    
    // Inicjalizuj device passthrough
    passthrough::init()?;
    
    // Inicjalizuj live migration
    migration::init()?;
    
    // Inicjalizuj snapshot
    snapshot::init()?;
    
    Ok(())
}

/// Zwraca liczbę aktywnych VM
pub fn vm_count() -> usize {
    ACTIVE_VMS.load(Ordering::Acquire) as usize
}

/// Błędy wirtualizacji
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualizationError {
    /// VM nie istnieje
    VmNotFound,
    /// Błąd tworzenia VM
    CreateFailed,
    /// Błąd uruchamiania VM
    StartFailed,
    /// Błąd zatrzymywania VM
    StopFailed,
    /// Błąd hypervisora
    HypervisorError,
    /// Brak zasobów
    OutOfResources,
}

impl core::fmt::Display for VirtualizationError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VirtualizationError::VmNotFound => write!(f, "VM not found"),
            VirtualizationError::CreateFailed => write!(f, "VM creation failed"),
            VirtualizationError::StartFailed => write!(f, "VM start failed"),
            VirtualizationError::StopFailed => write!(f, "VM stop failed"),
            VirtualizationError::HypervisorError => write!(f, "Hypervisor error"),
            VirtualizationError::OutOfResources => write!(f, "Out of resources"),
        }
    }
}

impl core::error::Error for VirtualizationError {}