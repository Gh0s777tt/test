//! PCI DSS Certification Module
//! 
//! This module provides PCI DSS (Payment Card Industry Data Security
//! Standard) certification capabilities.

use alloc::string::String;

/// PCI DSS requirement
#[derive(Debug, Clone, Copy)]
pub enum PciDssRequirement {
    InstallAndMaintainFirewall,
    ChangeDefaultPasswords,
    ProtectStoredCardholderData,
    EncryptTransmissionOfCardholderData,
    UseAndUpdateAntivirusSoftware,
    DevelopAndMaintainSecureSystems,
    RestrictAccessByBusinessNeed,
    IdentifyAndAuthenticateAccess,
    RestrictPhysicalAccess,
    TrackAndMonitorAccess,
    RegularlyTestSecuritySystems,
    MaintainInformationSecurityPolicy,
}

/// PCI DSS control
#[derive(Debug, Clone)]
pub struct PciDssControl {
    pub control_id: String,
    pub control_title: String,
    pub requirement: PciDssRequirement,
    pub implemented: bool,
    pub tested: bool,
    pub evidence_location: Option<String>,
}

impl PciDssControl {
    /// Create a new PCI DSS control
    pub fn new(
        control_id: impl Into<String>,
        control_title: impl Into<String>,
        requirement: PciDssRequirement,
    ) -> Self {
        Self {
            control_id: control_id.into(),
            control_title: control_title.into(),
            requirement,
            implemented: false,
            tested: false,
            evidence_location: None,
        }
    }

    /// Mark as implemented
    pub fn mark_implemented(&mut self, evidence: Option<String>) {
        self.implemented = true;
        self.evidence_location = evidence;
    }

    /// Mark as tested
    pub fn mark_tested(&mut self) {
        self.tested = true;
    }
}

/// PCI DSS compliance checker
pub struct PciDssComplianceChecker {
    controls: alloc::vec::Vec<PciDssControl>,
    compliance_score: f64,
}

impl PciDssComplianceChecker {
    /// Create a new PCI DSS compliance checker
    pub fn new() -> Self {
        let mut checker = Self {
            controls: Vec::new(),
            compliance_score: 0.0,
        };

        checker.initialize_controls();
        checker
    }

    /// Initialize standard PCI DSS controls
    fn initialize_controls(&mut self) {
        // Requirement 1: Install and maintain a firewall configuration
        self.controls.push(PciDssControl::new(
            "1.1",
            "Firewall configuration standards",
            PciDssRequirement::InstallAndMaintainFirewall,
        ));
        self.controls.push(PciDssControl::new(
            "1.2",
            "Restrict connections",
            PciDssRequirement::InstallAndMaintainFirewall,
        ));

        // Requirement 2: Change default passwords
        self.controls.push(PciDssControl::new(
            "2.1",
            "Change vendor defaults",
            PciDssRequirement::ChangeDefaultPasswords,
        ));
        self.controls.push(PciDssControl::new(
            "2.2",
            "System hardening",
            PciDssRequirement::ChangeDefaultPasswords,
        ));

        // Requirement 3: Protect stored cardholder data
        self.controls.push(PciDssControl::new(
            "3.1",
            "Keep cardholder data storage to minimum",
            PciDssRequirement::ProtectStoredCardholderData,
        ));
        self.controls.push(PciDssControl::new(
            "3.4",
            "Render PAN unreadable",
            PciDssRequirement::ProtectStoredCardholderData,
        ));

        // Requirement 4: Encrypt transmission of cardholder data
        self.controls.push(PciDssControl::new(
            "4.1",
            "Encrypt transmission of cardholder data",
            PciDssRequirement::EncryptTransmissionOfCardholderData,
        ));

        // Requirement 5: Use and regularly update anti-virus software
        self.controls.push(PciDssControl::new(
            "5.1",
            "Deploy anti-virus software",
            PciDssRequirement::UseAndUpdateAntivirusSoftware,
        ));

        // Requirement 6: Develop and maintain secure systems
        self.controls.push(PciDssControl::new(
            "6.1",
            "Secure development process",
            PciDssRequirement::DevelopAndMaintainSecureSystems,
        ));
        self.controls.push(PciDssControl::new(
            "6.2",
            "Patch management",
            PciDssRequirement::DevelopAndMaintainSecureSystems,
        ));

        // Requirement 7: Restrict access by business need
        self.controls.push(PciDssControl::new(
            "7.1",
            "Limit access",
            PciDssRequirement::RestrictAccessByBusinessNeed,
        ));
        self.controls.push(PciDssControl::new(
            "7.2",
            "Establish access control systems",
            PciDssRequirement::RestrictAccessByBusinessNeed,
        ));

        // Requirement 8: Identify and authenticate access
        self.controls.push(PciDssControl::new(
            "8.1",
            "Assign unique ID",
            PciDssRequirement::IdentifyAndAuthenticateAccess,
        ));
        self.controls.push(PciDssControl::new(
            "8.2",
            "Authentication mechanisms",
            PciDssRequirement::IdentifyAndAuthenticateAccess,
        ));

        // Requirement 9: Restrict physical access
        self.controls.push(PciDssControl::new(
            "9.1",
            "Physical access controls",
            PciDssRequirement::RestrictPhysicalAccess,
        ));

        // Requirement 10: Track and monitor access
        self.controls.push(PciDssControl::new(
            "10.1",
            "Audit trails",
            PciDssRequirement::TrackAndMonitorAccess,
        ));
        self.controls.push(PciDssControl::new(
            "10.2",
            "Audit trail implementation",
            PciDssRequirement::TrackAndMonitorAccess,
        ));

        // Requirement 11: Regularly test security systems
        self.controls.push(PciDssControl::new(
            "11.1",
            "Test security systems",
            PciDssRequirement::RegularlyTestSecuritySystems,
        ));

        // Requirement 12: Maintain information security policy
        self.controls.push(PciDssControl::new(
            "12.1",
            "Security policy",
            PciDssRequirement::MaintainInformationSecurityPolicy,
        ));
        self.controls.push(PciDssControl::new(
            "12.2",
            "Risk assessment",
            PciDssRequirement::MaintainInformationSecurityPolicy,
        ));
    }

