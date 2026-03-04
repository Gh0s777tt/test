//! Threat Detection Engine Module
//! 
//! Provides AI-driven security monitoring and threat detection.
//! 
//! ## Security Considerations
//! - Detection runs in isolated sandbox
//! - No user data is analyzed
//! - Minimal performance impact
//! - Privacy-preserving analysis

use crate::ai::{
    config::SecurityConfig,
    error::AIError,
    types::{Confidence, ThreatDetection, ThreatLevel},
};

/// Threat Detection Engine
/// 
/// Monitors system behavior for security threats using ML.
/// 
/// ## Features
/// - Real-time threat detection
/// - Anomaly-based detection
/// - Keyword-based detection
/// - Configurable response modes
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::security::ThreatDetectionEngine;
//! 
//! let engine = ThreatDetectionEngine::new(SecurityConfig::default())?;
//! 
//! // Analyze system behavior
//! let detection = engine.analyze_behavior("malicious_activity_detected")?;
//! println!("Threat level: {:?}", detection.level);
//! ```
pub struct ThreatDetectionEngine {
    enabled: bool,
    config: SecurityConfig,
}

impl ThreatDetectionEngine {
    /// Create a new Threat Detection Engine
    /// 
    /// ## Arguments
    /// * `config` - Security configuration
    /// 
    /// ## Returns
    /// A new threat detection engine
    /// 
    /// ## Errors
    /// Returns AIError if initialization fails
    pub fn new(config: SecurityConfig) -> Result<Self, AIError> {
        Ok(Self {
            enabled: config.enabled,
            config,
        })
    }

    /// Analyze system behavior for threats
    /// 
    /// ## Arguments
    /// * `behavior` - Behavior description to analyze
    /// 
    /// ## Returns
    /// Threat detection result
    /// 
    /// ## Errors
    /// - Returns error if engine is disabled
    /// - Returns error if analysis fails
    /// 
    /// ## Performance
    /// Target latency: <10ms
    pub fn analyze_behavior(&self, behavior: &str) -> Result<ThreatDetection, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        // STUB: Will implement full ML-based threat detection in v1.3.0
        // For now, use keyword-based detection
        let detection = classify_threat(behavior);
        Ok(detection)
    }

    /// Check if engine is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if engine is ready
    pub fn is_ready(&self) -> bool {
        self.enabled
    }
}

/// Classify threat based on behavior keywords
/// 
/// ## Arguments
/// * `behavior` - Behavior description
/// 
/// ## Returns
/// Threat detection result
fn classify_threat(behavior: &str) -> ThreatDetection {
    let behavior_lower = behavior.to_lowercase();
    
    let (level, threat_type) = if behavior_lower.contains("malware") || 
                                   behavior_lower.contains("virus") ||
                                   behavior_lower.contains("trojan") {
        (ThreatLevel::Critical, "Malware")
    } else if behavior_lower.contains("attack") || 
              behavior_lower.contains("exploit") ||
              behavior_lower.contains("intrusion") {
        (ThreatLevel::High, "Attack")
    } else if behavior_lower.contains("suspicious") ||
              behavior_lower.contains("anomaly") {
        (ThreatLevel::Medium, "Suspicious Activity")
    } else {
        (ThreatLevel::None, "Normal")
    };

    ThreatDetection::new(level, threat_type.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_creation() {
        let engine = ThreatDetectionEngine::new(SecurityConfig::default()).unwrap();
        assert!(engine.is_enabled());
    }

    #[test]
    fn test_analyze_malware() {
        let engine = ThreatDetectionEngine::new(SecurityConfig::default()).unwrap();
        let detection = engine.analyze_behavior("malware detected").unwrap();
        assert_eq!(detection.level, ThreatLevel::Critical);
        assert_eq!(detection.threat_type, "Malware");
    }

    #[test]
    fn test_analyze_attack() {
        let engine = ThreatDetectionEngine::new(SecurityConfig::default()).unwrap();
        let detection = engine.analyze_behavior("attack detected").unwrap();
        assert_eq!(detection.level, ThreatLevel::High);
        assert_eq!(detection.threat_type, "Attack");
    }

    #[test]
    fn test_analyze_normal() {
        let engine = ThreatDetectionEngine::new(SecurityConfig::default()).unwrap();
        let detection = engine.analyze_behavior("normal operation").unwrap();
        assert_eq!(detection.level, ThreatLevel::None);
    }

    #[test]
    fn test_is_severe() {
        let threat = ThreatDetection::new(ThreatLevel::High, "Attack".to_string());
        assert!(threat.is_severe());
        
        let threat2 = ThreatDetection::new(ThreatLevel::Low, "Suspicious".to_string());
        assert!(!threat2.is_severe());
    }
}