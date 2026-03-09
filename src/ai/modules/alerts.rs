//! Alert System for VantisOS
//!
//! This module provides a comprehensive alert system for anomaly detection:
//! - Multi-level severity (Info, Warning, Error, Critical)
//! - Alert aggregation and deduplication
//! - Alert aging and auto-resolution
//! - Alert history and statistics
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::alerts::{AlertManager, AlertSeverity};
//!
//! let mut manager = AlertManager::new();
//! let alert = manager.create_alert(AlertSeverity::Critical, "System overload detected");
//! manager.trigger(alert);
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

/// Alert severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl AlertSeverity {
    pub fn to_string(&self) -> &str {
        match self {
            AlertSeverity::Info => "INFO",
            AlertSeverity::Warning => "WARNING",
            AlertSeverity::Error => "ERROR",
            AlertSeverity::Critical => "CRITICAL",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "INFO" => Some(AlertSeverity::Info),
            "WARNING" => Some(AlertSeverity::Warning),
            "ERROR" => Some(AlertSeverity::Error),
            "CRITICAL" => Some(AlertSeverity::Critical),
            _ => None,
        }
    }

    pub fn numeric_value(&self) -> u8 {
        match self {
            AlertSeverity::Info => 1,
            AlertSeverity::Warning => 2,
            AlertSeverity::Error => 3,
            AlertSeverity::Critical => 4,
        }
    }
}

/// Alert status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
    Suppressed,
}

/// Alert
#[derive(Debug, Clone)]
pub struct Alert {
    pub id: String,
    pub severity: AlertSeverity,
    pub title: String,
    pub description: String,
    pub source: String,
    pub timestamp: u64,
    pub status: AlertStatus,
    pub metadata: HashMap<String, String>,
    pub acknowledged_by: Option<String>,
    pub acknowledged_at: Option<u64>,
    pub resolved_at: Option<u64>,
}

impl Alert {
    pub fn new(severity: AlertSeverity, title: String) -> Self {
        let id = format!("alert_{}_{}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            uuid::Uuid::new_v4().to_string()[..8].to_string()
        );

        Self {
            id,
            severity,
            title,
            description: String::new(),
            source: "system".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            status: AlertStatus::Active,
            metadata: HashMap::new(),
            acknowledged_by: None,
            acknowledged_at: None,
            resolved_at: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source = source;
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn acknowledge(&mut self, user: String) {
        self.status = AlertStatus::Acknowledged;
        self.acknowledged_by = Some(user);
        self.acknowledged_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }

    pub fn resolve(&mut self) {
        self.status = AlertStatus::Resolved;
        self.resolved_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }
}

/// Alert configuration
#[derive(Debug, Clone)]
pub struct AlertConfig {
    pub max_active_alerts: usize,
    pub alert_history_size: usize,
    pub deduplication_window: u64, // seconds
    pub auto_resolve_age: u64, // seconds
    pub aggregation_enabled: bool,
    pub aggregation_window: u64, // seconds
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            max_active_alerts: 1000,
            alert_history_size: 10000,
            deduplication_window: 300, // 5 minutes
            auto_resolve_age: 86400, // 24 hours
            aggregation_enabled: true,
            aggregation_window: 60, // 1 minute
        }
    }
}

/// Alert statistics
#[derive(Debug, Clone, Default)]
pub struct AlertStats {
    pub total_alerts: usize,
    pub active_alerts: usize,
    pub acknowledged_alerts: usize,
    pub resolved_alerts: usize,
    pub suppressed_alerts: usize,
    pub alerts_by_severity: HashMap<AlertSeverity, usize>,
    pub alerts_by_source: HashMap<String, usize>,
}

