//! A/B Testing Framework Module for VantisOS
//!
//! This module provides comprehensive A/B testing capabilities for optimization
//! experiments including experiment management, statistical analysis, and decision making.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============================================================================
// Core Types
// ============================================================================

/// A/B Test Experiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    /// Unique identifier
    pub id: String,
    /// Experiment name
    pub name: String,
    /// Description
    pub description: String,
    /// Experiment status
    pub status: ExperimentStatus,
    /// Variants being tested
    pub variants: Vec<Variant>,
    /// Metrics being measured
    pub metrics: Vec<MetricDefinition>,
    /// Targeting rules
    pub targeting: TargetingConfig,
    /// Traffic allocation
    pub traffic_allocation: TrafficAllocation,
    /// Statistical configuration
    pub statistical_config: StatisticalConfig,
    /// Start time
    pub started_at: Option<DateTime<Utc>>,
    /// End time
    pub ended_at: Option<DateTime<Utc>>,
    /// Created time
    pub created_at: DateTime<Utc>,
    /// Created by
    pub created_by: String,
    /// Tags
    pub tags: Vec<String>,
}

/// Experiment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperimentStatus {
    Draft,
    Running,
    Paused,
    Completed,
    Cancelled,
    Archived,
}

/// Test variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    /// Variant identifier
    pub id: String,
    /// Variant name
    pub name: String,
    /// Variant description
    pub description: String,
    /// Variant type
    pub variant_type: VariantType,
    /// Configuration for this variant
    pub config: HashMap<String, f64>,
    /// Traffic percentage (0-100)
    pub traffic_percentage: f64,
    /// Is this the control group?
    pub is_control: bool,
}

/// Types of variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariantType {
    Control,
    Treatment,
    FeatureFlag,
    Parameter,
    Configuration,
}

/// Metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDefinition {
    /// Metric identifier
    pub id: String,
    /// Metric name
    pub name: String,
    /// Metric description
    pub description: String,
    /// Type of metric
    pub metric_type: MetricType,
    /// How to aggregate the metric
    pub aggregation: AggregationMethod,
    /// Whether higher is better
    pub higher_is_better: bool,
    /// Minimum detectable effect
    pub minimum_detectable_effect: f64,
    /// Weight for composite metrics
    pub weight: f64,
    /// Whether this is a primary metric
    pub is_primary: bool,
}

/// Types of metrics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    Continuous,
    Binary,
    Count,
    Ratio,
    Duration,
    Percentage,
}

/// Aggregation methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationMethod {
    Mean,
    Median,
    Sum,
    Count,
    Min,
    Max,
    Percentile(u8),
}

/// Targeting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingConfig {
    /// User segments to include
    pub include_segments: Vec<String>,
    /// User segments to exclude
    pub exclude_segments: Vec<String>,
    /// Conditions for inclusion
    pub conditions: Vec<TargetingCondition>,
    /// Maximum participants
    pub max_participants: Option<u64>,
}

/// Targeting condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingCondition {
    /// Attribute to check
    pub attribute: String,
    /// Operator
    pub operator: TargetingOperator,
    /// Values to compare against
    pub values: Vec<String>,
}

/// Targeting operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetingOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    GreaterThan,
    LessThan,
    Between,
}

/// Traffic allocation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficAllocation {
    /// Total traffic percentage to include
    pub total_percentage: f64,
    /// Allocation strategy
    pub strategy: AllocationStrategy,
    /// Variant allocations
    pub variant_allocations: HashMap<String, f64>,
}

/// Allocation strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllocationStrategy {
    Random,
    RoundRobin,
    Sticky,
    Weighted,
    Stratified,
}

/// Statistical configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalConfig {
    /// Significance level (alpha)
    pub significance_level: f64,
    /// Statistical power (1 - beta)
    pub power: f64,
    /// Minimum sample size per variant
    pub min_sample_size: u64,
    /// Maximum sample size per variant
    pub max_sample_size: Option<u64>,
    /// Test method
    pub test_method: StatisticalTestMethod,
    /// Multiple comparison correction
    pub multiple_comparison_correction: Option<MultipleComparisonCorrection>,
    /// Bayesian prior strength (for Bayesian tests)
    pub prior_strength: Option<f64>,
}

