//! Real-Time Anomaly Detection for VANTIS OS
//!
//! Provides kernel-level anomaly detection using statistical methods
//! for system monitoring. Supports Z-Score, Modified Z-Score, IQR,
//! and EWMA detection methods with severity classification and cooldown.

use core::fmt;

// ============================================================================
// Detection Methods
// ============================================================================

/// Statistical anomaly detection methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionMethod {
    /// Standard Z-Score (mean ± k*σ)
    ZScore,
    /// Modified Z-Score using median absolute deviation
    ModifiedZScore,
    /// Interquartile Range method
    Iqr,
    /// Exponentially Weighted Moving Average
    Ewma,
    /// Isolation-based scoring
    IsolationScore,
}

/// Severity levels for detected anomalies
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnomalySeverity {
    /// Informational - slight deviation
    Info,
    /// Warning - notable deviation
    Warning,
    /// Critical - severe deviation requiring attention
    Critical,
    /// Emergency - extreme deviation requiring immediate action
    Emergency,
}

impl fmt::Display for AnomalySeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnomalySeverity::Info => write!(f, "INFO"),
            AnomalySeverity::Warning => write!(f, "WARNING"),
            AnomalySeverity::Critical => write!(f, "CRITICAL"),
            AnomalySeverity::Emergency => write!(f, "EMERGENCY"),
        }
    }
}

// ============================================================================
// Anomaly Record
// ============================================================================

/// A detected anomaly event
#[derive(Debug, Clone)]
pub struct AnomalyEvent {
    pub timestamp: u64,
    pub value: f64,
    pub expected_range: (f64, f64),
    pub deviation_score: f64,
    pub severity: AnomalySeverity,
    pub method: DetectionMethod,
    pub metric_name: String,
}

impl AnomalyEvent {
    /// How far outside the expected range the value is
    pub fn excess(&self) -> f64 {
        if self.value > self.expected_range.1 {
            self.value - self.expected_range.1
        } else if self.value < self.expected_range.0 {
            self.expected_range.0 - self.value
        } else {
            0.0
        }
    }
}

// ============================================================================
// Rolling Statistics
// ============================================================================

/// Maintains rolling statistics over a sliding window
#[derive(Debug, Clone)]
pub struct RollingStats {
    window: Vec<f64>,
    max_window_size: usize,
}

impl RollingStats {
    pub fn new(max_window_size: usize) -> Self {
        Self {
            window: Vec::with_capacity(max_window_size),
            max_window_size,
        }
    }

    /// Add a new observation
    pub fn push(&mut self, value: f64) {
        if self.window.len() >= self.max_window_size {
            self.window.remove(0);
        }
        self.window.push(value);
    }

    /// Number of observations
    pub fn count(&self) -> usize {
        self.window.len()
    }

    /// Arithmetic mean
    pub fn mean(&self) -> f64 {
        if self.window.is_empty() {
            return 0.0;
        }
        self.window.iter().sum::<f64>() / self.window.len() as f64
    }

    /// Standard deviation (sample)
    pub fn std_dev(&self) -> f64 {
        if self.window.len() < 2 {
            return 0.0;
        }
        let mean = self.mean();
        let variance: f64 = self.window.iter()
            .map(|&x| (x - mean) * (x - mean))
            .sum::<f64>() / (self.window.len() - 1) as f64;
        variance.sqrt()
    }

    /// Median value
    pub fn median(&self) -> f64 {
        if self.window.is_empty() {
            return 0.0;
        }
        let mut sorted = self.window.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }

    /// Median Absolute Deviation
    pub fn mad(&self) -> f64 {
        if self.window.is_empty() {
            return 0.0;
        }
        let med = self.median();
        let mut deviations: Vec<f64> = self.window.iter().map(|&x| (x - med).abs()).collect();
        deviations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
        let mid = deviations.len() / 2;
        if deviations.len() % 2 == 0 && deviations.len() >= 2 {
            (deviations[mid - 1] + deviations[mid]) / 2.0
        } else {
            deviations[mid]
        }
    }

