//! AI Transparency and Explainability Module
//!
//! This module provides transparency features for AI decision-making,
//! enabling users to understand how AI systems reach their conclusions.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Type of explanation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExplanationType {
    /// Feature importance explanation
    FeatureImportance,
    /// SHAP values explanation
    Shap,
    /// LIME explanation
    Lime,
    /// Decision tree path
    DecisionTreePath,
    /// Rule-based explanation
    RuleBased,
    /// Counterfactual explanation
    Counterfactual,
    /// Attention weights visualization
    AttentionWeights,
    /// Saliency map
    SaliencyMap,
    /// Natural language explanation
    NaturalLanguage,
}

/// Decision explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionExplanation {
    /// Explanation ID
    pub id: String,
    /// Decision ID being explained
    pub decision_id: String,
    /// Type of explanation
    pub explanation_type: ExplanationType,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Input features
    pub input_features: HashMap<String, f64>,
    /// Feature importance scores
    pub feature_importance: HashMap<String, f64>,
    /// Human-readable explanation
    pub summary: String,
    /// Detailed explanation
    pub details: String,
    /// Confidence in the explanation
    pub confidence: f64,
    /// Alternative outcomes considered
    pub alternatives: Vec<AlternativeOutcome>,
    /// Model metadata
    pub model_info: ModelMetadata,
}

/// Alternative outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeOutcome {
    /// Outcome name
    pub outcome: String,
    /// Probability
    pub probability: f64,
    /// Reason for not being selected
    pub rejection_reason: String,
}

/// Model metadata for transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Model type
    pub model_type: String,
    /// Training data summary
    pub training_data_summary: String,
    /// Known limitations
    pub limitations: Vec<String>,
    /// Performance metrics
    pub performance_metrics: HashMap<String, f64>,
}

/// Explanation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyConfig {
    /// Enable feature importance
    pub enable_feature_importance: bool,
    /// Enable SHAP explanations
    pub enable_shap: bool,
    /// Enable LIME explanations
    pub enable_lime: bool,
    /// Enable counterfactual explanations
    pub enable_counterfactual: bool,
    /// Enable natural language explanations
    pub enable_natural_language: bool,
    /// Maximum features to explain
    pub max_features: usize,
    /// Minimum importance threshold
    pub min_importance_threshold: f64,
}

impl Default for TransparencyConfig {
    fn default() -> Self {
        Self {
            enable_feature_importance: true,
            enable_shap: true,
            enable_lime: false,
            enable_counterfactual: true,
            enable_natural_language: true,
            max_features: 10,
            min_importance_threshold: 0.01,
        }
    }
}

/// Transparency statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransparencyStats {
    /// Total explanations generated
    pub total_explanations: usize,
    /// Explanations by type
    pub by_type: HashMap<ExplanationType, usize>,
    /// Average explanation confidence
    pub avg_confidence: f64,
    /// User feedback received
    pub user_feedback_count: usize,
}

/// Transparency Manager
pub struct TransparencyManager {
    config: TransparencyConfig,
    explanations: Arc<RwLock<HashMap<String, DecisionExplanation>>>,
    model_registry: Arc<RwLock<HashMap<String, ModelMetadata>>>,
    statistics: Arc<RwLock<TransparencyStats>>,
}

