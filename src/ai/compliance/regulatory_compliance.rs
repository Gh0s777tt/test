//! Regulatory Compliance Management
//!
//! This module provides compliance verification for AI systems against various
//! regulatory frameworks including GDPR, HIPAA, SOC 2, and EU AI Act.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Regulatory compliance frameworks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceFramework {
    /// General Data Protection Regulation (EU)
    GDPR,
    /// Health Insurance Portability and Accountability Act (US)
    HIPAA,
    /// System and Organization Controls 2 (US)
    SOC2,
    /// EU AI Act
    EU_AI_ACT,
    /// California Consumer Privacy Act (US)
    CCPA,
    /// Payment Card Industry Data Security Standard
    PCI_DSS,
    /// ISO 27001
    ISO_27001,
    /// NIST AI Risk Management Framework
    NIST_AI_RM,
}

/// Compliance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    PartiallyCompliant,
    NonCompliant,
    Unknown,
}

/// Compliance requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    /// Requirement ID
    pub id: String,
    /// Framework
    pub framework: ComplianceFramework,
    /// Requirement title
    pub title: String,
    /// Description
    pub description: String,
    /// Compliance status
    pub status: ComplianceStatus,
    /// Evidence of compliance
    pub evidence: Vec<String>,
    /// Last verified
    pub last_verified: chrono::DateTime<chrono::Utc>,
    /// Next review date
    pub next_review: chrono::DateTime<chrono::Utc>,
}

/// Compliance check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    /// Check ID
    pub id: String,
    /// Framework being checked
    pub framework: ComplianceFramework,
    /// Requirement checked
    pub requirement_id: String,
    /// Check timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Result status
    pub status: ComplianceStatus,
    /// Findings
    pub findings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report ID
    pub id: String,
    /// Generated timestamp
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// Framework covered
    pub frameworks: Vec<ComplianceFramework>,
    /// Overall status
    pub overall_status: ComplianceStatus,
    /// Summary
    pub summary: String,
    /// Requirements status
    pub requirements: Vec<ComplianceRequirement>,
    /// Total requirements
    pub total_requirements: usize,
    /// Compliant requirements
    pub compliant_count: usize,
    /// Non-compliant requirements
    pub non_compliant_count: usize,
    /// Partially compliant
    pub partial_count: usize,
}

/// Data processing activity (GDPR Article 30)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessingActivity {
    /// Activity ID
    pub id: String,
    /// Controller name
    pub controller: String,
    /// Data subjects involved
    pub data_subjects: Vec<String>,
    /// Categories of personal data
    pub data_categories: Vec<String>,
    /// Recipients of data
    pub recipients: Vec<String>,
    /// Third country transfers
    pub third_country_transfers: Vec<String>,
    /// Retention period
    pub retention_period: String,
    /// Security measures
    pub security_measures: Vec<String>,
}

/// Risk assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Assessment ID
    pub id: String,
    /// Risk category
    pub category: String,
    /// Risk description
    pub description: String,
    /// Likelihood (1-5)
    pub likelihood: u8,
    /// Impact (1-5)
    pub impact: u8,
    /// Risk score
    pub risk_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Mitigation measures
    pub mitigation_measures: Vec<String>,
    /// Residual risk
    pub residual_risk: f64,
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    fn from_score(score: f64) -> Self {
        match score {
            s if s < 5.0 => RiskLevel::Low,
            s if s < 10.0 => RiskLevel::Medium,
            s if s < 15.0 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }
}

/// Compliance Manager
pub struct ComplianceManager {
    frameworks: Arc<RwLock<HashMap<ComplianceFramework, bool>>>,
    requirements: Arc<RwLock<HashMap<String, ComplianceRequirement>>>,
    checks: Arc<RwLock<Vec<ComplianceCheck>>>,
    data_activities: Arc<RwLock<Vec<DataProcessingActivity>>>,
    risk_assessments: Arc<RwLock<Vec<RiskAssessment>>>,
}