    /// Implement a control
    pub fn implement_control(&mut self, control_id: &str, evidence: Option<String>) {
        if let Some(control) = self.controls.iter_mut().find(|c| c.control_id == control_id) {
            control.mark_implemented(evidence);
        }
    }

    /// Test a control
    pub fn test_control(&mut self, control_id: &str) {
        if let Some(control) = self.controls.iter_mut().find(|c| c.control_id == control_id) {
            control.mark_tested();
        }
    }

    /// Calculate compliance score
    pub fn calculate_compliance(&mut self) -> f64 {
        let total = self.controls.len();
        if total == 0 {
            return 0.0;
        }

        let implemented = self.controls.iter().filter(|c| c.implemented).count();
        let tested = self.controls.iter().filter(|c| c.tested).count();

        let implementation_score = (implemented as f64) / (total as f64);
        let testing_score = (tested as f64) / (total as f64);

        self.compliance_score = (implementation_score * 0.7) + (testing_score * 0.3);
        self.compliance_score
    }

    /// Get controls by requirement
    pub fn controls_by_requirement(&self, requirement: PciDssRequirement) -> Vec<PciDssControl> {
        self.controls
            .iter()
            .filter(|c| c.requirement == requirement)
            .cloned()
            .collect()
    }

    /// Get all controls
    pub fn all_controls(&self) -> &[PciDssControl] {
        &self.controls
    }

    /// Get compliance score
    pub fn compliance_score(&self) -> f64 {
        self.compliance_score
    }

    /// Get non-compliant controls
    pub fn non_compliant_controls(&self) -> Vec<PciDssControl> {
        self.controls
            .iter()
            .filter(|c| !c.implemented || !c.tested)
            .cloned()
            .collect()
    }
}

impl Default for PciDssComplianceChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// PCI DSS SAQ (Self-Assessment Questionnaire) type
#[derive(Debug, Clone, Copy)]
pub enum PciDssSaqType {
    A,
    AEP,
    B,
    BIP,
    C,
    CVT,
    D,
}

/// Global PCI DSS compliance checker
static PCI_DSS_CHECKER: spin::Once<PciDssComplianceChecker> = spin::Once::new();

/// Get the global PCI DSS compliance checker
pub fn pci_dss_checker() -> &'static PciDssComplianceChecker {
    PCI_DSS_CHECKER.call_once(|| PciDssComplianceChecker::new())
}

/// Calculate PCI DSS compliance score
pub fn calculate_pci_dss_compliance() -> f64 {
    let checker = pci_dss_checker();
    checker.calculate_compliance()
}

/// Get non-compliant PCI DSS controls
pub fn get_non_compliant_pci_dss_controls() -> Vec<PciDssControl> {
    pci_dss_checker().non_compliant_controls()
}