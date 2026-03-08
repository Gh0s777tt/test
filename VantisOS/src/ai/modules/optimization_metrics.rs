// VantisOS - Optimization Metrics Module
// Comprehensive metrics tracking, analysis, and reporting for optimization systems

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============================================================================
// Core Metrics Types
// ============================================================================

/// Represents a single metric measurement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricMeasurement {
    pub id: Uuid,
    pub metric_type: MetricType,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub source: Option<String>,
    pub unit: Option<String>,
    pub confidence_interval: Option<ConfidenceInterval>,
    pub sample_size: Option<u64>,
}

impl MetricMeasurement {
    pub fn new(metric_type: MetricType, value: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            metric_type,
            value,
            timestamp: Utc::now(),
            tags: HashMap::new(),
            source: None,
            unit: None,
            confidence_interval: None,
            sample_size: None,
        }
    }

    pub fn with_tags(mut self, tags: HashMap<String, String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_unit(mut self, unit: String) -> Self {
        self.unit = Some(unit);
        self
    }

    pub fn with_confidence_interval(mut self, lower: f64, upper: f64, level: f64) -> Self {
        self.confidence_interval = Some(ConfidenceInterval {
            lower,
            upper,
            confidence_level: level,
        });
        self
    }

    pub fn with_sample_size(mut self, size: u64) -> Self {
        self.sample_size = Some(size);
        self
    }
}

/// Types of optimization metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MetricType {
    // Performance metrics
    ResponseTime,
    Throughput,
    Latency,
    QueueDepth,
    Concurrency,
    
    // Resource utilization
    CpuUtilization,
    MemoryUtilization,
    DiskUtilization,
    NetworkUtilization,
    GpuUtilization,
    
    // Cost metrics
    ComputeCost,
    StorageCost,
    NetworkCost,
    TotalCost,
    CostPerRequest,
    
    // Quality metrics
    Accuracy,
    Precision,
    Recall,
    F1Score,
    ErrorRate,
    DefectRate,
    
    // Business metrics
    ConversionRate,
    Revenue,
    UserSatisfaction,
    RetentionRate,
    EngagementScore,
    
    // Optimization-specific
    OptimizationScore,
    ConvergenceRate,
    ObjectiveValue,
    ConstraintViolation,
    ParetoDistance,
    
    // System health
    Availability,
    Uptime,
    MeanTimeToRecovery,
    MeanTimeBetweenFailures,
    ErrorBudgetRemaining,
    
    // Custom
    Custom(u32),
}

/// Confidence interval for metric values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub lower: f64,
    pub upper: f64,
    pub confidence_level: f64,
}

impl ConfidenceInterval {
    pub fn new(lower: f64, upper: f64, level: f64) -> Self {
        Self {
            lower,
            upper,
            confidence_level: level,
        }
    }

    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.lower && value <= self.upper
    }
}

/// Aggregated metric statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStatistics {
    pub metric_type: MetricType,
    pub count: u64,
    pub sum: f64,
    pub mean: f64,
    pub variance: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub p25: f64,
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub last_updated: DateTime<Utc>,
}

impl MetricStatistics {
    pub fn new(metric_type: MetricType) -> Self {
        Self {
            metric_type,
            count: 0,
            sum: 0.0,
            mean: 0.0,
            variance: 0.0,
            std_dev: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            median: 0.0,
            p25: 0.0,
            p75: 0.0,
            p90: 0.0,
            p95: 0.0,
            p99: 0.0,
            skewness: 0.0,
            kurtosis: 0.0,
            last_updated: Utc::now(),
        }
    }

    pub fn update(&mut self, value: f64) {
        let n = self.count as f64;
        let n1 = n + 1.0;
        
        // Update min/max
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        
        // Update sum and mean (Welford's algorithm)
        let delta = value - self.mean;
        self.sum += value;
        self.mean = self.sum / n1;
        
        // Update variance
        if self.count > 0 {
            let delta2 = value - self.mean;
            self.variance += delta * delta2;
        }
        
        self.count += 1;
        
        // Update std_dev
        if self.count > 1 {
            self.std_dev = (self.variance / n).sqrt();
        }
        
        self.last_updated = Utc::now();
    }

