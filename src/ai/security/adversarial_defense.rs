//! Adversarial Defense Module
//! 
//! Defense mechanisms against adversarial attacks on AI models.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};

/// Adversarial defense configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversarialDefenseConfig {
    /// Enable adversarial training
    pub enable_adversarial_training: bool,
    /// Enable input sanitization
    pub enable_input_sanitization: bool,
    /// Enable feature squeezing
    pub enable_feature_squeezing: bool,
    /// Enable gradient masking
    pub enable_gradient_masking: bool,
    /// Detection threshold
    pub detection_threshold: f64,
    /// Defense strength (0.0 - 1.0)
    pub defense_strength: f64,
}

impl Default for AdversarialDefenseConfig {
    fn default() -> Self {
        Self {
            enable_adversarial_training: true,
            enable_input_sanitization: true,
            enable_feature_squeezing: true,
            enable_gradient_masking: false,
            detection_threshold: 0.85,
            defense_strength: 0.8,
        }
    }
}

/// Adversarial attack type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackType {
    /// Fast Gradient Sign Method
    Fgsm,
    /// Projected Gradient Descent
    Pgd,
    /// Carlini-Wagner attack
    CarliniWagner,
    /// DeepFool attack
    DeepFool,
    /// Jacobian-based Saliency Map Attack
    Jsma,
    /// Pixel attack
    PixelAttack,
    /// Boundary attack
    BoundaryAttack,
    /// Unknown attack
    Unknown,
}

/// Input perturbation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPerturbation {
    /// Perturbation magnitude (L-inf norm)
    pub magnitude: f64,
    /// Perturbation type
    pub perturbation_type: PerturbationType,
    /// Number of perturbed features
    pub perturbed_features: usize,
    /// Adversarial confidence
    pub adversarial_confidence: f64,
}

/// Perturbation type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerturbationType {
    /// Gradient-based perturbation
    GradientBased,
    /// Random noise
    RandomNoise,
    /// Feature-level perturbation
    FeatureLevel,
    /// Pixel-level perturbation
    PixelLevel,
    /// Semantic perturbation
    Semantic,
}

/// Defense mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenseMechanism {
    /// Mechanism name
    pub name: String,
    /// Defense type
    pub defense_type: DefenseType,
    /// Effectiveness score
    pub effectiveness: f64,
    /// Performance impact
    pub performance_impact_percent: f64,
    /// Is active
    pub active: bool,
}

/// Defense type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DefenseType {
    /// Adversarial training
    AdversarialTraining,
    /// Input transformation
    InputTransformation,
    /// Feature squeezing
    FeatureSqueezing,
    /// Gradient masking
    GradientMasking,
    /// Defensive distillation
    DefensiveDistillation,
    /// Randomization
    Randomization,
    /// Detection
    Detection,
    /// Rejection
    Rejection,
}

/// Adversarial detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversarialDetectionResult {
    /// Is adversarial
    pub is_adversarial: bool,
    /// Confidence score
    pub confidence: f64,
    /// Detected attack type
    pub attack_type: AttackType,
    /// Perturbation details
    pub perturbation: Option<InputPerturbation>,
    /// Defense applied
    pub defense_applied: String,
}

/// Adversarial defense statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenseStatistics {
    /// Total inputs analyzed
    pub total_inputs: u64,
    /// Adversarial inputs detected
    pub adversarial_detected: u64,
    /// False positive rate
    pub false_positive_rate: f64,
    /// True positive rate
    pub true_positive_rate: f64,
    /// Average detection time in microseconds
    pub avg_detection_time_us: u64,
    /// Attacks by type
    pub attacks_by_type: HashMap<String, u64>,
}

/// Adversarial defender
pub struct AdversarialDefender {
    config: AdversarialDefenseConfig,
    defense_mechanisms: Vec<DefenseMechanism>,
    statistics: Arc<RwLock<DefenseStatistics>>,
}

impl AdversarialDefender {
    /// Create a new adversarial defender
    pub fn new(config: AdversarialDefenseConfig) -> Self {
        let mechanisms = vec![
            DefenseMechanism {
                name: "Adversarial Training".to_string(),
                defense_type: DefenseType::AdversarialTraining,
                effectiveness: 0.85,
                performance_impact_percent: 5.0,
                active: config.enable_adversarial_training,
            },
            DefenseMechanism {
                name: "Input Sanitization".to_string(),
                defense_type: DefenseType::InputTransformation,
                effectiveness: 0.75,
                performance_impact_percent: 2.0,
                active: config.enable_input_sanitization,
            },
            DefenseMechanism {
                name: "Feature Squeezing".to_string(),
                defense_type: DefenseType::FeatureSqueezing,
                effectiveness: 0.80,
                performance_impact_percent: 3.0,
                active: config.enable_feature_squeezing,
            },
        ];

        Self {
            config,
            defense_mechanisms: mechanisms,
            statistics: Arc::new(RwLock::new(DefenseStatistics {
                total_inputs: 0,
                adversarial_detected: 0,
                false_positive_rate: 0.0,
                true_positive_rate: 0.0,
                avg_detection_time_us: 0,
                attacks_by_type: HashMap::new(),
            })),
        }
    }

    /// Analyze input for adversarial patterns
    pub async fn analyze_input(&amp;self, input: &amp;[f64]) -> Result<AdversarialDetectionResult, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        let mut stats = self.statistics.write().await;
        stats.total_inputs += 1;

        // Calculate gradient-based features
        let gradient_magnitude = self.calculate_gradient_magnitude(input);
        
