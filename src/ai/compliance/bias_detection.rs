//! AI Bias Detection and Mitigation Module
//!
//! This module provides comprehensive bias detection and mitigation capabilities
//! for AI systems to ensure fair and equitable outcomes.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Types of bias
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiasType {
    /// Sampling bias in training data
    SamplingBias,
    /// Measurement bias in feature collection
    MeasurementBias,
    /// Label bias in annotations
    LabelBias,
    /// Algorithmic bias in model training
    AlgorithmicBias,
    /// Aggregation bias in model outputs
    AggregationBias,
    /// Evaluation bias in performance metrics
    EvaluationBias,
    /// Historical bias in data
    HistoricalBias,
    /// Representation bias
    RepresentationBias,
}

/// Protected attributes for fairness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtectedAttribute {
    Gender,
    Race,
    Age,
    Disability,
    Religion,
    NationalOrigin,
    SexualOrientation,
    SocioeconomicStatus,
}

/// Fairness metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FairnessMetric {
    /// Demographic parity
    DemographicParity,
    /// Equalized odds
    EqualizedOdds,
    /// Equal opportunity
    EqualOpportunity,
    /// Predictive parity
    PredictiveParity,
    /// Calibration
    Calibration,
    /// Individual fairness
    IndividualFairness,
}

/// Bias detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasDetectionResult {
    /// Detection ID
    pub id: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Bias type detected
    pub bias_type: BiasType,
    /// Protected attribute affected
    pub protected_attribute: ProtectedAttribute,
    /// Severity (0.0-1.0)
    pub severity: f64,
    /// Metric used for detection
    pub metric: FairnessMetric,
    /// Metric value
    pub metric_value: f64,
    /// Threshold for detection
    pub threshold: f64,
    /// Whether bias was detected
    pub is_biased: bool,
    /// Detailed findings
    pub findings: Vec<String>,
    /// Affected groups
    pub affected_groups: HashMap<String, GroupStatistics>,
    /// Recommended mitigations
    pub mitigations: Vec<MitigationRecommendation>,
}

/// Group statistics for bias analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupStatistics {
    /// Group name
    pub name: String,
    /// Sample size
    pub sample_size: usize,
    /// Positive rate
    pub positive_rate: f64,
    /// False positive rate
    pub false_positive_rate: f64,
    /// False negative rate
    pub false_negative_rate: f64,
    /// Prediction distribution
    pub prediction_mean: f64,
    /// Confidence interval
    pub confidence_interval: (f64, f64),
}

/// Mitigation recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationRecommendation {
    /// Strategy type
    pub strategy: MitigationStrategy,
    /// Priority (1-5)
    pub priority: u8,
    /// Description
    pub description: String,
    /// Expected impact
    pub expected_impact: f64,
    /// Implementation complexity
    pub complexity: Complexity,
}

/// Mitigation strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MitigationStrategy {
    /// Resample training data
    Resampling,
    /// Reweight samples
    Reweighting,
    /// Feature removal
    FeatureRemoval,
    /// Adversarial debiasing
    AdversarialDebiasing,
    /// Threshold adjustment
    ThresholdAdjustment,
    /// Post-processing calibration
    PostProcessingCalibration,
    /// Data augmentation
    DataAugmentation,
    /// Fairness constraints
    FairnessConstraints,
}

/// Implementation complexity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Complexity {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Bias detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasDetectionConfig {
    /// Enable proactive bias detection
    pub enabled: bool,
    /// Fairness metrics to monitor
    pub metrics: Vec<FairnessMetric>,
    /// Protected attributes to check
    pub protected_attributes: Vec<ProtectedAttribute>,
    /// Threshold for demographic parity
    pub demographic_parity_threshold: f64,
    /// Threshold for equalized odds
    pub equalized_odds_threshold: f64,
    /// Minimum sample size per group
    pub min_sample_size: usize,
    /// Enable automatic mitigation suggestions
    pub auto_mitigation_suggestions: bool,
    /// Continuous monitoring interval (seconds)
    pub monitoring_interval_secs: u64,
}

impl Default for BiasDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics: vec![
                FairnessMetric::DemographicParity,
                FairnessMetric::EqualizedOdds,
                FairnessMetric::EqualOpportunity,
            ],
            protected_attributes: vec![
                ProtectedAttribute::Gender,
                ProtectedAttribute::Race,
                ProtectedAttribute::Age,
            ],
            demographic_parity_threshold: 0.1,
            equalized_odds_threshold: 0.1,
            min_sample_size: 100,
            auto_mitigation_suggestions: true,
            monitoring_interval_secs: 3600,
        }
    }
}

