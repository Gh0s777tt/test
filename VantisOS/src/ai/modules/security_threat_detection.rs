//! AI-Powered Security Threat Detection Module
//!
//! Advanced security system that uses machine learning to detect,
//  analyze, and respond to security threats in real-time.

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for security threat detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Threat detection sensitivity (0.0 - 1.0)
    pub sensitivity: f64,
    
    /// False positive tolerance (0.0 - 1.0)
    pub false_positive_tolerance: f64,
    
    /// Number of historical events to analyze
    pub history_size: usize,
    
    /// Time window for anomaly detection (seconds)
    pub anomaly_window_seconds: u64,
    
    /// Enable behavioral analysis
    pub enable_behavioral_analysis: bool,
    
    /// Enable signature-based detection
    pub enable_signature_detection: bool,
    
    /// Enable ML-based detection
    pub enable_ml_detection: bool,
    
    /// Auto-quarantine threshold
    pub auto_quarantine_threshold: f64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            sensitivity: 0.8,
            false_positive_tolerance: 0.1,
            history_size: 10000,
            anomaly_window_seconds: 300, // 5 minutes
            enable_behavioral_analysis: true,
            enable_signature_detection: true,
            enable_ml_detection: true,
            auto_quarantine_threshold: 0.9,
        }
    }
}

/// Threat severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatSeverity {
    Info = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Threat category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatCategory {
    Malware,
    Phishing,
    Intrusion,
    DataExfiltration,
    PrivilegeEscalation,
    DoS,
    Anomaly,
    Unknown,
}

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_id: String,
    pub timestamp: SystemTime,
    pub event_type: String,
    pub source: String,
    pub target: Option<String>,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, String>,
}

/// Detected threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threat {
    pub threat_id: String,
    pub category: ThreatCategory,
    pub severity: ThreatSeverity,
    pub confidence: f64,
    pub detected_at: SystemTime,
    pub related_events: Vec<String>,
    pub description: String,
    pub recommended_action: RecommendedAction,
}

/// Recommended action for a threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendedAction {
    Monitor,
    Quarantine,
    Block,
    Alert,
    Investigate,
    Terminate,
}

/// Signature for known threats
#[derive(Debug, Clone)]
struct ThreatSignature {
    pattern: Vec<u8>,
    severity: ThreatSeverity,
    category: ThreatCategory,
}

/// Anomaly detection model
#[derive(Debug)]
struct AnomalyModel {
    baseline_metrics: HashMap<String, f64>,
    thresholds: HashMap<String, f64>,
    accuracy_history: VecDeque<f64>,
}

impl AnomalyModel {
    fn new() -> Self {
        Self {
            baseline_metrics: HashMap::new(),
            thresholds: HashMap::new(),
            accuracy_history: VecDeque::with_capacity(100),
        }
    }
    
    /// Train model on normal behavior
    fn train(&mut self, events: &[SecurityEvent]) {
        for event in events {
            let key = format!("{}:{}", event.event_type, event.source);
            *self.baseline_metrics.entry(key).or_insert(0.0) += 1.0;
        }
        
        // Calculate thresholds (2 standard deviations)
        for (_, count) in &self.baseline_metrics {
            *self.thresholds.entry("default".to_string()).or_insert(0.0) = count * 2.0;
        }
    }
    
    /// Detect anomaly in event
    fn detect(&self, event: &SecurityEvent) -> Option<f64> {
        let key = format!("{}:{}", event.event_type, event.source);
        let baseline = *self.baseline_metrics.get(&key).unwrap_or(&0.0);
        let threshold = *self.thresholds.get("default").unwrap_or(&10.0);
        
        if baseline > 0.0 {
            let deviation = (baseline - 0.0) / threshold;
            if deviation > 1.0 {
                return Some((deviation - 1.0).min(1.0));
            }
        }
        None
    }
    
    /// Update model accuracy
    fn update_accuracy(&mut self, correct: bool) {
        let acc = if self.accuracy_history.is_empty() {
            if correct { 1.0 } else { 0.0 }
        } else {
            let last = *self.accuracy_history.back().unwrap();
            let new = if correct {
                (last * 0.95) + 0.05
            } else {
                last * 0.95
            };
            new
        };
        self.accuracy_history.push_back(acc);
        if self.accuracy_history.len() > 100 {
            self.accuracy_history.pop_front();
        }
    }
}