impl TransparencyManager {
    /// Create a new transparency manager
    pub fn new() -> Result<Self> {
        info!("Initializing Transparency Manager");

        Ok(Self {
            config: TransparencyConfig::default(),
            explanations: Arc::new(RwLock::new(HashMap::new())),
            model_registry: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(TransparencyStats {
                total_explanations: 0,
                by_type: HashMap::new(),
                avg_confidence: 0.0,
                user_feedback_count: 0,
            })),
        })
    }

    /// Register a model for transparency
    pub async fn register_model(&self, metadata: ModelMetadata) -> Result<()> {
        let mut registry = self.model_registry.write().await;
        registry.insert(metadata.name.clone(), metadata);
        debug!("Registered model in transparency system");
        Ok(())
    }

    /// Generate explanation for a decision
    pub async fn explain_decision(
        &self,
        decision_id: String,
        input_features: HashMap<String, f64>,
        prediction: f64,
        model_name: &str,
        explanation_type: ExplanationType,
    ) -> Result<DecisionExplanation> {
        let registry = self.model_registry.read().await;
        let model_info = registry.get(model_name).cloned().unwrap_or_else(|| ModelMetadata {
            name: model_name.to_string(),
            version: "unknown".to_string(),
            model_type: "unknown".to_string(),
            training_data_summary: "Not available".to_string(),
            limitations: vec!["Model metadata not registered".to_string()],
            performance_metrics: HashMap::new(),
        });
        drop(registry);

        // Calculate feature importance based on input values
        let feature_importance = self.calculate_feature_importance(&input_features, explanation_type)?;

        // Generate human-readable summary
        let summary = self.generate_summary(&feature_importance, prediction)?;
        let details = self.generate_details(&feature_importance, &input_features)?;

        // Generate alternative outcomes
        let alternatives = self.generate_alternatives(prediction, &input_features)?;

        // Calculate explanation confidence
        let confidence = self.calculate_confidence(&feature_importance);

        let explanation = DecisionExplanation {
            id: uuid::Uuid::new_v4().to_string(),
            decision_id,
            explanation_type,
            timestamp: chrono::Utc::now(),
            input_features,
            feature_importance,
            summary,
            details,
            confidence,
            alternatives,
            model_info,
        };

        // Store and update statistics
        {
            let mut explanations = self.explanations.write().await;
            explanations.insert(explanation.id.clone(), explanation.clone());
        }

        {
            let mut stats = self.statistics.write().await;
            stats.total_explanations += 1;
            *stats.by_type.entry(explanation_type).or_insert(0) += 1;
            stats.avg_confidence = (stats.avg_confidence * (stats.total_explanations - 1) as f64
                + confidence)
                / stats.total_explanations as f64;
        }

        info!("Generated {} explanation with confidence {:.2}", 
              format!("{:?}", explanation_type).to_lowercase(), confidence);

        Ok(explanation)
    }

    /// Calculate feature importance
    fn calculate_feature_importance(
        &self,
        features: &HashMap<String, f64>,
        explanation_type: ExplanationType,
    ) -> Result<HashMap<String, f64>> {
        let mut importance = HashMap::new();

        match explanation_type {
            ExplanationType::FeatureImportance | ExplanationType::NaturalLanguage => {
                // Simple normalized importance based on absolute feature value
                let total: f64 = features.values().map(|v| v.abs()).sum();
                if total > 0.0 {
                    for (key, value) in features {
                        let imp = value.abs() / total;
                        if imp >= self.config.min_importance_threshold {
                            importance.insert(key.clone(), imp);
                        }
                    }
                }
            }
            ExplanationType::Shap => {
                // Simulated SHAP values (in production, use actual SHAP library)
                for (key, value) in features {
                    let shap_value = value * 0.1 + (rand::random::<f64>() - 0.5) * 0.05;
                    if shap_value.abs() >= self.config.min_importance_threshold {
                        importance.insert(key.clone(), shap_value.abs());
                    }
                }
            }
            ExplanationType::Counterfactual => {
                // Identify features that would change the decision
                for (key, value) in features {
                    let cf_importance = (value.abs() * 1.5).min(1.0);
                    if cf_importance >= self.config.min_importance_threshold {
                        importance.insert(key.clone(), cf_importance);
                    }
                }
            }
            _ => {
                // Default: equal importance
                let equal_weight = 1.0 / features.len() as f64;
                for key in features.keys() {
                    importance.insert(key.clone(), equal_weight);
                }
            }
        }

        // Sort and limit by max_features
        let mut sorted: Vec<_> = importance.into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sorted.truncate(self.config.max_features);

        Ok(sorted.into_iter().collect())
    }

    /// Generate human-readable summary
    fn generate_summary(
        &self,
        importance: &HashMap<String, f64>,
        prediction: f64,
    ) -> Result<String> {
        let mut top_features: Vec<_> = importance.iter().collect();
        top_features.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let top_3: Vec<_> = top_features.iter().take(3).collect();

        let feature_list = top_3
            .iter()
            .map(|(k, v)| format!("{} ({:.1}%)", k, v * 100.0))
            .collect::<Vec<_>>()
            .join(", ");

        Ok(format!(
            "The decision was primarily influenced by: {}. The model prediction was {:.2}.",
            feature_list, prediction
        ))
    }

    /// Generate detailed explanation
    fn generate_details(
        &self,
        importance: &HashMap<String, f64>,
        features: &HashMap<String, f64>,
    ) -> Result<String> {
        let mut details = String::new();
        details.push_str("Detailed Feature Analysis:\n\n");

        let mut sorted: Vec<_> = importance.iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        for (feature, imp) in sorted {
            let value = features.get(feature).unwrap_or(&0.0);
            details.push_str(&format!(
                "• {}: value = {:.2}, importance = {:.1}%\n",
                feature,
                value,
                imp * 100.0
            ));
        }

        details.push_str("\nThis explanation shows how each feature contributed to the decision. ");
        details.push_str("Higher importance values indicate greater influence on the outcome.");

        Ok(details)
    }

    /// Generate alternative outcomes
    fn generate_alternatives(
        &self,
        prediction: f64,
        features: &HashMap<String, f64>,
    ) -> Result<Vec<AlternativeOutcome>> {
        let mut alternatives = Vec::new();

        // In a real implementation, this would query the model for other possible outcomes
        if prediction > 0.5 {
            alternatives.push(AlternativeOutcome {
                outcome: "negative".to_string(),
                probability: 1.0 - prediction,
                rejection_reason: "Lower probability than positive outcome".to_string(),
            });
        } else {
            alternatives.push(AlternativeOutcome {
                outcome: "positive".to_string(),
                probability: prediction,
                rejection_reason: "Lower probability than negative outcome".to_string(),
            });
        }

        Ok(alternatives)
    }

    /// Calculate confidence in the explanation
    fn calculate_confidence(&self, importance: &HashMap<String, f64>) -> f64 {
        if importance.is_empty() {
            return 0.0;
        }

        // Higher confidence when importance is concentrated in few features
        let values: Vec<f64> = importance.values().cloned().collect();
        let max = values.iter().cloned().fold(0.0, f64::max);
        let sum: f64 = values.iter().sum();

        if sum > 0.0 {
            (max / sum) * 0.7 + 0.3 // Base confidence + concentration bonus
        } else {
            0.5
        }
    }

    /// Generate counterfactual explanation
    pub async fn generate_counterfactual(
        &self,
        decision_id: String,
        input_features: HashMap<String, f64>,
        model_name: &str,
        desired_outcome: f64,
    ) -> Result<HashMap<String, f64>> {
        // Simulate counterfactual generation
        // In production, use actual counterfactual generation algorithms
        let mut counterfactual = input_features.clone();

        // Identify the most important feature and modify it
        let feature_importance = self.calculate_feature_importance(&counterfactual, ExplanationType::Counterfactual)?;
        
        if let Some((top_feature, _)) = feature_importance.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)) {
            if let Some(value) = counterfactual.get_mut(top_feature) {
                // Modify the feature to flip the prediction
                *value = desired_outcome;
            }
        }

        debug!("Generated counterfactual for decision {}", decision_id);
        Ok(counterfactual)
    }

    /// Get explanation by ID
    pub async fn get_explanation(&self, id: &str) -> Option<DecisionExplanation> {
        let explanations = self.explanations.read().await;
        explanations.get(id).cloned()
    }

    /// Get explanations for a decision
    pub async fn get_decision_explanations(&self, decision_id: &str) -> Vec<DecisionExplanation> {
        let explanations = self.explanations.read().await;
        explanations
            .values()
            .filter(|e| e.decision_id == decision_id)
            .cloned()
            .collect()
    }

    /// Record user feedback on explanation
    pub async fn record_feedback(&self, explanation_id: &str, helpful: bool) -> Result<()> {
        let mut stats = self.statistics.write().await;
        stats.user_feedback_count += 1;
        debug!("Recorded feedback for explanation {}: helpful={}", explanation_id, helpful);
        Ok(())
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> TransparencyStats {
        self.statistics.read().await.clone()
    }

    /// Export explanations as report
    pub async fn export_explanations(&self, format: ExportFormat) -> Result<String> {
        let explanations = self.explanations.read().await;
        
        match format {
            ExportFormat::Json => {
                Ok(serde_json::to_string_pretty(&*explanations)?)
            }
            ExportFormat::Csv => {
                let mut csv = String::from("id,decision_id,type,confidence,summary\n");
                for exp in explanations.values() {
                    csv.push_str(&format!(
                        "{},{},{:?},{},{:?}\n",
                        exp.id,
                        exp.decision_id,
                        exp.explanation_type,
                        exp.confidence,
                        exp.summary
                    ));
                }
                Ok(csv)
            }
            ExportFormat::Html => {
                let mut html = String::from("<html><body><h1>AI Decision Explanations</h1>");
                for exp in explanations.values() {
                    html.push_str(&format!(
                        "<div><h2>Decision: {}</h2><p>{}</p><p>Confidence: {:.2}</p></div>",
                        exp.decision_id, exp.summary, exp.confidence
                    ));
                }
                html.push_str("</body></html>");
                Ok(html)
            }
        }
    }
}