/// Bias detection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasStats {
    /// Total checks performed
    pub total_checks: usize,
    /// Biases detected
    pub biases_detected: usize,
    /// By bias type
    pub by_type: HashMap<BiasType, usize>,
    /// By protected attribute
    pub by_attribute: HashMap<ProtectedAttribute, usize>,
    /// Average severity
    pub avg_severity: f64,
    /// Mitigations applied
    pub mitigations_applied: usize,
}

/// Bias Detector
pub struct BiasDetector {
    config: BiasDetectionConfig,
    results: Arc<RwLock<Vec<BiasDetectionResult>>>,
    statistics: Arc<RwLock<BiasStats>>,
    baseline_metrics: Arc<RwLock<HashMap<FairnessMetric, f64>>>,
}

impl BiasDetector {
    /// Create a new bias detector
    pub fn new() -> Result<Self> {
        info!("Initializing Bias Detector");

        Ok(Self {
            config: BiasDetectionConfig::default(),
            results: Arc::new(RwLock::new(Vec::new())),
            statistics: Arc::new(RwLock::new(BiasStats {
                total_checks: 0,
                biases_detected: 0,
                by_type: HashMap::new(),
                by_attribute: HashMap::new(),
                avg_severity: 0.0,
                mitigations_applied: 0,
            })),
            baseline_metrics: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Check for demographic parity bias
    pub async fn check_demographic_parity(
        &self,
        predictions: &HashMap<String, Vec<f64>>,
        attribute: ProtectedAttribute,
    ) -> Result<BiasDetectionResult> {
        let mut group_stats = HashMap::new();
        let mut positive_rates = Vec::new();

        for (group, preds) in predictions {
            if preds.len() < self.config.min_sample_size {
                continue;
            }

            let positive_count = preds.iter().filter(|p| **p > 0.5).count();
            let positive_rate = positive_count as f64 / preds.len() as f64;
            positive_rates.push(positive_rate);

            group_stats.insert(
                group.clone(),
                GroupStatistics {
                    name: group.clone(),
                    sample_size: preds.len(),
                    positive_rate,
                    false_positive_rate: 0.0, // Would need ground truth
                    false_negative_rate: 0.0,
                    prediction_mean: preds.iter().sum::<f64>() / preds.len() as f64,
                    confidence_interval: (positive_rate - 0.05, positive_rate + 0.05),
                },
            );
        }

        // Calculate demographic parity difference
        let max_rate = positive_rates.iter().cloned().fold(0.0, f64::max);
        let min_rate = positive_rates.iter().cloned().fold(f64::INFINITY, f64::min);
        let dp_difference = max_rate - min_rate;

        let is_biased = dp_difference > self.config.demographic_parity_threshold;
        let severity = dp_difference.min(1.0);

        let mut findings = Vec::new();
        if is_biased {
            findings.push(format!(
                "Demographic parity difference ({:.3}) exceeds threshold ({:.3})",
                dp_difference, self.config.demographic_parity_threshold
            ));
            findings.push(format!(
                "Group positive rates range from {:.3} to {:.3}",
                min_rate, max_rate
            ));
        }

        let mitigations = if is_biased && self.config.auto_mitigation_suggestions {
            self.generate_mitigations(BiasType::AlgorithmicBias, severity)?
        } else {
            Vec::new()
        };

        let result = BiasDetectionResult {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            bias_type: BiasType::AlgorithmicBias,
            protected_attribute: attribute,
            severity,
            metric: FairnessMetric::DemographicParity,
            metric_value: dp_difference,
            threshold: self.config.demographic_parity_threshold,
            is_biased,
            findings,
            affected_groups: group_stats,
            mitigations,
        };

        self.record_result(&result).await;

        Ok(result)
    }

    /// Check for equalized odds bias
    pub async fn check_equalized_odds(
        &self,
        predictions: &HashMap<String, Vec<(f64, bool)>>,
        attribute: ProtectedAttribute,
    ) -> Result<BiasDetectionResult> {
        let mut group_stats = HashMap::new();
        let mut fpr_diffs = Vec::new();
        let mut fnr_diffs = Vec::new();

        for (group, preds_labels) in predictions {
            if preds_labels.len() < self.config.min_sample_size {
                continue;
            }

            let (true_positives, false_positives, true_negatives, false_negatives) =
                self.calculate_confusion_matrix(preds_labels);

            let fpr = if false_positives + true_negatives > 0 {
                false_positives as f64 / (false_positives + true_negatives) as f64
            } else {
                0.0
            };

            let fnr = if false_negatives + true_positives > 0 {
                false_negatives as f64 / (false_negatives + true_positives) as f64
            } else {
                0.0
            };

            fpr_diffs.push(fpr);
            fnr_diffs.push(fnr);

            let positive_rate = (true_positives + false_positives) as f64 / preds_labels.len() as f64;

            group_stats.insert(
                group.clone(),
                GroupStatistics {
                    name: group.clone(),
                    sample_size: preds_labels.len(),
                    positive_rate,
                    false_positive_rate: fpr,
                    false_negative_rate: fnr,
                    prediction_mean: preds_labels.iter().map(|(p, _)| *p).sum::<f64>()
                        / preds_labels.len() as f64,
                    confidence_interval: (fpr - 0.05, fpr + 0.05),
                },
            );
        }

        // Calculate equalized odds difference
        let fpr_range = fpr_diffs.iter().cloned().fold(0.0, f64::max)
            - fpr_diffs.iter().cloned().fold(f64::INFINITY, f64::min);
        let fnr_range = fnr_diffs.iter().cloned().fold(0.0, f64::max)
            - fnr_diffs.iter().cloned().fold(f64::INFINITY, f64::min);
        let eo_difference = fpr_range.max(fnr_range);

        let is_biased = eo_difference > self.config.equalized_odds_threshold;
        let severity = eo_difference.min(1.0);

        let mut findings = Vec::new();
        if is_biased {
            findings.push(format!(
                "Equalized odds difference ({:.3}) exceeds threshold ({:.3})",
                eo_difference, self.config.equalized_odds_threshold
            ));
            findings.push(format!("False positive rate range: {:.3}", fpr_range));
            findings.push(format!("False negative rate range: {:.3}", fnr_range));
        }

        let mitigations = if is_biased && self.config.auto_mitigation_suggestions {
            self.generate_mitigations(BiasType::AlgorithmicBias, severity)?
        } else {
            Vec::new()
        };

        let result = BiasDetectionResult {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            bias_type: BiasType::AlgorithmicBias,
            protected_attribute: attribute,
            severity,
            metric: FairnessMetric::EqualizedOdds,
            metric_value: eo_difference,
            threshold: self.config.equalized_odds_threshold,
            is_biased,
            findings,
            affected_groups: group_stats,
            mitigations,
        };

        self.record_result(&result).await;

        Ok(result)
    }

    /// Calculate confusion matrix
    fn calculate_confusion_matrix(&self, preds_labels: &[(f64, bool)]) -> (usize, usize, usize, usize) {
        let mut tp = 0;
        let mut fp = 0;
        let mut tn = 0;
        let mut fn_ = 0;

        for (pred, label) in preds_labels {
            let predicted_positive = *pred > 0.5;
            let actual_positive = *label;

            match (predicted_positive, actual_positive) {
                (true, true) => tp += 1,
                (true, false) => fp += 1,
                (false, true) => fn_ += 1,
                (false, false) => tn += 1,
            }
        }

        (tp, fp, tn, fn_)
    }

    /// Generate mitigation recommendations
    fn generate_mitigations(
        &self,
        bias_type: BiasType,
        severity: f64,
    ) -> Result<Vec<MitigationRecommendation>> {
        let mut mitigations = Vec::new();

        // Priority based on severity
        let priority = if severity > 0.7 {
            1
        } else if severity > 0.5 {
            2
        } else if severity > 0.3 {
            3
        } else {
            4
        };

        match bias_type {
            BiasType::SamplingBias | BiasType::RepresentationBias => {
                mitigations.push(MitigationRecommendation {
                    strategy: MitigationStrategy::Resampling,
                    priority,
                    description: "Apply stratified sampling to balance group representation".to_string(),
                    expected_impact: 0.7,
                    complexity: Complexity::Low,
                });
                mitigations.push(MitigationRecommendation {
                    strategy: MitigationStrategy::DataAugmentation,
                    priority,
                    description: "Generate synthetic samples for underrepresented groups".to_string(),
                    expected_impact: 0.6,
                    complexity: Complexity::Medium,
                });
            }
            BiasType::AlgorithmicBias => {
                mitigations.push(MitigationRecommendation {
                    strategy: MitigationStrategy::Reweighting,
                    priority,
                    description: "Apply sample weights to balance group influence".to_string(),
                    expected_impact: 0.8,
                    complexity: Complexity::Low,
                });
                mitigations.push(MitigationRecommendation {
                    strategy: MitigationStrategy::AdversarialDebiasing,
                    priority: (priority + 1).min(5),
                    description: "Use adversarial training to remove protected attribute information".to_string(),
                    expected_impact: 0.9,
                    complexity: Complexity::High,
                });
                mitigations.push(MitigationRecommendation {
                    strategy: MitigationStrategy::FairnessConstraints,
                    priority,
                    description: "Add fairness constraints to the optimization objective".to_string(),
                    expected_impact: 0.85,
                    complexity: Complexity::Medium,
                });
            }
            _ => {
                mitigations.push(MitigationRecommendation {
                    strategy: MitigationStrategy::PostProcessingCalibration,
                    priority,
                    description: "Apply post-processing calibration to equalize outcomes".to_string(),
                    expected_impact: 0.75,
                    complexity: Complexity::Low,
                });
                mitigations.push(MitigationRecommendation {
                    strategy: MitigationStrategy::ThresholdAdjustment,
                    priority,
                    description: "Adjust decision thresholds per group for fairness".to_string(),
                    expected_impact: 0.7,
                    complexity: Complexity::Low,
                });
            }
        }

        Ok(mitigations)
    }

    /// Record detection result
    async fn record_result(&self, result: &BiasDetectionResult) {
        let mut results = self.results.write().await;
        results.push(result.clone());

        let mut stats = self.statistics.write().await;
        stats.total_checks += 1;
        if result.is_biased {
            stats.biases_detected += 1;
            *stats.by_type.entry(result.bias_type).or_insert(0) += 1;
            *stats.by_attribute.entry(result.protected_attribute).or_insert(0) += 1;
        }
        stats.avg_severity = (stats.avg_severity * (stats.total_checks - 1) as f64 + result.severity)
            / stats.total_checks as f64;
    }

    /// Set baseline fairness metric
    pub async fn set_baseline(&self, metric: FairnessMetric, value: f64) {
        let mut baseline = self.baseline_metrics.write().await;
        baseline.insert(metric, value);
    }

    /// Get recent bias detection results
    pub async fn get_recent_results(&self, limit: usize) -> Vec<BiasDetectionResult> {
        let results = self.results.read().await;
        results.iter().rev().take(limit).cloned().collect()
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> BiasStats {
        self.statistics.read().await.clone()
    }

    /// Check all protected attributes
    pub async fn comprehensive_check(
        &self,
        predictions_by_attribute: HashMap<ProtectedAttribute, HashMap<String, Vec<f64>>>,
    ) -> Result<Vec<BiasDetectionResult>> {
        let mut results = Vec::new();

        for (attribute, predictions) in predictions_by_attribute {
            let result = self.check_demographic_parity(&predictions, attribute).await?;
            results.push(result);
        }

        info!("Completed comprehensive bias check: {} results", results.len());
        Ok(results)
    }

    /// Export bias report
    pub async fn export_report(&self) -> Result<String> {
        let results = self.results.read().await;
        let stats = self.statistics.read().await;

        let report = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "statistics": {
                "total_checks": stats.total_checks,
                "biases_detected": stats.biases_detected,
                "average_severity": stats.avg_severity,
            },
            "results": results.iter().map(|r| serde_json::json!({
                "id": r.id,
                "bias_type": format!("{:?}", r.bias_type),
                "protected_attribute": format!("{:?}", r.protected_attribute),
                "severity": r.severity,
                "is_biased": r.is_biased,
                "findings": r.findings,
            })).collect::<Vec<_>>(),
        });

        Ok(serde_json::to_string_pretty(&report)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bias_detector_creation() {
        let detector = BiasDetector::new();
        assert!(detector.is_ok());
    }

    #[tokio::test]
    async fn test_demographic_parity_check() {
        let detector = BiasDetector::new().unwrap();

        let mut predictions = HashMap::new();
        predictions.insert("group_a".to_string(), vec![0.9, 0.8, 0.7, 0.6]);
        predictions.insert("group_b".to_string(), vec![0.4, 0.3, 0.2, 0.1]);

        let result = detector
            .check_demographic_parity(&predictions, ProtectedAttribute::Gender)
            .await
            .unwrap();

        assert!(result.is_biased);
    }

    #[tokio::test]
    async fn test_statistics() {
        let detector = BiasDetector::new().unwrap();

        let mut predictions = HashMap::new();
        predictions.insert("group_a".to_string(), vec![0.7; 100]);
        predictions.insert("group_b".to_string(), vec![0.7; 100]);

        detector
            .check_demographic_parity(&predictions, ProtectedAttribute::Gender)
            .await
            .unwrap();

        let stats = detector.get_statistics().await;
        assert_eq!(stats.total_checks, 1);
    }
}