    /// First quartile (Q1)
    pub fn q1(&self) -> f64 {
        if self.window.len() < 4 {
            return self.mean();
        }
        let mut sorted = self.window.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
        let idx = sorted.len() / 4;
        sorted[idx]
    }

    /// Third quartile (Q3)
    pub fn q3(&self) -> f64 {
        if self.window.len() < 4 {
            return self.mean();
        }
        let mut sorted = self.window.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
        let idx = (sorted.len() * 3) / 4;
        sorted[idx]
    }

    /// Interquartile range
    pub fn iqr(&self) -> f64 {
        self.q3() - self.q1()
    }

    /// Get the latest value
    pub fn latest(&self) -> Option<f64> {
        self.window.last().copied()
    }

    /// Get all values in the window
    pub fn values(&self) -> &[f64] {
        &self.window
    }
}

// ============================================================================
// EWMA State
// ============================================================================

/// Exponentially Weighted Moving Average tracker
#[derive(Debug, Clone)]
pub struct EwmaState {
    /// Smoothing factor (0 < α ≤ 1)
    pub alpha: f64,
    /// Current EWMA value
    pub ewma: f64,
    /// Current EWMA variance
    pub ewma_var: f64,
    /// Whether initialized
    pub initialized: bool,
}

impl EwmaState {
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha: alpha.clamp(0.01, 1.0),
            ewma: 0.0,
            ewma_var: 0.0,
            initialized: false,
        }
    }

    /// Update with a new observation
    pub fn update(&mut self, value: f64) {
        if !self.initialized {
            self.ewma = value;
            self.ewma_var = 0.0;
            self.initialized = true;
            return;
        }
        let diff = value - self.ewma;
        self.ewma += self.alpha * diff;
        self.ewma_var = (1.0 - self.alpha) * (self.ewma_var + self.alpha * diff * diff);
    }

    /// Current EWMA standard deviation
    pub fn std_dev(&self) -> f64 {
        self.ewma_var.sqrt()
    }
}

// ============================================================================
// Anomaly Detector
// ============================================================================

/// Configuration for the anomaly detector
#[derive(Debug, Clone)]
pub struct DetectorConfig {
    pub method: DetectionMethod,
    pub metric_name: String,
    /// Sensitivity multiplier (lower = more sensitive)
    pub threshold_multiplier: f64,
    /// Minimum observations before detection starts
    pub warmup_period: usize,
    /// Cooldown ticks between anomaly reports
    pub cooldown_ticks: u64,
    /// Window size for rolling statistics
    pub window_size: usize,
    /// EWMA alpha (only for EWMA method)
    pub ewma_alpha: f64,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self {
            method: DetectionMethod::ZScore,
            metric_name: "unknown".to_string(),
            threshold_multiplier: 3.0,
            warmup_period: 30,
            cooldown_ticks: 5,
            window_size: 100,
            ewma_alpha: 0.3,
        }
    }
}

/// The main anomaly detector
pub struct AnomalyDetector {
    config: DetectorConfig,
    stats: RollingStats,
    ewma: EwmaState,
    tick: u64,
    last_anomaly_tick: u64,
    anomaly_log: Vec<AnomalyEvent>,
    total_observations: u64,
    total_anomalies: u64,
}

impl AnomalyDetector {
    /// Create a new anomaly detector
    pub fn new(config: DetectorConfig) -> Self {
        let ewma = EwmaState::new(config.ewma_alpha);
        let stats = RollingStats::new(config.window_size);
        Self {
            config,
            stats,
            ewma,
            tick: 0,
            last_anomaly_tick: 0,
            anomaly_log: Vec::new(),
            total_observations: 0,
            total_anomalies: 0,
        }
    }

    /// Create with default Z-Score configuration
    pub fn zscore(metric_name: &str) -> Self {
        Self::new(DetectorConfig {
            method: DetectionMethod::ZScore,
            metric_name: metric_name.to_string(),
            ..Default::default()
        })
    }

    /// Create with EWMA configuration
    pub fn ewma(metric_name: &str, alpha: f64) -> Self {
        Self::new(DetectorConfig {
            method: DetectionMethod::Ewma,
            metric_name: metric_name.to_string(),
            ewma_alpha: alpha,
            ..Default::default()
        })
    }

