//! Root Cause Analysis for VantisOS
//!
//! This module provides root cause analysis capabilities:
//! - Causal graph analysis
//! - Feature importance ranking
//! - Correlation analysis
//! - Traceback and timeline analysis
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::root_cause::{RootCauseAnalyzer, AnalysisMethod};
//!
//! let mut analyzer = RootCauseAnalyzer::new(AnalysisMethod::CausalGraph);
//! let result = analyzer.analyze(&anomaly_data, &system_metrics);
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::{HashMap, HashSet};

/// Analysis method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisMethod {
    /// Causal graph analysis
    CausalGraph,
    /// Feature importance
    FeatureImportance,
    /// Correlation analysis
    Correlation,
    /// PCA-based analysis
    PCA,
    /// Ensemble of methods
    Ensemble,
}

/// Root cause candidate
#[derive(Debug, Clone)]
pub struct RootCauseCandidate {
    pub feature: String,
    pub score: f64,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub related_features: Vec<String>,
}

/// Root cause analysis result
#[derive(Debug, Clone)]
pub struct RootCauseResult {
    pub timestamp: u64,
    pub anomaly_value: f64,
    pub method: AnalysisMethod,
    pub candidates: Vec<RootCauseCandidate>,
    pub primary_cause: Option<String>,
    pub confidence: f64,
    pub timeline: Vec<TimelineEvent>,
}

/// Timeline event
#[derive(Debug, Clone)]
pub struct TimelineEvent {
    pub timestamp: u64,
    pub event_type: String,
    pub description: String,
    pub feature: Option<String>,
    pub value: Option<f64>,
}

/// Causal graph node
#[derive(Debug, Clone)]
struct CausalGraphNode {
    feature: String,
    parents: Vec<String>,
    children: Vec<String>,
    strength: f64,
}

/// Root cause analyzer
pub struct RootCauseAnalyzer {
    pub method: AnalysisMethod,
    pub feature_importance: HashMap<String, f64>,
    pub correlations: HashMap<(String, String), f64>,
    pub causal_graph: HashMap<String, CausalGraphNode>,
    pub history: Vec<RootCauseResult>,
}