/// Statistical test methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatisticalTestMethod {
    TTest,
    WelchTTest,
    MannWhitneyUTest,
    ChiSquare,
    FisherExact,
    BayesianAB,
    Sequential,
    GTest,
}

/// Multiple comparison corrections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MultipleComparisonCorrection {
    Bonferroni,
    Holm,
    BenjaminiHochberg,
    Tukey,
}

/// Observation data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    /// Observation ID
    pub id: String,
    /// Experiment ID
    pub experiment_id: String,
    /// Variant ID
    pub variant_id: String,
    /// User/session identifier
    pub user_id: String,
    /// Metric values
    pub metrics: HashMap<String, f64>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional attributes
    pub attributes: HashMap<String, String>,
}

/// Experiment results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResults {
    /// Experiment ID
    pub experiment_id: String,
    /// Analysis timestamp
    pub analyzed_at: DateTime<Utc>,
    /// Variant results
    pub variant_results: HashMap<String, VariantResult>,
    /// Statistical comparisons
    pub comparisons: Vec<StatisticalComparison>,
    /// Overall conclusion
    pub conclusion: Option<ExperimentConclusion>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Sample size
    pub total_sample_size: u64,
    /// Days running
    pub days_running: u64,
}

/// Results for a single variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantResult {
    /// Variant ID
    pub variant_id: String,
    /// Sample size
    pub sample_size: u64,
    /// Metric results
    pub metric_results: HashMap<String, MetricResult>,
    /// Confidence interval
    pub confidence_level: f64,
}

/// Result for a single metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricResult {
    /// Metric ID
    pub metric_id: String,
    /// Mean value
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Standard error
    pub std_error: f64,
    /// Confidence interval (lower, upper)
    pub confidence_interval: (f64, f64),
    /// Median
    pub median: f64,
    /// Min value
    pub min: f64,
    /// Max value
    pub max: f64,
    /// Percentiles (P5, P25, P50, P75, P95, P99)
    pub percentiles: (f64, f64, f64, f64, f64, f64),
}

/// Statistical comparison between variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalComparison {
    /// Control variant ID
    pub control_variant_id: String,
    /// Treatment variant ID
    pub treatment_variant_id: String,
    /// Metric ID
    pub metric_id: String,
    /// Effect size
    pub effect_size: f64,
    /// Effect size confidence interval
    pub effect_size_ci: (f64, f64),
    /// Relative effect (percentage)
    pub relative_effect: f64,
    /// P-value
    pub p_value: f64,
    /// Whether the result is statistically significant
    pub is_significant: bool,
    /// Confidence level
    pub confidence_level: f64,
    /// Statistical power achieved
    pub power_achieved: f64,
    /// Test statistic
    pub test_statistic: f64,
    /// Effect direction
    pub effect_direction: EffectDirection,
}

/// Direction of effect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectDirection {
    Positive,
    Negative,
    Neutral,
}

/// Experiment conclusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentConclusion {
    /// Winning variant ID
    pub winning_variant_id: Option<String>,
    /// Confidence in conclusion
    pub confidence: f64,
    /// Summary
    pub summary: String,
    /// Business impact
    pub business_impact: String,
    /// Recommended action
    pub recommended_action: RecommendedAction,
    /// Caveats
    pub caveats: Vec<String>,
}

/// Recommended actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendedAction {
    ImplementWinner,
    ContinueTesting,
    StopTest,
    IterateOnTreatments,
    NoClearWinner,
    NeedMoreData,
}

// ============================================================================
// A/B Testing Framework
// ============================================================================

