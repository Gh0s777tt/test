//! # Backup & Recovery Module
//! 
//! Ten moduł zawiera funkcje tworzenia kopii zapasowych i odzyskiwania danych.

pub mod system;
pub mod incremental;
pub mod deduplication;
pub mod compression;
pub mod restore;
pub mod disaster;

pub use system::{BackupSystem, BackupConfig, BackupJob};
pub use incremental::{IncrementalBackup, BackupSnapshot};
pub use deduplication::{DeduplicationEngine, DeduplicationStats};
pub use compression::{CompressionEngine, CompressionAlgorithm};
pub use restore::{RestoreManager, RestoreJob, RestoreResult};
pub use disaster::{DisasterRecovery, RecoveryPlan, RecoveryAction};

use core::sync::atomic::{AtomicU32, Ordering};

/// Stan systemu backup
static BACKUP_STATE: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł backup
pub fn init() -> Result<(), BackupError> {
    // Inicjalizuj system backup
    system::init()?;
    
    // Inicjalizuj backup przyrostowy
    incremental::init()?;
    
    // Inicjalizuj deduplikację
    deduplication::init()?;
    
    // Inicjalizuj kompresję
    compression::init()?;
    
    // Inicjalizuj restore
    restore::init()?;
    
    // Inicjalizuj disaster recovery
    disaster::init()?;
    
    BACKUP_STATE.store(1, Ordering::Release);
    
    Ok(())
}

/// Zwraca stan systemu backup
pub fn backup_state() -> bool {
    BACKUP_STATE.load(Ordering::Acquire) == 1
}

/// Błędy backup
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupError {
    /// Błąd systemu backup
    SystemError,
    /// Błąd backup przyrostowy
    IncrementalError,
    /// Błąd deduplikacji
    DeduplicationError,
    /// Błąd kompresji
    CompressionError,
    /// Błąd restore
    RestoreError,
    /// Błąd disaster recovery
    DisasterError,
}

impl core::fmt::Display for BackupError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BackupError::SystemError => write!(f, "Backup system error"),
            BackupError::IncrementalError => write!(f, "Incremental backup error"),
            BackupError::DeduplicationError => write!(f, "Deduplication error"),
            BackupError::CompressionError => write!(f, "Compression error"),
            BackupError::RestoreError => write!(f, "Restore error"),
            BackupError::DisasterError => write!(f, "Disaster recovery error"),
        }
    }
}

impl core::error::Error for BackupError {}