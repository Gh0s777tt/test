//! SOC 2 Certification Module
//! 
//! This module provides SOC 2 Type II service organization control
//! certification capabilities based on Trust Services Criteria.

use alloc::string::String;

/// SOC 2 trust services criteria
#[derive(Debug, Clone, Copy)]
pub enum Soc2TrustCriteria {
    Security,
    Availability,
    ProcessingIntegrity,
    Confidentiality,
    Privacy,
}

/// SOC 2 control
#[derive(Debug, Clone)]
pub struct Soc2Control {
    pub control_id: String,
    pub control_title: String,
    pub criteria: Soc2TrustCriteria,
    pub implemented: bool,
    pub tested: bool,
    pub evidence_location: Option<String>,
}

impl Soc2Control {
    /// Create a new SOC 2 control
    pub fn new(
        control_id: impl Into<String>,
        control_title: impl Into<String>,
        criteria: Soc2TrustCriteria,
    ) -> Self {
        Self {
            control_id: control_id.into(),
            control_title: control_title.into(),
            criteria,
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

/// SOC 2 compliance checker
pub struct Soc2ComplianceChecker {
    controls: alloc::vec::Vec<Soc2Control>,
    compliance_score: f64,
}

impl Soc2ComplianceChecker {
    /// Create a new SOC 2 compliance checker
    pub fn new() -> Self {
        let mut checker = Self {
            controls: Vec::new(),
            compliance_score: 0.0,
        };

        checker.initialize_controls();
        checker
    }

    /// Initialize standard SOC 2 controls
    fn initialize_controls(&mut self) {
        // Security Criteria (CC6.1)
        self.controls.push(Soc2Control::new(
            "CC6.1",
            "Logical and physical access controls",
            Soc2TrustCriteria::Security,
        ));
        self.controls.push(Soc2Control::new(
            "CC6.2",
            "System operations",
            Soc2TrustCriteria::Security,
        ));
        self.controls.push(Soc2Control::new(
            "CC6.3",
            "Change management",
            Soc2TrustCriteria::Security,
        ));

        // Availability Criteria (A1.1)
        self.controls.push(Soc2Control::new(
            "A1.1",
            "Performance and processing monitoring",
            Soc2TrustCriteria::Availability,
        ));
        self.controls.push(Soc2Control::new(
            "A1.2",
            "Recovery time objectives",
            Soc2TrustCriteria::Availability,
        ));

        // Processing Integrity Criteria (PI1.1)
        self.controls.push(Soc2Control::new(
            "PI1.1",
            "Input validation",
            Soc2TrustCriteria::ProcessingIntegrity,
        ));
        self.controls.push(Soc2Control::new(
            "PI1.2",
            "Processing accuracy",
            Soc2TrustCriteria::ProcessingIntegrity,
        ));

        // Confidentiality Criteria (C1.1)
        self.controls.push(Soc2Control::new(
            "C1.1",
            "Data classification",
            Soc2TrustCriteria::Confidentiality,
        ));
        self.controls.push(Soc2Control::new(
            "C1.2",
            "Data access controls",
            Soc2TrustCriteria::Confidentiality,
        ));

        // Privacy Criteria (P1.1)
        self.controls.push(Soc2Control::new(
            "P1.1",
            "Privacy notice",
            Soc2TrustCriteria::Privacy,
        ));
        self.controls.push(Soc2Control::new(
            "P1.2",
            "Consent and choice",
            Soc2TrustCriteria::Privacy,
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

    /// Get controls by criteria
    pub fn controls_by_criteria(&self, criteria: Soc2TrustCriteria) -> Vec<Soc2Control> {
        self.controls
            .iter()
            .filter(|c| c.criteria == criteria)
            .cloned()
            .collect()
    }

    /// Get all controls
    pub fn all_controls(&self) -> &[Soc2Control] {
        &self.controls
    }

    /// Get compliance score
    pub fn compliance_score(&self) -> f64 {
        self.compliance_score
    }

    /// Get non-compliant controls
    pub fn non_compliant_controls(&self) -> Vec<Soc2Control> {
        self.controls
            .iter()
            .filter(|c| !c.implemented || !c.tested)
            .cloned()
            .collect()
    }
}

impl Default for Soc2ComplianceChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// SOC 2 Type II requirements
#[derive(Debug, Clone)]
pub struct Soc2Requirements {
    pub has_security_controls: bool,
    pub has_availability_controls: bool,
    pub has_integrity_controls: bool,
    pub has_confidentiality_controls: bool,
    pub has_privacy_controls: bool,
    pub has_audit_trail: bool,
}

impl Soc2Requirements {
    /// Check if all requirements are met
    pub fn all_met(&self) -> bool {
        self.has_security_controls
            && self.has_availability_controls
            && self.has_integrity_controls
            && self.has_confidentiality_controls
            && self.has_privacy_controls
            && self.has_audit_trail
    }
}

impl Default for Soc2Requirements {
    fn default() -> Self {
        Self {
            has_security_controls: false,
            has_availability_controls: false,
            has_integrity_controls: false,
            has_confidentiality_controls: false,
            has_privacy_controls: false,
            has_audit_trail: false,
        }
    }
}

/// Global SOC 2 compliance checker
static SOC2_CHECKER: spin::Once<Soc2ComplianceChecker> = spin::Once::new();

/// Get the global SOC 2 compliance checker
pub fn soc2_checker() -> &'static Soc2ComplianceChecker {
    SOC2_CHECKER.call_once(|| Soc2ComplianceChecker::new())
}

/// Calculate SOC 2 compliance score
pub fn calculate_soc2_compliance() -> f64 {
    let checker = soc2_checker();
    checker.calculate_compliance()
}

/// Get non-compliant SOC 2 controls
pub fn get_non_compliant_soc2_controls() -> Vec<Soc2Control> {
    soc2_checker().non_compliant_controls()
}