/// Main A/B Testing framework
pub struct ABTestingFramework {
    /// Active experiments
    experiments: HashMap<String, Experiment>,
    /// Observations storage
    observations: HashMap<String, Vec<Observation>>,
    /// Results cache
    results_cache: HashMap<String, ExperimentResults>,
    /// Configuration
    config: ABTestingConfig,
    /// Statistics
    stats: FrameworkStats,
}

/// Framework configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTestingConfig {
    /// Maximum experiments to run concurrently
    pub max_concurrent_experiments: usize,
    /// Default significance level
    pub default_significance_level: f64,
    /// Default power
    pub default_power: f64,
    /// Auto-stop on significance
    pub auto_stop_on_significance: bool,
    /// Minimum observations per variant
    pub min_observations: u64,
    /// Maximum experiment duration (days)
    pub max_experiment_days: u64,
    /// Enable real-time analysis
    pub real_time_analysis: bool,
    /// Cache TTL for results (seconds)
    pub results_cache_ttl_seconds: u64,
}

impl Default for ABTestingConfig {
    fn default() -> Self {
        Self {
            max_concurrent_experiments: 100,
            default_significance_level: 0.05,
            default_power: 0.8,
            auto_stop_on_significance: false,
            min_observations: 100,
            max_experiment_days: 30,
            real_time_analysis: true,
            results_cache_ttl_seconds: 300,
        }
    }
}

/// Framework statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FrameworkStats {
    pub total_experiments: u64,
    pub completed_experiments: u64,
    pub total_observations: u64,
    pub average_duration_days: f64,
    pub success_rate: f64,
}