impl ComplianceManager {
    /// Create a new compliance manager
    pub fn new(config: &crate::compliance::ComplianceConfig) -> Result<Self> {
        info!("Initializing Compliance Manager");

        let mut frameworks = HashMap::new();
        frameworks.insert(ComplianceFramework::GDPR, config.gdpr_enabled);
        frameworks.insert(ComplianceFramework::HIPAA, config.hipaa_enabled);
        frameworks.insert(ComplianceFramework::SOC2, config.soc2_enabled);
        frameworks.insert(ComplianceFramework::EU_AI_ACT, config.ai_act_enabled);

        let manager = Self {
            frameworks: Arc::new(RwLock::new(frameworks)),
            requirements: Arc::new(RwLock::new(HashMap::new())),
            checks: Arc::new(RwLock::new(Vec::new())),
            data_activities: Arc::new(RwLock::new(Vec::new())),
            risk_assessments: Arc::new(RwLock::new(Vec::new())),
        };

        Ok(manager)
    }

    /// Initialize compliance requirements
    pub async fn initialize_requirements(&self) {
        let mut requirements = self.requirements.write().await;

        // GDPR Requirements
        requirements.insert(
            "GDPR-01".to_string(),
            ComplianceRequirement {
                id: "GDPR-01".to_string(),
                framework: ComplianceFramework::GDPR,
                title: "Lawful basis for processing".to_string(),
                description: "Personal data must be processed lawfully, fairly, and transparently".to_string(),
                status: ComplianceStatus::Compliant,
                evidence: vec!["Consent management system implemented".to_string()],
                last_verified: chrono::Utc::now(),
                next_review: chrono::Utc::now() + chrono::Duration::days(365),
            },
        );

        requirements.insert(
            "GDPR-02".to_string(),
            ComplianceRequirement {
                id: "GDPR-02".to_string(),
                framework: ComplianceFramework::GDPR,
                title: "Data minimization".to_string(),
                description: "Collect only data necessary for the stated purpose".to_string(),
                status: ComplianceStatus::Compliant,
                evidence: vec!["Data filtering implemented".to_string()],
                last_verified: chrono::Utc::now(),
                next_review: chrono::Utc::now() + chrono::Duration::days(365),
            },
        );

        requirements.insert(
            "GDPR-21".to_string(),
            ComplianceRequirement {
                id: "GDPR-21".to_string(),
                framework: ComplianceFramework::GDPR,
                title: "Right to explanation".to_string(),
                description: "Provide meaningful information about logic involved in automated decision making".to_string(),
                status: ComplianceStatus::PartiallyCompliant,
                evidence: vec![
                    "Explainability module implemented".to_string(),
                    "Decision audit trail active".to_string(),
                ],
                last_verified: chrono::Utc::now(),
                next_review: chrono::Utc::now() + chrono::Duration::days(180),
            },
        );

        // HIPAA Requirements
        requirements.insert(
            "HIPAA-01".to_string(),
            ComplianceRequirement {
                id: "HIPAA-01".to_string(),
                framework: ComplianceFramework::HIPAA,
                title: "Privacy Rule compliance".to_string(),
                description: "Protect individually identifiable health information".to_string(),
                status: ComplianceStatus::Unknown,
                evidence: vec![],
                last_verified: chrono::Utc::now(),
                next_review: chrono::Utc::now() + chrono::Duration::days(365),
            },
        );

        // EU AI Act Requirements
        requirements.insert(
            "AIA-01".to_string(),
            ComplianceRequirement {
                id: "AIA-01".to_string(),
                framework: ComplianceFramework::EU_AI_ACT,
                title: "Risk management system".to_string(),
                description: "Establish and implement a risk management system".to_string(),
                status: ComplianceStatus::Compliant,
                evidence: vec![
                    "Risk assessment module implemented".to_string(),
                    "Mitigation strategies defined".to_string(),
                ],
                last_verified: chrono::Utc::now(),
                next_review: chrono::Utc::now() + chrono::Duration::days(180),
            },
        );

        requirements.insert(
            "AIA-02".to_string(),
            ComplianceRequirement {
                id: "AIA-02".to_string(),
                framework: ComplianceFramework::EU_AI_ACT,
                title: "Data and data governance".to_string(),
                description: "Ensure proper data governance practices".to_string(),
                status: ComplianceStatus::Compliant,
                evidence: vec![
                    "Data quality checks implemented".to_string(),
                    "Bias detection active".to_string(),
                ],
                last_verified: chrono::Utc::now(),
                next_review: chrono::Utc::now() + chrono::Duration::days(180),
            },
        );

        requirements.insert(
            "AIA-03".to_string(),
            ComplianceRequirement {
                id: "AIA-03".to_string(),
                framework: ComplianceFramework::EU_AI_ACT,
                title: "Transparency and provision of information".to_string(),
                description: "Ensure users are informed when interacting with AI systems".to_string(),
                status: ComplianceStatus::Compliant,
                evidence: vec![
                    "Transparency module active".to_string(),
                    "User disclosure implemented".to_string(),
                ],
                last_verified: chrono::Utc::now(),
                next_review: chrono::Utc::now() + chrono::Duration::days(180),
            },
        );

        debug!("Initialized {} compliance requirements", requirements.len());
    }