    pub fn finalize_percentiles(&mut self, sorted_values: &[f64]) {
        if sorted_values.is_empty() {
            return;
        }
        
        let n = sorted_values.len();
        self.median = Self::percentile(sorted_values, 50.0);
        self.p25 = Self::percentile(sorted_values, 25.0);
        self.p75 = Self::percentile(sorted_values, 75.0);
        self.p90 = Self::percentile(sorted_values, 90.0);
        self.p95 = Self::percentile(sorted_values, 95.0);
        self.p99 = Self::percentile(sorted_values, 99.0);
    }

    fn percentile(sorted_values: &[f64], p: f64) -> f64 {
        if sorted_values.is_empty() {
            return 0.0;
        }
        
        let n = sorted_values.len();
        let index = (p / 100.0) * (n - 1) as f64;
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;
        
        if lower == upper || upper >= n {
            return sorted_values[lower.min(n - 1)];
        }
        
        let weight = index - lower as f64;
        sorted_values[lower] * (1.0 - weight) + sorted_values[upper] * weight
    }
}

/// Metric trend direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

/// Metric trend analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTrend {
    pub metric_type: MetricType,
    pub direction: TrendDirection,
    pub slope: f64,
    pub r_squared: f64,
    pub change_rate: f64,
    pub confidence: f64,
    pub prediction: Option<MetricPrediction>,
    pub anomalies: Vec<MetricAnomaly>,
}

/// Metric prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricPrediction {
    pub predicted_value: f64,
    pub prediction_interval: ConfidenceInterval,
    pub horizon_minutes: u32,
    pub confidence: f64,
}

/// Metric anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAnomaly {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub expected_value: f64,
    pub deviation: f64,
    pub severity: AnomalySeverity,
    pub description: String,
}

/// Anomaly severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Metric alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAlertConfig {
    pub metric_type: MetricType,
    pub threshold_type: ThresholdType,
    pub threshold: f64,
    pub comparison: ComparisonOperator,
    pub duration_minutes: u32,
    pub severity: AlertSeverity,
    pub notification_channels: Vec<String>,
    pub enabled: bool,
    pub cooldown_minutes: u32,
}

/// Threshold types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThresholdType {
    Static,
    PercentageOfMean,
    StandardDeviations,
    Percentile,
    Dynamic,
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equals,
    NotEquals,
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Active metric alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAlert {
    pub id: Uuid,
    pub config: MetricAlertConfig,
    pub triggered_at: DateTime<Utc>,
    pub current_value: f64,
    pub threshold_value: f64,
    pub message: String,
    pub acknowledged: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_message: Option<String>,
}

/// Metrics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsReport {
    pub report_id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub time_range: TimeRange,
    pub metrics_summary: HashMap<MetricType, MetricStatistics>,
    pub trends: HashMap<MetricType, MetricTrend>,
    pub alerts: Vec<MetricAlert>,
    pub recommendations: Vec<MetricRecommendation>,
    pub scorecard: MetricScorecard,
}

/// Time range for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub granularity: TimeGranularity,
}

/// Time granularity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeGranularity {
    Minute,
    Hour,
    Day,
    Week,
    Month,
}

/// Metric recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricRecommendation {
    pub metric_type: MetricType,
    pub priority: RecommendationPriority,
    pub title: String,
    pub description: String,
    pub suggested_action: String,
    pub expected_impact: f64,
    pub confidence: f64,
}

/// Recommendation priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
    Immediate,
}

/// Metric scorecard for overall system health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricScorecard {
    pub overall_score: f64,
    pub performance_score: f64,
    pub cost_score: f64,
    pub quality_score: f64,
    pub reliability_score: f64,
    pub optimization_score: f64,
    pub grade: SystemGrade,
    pub summary: String,
}

/// System grade based on metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemGrade {
    A,
    B,
    C,
    D,
    F,
}

// ============================================================================
// Metrics Tracker
// ============================================================================

