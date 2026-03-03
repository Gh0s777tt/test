//! # Server Device Drivers Module
//! 
//! Ten moduł zawiera sterowniki dla urządzeń serwerowych.

pub mod nic;
pub mod rdma;
pub mod nvme;
pub mod raid;
pub mod hba;
pub mod gpu;

pub use nic::{NicDriver, NicConfig, NicStats};
pub use rdma::{RdmaDriver, RdmaConfig, RdmaStats};
pub use nvme::{NvmeDriver, NvmeConfig, NvmeStats};
pub use raid::{RaidDriver, RaidConfig, RaidStats};
pub use hba::{HbaDriver, HbaConfig, HbaStats};
pub use gpu::{GpuDriver, GpuConfig, GpuStats};

use core::sync::atomic::{AtomicU32, Ordering};

/// Liczba aktywnych sterowników serwerowych
static ACTIVE_DRIVERS: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje sterowniki serwerowe
pub fn init() -> Result<(), ServerDriverError> {
    // Inicjalizuj sterowniki NIC
    nic::init()?;
    
    // Inicjalizuj sterowniki RDMA
    rdma::init()?;
    
    // Inicjalizuj sterowniki NVMe
    nvme::init()?;
    
    // Inicjalizuj sterowniki RAID
    raid::init()?;
    
    // Inicjalizuj sterowniki HBA
    hba::init()?;
    
    // Inicjalizuj sterowniki GPU
    gpu::init()?;
    
    Ok(())
}

/// Zwraca liczbę aktywnych sterowników
pub fn driver_count() -> usize {
    ACTIVE_DRIVERS.load(Ordering::Acquire) as usize
}

/// Błędy sterowników serwerowych
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerDriverError {
    /// Sterownik nie istnieje
    DriverNotFound,
    /// Błąd inicjalizacji
    InitFailed,
    /// Błąd konfiguracji
    ConfigError,
    /// Błąd operacji I/O
    IoError,
    /// Brak zasobów
    OutOfResources,
}

impl core::fmt::Display for ServerDriverError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ServerDriverError::DriverNotFound => write!(f, "Driver not found"),
            ServerDriverError::InitFailed => write!(f, "Driver initialization failed"),
            ServerDriverError::ConfigError => write!(f, "Driver configuration error"),
            ServerDriverError::IoError => write!(f, "I/O error"),
            ServerDriverError::OutOfResources => write!(f, "Out of resources"),
        }
    }
}

impl core::error::Error for ServerDriverError {}