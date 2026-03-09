//! # High Performance Networking Module
//! 
//! Ten moduł zawiera funkcje sieciowe o wysokiej wydajności.

pub mod dpdk;
pub mod bypass;
pub mod zerocopy;
pub mod acceleration;

pub use dpdk::{DpdkContext, DpdkPort, DpdkConfig};
pub use bypass::{KernelBypass, BypassMode};
pub use zerocopy::{ZeroCopyBuffer, ZeroCopyPool};
pub use acceleration::{NetworkAccelerator, AccelerationType};

use core::sync::atomic::{AtomicU32, Ordering};

/// Inicjalizuje moduł sieciowy o wysokiej wydajności
pub fn init() -> Result<(), NetworkingError> {
    // Inicjalizuj DPDK
    dpdk::init()?;
    
    // Inicjalizuj kernel bypass
    bypass::init()?;
    
    // Inicjalizuj zero-copy
    zerocopy::init()?;
    
    // Inicjalizuj akcelerację sieciową
    acceleration::init()?;
    
    Ok(())
}

/// Błędy sieciowe
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkingError {
    /// Błąd inicjalizacji
    InitFailed,
    /// Błąd konfiguracji
    ConfigError,
    /// Brak pamięci
    OutOfMemory,
    /// Błąd operacji I/O
    IoError,
    /// Błąd akceleracji
    AccelerationError,
}

impl core::fmt::Display for NetworkingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NetworkingError::InitFailed => write!(f, "Initialization failed"),
            NetworkingError::ConfigError => write!(f, "Configuration error"),
            NetworkingError::OutOfMemory => write!(f, "Out of memory"),
            NetworkingError::IoError => write!(f, "I/O error"),
            NetworkingError::AccelerationError => write!(f, "Acceleration error"),
        }
    }
}

impl core::error::Error for NetworkingError {}