/// Main metrics tracking system
pub struct OptimizationMetrics {
    /// Raw measurements storage
    measurements: HashMap<MetricType, Vec<MetricMeasurement>>,
    /// Computed statistics
    statistics: HashMap<MetricType, MetricStatistics>,
    /// Alert configurations
    alert_configs: Vec<MetricAlertConfig>,
    /// Active alerts
    active_alerts: HashMap<Uuid, MetricAlert>,
    /// Configuration
    config: MetricsConfig,
    /// Time-series data (for trend analysis)
    time_series: BTreeMap<DateTime<Utc>, HashMap<MetricType, f64>>,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub max_measurements_per_metric: usize,
    pub retention_hours: i64,
    pub enable_trend_analysis: bool,
    pub enable_anomaly_detection: bool,
    pub trend_window_size: usize,
    pub anomaly_z_threshold: f64,
    pub auto_expire_data: bool,
    pub export_format: ExportFormat,
}

/// Export format for metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Prometheus,
    InfluxDb,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            max_measurements_per_metric: 10000,
            retention_hours: 168, // 7 days
            enable_trend_analysis: true,
            enable_anomaly_detection: true,
            trend_window_size: 100,
            anomaly_z_threshold: 3.0,
            auto_expire_data: true,
            export_format: ExportFormat::Json,
        }
    }
}

impl OptimizationMetrics {
    /// Create a new metrics tracker
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            measurements: HashMap::new(),
            statistics: HashMap::new(),
            alert_configs: Vec::new(),
            active_alerts: HashMap::new(),
            config,
            time_series: BTreeMap::new(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(MetricsConfig::default())
    }

    /// Record a metric measurement
    pub fn record(&mut self, measurement: MetricMeasurement) {
        let metric_type = measurement.metric_type;
        let value = measurement.value;
        let timestamp = measurement.timestamp;
        
        // Store measurement
        self.measurements
            .entry(metric_type)
            .or_insert_with(Vec::new)
            .push(measurement);
        
        // Enforce limit
        if let Some(measurements) = self.measurements.get_mut(&metric_type) {
            if measurements.len() > self.config.max_measurements_per_metric {
                let excess = measurements.len() - self.config.max_measurements_per_metric;
                measurements.drain(0..excess);
            }
        }
        
        // Update statistics
        let stats = self.statistics
            .entry(metric_type)
            .or_insert_with(|| MetricStatistics::new(metric_type));
        stats.update(value);
        
        // Update time series
        self.time_series
            .entry(timestamp)
            .or_insert_with(HashMap::new)
            .insert(metric_type, value);
        
        // Check alerts
        self._check_alerts(metric_type, value);
        
        // Enforce retention
        if self.config.auto_expire_data {
            self._expire_old_data();
        }
    }

    /// Record a simple metric value
    pub fn record_value(&mut self, metric_type: MetricType, value: f64) {
        let measurement = MetricMeasurement::new(metric_type, value);
        self.record(measurement);
    }

    /// Record with tags
    pub fn record_with_tags(&mut self, metric_type: MetricType, value: f64, tags: HashMap<String, String>) {
        let measurement = MetricMeasurement::new(metric_type, value)
            .with_tags(tags);
        self.record(measurement);
    }

    /// Get statistics for a metric
    pub fn get_statistics(&self, metric_type: MetricType) -> Option<&MetricStatistics> {
        self.statistics.get(&metric_type)
    }

    /// Get all statistics
    pub fn get_all_statistics(&self) -> &HashMap<MetricType, MetricStatistics> {
        &self.statistics
    }

    /// Get measurements for a metric
    pub fn get_measurements(&self, metric_type: MetricType) -> Option<&Vec<MetricMeasurement>> {
        self.measurements.get(&metric_type)
    }

