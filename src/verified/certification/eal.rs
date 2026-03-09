//! EAL Certification Module
//! 
//! This module provides EAL (Evaluation Assurance Level) certification
//! capabilities based on Common Criteria for IT Security Evaluation.

use alloc::string::String;

/// EAL level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EalLevel {
    EAL1,
    EAL2,
    EAL3,
    EAL4,
    EAL5,
    EAL6,
    EAL7,
}

/// EAL assurance class
#[derive(Debug, Clone, Copy)]
pub enum EalAssuranceClass {
    ClassAPE, // Protection Profile Evaluation
    ClassASE, // Security Target Evaluation
    ClassADV, // Development
    ClassAGD, // Guidance Documents
    ClassALC, // Life-cycle Support
    ClassATE, // Tests
    ClassAVA, // Vulnerability Assessment
}

/// EAL component family
#[derive(Debug, Clone, Copy)]
pub enum EalComponentFamily {
    APEINT, // Introduction
    ASEINT, // Introduction
    ADVARC, // Architecture
    ADVDES, // Design
    ADVIMP, // Implementation
    ADVINT, // Internal communication
    ADVSPM, // Security policy model
    AGDPRE, // Preparation procedures
    AGDUSE, // User guidance
    ALCDVS, // Development security
    ALCFLR, // Flaw remediation
    ALCCMC, // Configuration management
    ALCTMS, // Tool and measures
    ALCDEL, // Delivery
    ALCLCD, // Life-cycle definition
    ALCCRMS, // Configuration management scope
    ATECOV, // Coverage
    ATEFUN, // Functional testing
    ATEIND, // Independent testing
    ATEIDS, // Depth of testing
    AVAVAN, // Vulnerability survey
    AVAVCA, // Covert channel analysis
    AVAVMS, // Misuse analysis
}

/// EAL control
#[derive(Debug, Clone)]
pub struct EalControl {
    pub component_family: EalComponentFamily,
    pub component_id: String,
    pub component_title: String,
    pub eal_level: EalLevel,
    pub implemented: bool,
    pub evaluated: bool,
    pub evidence_location: Option<String>,
}

impl EalControl {
    /// Create a new EAL control
    pub fn new(
        component_family: EalComponentFamily,
        component_id: impl Into<String>,
        component_title: impl Into<String>,
        eal_level: EalLevel,
    ) -> Self {
        Self {
            component_family,
            component_id: component_id.into(),
            component_title: component_title.into(),
            eal_level,
            implemented: false,
            evaluated: false,
            evidence_location: None,
        }
    }

    /// Mark as implemented
    pub fn mark_implemented(&mut self, evidence: Option<String>) {
        self.implemented = true;
        self.evidence_location = evidence;
    }

    /// Mark as evaluated
    pub fn mark_evaluated(&mut self) {
        self.evaluated = true;
    }
}

/// EAL compliance checker
pub struct EalComplianceChecker {
    controls: alloc::vec::Vec<EalControl>,
    compliance_score: f64,
    target_eal_level: EalLevel,
}

impl EalComplianceChecker {
    /// Create a new EAL compliance checker
    pub fn new(target_level: EalLevel) -> Self {
        let mut checker = Self {
            controls: Vec::new(),
            compliance_score: 0.0,
            target_eal_level: target_level,
        };

        checker.initialize_controls();
        checker
    }

    /// Initialize standard EAL controls
    fn initialize_controls(&mut self) {
        // Class ADV: Development
        self.controls.push(EalControl::new(
            EalComponentFamily::ADVARC,
            "ADV_ARC",
            "Architecture",
            self.target_eal_level,
        ));
        self.controls.push(EalControl::new(
            EalComponentFamily::ADVDES,
            "ADV_DES",
            "Design",
            self.target_eal_level,
        ));
        self.controls.push(EalControl::new(
            EalComponentFamily::ADVIMP,
            "ADV_IMP",
            "Implementation",
            self.target_eal_level,
        ));
        self.controls.push(EalControl::new(
            EalComponentFamily::ADVINT,
            "ADV_INT",
            "Internal communication",
            self.target_eal_level,
        ));
        self.controls.push(EalControl::new(
            EalComponentFamily::ADVSPM,
            "ADV_SPM",
            "Security policy model",
            self.target_eal_level,
        ));

        // Class AGD: Guidance Documents
        self.controls.push(EalControl::new(
            EalComponentFamily::AGDPRE,
            "AGD_PRE",
            "Preparation procedures",
            self.target_eal_level,
        ));
        self.controls.push(EalControl::new(
            EalComponentFamily::AGDUSE,
            "AGD_USE",
            "User guidance",
            self.target_eal_level,
        ));

        // Class ALC: Life-cycle Support
        self.controls.push(EalControl::new(
            EalComponentFamily::ALCDVS,
            "ALC_DVS",
            "Development security",
            self.target_eal_level,
        ));
        self.controls.push(EalControl::new(
            EalComponentFamily::ALCFLR,
            "ALC_FLR",
            "Flaw remediation",
            self.target_eal_level,
        ));
        self.controls.push(EalControl::new(
            EalComponentFamily::ALCCMC,
            "ALC_CMC",
            "Configuration management",
            self.target_eal_level,
        ));

        // Class ATE: Tests
        self.controls.push(EalControl::new(
            EalComponentFamily::ATECOV,
            "ATE_COV",
            "Coverage",
            self.target_eal_level,
        ));
        self.controls.push(EalControl::new(
            EalComponentFamily::ATEFUN,
            "ATE_FUN",
            "Functional testing",
            self.target_eal_level,
        ));
        self.controls.push(EalControl::new(
            EalComponentFamily::ATEIND,
            "ATE_IND",
            "Independent testing",
            self.target_eal_level,
        ));

        // Class AVA: Vulnerability Assessment
        self.controls.push(EalControl::new(
            EalComponentFamily::AVAVAN,
            "AVA_VAN",
            "Vulnerability survey",
            self.target_eal_level,
        ));

        // Add EAL-specific controls based on target level
        if self.target_eal_level as u8 >= EalLevel::EAL4 as u8 {
            self.controls.push(EalControl::new(
                EalComponentFamily::ATEIDS,
                "ATE_IDS",
                "Depth of testing",
                self.target_eal_level,
            ));
        }

        if self.target_eal_level as u8 >= EalLevel::EAL5 as u8 {
            self.controls.push(EalControl::new(
                EalComponentFamily::AVAVCA,
                "AVA_VCA",
                "Covert channel analysis",
                self.target_eal_level,
            ));
        }

        if self.target_eal_level as u8 >= EalLevel::EAL6 as u8 {
            self.controls.push(EalControl::new(
                EalComponentFamily::ALCTMS,
                "ALC_TMS",
                "Tool and measures",
                self.target_eal_level,
            ));
        }

        if self.target_eal_level as u8 >= EalLevel::EAL7 as u8 {
            self.controls.push(EalControl::new(
                EalComponentFamily::AVAVMS,
                "AVA_VMS",
                "Misuse analysis",
                self.target_eal_level,
            ));
        }
    }

