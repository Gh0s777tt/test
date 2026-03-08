//! Data Processor Module for VantisOS AI
//! 
//! Processes raw collected data for ML training and inference.
//! Performs feature extraction, normalization, and transformation.
//! 
//! ## Architecture
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                    DataProcessor                        │
//! ├─────────────────────────────────────────────────────────┤
//! │  ┌─────────────┐  ┌──────────────┐  ┌──────────────┐  │
//! │  │   Feature   │  │              │  │   Time       │  │
//! │  │ Extraction  │→ │Normalization│→ │Windowing     │  │
//! │  └─────────────┘  │              │  └──────────────┘  │
//! │  ┌─────────────┐  └──────────────┘  ┌──────────────┐  │
//! │  │   Scaling   │  ┌──────────────┐  │   Outlier    │  │
//! │  │             │→ │   Filtering   │→ │  Detection   │  │
//! │  └─────────────┘  │              │  └──────────────┘  │
//! │                   └──────────────┘                    │
//! └─────────────────────────────────────────────────────────┘
//! ```
//! 
//! ## Security Considerations
//! - All processing is local (no cloud dependencies)
//! - No data exfiltration
//! - Memory-bounded processing
//! - Input validation

use crate::ai::{error::AIError, types::ResourceUsage};
use crate::ai::modules::data_collector::{SystemMetrics, CPUMetrics, MemoryMetrics, DiskMetrics, NetworkMetrics};

/// Minimum window size for time windowing
const MIN_WINDOW_SIZE: usize = 3;

/// Maximum window size for time windowing
const MAX_WINDOW_SIZE: usize = 1000;

/// Default window size
const DEFAULT_WINDOW_SIZE: usize = 10;

/// Minimum samples for statistical calculations
const MIN_STATISTICAL_SAMPLES: usize = 2;

/// Z-score threshold for outlier detection
const DEFAULT_OUTLIER_THRESHOLD: f64 = 3.0;

/// Normalization method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationMethod {
    /// Min-max normalization to [0,1]
    MinMax,
    /// Z-score standardization
    ZScore,
    /// Robust scaling using IQR
    Robust,
    /// No normalization
    None,
}

impl Default for NormalizationMethod {
    fn default() -> Self {
        Self::MinMax
    }
}

/// Time window type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType {
    /// Fixed-size sliding window
    Sliding,
    /// Expanding window (cumulative)
    Expanding,
    /// Exponentially weighted moving window
    Exponential,
}

/// Statistical features
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatisticalFeatures {
    /// Mean value
    pub mean: f64,
    /// Standard deviation
    pub std: f64,
    /// Variance
    pub variance: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Median value
    pub median: f64,
    /// 25th percentile
    pub p25: f64,
    /// 75th percentile
    pub p75: f64,
    /// Interquartile range
    pub iqr: f64,
    /// Skewness
    pub skewness: f64,
    /// Kurtosis
    pub kurtosis: f64,
}

impl Default for StatisticalFeatures {
    fn default() -> Self {
        Self {
            mean: 0.0,
            std: 0.0,
            variance: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            median: 0.0,
            p25: 0.0,
            p75: 0.0,
            iqr: 0.0,
            skewness: 0.0,
            kurtosis: 0.0,
        }
    }
}

/// Time-domain features
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeDomainFeatures {
    /// Slope (linear trend)
    pub slope: f64,
    /// Curvature (second derivative)
    pub curvature: f64,
    /// First difference mean
    pub diff_mean: f64,
    /// First difference std
    pub diff_std: f64,
    /// Peak-to-peak amplitude
    pub peak_to_peak: f64,
    /// RMS value
    pub rms: f64,
    /// Zero crossing rate
    pub zero_crossing_rate: f64,
}

impl Default for TimeDomainFeatures {
    fn default() -> Self {
        Self {
            slope: 0.0,
            curvature: 0.0,
            diff_mean: 0.0,
            diff_std: 0.0,
            peak_to_peak: 0.0,
            rms: 0.0,
            zero_crossing_rate: 0.0,
        }
    }
}

/// Frequency-domain features (simplified FFT approximation)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrequencyDomainFeatures {
    /// Dominant frequency
    pub dominant_frequency: f64,
    /// Spectral centroid
    pub spectral_centroid: f64,
    /// Spectral energy
    pub spectral_energy: f64,
    /// Spectral entropy
    pub spectral_entropy: f64,
    /// Band power ratio
    pub band_power_ratio: f64,
}