impl ABTestingFramework {
    /// Create a new A/B testing framework
    pub fn new(config: ABTestingConfig) -> Self {
        Self {
            experiments: HashMap::new(),
            observations: HashMap::new(),
            results_cache: HashMap::new(),
            config,
            stats: FrameworkStats::default(),
        }
    }
    
    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(ABTestingConfig::default())
    }
    
    /// Create a new experiment
    pub fn create_experiment(&mut self, experiment: Experiment) -> Result<String, ABTestingError> {
        // Validate experiment
        self.validate_experiment(&experiment)?;
        
        // Check concurrent limit
        let running_count = self.experiments.values()
            .filter(|e| e.status == ExperimentStatus::Running)
            .count();
        
        if running_count >= self.config.max_concurrent_experiments {
            return Err(ABTestingError::MaxConcurrentExperimentsReached);
        }
        
        let id = experiment.id.clone();
        self.experiments.insert(id.clone(), experiment);
        self.observations.insert(id.clone(), Vec::new());
        self.stats.total_experiments += 1;
        
        Ok(id)
    }
    
    /// Start an experiment
    pub fn start_experiment(&mut self, experiment_id: &str) -> Result<(), ABTestingError> {
        let experiment = self.experiments.get_mut(experiment_id)
            .ok_or(ABTestingError::ExperimentNotFound)?;
        
        if experiment.status != ExperimentStatus::Draft {
            return Err(ABTestingError::InvalidStatusTransition);
        }
        
        experiment.status = ExperimentStatus::Running;
        experiment.started_at = Some(Utc::now());
        
        Ok(())
    }
    
    /// Record an observation
    pub fn record_observation(&mut self, observation: Observation) -> Result<(), ABTestingError> {
        // Verify experiment exists and is running
        let experiment = self.experiments.get(&observation.experiment_id)
            .ok_or(ABTestingError::ExperimentNotFound)?;
        
        if experiment.status != ExperimentStatus::Running {
            return Err(ABTestingError::ExperimentNotRunning);
        }
        
        // Verify variant exists
        if !experiment.variants.iter().any(|v| v.id == observation.variant_id) {
            return Err(ABTestingError::InvalidVariant);
        }
        
        // Store observation
        self.observations
            .get_mut(&observation.experiment_id)
            .ok_or(ABTestingError::ExperimentNotFound)?
            .push(observation);
        
        self.stats.total_observations += 1;
        
        // Invalidate cached results
        self.results_cache.remove(&experiment.id);
        
        Ok(())
    }
    
    /// Analyze experiment results
    pub fn analyze_experiment(&mut self, experiment_id: &str) -> Result<ExperimentResults, ABTestingError> {
        let experiment = self.experiments.get(experiment_id)
            .ok_or(ABTestingError::ExperimentNotFound)?
            .clone();
        
        let observations = self.observations.get(experiment_id)
            .ok_or(ABTestingError::ExperimentNotFound)?;
        
        if observations.len() < self.config.min_observations as usize {
            return Err(ABTestingError::InsufficientData);
        }
        
        // Group observations by variant
        let mut variant_observations: HashMap<String, Vec<&Observation>> = HashMap::new();
        for obs in observations {
            variant_observations
                .entry(obs.variant_id.clone())
                .or_insert_with(Vec::new)
                .push(obs);
        }
        
        // Calculate variant results
        let mut variant_results = HashMap::new();
        for variant in &experiment.variants {
            let obs = variant_observations.get(&variant.id).cloned().unwrap_or_default();
            let result = self.calculate_variant_result(obs, &experiment.metrics);
            variant_results.insert(variant.id.clone(), result);
        }
        
        // Perform statistical comparisons
        let comparisons = self.perform_comparisons(&variant_results, &experiment);
        
        // Generate conclusion
        let conclusion = self.generate_conclusion(&variant_results, &comparisons, &experiment);
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(&conclusion, &comparisons);
        
        // Calculate days running
        let days_running = experiment.started_at
            .map(|start| (Utc::now() - start).num_days() as u64)
            .unwrap_or(0);
        
        let results = ExperimentResults {
            experiment_id: experiment_id.to_string(),
            analyzed_at: Utc::now(),
            variant_results,
            comparisons,
            conclusion,
            recommendations,
            total_sample_size: observations.len() as u64,
            days_running,
        };
        
        // Cache results
        self.results_cache.insert(experiment_id.to_string(), results.clone());
        
        Ok(results)
    }
    
    /// Stop an experiment
    pub fn stop_experiment(&mut self, experiment_id: &str, reason: Option<String>) -> Result<(), ABTestingError> {
        let experiment = self.experiments.get_mut(experiment_id)
            .ok_or(ABTestingError::ExperimentNotFound)?;
        
        if experiment.status != ExperimentStatus::Running {
            return Err(ABTestingError::InvalidStatusTransition);
        }
        
        experiment.status = ExperimentStatus::Completed;
        experiment.ended_at = Some(Utc::now());
        
        self.stats.completed_experiments += 1;
        
        Ok(())
    }
    
    /// Get experiment by ID
    pub fn get_experiment(&self, experiment_id: &str) -> Option<&Experiment> {
        self.experiments.get(experiment_id)
    }
    
    /// Get all experiments
    pub fn get_experiments(&self) -> Vec<&Experiment> {
        self.experiments.values().collect()
    }
    
    /// Get observations for an experiment
    pub fn get_observations(&self, experiment_id: &str) -> Option<&[Observation]> {
        self.observations.get(experiment_id).map(|v| v.as_slice())
    }
    
    /// Validate experiment configuration
    fn validate_experiment(&self, experiment: &Experiment) -> Result<(), ABTestingError> {
        // Must have at least 2 variants
        if experiment.variants.len() < 2 {
            return Err(ABTestingError::InvalidConfiguration("Must have at least 2 variants".to_string()));
        }
        
        // Must have exactly one control
        let control_count = experiment.variants.iter().filter(|v| v.is_control).count();
        if control_count != 1 {
            return Err(ABTestingError::InvalidConfiguration("Must have exactly one control variant".to_string()));
        }
        
        // Traffic allocation must sum to 100
        let total_traffic: f64 = experiment.traffic_allocation.variant_allocations.values().sum();
        if (total_traffic - 100.0).abs() > 0.01 {
            return Err(ABTestingError::InvalidConfiguration("Traffic allocation must sum to 100%".to_string()));
        }
        
        // Must have at least one primary metric
        if !experiment.metrics.iter().any(|m| m.is_primary) {
            return Err(ABTestingError::InvalidConfiguration("Must have at least one primary metric".to_string()));
        }
        
        Ok(())
    }
    
    /// Calculate results for a variant
    fn calculate_variant_result(
        &self,
        observations: Vec<&Observation>,
        metrics: &[MetricDefinition],
    ) -> VariantResult {
        let sample_size = observations.len() as u64;
        let mut metric_results = HashMap::new();
        
        for metric_def in metrics {
            let values: Vec<f64> = observations
                .iter()
                .filter_map(|obs| obs.metrics.get(&metric_def.id))
                .cloned()
                .collect();
            
            let result = self.calculate_metric_statistics(&values);
            metric_results.insert(metric_def.id.clone(), result);
        }
        
        VariantResult {
            variant_id: observations.first()
                .map(|o| o.variant_id.clone())
                .unwrap_or_default(),
            sample_size,
            metric_results,
            confidence_level: 0.95,
        }
    }
    
    /// Calculate statistics for a metric
    fn calculate_metric_statistics(&self, values: &[f64]) -> MetricResult {
        if values.is_empty() {
            return MetricResult {
                metric_id: String::new(),
                mean: 0.0,
                std_dev: 0.0,
                std_error: 0.0,
                confidence_interval: (0.0, 0.0),
                median: 0.0,
                min: 0.0,
                max: 0.0,
                percentiles: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            };
        }
        
        let n = values.len() as f64;
        let mean = values.iter().sum::<f64>() / n;
        
        let variance = if n > 1.0 {
            values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0)
        } else {
            0.0
        };
        let std_dev = variance.sqrt();
        let std_error = std_dev / n.sqrt();
        
        // 95% confidence interval
        let z = 1.96;
        let ci_lower = mean - z * std_error;
        let ci_upper = mean + z * std_error;
        
        // Calculate percentiles
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let percentile = |p: f64| -> f64 {
            let idx = ((n - 1.0) * p / 100.0).floor() as usize;
            sorted[idx.min(sorted.len() - 1)]
        };
        
        MetricResult {
            metric_id: String::new(),
            mean,
            std_dev,
            std_error,
            confidence_interval: (ci_lower, ci_upper),
            median: percentile(50.0),
            min: sorted.first().copied().unwrap_or(0.0),
            max: sorted.last().copied().unwrap_or(0.0),
            percentiles: (
                percentile(5.0),
                percentile(25.0),
                percentile(50.0),
                percentile(75.0),
                percentile(95.0),
                percentile(99.0),
            ),
        }
    }
    
    /// Perform statistical comparisons
    fn perform_comparisons(
        &self,
        variant_results: &HashMap<String, VariantResult>,
        experiment: &Experiment,
    ) -> Vec<StatisticalComparison> {
        let mut comparisons = Vec::new();
        
        // Find control variant
        let control_variant = experiment.variants.iter()
            .find(|v| v.is_control);
        
        if let Some(control) = control_variant {
            let control_result = variant_results.get(&control.id);
            
            for treatment in experiment.variants.iter().filter(|v| !v.is_control) {
                let treatment_result = variant_results.get(&treatment.id);
                
                if let (Some(control_res), Some(treatment_res)) = (control_result, treatment_result) {
                    for metric in &experiment.metrics {
                        let control_metric = control_res.metric_results.get(&metric.id);
                        let treatment_metric = treatment_res.metric_results.get(&metric.id);
                        
                        if let (Some(c_metric), Some(t_metric)) = (control_metric, treatment_metric) {
                            let comparison = self.compare_metrics(
                                &control.id,
                                &treatment.id,
                                &metric.id,
                                c_metric,
                                t_metric,
                                metric.higher_is_better,
                                experiment.statistical_config.significance_level,
                            );
                            comparisons.push(comparison);
                        }
                    }
                }
            }
        }
        
        comparisons
    }
    
    /// Compare two metric results
    fn compare_metrics(
        &self,
        control_id: &str,
        treatment_id: &str,
        metric_id: &str,
        control: &MetricResult,
        treatment: &MetricResult,
        higher_is_better: bool,
        alpha: f64,
    ) -> StatisticalComparison {
        // Calculate effect size (Cohen's d)
        let pooled_std = ((control.std_dev.powi(2) + treatment.std_dev.powi(2)) / 2.0).sqrt();
        let effect_size = if pooled_std > 0.0 {
            (treatment.mean - control.mean) / pooled_std
        } else {
            0.0
        };
        
        // Calculate relative effect
        let relative_effect = if control.mean.abs() > 1e-10 {
            (treatment.mean - control.mean) / control.mean.abs() * 100.0
        } else {
            0.0
        };
        
        // Calculate t-statistic
        let n1 = control.std_error.powi(2).recip() * control.std_dev.powi(2);
        let n2 = treatment.std_error.powi(2).recip() * treatment.std_dev.powi(2);
        
        let se_diff = (control.std_error.powi(2) + treatment.std_error.powi(2)).sqrt();
        let t_stat = if se_diff > 0.0 {
            (treatment.mean - control.mean) / se_diff
        } else {
            0.0
        };
        
        // Calculate p-value (simplified two-tailed t-test)
        let df = n1 + n2 - 2.0;
        let p_value = self.calculate_p_value(t_stat.abs(), df);
        
        // Determine significance
        let is_significant = p_value < alpha;
        
        // Calculate effect direction
        let effect_direction = if is_significant {
            if (higher_is_better && treatment.mean > control.mean) ||
               (!higher_is_better && treatment.mean < control.mean) {
                EffectDirection::Positive
            } else {
                EffectDirection::Negative
            }
        } else {
            EffectDirection::Neutral
        };
        
        // Effect size confidence interval
        let z = 1.96;
        let effect_se = (2.0 / (n1 + n2)).sqrt();
        let effect_ci = (effect_size - z * effect_se, effect_size + z * effect_se);
        
        StatisticalComparison {
            control_variant_id: control_id.to_string(),
            treatment_variant_id: treatment_id.to_string(),
            metric_id: metric_id.to_string(),
            effect_size,
            effect_size_ci: effect_ci,
            relative_effect,
            p_value,
            is_significant,
            confidence_level: 1.0 - alpha,
            power_achieved: 0.8, // Simplified
            test_statistic: t_stat,
            effect_direction,
        }
    }
    
    /// Calculate p-value from t-statistic (simplified approximation)
    fn calculate_p_value(&self, t: f64, df: f64) -> f64 {
        // Simplified approximation using normal distribution for large samples
        if df > 30.0 {
            2.0 * (1.0 - self.normal_cdf(t))
        } else {
            // Use approximation for smaller samples
            let x = df / (df + t.powi(2));
            2.0 * (1.0 - self.incomplete_beta(0.5 * df, 0.5, x))
        }
    }
    
    /// Standard normal CDF approximation
    fn normal_cdf(&self, x: f64) -> f64 {
        // Approximation using error function
        0.5 * (1.0 + self.erf(x / std::f64::consts::SQRT_2))
    }
    
    /// Error function approximation
    fn erf(&self, x: f64) -> f64 {
        // Approximation from Abramowitz and Stegun
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let p = 0.3275911;
        
        let sign = if x >= 0.0 { 1.0 } else { -1.0 };
        let x = x.abs();
        
        let t = 1.0 / (1.0 + p * x);
        let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
        
        sign * y
    }
    
    /// Incomplete beta function approximation (simplified)
    fn incomplete_beta(&self, _a: f64, _b: f64, _x: f64) -> f64 {
        // Simplified placeholder - would use proper numerical integration
        0.5
    }
    
    /// Generate conclusion from results
    fn generate_conclusion(
        &self,
        variant_results: &HashMap<String, VariantResult>,
        comparisons: &[StatisticalComparison],
        experiment: &Experiment,
    ) -> Option<ExperimentConclusion> {
        // Find primary metric
        let primary_metric = experiment.metrics.iter()
            .find(|m| m.is_primary)?;
        
        // Find significant positive results
        let significant_positive: Vec<_> = comparisons.iter()
            .filter(|c| c.metric_id == primary_metric.id && 
                        c.is_significant && 
                        c.effect_direction == EffectDirection::Positive)
            .collect();
        
        // Find winning variant
        let winning_variant_id = if !significant_positive.is_empty() {
            // Find the treatment with the largest effect
            significant_positive.iter()
                .max_by(|a, b| a.relative_effect.partial_cmp(&b.relative_effect).unwrap_or(std::cmp::Ordering::Equal))
                .map(|c| c.treatment_variant_id.clone())
        } else {
            None
        };
        
        // Calculate confidence
        let confidence = if let Some(ref winner_id) = winning_variant_id {
            comparisons.iter()
                .filter(|c| c.treatment_variant_id == *winner_id && c.is_significant)
                .map(|c| c.confidence_level)
                .max()
                .unwrap_or(0.0)
        } else {
            0.0
        };
        
        // Generate summary
        let summary = if let Some(ref winner_id) = winning_variant_id {
            format!("Variant '{}' is the winner with {:.1}% confidence", 
                winner_id, confidence * 100.0)
        } else {
            "No statistically significant winner found".to_string()
        };
        
        // Determine recommended action
        let recommended_action = if winning_variant_id.is_some() && confidence > 0.95 {
            RecommendedAction::ImplementWinner
        } else if confidence > 0.8 {
            RecommendedAction::ContinueTesting
        } else if comparisons.iter().any(|c| c.is_significant) {
            RecommendedAction::IterateOnTreatments
        } else {
            RecommendedAction::NeedMoreData
        };
        
        Some(ExperimentConclusion {
            winning_variant_id,
            confidence,
            summary,
            business_impact: "Estimated impact will be calculated based on metric improvements".to_string(),
            recommended_action,
            caveats: vec!["Results may vary in production environment".to_string()],
        })
    }
    
    /// Generate recommendations
    fn generate_recommendations(
        &self,
        conclusion: &Option<ExperimentConclusion>,
        comparisons: &[StatisticalComparison],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if let Some(conc) = conclusion {
            match conc.recommended_action {
                RecommendedAction::ImplementWinner => {
                    recommendations.push(format!("Implement winning variant '{}' across all traffic", 
                        conc.winning_variant_id.as_ref().unwrap_or(&"unknown".to_string())));
                }
                RecommendedAction::ContinueTesting => {
                    recommendations.push("Continue the experiment to reach higher confidence".to_string());
                    recommendations.push("Consider increasing sample size".to_string());
                }
                RecommendedAction::IterateOnTreatments => {
                    recommendations.push("Consider iterating on treatment variants based on insights".to_string());
                }
                RecommendedAction::NeedMoreData => {
                    recommendations.push("Gather more data before making a decision".to_string());
                    recommendations.push("Check for data quality issues".to_string());
                }
                _ => {}
            }
        }
        
        // Add warnings for unexpected results
        let negative_significant: Vec<_> = comparisons.iter()
            .filter(|c| c.is_significant && c.effect_direction == EffectDirection::Negative)
            .collect();
        
        if !negative_significant.is_empty() {
            recommendations.push(format!("Warning: {} metric(s) showed significant negative impact", 
                negative_significant.len()));
        }
        
        recommendations
    }
    
    /// Get framework statistics
    pub fn get_stats(&self) -> &FrameworkStats {
        &self.stats
    }
}

