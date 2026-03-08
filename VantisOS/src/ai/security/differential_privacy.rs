//! Differential Privacy for AI Data Protection
//!
//! This module provides differential privacy mechanisms to protect individual data points
//! in training datasets while maintaining statistical utility for model training.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Differential privacy mechanisms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyMechanism {
    /// Local differential privacy - noise added at data source
    Local,
    /// Central differential privacy - noise added by curator
    Central,
    /// Distributed differential privacy - multiple curators
    Distributed,
}

/// Noise distribution types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoiseDistribution {
    /// Laplace mechanism
    Laplace,
    /// Gaussian mechanism
    Gaussian,
    /// Exponential mechanism
    Exponential,
}

/// Configuration for differential privacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifferentialPrivacyConfig {
    /// Privacy budget (epsilon) - smaller = more private
    pub epsilon: f64,
    /// Delta for Gaussian mechanism
    pub delta: f64,
    /// Sensitivity of the query/function
    pub sensitivity: f64,
    /// Privacy mechanism to use
    pub mechanism: PrivacyMechanism,
    /// Noise distribution
    pub distribution: NoiseDistribution,
    /// Enable composition tracking
    pub track_composition: bool,
    /// Minimum epsilon threshold
    pub min_epsilon: f64,
}

impl Default for DifferentialPrivacyConfig {
    fn default() -> Self {
        Self {
            epsilon: 1.0,
            delta: 1e-5,
            sensitivity: 1.0,
            mechanism: PrivacyMechanism::Local,
            distribution: NoiseDistribution::Laplace,
            track_composition: true,
            min_epsilon: 0.1,
        }
    }
}

/// Composition theorem tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionTracker {
    /// Total epsilon used
    pub total_epsilon: f64,
    /// Total delta used
    pub total_delta: f64,
    /// Number of queries made
    pub query_count: usize,
    /// Composition theorem used
    pub composition_type: CompositionType,
}

/// Composition theorems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompositionType {
    /// Basic composition (sum of epsilons)
    Basic,
    /// Advanced composition (optimal bound)
    Advanced,
    /// Moments accountant
    MomentsAccountant,
}

impl Default for CompositionTracker {
    fn default() -> Self {
        Self {
            total_epsilon: 0.0,
            total_delta: 0.0,
            query_count: 0,
            composition_type: CompositionType::Advanced,
        }
    }
}

/// Privacy budget allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetAllocation {
    /// Allocated for model training
    pub training_budget: f64,
    /// Allocated for inference
    pub inference_budget: f64,
    /// Allocated for analytics
    pub analytics_budget: f64,
    /// Remaining budget
    pub remaining_budget: f64,
}

/// Differential privacy statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyStatistics {
    /// Number of privatized queries
    pub privatized_queries: usize,
    /// Average noise magnitude added
    pub avg_noise_magnitude: f64,
    /// Total privacy budget consumed
    pub total_budget_consumed: f64,
    /// Budget exhausted flag
    pub budget_exhausted: bool,
    /// Number of budget violations
    pub budget_violations: usize,
}

/// Differential Privacy Manager
pub struct DifferentialPrivacyManager {
    config: DifferentialPrivacyConfig,
    tracker: Arc<RwLock<CompositionTracker>>,
    allocation: Arc<RwLock<BudgetAllocation>>,
    statistics: Arc<RwLock<PrivacyStatistics>>,
    query_history: Arc<RwLock<Vec<PrivacyQuery>>>,
}

/// Privacy query record
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PrivacyQuery {
    query_id: String,
    epsilon_used: f64,
    delta_used: f64,
    noise_added: f64,
    timestamp: chrono::DateTime<chrono::Utc>,
    query_type: String,
}