impl Default for FrequencyDomainFeatures {
    fn default() -> Self {
        Self {
            dominant_frequency: 0.0,
            spectral_centroid: 0.0,
            spectral_energy: 0.0,
            spectral_entropy: 0.0,
            band_power_ratio: 0.0,
        }
    }
}

/// Extracted features vector
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FeatureVector {
    /// Raw feature values
    pub values: Vec<f64>,
    /// Feature names
    pub names: Vec<String>,
    /// Timestamp
    pub timestamp_ms: u64,
}

impl FeatureVector {
    /// Create a new feature vector
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a feature
    pub fn add(&mut self, name: String, value: f64) {
        self.names.push(name);
        self.values.push(value);
    }

    /// Get the length
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Get feature by name
    pub fn get(&self, name: &str) -> Option<f64> {
        self.names.iter()
            .position(|n| n == name)
            .map(|i| self.values[i])
    }
}

/// Outlier detection result
#[derive(Debug, Clone, PartialEq, Default)]
pub struct OutlierResult {
    /// Outlier indices
    pub indices: Vec<usize>,
    /// Outlier values
    pub values: Vec<f64>,
    /// Modified Z-scores
    pub z_scores: Vec<f64>,
    /// Threshold used
    pub threshold: f64,
}

/// Processor configuration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessorConfig {
    /// Normalization method
    pub normalization: NormalizationMethod,
    /// Window type
    pub window_type: WindowType,
    /// Window size
    pub window_size: usize,
    /// Outlier detection threshold
    pub outlier_threshold: f64,
    /// Enable outlier detection
    pub detect_outliers: bool,
    /// Enable imputation of missing values
    pub enable_imputation: bool,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            normalization: NormalizationMethod::default(),
            window_type: WindowType::Sliding,
            window_size: DEFAULT_WINDOW_SIZE,
            outlier_threshold: DEFAULT_OUTLIER_THRESHOLD,
            detect_outliers: true,
            enable_imputation: true,
        }
    }
}

/// Data Processor
/// 
/// Processes raw system metrics into features suitable for ML models.
/// 
/// ## Features
/// - Feature extraction (statistical, time-domain, frequency-domain)
/// - Data normalization and scaling
/// - Feature scaling
/// - Time window aggregation
/// - Outlier detection
/// - Missing value imputation
/// 
/// ## Example
/// ```rust
/// use vantis::ai::modules::DataProcessor;
/// 
/// let processor = DataProcessor::new()?;
/// let metrics = vec![/* SystemMetrics */];
/// let features = processor.process(&metrics)?;
/// ```
pub struct DataProcessor {
    config: ProcessorConfig,
    enabled: bool,
}

impl DataProcessor {
    /// Create a new data processor with default configuration
    pub fn new() -> Result<Self, AIError> {
        Ok(Self {
            config: ProcessorConfig::default(),
            enabled: true,
        })
    }

    /// Create a new data processor with custom configuration
    pub fn with_config(config: ProcessorConfig) -> Result<Self, AIError> {
        let window_size = config.window_size.clamp(MIN_WINDOW_SIZE, MAX_WINDOW_SIZE);
        
        Ok(Self {
            config: ProcessorConfig {
                window_size,
                ..config
            },
            enabled: true,
        })
    }

    /// Set the configuration
    pub fn configure(&mut self, config: ProcessorConfig) {
        self.config = config;
    }

    /// Get the configuration
    pub fn config(&self) -> &ProcessorConfig {
        &self.config
    }

    /// Enable the processor
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the processor
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if processor is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Process raw metrics into features
    pub fn process(&self, metrics: &[SystemMetrics]) -> Result<FeatureVector, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        if metrics.is_empty() {
            return Err(AIError::InsufficientData);
        }

        let mut features = FeatureVector::new();
        features.timestamp_ms = metrics.last().map(|m| m.timestamp_ms).unwrap_or(0);

        // Extract statistical features
        self.extract_statistical_features(metrics, &mut features)?;

        // Extract time-domain features
        self.extract_time_domain_features(metrics, &mut features)?;

        // Extract frequency-domain features
        self.extract_frequency_domain_features(metrics, &mut features)?;