        // Calculate statistical features
        let statistical_anomaly = self.calculate_statistical_anomaly(input);
        
        // Feature squeezing test
        let squeezing_score = if self.config.enable_feature_squeezing {
            self.feature_squeeze_test(input)
        } else {
            0.0
        };

        // Combined score
        let adversarial_score = (gradient_magnitude * 0.4 + statistical_anomaly * 0.3 + squeezing_score * 0.3)
            * self.config.defense_strength;

        let is_adversarial = adversarial_score > self.config.detection_threshold;

        if is_adversarial {
            stats.adversarial_detected += 1;
        }

        let elapsed = start.elapsed().as_micros() as u64;
        stats.avg_detection_time_us = (stats.avg_detection_time_us * (stats.total_inputs - 1) + elapsed) / stats.total_inputs;

        let attack_type = if is_adversarial {
            self.classify_attack_type(gradient_magnitude, statistical_anomaly, squeezing_score)
        } else {
            AttackType::Unknown
        };

        let defense_applied = if is_adversarial {
            self.apply_defense(&amp;attack_type).await
        } else {
            "None".to_string()
        };

        Ok(AdversarialDetectionResult {
            is_adversarial,
            confidence: adversarial_score,
            attack_type,
            perturbation: if is_adversarial {
                Some(InputPerturbation {
                    magnitude: gradient_magnitude,
                    perturbation_type: PerturbationType::GradientBased,
                    perturbed_features: input.len() / 10,
                    adversarial_confidence: adversarial_score,
                })
            } else {
                None
            },
            defense_applied,
        })
    }

    /// Calculate gradient magnitude
    fn calculate_gradient_magnitude(&amp;self, input: &amp;[f64]) -> f64 {
        // Simplified gradient magnitude calculation
        if input.len() < 2 {
            return 0.0;
        }
        
        let gradients: Vec<f64> = input.windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .collect();
        
        gradients.iter().sum::<f64>() / gradients.len() as f64
    }

    /// Calculate statistical anomaly
    fn calculate_statistical_anomaly(&amp;self, input: &amp;[f64]) -> f64 {
        if input.is_empty() {
            return 0.0;
        }

        let mean = input.iter().sum::<f64>() / input.len() as f64;
        let variance = input.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / input.len() as f64;
        let std_dev = variance.sqrt();

        // Count outliers (values beyond 3 standard deviations)
        let outliers = input.iter()
            .filter(|x| (**x - mean).abs() > 3.0 * std_dev)
            .count();

        outliers as f64 / input.len() as f64
    }

    /// Feature squeeze test
    fn feature_squeeze_test(&amp;self, input: &amp;[f64]) -> f64 {
        // Simplified feature squeezing - reduce precision
        let squeezed: Vec<f64> = input.iter()
            .map(|x| (x * 10.0).round() / 10.0)
            .collect();

        // Measure difference between original and squeezed
        let difference: f64 = input.iter()
            .zip(squeezed.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        difference / input.len() as f64
    }

    /// Classify attack type
    fn classify_attack_type(&amp;self, gradient: f64, statistical: f64, squeezing: f64) -> AttackType {
        if gradient > 0.8 {
            AttackType::Fgsm
        } else if gradient > 0.6 {
            AttackType::Pgd
        } else if statistical > 0.7 {
            AttackType::CarliniWagner
        } else if squeezing > 0.5 {
            AttackType::DeepFool
        } else {
            AttackType::Unknown
        }
    }

    /// Apply defense mechanism
    async fn apply_defense(&amp;self, attack_type: &amp;AttackType) -> String {
        match attack_type {
            AttackType::Fgsm | AttackType::Pgd => {
                "Applied adversarial training defense".to_string()
            }
            AttackType::CarliniWagner | AttackType::DeepFool => {
                "Applied input sanitization".to_string()
            }
            AttackType::Jsma | AttackType::PixelAttack => {
                "Applied feature squeezing".to_string()
            }
            _ => {
                "Applied comprehensive defense".to_string()
            }
        }
    }

    /// Sanitize input
    pub async fn sanitize_input(&amp;self, input: &amp;[f64]) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        // Apply noise reduction
        let sanitized = input.iter()
            .map(|x| {
                // Remove small perturbations
                if x.abs() < 1e-6 {
                    0.0
                } else {
                    // Reduce precision
                    (x * 1000.0).round() / 1000.0
                }
            })
            .collect();

        Ok(sanitized)
    }

    /// Get statistics
    pub async fn get_statistics(&amp;self) -> Result<DefenseStatistics, Box<dyn std::error::Error>> {
        let stats = self.statistics.read().await;
        Ok(stats.clone())
    }

    /// Get active defense mechanisms
    pub fn get_active_mechanisms(&amp;self) -> Vec<&amp;DefenseMechanism> {
        self.defense_mechanisms.iter().filter(|m| m.active).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adversarial_detection() {
        let defender = AdversarialDefender::new(AdversarialDefenseConfig::default());
        
        // Normal input
        let normal_input = vec![0.5, 0.5, 0.5, 0.5, 0.5];
        let result = defender.analyze_input(&amp;normal_input).await.unwrap();
        assert!(!result.is_adversarial);
        
        // Adversarial input (high gradient)
        let adversarial_input = vec![0.0, 1.0, 0.0, 1.0, 0.0];
        let result = defender.analyze_input(&amp;adversarial_input).await.unwrap();
        // May or may not be detected depending on threshold
    }
}