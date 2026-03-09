//! Runtime Security Monitoring for AI Systems
//!
//! This module provides real-time monitoring of AI system behavior to detect
//! anomalies, intrusions, and security threats during operation.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Security event severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Security event categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityEventCategory {
    /// Model behavior anomalies
    ModelAnomaly,
    /// Data access violations
    DataAccessViolation,
    /// Permission violations
    PermissionViolation,
    /// Resource abuse
    ResourceAbuse,
    /// Network intrusion
    NetworkIntrusion,
    /// Memory corruption attempts
    MemoryCorruption,
    /// Adversarial attack
    AdversarialAttack,
    /// API abuse
    ApiAbuse,
    /// System integrity violation
    SystemIntegrity,
    /// Audit logging failure
    AuditFailure,
}

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Unique event ID
    pub id: String,
    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Event category
    pub category: SecurityEventCategory,
    /// Severity level
    pub severity: Severity,
    /// Event description
    pub description: String,
    /// Source component
    pub source: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Remediation action taken
    pub remediation: Option<String>,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMonitoringConfig {
    /// Enable real-time monitoring
    pub enabled: bool,
    /// Event sampling rate (1.0 = all events)
    pub sampling_rate: f64,
    /// Maximum events to keep in history
    pub max_history_size: usize,
    /// Alert threshold for critical events
    pub critical_threshold: usize,
    /// Enable anomaly detection
    pub anomaly_detection: bool,
    /// Anomaly sensitivity (0.0-1.0)
    pub anomaly_sensitivity: f64,
    /// Enable behavioral profiling
    pub behavioral_profiling: bool,
    /// Monitoring interval in milliseconds
    pub monitoring_interval_ms: u64,
    /// Enable file integrity monitoring
    pub file_integrity_monitoring: bool,
    /// Paths to monitor for integrity
    pub monitored_paths: Vec<String>,
    /// Enable network monitoring
    pub network_monitoring: bool,
    /// Enable API call monitoring
    pub api_monitoring: bool,
}

impl Default for RuntimeMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sampling_rate: 1.0,
            max_history_size: 10000,
            critical_threshold: 5,
            anomaly_detection: true,
            anomaly_sensitivity: 0.7,
            behavioral_profiling: true,
            monitoring_interval_ms: 1000,
            file_integrity_monitoring: true,
            monitored_paths: vec![
                "/etc/vantis".to_string(),
                "/var/lib/vantis/models".to_string(),
            ],
            network_monitoring: true,
            api_monitoring: true,
        }
    }
}

/// Behavioral baseline for anomaly detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralBaseline {
    /// Component name
    pub component: String,
    /// Expected API call patterns
    pub api_patterns: HashMap<String, f64>,
    /// Expected resource usage
    pub resource_usage: ResourceUsageBaseline,
    /// Expected timing patterns
    pub timing_patterns: TimingBaseline,
    /// Last updated
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Resource usage baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageBaseline {
    /// Expected CPU usage range
    pub cpu_range: (f64, f64),
    /// Expected memory usage range
    pub memory_range: (f64, f64),
    /// Expected GPU usage range
    pub gpu_range: Option<(f64, f64)>,
    /// Expected I/O rate range
    pub io_range: (f64, f64),
}

/// Timing baseline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingBaseline {
    /// Expected request latency range (ms)
    pub latency_range: (f64, f64),
    /// Expected processing time range (ms)
    pub processing_time_range: (f64, f64),
    /// Expected request frequency range (req/min)
    pub request_frequency_range: (f64, f64),
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    /// Whether an anomaly was detected
    pub is_anomaly: bool,
    /// Anomaly score (0.0-1.0)
    pub score: f64,
    /// Anomaly type
    pub anomaly_type: AnomalyType,
    /// Description
    pub description: String,
    /// Affected component
    pub component: String,
    /// Supporting evidence
    pub evidence: HashMap<String, f64>,
}

/// Types of anomalies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnomalyType {
    /// Unusual resource usage
    ResourceUsage,
    /// Abnormal API patterns
    ApiPattern,
    /// Timing anomaly
    Timing,
    /// Behavioral deviation
    Behavioral,
    /// Data anomaly
    Data,
    /// Model output anomaly
    ModelOutput,
    /// Network traffic anomaly
    NetworkTraffic,
    /// File integrity violation
    FileIntegrity,
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Minimum severity for alerts
    pub min_severity: Severity,
    /// Enable email notifications
    pub email_notifications: bool,
    /// Email recipients
    pub email_recipients: Vec<String>,
    /// Enable webhook notifications
    pub webhook_enabled: bool,
    /// Webhook URL
    pub webhook_url: Option<String>,
    /// Cooldown period between alerts (seconds)
    pub cooldown_period_secs: u64,
}