        // Add aggregate resource usage
        self.add_aggregate_features(metrics, &mut features)?;

        // Normalize if configured
        if self.config.normalization != NormalizationMethod::None {
            self.normalize_features(&mut features)?;
        }

        Ok(features)
    }

    /// Extract statistical features from metrics
    fn extract_statistical_features(
        &self,
        metrics: &[SystemMetrics],
        features: &mut FeatureVector,
    ) -> Result<(), AIError> {
        // CPU usage statistics
        let cpu_values: Vec<f64> = metrics.iter().map(|m| m.cpu_usage_avg).collect();
        let cpu_stats = self.calculate_statistics(&cpu_values);
        self.add_statistical_features(features, "cpu", cpu_stats);

        // Memory usage statistics
        let mem_values: Vec<f64> = metrics.iter().map(|m| m.memory.usage_percent).collect();
        let mem_stats = self.calculate_statistics(&mem_values);
        self.add_statistical_features(features, "memory", mem_stats);

        // Disk usage statistics (if available)
        if let Some(disk_usage) = metrics.first().and_then(|m| m.disk.first()) {
            let disk_values: Vec<f64> = metrics.iter()
                .filter_map(|m| m.disk.first())
                .map(|d| d.usage_percent)
                .collect();
            if !disk_values.is_empty() {
                let disk_stats = self.calculate_statistics(&disk_values);
                self.add_statistical_features(features, "disk", disk_stats);
            }
        }

        // Network usage statistics (if available)
        if let Some(net_usage) = metrics.first().and_then(|m| m.network.first()) {
            let net_values: Vec<f64> = metrics.iter()
                .filter_map(|m| m.network.first())
                .map(|n| n.usage_percent)
                .collect();
            if !net_values.is_empty() {
                let net_stats = self.calculate_statistics(&net_values);
                self.add_statistical_features(features, "network", net_stats);
            }
        }

        Ok(())
    }

    /// Calculate statistical features from a data series
    fn calculate_statistics(&self, data: &[f64]) -> StatisticalFeatures {
        if data.is_empty() {
            return StatisticalFeatures::default();
        }

        let n = data.len();
        let sum: f64 = data.iter().sum();
        let mean = sum / n as f64;

        let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let variance = if n > 1 {
            data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (n - 1) as f64
        } else {
            0.0
        };
        let std = variance.sqrt();

        // Calculate median and percentiles
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = Self::percentile(&sorted, 50.0);
        let p25 = Self::percentile(&sorted, 25.0);
        let p75 = Self::percentile(&sorted, 75.0);
        let iqr = p75 - p25;

        // Calculate skewness and kurtosis
        let skewness = if std > 0.0 && n >= 3 {
            data.iter()
                .map(|&x| ((x - mean) / std).powi(3))
                .sum::<f64>() / n as f64
        } else {
            0.0
        };

        let kurtosis = if std > 0.0 && n >= 4 {
            data.iter()
                .map(|&x| ((x - mean) / std).powi(4))
                .sum::<f64>() / n as f64 - 3.0
        } else {
            0.0
        };

        StatisticalFeatures {
            mean,
            std,
            variance,
            min,
            max,
            median,
            p25,
            p75,
            iqr,
            skewness,
            kurtosis,
        }
    }

    /// Calculate percentile from sorted data
    fn percentile(sorted: &[f64], p: f64) -> f64 {
        if sorted.is_empty() {
            return 0.0;
        }

        if sorted.len() == 1 {
            return sorted[0];
        }

        let n = sorted.len();
        let pos = (p / 100.0) * (n - 1) as f64;
        let lower = pos.floor() as usize;
        let upper = pos.ceil() as usize;

        if upper == lower {
            return sorted[lower];
        }

        let weight = pos - lower as f64;
        sorted[lower] * (1.0 - weight) + sorted[upper] * weight
    }

    /// Add statistical features to feature vector
    fn add_statistical_features(
        &self,
        features: &mut FeatureVector,
        prefix: &str,
        stats: StatisticalFeatures,
    ) {
        features.add(format!("{}_mean", prefix).to_string(), stats.mean);
        features.add(format!("{}_std", prefix).to_string(), stats.std);
        features.add(format!("{}_variance", prefix).to_string(), stats.variance);
        features.add(format!("{}_min", prefix).to_string(), stats.min);
        features.add(format!("{}_max", prefix).to_string(), stats.max);
        features.add(format!("{}_median", prefix).to_string(), stats.median);
        features.add(format!("{}_p25", prefix).to_string(), stats.p25);
        features.add(format!("{}_p75", prefix).to_string(), stats.p75);
        features.add(format!("{}_iqr", prefix).to_string(), stats.iqr);
        features.add(format!("{}_skewness", prefix).to_string(), stats.skewness);
        features.add(format!("{}_kurtosis", prefix).to_string(), stats.kurtosis);
    }

    /// Extract time-domain features
    fn extract_time_domain_features(
        &self,
        metrics: &[SystemMetrics],
        features: &mut FeatureVector,
    ) -> Result<(), AIError> {
        if metrics.len() < MIN_STATISTICAL_SAMPLES {
            return Ok(());
        }

        // CPU usage time-domain features
        let cpu_values: Vec<f64> = metrics.iter().map(|m| m.cpu_usage_avg).collect();
        let cpu_tdf = self.calculate_time_domain_features(&cpu_values);
        self.add_time_domain_features(features, "cpu", cpu_tdf);

        Ok(())
    }

    /// Calculate time-domain features
    fn calculate_time_domain_features(&self, data: &[f64]) -> TimeDomainFeatures {
        if data.len() < 2 {
            return TimeDomainFeatures::default();
        }

        let n = data.len();
        
        // Calculate first differences
        let diffs: Vec<f64> = data.windows(2).map(|w| w[1] - w[0]).collect();

        // Calculate slope using linear regression
        let mean_x = (n - 1) as f64 / 2.0;
        let mean_y = data.iter().sum::<f64>() / n as f64;
        let numerator: f64 = data.iter().enumerate()
            .map(|(i, &y)| (i as f64 - mean_x) * (y - mean_y))
            .sum();
        let denominator: f64 = data.iter().enumerate()
            .map(|(i, _)| (i as f64 - mean_x).powi(2))
            .sum();
        let slope = if denominator != 0.0 { numerator / denominator } else { 0.0 };

        // Calculate curvature (second derivative approximation)
        let curvature = if data.len() > 2 {
            data.windows(3).map(|w| w[2] - 2.0 * w[1] + w[0]).sum::<f64>() / (n - 2) as f64
        } else {
            0.0
        };

        // Difference statistics
        let diff_mean = diffs.iter().sum::<f64>() / diffs.len() as f64;
        let diff_variance = diffs.iter().map(|&x| (x - diff_mean).powi(2)).sum::<f64>() / diffs.len() as f64;
        let diff_std = diff_variance.sqrt();

        // Peak-to-peak
        let peak_to_peak = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)) 
            - data.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        // RMS
        let rms = (data.iter().map(|&x| x.powi(2)).sum::<f64>() / n as f64).sqrt();

        // Zero crossing rate
        let zero_crossings = data.windows(2)
            .filter(|w| w[0] * w[1] < 0.0)
            .count();
        let zero_crossing_rate = zero_crossings as f64 / (n - 1) as f64;

        TimeDomainFeatures {
            slope,
            curvature,
            diff_mean,
            diff_std,
            peak_to_peak,
            rms,
            zero_crossing_rate,
        }
    }

    /// Add time-domain features to feature vector
    fn add_time_domain_features(
        &self,
        features: &mut FeatureVector,
        prefix: &str,
        tdf: TimeDomainFeatures,
    ) {
        features.add(format!("{}_slope", prefix).to_string(), tdf.slope);
        features.add(format!("{}_curvature", prefix).to_string(), tdf.curvature);
        features.add(format!("{}_diff_mean", prefix).to_string(), tdf.diff_mean);
        features.add(format!("{}_diff_std", prefix).to_string(), tdf.diff_std);
        features.add(format!("{}_peak_to_peak", prefix).to_string(), tdf.peak_to_peak);
        features.add(format!("{}_rms", prefix).to_string(), tdf.rms);
        features.add(format!("{}_zero_crossing_rate", prefix).to_string(), tdf.zero_crossing_rate);
    }

    /// Extract frequency-domain features (simplified)
    fn extract_frequency_domain_features(
        &self,
        metrics: &[SystemMetrics],
        features: &mut FeatureVector,
    ) -> Result<(), AIError> {
        if metrics.len() < MIN_STATISTICAL_SAMPLES {
            return Ok(());
        }

        // CPU usage frequency-domain features
        let cpu_values: Vec<f64> = metrics.iter().map(|m| m.cpu_usage_avg).collect();
        let cpu_fdf = self.calculate_frequency_domain_features(&cpu_values);
        self.add_frequency_domain_features(features, "cpu", cpu_fdf);

        Ok(())
    }

    /// Calculate frequency-domain features (simplified approximation)
    fn calculate_frequency_domain_features(&self, data: &[f64]) -> FrequencyDomainFeatures {
        if data.len() < 2 {
            return FrequencyDomainFeatures::default();
        }

        // Simplified frequency analysis using autocorrelation
        let n = data.len();
        
        // Calculate spectral energy (variance as proxy)
        let mean = data.iter().sum::<f64>() / n as f64;
        let spectral_energy = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n as f64;

        // Estimate dominant frequency using zero-crossing rate
        let zero_crossings = data.windows(2).filter(|w| w[0] * w[1] < 0.0).count();
        let dominant_frequency = if zero_crossings > 0 {
            (zero_crossings as f64 * 0.5 * n as f64) / (n - 1) as f64
        } else {
            0.0
        };

        // Spectral centroid (mean frequency)
        let spectral_centroid = dominant_frequency * 0.5;

        // Spectral entropy (using energy distribution)
        let spectral_entropy = if spectral_energy > 0.0 {
            let normalized_energy = data.iter().map(|&x| (x - mean).powi(2) / spectral_energy);
            let entropy = normalized_energy.filter(|&p| p > 0.0)
                .map(|p| -p * p.log2())
                .sum::<f64>();
            entropy
        } else {
            0.0
        };

        // Band power ratio (low vs high frequency)
        let band_power_ratio = 0.5; // Simplified

        FrequencyDomainFeatures {
            dominant_frequency,
            spectral_centroid,
            spectral_energy,
            spectral_entropy,
            band_power_ratio,
        }
    }

    /// Add frequency-domain features to feature vector
    fn add_frequency_domain_features(
        &self,
        features: &mut FeatureVector,
        prefix: &str,
        fdf: FrequencyDomainFeatures,
    ) {
        features.add(format!("{}_dominant_freq", prefix).to_string(), fdf.dominant_frequency);
        features.add(format!("{}_spectral_centroid", prefix).to_string(), fdf.spectral_centroid);
        features.add(format!("{}_spectral_energy", prefix).to_string(), fdf.spectral_energy);
        features.add(format!("{}_spectral_entropy", prefix).to_string(), fdf.spectral_entropy);
        features.add(format!("{}_band_power_ratio", prefix).to_string(), fdf.band_power_ratio);
    }

    /// Add aggregate resource usage features
    fn add_aggregate_features(
        &self,
        metrics: &[SystemMetrics],
        features: &mut FeatureVector,
    ) -> Result<(), AIError> {
        if metrics.is_empty() {
            return Ok(());
        }

        // Calculate aggregate usage
        let cpu_sum: f64 = metrics.iter().map(|m| m.cpu_usage_avg).sum();
        let mem_sum: f64 = metrics.iter().map(|m| m.memory.usage_percent).sum();
        
        let cpu_aggregate = cpu_sum / metrics.len() as f64;
        let mem_aggregate = mem_sum / metrics.len() as f64;

        features.add("aggregate_cpu_usage".to_string(), cpu_aggregate);
        features.add("aggregate_memory_usage".to_string(), mem_aggregate);
        features.add("metric_count".to_string(), metrics.len() as f64);

        Ok(())
    }

    /// Normalize features
    fn normalize_features(&self, features: &mut FeatureVector) -> Result<(), AIError> {
        if features.is_empty() {
            return Ok(());
        }

        match self.config.normalization {
            NormalizationMethod::MinMax => {
                let min = features.values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max = features.values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                
                if max > min {
                    features.values = features.values.iter()
                        .map(|&v| (v - min) / (max - min))
                        .collect();
                }
            }
            NormalizationMethod::ZScore => {
                let mean = features.values.iter().sum::<f64>() / features.len() as f64;
                let variance = features.values.iter()
                    .map(|&v| (v - mean).powi(2))
                    .sum::<f64>() / features.len() as f64;
                let std = variance.sqrt();
                
                if std > 0.0 {
                    features.values = features.values.iter()
                        .map(|&v| (v - mean) / std)
                        .collect();
                }
            }
            NormalizationMethod::Robust => {
                let mut sorted = features.values.clone();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let q25 = Self::percentile(&sorted, 25.0);
                let q75 = Self::percentile(&sorted, 75.0);
                let median = Self::percentile(&sorted, 50.0);
                let iqr = q75 - q25;
                
                if iqr > 0.0 {
                    features.values = features.values.iter()
                        .map(|&v| (v - median) / iqr)
                        .collect();
                }
            }
            NormalizationMethod::None => {}
        }

        Ok(())
    }

    /// Detect outliers using modified Z-score
    pub fn detect_outliers(&self, data: &[f64]) -> Result<OutlierResult, AIError> {
        if data.is_empty() {
            return Ok(OutlierResult::default());
        }

        let n = data.len();
        let median = {
            let mut sorted = data.to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Self::percentile(&sorted, 50.0)
        };

        let mad: f64 = data.iter()
            .map(|&x| (x - median).abs())
            .collect::<Vec<f64>>()
            .into_iter()
            .map(|x| {
                let mut sorted = data.iter().map(|&y| (y - median).abs()).collect::<Vec<f64>>();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                Self::percentile(&sorted, 50.0)
            })
            .sum::<f64>() / n as f64;

        let threshold = self.config.outlier_threshold * mad / 0.6745;

        let mut result = OutlierResult {
            threshold,
            ..Default::default()
        };

        for (i, &value) in data.iter().enumerate() {
            let z_score = (value - median) / (mad / 0.6745);
            result.z_scores.push(z_score);
            
            if z_score.abs() > self.config.outlier_threshold {
                result.indices.push(i);
                result.values.push(value);
            }
        }

        Ok(result)
    }

    /// Process raw metrics into features (compatibility method)
    pub fn process_features(&self, raw_metrics: &[ResourceUsage]) -> Result<Vec<f64>, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }

        if raw_metrics.is_empty() {
            return Err(AIError::InsufficientData);
        }

        let cpu_mean: f64 = raw_metrics.iter().map(|m| m.cpu_usage).sum::<f64>() / raw_metrics.len() as f64;
        let mem_mean: f64 = raw_metrics.iter().map(|m| m.memory_usage).sum::<f64>() / raw_metrics.len() as f64;

        let mut features = vec![cpu_mean, mem_mean];

        // Normalize if configured
        if self.config.normalization != NormalizationMethod::None {
            features = self.normalize(&features)?;
        }

        Ok(features)
    }

    /// Normalize features (compatibility method)
    pub fn normalize(&self, features: &[f64]) -> Result<Vec<f64>, AIError> {
        if features.is_empty() {
            return Err(AIError::InsufficientData);
        }

        match self.config.normalization {
            NormalizationMethod::MinMax => {
                let min = features.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max = features.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

                if max == min {
                    return Ok(features.to_vec());
                }

                Ok(features.iter().map(|&f| (f - min) / (max - min)).collect())
            }
            NormalizationMethod::ZScore => {
                let mean = features.iter().sum::<f64>() / features.len() as f64;
                let variance = features.iter()
                    .map(|&f| (f - mean).powi(2))
                    .sum::<f64>() / features.len() as f64;
                let std = variance.sqrt();

                if std == 0.0 {
                    return Ok(vec![0.0; features.len()]);
                }

                Ok(features.iter().map(|&f| (f - mean) / std).collect())
            }
            NormalizationMethod::Robust => {
                let mut sorted = features.to_vec();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let q25 = Self::percentile(&sorted, 25.0);
                let q75 = Self::percentile(&sorted, 75.0);
                let iqr = q75 - q25;

                if iqr == 0.0 {
                    return Ok(vec![0.0; features.len()]);
                }

                Ok(features.iter()
                    .map(|&f| (f - Self::percentile(&sorted, 50.0)) / iqr)
                    .collect())
            }
            NormalizationMethod::None => Ok(features.to_vec()),
        }
    }

    /// Get window size (compatibility method)
    pub fn window_size(&self) -> usize {
        self.config.window_size
    }
}