/// A/B Testing errors
#[derive(Debug, Clone)]
pub enum ABTestingError {
    ExperimentNotFound,
    ExperimentNotRunning,
    InvalidVariant,
    InvalidStatusTransition,
    InvalidConfiguration(String),
    InsufficientData,
    MaxConcurrentExperimentsReached,
    AnalysisError(String),
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_framework_creation() {
        let framework = ABTestingFramework::with_defaults();
        assert_eq!(framework.experiments.len(), 0);
    }
    
    #[test]
    fn test_create_experiment() {
        let mut framework = ABTestingFramework::with_defaults();
        
        let experiment = Experiment {
            id: "exp1".to_string(),
            name: "Test Experiment".to_string(),
            description: "A test experiment".to_string(),
            status: ExperimentStatus::Draft,
            variants: vec![
                Variant {
                    id: "control".to_string(),
                    name: "Control".to_string(),
                    description: "Control variant".to_string(),
                    variant_type: VariantType::Control,
                    config: HashMap::new(),
                    traffic_percentage: 50.0,
                    is_control: true,
                },
                Variant {
                    id: "treatment".to_string(),
                    name: "Treatment".to_string(),
                    description: "Treatment variant".to_string(),
                    variant_type: VariantType::Treatment,
                    config: HashMap::new(),
                    traffic_percentage: 50.0,
                    is_control: false,
                },
            ],
            metrics: vec![
                MetricDefinition {
                    id: "conversion".to_string(),
                    name: "Conversion Rate".to_string(),
                    description: "Primary conversion rate".to_string(),
                    metric_type: MetricType::Percentage,
                    aggregation: AggregationMethod::Mean,
                    higher_is_better: true,
                    minimum_detectable_effect: 5.0,
                    weight: 1.0,
                    is_primary: true,
                },
            ],
            targeting: TargetingConfig {
                include_segments: vec!["all".to_string()],
                exclude_segments: vec![],
                conditions: vec![],
                max_participants: None,
            },
            traffic_allocation: TrafficAllocation {
                total_percentage: 100.0,
                strategy: AllocationStrategy::Random,
                variant_allocations: vec![
                    ("control".to_string(), 50.0),
                    ("treatment".to_string(), 50.0),
                ].into_iter().collect(),
            },
            statistical_config: StatisticalConfig {
                significance_level: 0.05,
                power: 0.8,
                min_sample_size: 100,
                max_sample_size: None,
                test_method: StatisticalTestMethod::TTest,
                multiple_comparison_correction: None,
                prior_strength: None,
            },
            started_at: None,
            ended_at: None,
            created_at: Utc::now(),
            created_by: "test".to_string(),
            tags: vec![],
        };
        
        let result = framework.create_experiment(experiment);
        assert!(result.is_ok());
        assert_eq!(framework.experiments.len(), 1);
    }
    
