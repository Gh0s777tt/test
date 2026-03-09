//! AI Compliance Module
//!
//! This module provides comprehensive compliance management for AI systems including:
//! - Regulatory compliance verification (GDPR, HIPAA, SOC2, etc.)
//! - AI transparency and explainability
//! - Decision audit trails
//! - Bias detection and mitigation
//! - Ethics compliance documentation

pub mod regulatory_compliance;
pub mod transparency;
pub mod bias_detection;
pub mod audit_trail;
pub mod ethics;

pub use regulatory_compliance::{
    ComplianceManager, ComplianceFramework, ComplianceStatus, ComplianceReport,
};
pub use transparency::{
    TransparencyManager, ExplanationType, DecisionExplanation,
};
pub use bias_detection::{
    BiasDetector, BiasType, BiasDetectionResult, MitigationStrategy,
};
pub use audit_trail::{
    AuditTrailManager, AuditEntry, AuditQuery,
};
pub use ethics::{
    EthicsManager, EthicalPrinciple, EthicsViolation,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// Enable GDPR compliance
    pub gdpr_enabled: bool,
    /// Enable HIPAA compliance
    pub hipaa_enabled: bool,
    /// Enable SOC 2 compliance
    pub soc2_enabled: bool,
    /// Enable AI Act compliance (EU)
    pub ai_act_enabled: bool,
    /// Enable bias detection
    pub bias_detection_enabled: bool,
    /// Enable explainability
    pub explainability_enabled: bool,
    /// Enable audit trail
    pub audit_trail_enabled: bool,
    /// Retention period for audit logs (days)
    pub audit_retention_days: u64,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            gdpr_enabled: true,
            hipaa_enabled: false,
            soc2_enabled: true,
            ai_act_enabled: true,
            bias_detection_enabled: true,
            explainability_enabled: true,
            audit_trail_enabled: true,
            audit_retention_days: 365,
        }
    }
}

/// Compliance manager for AI systems
pub struct ComplianceSystem {
    config: ComplianceConfig,
    compliance_manager: Arc<ComplianceManager>,
    transparency_manager: Arc<TransparencyManager>,
    bias_detector: Arc<BiasDetector>,
    audit_manager: Arc<AuditTrailManager>,
    ethics_manager: Arc<EthicsManager>,
}

impl ComplianceSystem {
    /// Create a new compliance system
    pub fn new(config: ComplianceConfig) -> Result<Self> {
        info!("Initializing AI Compliance System");

        let compliance_manager = Arc::new(ComplianceManager::new(&config)?);
        let transparency_manager = Arc::new(TransparencyManager::new()?);
        let bias_detector = Arc::new(BiasDetector::new()?);
        let audit_manager = Arc::new(AuditTrailManager::new(config.audit_retention_days)?);
        let ethics_manager = Arc::new(EthicsManager::new()?);

        Ok(Self {
            config,
            compliance_manager,
            transparency_manager,
            bias_detector,
            audit_manager,
            ethics_manager,
        })
    }

    /// Get compliance manager
    pub fn compliance(&self) -> &ComplianceManager {
        &self.compliance_manager
    }

    /// Get transparency manager
    pub fn transparency(&self) -> &TransparencyManager {
        &self.transparency_manager
    }

    /// Get bias detector
    pub fn bias_detector(&self) -> &BiasDetector {
        &self.bias_detector
    }

    /// Get audit manager
    pub fn audit(&self) -> &AuditTrailManager {
        &self.audit_manager
    }

    /// Get ethics manager
    pub fn ethics(&self) -> &EthicsManager {
        &self.ethics_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_system_creation() {
        let config = ComplianceConfig::default();
        let system = ComplianceSystem::new(config);
        assert!(system.is_ok());
    }
}