//! FIPS 140-3 Certification Module
//! 
//! This module provides FIPS 140-3 (Federal Information Processing
//! Standard) cryptographic module certification capabilities.

use alloc::string::String;

/// FIPS 140-3 security level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fips1403SecurityLevel {
    Level1,
    Level2,
    Level3,
    Level4,
}

/// FIPS 140-3 algorithm
#[derive(Debug, Clone, Copy)]
pub enum Fips1403Algorithm {
    Aes128,
    Aes192,
    Aes256,
    TripleDes,
    Rsa1024,
    Rsa2048,
    Rsa3072,
    Rsa4096,
    Dsa1024,
    Dsa2048,
    Dsa3072,
    EcdsaP256,
    EcdsaP384,
    EcdsaP521,
    EcdhP256,
    EcdhP384,
    EcdhP521,
    Sha256,
    Sha384,
    Sha512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    HmacSha256,
    HmacSha384,
    HmacSha512,
}

/// FIPS 140-3 control
#[derive(Debug, Clone)]
pub struct Fips1403Control {
    pub control_id: String,
    pub control_title: String,
    pub security_level: Fips1403SecurityLevel,
    pub algorithm: Option<Fips1403Algorithm>,
    pub implemented: bool,
    pub validated: bool,
    pub evidence_location: Option<String>,
}

impl Fips1403Control {
    /// Create a new FIPS 140-3 control
    pub fn new(
        control_id: impl Into<String>,
        control_title: impl Into<String>,
        security_level: Fips1403SecurityLevel,
    ) -> Self {
        Self {
            control_id: control_id.into(),
            control_title: control_title.into(),
            security_level,
            algorithm: None,
            implemented: false,
            validated: false,
            evidence_location: None,
        }
    }

    /// Set algorithm
    pub fn set_algorithm(&mut self, algorithm: Fips1403Algorithm) {
        self.algorithm = Some(algorithm);
    }

    /// Mark as implemented
    pub fn mark_implemented(&mut self, evidence: Option<String>) {
        self.implemented = true;
        self.evidence_location = evidence;
    }

    /// Mark as validated
    pub fn mark_validated(&mut self) {
        self.validated = true;
    }
}

/// FIPS 140-3 compliance checker
pub struct Fips1403ComplianceChecker {
    controls: alloc::vec::Vec<Fips1403Control>,
    compliance_score: f64,
    target_security_level: Fips1403SecurityLevel,
}

impl Fips1403ComplianceChecker {
    /// Create a new FIPS 140-3 compliance checker
    pub fn new(target_level: Fips1403SecurityLevel) -> Self {
        let mut checker = Self {
            controls: Vec::new(),
            compliance_score: 0.0,
            target_security_level: target_level,
        };

        checker.initialize_controls();
        checker
    }

    /// Initialize standard FIPS 140-3 controls
    fn initialize_controls(&mut self) {
        // Cryptographic module specification
        let mut control = Fips1403Control::new(
            "1.0",
            "Cryptographic module specification",
            self.target_security_level,
        );
        self.controls.push(control);

        // Cryptographic module ports and interfaces
        control = Fips1403Control::new(
            "2.0",
            "Cryptographic module ports and interfaces",
            self.target_security_level,
        );
        self.controls.push(control);

        // Roles, services, and authentication
        control = Fips1403Control::new(
            "3.0",
            "Roles, services, and authentication",
            self.target_security_level,
        );
        self.controls.push(control);

        // Finite state model
        control = Fips1403Control::new(
            "4.0",
            "Finite state model",
            self.target_security_level,
        );
        self.controls.push(control);

        // Physical security
        control = Fips1403Control::new(
            "5.0",
            "Physical security",
            self.target_security_level,
        );
        self.controls.push(control);

        // Operational environment
        control = Fips1403Control::new(
            "6.0",
            "Operational environment",
            self.target_security_level,
        );
        self.controls.push(control);

        // Cryptographic key management
        control = Fips1403Control::new(
            "7.0",
            "Cryptographic key management",
            self.target_security_level,
        );
        self.controls.push(control);

        // Self-tests
        control = Fips1403Control::new(
            "8.0",
            "Self-tests",
            self.target_security_level,
        );
        self.controls.push(control);

        // Design assurance
        control = Fips1403Control::new(
            "9.0",
            "Design assurance",
            self.target_security_level,
        );
        self.controls.push(control);

        // Mitigation of other attacks
        control = Fips1403Control::new(
            "10.0",
            "Mitigation of other attacks",
            self.target_security_level,
        );
        self.controls.push(control);

        // Approved algorithms
        for algorithm in [
            Fips1403Algorithm::Aes128,
            Fips1403Algorithm::Aes256,
            Fips1403Algorithm::Rsa2048,
            Fips1403Algorithm::EcdsaP256,
            Fips1403Algorithm::EcdhP256,
            Fips1403Algorithm::Sha256,
            Fips1403Algorithm::Sha384,
            Fips1403Algorithm::Sha512,
            Fips1403Algorithm::HmacSha256,
        ] {
            control = Fips1403Control::new(
                alloc::format!("ALG.{}", alloc::format!("{:?}", algorithm)),
                alloc::format!("Algorithm validation: {:?}", algorithm),
                self.target_security_level,
            );
            control.set_algorithm(algorithm);
            self.controls.push(control);
        }
    }

