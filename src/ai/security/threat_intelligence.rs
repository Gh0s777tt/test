//! Threat Intelligence Integration for AI Security
//!
//! This module provides threat intelligence capabilities to proactively identify
//! and defend against emerging threats targeting AI systems.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Threat intelligence source
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatSource {
    /// MITRE ATT&CK framework
    MitreAttck,
    /// CVE database
    CveDatabase,
    /// AI-specific threat feeds
    AiThreatFeeds,
    /// Internal telemetry
    InternalTelemetry,
    /// Security research communities
    SecurityResearch,
    /// Vendor advisories
    VendorAdvisories,
    /// Open source intelligence
    Osint,
}

/// Threat category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatCategory {
    /// Adversarial attacks
    Adversarial,
    /// Data poisoning
    DataPoisoning,
    /// Model extraction
    ModelExtraction,
    /// Privacy attacks
    PrivacyAttack,
    /// Supply chain attacks
    SupplyChain,
    /// Zero-day exploits
    ZeroDay,
    /// Denial of service
    DenialOfService,
    /// Social engineering
    SocialEngineering,
}

/// Threat severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

/// Threat indicator type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndicatorType {
    /// IP address
    IpAddress,
    /// Domain name
    Domain,
    /// URL
    Url,
    /// File hash
    FileHash,
    /// Email address
    Email,
    /// Certificate fingerprint
    CertificateFingerprint,
    /// Registry key
    RegistryKey,
    /// Process name
    ProcessName,
    /// Network signature
    NetworkSignature,
    /// Model signature
    ModelSignature,
}

/// Threat indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator ID
    pub id: String,
    /// Indicator type
    pub indicator_type: IndicatorType,
    /// Indicator value
    pub value: String,
    /// Threat category
    pub category: ThreatCategory,
    /// Severity
    pub severity: ThreatSeverity,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    /// Source
    pub source: ThreatSource,
    /// First seen
    pub first_seen: chrono::DateTime<chrono::Utc>,
    /// Last seen
    pub last_seen: chrono::DateTime<chrono::Utc>,
    /// Description
    pub description: String,
    /// Mitigation actions
    pub mitigations: Vec<String>,
    /// Related indicators
    pub related_indicators: Vec<String>,
    /// Active flag
    pub is_active: bool,
}

/// Threat actor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatActor {
    /// Actor ID
    pub id: String,
    /// Actor name
    pub name: String,
    /// Aliases
    pub aliases: Vec<String>,
    /// Known motivations
    pub motivations: Vec<String>,
    /// Typical tactics
    pub tactics: Vec<String>,
    /// Countries of origin
    pub origin_countries: Vec<String>,
    /// Known vulnerabilities exploited
    pub exploits: Vec<String>,
}

/// Threat campaign
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatCampaign {
    /// Campaign ID
    pub id: String,
    /// Campaign name
    pub name: String,
    /// Description
    pub description: String,
    /// Associated actors
    pub actors: Vec<String>,
    /// Start date
    pub start_date: chrono::DateTime<chrono::Utc>,
    /// End date (if known)
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    /// Targeted industries
    pub target_industries: Vec<String>,
    /// Targeted regions
    pub target_regions: Vec<String>,
    /// Active flag
    pub is_active: bool,
}

/// Vulnerability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// CVE ID
    pub cve_id: String,
    /// Description
    pub description: String,
    /// CVSS score
    pub cvss_score: Option<f64>,
    /// Severity
    pub severity: ThreatSeverity,
    /// Affected components
    pub affected_components: Vec<String>,
    /// Published date
    pub published_date: chrono::DateTime<chrono::Utc>,
    /// Patch available
    pub patch_available: bool,
    /// Exploit available
    pub exploit_available: bool,
}

/// Threat intelligence configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligenceConfig {
    /// Enable threat intelligence
    pub enabled: bool,
    /// Update interval in seconds
    pub update_interval_secs: u64,
    /// Enable MITRE ATT&CK integration
    pub enable_mitre_attck: bool,
    /// Enable CVE feed
    pub enable_cve_feed: bool,
    /// Enable AI-specific threat feeds
    pub enable_ai_threats: bool,
    /// Minimum confidence threshold
    pub min_confidence: f64,
    /// Retention period for indicators (days)
    pub retention_days: u64,
    /// Enable proactive hunting
    pub enable_proactive_hunting: bool,
    /// API keys for threat feeds
    pub api_keys: HashMap<String, String>,
}

