//! # Compliance Features Module
//! 
//! Ten moduł zawiera funkcje zgodności z regulacjami i standardami.

pub mod audit;
pub mod reporting;
pub mod encryption;
pub mod keys;
pub mod certificates;

pub use audit::{AuditLogger, AuditEvent, AuditLog};
pub use reporting::{ComplianceReporter, ComplianceReport, ComplianceStatus};
pub use encryption::{EncryptionManager, EncryptionAlgorithm, EncryptionKey};
pub use keys::{KeyManager, KeyType, KeyRotation};
pub use certificates::{CertificateManager, Certificate, CertificateAuthority};

use core::sync::atomic::{AtomicU32, Ordering};

/// Stan systemu zgodności
static COMPLIANCE_STATE: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje moduł zgodności
pub fn init() -> Result<(), ComplianceError> {
    // Inicjalizuj audit logging
    audit::init()?;
    
    // Inicjalizuj compliance reporting
    reporting::init()?;
    
    // Inicjalizuj encryption
    encryption::init()?;
    
    // Inicjalizuj key management
    keys::init()?;
    
    // Inicjalizuj certificate management
    certificates::init()?;
    
    COMPLIANCE_STATE.store(1, Ordering::Release);
    
    Ok(())
}

/// Zwraca stan systemu zgodności
pub fn compliance_state() -> bool {
    COMPLIANCE_STATE.load(Ordering::Acquire) == 1
}

/// Błędy zgodności
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceError {
    /// Błąd audit logging
    AuditError,
    /// Błąd reporting
    ReportingError,
    /// Błąd encryption
    EncryptionError,
    /// Błąd key management
    KeyError,
    /// Błąd certificate management
    CertificateError,
    /// Naruszenie zgodności
    ComplianceViolation,
}

impl core::fmt::Display for ComplianceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ComplianceError::AuditError => write!(f, "Audit error"),
            ComplianceError::ReportingError => write!(f, "Reporting error"),
            ComplianceError::EncryptionError => write!(f, "Encryption error"),
            ComplianceError::KeyError => write!(f, "Key error"),
            ComplianceError::CertificateError => write!(f, "Certificate error"),
            ComplianceError::ComplianceViolation => write!(f, "Compliance violation"),
        }
    }
}

impl core::error::Error for ComplianceError {}