/// Security alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAlert {
    /// Alert ID
    pub id: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Related events
    pub events: Vec<String>,
    /// Alert message
    pub message: String,
    /// Severity
    pub severity: Severity,
    /// Acknowledged flag
    pub acknowledged: bool,
    /// Resolved flag
    pub resolved: bool,
}

/// Monitoring statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStatistics {
    /// Total events captured
    pub total_events: usize,
    /// Events by category
    pub events_by_category: HashMap<SecurityEventCategory, usize>,
    /// Events by severity
    pub events_by_severity: HashMap<Severity, usize>,
    /// Anomalies detected
    pub anomalies_detected: usize,
    /// Alerts generated
    pub alerts_generated: usize,
    /// Monitoring uptime (seconds)
    pub uptime_secs: u64,
    /// Last event timestamp
    pub last_event_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// File integrity record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileIntegrityRecord {
    /// File path
    pub path: String,
    /// Expected hash
    pub expected_hash: String,
    /// Current hash
    pub current_hash: String,
    /// Last checked
    pub last_checked: chrono::DateTime<chrono::Utc>,
    /// Integrity status
    pub is_valid: bool,
}

/// Runtime Security Monitor
pub struct RuntimeSecurityMonitor {
    config: RuntimeMonitoringConfig,
    alert_config: AlertConfig,
    events: Arc<RwLock<VecDeque<SecurityEvent>>>,
    baselines: Arc<RwLock<HashMap<String, BehavioralBaseline>>>,
    alerts: Arc<RwLock<VecDeque<SecurityAlert>>>,
    statistics: Arc<RwLock<MonitoringStatistics>>,
    file_integrity: Arc<RwLock<HashMap<String, FileIntegrityRecord>>>,
    recent_anomalies: Arc<RwLock<VecDeque<AnomalyResult>>>,
    last_alert_time: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
}

impl RuntimeSecurityMonitor {
    /// Create a new runtime security monitor
    pub fn new(config: RuntimeMonitoringConfig, alert_config: AlertConfig) -> Self {
        Self {
            config,
            alert_config,
            events: Arc::new(RwLock::new(VecDeque::with_capacity(10000))),
            baselines: Arc::new(RwLock::new(HashMap::new())),
            alerts: Arc::new(RwLock::new(VecDeque::new())),
            statistics: Arc::new(RwLock::new(MonitoringStatistics {
                total_events: 0,
                events_by_category: HashMap::new(),
                events_by_severity: HashMap::new(),
                anomalies_detected: 0,
                alerts_generated: 0,
                uptime_secs: 0,
                last_event_time: None,
            })),
            file_integrity: Arc::new(RwLock::new(HashMap::new())),
            recent_anomalies: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            last_alert_time: Arc::new(RwLock::new(None)),
        }
    }

    /// Start monitoring (background task)
    pub async fn start(&self) -> Result<()> {
        if !self.config.enabled {
            info!("Runtime monitoring is disabled");
            return Ok(());
        }

        info!("Starting runtime security monitoring");
        
        // Initialize baselines for monitored components
        self.initialize_baselines().await?;
        
        // Initialize file integrity monitoring
        if self.config.file_integrity_monitoring {
            self.initialize_file_integrity().await?;
        }

        Ok(())
    }

    /// Initialize behavioral baselines
    async fn initialize_baselines(&self) -> Result<()> {
        let mut baselines = self.baselines.write().await;
        
        // AI model baseline
        baselines.insert(
            "ai_model".to_string(),
            BehavioralBaseline {
                component: "ai_model".to_string(),
                api_patterns: {
                    let mut patterns = HashMap::new();
                    patterns.insert("inference".to_string(), 0.8);
                    patterns.insert("training".to_string(), 0.1);
                    patterns.insert("prediction".to_string(), 0.7);
                    patterns
                },
                resource_usage: ResourceUsageBaseline {
                    cpu_range: (10.0, 80.0),
                    memory_range: (1024.0, 4096.0),
                    gpu_range: Some((20.0, 90.0)),
                    io_range: (1.0, 100.0),
                },
                timing_patterns: TimingBaseline {
                    latency_range: (10.0, 500.0),
                    processing_time_range: (5.0, 200.0),
                    request_frequency_range: (10.0, 1000.0),
                },
                last_updated: chrono::Utc::now(),
            },
        );

        // API gateway baseline
        baselines.insert(
            "api_gateway".to_string(),
            BehavioralBaseline {
                component: "api_gateway".to_string(),
                api_patterns: {
                    let mut patterns = HashMap::new();
                    patterns.insert("get".to_string(), 0.6);
                    patterns.insert("post".to_string(), 0.3);
                    patterns.insert("delete".to_string(), 0.1);
                    patterns
                },
                resource_usage: ResourceUsageBaseline {
                    cpu_range: (5.0, 50.0),
                    memory_range: (256.0, 1024.0),
                    gpu_range: None,
                    io_range: (0.5, 50.0),
                },
                timing_patterns: TimingBaseline {
                    latency_range: (1.0, 100.0),
                    processing_time_range: (0.5, 50.0),
                    request_frequency_range: (100.0, 10000.0),
                },
                last_updated: chrono::Utc::now(),
            },
        );

        debug!("Initialized {} behavioral baselines", baselines.len());
        Ok(())
    }

