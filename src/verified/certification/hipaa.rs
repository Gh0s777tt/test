//! HIPAA Certification Module
//! 
//! This module provides HIPAA (Health Insurance Portability and Accountability
//! Act) certification capabilities including Privacy Rule and Security Rule.

use alloc::string::String;

/// HIPAA rule type
#[derive(Debug, Clone, Copy)]
pub enum HipaaRule {
    PrivacyRule,
    SecurityRule,
    BreachNotificationRule,
    EnforcementRule,
}

/// HIPAA safeguard
#[derive(Debug, Clone, Copy)]
pub enum HipaaSafeguard {
    Administrative,
    Physical,
    Technical,
}

/// HIPAA control
#[derive(Debug, Clone)]
pub struct HipaaControl {
    pub control_id: String,
    pub control_title: String,
    pub rule: HipaaRule,
    pub safeguard: HipaaSafeguard,
    pub implemented: bool,
    pub tested: bool,
    pub evidence_location: Option<String>,
}

impl HipaaControl {
    /// Create a new HIPAA control
    pub fn new(
        control_id: impl Into<String>,
        control_title: impl Into<String>,
        rule: HipaaRule,
        safeguard: HipaaSafeguard,
    ) -> Self {
        Self {
            control_id: control_id.into(),
            control_title: control_title.into(),
            rule,
            safeguard,
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

/// HIPAA compliance checker
pub struct HipaaComplianceChecker {
    controls: alloc::vec::Vec<HipaaControl>,
    compliance_score: f64,
}

impl HipaaComplianceChecker {
    /// Create a new HIPAA compliance checker
    pub fn new() -> Self {
        let mut checker = Self {
            controls: Vec::new(),
            compliance_score: 0.0,
        };

        checker.initialize_controls();
        checker
    }

    /// Initialize standard HIPAA controls
    fn initialize_controls(&mut self) {
        // Privacy Rule - Administrative Safeguards
        self.controls.push(HipaaControl::new(
            "PR.164.308(a)(1)",
            "Security management process",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Administrative,
        ));
        self.controls.push(HipaaControl::new(
            "PR.164.308(a)(2)",
            "Assigned security responsibility",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Administrative,
        ));
        self.controls.push(HipaaControl::new(
            "PR.164.308(a)(3)",
            "Workforce security",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Administrative,
        ));

        // Security Rule - Physical Safeguards
        self.controls.push(HipaaControl::new(
            "SR.164.310(a)(1)",
            "Facility access controls",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Physical,
        ));
        self.controls.push(HipaaControl::new(
            "SR.164.310(b)",
            "Workstation use",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Physical,
        ));
        self.controls.push(HipaaControl::new(
            "SR.164.310(c)",
            "Workstation security",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Physical,
        ));

        // Security Rule - Technical Safeguards
        self.controls.push(HipaaControl::new(
            "SR.164.312(a)(1)",
            "Access control",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Technical,
        ));
        self.controls.push(HipaaControl::new(
            "SR.164.312(a)(2)",
            "Audit controls",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Technical,
        ));
        self.controls.push(HipaaControl::new(
            "SR.164.312(a)(3)",
            "Integrity controls",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Technical,
        ));
        self.controls.push(HipaaControl::new(
            "SR.164.312(a)(4)",
            "Transmission security",
            HipaaRule::SecurityRule,
            HipaaSafeguard::Technical,
        ));

        // Breach Notification Rule
        self.controls.push(HipaaControl::new(
            "BNR.164.400",
            "Breach notification",
            HipaaRule::BreachNotificationRule,
            HipaaSafeguard::Administrative,
        ));

        // Privacy Rule
        self.controls.push(HipaaControl::new(
            "PR.164.502(a)",
            "Uses and disclosures",
            HipaaRule::PrivacyRule,
            HipaaSafeguard::Administrative,
        ));
        self.controls.push(HipaaControl::new(
            "PR.164.512",
            "Uses and disclosures for which authorization is not required",
            HipaaRule::PrivacyRule,
            HipaaSafeguard::Administrative,
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

    /// Get controls by rule
    pub fn controls_by_rule(&self, rule: HipaaRule) -> Vec<HipaaControl> {
        self.controls
            .iter()
            .filter(|c| c.rule == rule)
            .cloned()
            .collect()
    }

    /// Get all controls
    pub fn all_controls(&self) -> &[HipaaControl] {
        &self.controls
    }

    /// Get compliance score
    pub fn compliance_score(&self) -> f64 {
        self.compliance_score
    }

    /// Get non-compliant controls
    pub fn non_compliant_controls(&self) -> Vec<HipaaControl> {
        self.controls
            .iter()
            .filter(|c| !c.implemented || !c.tested)
            .cloned()
            .collect()
    }
}

impl Default for HipaaComplianceChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// HIPAA required safeguards
#[derive(Debug, Clone)]
pub struct HipaaRequiredSafeguards {
    pub has_administrative_safeguards: bool,
    pub has_physical_safeguards: bool,
    pub has_technical_safeguards: bool,
    pub has_breach_notification_procedure: bool,
    pub has_privacy_policy: bool,
}

impl HipaaRequiredSafeguards {
    /// Check if all safeguards are in place
    pub fn all_present(&self) -> bool {
        self.has_administrative_safeguards
            && self.has_physical_safeguards
            && self.has_technical_safeguards
            && self.has_breach_notification_procedure
            && self.has_privacy_policy
    }
}

impl Default for HipaaRequiredSafeguards {
    fn default() -> Self {
        Self {
            has_administrative_safeguards: false,
            has_physical_safeguards: false,
            has_technical_safeguards: false,
            has_breach_notification_procedure: false,
            has_privacy_policy: false,
        }
    }
}

/// Global HIPAA compliance checker
static HIPAA_CHECKER: spin::Once<HipaaComplianceChecker> = spin::Once::new();

/// Get the global HIPAA compliance checker
pub fn hipaa_checker() -> &'static HipaaComplianceChecker {
    HIPAA_CHECKER.call_once(|| HipaaComplianceChecker::new())
}

/// Calculate HIPAA compliance score
pub fn calculate_hipaa_compliance() -> f64 {
    let checker = hipaa_checker();
    checker.calculate_compliance()
}

/// Get non-compliant HIPAA controls
pub fn get_non_compliant_hipaa_controls() -> Vec<HipaaControl> {
    hipaa_checker().non_compliant_controls()
}