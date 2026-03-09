//! Certification Module
//! 
//! This module provides comprehensive certification management for
//! VantisOS including ISO 27001, SOC 2, PCI DSS, HIPAA, FIPS 140-3,
//! and EAL 7+ certifications.

pub mod iso27001;
pub mod soc2;
pub mod pci_dss;
pub mod hipaa;
pub mod fips1403;
pub mod eal;

use alloc::sync::Arc;
use spin::Mutex;

/// Certification type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationType {
    ISO27001,
    SOC2,
    PciDss,
    Hipaa,
    Fips1403,
    EAL,
    Custom(usize),
}

/// Certification status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationStatus {
    NotStarted,
    InProgress,
    Review,
    Certified,
    Expired,
    Revoked,
}

/// Certification level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationLevel {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

/// Certification information
#[derive(Debug, Clone)]
pub struct CertificationInfo {
    pub cert_type: CertificationType,
    pub status: CertificationStatus,
    pub level: Option<CertificationLevel>,
    pub issued_date: Option<u64>,
    pub expiry_date: Option<u64>,
    pub issuer: Option<alloc::string::String>,
    pub certificate_id: Option<alloc::string::String>,
    pub compliance_score: f64, // 0.0 to 1.0
}

/// Certification manager
pub struct CertificationManager {
    certifications: Arc<Mutex<alloc::collections::BTreeMap<CertificationType, CertificationInfo>>>,
}

impl CertificationManager {
    /// Create a new certification manager
    pub fn new() -> Self {
        Self {
            certifications: Arc::new(Mutex::new(alloc::collections::BTreeMap::new())),
        }
    }

    /// Register a certification
    pub fn register_certification(&self, cert_type: CertificationType) {
        let mut certs = self.certifications.lock();
        if !certs.contains_key(&cert_type) {
            certs.insert(cert_type, CertificationInfo {
                cert_type,
                status: CertificationStatus::NotStarted,
                level: None,
                issued_date: None,
                expiry_date: None,
                issuer: None,
                certificate_id: None,
                compliance_score: 0.0,
            });
        }
    }

    /// Update certification status
    pub fn update_status(&self, cert_type: CertificationType, status: CertificationStatus) {
        let mut certs = self.certifications.lock();
        if let Some(cert) = certs.get_mut(&cert_type) {
            cert.status = status;
        }
    }

    /// Update compliance score
    pub fn update_compliance_score(&self, cert_type: CertificationType, score: f64) {
        let mut certs = self.certifications.lock();
        if let Some(cert) = certs.get_mut(&cert_type) {
            cert.compliance_score = score;
        }
    }

    /// Set certification details
    pub fn set_details(
        &self,
        cert_type: CertificationType,
        issued_date: Option<u64>,
        expiry_date: Option<u64>,
        issuer: Option<alloc::string::String>,
        certificate_id: Option<alloc::string::String>,
    ) {
        let mut certs = self.certifications.lock();
        if let Some(cert) = certs.get_mut(&cert_type) {
            cert.issued_date = issued_date;
            cert.expiry_date = expiry_date;
            cert.issuer = issuer;
            cert.certificate_id = certificate_id;
        }
    }

    /// Get certification information
    pub fn get_certification(&self, cert_type: CertificationType) -> Option<CertificationInfo> {
        self.certifications.lock().get(&cert_type).cloned()
    }

    /// Get all certifications
    pub fn all_certifications(&self) -> Vec<CertificationInfo> {
        self.certifications.lock().values().cloned().collect()
    }

    /// Get certified certifications
    pub fn certified_certifications(&self) -> Vec<CertificationInfo> {
        self.certifications
            .lock()
            .values()
            .filter(|c| c.status == CertificationStatus::Certified)
            .cloned()
            .collect()
    }

    /// Check if a certification is valid
    pub fn is_valid(&self, cert_type: CertificationType) -> bool {
        if let Some(cert) = self.get_certification(cert_type) {
            if cert.status == CertificationStatus::Certified {
                if let Some(expiry) = cert.expiry_date {
                    expiry > self.current_timestamp()
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Get overall compliance score
    pub fn overall_compliance_score(&self) -> f64 {
        let certs = self.certifications.lock();
        if certs.is_empty() {
            0.0
        } else {
            let total_score: f64 = certs.values().map(|c| c.compliance_score).sum();
            total_score / certs.len() as f64
        }
    }

    /// Get current timestamp (placeholder)
    fn current_timestamp(&self) -> u64 {
        // In a real implementation, this would return actual time
        0
    }
}

impl Default for CertificationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global certification manager
static CERTIFICATION_MANAGER: spin::Once<CertificationManager> = spin::Once::new();

/// Get the global certification manager
pub fn certification_manager() -> &'static CertificationManager {
    CERTIFICATION_MANAGER.call_once(|| CertificationManager::new())
}

/// Register a certification
pub fn register_certification(cert_type: CertificationType) {
    certification_manager().register_certification(cert_type);
}

/// Update certification status
pub fn update_certification_status(cert_type: CertificationType, status: CertificationStatus) {
    certification_manager().update_status(cert_type, status);
}

/// Get all certifications
pub fn all_certifications() -> Vec<CertificationInfo> {
    certification_manager().all_certifications()
}

/// Get overall compliance score
pub fn overall_compliance_score() -> f64 {
    certification_manager().overall_compliance_score()
}