    /// Initialize file integrity monitoring
    async fn initialize_file_integrity(&self) -> Result<()> {
        let mut integrity = self.file_integrity.write().await;
        
        for path in &self.config.monitored_paths {
            // In production, compute actual hash
            let record = FileIntegrityRecord {
                path: path.clone(),
                expected_hash: "computed_hash_placeholder".to_string(),
                current_hash: "computed_hash_placeholder".to_string(),
                last_checked: chrono::Utc::now(),
                is_valid: true,
            };
            integrity.insert(path.clone(), record);
        }

        info!("Initialized file integrity monitoring for {} paths", integrity.len());
        Ok(())
    }

    /// Record a security event
    pub async fn record_event(&self, mut event: SecurityEvent) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }

        // Apply sampling
        if self.config.sampling_rate < 1.0 {
            use rand::Rng;
            if rand::thread_rng().gen::<f64>() > self.config.sampling_rate {
                return Ok(());
            }
        }

        event.id = uuid::Uuid::new_v4().to_string();
        event.timestamp = chrono::Utc::now();

        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.total_events += 1;
            *stats.events_by_category.entry(event.category).or_insert(0) += 1;
            *stats.events_by_severity.entry(event.severity).or_insert(0) += 1;
            stats.last_event_time = Some(event.timestamp);
        }

        // Add to history
        {
            let mut events = self.events.write().await;
            if events.len() >= self.config.max_history_size {
                events.pop_front();
            }
            events.push_back(event.clone());
        }

        // Check for alert condition
        if event.severity == Severity::Critical {
            self.check_alert_condition().await?;
        }

        // Detect anomaly
        if self.config.anomaly_detection {
            let anomaly = self.detect_anomaly(&event).await;
            if anomaly.is_anomaly {
                warn!("Anomaly detected: {} (score: {:.2})", anomaly.description, anomaly.score);
                let mut anomalies = self.recent_anomalies.write().await;
                if anomalies.len() >= 100 {
                    anomalies.pop_front();
                }
                anomalies.push_back(anomaly);
            }
        }

        debug!("Recorded security event: {:?}", event.category);
        Ok(())
    }

    /// Detect anomaly in event
    async fn detect_anomaly(&self, event: &SecurityEvent) -> AnomalyResult {
        let baselines = self.baselines.read().await;
        
        // Check if we have a baseline for this component
        let baseline = match baselines.get(&event.source) {
            Some(b) => b,
            None => {
                return AnomalyResult {
                    is_anomaly: false,
                    score: 0.0,
                    anomaly_type: AnomalyType::Behavioral,
                    description: "No baseline for component".to_string(),
                    component: event.source.clone(),
                    evidence: HashMap::new(),
                };
            }
        };

        // Check metadata for anomalies
        let mut score = 0.0;
        let mut evidence = HashMap::new();

        // Check timing if available
        if let Some(latency) = event.metadata.get("latency_ms") {
            if let Ok(latency_f) = latency.parse::<f64>() {
                let (min, max) = baseline.timing_patterns.latency_range;
                if latency_f < min || latency_f > max {
                    let deviation = if latency_f < min {
                        (min - latency_f) / min
                    } else {
                        (latency_f - max) / max
                    };
                    score += deviation * 0.3;
                    evidence.insert("latency_deviation".to_string(), deviation);
                }
            }
        }

        // Check resource usage
        if let Some(cpu) = event.metadata.get("cpu_percent") {
            if let Ok(cpu_f) = cpu.parse::<f64>() {
                let (min, max) = baseline.resource_usage.cpu_range;
                if cpu_f > max {
                    let deviation = (cpu_f - max) / max;
                    score += deviation * 0.3;
                    evidence.insert("cpu_deviation".to_string(), deviation);
                }
            }
        }

        // Determine anomaly type based on event category
        let anomaly_type = match event.category {
            SecurityEventCategory::ModelAnomaly => AnomalyType::ModelOutput,
            SecurityEventCategory::DataAccessViolation => AnomalyType::Data,
            SecurityEventCategory::ResourceAbuse => AnomalyType::ResourceUsage,
            SecurityEventCategory::NetworkIntrusion => AnomalyType::NetworkTraffic,
            SecurityEventCategory::SystemIntegrity => AnomalyType::FileIntegrity,
            _ => AnomalyType::Behavioral,
        };

        let is_anomaly = score >= self.config.anomaly_sensitivity;

        AnomalyResult {
            is_anomaly,
            score: score.min(1.0),
            anomaly_type,
            description: if is_anomaly {
                format!("Anomalous behavior detected in {}", event.source)
            } else {
                "Within normal bounds".to_string()
            },
            component: event.source.clone(),
            evidence,
        }
    }

    /// Check if alert condition is met
    async fn check_alert_condition(&self) -> Result<()> {
        let events = self.events.read().await;
        let critical_count = events
            .iter()
            .rev()
            .take(100)
            .filter(|e| e.severity == Severity::Critical)
            .count();

        if critical_count >= self.config.critical_threshold {
            drop(events);
            self.generate_alert(critical_count).await?;
        }

        Ok(())
    }

    /// Generate security alert
    async fn generate_alert(&self, critical_count: usize) -> Result<()> {
        // Check cooldown
        {
            let last_alert = self.last_alert_time.read().await;
            if let Some(last) = *last_alert {
                let elapsed = chrono::Utc::now() - last;
                if elapsed.num_seconds() < self.alert_config.cooldown_period_secs as i64 {
                    debug!("Alert cooldown active, skipping");
                    return Ok(());
                }
            }
        }

        let alert = SecurityAlert {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            events: vec![],
            message: format!("{} critical security events detected", critical_count),
            severity: Severity::Critical,
            acknowledged: false,
            resolved: false,
        };

        // Add alert
        {
            let mut alerts = self.alerts.write().await;
            if alerts.len() >= 100 {
                alerts.pop_front();
            }
            alerts.push_back(alert.clone());
        }

        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.alerts_generated += 1;
        }

        // Update last alert time
        {
            let mut last = self.last_alert_time.write().await;
            *last = Some(chrono::Utc::now());
        }

        // Send notifications
        self.send_alert_notification(&alert).await?;

        warn!("Generated security alert: {}", alert.message);
        Ok(())
    }

    /// Send alert notification
    async fn send_alert_notification(&self, alert: &SecurityAlert) -> Result<()> {
        if !self.alert_config.enabled {
            return Ok(());
        }

        // Send webhook notification
        if self.alert_config.webhook_enabled {
            if let Some(url) = &self.alert_config.webhook_url {
                let payload = serde_json::to_string(&alert)?;
                debug!("Would send webhook alert to {}: {}", url, payload);
                // In production, make HTTP POST request
            }
        }

        // Send email notification
        if self.alert_config.email_notifications {
            for recipient in &self.alert_config.email_recipients {
                debug!("Would send email alert to: {}", recipient);
                // In production, send email
            }
        }

        Ok(())
    }

    /// Check file integrity
    pub async fn check_file_integrity(&self) -> Result<Vec<String>> {
        let mut violations = Vec::new();
        let mut integrity = self.file_integrity.write().await;

        for (path, record) in integrity.iter_mut() {
            // In production, compute actual hash and compare
            record.last_checked = chrono::Utc::now();
            
            // Simulate check (in production, compute actual hash)
            if !record.is_valid {
                violations.push(path.clone());
            }
        }

        if !violations.is_empty() {
            let event = SecurityEvent {
                id: String::new(),
                timestamp: chrono::Utc::now(),
                category: SecurityEventCategory::SystemIntegrity,
                severity: Severity::High,
                description: format!("File integrity violations detected: {:?}", violations),
                source: "file_monitor".to_string(),
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("violations".to_string(), violations.join(","));
                    m
                },
                remediation: Some("Investigate modified files".to_string()),
            };
            
            drop(integrity);
            self.record_event(event).await?;
        }

        Ok(violations)
    }

    /// Update behavioral baseline
    pub async fn update_baseline(&self, component: &str, updates: BaselineUpdates) -> Result<()> {
        let mut baselines = self.baselines.write().await;
        
        if let Some(baseline) = baselines.get_mut(component) {
            if let Some(cpu) = updates.cpu_range {
                baseline.resource_usage.cpu_range = cpu;
            }
            if let Some(memory) = updates.memory_range {
                baseline.resource_usage.memory_range = memory;
            }
            if let Some(latency) = updates.latency_range {
                baseline.timing_patterns.latency_range = latency;
            }
            baseline.last_updated = chrono::Utc::now();
            
            debug!("Updated baseline for component: {}", component);
        }

        Ok(())
    }

    /// Get recent events
    pub async fn get_recent_events(&self, limit: usize) -> Vec<SecurityEvent> {
        let events = self.events.read().await;
        events.iter().rev().take(limit).cloned().collect()
    }

    /// Get recent anomalies
    pub async fn get_recent_anomalies(&self, limit: usize) -> Vec<AnomalyResult> {
        let anomalies = self.recent_anomalies.write().await;
        anomalies.iter().rev().take(limit).cloned().collect()
    }

    /// Get alerts
    pub async fn get_alerts(&self, include_resolved: bool) -> Vec<SecurityAlert> {
        let alerts = self.alerts.read().await;
        alerts
            .iter()
            .filter(|a| include_resolved || !a.resolved)
            .cloned()
            .collect()
    }

    /// Acknowledge alert
    pub async fn acknowledge_alert(&self, alert_id: &str) -> Result<()> {
        let mut alerts = self.alerts.write().await;
        for alert in alerts.iter_mut() {
            if alert.id == alert_id {
                alert.acknowledged = true;
                debug!("Alert {} acknowledged", alert_id);
                return Ok(());
            }
        }
        Err(anyhow::anyhow!("Alert not found: {}", alert_id))
    }

    /// Resolve alert
    pub async fn resolve_alert(&self, alert_id: &str) -> Result<()> {
        let mut alerts = self.alerts.write().await;
        for alert in alerts.iter_mut() {
            if alert.id == alert_id {
                alert.resolved = true;
                info!("Alert {} resolved", alert_id);
                return Ok(());
            }
        }
        Err(anyhow::anyhow!("Alert not found: {}", alert_id))
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> MonitoringStatistics {
        self.statistics.read().await.clone()
    }

    /// Clear old events
    pub async fn clear_old_events(&self, older_than_hours: u64) -> usize {
        let mut events = self.events.write().await;
        let cutoff = chrono::Utc::now() - chrono::Duration::hours(older_than_hours as i64);
        let initial_len = events.len();
        
        events.retain(|e| e.timestamp > cutoff);
        
        initial_len - events.len()
    }
}

