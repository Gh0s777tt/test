// VantisOS Nexus Analytics - Real-time Analytics Dashboard
// Copyright (c) 2025 VantisOS Foundation
// SPDX-License-Identifier: MIT

//! # Nexus Analytics
//!
//! Real-time analytics engine for metrics collection, visualization, and alerting.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{NexusError};
use super::storage::NexusStorage;

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter (monotonically increasing)
    Counter,
    /// Gauge (can go up or down)
    Gauge,
    /// Histogram (distribution of values)
    Histogram,
    /// Summary (percentiles)
    Summary,
}

/// Metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Metric ID
    pub metric_id: Uuid,
    
    /// Metric type
    pub metric_type: MetricType,
    
    /// Metric name
    pub name: String,
    
    /// Metric value
    pub value: f64,
    
    /// Metric unit
    pub unit: String,
    
    /// Timestamp
    pub timestamp: u64,
    
    /// Labels (key-value pairs)
    pub labels: HashMap<String, String>,
    
    /// Node ID (if applicable)
    pub node_id: Option<Uuid>,
}

impl Metric {
    /// Create a new metric
    pub fn new(
        metric_type: MetricType,
        name: String,
        value: f64,
        unit: String,
    ) -> Self {
        Self {
            metric_id: Uuid::new_v4(),
            metric_type,
            name,
            value,
            unit,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            labels: HashMap::new(),
            node_id: None,
        }
    }
    
    /// Add a label
    pub fn with_label(mut self, key: String, value: String) -> Self {
        self.labels.insert(key, value);
        self
    }
    
    /// Set node ID
    pub fn with_node_id(mut self, node_id: Uuid) -> Self {
        self.node_id = Some(node_id);
        self
    }
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Info
    Info = 0,
    /// Warning
    Warning = 1,
    /// Error
    Error = 2,
    /// Critical
    Critical = 3,
}

/// Alert status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertStatus {
    /// Alert is active
    Active,
    /// Alert is acknowledged
    Acknowledged,
    /// Alert is resolved
    Resolved,
}

/// Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub alert_id: Uuid,
    
    /// Alert name
    pub name: String,
    
    /// Alert description
    pub description: String,
    
    /// Alert severity
    pub severity: AlertSeverity,
    
    /// Alert status
    pub status: AlertStatus,
    
    /// Metric name that triggered the alert
    pub metric_name: String,
    
    /// Threshold value
    pub threshold: f64,
    
    /// Actual value
    pub actual_value: f64,
    
    /// Alert timestamp
    pub timestamp: u64,
    
    /// Acknowledged by
    pub acknowledged_by: Option<String>,
    
    /// Acknowledged at
    pub acknowledged_at: Option<u64>,
    
    /// Resolved at
    pub resolved_at: Option<u64>,
    
    /// Labels
    pub labels: HashMap<String, String>,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule ID
    pub rule_id: Uuid,
    
    /// Rule name
    pub name: String,
    
    /// Metric name to monitor
    pub metric_name: String,
    
    /// Condition (>, <, >=, <=, ==, !=)
    pub condition: String,
    
    /// Threshold value
    pub threshold: f64,
    
    /// Alert severity
    pub severity: AlertSeverity,
    
    /// Duration before triggering (seconds)
    pub duration: u64,
    
    /// Rule enabled
    pub enabled: bool,
    
    /// Labels to match
    pub label_matchers: HashMap<String, String>,
}

/// Dashboard widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    /// Widget ID
    pub widget_id: Uuid,
    
    /// Widget type
    pub widget_type: WidgetType,
    
    /// Widget title
    pub title: String,
    
    /// Widget position (x, y, width, height)
    pub position: (u32, u32, u32, u32),
    
    /// Widget configuration
    pub config: WidgetConfig,
}

/// Widget type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    /// Line chart
    LineChart,
    /// Bar chart
    BarChart,
    /// Gauge
    Gauge,
    /// Stat card
    StatCard,
    /// Table
    Table,
    /// Heatmap
    Heatmap,
}

/// Widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    /// Metric name
    pub metric_name: String,
    
    /// Time range (seconds)
    pub time_range: u64,
    
    /// Aggregation function (avg, sum, min, max, count)
    pub aggregation: String,
    
    /// Labels to filter
    pub label_filters: HashMap<String, String>,
    
    /// Additional configuration
    pub extra: HashMap<String, String>,
}