    /// Analyze trend for a metric
    pub fn analyze_trend(&self, metric_type: MetricType) -> Option<MetricTrend> {
        let measurements = self.measurements.get(&metric_type)?;
        
        if measurements.len() < 3 {
            return None;
        }
        
        // Simple linear regression for trend
        let n = measurements.len() as f64;
        let values: Vec<f64> = measurements.iter().map(|m| m.value).collect();
        
        // Calculate linear regression
        let sum_x: f64 = (0..measurements.len()).map(|i| i as f64).sum();
        let sum_y: f64 = values.iter().sum();
        let sum_xy: f64 = values.iter()
            .enumerate()
            .map(|(i, y)| i as f64 * y)
            .sum();
        let sum_x2: f64 = (0..measurements.len()).map(|i| (i as f64).powi(2)).sum();
        let sum_y2: f64 = values.iter().map(|y| y.powi(2)).sum();
        
        let denominator = n * sum_x2 - sum_x * sum_x;
        if denominator.abs() < 1e-10 {
            return None;
        }
        
        let slope = (n * sum_xy - sum_x * sum_y) / denominator;
        let intercept = (sum_y - slope * sum_x) / n;
        
        // Calculate R-squared
        let y_mean = sum_y / n;
        let ss_tot: f64 = values.iter().map(|y| (y - y_mean).powi(2)).sum();
        let ss_res: f64 = values.iter()
            .enumerate()
            .map(|(i, y)| (y - (slope * i as f64 + intercept)).powi(2))
            .sum();
        let r_squared = 1.0 - ss_res / ss_tot.max(1e-10);
        
        // Determine trend direction
        let direction = if slope.abs() < 1e-6 {
            TrendDirection::Stable
        } else if slope > 0.0 {
            TrendDirection::Increasing
        } else {
            TrendDirection::Decreasing
        };
        
        // Calculate change rate
        let first_val = values.first().unwrap_or(&0.0);
        let last_val = values.last().unwrap_or(&0.0);
        let change_rate = if first_val.abs() > 1e-10 {
            (last_val - first_val) / first_val.abs()
        } else {
            0.0
        };
        
        // Detect anomalies
        let anomalies = self._detect_anomalies(measurements);
        
        // Generate prediction if enough data
        let prediction = if measurements.len() >= 10 {
            let stats = self.statistics.get(&metric_type)?;
            let predicted = slope * (measurements.len() as f64 + 60.0) + intercept; // Predict 60 intervals ahead
            let std_err = stats.std_dev * (1.0 + 1.0 / n).sqrt();
            
            Some(MetricPrediction {
                predicted_value: predicted,
                prediction_interval: ConfidenceInterval::new(
                    predicted - 1.96 * std_err,
                    predicted + 1.96 * std_err,
                    0.95,
                ),
                horizon_minutes: 60,
                confidence: r_squared,
            })
        } else {
            None
        };
        
        Some(MetricTrend {
            metric_type,
            direction,
            slope,
            r_squared,
            change_rate,
            confidence: r_squared,
            prediction,
            anomalies,
        })
    }

    /// Configure an alert
    pub fn configure_alert(&mut self, config: MetricAlertConfig) {
        // Remove existing config for same metric if exists
        self.alert_configs.retain(|c| c.metric_type != config.metric_type);
        self.alert_configs.push(config);
    }

    /// Get active alerts
    pub fn get_active_alerts(&self) -> Vec<&MetricAlert> {
        self.active_alerts.values().filter(|a| !a.acknowledged).collect()
    }

    /// Acknowledge an alert
    pub fn acknowledge_alert(&mut self, alert_id: Uuid) -> Result<(), String> {
        if let Some(alert) = self.active_alerts.get_mut(&alert_id) {
            alert.acknowledged = true;
            Ok(())
        } else {
            Err(format!("Alert {} not found", alert_id))
        }
    }

    /// Resolve an alert
    pub fn resolve_alert(&mut self, alert_id: Uuid, message: String) -> Result<(), String> {
        if let Some(alert) = self.active_alerts.get_mut(&alert_id) {
            alert.resolved_at = Some(Utc::now());
            alert.resolution_message = Some(message);
            Ok(())
        } else {
            Err(format!("Alert {} not found", alert_id))
        }
    }

    /// Generate a metrics report
    pub fn generate_report(&self, time_range: TimeRange) -> MetricsReport {
        // Compute statistics for time range
        let mut metrics_summary = HashMap::new();
        let mut trends = HashMap::new();
        
        for metric_type in self.measurements.keys() {
            if let Some(stats) = self._compute_range_statistics(*metric_type, &time_range) {
                metrics_summary.insert(*metric_type, stats);
            }
            
            if let Some(trend) = self.analyze_trend(*metric_type) {
                trends.insert(*metric_type, trend);
            }
        }
        
        // Get active alerts
        let alerts: Vec<MetricAlert> = self.active_alerts.values().cloned().collect();
        
        // Generate recommendations
        let recommendations = self._generate_recommendations(&metrics_summary, &trends);
        
        // Compute scorecard
        let scorecard = self._compute_scorecard(&metrics_summary);
        
        MetricsReport {
            report_id: Uuid::new_v4(),
            generated_at: Utc::now(),
            time_range,
            metrics_summary,
            trends,
            alerts,
            recommendations,
            scorecard,
        }
    }