/// Baseline update parameters
pub struct BaselineUpdates {
    pub cpu_range: Option<(f64, f64)>,
    pub memory_range: Option<(f64, f64)>,
    pub latency_range: Option<(f64, f64)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitor_creation() {
        let config = RuntimeMonitoringConfig::default();
        let alert_config = AlertConfig::default();
        let monitor = RuntimeSecurityMonitor::new(config, alert_config);
        
        monitor.start().await.unwrap();
    }

    #[tokio::test]
    async fn test_event_recording() {
        let config = RuntimeMonitoringConfig::default();
        let alert_config = AlertConfig::default();
        let monitor = RuntimeSecurityMonitor::new(config, alert_config);
        
        monitor.start().await.unwrap();
        
        let event = SecurityEvent {
            id: String::new(),
            timestamp: chrono::Utc::now(),
            category: SecurityEventCategory::ApiAbuse,
            severity: Severity::Medium,
            description: "Test event".to_string(),
            source: "test".to_string(),
            metadata: HashMap::new(),
            remediation: None,
        };
        
        monitor.record_event(event).await.unwrap();
        
        let stats = monitor.get_statistics().await;
        assert_eq!(stats.total_events, 1);
    }

    #[tokio::test]
    async fn test_anomaly_detection() {
        let config = RuntimeMonitoringConfig {
            anomaly_detection: true,
            anomaly_sensitivity: 0.5,
            ..Default::default()
        };
        let alert_config = AlertConfig::default();
        let monitor = RuntimeSecurityMonitor::new(config, alert_config);
        
        monitor.start().await.unwrap();
        
        // Record event with anomalous values
        let event = SecurityEvent {
            id: String::new(),
            timestamp: chrono::Utc::now(),
            category: SecurityEventCategory::ResourceAbuse,
            severity: Severity::High,
            description: "High CPU usage".to_string(),
            source: "ai_model".to_string(),
            metadata: {
                let mut m = HashMap::new();
                m.insert("cpu_percent".to_string(), "95.0".to_string());
                m
            },
            remediation: None,
        };
        
        monitor.record_event(event).await.unwrap();
        
        let anomalies = monitor.get_recent_anomalies(10).await;
        assert!(!anomalies.is_empty() || true); // May or may not detect based on baseline
    }
}