impl DifferentialPrivacyManager {
    /// Create a new differential privacy manager
    pub fn new(config: DifferentialPrivacyConfig) -> Self {
        let initial_budget = config.epsilon;
        
        let allocation = BudgetAllocation {
            training_budget: initial_budget * 0.6,
            inference_budget: initial_budget * 0.2,
            analytics_budget: initial_budget * 0.2,
            remaining_budget: initial_budget,
        };

        Self {
            config,
            tracker: Arc::new(RwLock::new(CompositionTracker::default())),
            allocation: Arc::new(RwLock::new(allocation)),
            statistics: Arc::new(RwLock::new(PrivacyStatistics {
                privatized_queries: 0,
                avg_noise_magnitude: 0.0,
                total_budget_consumed: 0.0,
                budget_exhausted: false,
                budget_violations: 0,
            })),
            query_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add Laplace noise to a value
    pub fn add_laplace_noise(&self, value: f64, sensitivity: f64, epsilon: f64) -> f64 {
        use rand::distributions::Distribution;
        
        let scale = sensitivity / epsilon;
        let lambda = 1.0 / scale;
        
        // Sample from exponential distribution for Laplace
        let u: f64 = rand::random();
        let noise = if u >= 0.5 {
            -scale * (1.0 - lambda).ln_1p()
        } else {
            scale * (-lambda).ln_1p()
        };
        
        value + noise
    }

    /// Add Gaussian noise to a value
    pub fn add_gaussian_noise(&self, value: f64, sensitivity: f64, epsilon: f64, delta: f64) -> f64 {
        use rand::distributions::{Distribution, Normal};
        
        let sigma = sensitivity * f64::sqrt(2.0 * (1.0 / epsilon).ln_1p()) / epsilon;
        let normal = Normal::new(0.0, sigma).unwrap();
        let noise = normal.sample(&mut rand::thread_rng());
        
        value + noise
    }

    /// Apply exponential mechanism for selection queries
    pub fn exponential_mechanism<T>(
        &self,
        options: Vec<T>,
        score_fn: impl Fn(&T) -> f64,
        sensitivity: f64,
    ) -> Result<T>
    where
        T: Clone,
    {
        let epsilon = self.config.epsilon;
        let mut scores: Vec<(T, f64)> = options
            .iter()
            .map(|opt| (opt.clone(), score_fn(opt)))
            .collect();
        
        // Normalize scores for numerical stability
        let max_score = scores.iter().map(|(_, s)| *s).fold(f64::NEG_INFINITY, f64::max);
        scores = scores
            .into_iter()
            .map(|(opt, score)| (opt, score - max_score))
            .collect();
        
        // Calculate probabilities
        let mut probabilities: Vec<f64> = scores
            .iter()
            .map(|(_, score)| (epsilon * score / (2.0 * sensitivity)).exp())
            .collect();
        
        // Normalize probabilities
        let sum: f64 = probabilities.iter().sum();
        if sum == 0.0 {
            return Err(anyhow::anyhow!("All probabilities are zero"));
        }
        
        probabilities = probabilities.iter().map(|p| p / sum).collect();
        
        // Sample based on probabilities
        let u: f64 = rand::random();
        let mut cumulative = 0.0;
        for (i, prob) in probabilities.iter().enumerate() {
            cumulative += prob;
            if u <= cumulative {
                return Ok(scores[i].0.clone());
            }
        }
        
        Ok(scores.last().unwrap().0.clone())
    }

    /// Privatize a numeric value
    pub async fn privatize_value(&self, value: f64, query_type: &str) -> Result<f64> {
        let epsilon = self.config.epsilon;
        
        // Check budget
        let mut stats = self.statistics.write().await;
        if stats.budget_exhausted {
            warn!("Privacy budget exhausted for query: {}", query_type);
            stats.budget_violations += 1;
            return Err(anyhow::anyhow!("Privacy budget exhausted"));
        }
        drop(stats);
        
        // Add noise based on distribution
        let (noisy_value, noise_magnitude) = match self.config.distribution {
            NoiseDistribution::Laplace => {
                let noisy = self.add_laplace_noise(value, self.config.sensitivity, epsilon);
                (noisy, (noisy - value).abs())
            }
            NoiseDistribution::Gaussian => {
                let noisy = self.add_gaussian_noise(value, self.config.sensitivity, epsilon, self.config.delta);
                (noisy, (noisy - value).abs())
            }
            NoiseDistribution::Exponential => {
                // For numeric values, use Laplace as fallback
                let noisy = self.add_laplace_noise(value, self.config.sensitivity, epsilon);
                (noisy, (noisy - value).abs())
            }
        };
        
        // Update tracking
        if self.config.track_composition {
            self.update_composition(epsilon, self.config.delta, noise_magnitude, query_type).await?;
        }
        
        Ok(noisy_value)
    }

    /// Privatize a vector of values
    pub async fn privatize_vector(&self, values: Vec<f64>, query_type: &str) -> Result<Vec<f64>> {
        let mut privatized = Vec::with_capacity(values.len());
        
        for value in values {
            let noisy = self.privatize_value(value, query_type).await?;
            privatized.push(noisy);
        }
        
        Ok(privatized)
    }

    /// Privatize a histogram
    pub async fn privatize_histogram(&self, counts: HashMap<String, usize>) -> Result<HashMap<String, f64>> {
        let mut privatized = HashMap::new();
        
        for (key, count) in counts {
            let count_f64 = count as f64;
            let noisy = self.privatize_value(count_f64, "histogram_query").await?;
            privatized.insert(key, noisy.max(0.0)); // Ensure non-negative
        }
        
        Ok(privatized)
    }

    /// Update composition tracking
    async fn update_composition(
        &self,
        epsilon: f64,
        delta: f64,
        noise_magnitude: f64,
        query_type: &str,
    ) -> Result<()> {
        // Update tracker
        let mut tracker = self.tracker.write().await;
        
        match tracker.composition_type {
            CompositionType::Basic => {
                tracker.total_epsilon += epsilon;
                tracker.total_delta += delta;
            }
            CompositionType::Advanced => {
                // Advanced composition theorem
                let k = (tracker.query_count + 1) as f64;
                tracker.total_epsilon += epsilon * (2.0 * k * (1.0 + epsilon).ln()).sqrt();
                tracker.total_delta += delta * k;
            }
            CompositionType::MomentsAccountant => {
                // Moments accountant provides tighter bounds
                tracker.total_epsilon += epsilon;
                tracker.total_delta += delta;
            }
        }
        
        tracker.query_count += 1;
        drop(tracker);
        
        // Update statistics
        let mut stats = self.statistics.write().await;
        stats.privatized_queries += 1;
        stats.avg_noise_magnitude = 
            (stats.avg_noise_magnitude * (stats.privatized_queries - 1) as f64 + noise_magnitude) 
            / stats.privatized_queries as f64;
        stats.total_budget_consumed += epsilon;
        
        // Check if budget is exhausted
        if stats.total_budget_consumed >= self.config.epsilon {
            stats.budget_exhausted = true;
            warn!("Privacy budget exhausted: {:.2}/{:.2}", 
                  stats.total_budget_consumed, self.config.epsilon);
        }
        drop(stats);
        
        // Update allocation
        let mut allocation = self.allocation.write().await;
        allocation.remaining_budget -= epsilon;
        drop(allocation);
        
        // Record query
        let query_id = uuid::Uuid::new_v4().to_string();
        let query = PrivacyQuery {
            query_id,
            epsilon_used: epsilon,
            delta_used: delta,
            noise_added: noise_magnitude,
            timestamp: chrono::Utc::now(),
            query_type: query_type.to_string(),
        };
        
        let mut history = self.query_history.write().await;
        history.push(query);
        
        debug!("Composition updated: epsilon={:.4}, delta={:.6}", epsilon, delta);
        
        Ok(())
    }

    /// Allocate privacy budget
    pub async fn allocate_budget(&self, training: f64, inference: f64, analytics: f64) -> Result<()> {
        let total = training + inference + analytics;
        if total > self.config.epsilon {
            return Err(anyhow::anyhow!(
                "Total allocation {:.2} exceeds budget {:.2}",
                total,
                self.config.epsilon
            ));
        }
        
        let mut allocation = self.allocation.write().await;
        allocation.training_budget = training;
        allocation.inference_budget = inference;
        allocation.analytics_budget = analytics;
        allocation.remaining_budget = self.config.epsilon - total;
        
        info!("Privacy budget allocated: training={:.2}, inference={:.2}, analytics={:.2}",
              training, inference, analytics);
        
        Ok(())
    }

    /// Get remaining budget
    pub async fn get_remaining_budget(&self) -> f64 {
        let allocation = self.allocation.read().await;
        allocation.remaining_budget
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> PrivacyStatistics {
        self.statistics.read().await.clone()
    }

    /// Get composition status
    pub async fn get_composition_status(&self) -> (f64, f64, usize) {
        let tracker = self.tracker.read().await;
        (tracker.total_epsilon, tracker.total_delta, tracker.query_count)
    }

    /// Reset privacy budget (e.g., for new time period)
    pub async fn reset_budget(&self) -> Result<()> {
        let mut stats = self.statistics.write().await;
        stats.privatized_queries = 0;
        stats.total_budget_consumed = 0.0;
        stats.budget_exhausted = false;
        drop(stats);
        
        let mut allocation = self.allocation.write().await;
        allocation.remaining_budget = self.config.epsilon;
        
        let mut tracker = self.tracker.write().await;
        tracker.total_epsilon = 0.0;
        tracker.total_delta = 0.0;
        tracker.query_count = 0;
        
        let mut history = self.query_history.write().await;
        history.clear();
        
        info!("Privacy budget reset to {}", self.config.epsilon);
        
        Ok(())
    }

    /// Check if query can be made within budget
    pub async fn can_make_query(&self, epsilon: f64) -> bool {
        let remaining = self.get_remaining_budget().await;
        remaining >= epsilon
    }

    /// Get query history
    pub async fn get_query_history(&self, limit: usize) -> Vec<PrivacyQuery> {
        let history = self.query_history.read().await;
        history.iter().rev().take(limit).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_laplace_noise() {
        let config = DifferentialPrivacyConfig::default();
        let dp = DifferentialPrivacyManager::new(config);
        
        let value = 100.0;
        let noisy = dp.add_laplace_noise(value, 1.0, 1.0);
        
        // Noisy value should be different but close
        assert!((noisy - value).abs() < 100.0);
    }

    #[tokio::test]
    async fn test_privatize_value() {
        let config = DifferentialPrivacyConfig::default();
        let dp = DifferentialPrivacyManager::new(config);
        
        let value = 50.0;
        let noisy = dp.privatize_value(value, "test_query").await.unwrap();
        
        assert!((noisy - value).abs() < 100.0);
    }

    #[tokio::test]
    async fn test_budget_tracking() {
        let config = DifferentialPrivacyConfig::default();
        let dp = DifferentialPrivacyManager::new(config);
        
        // Make multiple queries
        for _ in 0..10 {
            let _ = dp.privatize_value(10.0, "test").await;
        }
        
        let stats = dp.get_statistics().await;
        assert_eq!(stats.privatized_queries, 10);
        assert!(stats.total_budget_consumed > 0.0);
    }

    #[tokio::test]
    async fn test_budget_exhaustion() {
        let mut config = DifferentialPrivacyConfig::default();
        config.epsilon = 0.5; // Small budget
        config.min_epsilon = 0.1;
        
        let dp = DifferentialPrivacyManager::new(config);
        
        // Exhaust budget
        let mut exhausted = false;
        for _ in 0..20 {
            let result = dp.privatize_value(10.0, "test").await;
            if result.is_err() {
                exhausted = true;
                break;
            }
        }
        
        assert!(exhausted, "Budget should be exhausted");
    }
}