    /// Implement a control
    pub fn implement_control(&mut self, component_id: &str, evidence: Option<String>) {
        if let Some(control) = self.controls.iter_mut().find(|c| c.component_id == component_id) {
            control.mark_implemented(evidence);
        }
    }

    /// Evaluate a control
    pub fn evaluate_control(&mut self, component_id: &str) {
        if let Some(control) = self.controls.iter_mut().find(|c| c.component_id == component_id) {
            control.mark_evaluated();
        }
    }

    /// Calculate compliance score
    pub fn calculate_compliance(&mut self) -> f64 {
        let total = self.controls.len();
        if total == 0 {
            return 0.0;
        }

        let implemented = self.controls.iter().filter(|c| c.implemented).count();
        let evaluated = self.controls.iter().filter(|c| c.evaluated).count();

        let implementation_score = (implemented as f64) / (total as f64);
        let evaluation_score = (evaluated as f64) / (total as f64);

        self.compliance_score = (implementation_score * 0.7) + (evaluation_score * 0.3);
        self.compliance_score
    }

    /// Get controls by component family
    pub fn controls_by_family(&self, family: EalComponentFamily) -> Vec<EalControl> {
        self.controls
            .iter()
            .filter(|c| c.component_family == family)
            .cloned()
            .collect()
    }

    /// Get all controls
    pub fn all_controls(&self) -> &[EalControl] {
        &self.controls
    }

    /// Get compliance score
    pub fn compliance_score(&self) -> f64 {
        self.compliance_score
    }

    /// Get non-compliant controls
    pub fn non_compliant_controls(&self) -> Vec<EalControl> {
        self.controls
            .iter()
            .filter(|c| !c.implemented || !c.evaluated)
            .cloned()
            .collect()
    }

    /// Get target EAL level
    pub fn target_eal_level(&self) -> EalLevel {
        self.target_eal_level
    }
}

impl Default for EalComplianceChecker {
    fn default() -> Self {
        Self::new(EalLevel::EAL4)
    }
}

/// EAL requirements summary
#[derive(Debug, Clone)]
pub struct EalRequirements {
    pub eal_level: EalLevel,
    pub has_security_target: bool,
    pub has_protection_profile: bool,
    pub has_formal_model: bool,
    pub has_independent_testing: bool,
    pub has_vulnerability_assessment: bool,
}

impl EalRequirements {
    /// Check if all requirements are met for the target level
    pub fn all_met(&self) -> bool {
        let base_requirements = self.has_security_target
            && self.has_protection_profile
            && self.has_independent_testing
            && self.has_vulnerability_assessment;

        if self.eal_level as u8 >= EalLevel::EAL5 as u8 {
            base_requirements && self.has_formal_model
        } else {
            base_requirements
        }
    }
}

impl Default for EalRequirements {
    fn default() -> Self {
        Self {
            eal_level: EalLevel::EAL4,
            has_security_target: false,
            has_protection_profile: false,
            has_formal_model: false,
            has_independent_testing: false,
            has_vulnerability_assessment: false,
        }
    }
}

/// Global EAL compliance checker
static EAL_CHECKER: spin::Once<EalComplianceChecker> = spin::Once::new();

/// Get the global EAL compliance checker
pub fn eal_checker() -> &'static EalComplianceChecker {
    EAL_CHECKER.call_once(|| EalComplianceChecker::default())
}

/// Calculate EAL compliance score
pub fn calculate_eal_compliance() -> f64 {
    let checker = eal_checker();
    checker.calculate_compliance()
}

/// Get non-compliant EAL controls
pub fn get_non_compliant_eal_controls() -> Vec<EalControl> {
    eal_checker().non_compliant_controls()
}