    /// Export metrics in configured format
    pub fn export(&self, format: Option<ExportFormat>) -> Result<String, String> {
        let format = format.unwrap_or(self.config.export_format);
        
        match format {
            ExportFormat::Json => self._export_json(),
            ExportFormat::Csv => self._export_csv(),
            _ => Err("Export format not yet implemented".to_string()),
        }
    }

    /// Clear all metrics data
    pub fn clear(&mut self) {
        self.measurements.clear();
        self.statistics.clear();
        self.time_series.clear();
    }

    /// Clear metrics older than retention period
    pub fn clear_expired(&mut self) -> usize {
        let cutoff = Utc::now() - chrono::Duration::hours(self.config.retention_hours);
        let mut removed = 0;
        
        for measurements in self.measurements.values_mut() {
            let before = measurements.len();
            measurements.retain(|m| m.timestamp > cutoff);
            removed += before - measurements.len();
        }
        
        // Clean time series
        self.time_series.retain(|t, _| *t > cutoff);
        
        removed
    }

    // Private helper methods

    fn _check_alerts(&mut self, metric_type: MetricType, value: f64) {
        for config in &self.alert_configs {
            if config.metric_type != metric_type || !config.enabled {
                continue;
            }
            
            let threshold_value = self._compute_threshold(config);
            let triggered = match config.comparison {
                ComparisonOperator::GreaterThan => value > threshold_value,
                ComparisonOperator::LessThan => value < threshold_value,
                ComparisonOperator::GreaterThanOrEqual => value >= threshold_value,
                ComparisonOperator::LessThanOrEqual => value <= threshold_value,
                ComparisonOperator::Equals => (value - threshold_value).abs() < 1e-10,
                ComparisonOperator::NotEquals => (value - threshold_value).abs() >= 1e-10,
            };
            
            if triggered {
                let alert = MetricAlert {
                    id: Uuid::new_v4(),
                    config: config.clone(),
                    triggered_at: Utc::now(),
                    current_value: value,
                    threshold_value,
                    message: format!(
                        "Metric {:?} {} threshold: current={}, threshold={}",
                        metric_type,
                        if triggered { "exceeded" } else { "below" },
                        value,
                        threshold_value
                    ),
                    acknowledged: false,
                    resolved_at: None,
                    resolution_message: None,
                };
                
                self.active_alerts.insert(alert.id, alert);
            }
        }
    }

    fn _compute_threshold(&self, config: &MetricAlertConfig) -> f64 {
        match config.threshold_type {
            ThresholdType::Static => config.threshold,
            ThresholdType::PercentageOfMean => {
                if let Some(stats) = self.statistics.get(&config.metric_type) {
                    stats.mean * config.threshold
                } else {
                    config.threshold
                }
            }
            ThresholdType::StandardDeviations => {
                if let Some(stats) = self.statistics.get(&config.metric_type) {
                    stats.mean + config.threshold * stats.std_dev
                } else {
                    config.threshold
                }
            }
            ThresholdType::Percentile => {
                // Would need full data to compute percentile
                config.threshold
            }
            ThresholdType::Dynamic => {
                // Would implement dynamic thresholding
                config.threshold
            }
        }
    }

    fn _detect_anomalies(&self, measurements: &[MetricMeasurement]) -> Vec<MetricAnomaly> {
        if measurements.len() < 10 || !self.config.enable_anomaly_detection {
            return Vec::new();
        }
        
        let values: Vec<f64> = measurements.iter().map(|m| m.value).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();
        
        let mut anomalies = Vec::new();
        
        for measurement in measurements {
            let z_score = (measurement.value - mean) / std_dev.max(1e-10);
            
            if z_score.abs() > self.config.anomaly_z_threshold {
                let severity = if z_score.abs() > 4.0 {
                    AnomalySeverity::Critical
                } else if z_score.abs() > 3.5 {
                    AnomalySeverity::High
                } else if z_score.abs() > 3.0 {
                    AnomalySeverity::Medium
                } else {
                    AnomalySeverity::Low
                };
                
                anomalies.push(MetricAnomaly {
                    timestamp: measurement.timestamp,
                    value: measurement.value,
                    expected_value: mean,
                    deviation: z_score,
                    severity,
                    description: format!(
                        "Anomaly detected: value {:.2} deviates {:.2}σ from mean {:.2}",
                        measurement.value, z_score, mean
                    ),
                });
            }
        }
        
        anomalies
    }

