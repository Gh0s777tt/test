//! # Advanced Security Module
//! 
//! Ten moduł zawiera zaawansowane funkcje bezpieczeństwa dla systemu enterprise.

pub mod selinux;
pub mod apparmor;
pub mod tpm;
pub mod secureboot;
pub mod measuredboot;
pub mod integrity;

pub use selinux::{SelinuxManager, SelinuxPolicy, SelinuxContext};
pub use apparmor::{AppArmorManager, AppArmorProfile, AppArmorMode};
pub use tpm::{TpmManager, TpmKey, TpmMeasurement};
pub use secureboot::{SecureBootManager, SecureBootKey, SecureBootState};
pub use measuredboot::{MeasuredBootManager, BootMeasurement, BootLog};
pub use integrity::{IntegrityChecker, IntegrityReport, IntegrityViolation};

use core::sync::atomic::{AtomicU32, Ordering};

/// Stan systemu bezpieczeństwa
static SECURITY_STATE: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł bezpieczeństwa
pub fn init() -> Result<(), SecurityError> {
    // Inicjalizuj SELinux
    selinux::init()?;
    
    // Inicjalizuj AppArmor
    apparmor::init()?;
    
    // Inicjalizuj TPM
    tpm::init()?;
    
    // Inicjalizuj Secure Boot
    secureboot::init()?;
    
    // Inicjalizuj Measured Boot
    measuredboot::init()?;
    
    // Inicjalizuj Integrity Checking
    integrity::init()?;
    
    SECURITY_STATE.store(1, Ordering::Release);
    
    Ok(())
}

/// Zwraca stan systemu bezpieczeństwa
pub fn security_state() -> bool {
    SECURITY_STATE.load(Ordering::Acquire) == 1
}

/// Błędy bezpieczeństwa
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityError {
    /// Błąd SELinux
    SelinuxError,
    /// Błąd AppArmor
    AppArmorError,
    /// Błąd TPM
    TpmError,
    /// Błąd Secure Boot
    SecureBootError,
    /// Błąd Measured Boot
    MeasuredBootError,
    /// Błąd Integrity Checking
    IntegrityError,
    /// Naruszenie bezpieczeństwa
    SecurityViolation,
}

impl core::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SecurityError::SelinuxError => write!(f, "SELinux error"),
            SecurityError::AppArmorError => write!(f, "AppArmor error"),
            SecurityError::TpmError => write!(f, "TPM error"),
            SecurityError::SecureBootError => write!(f, "Secure Boot error"),
            SecurityError::MeasuredBootError => write!(f, "Measured Boot error"),
            SecurityError::IntegrityError => write!(f, "Integrity error"),
            SecurityError::SecurityViolation => write!(f, "Security violation"),
        }
    }
}

impl core::error::Error for SecurityError {}