/// Security threat detector
pub struct ThreatDetector {
    config: SecurityConfig,
    event_history: Arc<RwLock<VecDeque<SecurityEvent>>>,
    known_signatures: Arc<RwLock<Vec<ThreatSignature>>>,
    anomaly_model: Arc<RwLock<AnomalyModel>>,
    active_threats: Arc<RwLock<HashMap<String, Threat>>>,
    event_counter: Arc<RwLock<usize>>,
}

impl ThreatDetector {
    /// Create a new threat detector
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config,
            event_history: Arc::new(RwLock::new(VecDeque::with_capacity(
                config.history_size,
            ))),
            known_signatures: Arc::new(RwLock::new(Vec::new())),
            anomaly_model: Arc::new(RwLock::new(AnomalyModel::new())),
            active_threats: Arc::new(RwLock::new(HashMap::new())),
            event_counter: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Process a security event
    pub async fn process_event(&self, event: SecurityEvent) -> Vec<Threat> {
        let mut detected_threats = Vec::new();
        
        // Add to history
        self.add_to_history(event.clone()).await;
        
        // Signature-based detection
        if self.config.enable_signature_detection {
            if let Some(threat) = self.signature_detection(&event).await {
                detected_threats.push(threat);
            }
        }
        
        // Anomaly-based detection
        if self.config.enable_behavioral_analysis {
            if let Some(threat) = self.anomaly_detection(&event).await {
                detected_threats.push(threat);
            }
        }
        
        // ML-based detection
        if self.config.enable_ml_detection {
            if let Some(threat) = self.ml_detection(&event).await {
                detected_threats.push(threat);
            }
        }
        
        // Update active threats
        for threat in &detected_threats {
            self.active_threats
                .write()
                .await
                .insert(threat.threat_id.clone(), threat.clone());
        }
        
        detected_threats
    }
    
    /// Add event to history
    async fn add_to_history(&self, event: SecurityEvent) {
        let mut history = self.event_history.write().await;
        history.push_back(event);
        
        // Maintain size limit
        while history.len() > self.config.history_size {
            history.pop_front();
        }
        
        // Train model periodically
        *self.event_counter.write().await += 1;
        if *self.event_counter.read().await % 100 == 0 {
            let events: Vec<SecurityEvent> = history.iter().cloned().collect();
            let mut model = self.anomaly_model.write().await;
            model.train(&events);
        }
    }
    
    /// Signature-based threat detection
    async fn signature_detection(&self, event: &SecurityEvent) -> Option<Threat> {
        let signatures = self.known_signatures.read().await;
        
        for signature in signatures.iter() {
            if self.match_signature(&event.data, &signature.pattern) {
                return Some(Threat {
                    threat_id: format!("sig_{}", uuid::Uuid::new_v4()),
                    category: signature.category,
                    severity: signature.severity,
                    confidence: 0.95,
                    detected_at: SystemTime::now(),
                    related_events: vec![event.event_id.clone()],
                    description: "Known threat signature detected".to_string(),
                    recommended_action: RecommendedAction::Quarantine,
                });
            }
        }
        
        None
    }
    
    /// Anomaly-based threat detection
    async fn anomaly_detection(&self, event: &SecurityEvent) -> Option<Threat> {
        let model = self.anomaly_model.read().await;
        
        if let Some(anomaly_score) = model.detect(event) {
            if anomaly_score > self.config.sensitivity {
                let severity = if anomaly_score > 0.9 {
                    ThreatSeverity::Critical
                } else if anomaly_score > 0.7 {
                    ThreatSeverity::High
                } else {
                    ThreatSeverity::Medium
                };
                
                return Some(Threat {
                    threat_id: format!("ano_{}", uuid::Uuid::new_v4()),
                    category: ThreatCategory::Anomaly,
                    severity,
                    confidence: anomaly_score,
                    detected_at: SystemTime::now(),
                    related_events: vec![event.event_id.clone()],
                    description: format!("Anomalous behavior detected (score: {:.2})", anomaly_score),
                    recommended_action: RecommendedAction::Monitor,
                });
            }
        }
        
        None
    }
    