    fn _expire_old_data(&mut self) {
        // Only expire occasionally to avoid overhead
        if self.measurements.values().map(|v| v.len()).sum::<usize>() % 1000 != 0 {
            return;
        }
        
        self.clear_expired();
    }

    fn _compute_range_statistics(&self, metric_type: MetricType, time_range: &TimeRange) -> Option<MetricStatistics> {
        let measurements = self.measurements.get(&metric_type)?;
        let filtered: Vec<&MetricMeasurement> = measurements
            .iter()
            .filter(|m| m.timestamp >= time_range.start && m.timestamp <= time_range.end)
            .collect();
        
        if filtered.is_empty() {
            return None;
        }
        
        let mut stats = MetricStatistics::new(metric_type);
        let mut values: Vec<f64> = filtered.iter().map(|m| m.value).collect();
        
        for &value in &values {
            stats.update(value);
        }
        
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        stats.finalize_percentiles(&values);
        
        Some(stats)
    }

    fn _generate_recommendations(
        &self,
        metrics_summary: &HashMap<MetricType, MetricStatistics>,
        trends: &HashMap<MetricType, MetricTrend>,
    ) -> Vec<MetricRecommendation> {
        let mut recommendations = Vec::new();
        
        // Check for high error rates
        if let Some(error_stats) = metrics_summary.get(&MetricType::ErrorRate) {
            if error_stats.mean > 0.05 {
                recommendations.push(MetricRecommendation {
                    metric_type: MetricType::ErrorRate,
                    priority: RecommendationPriority::High,
                    title: "High Error Rate Detected".to_string(),
                    description: format!("Error rate is {:.1}% which exceeds acceptable threshold", error_stats.mean * 100.0),
                    suggested_action: "Investigate root cause and implement fixes".to_string(),
                    expected_impact: 0.8,
                    confidence: 0.9,
                });
            }
        }
        
        // Check for resource saturation
        if let Some(cpu_stats) = metrics_summary.get(&MetricType::CpuUtilization) {
            if cpu_stats.mean > 0.8 {
                recommendations.push(MetricRecommendation {
                    metric_type: MetricType::CpuUtilization,
                    priority: RecommendationPriority::Medium,
                    title: "High CPU Utilization".to_string(),
                    description: format!("CPU utilization averages {:.1}%", cpu_stats.mean * 100.0),
                    suggested_action: "Consider scaling resources or optimizing compute-intensive operations".to_string(),
                    expected_impact: 0.6,
                    confidence: 0.85,
                });
            }
        }
        
        // Check for memory pressure
        if let Some(mem_stats) = metrics_summary.get(&MetricType::MemoryUtilization) {
            if mem_stats.mean > 0.85 {
                recommendations.push(MetricRecommendation {
                    metric_type: MetricType::MemoryUtilization,
                    priority: RecommendationPriority::High,
                    title: "Memory Pressure Detected".to_string(),
                    description: format!("Memory utilization averages {:.1}%", mem_stats.mean * 100.0),
                    suggested_action: "Increase memory allocation or optimize memory usage".to_string(),
                    expected_impact: 0.7,
                    confidence: 0.9,
                });
            }
        }
        
        // Check for latency issues
        if let Some(latency_stats) = metrics_summary.get(&MetricType::Latency) {
            if latency_stats.p95 > 1000.0 {
                recommendations.push(MetricRecommendation {
                    metric_type: MetricType::Latency,
                    priority: RecommendationPriority::Medium,
                    title: "High Latency Observed".to_string(),
                    description: format!("P95 latency is {:.1}ms", latency_stats.p95),
                    suggested_action: "Investigate bottlenecks and optimize critical paths".to_string(),
                    expected_impact: 0.5,
                    confidence: 0.8,
                });
            }
        }
        
        recommendations
    }