impl Default for ThreatIntelligenceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            update_interval_secs: 3600,
            enable_mitre_attck: true,
            enable_cve_feed: true,
            enable_ai_threats: true,
            min_confidence: 0.6,
            retention_days: 90,
            enable_proactive_hunting: true,
            api_keys: HashMap::new(),
        }
    }
}

/// Threat intelligence statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelStats {
    /// Total indicators
    pub total_indicators: usize,
    /// Active indicators
    pub active_indicators: usize,
    /// Indicators by category
    pub indicators_by_category: HashMap<ThreatCategory, usize>,
    /// Indicators by severity
    pub indicators_by_severity: HashMap<ThreatSeverity, usize>,
    /// Total vulnerabilities tracked
    pub total_vulnerabilities: usize,
    /// Vulnerabilities with exploits
    pub vulnerabilities_with_exploits: usize,
    /// Last update time
    pub last_update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Threats detected this week
    pub threats_detected_this_week: usize,
}

/// Threat intelligence match result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatMatch {
    /// Matched indicator
    pub indicator: ThreatIndicator,
    /// Match confidence
    pub confidence: f64,
    /// Context information
    pub context: HashMap<String, String>,
    /// Recommended actions
    pub actions: Vec<String>,
}

/// Threat Intelligence Manager
pub struct ThreatIntelligenceManager {
    config: ThreatIntelligenceConfig,
    indicators: Arc<RwLock<HashMap<String, ThreatIndicator>>>,
    actors: Arc<RwLock<HashMap<String, ThreatActor>>>,
    campaigns: Arc<RwLock<HashMap<String, ThreatCampaign>>>,
    vulnerabilities: Arc<RwLock<HashMap<String, Vulnerability>>>,
    statistics: Arc<RwLock<ThreatIntelStats>>,
    cache: Arc<RwLock<HashMap<String, Vec<ThreatIndicator>>>>,
}

