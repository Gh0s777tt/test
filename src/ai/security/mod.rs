//! Security Hardening Module
//!
//! This module provides comprehensive security features for AI systems including:
//! - Adversarial defense mechanisms
//! - Model poisoning detection
//! - Model encryption at rest
//! - Federated learning security
//! - Secure inference pipeline
//! - Differential privacy
//! - Runtime security monitoring
//! - Threat intelligence integration

pub mod adversarial_defense;
pub mod model_poisoning_detection;
pub mod model_encryption;
pub mod federated_learning_security;
pub mod secure_inference;
pub mod differential_privacy;
pub mod runtime_monitoring;
pub mod threat_intelligence;

pub use adversarial_defense::{
    AdversarialDefenseConfig, AdversarialDefenseManager, AttackType, DefenseStrategy,
};
pub use model_poisoning_detection::{
    ModelPoisoningDetector, PoisoningAttackType, PoisoningDetectionResult,
};
pub use model_encryption::{
    EncryptionAlgorithm, ModelEncryptionConfig, ModelEncryptionManager,
};
pub use federated_learning_security::{
    FederatedLearningSecurityManager, FederatedLearningSecurityConfig,
};
pub use secure_inference::{
    SecureInferenceConfig, SecureInferencePipeline,
};
pub use differential_privacy::{
    DifferentialPrivacyConfig, DifferentialPrivacyManager, NoiseDistribution, PrivacyMechanism,
};
pub use runtime_monitoring::{
    RuntimeSecurityMonitor, RuntimeMonitoringConfig, AlertConfig, SecurityEvent, Severity,
    SecurityEventCategory, AnomalyType,
};
pub use threat_intelligence::{
    ThreatIntelligenceManager, ThreatIntelligenceConfig, ThreatSource, ThreatCategory,
    ThreatSeverity, IndicatorType, ThreatIndicator, Vulnerability,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Security configuration for AI systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable adversarial defense
    pub adversarial_defense: AdversarialDefenseConfig,
    /// Enable model poisoning detection
    pub poisoning_detection: bool,
    /// Enable model encryption
    pub model_encryption: ModelEncryptionConfig,
    /// Enable federated learning security
    pub federated_learning_security: bool,
    /// Enable secure inference
    pub secure_inference: SecureInferenceConfig,
    /// Enable differential privacy
    pub differential_privacy: DifferentialPrivacyConfig,
    /// Enable runtime monitoring
    pub runtime_monitoring: RuntimeMonitoringConfig,
    /// Enable threat intelligence
    pub threat_intelligence: ThreatIntelligenceConfig,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            adversarial_defense: AdversarialDefenseConfig::default(),
            poisoning_detection: true,
            model_encryption: ModelEncryptionConfig::default(),
            federated_learning_security: true,
            secure_inference: SecureInferenceConfig::default(),
            differential_privacy: DifferentialPrivacyConfig::default(),
            runtime_monitoring: RuntimeMonitoringConfig::default(),
            threat_intelligence: ThreatIntelligenceConfig::default(),
        }
    }
}

/// Security incident
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncident {
    /// Unique incident ID
    pub id: String,
    /// Incident timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Incident type
    pub incident_type: ThreatType,
    /// Severity level
    pub severity: Severity,
    /// Description
    pub description: String,
    /// Affected components
    pub affected_components: Vec<String>,
    /// Mitigation actions taken
    pub actions_taken: Vec<String>,
    /// Resolved flag
    pub resolved: bool,
}

/// Type of security threat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatType {
    AdversarialAttack,
    DataPoisoning,
    ModelExtraction,
    PrivacyViolation,
    UnauthorizedAccess,
    ResourceAbuse,
    SystemIntrusion,
    Unknown,
}

/// Overall security manager that coordinates all security components
pub struct SecurityManager {
    config: SecurityConfig,
    adversarial_defense: Arc<AdversarialDefenseManager>,
    poisoning_detector: Arc<ModelPoisoningDetector>,
    encryption_manager: Arc<ModelEncryptionManager>,
    federated_security: Option<Arc<FederatedLearningSecurityManager>>,
    secure_inference: Arc<SecureInferencePipeline>,
    differential_privacy: Arc<DifferentialPrivacyManager>,
    runtime_monitor: Arc<RuntimeSecurityMonitor>,
    threat_intel: Arc<ThreatIntelligenceManager>,
    incidents: Arc<RwLock<Vec<SecurityIncident>>>,
}