/// Export format for explanations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Csv,
    Html,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transparency_manager_creation() {
        let manager = TransparencyManager::new().unwrap();
        let stats = manager.get_statistics().await;
        assert_eq!(stats.total_explanations, 0);
    }

    #[tokio::test]
    async fn test_generate_explanation() {
        let manager = TransparencyManager::new().unwrap();
        
        let mut features = HashMap::new();
        features.insert("age".to_string(), 35.0);
        features.insert("income".to_string(), 50000.0);
        features.insert("score".to_string(), 0.85);

        let explanation = manager
            .explain_decision(
                "decision-1".to_string(),
                features,
                0.75,
                "test_model",
                ExplanationType::FeatureImportance,
            )
            .await
            .unwrap();

        assert!(!explanation.feature_importance.is_empty());
        assert!(!explanation.summary.is_empty());
    }

    #[tokio::test]
    async fn test_counterfactual() {
        let manager = TransparencyManager::new().unwrap();
        
        let mut features = HashMap::new();
        features.insert("age".to_string(), 35.0);
        features.insert("income".to_string(), 50000.0);

        let cf = manager
            .generate_counterfactual(
                "decision-1".to_string(),
                features,
                "test_model",
                0.3,
            )
            .await
            .unwrap();

        assert!(!cf.is_empty());
    }
}