    /// Implement a control
    pub fn implement_control(&mut self, control_id: &str, evidence: Option<String>) {
        if let Some(control) = self.controls.iter_mut().find(|c| c.control_id == control_id) {
            control.mark_implemented(evidence);
        }
    }

    /// Validate a control
    pub fn validate_control(&mut self, control_id: &str) {
        if let Some(control) = self.controls.iter_mut().find(|c| c.control_id == control_id) {
            control.mark_validated();
        }
    }

    /// Calculate compliance score
    pub fn calculate_compliance(&mut self) -> f64 {
        let total = self.controls.len();
        if total == 0 {
            return 0.0;
        }

        let implemented = self.controls.iter().filter(|c| c.implemented).count();
        let validated = self.controls.iter().filter(|c| c.validated).count();

        let implementation_score = (implemented as f64) / (total as f64);
        let validation_score = (validated as f64) / (total as f64);

        self.compliance_score = (implementation_score * 0.7) + (validation_score * 0.3);
        self.compliance_score
    }

    /// Get controls by security level
    pub fn controls_by_level(&self, level: Fips1403SecurityLevel) -> Vec<Fips1403Control> {
        self.controls
            .iter()
            .filter(|c| c.security_level == level)
            .cloned()
            .collect()
    }

    /// Get all controls
    pub fn all_controls(&self) -> &[Fips1403Control] {
        &self.controls
    }

    /// Get compliance score
    pub fn compliance_score(&self) -> f64 {
        self.compliance_score
    }

    /// Get non-compliant controls
    pub fn non_compliant_controls(&self) -> Vec<Fips1403Control> {
        self.controls
            .iter()
            .filter(|c| !c.implemented || !c.validated)
            .cloned()
            .collect()
    }

    /// Get target security level
    pub fn target_security_level(&self) -> Fips1403SecurityLevel {
        self.target_security_level
    }
}

impl Default for Fips1403ComplianceChecker {
    fn default() -> Self {
        Self::new(Fips1403SecurityLevel::Level1)
    }
}

/// FIPS 140-3 approved mode of operation
#[derive(Debug, Clone, Copy)]
pub enum Fips1403ModeOfOperation {
    Ecb,
    Cbc,
    Cfb,
    Ofb,
    Ctr,
    Gcm,
    Ccm,
}

/// Global FIPS 140-3 compliance checker
static FIPS1403_CHECKER: spin::Once<Fips1403ComplianceChecker> = spin::Once::new();

/// Get the global FIPS 140-3 compliance checker
pub fn fips1403_checker() -> &'static Fips1403ComplianceChecker {
    FIPS1403_CHECKER.call_once(|| Fips1403ComplianceChecker::default())
}

/// Calculate FIPS 140-3 compliance score
pub fn calculate_fips1403_compliance() -> f64 {
    let checker = fips1403_checker();
    checker.calculate_compliance()
}

/// Get non-compliant FIPS 140-3 controls
pub fn get_non_compliant_fips1403_controls() -> Vec<Fips1403Control> {
    fips1403_checker().non_compliant_controls()
}