impl SecurityManager {
    /// Create a new security manager
    pub fn new(config: SecurityConfig) -> Result<Self> {
        info!("Initializing AI Security Manager");

        let adversarial_defense = Arc::new(AdversarialDefenseManager::new(
            config.adversarial_defense.clone(),
        )?);

        let poisoning_detector = Arc::new(ModelPoisoningDetector::new()?);

        let encryption_manager = Arc::new(ModelEncryptionManager::new(
            config.model_encryption.clone(),
        )?);

        let federated_security = if config.federated_learning_security {
            Some(Arc::new(FederatedLearningSecurityManager::new(
                FederatedLearningSecurityConfig::default(),
            )))
        } else {
            None
        };

        let secure_inference = Arc::new(SecureInferencePipeline::new(
            config.secure_inference.clone(),
        ));

        let differential_privacy = Arc::new(DifferentialPrivacyManager::new(
            config.differential_privacy.clone(),
        ));

        let runtime_monitor = Arc::new(RuntimeSecurityMonitor::new(
            config.runtime_monitoring.clone(),
            AlertConfig::default(),
        ));

        let threat_intel = Arc::new(ThreatIntelligenceManager::new(
            config.threat_intelligence.clone(),
        ));

        Ok(Self {
            config,
            adversarial_defense,
            poisoning_detector,
            encryption_manager,
            federated_security,
            secure_inference,
            differential_privacy,
            runtime_monitor,
            threat_intel,
            incidents: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Initialize all security components
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing security components");

        // Initialize threat intelligence first
        self.threat_intel.initialize().await?;

        // Start runtime monitoring
        self.runtime_monitor.start().await?;

        info!("Security manager initialized successfully");
        Ok(())
    }

    /// Get adversarial defense manager
    pub fn adversarial_defense(&self) -> &AdversarialDefenseManager {
        &self.adversarial_defense
    }

    /// Get poisoning detector
    pub fn poisoning_detector(&self) -> &ModelPoisoningDetector {
        &self.poisoning_detector
    }

    /// Get encryption manager
    pub fn encryption_manager(&self) -> &ModelEncryptionManager {
        &self.encryption_manager
    }

    /// Get federated learning security manager
    pub fn federated_security(&self) -> Option<&FederatedLearningSecurityManager> {
        self.federated_security.as_deref()
    }

    /// Get secure inference pipeline
    pub fn secure_inference(&self) -> &SecureInferencePipeline {
        &self.secure_inference
    }

    /// Get differential privacy manager
    pub fn differential_privacy(&self) -> &DifferentialPrivacyManager {
        &self.differential_privacy
    }

    /// Get runtime monitor
    pub fn runtime_monitor(&self) -> &RuntimeSecurityMonitor {
        &self.runtime_monitor
    }

    /// Get threat intelligence manager
    pub fn threat_intel(&self) -> &ThreatIntelligenceManager {
        &self.threat_intel
    }

    /// Record a security incident
    pub async fn record_incident(&self, incident: SecurityIncident) -> Result<()> {
        let mut incidents = self.incidents.write().await;
        incidents.push(incident);
        Ok(())
    }

    /// Get security incidents
    pub async fn get_incidents(&self) -> Vec<SecurityIncident> {
        self.incidents.read().await.clone()
    }

    /// Get security status summary
    pub async fn get_security_status(&self) -> SecurityStatus {
        let stats = self.runtime_monitor.get_statistics().await;
        let threat_stats = self.threat_intel.get_statistics().await;

        SecurityStatus {
            events_monitored: stats.total_events,
            active_threats: threat_stats.active_indicators,
            vulnerabilities_tracked: threat_stats.total_vulnerabilities,
            last_threat_update: threat_stats.last_update_time,
            monitoring_active: self.config.runtime_monitoring.enabled,
            threat_intel_active: self.config.threat_intelligence.enabled,
        }
    }
}

/// Security status summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatus {
    /// Number of security events monitored
    pub events_monitored: usize,
    /// Number of active threats tracked
    pub active_threats: usize,
    /// Number of vulnerabilities being tracked
    pub vulnerabilities_tracked: usize,
    /// Last threat intelligence update
    pub last_threat_update: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether monitoring is active
    pub monitoring_active: bool,
    /// Whether threat intelligence is active
    pub threat_intel_active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_manager_creation() {
        let config = SecurityConfig::default();
        let manager = SecurityManager::new(config).unwrap();
        manager.initialize().await.unwrap();
    }

    #[tokio::test]
    async fn test_security_status() {
        let config = SecurityConfig::default();
        let manager = SecurityManager::new(config).unwrap();
        manager.initialize().await.unwrap();
        
        let status = manager.get_security_status().await;
        assert!(status.monitoring_active);
    }
}