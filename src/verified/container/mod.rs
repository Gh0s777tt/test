//! # Containerization Module
//! 
//! Ten moduł zawiera funkcje konteneryzacji dla izolacji aplikacji.

pub mod runtime;
pub mod orchestration;
pub mod isolation;
pub mod networking;
pub mod storage;

pub use runtime::{Container, ContainerRuntime, ContainerConfig};
pub use orchestration::{Orchestrator, ContainerPod, Service};
pub use isolation::{Namespace, Cgroup, ResourceLimits};
pub use networking::{ContainerNetwork, NetworkConfig};
pub use storage::{ContainerStorage, Volume, StorageConfig};

use core::sync::atomic::{AtomicU32, Ordering};

/// Liczba aktywnych kontenerów
static ACTIVE_CONTAINERS: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł konteneryzacji
pub fn init() -> Result<(), ContainerError> {
    // Inicjalizuj runtime kontenerów
    runtime::init()?;
    
    // Inicjalizuj orkiestrację
    orchestration::init()?;
    
    // Inicjalizuj izolację
    isolation::init()?;
    
    // Inicjalizuj sieci kontenerów
    networking::init()?;
    
    // Inicjalizuj pamięć kontenerów
    storage::init()?;
    
    Ok(())
}

/// Zwraca liczbę aktywnych kontenerów
pub fn container_count() -> usize {
    ACTIVE_CONTAINERS.load(Ordering::Acquire) as usize
}

/// Błędy konteneryzacji
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerError {
    /// Kontener nie istnieje
    ContainerNotFound,
    /// Błąd tworzenia kontenera
    CreateFailed,
    /// Błąd uruchamiania kontenera
    StartFailed,
    /// Błąd zatrzymywania kontenera
    StopFailed,
    /// Błąd izolacji
    IsolationError,
    /// Brak zasobów
    OutOfResources,
}

impl core::fmt::Display for ContainerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ContainerError::ContainerNotFound => write!(f, "Container not found"),
            ContainerError::CreateFailed => write!(f, "Container creation failed"),
            ContainerError::StartFailed => write!(f, "Container start failed"),
            ContainerError::StopFailed => write!(f, "Container stop failed"),
            ContainerError::IsolationError => write!(f, "Isolation error"),
            ContainerError::OutOfResources => write!(f, "Out of resources"),
        }
    }
}

impl core::error::Error for ContainerError {}