    fn _compute_scorecard(&self, metrics_summary: &HashMap<MetricType, MetricStatistics>) -> MetricScorecard {
        let mut scores = Vec::new();
        
        // Performance score (based on latency and throughput)
        let performance_score = self._compute_performance_score(metrics_summary);
        scores.push(performance_score);
        
        // Cost score
        let cost_score = self._compute_cost_score(metrics_summary);
        scores.push(cost_score);
        
        // Quality score
        let quality_score = self._compute_quality_score(metrics_summary);
        scores.push(quality_score);
        
        // Reliability score
        let reliability_score = self._compute_reliability_score(metrics_summary);
        scores.push(reliability_score);
        
        // Optimization score
        let optimization_score = self._compute_optimization_score(metrics_summary);
        scores.push(optimization_score);
        
        let overall_score = scores.iter().sum::<f64>() / scores.len() as f64;
        
        let grade = if overall_score >= 0.9 {
            SystemGrade::A
        } else if overall_score >= 0.8 {
            SystemGrade::B
        } else if overall_score >= 0.7 {
            SystemGrade::C
        } else if overall_score >= 0.6 {
            SystemGrade::D
        } else {
            SystemGrade::F
        };
        
        let summary = format!(
            "Overall system health: {:.0}% (Grade {:?}). Performance: {:.0}%, Cost: {:.0}%, Quality: {:.0}%, Reliability: {:.0}%",
            overall_score * 100.0,
            grade,
            performance_score * 100.0,
            cost_score * 100.0,
            quality_score * 100.0,
            reliability_score * 100.0
        );
        
        MetricScorecard {
            overall_score,
            performance_score,
            cost_score,
            quality_score,
            reliability_score,
            optimization_score,
            grade,
            summary,
        }
    }

    fn _compute_performance_score(&self, metrics: &HashMap<MetricType, MetricStatistics>) -> f64 {
        let mut score = 1.0;
        
        if let Some(latency) = metrics.get(&MetricType::Latency) {
            // Lower latency is better
            if latency.mean > 100.0 {
                score *= 0.9;
            }
            if latency.p95 > 500.0 {
                score *= 0.85;
            }
        }
        
        if let Some(throughput) = metrics.get(&MetricType::Throughput) {
            // Higher throughput is better (assume normalized 0-1)
            score *= (1.0 - throughput.mean * 0.1).max(0.5);
        }
        
        score
    }

    fn _compute_cost_score(&self, metrics: &HashMap<MetricType, MetricStatistics>) -> f64 {
        let mut score = 1.0;
        
        if let Some(cost) = metrics.get(&MetricType::TotalCost) {
            // Assume cost is normalized (lower is better)
            score *= (1.0 - cost.mean * 0.5).max(0.3);
        }
        
        score
    }

    fn _compute_quality_score(&self, metrics: &HashMap<MetricType, MetricStatistics>) -> f64 {
        let mut score = 1.0;
        
        if let Some(error) = metrics.get(&MetricType::ErrorRate) {
            score *= (1.0 - error.mean).max(0.0);
        }
        
        if let Some(accuracy) = metrics.get(&MetricType::Accuracy) {
            score *= accuracy.mean;
        }
        
        score
    }

    fn _compute_reliability_score(&self, metrics: &HashMap<MetricType, MetricStatistics>) -> f64 {
        let mut score = 1.0;
        
        if let Some(availability) = metrics.get(&MetricType::Availability) {
            score *= availability.mean;
        }
        
        if let Some(mttr) = metrics.get(&MetricType::MeanTimeToRecovery) {
            // Lower MTTR is better (assume in minutes)
            if mttr.mean > 60.0 {
                score *= 0.8;
            }
        }
        
        score
    }

    fn _compute_optimization_score(&self, metrics: &HashMap<MetricType, MetricStatistics>) -> f64 {
        let mut score = 0.8; // Default baseline
        
        if let Some(opt_score) = metrics.get(&MetricType::OptimizationScore) {
            score = opt_score.mean;
        }
        
        if let Some(convergence) = metrics.get(&MetricType::ConvergenceRate) {
            score *= 0.8 + 0.2 * convergence.mean;
        }
        
        score
    }