    /// Check compliance for a specific framework
    pub async fn check_compliance(&self, framework: ComplianceFramework) -> Result<ComplianceCheck> {
        let check_id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now();

        let requirements = self.requirements.read().await;
        let framework_requirements: Vec<_> = requirements
            .values()
            .filter(|r| r.framework == framework)
            .collect();

        let mut findings = Vec::new();
        let mut recommendations = Vec::new();

        for req in &framework_requirements {
            match req.status {
                ComplianceStatus::NonCompliant => {
                    findings.push(format!(
                        "Requirement {} is non-compliant: {}",
                        req.id, req.title
                    ));
                    recommendations.push(format!(
                        "Address non-compliance for {}: {}",
                        req.id, req.title
                    ));
                }
                ComplianceStatus::PartiallyCompliant => {
                    findings.push(format!(
                        "Requirement {} is partially compliant: {}",
                        req.id, req.title
                    ));
                    recommendations.push(format!(
                        "Complete implementation for {}: {}",
                        req.id, req.title
                    ));
                }
                _ => {}
            }
        }

        let overall_status = if framework_requirements.is_empty() {
            ComplianceStatus::Unknown
        } else if framework_requirements
            .iter()
            .all(|r| r.status == ComplianceStatus::Compliant)
        {
            ComplianceStatus::Compliant
        } else if framework_requirements
            .iter()
            .all(|r| r.status != ComplianceStatus::NonCompliant)
        {
            ComplianceStatus::PartiallyCompliant
        } else {
            ComplianceStatus::NonCompliant
        };

        let check = ComplianceCheck {
            id: check_id,
            framework,
            requirement_id: framework_requirements
                .iter()
                .map(|r| r.id.clone())
                .collect::<Vec<_>>()
                .join(","),
            timestamp,
            status: overall_status,
            findings,
            recommendations,
        };

        let mut checks = self.checks.write().await;
        checks.push(check.clone());

        info!(
            "Completed compliance check for {:?}: {:?}",
            framework, overall_status
        );

        Ok(check)
    }

