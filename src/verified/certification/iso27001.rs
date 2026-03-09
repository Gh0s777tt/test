//! ISO 27001 Certification Module
//! 
//! This module provides ISO 27001:2022 information security management
//! system certification capabilities.

use alloc::collections::BTreeSet;
use alloc::string::String;

/// ISO 27001 control categories
#[derive(Debug, Clone, Copy)]
pub enum Iso27001ControlCategory {
    InformationSecurityPolicies,
    OrganizationRolesAndResponsibilities,
    HumanResourceSecurity,
    AssetManagement,
    AccessControl,
    Cryptography,
    PhysicalSecurity,
    OperationsSecurity,
    CommunicationsSecurity,
    SystemAcquisitionDevelopmentAndMaintenance,
    SupplierRelationships,
    InformationSecurityIncidentManagement,
    InformationSecurityContinuity,
    Compliance,
}

/// ISO 27001 control
#[derive(Debug, Clone)]
pub struct Iso27001Control {
    pub control_id: String,
    pub control_title: String,
    pub category: Iso27001ControlCategory,
    pub implemented: bool,
    pub tested: bool,
    pub evidence_location: Option<String>,
}

impl Iso27001Control {
    /// Create a new ISO 27001 control
    pub fn new(
        control_id: impl Into<String>,
        control_title: impl Into<String>,
        category: Iso27001ControlCategory,
    ) -> Self {
        Self {
            control_id: control_id.into(),
            control_title: control_title.into(),
            category,
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

/// ISO 27001 compliance checker
pub struct Iso27001ComplianceChecker {
    controls: alloc::vec::Vec<Iso27001Control>,
    compliance_score: f64,
}

impl Iso27001ComplianceChecker {
    /// Create a new ISO 27001 compliance checker
    pub fn new() -> Self {
        let mut checker = Self {
            controls: Vec::new(),
            compliance_score: 0.0,
        };

        // Initialize with standard ISO 27001 controls
        checker.initialize_controls();
        checker
    }

    /// Initialize standard ISO 27001 controls
    fn initialize_controls(&mut self) {
        // Information Security Policies
        self.controls.push(Iso27001Control::new(
            "5.1",
            "Policies for information security",
            Iso27001ControlCategory::InformationSecurityPolicies,
        ));

        // Organization Roles and Responsibilities
        self.controls.push(Iso27001Control::new(
            "5.2",
            "Roles and responsibilities for information security",
            Iso27001ControlCategory::OrganizationRolesAndResponsibilities,
        ));

        // Human Resource Security
        self.controls.push(Iso27001Control::new(
            "6.1",
            "Screening",
            Iso27001ControlCategory::HumanResourceSecurity,
        ));
        self.controls.push(Iso27001Control::new(
            "6.2",
            "Terms and conditions of employment",
            Iso27001ControlCategory::HumanResourceSecurity,
        ));
        self.controls.push(Iso27001Control::new(
            "6.3",
            "Awareness, education and training",
            Iso27001ControlCategory::HumanResourceSecurity,
        ));

        // Asset Management
        self.controls.push(Iso27001Control::new(
            "7.1",
            "Inventory of information and other associated assets",
            Iso27001ControlCategory::AssetManagement,
        ));
        self.controls.push(Iso27001Control::new(
            "7.2",
            "Information classification",
            Iso27001ControlCategory::AssetManagement,
        ));

        // Access Control
        self.controls.push(Iso27001Control::new(
            "8.1",
            "Selection of access control rules",
            Iso27001ControlCategory::AccessControl,
        ));
        self.controls.push(Iso27001Control::new(
            "8.2",
            "Access rights",
            Iso27001ControlCategory::AccessControl,
        ));

        // Cryptography
        self.controls.push(Iso27001Control::new(
            "9.1",
            "Selection and usage of cryptographic controls",
            Iso27001ControlCategory::Cryptography,
        ));

        // Physical Security
        self.controls.push(Iso27001Control::new(
            "10.1",
            "Physical security perimeters",
            Iso27001ControlCategory::PhysicalSecurity,
        ));
        self.controls.push(Iso27001Control::new(
            "10.2",
            "Equipment security",
            Iso27001ControlCategory::PhysicalSecurity,
        ));

        // Operations Security
        self.controls.push(Iso27001Control::new(
            "11.1",
            "Operating procedures",
            Iso27001ControlCategory::OperationsSecurity,
        ));
        self.controls.push(Iso27001Control::new(
            "11.2",
            "Protection from malware",
            Iso27001ControlCategory::OperationsSecurity,
        ));

        // Communications Security
        self.controls.push(Iso27001Control::new(
            "12.1",
            "Network security management",
            Iso27001ControlCategory::CommunicationsSecurity,
        ));

        // System Acquisition
        self.controls.push(Iso27001Control::new(
            "13.1",
            "Secure development lifecycle",
            Iso27001ControlCategory::SystemAcquisitionDevelopmentAndMaintenance,
        ));

        // Supplier Relationships
        self.controls.push(Iso27001Control::new(
            "14.1",
            "Supplier relationship security",
            Iso27001ControlCategory::SupplierRelationships,
        ));

        // Incident Management
        self.controls.push(Iso27001Control::new(
            "15.1",
            "Information security incident management",
            Iso27001ControlCategory::InformationSecurityIncidentManagement,
        ));

        // Continuity
        self.controls.push(Iso27001Control::new(
            "16.1",
            "Information security in business continuity",
            Iso27001ControlCategory::InformationSecurityContinuity,
        ));

        // Compliance
        self.controls.push(Iso27001Control::new(
            "17.1",
            "Identification of applicable legislation and requirements",
            Iso27001ControlCategory::Compliance,
        ));
        self.controls.push(Iso27001Control::new(
            "17.2",
            "Intellectual property rights",
            Iso27001ControlCategory::Compliance,
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

        // Compliance score is based on implementation and testing
        let implementation_score = (implemented as f64) / (total as f64);
        let testing_score = (tested as f64) / (total as f64);

        self.compliance_score = (implementation_score * 0.7) + (testing_score * 0.3);
        self.compliance_score
    }

    /// Get controls by category
    pub fn controls_by_category(&self, category: Iso27001ControlCategory) -> Vec<Iso27001Control> {
        self.controls
            .iter()
            .filter(|c| c.category == category)
            .cloned()
            .collect()
    }

    /// Get all controls
    pub fn all_controls(&self) -> &[Iso27001Control] {
        &self.controls
    }

    /// Get compliance score
    pub fn compliance_score(&self) -> f64 {
        self.compliance_score
    }

    /// Get non-compliant controls
    pub fn non_compliant_controls(&self) -> Vec<Iso27001Control> {
        self.controls
            .iter()
            .filter(|c| !c.implemented || !c.tested)
            .cloned()
            .collect()
    }
}

impl Default for Iso27001ComplianceChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// ISO 27001 certification requirements
#[derive(Debug, Clone)]
pub struct Iso27001Requirements {
    pub has_isms: bool,
    pub has_risk_assessment: bool,
    pub has_statement_of_applicability: bool,
    pub has_internal_audits: bool,
    pub has_management_reviews: bool,
    pub has_continual_improvement: bool,
}

impl Iso27001Requirements {
    /// Check if all requirements are met
    pub fn all_met(&self) -> bool {
        self.has_isms
            && self.has_risk_assessment
            && self.has_statement_of_applicability
            && self.has_internal_audits
            && self.has_management_reviews
            && self.has_continual_improvement
    }
}

impl Default for Iso27001Requirements {
    fn default() -> Self {
        Self {
            has_isms: false,
            has_risk_assessment: false,
            has_statement_of_applicability: false,
            has_internal_audits: false,
            has_management_reviews: false,
            has_continual_improvement: false,
        }
    }
}

/// Global ISO 27001 compliance checker
static ISO27001_CHECKER: spin::Once<Iso27001ComplianceChecker> = spin::Once::new();

/// Get the global ISO 27001 compliance checker
pub fn iso27001_checker() -> &'static Iso27001ComplianceChecker {
    ISO27001_CHECKER.call_once(|| Iso27001ComplianceChecker::new())
}

/// Calculate ISO 27001 compliance score
pub fn calculate_iso27001_compliance() -> f64 {
    let checker = iso27001_checker();
    checker.calculate_compliance()
}

/// Get non-compliant ISO 27001 controls
pub fn get_non_compliant_iso27001_controls() -> Vec<Iso27001Control> {
    iso27001_checker().non_compliant_controls()
}