    #[test]
    fn test_statistical_comparison() {
        let framework = ABTestingFramework::with_defaults();
        
        let control = MetricResult {
            metric_id: "conversion".to_string(),
            mean: 10.0,
            std_dev: 2.0,
            std_error: 0.2,
            confidence_interval: (9.6, 10.4),
            median: 10.0,
            min: 5.0,
            max: 15.0,
            percentiles: (7.0, 8.5, 10.0, 11.5, 13.0, 14.0),
        };
        
        let treatment = MetricResult {
            metric_id: "conversion".to_string(),
            mean: 12.0,
            std_dev: 2.0,
            std_error: 0.2,
            confidence_interval: (11.6, 12.4),
            median: 12.0,
            min: 6.0,
            max: 17.0,
            percentiles: (8.0, 10.0, 12.0, 14.0, 15.5, 16.5),
        };
        
        let comparison = framework.compare_metrics(
            "control",
            "treatment",
            "conversion",
            &control,
            &treatment,
            true,
            0.05,
        );
        
        assert!(comparison.relative_effect > 0.0);
        assert_eq!(comparison.effect_direction, EffectDirection::Positive);
    }
    
    #[test]
    fn test_experiment_status() {
        assert_ne!(ExperimentStatus::Draft, ExperimentStatus::Running);
        assert_ne!(ExperimentStatus::Running, ExperimentStatus::Completed);
    }
    
    #[test]
    fn test_variant_type() {
        assert_ne!(VariantType::Control, VariantType::Treatment);
    }
}