    /// Observe a new value and check for anomalies
    pub fn observe(&mut self, value: f64) -> Option<AnomalyEvent> {
        self.tick += 1;
        self.total_observations += 1;
        self.stats.push(value);
        self.ewma.update(value);

        // Don't detect during warmup
        if self.total_observations < self.config.warmup_period as u64 {
            return None;
        }

        // Check cooldown
        if self.tick - self.last_anomaly_tick < self.config.cooldown_ticks {
            return None;
        }

        let (is_anomaly, score, expected_range) = self.detect(value);

        if is_anomaly {
            let severity = self.classify_severity(score);
            let event = AnomalyEvent {
                timestamp: self.tick,
                value,
                expected_range,
                deviation_score: score,
                severity,
                method: self.config.method,
                metric_name: self.config.metric_name.clone(),
            };

            self.last_anomaly_tick = self.tick;
            self.total_anomalies += 1;
            self.anomaly_log.push(event.clone());
            Some(event)
        } else {
            None
        }
    }

    /// Run detection based on configured method
    fn detect(&self, value: f64) -> (bool, f64, (f64, f64)) {
        match self.config.method {
            DetectionMethod::ZScore => self.detect_zscore(value),
            DetectionMethod::ModifiedZScore => self.detect_modified_zscore(value),
            DetectionMethod::Iqr => self.detect_iqr(value),
            DetectionMethod::Ewma => self.detect_ewma(value),
            DetectionMethod::IsolationScore => self.detect_isolation(value),
        }
    }

    /// Z-Score detection: |z| > threshold
    fn detect_zscore(&self, value: f64) -> (bool, f64, (f64, f64)) {
        let mean = self.stats.mean();
        let std = self.stats.std_dev();
        if std < 1e-10 {
            return (false, 0.0, (mean, mean));
        }
        let z = (value - mean).abs() / std;
        let k = self.config.threshold_multiplier;
        let range = (mean - k * std, mean + k * std);
        (z > k, z, range)
    }

    /// Modified Z-Score using MAD
    fn detect_modified_zscore(&self, value: f64) -> (bool, f64, (f64, f64)) {
        let median = self.stats.median();
        let mad = self.stats.mad();
        if mad < 1e-10 {
            return (false, 0.0, (median, median));
        }
        // Modified Z-Score = 0.6745 * (x - median) / MAD
        let mz = 0.6745 * (value - median).abs() / mad;
        let k = self.config.threshold_multiplier;
        let range_half = k * mad / 0.6745;
        let range = (median - range_half, median + range_half);
        (mz > k, mz, range)
    }

    /// IQR detection: value outside [Q1 - k*IQR, Q3 + k*IQR]
    fn detect_iqr(&self, value: f64) -> (bool, f64, (f64, f64)) {
        let q1 = self.stats.q1();
        let q3 = self.stats.q3();
        let iqr = self.stats.iqr();
        let k = self.config.threshold_multiplier;
        let lower = q1 - k * iqr;
        let upper = q3 + k * iqr;

        let score = if value < lower {
            (lower - value) / iqr.max(1e-10)
        } else if value > upper {
            (value - upper) / iqr.max(1e-10)
        } else {
            0.0
        };

        (value < lower || value > upper, score, (lower, upper))
    }

    /// EWMA detection: deviation from EWMA exceeds threshold
    fn detect_ewma(&self, value: f64) -> (bool, f64, (f64, f64)) {
        let ewma_val = self.ewma.ewma;
        let ewma_std = self.ewma.std_dev().max(self.stats.std_dev() * 0.1);
        if ewma_std < 1e-10 {
            return (false, 0.0, (ewma_val, ewma_val));
        }
        let deviation = (value - ewma_val).abs() / ewma_std;
        let k = self.config.threshold_multiplier;
        let range = (ewma_val - k * ewma_std, ewma_val + k * ewma_std);
        (deviation > k, deviation, range)
    }