    fn _export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.statistics)
            .map_err(|e| format!("Failed to export JSON: {}", e))
    }

    fn _export_csv(&self) -> Result<String, String> {
        let mut csv = String::from("metric_type,count,mean,std_dev,min,max,p50,p95,p99\n");
        
        for (metric_type, stats) in &self.statistics {
            csv.push_str(&format!(
                "{:?},{},{},{},{},{},{},{},{}\n",
                metric_type,
                stats.count,
                stats.mean,
                stats.std_dev,
                stats.min,
                stats.max,
                stats.median,
                stats.p95,
                stats.p99
            ));
        }
        
        Ok(csv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric_measurement_creation() {
        let measurement = MetricMeasurement::new(MetricType::CpuUtilization, 0.75)
            .with_unit("%".to_string())
            .with_source("system".to_string());
        
        assert_eq!(measurement.metric_type, MetricType::CpuUtilization);
        assert_eq!(measurement.value, 0.75);
        assert_eq!(measurement.unit, Some("%".to_string()));
    }

    #[test]
    fn test_statistics_update() {
        let mut stats = MetricStatistics::new(MetricType::ResponseTime);
        
        for i in 0..100 {
            stats.update(i as f64);
        }
        
        assert_eq!(stats.count, 100);
        assert!((stats.mean - 49.5).abs() < 1.0);
        assert_eq!(stats.min, 0.0);
        assert_eq!(stats.max, 99.0);
    }

    #[test]
    fn test_metrics_tracker_record() {
        let mut tracker = OptimizationMetrics::with_defaults();
        
        tracker.record_value(MetricType::CpuUtilization, 0.5);
        tracker.record_value(MetricType::CpuUtilization, 0.6);
        tracker.record_value(MetricType::CpuUtilization, 0.7);
        
        let stats = tracker.get_statistics(MetricType::CpuUtilization);
        assert!(stats.is_some());
        
        let stats = stats.unwrap();
        assert_eq!(stats.count, 3);
        assert!((stats.mean - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_trend_analysis() {
        let mut tracker = OptimizationMetrics::with_defaults();
        
        // Increasing trend
        for i in 0..50 {
            tracker.record_value(MetricType::ResponseTime, i as f64 * 2.0);
        }
        
        let trend = tracker.analyze_trend(MetricType::ResponseTime);
        assert!(trend.is_some());
        
        let trend = trend.unwrap();
        assert_eq!(trend.direction, TrendDirection::Increasing);
        assert!(trend.slope > 0.0);
    }

    #[test]
    fn test_alert_configuration() {
        let mut tracker = OptimizationMetrics::with_defaults();
        
        let alert_config = MetricAlertConfig {
            metric_type: MetricType::CpuUtilization,
            threshold_type: ThresholdType::Static,
            threshold: 0.8,
            comparison: ComparisonOperator::GreaterThan,
            duration_minutes: 5,
            severity: AlertSeverity::Warning,
            notification_channels: vec!["email".to_string()],
            enabled: true,
            cooldown_minutes: 30,
        };
        
        tracker.configure_alert(alert_config);
        
        // Record value that triggers alert
        tracker.record_value(MetricType::CpuUtilization, 0.9);
        
        let alerts = tracker.get_active_alerts();
        assert!(!alerts.is_empty());
    }

    #[test]
    fn test_report_generation() {
        let mut tracker = OptimizationMetrics::with_defaults();
        
        // Record various metrics
        tracker.record_value(MetricType::CpuUtilization, 0.5);
        tracker.record_value(MetricType::MemoryUtilization, 0.6);
        tracker.record_value(MetricType::ResponseTime, 100.0);
        tracker.record_value(MetricType::ErrorRate, 0.01);
        
        let time_range = TimeRange {
            start: Utc::now() - chrono::Duration::hours(1),
            end: Utc::now(),
            granularity: TimeGranularity::Hour,
        };
        
        let report = tracker.generate_report(time_range);
        
        assert!(!report.metrics_summary.is_empty());
        assert!(report.scorecard.overall_score > 0.0);
    }

    #[test]
    fn test_export_json() {
        let mut tracker = OptimizationMetrics::with_defaults();
        
        tracker.record_value(MetricType::CpuUtilization, 0.5);
        tracker.record_value(MetricType::MemoryUtilization, 0.6);
        
        let json = tracker.export(Some(ExportFormat::Json));
        assert!(json.is_ok());
        
        let json = json.unwrap();
        assert!(json.contains("CpuUtilization"));
    }
}