impl RootCauseAnalyzer {
    pub fn new(method: AnalysisMethod) -> Self {
        Self {
            method,
            feature_importance: HashMap::new(),
            correlations: HashMap::new(),
            causal_graph: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Analyze root cause
    pub fn analyze(
        &mut self,
        anomaly_features: &HashMap<String, f64>,
        baseline_features: &HashMap<String, f64>,
    ) -> Result<RootCauseResult> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let candidates = match self.method {
            AnalysisMethod::CausalGraph => self.causal_graph_analysis(anomaly_features, baseline_features),
            AnalysisMethod::FeatureImportance => self.feature_importance_analysis(anomaly_features, baseline_features),
            AnalysisMethod::Correlation => self.correlation_analysis(anomaly_features, baseline_features),
            AnalysisMethod::PCA => self.pca_analysis(anomaly_features, baseline_features),
            AnalysisMethod::Ensemble => self.ensemble_analysis(anomaly_features, baseline_features),
        };

        let primary_cause = candidates.first().map(|c| c.feature.clone());
        let confidence = candidates.first()
            .map(|c| c.confidence)
            .unwrap_or(0.0);

        let anomaly_value = anomaly_features.values()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        let result = RootCauseResult {
            timestamp,
            anomaly_value,
            method: self.method,
            candidates,
            primary_cause,
            confidence,
            timeline: self.build_timeline(anomaly_features, baseline_features),
        };

        self.history.push(result.clone());
        Ok(result)
    }

    /// Causal graph analysis
    fn causal_graph_analysis(
        &mut self,
        anomaly_features: &HashMap<String, f64>,
        baseline_features: &HashMap<String, f64>,
    ) -> Vec<RootCauseCandidate> {
        // Build or update causal graph
        self.build_causal_graph(anomaly_features, baseline_features);

        let mut candidates = Vec::new();

        for (feature, value) in anomaly_features {
            if let Some(baseline) = baseline_features.get(feature) {
                let deviation = (value - baseline).abs() / (baseline.abs() + 1e-10);
                
                if deviation > 0.5 {
                    let node = self.causal_graph.get(feature);
                    let related_features = node.map(|n| n.parents.clone())
                        .unwrap_or_default();

                    candidates.push(RootCauseCandidate {
                        feature: feature.clone(),
                        score: deviation,
                        confidence: deviation.min(1.0),
                        evidence: vec![
                            format!("High deviation: {:.2}%", deviation * 100.0),
                            format!("Value: {:.2}, Baseline: {:.2}", value, baseline),
                        ],
                        related_features,
                    });
                }
            }
        }

        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        candidates.truncate(10);
        candidates
    }

    /// Feature importance analysis
    fn feature_importance_analysis(
        &self,
        anomaly_features: &HashMap<String, f64>,
        baseline_features: &HashMap<String, f64>,
    ) -> Vec<RootCauseCandidate> {
        let mut candidates = Vec::new();

        for (feature, value) in anomaly_features {
            if let Some(baseline) = baseline_features.get(feature) {
                let deviation = (value - baseline).abs() / (baseline.abs() + 1e-10);
                let importance = self.feature_importance.get(feature)
                    .copied()
                    .unwrap_or(0.5);

                let score = deviation * importance;

                if score > 0.1 {
                    candidates.push(RootCauseCandidate {
                        feature: feature.clone(),
                        score,
                        confidence: score.min(1.0),
                        evidence: vec![
                            format!("Deviation: {:.2}%", deviation * 100.0),
                            format!("Importance: {:.2}", importance),
                        ],
                        related_features: Vec::new(),
                    });
                }
            }
        }

        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        candidates
    }

    /// Correlation analysis
    fn correlation_analysis(
        &self,
        anomaly_features: &HashMap<String, f64>,
        baseline_features: &HashMap<String, f64>,
    ) -> Vec<RootCauseCandidate> {
        let mut candidates = Vec::new();

        for (feature, value) in anomaly_features {
            if let Some(baseline) = baseline_features.get(feature) {
                let deviation = (value - baseline).abs() / (baseline.abs() + 1e-10);
                
                // Find highly correlated features
                let mut related_features = Vec::new();
                for ((f1, f2), corr) in &self.correlations {
                    if (f1 == feature || f2 == feature) && corr.abs() > 0.7 {
                        let other_feature = if f1 == feature { f2 } else { f1 };
                        related_features.push(other_feature.clone());
                    }
                }

                let correlation_factor = if related_features.is_empty() {
                    1.0
                } else {
                    1.0 + (related_features.len() as f64) * 0.1
                };

                let score = deviation * correlation_factor;

                if score > 0.1 {
                    candidates.push(RootCauseCandidate {
                        feature: feature.clone(),
                        score,
                        confidence: score.min(1.0),
                        evidence: vec![
                            format!("Deviation: {:.2}%", deviation * 100.0),
                            format!("Correlated features: {}", related_features.len()),
                        ],
                        related_features,
                    });
                }
            }
        }

        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        candidates
    }

    /// PCA-based analysis
    fn pca_analysis(
        &self,
        anomaly_features: &HashMap<String, f64>,
        baseline_features: &HashMap<String, f64>,
    ) -> Vec<RootCauseCandidate> {
        let mut candidates = Vec::new();
        let features: Vec<String> = anomaly_features.keys().cloned().collect();

        if features.len() < 2 {
            return candidates;
        }

        // Simple PCA: find features with highest variance
        let values: Vec<f64> = features.iter()
            .map(|f| anomaly_features.get(f).unwrap() - baseline_features.get(f).unwrap_or(&0.0))
            .collect();

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let variance = values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / values.len() as f64;

        for (i, feature) in features.iter().enumerate() {
            let value = anomaly_features.get(feature).unwrap();
            let baseline = baseline_features.get(feature).unwrap_or(&0.0);
            let deviation = (value - baseline).abs();
            
            // Score based on contribution to variance
            let contribution = (deviation - mean).powi(2) / variance;
            
            if contribution > 0.1 {
                candidates.push(RootCauseCandidate {
                    feature: feature.clone(),
                    score: contribution,
                    confidence: contribution.min(1.0),
                    evidence: vec![
                        format!("Variance contribution: {:.2}%", contribution * 100.0),
                        format!("Deviation: {:.2}", deviation),
                    ],
                    related_features: Vec::new(),
                });
            }
        }

        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        candidates
    }

    /// Ensemble analysis
    fn ensemble_analysis(
        &mut self,
        anomaly_features: &HashMap<String, f64>,
        baseline_features: &HashMap<String, f64>,
    ) -> Vec<RootCauseCandidate> {
        let causal = self.causal_graph_analysis(anomaly_features, baseline_features);
        let importance = self.feature_importance_analysis(anomaly_features, baseline_features);
        let correlation = self.correlation_analysis(anomaly_features, baseline_features);
        
        // Combine scores from multiple methods
        let mut ensemble_scores: HashMap<String, (f64, usize)> = HashMap::new();

        for candidate in &causal {
            let entry = ensemble_scores.entry(candidate.feature.clone())
                .or_insert((0.0, 0));
            entry.0 += candidate.score;
            entry.1 += 1;
        }

        for candidate in &importance {
            let entry = ensemble_scores.entry(candidate.feature.clone())
                .or_insert((0.0, 0));
            entry.0 += candidate.score;
            entry.1 += 1;
        }

        for candidate in &correlation {
            let entry = ensemble_scores.entry(candidate.feature.clone())
                .or_insert((0.0, 0));
            entry.0 += candidate.score;
            entry.1 += 1;
        }

        let mut candidates: Vec<RootCauseCandidate> = ensemble_scores
            .into_iter()
            .map(|(feature, (score, count))| {
                let avg_score = score / count as f64;
                RootCauseCandidate {
                    feature,
                    score: avg_score,
                    confidence: avg_score.min(1.0),
                    evidence: vec![
                        format!("Aggregated from {} methods", count),
                        format!("Total score: {:.2}", score),
                    ],
                    related_features: Vec::new(),
                }
            })
            .collect();

        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        candidates.truncate(10);
        candidates
    }

    /// Build causal graph
    fn build_causal_graph(&mut self, anomaly_features: &HashMap<String, f64>, baseline_features: &HashMap<String, f64>) {
        let features: Vec<String> = anomaly_features.keys().cloned().collect();

        // Simplified causal graph construction based on correlation
        for i in 0..features.len() {
            for j in i + 1..features.len() {
                let f1 = &features[i];
                let f2 = &features[j];

                if let (Some(v1), Some(b1), Some(v2), Some(b2)) = (
                    anomaly_features.get(f1),
                    baseline_features.get(f1),
                    anomaly_features.get(f2),
                    baseline_features.get(f2),
                ) {
                    let dev1 = (v1 - b1).abs() / (b1.abs() + 1e-10);
                    let dev2 = (v2 - b2).abs() / (b2.abs() + 1e-10);

                    // Assume causality from larger deviation
                    if dev1 > dev2 {
                        let strength = dev1 / (dev1 + dev2);
                        self.causal_graph.entry(f1.clone())
                            .or_insert_with(|| CausalGraphNode {
                                feature: f1.clone(),
                                parents: Vec::new(),
                                children: Vec::new(),
                                strength,
                            })
                            .children.push(f2.clone());

                        self.causal_graph.entry(f2.clone())
                            .or_insert_with(|| CausalGraphNode {
                                feature: f2.clone(),
                                parents: vec![f1.clone()],
                                children: Vec::new(),
                                strength,
                            });
                    } else if dev2 > dev1 {
                        let strength = dev2 / (dev1 + dev2);
                        self.causal_graph.entry(f2.clone())
                            .or_insert_with(|| CausalGraphNode {
                                feature: f2.clone(),
                                parents: Vec::new(),
                                children: Vec::new(),
                                strength,
                            })
                            .children.push(f1.clone());

                        self.causal_graph.entry(f1.clone())
                            .or_insert_with(|| CausalGraphNode {
                                feature: f1.clone(),
                                parents: vec![f2.clone()],
                                children: Vec::new(),
                                strength,
                            });
                    }
                }
            }
        }
    }

    /// Build timeline
    fn build_timeline(&self, anomaly_features: &HashMap<String, f64>, baseline_features: &HashMap<String, f64>) -> Vec<TimelineEvent> {
        let mut timeline = Vec::new();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut deviations: Vec<_> = anomaly_features.iter()
            .map(|(f, v)| {
                let baseline = baseline_features.get(f).unwrap_or(&0.0);
                let deviation = (v - baseline).abs() / (baseline.abs() + 1e-10);
                (f.clone(), deviation)
            })
            .collect();

        deviations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (feature, deviation) in deviations.iter().take(5) {
            timeline.push(TimelineEvent {
                timestamp,
                event_type: "deviation_detected".to_string(),
                description: format!("Feature {} deviation detected", feature),
                feature: Some(feature.clone()),
                value: anomaly_features.get(feature).copied(),
            });
        }

        timeline
    }

    /// Get analysis history
    pub fn get_history(&self) -> &[RootCauseResult] {
        &self.history
    }

    /// Reset analyzer
    pub fn reset(&mut self) {
        self.feature_importance.clear();
        self.correlations.clear();
        self.causal_graph.clear();
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_cause_analyzer() {
        let mut analyzer = RootCauseAnalyzer::new(AnalysisMethod::FeatureImportance);

        let mut anomaly = HashMap::new();
        anomaly.insert("cpu".to_string(), 95.0);
        anomaly.insert("memory".to_string(), 80.0);

        let mut baseline = HashMap::new();
        baseline.insert("cpu".to_string(), 50.0);
        baseline.insert("memory".to_string(), 40.0);

        let result = analyzer.analyze(&anomaly, &baseline);
        assert!(result.is_ok());
        assert!(result.unwrap().candidates.len() > 0);
    }

    #[test]
    fn test_causal_graph_analysis() {
        let mut analyzer = RootCauseAnalyzer::new(AnalysisMethod::CausalGraph);

        let mut anomaly = HashMap::new();
        anomaly.insert("feature1".to_string(), 100.0);
        anomaly.insert("feature2".to_string(), 60.0);

        let mut baseline = HashMap::new();
        baseline.insert("feature1".to_string(), 50.0);
        baseline.insert("feature2".to_string(), 50.0);

        let result = analyzer.analyze(&anomaly, &baseline);
        assert!(result.is_ok());
        
        let candidates = result.unwrap().candidates;
        assert!(candidates.len() > 0);
        assert!(candidates[0].feature == "feature1");
    }

    #[test]
    fn test_correlation_analysis() {
        let mut analyzer = RootCauseAnalyzer::new(AnalysisMethod::Correlation);

        // Add some correlations
        analyzer.correlations.insert(("cpu".to_string(), "memory".to_string()), 0.8);

        let mut anomaly = HashMap::new();
        anomaly.insert("cpu".to_string(), 95.0);

        let mut baseline = HashMap::new();
        baseline.insert("cpu".to_string(), 50.0);

        let result = analyzer.analyze(&anomaly, &baseline);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pca_analysis() {
        let mut analyzer = RootCauseAnalyzer::new(AnalysisMethod::PCA);

        let mut anomaly = HashMap::new();
        anomaly.insert("f1".to_string(), 100.0);
        anomaly.insert("f2".to_string(), 80.0);
        anomaly.insert("f3".to_string(), 60.0);

        let mut baseline = HashMap::new();
        baseline.insert("f1".to_string(), 50.0);
        baseline.insert("f2".to_string(), 50.0);
        baseline.insert("f3".to_string(), 50.0);

        let result = analyzer.analyze(&anomaly, &baseline);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ensemble_analysis() {
        let mut analyzer = RootCauseAnalyzer::new(AnalysisMethod::Ensemble);

        let mut anomaly = HashMap::new();
        anomaly.insert("cpu".to_string(), 95.0);
        anomaly.insert("memory".to_string(), 80.0);

        let mut baseline = HashMap::new();
        baseline.insert("cpu".to_string(), 50.0);
        baseline.insert("memory".to_string(), 40.0);

        let result = analyzer.analyze(&anomaly, &baseline);
        assert!(result.is_ok());
        
        let candidates = result.unwrap().candidates;
        assert!(candidates.len() > 0);
    }

    #[test]
    fn test_timeline() {
        let mut analyzer = RootCauseAnalyzer::new(AnalysisMethod::FeatureImportance);

        let mut anomaly = HashMap::new();
        anomaly.insert("cpu".to_string(), 95.0);

        let mut baseline = HashMap::new();
        baseline.insert("cpu".to_string(), 50.0);

        let result = analyzer.analyze(&anomaly, &baseline).unwrap();
        assert!(result.timeline.len() > 0);
    }

    #[test]
    fn test_reset() {
        let mut analyzer = RootCauseAnalyzer::new(AnalysisMethod::FeatureImportance);

        let mut anomaly = HashMap::new();
        anomaly.insert("cpu".to_string(), 95.0);

        let mut baseline = HashMap::new();
        baseline.insert("cpu".to_string(), 50.0);

        analyzer.analyze(&anomaly, &baseline).unwrap();
        analyzer.reset();

        assert_eq!(analyzer.history.len(), 0);
    }
}