/// Alert manager
pub struct AlertManager {
    pub config: AlertConfig,
    pub active_alerts: VecDeque<Alert>,
    pub alert_history: VecDeque<Alert>,
    pub alert_counts: HashMap<String, usize>,
    pub last_alert_times: HashMap<String, u64>,
    pub stats: AlertStats,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            config: AlertConfig::default(),
            active_alerts: VecDeque::new(),
            alert_history: VecDeque::new(),
            alert_counts: HashMap::new(),
            last_alert_times: HashMap::new(),
            stats: AlertStats::default(),
        }
    }

    pub fn with_config(config: AlertConfig) -> Self {
        Self {
            config,
            active_alerts: VecDeque::new(),
            alert_history: VecDeque::new(),
            alert_counts: HashMap::new(),
            last_alert_times: HashMap::new(),
            stats: AlertStats::default(),
        }
    }

    /// Create a new alert
    pub fn create_alert(&self, severity: AlertSeverity, title: String) -> Alert {
        Alert::new(severity, title)
    }

    /// Trigger an alert
    pub fn trigger(&mut self, mut alert: Alert) -> Result<()> {
        // Check deduplication
        let alert_key = format!("{}:{}", alert.severity.to_string(), alert.title);
        if let Some(&last_time) = self.last_alert_times.get(&alert_key) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if now - last_time < self.config.deduplication_window {
                return Err(AIServiceError::AlertDeduplicated);
            }
        }

        // Check max active alerts
        if self.active_alerts.len() >= self.config.max_active_alerts {
            return Err(AIServiceError::TooManyAlerts);
        }

        // Update timestamp
        alert.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Add to active alerts
        self.active_alerts.push_front(alert.clone());

        // Update tracking
        *self.alert_counts.entry(alert_key.clone()).or_insert(0) += 1;
        self.last_alert_times.insert(alert_key, alert.timestamp);

        // Update stats
        self.update_stats(&alert, true);

        Ok(())
    }

    /// Acknowledge an alert
    pub fn acknowledge_alert(&mut self, alert_id: &str, user: String) -> Result<()> {
        if let Some(alert) = self.active_alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.acknowledge(user);
            self.update_alert_stats();
            Ok(())
        } else {
            Err(AIServiceError::AlertNotFound(alert_id.to_string()))
        }
    }

    /// Resolve an alert
    pub fn resolve_alert(&mut self, alert_id: &str) -> Result<()> {
        if let Some(pos) = self.active_alerts.iter().position(|a| a.id == alert_id) {
            let mut alert = self.active_alerts.remove(pos).unwrap();
            alert.resolve();
            
            // Add to history
            self.alert_history.push_front(alert);
            
            // Manage history size
            while self.alert_history.len() > self.config.alert_history_size {
                self.alert_history.pop_back();
            }
            
            self.update_alert_stats();
            Ok(())
        } else {
            Err(AIServiceError::AlertNotFound(alert_id.to_string()))
        }
    }

    /// Get active alerts
    pub fn get_active_alerts(&self) -> Vec<Alert> {
        self.active_alerts.iter().cloned().collect()
    }

    /// Get alerts by severity
    pub fn get_alerts_by_severity(&self, severity: AlertSeverity) -> Vec<Alert> {
        self.active_alerts
            .iter()
            .filter(|a| a.severity == severity)
            .cloned()
            .collect()
    }

    /// Get alerts by source
    pub fn get_alerts_by_source(&self, source: &str) -> Vec<Alert> {
        self.active_alerts
            .iter()
            .filter(|a| a.source == source)
            .cloned()
            .collect()
    }

    /// Get alert statistics
    pub fn get_stats(&self) -> AlertStats {
        self.stats.clone()
    }

    /// Auto-resolve old alerts
    pub fn auto_resolve_old_alerts(&mut self) -> usize {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut resolved_count = 0;
        let mut to_remove = Vec::new();

        for (i, alert) in self.active_alerts.iter().enumerate() {
            if now - alert.timestamp > self.config.auto_resolve_age {
                to_remove.push(i);
            }
        }

        for &pos in to_remove.iter().rev() {
            let mut alert = self.active_alerts.remove(pos).unwrap();
            alert.resolve();
            self.alert_history.push_front(alert);
            resolved_count += 1;
        }

        // Manage history size
        while self.alert_history.len() > self.config.alert_history_size {
            self.alert_history.pop_back();
        }

        self.update_alert_stats();
        resolved_count
    }

    /// Update alert statistics
    fn update_stats(&mut self, alert: &Alert, added: bool) {
        if added {
            self.stats.total_alerts += 1;
            self.stats.active_alerts += 1;
            *self.stats.alerts_by_severity.entry(alert.severity).or_insert(0) += 1;
            *self.stats.alerts_by_source.entry(alert.source.clone()).or_insert(0) += 1;
        }
    }

    /// Recalculate alert statistics
    fn update_alert_stats(&mut self) {
        self.stats = AlertStats::default();
        self.stats.total_alerts = self.alert_history.len() + self.active_alerts.len();
        self.stats.active_alerts = self.active_alerts.len();
        self.stats.resolved_alerts = self.alert_history.len();

        for alert in &self.active_alerts {
            match alert.status {
                AlertStatus::Acknowledged => self.stats.acknowledged_alerts += 1,
                AlertStatus::Suppressed => self.stats.suppressed_alerts += 1,
                _ => {}
            }
            *self.stats.alerts_by_severity.entry(alert.severity).or_insert(0) += 1;
            *self.stats.alerts_by_source.entry(alert.source.clone()).or_insert(0) += 1;
        }
    }

    /// Clear all active alerts
    pub fn clear_active_alerts(&mut self) {
        for mut alert in self.active_alerts.drain(..) {
            alert.resolve();
            self.alert_history.push_front(alert);
        }
        self.update_alert_stats();
    }

    /// Reset manager
    pub fn reset(&mut self) {
        self.active_alerts.clear();
        self.alert_history.clear();
        self.alert_counts.clear();
        self.last_alert_times.clear();
        self.stats = AlertStats::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_creation() {
        let alert = Alert::new(AlertSeverity::Critical, "Test Alert".to_string());
        assert_eq!(alert.severity, AlertSeverity::Critical);
        assert_eq!(alert.title, "Test Alert");
        assert_eq!(alert.status, AlertStatus::Active);
    }

    #[test]
    fn test_alert_builder() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        let alert = Alert::new(AlertSeverity::Warning, "Test".to_string())
            .with_description("Test description".to_string())
            .with_source("test_source".to_string())
            .with_metadata(metadata);

        assert_eq!(alert.description, "Test description");
        assert_eq!(alert.source, "test_source");
        assert_eq!(alert.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_alert_acknowledge() {
        let mut alert = Alert::new(AlertSeverity::Error, "Test".to_string());
        alert.acknowledge("user1".to_string());
        
        assert_eq!(alert.status, AlertStatus::Acknowledged);
        assert_eq!(alert.acknowledged_by, Some("user1".to_string()));
        assert!(alert.acknowledged_at.is_some());
    }

    #[test]
    fn test_alert_resolve() {
        let mut alert = Alert::new(AlertSeverity::Critical, "Test".to_string());
        alert.resolve();
        
        assert_eq!(alert.status, AlertStatus::Resolved);
        assert!(alert.resolved_at.is_some());
    }

    #[test]
    fn test_alert_manager() {
        let mut manager = AlertManager::new();
        let alert = manager.create_alert(AlertSeverity::Critical, "System overload".to_string());
        
        assert!(manager.trigger(alert).is_ok());
        assert_eq!(manager.get_active_alerts().len(), 1);
    }

    #[test]
    fn test_alert_deduplication() {
        let mut manager = AlertManager::new();
        
        let alert1 = manager.create_alert(AlertSeverity::Critical, "Test Alert".to_string());
        manager.trigger(alert1).unwrap();
        
        let alert2 = manager.create_alert(AlertSeverity::Critical, "Test Alert".to_string());
        assert!(manager.trigger(alert2).is_err());
    }

    #[test]
    fn test_acknowledge_alert() {
        let mut manager = AlertManager::new();
        let alert = manager.create_alert(AlertSeverity::Error, "Test".to_string());
        manager.trigger(alert).unwrap();
        
        let alert_id = manager.get_active_alerts()[0].id.clone();
        assert!(manager.acknowledge_alert(&alert_id, "user1".to_string()).is_ok());
    }

    #[test]
    fn test_resolve_alert() {
        let mut manager = AlertManager::new();
        let alert = manager.create_alert(AlertSeverity::Critical, "Test".to_string());
        manager.trigger(alert).unwrap();
        
        let alert_id = manager.get_active_alerts()[0].id.clone();
        assert!(manager.resolve_alert(&alert_id).is_ok());
        assert_eq!(manager.get_active_alerts().len(), 0);
    }

    #[test]
    fn test_alert_stats() {
        let mut manager = AlertManager::new();
        
        for _ in 0..5 {
            let alert = manager.create_alert(AlertSeverity::Critical, "Test".to_string());
            manager.trigger(alert).unwrap();
        }
        
        let stats = manager.get_stats();
        assert_eq!(stats.active_alerts, 5);
        assert_eq!(stats.total_alerts, 5);
    }

    #[test]
    fn test_clear_active_alerts() {
        let mut manager = AlertManager::new();
        
        for _ in 0..10 {
            let alert = manager.create_alert(AlertSeverity::Error, "Test".to_string());
            manager.trigger(alert).unwrap();
        }
        
        manager.clear_active_alerts();
        assert_eq!(manager.get_active_alerts().len(), 0);
        assert_eq!(manager.get_stats().resolved_alerts, 10);
    }
}