/// Analytics engine
pub struct AnalyticsEngine {
    /// Storage backend
    storage: Arc<NexusStorage>,
    
    /// Metrics retention period in days
    retention_days: u32,
    
    /// In-memory metrics buffer
    metrics_buffer: Arc<RwLock<VecDeque<Metric>>>,
    
    /// Alert rules
    alert_rules: Arc<RwLock<HashMap<Uuid, AlertRule>>>,
    
    /// Active alerts
    active_alerts: Arc<RwLock<HashMap<Uuid, Alert>>>,
    
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl AnalyticsEngine {
    /// Create a new Analytics Engine instance
    pub fn new(
        storage: Arc<NexusStorage>,
        retention_days: u32,
    ) -> Result<Self, NexusError> {
        Ok(Self {
            storage,
            retention_days,
            metrics_buffer: Arc::new(RwLock::new(VecDeque::with_capacity(10000))),
            alert_rules: Arc::new(RwLock::new(HashMap::new())),
            active_alerts: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Record a metric
    pub async fn record_metric(&self, metric: Metric) -> Result<(), NexusError> {
        // Add to buffer
        let mut buffer = self.metrics_buffer.write()
            .map_err(|_| NexusError::LockError)?;
        buffer.push_back(metric.clone());
        
        // Check if buffer should be flushed
        if buffer.len() >= 1000 {
            self.flush_metrics().await?;
        }
        
        // Check alert rules
        self.check_alerts(&metric).await?;
        
        Ok(())
    }
    
    /// Flush metrics buffer to storage
    async fn flush_metrics(&self) -> Result<(), NexusError> {
        let mut buffer = self.metrics_buffer.write()
            .map_err(|_| NexusError::LockError)?;
        
        if buffer.is_empty() {
            return Ok(());
        }
        
        // TODO: Store metrics in ClickHouse
        for metric in buffer.drain(..) {
            self.storage.store_metric(&metric).await?;
        }
        
        Ok(())
    }
    
    /// Check alert rules against a metric
    async fn check_alerts(&self, metric: &Metric) -> Result<(), NexusError> {
        let rules = self.alert_rules.read()
            .map_err(|_| NexusError::LockError)?;
        
        for rule in rules.values() {
            if !rule.enabled {
                continue;
            }
            
            // Check if metric name matches
            if rule.metric_name != metric.name {
                continue;
            }
            
            // Check label matchers
            let labels_match = rule.label_matchers.iter()
                .all(|(key, value)| metric.labels.get(key) == Some(value));
            
            if !labels_match {
                continue;
            }
            
            // Check condition
            let triggered = match rule.condition.as_str() {
                ">" => metric.value > rule.threshold,
                "<" => metric.value < rule.threshold,
                ">=" => metric.value >= rule.threshold,
                "<=" => metric.value <= rule.threshold,
                "==" => metric.value == rule.threshold,
                "!=" => metric.value != rule.threshold,
                _ => false,
            };
            
            if triggered {
                self.trigger_alert(rule, metric).await?;
            }
        }
        
        Ok(())
    }
    
    /// Trigger an alert
    async fn trigger_alert(&self, rule: &AlertRule, metric: &Metric) -> Result<(), NexusError> {
        let alert = Alert {
            alert_id: Uuid::new_v4(),
            name: rule.name.clone(),
            description: format!("Metric {} {} {}", rule.metric_name, rule.condition, rule.threshold),
            severity: rule.severity,
            status: AlertStatus::Active,
            metric_name: rule.metric_name.clone(),
            threshold: rule.threshold,
            actual_value: metric.value,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            acknowledged_by: None,
            acknowledged_at: None,
            resolved_at: None,
            labels: metric.labels.clone(),
        };
        
        // Store alert
        let mut alerts = self.active_alerts.write()
            .map_err(|_| NexusError::LockError)?;
        alerts.insert(alert.alert_id, alert.clone());
        
        log::warn!("Alert triggered: {} (severity: {:?})", alert.name, alert.severity);
        
        Ok(())
    }
    
    /// Acknowledge an alert
    pub fn acknowledge_alert(&self, alert_id: Uuid, user: String) -> Result<(), NexusError> {
        let mut alerts = self.active_alerts.write()
            .map_err(|_| NexusError::LockError)?;
        
        if let Some(alert) = alerts.get_mut(&alert_id) {
            alert.status = AlertStatus::Acknowledged;
            alert.acknowledged_by = Some(user);
            alert.acknowledged_at = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs());
        }
        
        Ok(())
    }
    
    /// Resolve an alert
    pub fn resolve_alert(&self, alert_id: Uuid) -> Result<(), NexusError> {
        let mut alerts = self.active_alerts.write()
            .map_err(|_| NexusError::LockError)?;
        
        if let Some(alert) = alerts.get_mut(&alert_id) {
            alert.status = AlertStatus::Resolved;
            alert.resolved_at = Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs());
        }
        
        Ok(())
    }
    
    /// Get active alerts
    pub fn get_active_alerts(&self) -> Vec<Alert> {
        let alerts = self.active_alerts.read()
            .unwrap_or_else(|_| Arc::new(RwLock::new(HashMap::new())));
        alerts.values().cloned().collect()
    }
    
    /// Query metrics
    pub async fn query_metrics(
        &self,
        metric_name: Option<String>,
        start_time: u64,
        end_time: u64,
        limit: usize,
    ) -> Result<Vec<Metric>, NexusError> {
        // Flush buffer first
        self.flush_metrics().await?;
        
        // Query from storage
        self.storage.query_metrics(metric_name, start_time, end_time, limit).await
    }
    
    /// Add an alert rule
    pub fn add_alert_rule(&self, rule: AlertRule) -> Result<(), NexusError> {
        let mut rules = self.alert_rules.write()
            .map_err(|_| NexusError::LockError)?;
        rules.insert(rule.rule_id, rule);
        Ok(())
    }
    
    /// Remove an alert rule
    pub fn remove_alert_rule(&self, rule_id: Uuid) -> Result<(), NexusError> {
        let mut rules = self.alert_rules.write()
            .map_err(|_| NexusError::LockError)?;
        rules.remove(&rule_id);
        Ok(())
    }
    
    /// Get metrics count
    pub fn get_metrics_count(&self) -> usize {
        let buffer = self.metrics_buffer.read()
            .unwrap_or_else(|_| Arc::new(RwLock::new(VecDeque::new())));
        buffer.len()
    }
    
    /// Start the analytics engine
    pub async fn start(&self) -> Result<(), NexusError> {
        let mut running = self.running.write()
            .map_err(|_| NexusError::LockError)?;
        
        if *running {
            return Err(NexusError::AlreadyRunning);
        }
        
        *running = true;
        drop(running);
        
        // TODO: Start background task to flush metrics periodically
        // TODO: Start background task to check retention and delete old metrics
        
        log::info!("Analytics Engine started successfully");
        
        Ok(())
    }
    
    /// Stop the analytics engine
    pub async fn stop(&self) -> Result<(), NexusError> {
        let mut running = self.running.write()
            .map_err(|_| NexusError::LockError)?;
        
        if !*running {
            return Err(NexusError::NotRunning);
        }
        
        *running = false;
        drop(running);
        
        // Flush remaining metrics
        self.flush_metrics().await?;
        
        log::info!("Analytics Engine stopped successfully");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metric_creation() {
        let metric = Metric::new(
            MetricType::Gauge,
            "cpu_usage".to_string(),
            75.5,
            "percent".to_string(),
        )
        .with_label("host".to_string(), "node1".to_string())
        .with_node_id(Uuid::new_v4());
        
        assert_eq!(metric.name, "cpu_usage");
        assert_eq!(metric.value, 75.5);
        assert_eq!(metric.labels.get("host"), Some(&"node1".to_string()));
    }
    
    #[test]
    fn test_alert_rule_creation() {
        let rule = AlertRule {
            rule_id: Uuid::new_v4(),
            name: "High CPU Usage".to_string(),
            metric_name: "cpu_usage".to_string(),
            condition: ">".to_string(),
            threshold: 90.0,
            severity: AlertSeverity::Warning,
            duration: 300,
            enabled: true,
            label_matchers: HashMap::new(),
        };
        
        assert_eq!(rule.metric_name, "cpu_usage");
        assert_eq!(rule.condition, ">");
        assert_eq!(rule.threshold, 90.0);
    }
}