    /// Isolation-based scoring (simplified)
    fn detect_isolation(&self, value: f64) -> (bool, f64, (f64, f64)) {
        // Use distance from median normalized by range
        let median = self.stats.median();
        let values = self.stats.values();
        if values.is_empty() {
            return (false, 0.0, (0.0, 0.0));
        }

        let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = (max_val - min_val).max(1e-10);

        let isolation_score = (value - median).abs() / range;
        let k = self.config.threshold_multiplier;
        let threshold = 0.5 * k; // normalized threshold
        let range_bounds = (min_val - range * 0.1, max_val + range * 0.1);

        (isolation_score > threshold, isolation_score, range_bounds)
    }

    /// Classify anomaly severity based on deviation score
    fn classify_severity(&self, score: f64) -> AnomalySeverity {
        let k = self.config.threshold_multiplier;
        if score > k * 3.0 {
            AnomalySeverity::Emergency
        } else if score > k * 2.0 {
            AnomalySeverity::Critical
        } else if score > k * 1.5 {
            AnomalySeverity::Warning
        } else {
            AnomalySeverity::Info
        }
    }

    /// Get total observations processed
    pub fn total_observations(&self) -> u64 {
        self.total_observations
    }

    /// Get total anomalies detected
    pub fn total_anomalies(&self) -> u64 {
        self.total_anomalies
    }

    /// Anomaly rate (anomalies / observations)
    pub fn anomaly_rate(&self) -> f64 {
        if self.total_observations == 0 {
            return 0.0;
        }
        self.total_anomalies as f64 / self.total_observations as f64
    }

    /// Get the anomaly log
    pub fn anomaly_log(&self) -> &[AnomalyEvent] {
        &self.anomaly_log
    }

    /// Get current rolling statistics
    pub fn current_stats(&self) -> &RollingStats {
        &self.stats
    }