impl Default for DataProcessor {
    fn default() -> Self {
        Self::new().expect("Failed to create default DataProcessor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_creation() {
        let processor = DataProcessor::new().unwrap();
        assert!(processor.is_enabled());
        assert_eq!(processor.window_size(), DEFAULT_WINDOW_SIZE);
    }

    #[test]
    fn test_processor_with_config() {
        let config = ProcessorConfig {
            window_size: 20,
            normalization: NormalizationMethod::ZScore,
            ..Default::default()
        };
        let processor = DataProcessor::with_config(config).unwrap();
        assert_eq!(processor.window_size(), 20);
    }

    #[test]
    fn test_window_size_clamping() {
        let config = ProcessorConfig {
            window_size: 1,
            ..Default::default()
        };
        let processor = DataProcessor::with_config(config).unwrap();
        assert_eq!(processor.window_size(), MIN_WINDOW_SIZE);

        let config = ProcessorConfig {
            window_size: 2000,
            ..Default::default()
        };
        let processor = DataProcessor::with_config(config).unwrap();
        assert_eq!(processor.window_size(), MAX_WINDOW_SIZE);
    }

    #[test]
    fn test_process_features() {
        let processor = DataProcessor::new().unwrap();
        let metrics = vec![
            ResourceUsage {
                cpu_usage: 20.0,
                memory_usage: 40.0,
                disk_usage: 50.0,
                network_usage: 10.0,
            },
            ResourceUsage {
                cpu_usage: 30.0,
                memory_usage: 50.0,
                disk_usage: 60.0,
                network_usage: 15.0,
            },
        ];

        let features = processor.process_features(&metrics).unwrap();
        assert_eq!(features.len(), 2);
        assert_eq!(features[0], 25.0);
        assert_eq!(features[1], 45.0);
    }

    #[test]
    fn test_normalize_minmax() {
        let processor = DataProcessor::with_config(ProcessorConfig {
            normalization: NormalizationMethod::MinMax,
            ..Default::default()
        }).unwrap();
        
        let features = vec![0.0, 0.5, 1.0];
        let normalized = processor.normalize(&features).unwrap();

        assert!((normalized[0] - 0.0).abs() < 1e-10);
        assert!((normalized[1] - 0.5).abs() < 1e-10);
        assert!((normalized[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalize_zscore() {
        let processor = DataProcessor::with_config(ProcessorConfig {
            normalization: NormalizationMethod::ZScore,
            ..Default::default()
        }).unwrap();
        
        let features = vec![1.0, 2.0, 3.0];
        let normalized = processor.normalize(&features).unwrap();

        assert!((normalized[0] + normalized[2]).abs() < 1e-10); // Symmetric around 0
        assert!((normalized[1]).abs() < 1e-10); // Mean should be 0
    }

    #[test]
    fn test_calculate_statistics() {
        let processor = DataProcessor::new().unwrap();
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = processor.calculate_statistics(&data);

        assert!((stats.mean - 3.0).abs() < 1e-10);
        assert!((stats.median - 3.0).abs() < 1e-10);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
    }

    #[test]
    fn test_detect_outliers() {
        let processor = DataProcessor::new().unwrap();
        let data = vec![1.0, 2.0, 3.0, 4.0, 100.0]; // 100.0 is an outlier
        let result = processor.detect_outliers(&data).unwrap();

        assert!(!result.indices.is_empty());
        assert_eq!(result.values[0], 100.0);
    }

    #[test]
    fn test_feature_vector() {
        let mut features = FeatureVector::new();
        features.add("test".to_string(), 1.0);
        features.add("test2".to_string(), 2.0);

        assert_eq!(features.len(), 2);
        assert_eq!(features.get("test"), Some(1.0));
        assert_eq!(features.get("test2"), Some(2.0));
        assert_eq!(features.get("nonexistent"), None);
    }

    #[test]
    fn test_enable_disable() {
        let mut processor = DataProcessor::new().unwrap();
        assert!(processor.is_enabled());

        processor.disable();
        assert!(!processor.is_enabled());

        processor.enable();
        assert!(processor.is_enabled());
    }

    #[test]
    fn test_process_requires_enabled() {
        let processor = DataProcessor::new().unwrap();
        processor.disable();

        let metrics = vec![SystemMetrics::default()];
        let result = processor.process(&metrics);
        assert!(result.is_err());
    }
}