impl ThreatIntelligenceManager {
    /// Create a new threat intelligence manager
    pub fn new(config: ThreatIntelligenceConfig) -> Self {
        Self {
            config,
            indicators: Arc::new(RwLock::new(HashMap::new())),
            actors: Arc::new(RwLock::new(HashMap::new())),
            campaigns: Arc::new(RwLock::new(HashMap::new())),
            vulnerabilities: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(ThreatIntelStats {
                total_indicators: 0,
                active_indicators: 0,
                indicators_by_category: HashMap::new(),
                indicators_by_severity: HashMap::new(),
                total_vulnerabilities: 0,
                vulnerabilities_with_exploits: 0,
                last_update_time: None,
                threats_detected_this_week: 0,
            })),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize threat intelligence
    pub async fn initialize(&self) -> Result<()> {
        if !self.config.enabled {
            info!("Threat intelligence is disabled");
            return Ok(());
        }

        info!("Initializing threat intelligence system");

        // Load MITRE ATT&CK framework
        if self.config.enable_mitre_attck {
            self.load_mitre_attck().await?;
        }

        // Load known AI threat indicators
        if self.config.enable_ai_threats {
            self.load_ai_threats().await?;
        }

        // Load sample vulnerabilities
        if self.config.enable_cve_feed {
            self.load_cve_feed().await?;
        }

        info!("Threat intelligence initialized with {} indicators", 
              self.indicators.read().await.len());

        Ok(())
    }

    /// Load MITRE ATT&CK framework
    async fn load_mitre_attck(&self) -> Result<()> {
        debug!("Loading MITRE ATT&CK framework data");

        let mut indicators = self.indicators.write().await;

        // Sample MITRE ATT&CK techniques relevant to AI systems
        let attck_techniques = vec![
            ThreatIndicator {
                id: "T1190".to_string(),
                indicator_type: IndicatorType::NetworkSignature,
                value: "exploit_public_facing_application".to_string(),
                category: ThreatCategory::ZeroDay,
                severity: ThreatSeverity::High,
                confidence: 0.95,
                source: ThreatSource::MitreAttck,
                first_seen: chrono::Utc::now() - chrono::Duration::days(365),
                last_seen: chrono::Utc::now(),
                description: "Exploitation of a public-facing application or web server".to_string(),
                mitigations: vec![
                    "Patch vulnerabilities".to_string(),
                    "Implement web application firewall".to_string(),
                    "Network segmentation".to_string(),
                ],
                related_indicators: vec!["T1195".to_string(), "T1059".to_string()],
                is_active: true,
            },
            ThreatIndicator {
                id: "T1059".to_string(),
                indicator_type: IndicatorType::ProcessName,
                value: "command_and_scripting_interpreter".to_string(),
                category: ThreatCategory::SupplyChain,
                severity: ThreatSeverity::Medium,
                confidence: 0.9,
                source: ThreatSource::MitreAttck,
                first_seen: chrono::Utc::now() - chrono::Duration::days(365),
                last_seen: chrono::Utc::now(),
                description: "Command-line interface or scripting languages execution".to_string(),
                mitigations: vec![
                    "Restrict command execution".to_string(),
                    "Monitor process execution".to_string(),
                    "Application whitelisting".to_string(),
                ],
                related_indicators: vec!["T1190".to_string(), "T1566".to_string()],
                is_active: true,
            },
        ];

        for indicator in attck_techniques {
            indicators.insert(indicator.id.clone(), indicator);
        }

        debug!("Loaded {} MITRE ATT&CK techniques", attck_techniques.len());
        Ok(())
    }

    /// Load AI-specific threats
    async fn load_ai_threats(&self) -> Result<()> {
        debug!("Loading AI-specific threat indicators");

        let mut indicators = self.indicators.write().await;

        let ai_threats = vec![
            ThreatIndicator {
                id: "AI-THREAT-001".to_string(),
                indicator_type: IndicatorType::ModelSignature,
                value: "fgsm_attack_pattern".to_string(),
                category: ThreatCategory::Adversarial,
                severity: ThreatSeverity::High,
                confidence: 0.95,
                source: ThreatSource::AiThreatFeeds,
                first_seen: chrono::Utc::now() - chrono::Duration::days(180),
                last_seen: chrono::Utc::now(),
                description: "Fast Gradient Sign Method (FGSM) adversarial attack pattern".to_string(),
                mitigations: vec![
                    "Implement adversarial training".to_string(),
                    "Use input sanitization".to_string(),
                    "Enable feature squeezing".to_string(),
                    "Deploy defensive distillation".to_string(),
                ],
                related_indicators: vec!["AI-THREAT-002".to_string()],
                is_active: true,
            },
            ThreatIndicator {
                id: "AI-THREAT-002".to_string(),
                indicator_type: IndicatorType::ModelSignature,
                value: "pgd_attack_pattern".to_string(),
                category: ThreatCategory::Adversarial,
                severity: ThreatSeverity::High,
                confidence: 0.93,
                source: ThreatSource::AiThreatFeeds,
                first_seen: chrono::Utc::now() - chrono::Duration::days(150),
                last_seen: chrono::Utc::now(),
                description: "Projected Gradient Descent (PGD) adversarial attack pattern".to_string(),
                mitigations: vec![
                    "Implement robust adversarial training".to_string(),
                    "Use random initialization".to_string(),
                    "Apply input preprocessing".to_string(),
                ],
                related_indicators: vec!["AI-THREAT-001".to_string()],
                is_active: true,
            },
            ThreatIndicator {
                id: "AI-THREAT-003".to_string(),
                indicator_type: IndicatorType::ModelSignature,
                value: "model_extraction_pattern".to_string(),
                category: ThreatCategory::ModelExtraction,
                severity: ThreatSeverity::Critical,
                confidence: 0.9,
                source: ThreatSource::AiThreatFeeds,
                first_seen: chrono::Utc::now() - chrono::Duration::days(120),
                last_seen: chrono::Utc::now(),
                description: "Model extraction attack pattern - querying model to reconstruct parameters".to_string(),
                mitigations: vec![
                    "Implement query rate limiting".to_string(),
                    "Add output perturbation".to_string(),
                    "Monitor query patterns".to_string(),
                    "Use differential privacy".to_string(),
                ],
                related_indicators: vec![],
                is_active: true,
            },
            ThreatIndicator {
                id: "AI-THREAT-004".to_string(),
                indicator_type: IndicatorType::ModelSignature,
                value: "data_poisoning_pattern".to_string(),
                category: ThreatCategory::DataPoisoning,
                severity: ThreatSeverity::High,
                confidence: 0.88,
                source: ThreatSource::AiThreatFeeds,
                first_seen: chrono::Utc::now() - chrono::Duration::days(90),
                last_seen: chrono::Utc::now(),
                description: "Data poisoning attack pattern - malicious data injection".to_string(),
                mitigations: vec![
                    "Validate training data".to_string(),
                    "Implement data sanitization".to_string(),
                    "Use robust training algorithms".to_string(),
                    "Monitor training metrics".to_string(),
                ],
                related_indicators: vec!["AI-THREAT-005".to_string()],
                is_active: true,
            },
            ThreatIndicator {
                id: "AI-THREAT-005".to_string(),
                indicator_type: IndicatorType::ModelSignature,
                value: "backdoor_attack_pattern".to_string(),
                category: ThreatCategory::DataPoisoning,
                severity: ThreatSeverity::Critical,
                confidence: 0.92,
                source: ThreatSource::AiThreatFeeds,
                first_seen: chrono::Utc::now() - chrono::Duration::days(60),
                last_seen: chrono::Utc::now(),
                description: "Backdoor attack pattern - hidden triggers in model".to_string(),
                mitigations: vec![
                    "Use robust training with outlier detection".to_string(),
                    "Implement model backdoor scanning".to_string(),
                    "Validate model behavior on clean data".to_string(),
                    "Monitor for suspicious predictions".to_string(),
                ],
                related_indicators: vec!["AI-THREAT-004".to_string()],
                is_active: true,
            },
        ];

        for indicator in ai_threats {
            indicators.insert(indicator.id.clone(), indicator);
        }

        debug!("Loaded {} AI-specific threat indicators", ai_threats.len());
        Ok(())
    }

    /// Load CVE feed
    async fn load_cve_feed(&self) -> Result<()> {
        debug!("Loading CVE vulnerability data");

        let mut vulnerabilities = self.vulnerabilities.write().await;

        // Sample vulnerabilities relevant to AI/ML systems
        let cves = vec![
            Vulnerability {
                cve_id: "CVE-2023-1234".to_string(),
                description: "Buffer overflow in popular ML framework model loading".to_string(),
                cvss_score: Some(8.5),
                severity: ThreatSeverity::High,
                affected_components: vec!["ml_framework_core".to_string()],
                published_date: chrono::Utc::now() - chrono::Duration::days(30),
                patch_available: true,
                exploit_available: false,
            },
            Vulnerability {
                cve_id: "CVE-2023-5678".to_string(),
                description: "Deserialization vulnerability in model format handler".to_string(),
                cvss_score: Some(9.1),
                severity: ThreatSeverity::Critical,
                affected_components: vec!["model_deserializer".to_string()],
                published_date: chrono::Utc::now() - chrono::Duration::days(15),
                patch_available: true,
                exploit_available: true,
            },
            Vulnerability {
                cve_id: "CVE-2023-9012".to_string(),
                description: "SQL injection in AI training data management system".to_string(),
                cvss_score: Some(7.5),
                severity: ThreatSeverity::High,
                affected_components: vec!["training_data_store".to_string()],
                published_date: chrono::Utc::now() - chrono::Duration::days(45),
                patch_available: true,
                exploit_available: false,
            },
        ];

        for cve in cves {
            vulnerabilities.insert(cve.cve_id.clone(), cve);
        }

        debug!("Loaded {} CVE vulnerabilities", cves.len());
        Ok(())
    }

    /// Check if an indicator matches any threat
    pub async fn check_indicator(&self, indicator_type: IndicatorType, value: &str) -> Vec<ThreatMatch> {
        let mut matches = Vec::new();
        let indicators = self.indicators.read().await;

        for indicator in indicators.values() {
            if indicator.indicator_type == indicator_type && indicator.is_active {
                if self.matches_value(indicator, value) {
                    if indicator.confidence >= self.config.min_confidence {
                        let context = self.build_context(indicator);
                        let threat_match = ThreatMatch {
                            indicator: indicator.clone(),
                            confidence: indicator.confidence,
                            context,
                            actions: indicator.mitigations.clone(),
                        };
                        matches.push(threat_match);
                    }
                }
            }
        }

        matches
    }

    /// Check if value matches indicator
    fn matches_value(&self, indicator: &ThreatIndicator, value: &str) -> bool {
        match indicator.indicator_type {
            IndicatorType::IpAddress => indicator.value == value,
            IndicatorType::Domain => value.contains(&indicator.value),
            IndicatorType::Url => value.contains(&indicator.value),
            IndicatorType::FileHash => indicator.value == value,
            IndicatorType::Email => value.contains(&indicator.value),
            IndicatorType::ProcessName => value.contains(&indicator.value),
            IndicatorType::NetworkSignature => value.contains(&indicator.value),
            IndicatorType::ModelSignature => value.contains(&indicator.value),
            _ => false,
        }
    }

    /// Build context for a threat match
    fn build_context(&self, indicator: &ThreatIndicator) -> HashMap<String, String> {
        let mut context = HashMap::new();
        context.insert("category".to_string(), format!("{:?}", indicator.category));
        context.insert("severity".to_string(), format!("{:?}", indicator.severity));
        context.insert("first_seen".to_string(), indicator.first_seen.to_rfc3339());
        context.insert("source".to_string(), format!("{:?}", indicator.source));
        context
    }

    /// Search threats by category
    pub async fn search_by_category(&self, category: ThreatCategory) -> Vec<ThreatIndicator> {
        let indicators = self.indicators.read().await;
        indicators
            .values()
            .filter(|i| i.category == category && i.is_active)
            .cloned()
            .collect()
    }

    /// Search threats by severity
    pub async fn search_by_severity(&self, severity: ThreatSeverity) -> Vec<ThreatIndicator> {
        let indicators = self.indicators.read().await;
        indicators
            .values()
            .filter(|i| i.severity >= severity && i.is_active)
            .cloned()
            .collect()
    }

    /// Get vulnerabilities by component
    pub async fn get_vulnerabilities_by_component(&self, component: &str) -> Vec<Vulnerability> {
        let vulnerabilities = self.vulnerabilities.read().await;
        vulnerabilities
            .values()
            .filter(|v| v.affected_components.iter().any(|c| c.contains(component)))
            .cloned()
            .collect()
    }

    /// Get unpatched vulnerabilities
    pub async fn get_unpatched_vulnerabilities(&self) -> Vec<Vulnerability> {
        let vulnerabilities = self.vulnerabilities.read().await;
        vulnerabilities
            .values()
            .filter(|v| !v.patch_available)
            .cloned()
            .collect()
    }

    /// Get vulnerabilities with exploits
    pub async fn get_exploitable_vulnerabilities(&self) -> Vec<Vulnerability> {
        let vulnerabilities = self.vulnerabilities.read().await;
        vulnerabilities
            .values()
            .filter(|v| v.exploit_available)
            .cloned()
            .collect()
    }

    /// Add custom threat indicator
    pub async fn add_indicator(&self, indicator: ThreatIndicator) -> Result<()> {
        let mut indicators = self.indicators.write().await;
        indicators.insert(indicator.id.clone(), indicator);
        
        self.update_statistics().await;
        debug!("Added custom threat indicator");
        
        Ok(())
    }

    /// Update threat intelligence from sources
    pub async fn update(&self) -> Result<usize> {
        info!("Updating threat intelligence from sources");

        let mut updated_count = 0;

        // In production, fetch updates from external APIs
        // For now, simulate update
        if self.config.enable_cve_feed {
            updated_count += self.update_cve_feed().await?;
        }

        if self.config.enable_ai_threats {
            updated_count += self.update_ai_threats().await?;
        }

        // Update statistics
        self.update_statistics().await;

        // Update last update time
        {
            let mut stats = self.statistics.write().await;
            stats.last_update_time = Some(chrono::Utc::now());
        }

        info!("Threat intelligence updated: {} new indicators", updated_count);
        Ok(updated_count)
    }

    /// Update CVE feed
    async fn update_cve_feed(&self) -> Result<usize> {
        // In production, fetch from CVE API
        Ok(0)
    }

    /// Update AI threats
    async fn update_ai_threats(&self) -> Result<usize> {
        // In production, fetch from AI threat feeds
        Ok(0)
    }

    /// Update statistics
    async fn update_statistics(&self) {
        let indicators = self.indicators.read().await;
        let vulnerabilities = self.vulnerabilities.read().await;
        
        let mut stats = self.statistics.write().await;
        
        stats.total_indicators = indicators.len();
        stats.active_indicators = indicators.values().filter(|i| i.is_active).count();
        
        stats.indicators_by_category.clear();
        for indicator in indicators.values() {
            *stats.indicators_by_category.entry(indicator.category).or_insert(0) += 1;
        }
        
        stats.indicators_by_severity.clear();
        for indicator in indicators.values() {
            *stats.indicators_by_severity.entry(indicator.severity).or_insert(0) += 1;
        }
        
        stats.total_vulnerabilities = vulnerabilities.len();
        stats.vulnerabilities_with_exploits = vulnerabilities.values().filter(|v| v.exploit_available).count();
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> ThreatIntelStats {
        self.statistics.read().await.clone()
    }

    /// Get all active indicators
    pub async fn get_active_indicators(&self) -> Vec<ThreatIndicator> {
        let indicators = self.indicators.read().await;
        indicators.values().filter(|i| i.is_active).cloned().collect()
    }

    /// Deactivate an indicator
    pub async fn deactivate_indicator(&self, id: &str) -> Result<()> {
        let mut indicators = self.indicators.write().await;
        if let Some(indicator) = indicators.get_mut(id) {
            indicator.is_active = false;
            debug!("Deactivated indicator: {}", id);
            return Ok(());
        }
        Err(anyhow::anyhow!("Indicator not found: {}", id))
    }

    /// Get security recommendations based on current threats
    pub async fn get_security_recommendations(&self) -> Vec<String> {
        let indicators = self.indicators.read().await;
        let vulnerabilities = self.vulnerabilities.read().await;
        
        let mut recommendations = Vec::new();

        // Check for critical threats
        let critical_threats: Vec<_> = indicators
            .values()
            .filter(|i| i.is_active && i.severity == ThreatSeverity::Critical)
            .collect();
        
        if !critical_threats.is_empty() {
            recommendations.push(format!(
                "⚠️  {} CRITICAL threats detected - Immediate action required",
                critical_threats.len()
            ));
            
            for threat in critical_threats {
                recommendations.push(format!("  - {}: {}", threat.id, threat.description));
            }
        }

        // Check for unpatched vulnerabilities with exploits
        let exploitable: Vec<_> = vulnerabilities
            .values()
            .filter(|v| v.exploit_available && !v.patch_available)
            .collect();
        
        if !exploitable.is_empty() {
            recommendations.push(format!(
                "🔴 {} vulnerabilities with known exploits - Patch immediately",
                exploitable.len()
            ));
            
            for vuln in exploitable.iter().take(5) {
                recommendations.push(format!("  - {}: {}", vuln.cve_id, vuln.description));
            }
        }

        // General recommendations
        recommendations.push(
            "💡 Enable all security hardening features in production".to_string()
        );
        recommendations.push(
            "💡 Keep all AI frameworks and dependencies updated".to_string()
        );
        recommendations.push(
            "💡 Monitor for adversarial attack patterns in production".to_string()
        );

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_threat_intelligence_initialization() {
        let config = ThreatIntelligenceConfig::default();
        let intel = ThreatIntelligenceManager::new(config);
        
        intel.initialize().await.unwrap();
        
        let stats = intel.get_statistics().await;
        assert!(stats.total_indicators > 0);
    }

    #[tokio::test]
    async fn test_indicator_checking() {
        let config = ThreatIntelligenceConfig::default();
        let intel = ThreatIntelligenceManager::new(config);
        
        intel.initialize().await.unwrap();
        
        let matches = intel.check_indicator(IndicatorType::ModelSignature, "fgsm_attack_pattern").await;
        assert!(!matches.is_empty());
    }

    #[tokio::test]
    async fn test_search_by_category() {
        let config = ThreatIntelligenceConfig::default();
        let intel = ThreatIntelligenceManager::new(config);
        
        intel.initialize().await.unwrap();
        
        let threats = intel.search_by_category(ThreatCategory::Adversarial).await;
        assert!(!threats.is_empty());
    }

    #[tokio::test]
    async fn test_get_recommendations() {
        let config = ThreatIntelligenceConfig::default();
        let intel = ThreatIntelligenceManager::new(config);
        
        intel.initialize().await.unwrap();
        
        let recommendations = intel.get_security_recommendations().await;
        assert!(!recommendations.is_empty());
    }
}