    /// Generate compliance report
    pub async fn generate_report(&self) -> Result<ComplianceReport> {
        let requirements = self.requirements.read().await;
        let all_requirements: Vec<_> = requirements.values().cloned().collect();

        let frameworks: Vec<_> = all_requirements
            .iter()
            .map(|r| r.framework)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let compliant_count = all_requirements
            .iter()
            .filter(|r| r.status == ComplianceStatus::Compliant)
            .count();
        let non_compliant_count = all_requirements
            .iter()
            .filter(|r| r.status == ComplianceStatus::NonCompliant)
            .count();
        let partial_count = all_requirements
            .iter()
            .filter(|r| r.status == ComplianceStatus::PartiallyCompliant)
            .count();

        let overall_status = if non_compliant_count > 0 {
            ComplianceStatus::NonCompliant
        } else if partial_count > 0 {
            ComplianceStatus::PartiallyCompliant
        } else {
            ComplianceStatus::Compliant
        };

        let summary = format!(
            "Compliance Report: {} requirements total, {} compliant, {} partially compliant, {} non-compliant",
            all_requirements.len(),
            compliant_count,
            partial_count,
            non_compliant_count
        );

        Ok(ComplianceReport {
            id: uuid::Uuid::new_v4().to_string(),
            generated_at: chrono::Utc::now(),
            frameworks,
            overall_status,
            summary,
            requirements: all_requirements,
            total_requirements: requirements.len(),
            compliant_count,
            non_compliant_count,
            partial_count,
        })
    }

    /// Register data processing activity (GDPR Article 30)
    pub async fn register_data_activity(&self, activity: DataProcessingActivity) -> Result<()> {
        let mut activities = self.data_activities.write().await;
        activities.push(activity);
        debug!("Registered data processing activity");
        Ok(())
    }

    /// Create risk assessment
    pub async fn create_risk_assessment(
        &self,
        category: String,
        description: String,
        likelihood: u8,
        impact: u8,
        mitigation_measures: Vec<String>,
    ) -> Result<RiskAssessment> {
        let risk_score = (likelihood as f64) * (impact as f64);
        let risk_level = RiskLevel::from_score(risk_score);

        // Calculate residual risk (simplified - typically 50% reduction with mitigation)
        let residual_risk = risk_score * 0.5;

        let assessment = RiskAssessment {
            id: uuid::Uuid::new_v4().to_string(),
            category,
            description,
            likelihood,
            impact,
            risk_score,
            risk_level,
            mitigation_measures,
            residual_risk,
        };

        let mut assessments = self.risk_assessments.write().await;
        assessments.push(assessment.clone());

        info!(
            "Created risk assessment: {} (score: {:.1}, level: {:?})",
            assessment.id, risk_score, risk_level
        );

        Ok(assessment)
    }

    /// Get requirements for a specific framework
    pub async fn get_framework_requirements(
        &self,
        framework: ComplianceFramework,
    ) -> Vec<ComplianceRequirement> {
        let requirements = self.requirements.read().await;
        requirements
            .values()
            .filter(|r| r.framework == framework)
            .cloned()
            .collect()
    }

    /// Get high-risk assessments
    pub async fn get_high_risk_assessments(&self) -> Vec<RiskAssessment> {
        let assessments = self.risk_assessments.read().await;
        assessments
            .iter()
            .filter(|r| r.risk_level == RiskLevel::High || r.risk_level == RiskLevel::Critical)
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compliance_manager_creation() {
        let config = crate::compliance::ComplianceConfig::default();
        let manager = ComplianceManager::new(&config);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_gdpr_compliance_check() {
        let config = crate::compliance::ComplianceConfig::default();
        let manager = ComplianceManager::new(&config).unwrap();
        manager.initialize_requirements().await;
        
        let result = manager.check_compliance(ComplianceFramework::GDPR).await.unwrap();
        assert!(matches!(result.status, ComplianceStatus::PartiallyCompliant | ComplianceStatus::Compliant));
    }

    #[tokio::test]
    async fn test_risk_assessment() {
        let config = crate::compliance::ComplianceConfig::default();
        let manager = ComplianceManager::new(&config).unwrap();
        
        let assessment = manager
            .create_risk_assessment(
                "Data Privacy".to_string(),
                "Unauthorized data access".to_string(),
                4,
                5,
                vec!["Encryption".to_string(), "Access controls".to_string()],
            )
            .await
            .unwrap();
        
        assert!(assessment.risk_score > 0.0);
        assert!(matches!(assessment.risk_level, RiskLevel::High | RiskLevel::Critical));
    }
}