    /// Reset the detector state
    pub fn reset(&mut self) {
        self.stats = RollingStats::new(self.config.window_size);
        self.ewma = EwmaState::new(self.config.ewma_alpha);
        self.tick = 0;
        self.last_anomaly_tick = 0;
        self.anomaly_log.clear();
        self.total_observations = 0;
        self.total_anomalies = 0;
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_stats_basic() {
        let mut stats = RollingStats::new(10);
        for i in 1..=5 {
            stats.push(i as f64);
        }
        assert_eq!(stats.count(), 5);
        assert!((stats.mean() - 3.0).abs() < 1e-10);
        assert!((stats.median() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_rolling_stats_window() {
        let mut stats = RollingStats::new(3);
        for i in 1..=5 {
            stats.push(i as f64);
        }
        assert_eq!(stats.count(), 3);
        assert!((stats.mean() - 4.0).abs() < 1e-10); // [3, 4, 5]
    }

    #[test]
    fn test_rolling_stats_std_dev() {
        let mut stats = RollingStats::new(100);
        stats.push(2.0);
        stats.push(4.0);
        stats.push(4.0);
        stats.push(4.0);
        stats.push(5.0);
        stats.push(5.0);
        stats.push(7.0);
        stats.push(9.0);
        let std = stats.std_dev();
        assert!(std > 1.0 && std < 3.0);
    }

    #[test]
    fn test_rolling_stats_iqr() {
        let mut stats = RollingStats::new(100);
        for i in 1..=20 {
            stats.push(i as f64);
        }
        let q1 = stats.q1();
        let q3 = stats.q3();
        assert!(q1 < q3);
        assert!(stats.iqr() > 0.0);
    }

    #[test]
    fn test_ewma_state() {
        let mut ewma = EwmaState::new(0.3);
        assert!(!ewma.initialized);
        ewma.update(10.0);
        assert!(ewma.initialized);
        assert!((ewma.ewma - 10.0).abs() < 1e-10);
        ewma.update(10.0);
        assert!((ewma.ewma - 10.0).abs() < 1e-10);
        ewma.update(20.0);
        assert!(ewma.ewma > 10.0 && ewma.ewma < 20.0);
    }

    #[test]
    fn test_zscore_no_anomaly() {
        let mut detector = AnomalyDetector::zscore("cpu_usage");
        // Feed normal data
        for i in 0..50 {
            let value = 50.0 + (i as f64 * 0.1).sin();
            let result = detector.observe(value);
            // During warmup, should be None
            if i < 30 {
                assert!(result.is_none());
            }
        }
        // Normal values shouldn't trigger anomalies
        assert!(detector.total_anomalies() < 5);
    }

    #[test]
    fn test_zscore_detects_anomaly() {
        let mut detector = AnomalyDetector::new(DetectorConfig {
            method: DetectionMethod::ZScore,
            metric_name: "cpu_usage".to_string(),
            threshold_multiplier: 2.0,
            warmup_period: 10,
            cooldown_ticks: 1,
            window_size: 50,
            ..Default::default()
        });

        // Feed stable data
        for _ in 0..20 {
            detector.observe(50.0);
        }

        // Inject anomaly
        let result = detector.observe(200.0);
        assert!(result.is_some());
        let event = result.unwrap();
        assert!(event.severity >= AnomalySeverity::Info);
        assert!(event.deviation_score > 2.0);
    }

    #[test]
    fn test_iqr_detection() {
        let mut detector = AnomalyDetector::new(DetectorConfig {
            method: DetectionMethod::Iqr,
            metric_name: "memory".to_string(),
            threshold_multiplier: 1.5,
            warmup_period: 20,
            cooldown_ticks: 1,
            window_size: 50,
            ..Default::default()
        });

        for i in 0..30 {
            detector.observe(100.0 + (i % 5) as f64);
        }

        let result = detector.observe(500.0);
        assert!(result.is_some());
    }

    #[test]
    fn test_ewma_detection() {
        let mut detector = AnomalyDetector::ewma("latency", 0.2);

        for _ in 0..40 {
            detector.observe(10.0);
        }

        // Sudden spike
        let result = detector.observe(100.0);
        assert!(result.is_some());
    }

    #[test]
    fn test_cooldown() {
        let mut detector = AnomalyDetector::new(DetectorConfig {
            method: DetectionMethod::ZScore,
            metric_name: "test".to_string(),
            threshold_multiplier: 2.0,
            warmup_period: 10,
            cooldown_ticks: 5,
            window_size: 50,
            ..Default::default()
        });

        for _ in 0..20 {
            detector.observe(50.0);
        }

        // First anomaly should be detected
        let r1 = detector.observe(200.0);
        assert!(r1.is_some());

        // Second anomaly within cooldown should be suppressed
        let r2 = detector.observe(200.0);
        assert!(r2.is_none());
    }

    #[test]
    fn test_severity_classification() {
        let mut detector = AnomalyDetector::new(DetectorConfig {
            method: DetectionMethod::ZScore,
            metric_name: "test".to_string(),
            threshold_multiplier: 2.0,
            warmup_period: 10,
            cooldown_ticks: 1,
            window_size: 50,
            ..Default::default()
        });

        for _ in 0..20 {
            detector.observe(50.0);
        }

        // Extreme anomaly should be high severity
        let result = detector.observe(1000.0);
        assert!(result.is_some());
        let event = result.unwrap();
        assert!(event.severity >= AnomalySeverity::Warning);
    }

    #[test]
    fn test_anomaly_rate() {
        let mut detector = AnomalyDetector::new(DetectorConfig {
            method: DetectionMethod::ZScore,
            metric_name: "test".to_string(),
            warmup_period: 5,
            cooldown_ticks: 1,
            ..Default::default()
        });

        for _ in 0..100 {
            detector.observe(50.0);
        }

        assert!(detector.anomaly_rate() < 0.1); // Low anomaly rate for stable data
        assert_eq!(detector.total_observations(), 100);
    }

    #[test]
    fn test_reset() {
        let mut detector = AnomalyDetector::zscore("test");
        for _ in 0..50 {
            detector.observe(50.0);
        }
        assert!(detector.total_observations() > 0);

        detector.reset();
        assert_eq!(detector.total_observations(), 0);
        assert_eq!(detector.total_anomalies(), 0);
        assert!(detector.anomaly_log().is_empty());
    }
}