    /// ML-based threat detection
    async fn ml_detection(&self, event: &SecurityEvent) -> Option<Threat> {
        // Simulate ML-based detection
        // In production, this would use trained models
        let is_malicious = self.is_malicious_pattern(event);
        
        if is_malicious {
            return Some(Threat {
                threat_id: format!("ml_{}", uuid::Uuid::new_v4()),
                category: ThreatCategory::Unknown,
                severity: ThreatSeverity::High,
                confidence: 0.85,
                detected_at: SystemTime::now(),
                related_events: vec![event.event_id.clone()],
                description: "ML model detected suspicious activity".to_string(),
                recommended_action: RecommendedAction::Investigate,
            });
        }
        
        None
    }
    
    /// Check if event matches a known signature
    fn match_signature(&self, data: &[u8], pattern: &[u8]) -> bool {
        if pattern.len() > data.len() {
            return false;
        }
        
        for i in 0..=data.len().saturating_sub(pattern.len()) {
            if data[i..i.saturating_add(pattern.len())] == *pattern {
                return true;
            }
        }
        
        false
    }
    
    /// Check if event shows malicious pattern
    fn is_malicious_pattern(&self, event: &SecurityEvent) -> bool {
        // Simplified ML-based detection
        // In production, use actual ML models
        event.metadata.contains_key("suspicious") || 
        event.event_type.contains("attack")
    }
    
    /// Add a known threat signature
    pub async fn add_signature(&self, pattern: Vec<u8>, severity: ThreatSeverity, category: ThreatCategory) {
        let signature = ThreatSignature {
            pattern,
            severity,
            category,
        };
        
        let mut signatures = self.known_signatures.write().await;
        signatures.push(signature);
    }
    
    /// Get active threats
    pub async fn get_active_threats(&self) -> Vec<Threat> {
        let threats = self.active_threats.read().await;
        threats.values().cloned().collect()
    }
    
    /// Resolve a threat
    pub async fn resolve_threat(&self, threat_id: &str) {
        self.active_threats.write().await.remove(threat_id);
    }
    
    /// Get security statistics
    pub async fn get_stats(&self) -> SecurityStats {
        let history = self.event_history.read().await;
        let threats = self.active_threats.read().await;
        let model = self.anomaly_model.read().await;
        
        SecurityStats {
            total_events_processed: *self.event_counter.read().await,
            events_in_history: history.len(),
            active_threats: threats.len(),
            known_signatures: self.known_signatures.read().await.len(),
            model_accuracy: model.accuracy_history.back().copied().unwrap_or(0.5),
        }
    }
}

/// Security statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStats {
    pub total_events_processed: usize,
    pub events_in_history: usize,
    pub active_threats: usize,
    pub known_signatures: usize,
    pub model_accuracy: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_event() {
        let config = SecurityConfig::default();
        let detector = ThreatDetector::new(config);
        
        let event = SecurityEvent {
            event_id: "evt1".to_string(),
            timestamp: SystemTime::now(),
            event_type: "file_access".to_string(),
            source: "process1".to_string(),
            target: Some("/path/to/file".to_string()),
            data: vec![1, 2, 3],
            metadata: HashMap::new(),
        };
        
        let threats = detector.process_event(event).await;
        // No threats should be detected initially
        assert_eq!(threats.len(), 0);
    }

    #[tokio::test]
    async fn test_signature_detection() {
        let config = SecurityConfig::default();
        let detector = ThreatDetector::new(config);
        
        // Add a signature
        detector.add_signature(
            vec![0x4D, 0x5A], // MZ header
            ThreatSeverity::High,
            ThreatCategory::Malware,
        ).await;
        
        let event = SecurityEvent {
            event_id: "evt1".to_string(),
            timestamp: SystemTime::now(),
            event_type: "file_scan".to_string(),
            source: "scanner".to_string(),
            target: Some("malware.exe".to_string()),
            data: vec![0x4D, 0x5A, 0x90, 0x00], // PE file header
            metadata: HashMap::new(),
        };
        
        let threats = detector.process_event(event).await;
        assert_eq!(threats.len(), 1);
        assert_eq!(threats[0].category, ThreatCategory::Malware);
    }

    #[tokio::test]
    async fn test_get_stats() {
        let config = SecurityConfig::default();
        let detector = ThreatDetector::new(config);
        
        let stats = detector.get_stats().await;
        assert_eq!(stats.total_events_processed, 0);
    }
}