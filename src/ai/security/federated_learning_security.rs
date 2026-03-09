//! Federated Learning Security Module
//! 
//! Security mechanisms for federated learning and distributed AI training.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};

/// Federated learning security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedSecurityConfig {
    /// Enable secure aggregation
    pub enable_secure_aggregation: bool,
    /// Enable client authentication
    pub enable_client_auth: bool,
    /// Enable differential privacy
    pub enable_differential_privacy: bool,
    /// Minimum clients for aggregation
    pub min_clients_for_aggregation: usize,
    /// Maximum clients per round
    pub max_clients_per_round: usize,
    /// Anomaly detection threshold
    pub anomaly_threshold: f64,
}

impl Default for FederatedSecurityConfig {
    fn default() -> Self {
        Self {
            enable_secure_aggregation: true,
            enable_client_auth: true,
            enable_differential_privacy: true,
            min_clients_for_aggregation: 10,
            max_clients_per_round: 100,
            anomaly_threshold: 0.85,
        }
    }
}

/// Client credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentials {
    /// Client ID
    pub client_id: String,
    /// Authentication token
    pub auth_token: String,
    /// Public key
    pub public_key: Vec<u8>,
    /// Reputation score
    pub reputation_score: f64,
    /// Last seen timestamp
    pub last_seen: u64,
}

/// Model update from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientUpdate {
    /// Client ID
    pub client_id: String,
    /// Model update data
    pub update_data: Vec<f64>,
    /// Number of samples used
    pub num_samples: u64,
    /// Update timestamp
    pub timestamp: u64,
    /// Update signature
    pub signature: Vec<u8>,
}

/// Aggregation round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationRound {
    /// Round number
    pub round_number: u64,
    /// Participating clients
    pub participating_clients: Vec<String>,
    /// Aggregated model
    pub aggregated_model: Vec<f64>,
    /// Security metrics
    pub security_metrics: SecurityMetrics,
    /// Round timestamp
    pub timestamp: u64,
}

/// Security metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    /// Anomalous clients detected
    pub anomalous_clients: Vec<String>,
    /// Anomaly scores
    pub anomaly_scores: HashMap<String, f64>,
    /// Privacy loss
    pub privacy_loss: f64,
    /// Aggregation time in seconds
    pub aggregation_time_secs: f64,
}

/// Secure aggregation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureAggregationResult {
    /// Aggregated model
    pub aggregated_model: Vec<f64>,
    /// Excluded clients
    pub excluded_clients: Vec<String>,
    /// Privacy budget used
    pub privacy_budget_used: f64,
    /// Security score
    pub security_score: f64,
}

/// Anomaly detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetectionResult {
    /// Is anomalous
    pub is_anomalous: bool,
    /// Anomaly score
    pub anomaly_score: f64,
    /// Anomaly type
    pub anomaly_type: AnomalyType,
    /// Reason
    pub reason: String,
}

/// Anomaly type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    /// Model poisoning
    ModelPoisoning,
    /// Data poisoning
    DataPoisoning,
    /// Byzantine behavior
    Byzantine,
    /// Free-riding
    FreeRiding,
    /// Replay attack
    ReplayAttack,
    /// Sybil attack
    SybilAttack,
    /// Unknown
    Unknown,
}

/// Federated learning security manager
pub struct FederatedSecurityManager {
    config: FederatedSecurityConfig,
    registered_clients: Arc<RwLock<HashMap<String, ClientCredentials>>>,
    round_history: Arc<RwLock<Vec<AggregationRound>>>,
    statistics: Arc<RwLock<FederatedLearningStats>>,
}

/// Federated learning statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedLearningStats {
    /// Total rounds completed
    pub total_rounds: u64,
    /// Total clients participated
    pub total_clients_participated: u64,
    /// Total anomalous updates detected
    pub anomalous_updates_detected: u64,
    /// Total models aggregated
    pub total_models_aggregated: u64,
    /// Average aggregation time in seconds
    pub avg_aggregation_time_secs: f64,
    /// Average clients per round
    pub avg_clients_per_round: f64,
}

impl FederatedSecurityManager {
    /// Create a new federated learning security manager
    pub fn new(config: FederatedSecurityConfig) -> Self {
        Self {
            config,
            registered_clients: Arc::new(RwLock::new(HashMap::new())),
            round_history: Arc::new(RwLock::new(Vec::new())),
            statistics: Arc::new(RwLock::new(FederatedLearningStats {
                total_rounds: 0,
                total_clients_participated: 0,
                anomalous_updates_detected: 0,
                total_models_aggregated: 0,
                avg_aggregation_time_secs: 0.0,
                avg_clients_per_round: 0.0,
            })),
        }
    }

    /// Register client
    pub async fn register_client(&self, credentials: ClientCredentials) -> Result<(), Box<dyn std::error::Error>> {
        let mut clients = self.registered_clients.write().await;
        clients.insert(credentials.client_id.clone(), credentials);
        Ok(())
    }

    /// Authenticate client
    pub async fn authenticate_client(&self, client_id: &str, auth_token: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let clients = self.registered_clients.read().await;
        
        if let Some(client) = clients.get(client_id) {
            Ok(client.auth_token == auth_token && client.reputation_score >= 0.5)
        } else {
            Ok(false)
        }
    }

    /// Detect anomaly in client update
    pub async fn detect_anomaly(&self, update: &ClientUpdate) -> Result<AnomalyDetectionResult, Box<dyn std::error::Error>> {
        let clients = self.registered_clients.read().await;
        let client = clients.get(&update.client_id);
        
        let mut anomaly_score = 0.0;
        let mut anomaly_type = AnomalyType::Unknown;
        let mut reason = String::new();

        // Check if client is registered
        if client.is_none() {
            return Ok(AnomalyDetectionResult {
                is_anomalous: true,
                anomaly_score: 1.0,
                anomaly_type: AnomalyType::SybilAttack,
                reason: "Unregistered client".to_string(),
            });
        }

        let client = client.unwrap();

        // Check reputation
        if client.reputation_score < 0.5 {
            anomaly_score = anomaly_score.max(0.8);
            anomaly_type = AnomalyType::Byzantine;
            reason.push_str("Low reputation score; ");
        }

        // Check update size
        if update.update_data.is_empty() || update.update_data.len() > 1_000_000 {
            anomaly_score = anomaly_score.max(0.6);
            anomaly_type = AnomalyType::DataPoisoning;
            reason.push_str("Abnormal update size; ");
        }

        // Check sample count
        if update.num_samples == 0 {
            anomaly_score = anomaly_score.max(0.7);
            anomaly_type = AnomalyType::FreeRiding;
            reason.push_str("Zero samples reported; ");
        }

        // Check for statistical anomalies
        let update_mean = update.update_data.iter().sum::<f64>() / update.update_data.len() as f64;
        let update_std = update.update_data.iter()
            .map(|x| (x - update_mean).powi(2))
            .sum::<f64>()
            .sqrt() / update.update_data.len() as f64;

        if update_std > 10.0 {
            anomaly_score = anomaly_score.max(0.5);
            reason.push_str("High update variance; ");
        }

        let is_anomalous = anomaly_score > self.config.anomaly_threshold;

        Ok(AnomalyDetectionResult {
            is_anomalous,
            anomaly_score,
            anomaly_type,
            reason,
        })
    }

    /// Secure aggregation
    pub async fn secure_aggregate(&self, updates: Vec<ClientUpdate>) -> Result<SecureAggregationResult, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        let mut stats = self.statistics.write().await;

        // Filter anomalous updates
        let mut valid_updates = Vec::new();
        let mut excluded_clients = Vec::new();
        let mut anomaly_scores = HashMap::new();

        for update in updates {
            let anomaly_result = self.detect_anomaly(&update).await?;
            
            if anomaly_result.is_anomalous {
                excluded_clients.push(update.client_id.clone());
                anomaly_scores.insert(update.client_id.clone(), anomaly_result.anomaly_score);
                stats.anomalous_updates_detected += 1;
            } else {
                valid_updates.push(update);
            }
        }

        // Check minimum clients
        if valid_updates.len() < self.config.min_clients_for_aggregation {
            return Err(format!(
                "Insufficient valid clients: {} (minimum {})",
                valid_updates.len(),
                self.config.min_clients_for_aggregation
            ).into());
        }

        // Weighted aggregation based on sample count
        let total_samples: u64 = valid_updates.iter().map(|u| u.num_samples).sum();
        let mut aggregated_model = vec![0.0; valid_updates[0].update_data.len()];

        for update in &valid_updates {
            let weight = update.num_samples as f64 / total_samples as f64;
            for (i, val) in update.update_data.iter().enumerate() {
                aggregated_model[i] += val * weight;
            }
        }

        // Apply differential privacy if enabled
        if self.config.enable_differential_privacy {
            aggregated_model = self.apply_differential_privacy(aggregated_model).await;
        }

        let elapsed = start.elapsed().as_secs_f64();
        let security_score = 1.0 - (excluded_clients.len() as f64 / valid_updates.len() as f64 + excluded_clients.len() as f64);

        // Update statistics
        stats.total_rounds += 1;
        stats.total_clients_participated += valid_updates.len() as u64;
        stats.total_models_aggregated += valid_updates.len() as u64;
        stats.avg_aggregation_time_secs = (stats.avg_aggregation_time_secs * (stats.total_rounds - 1) as f64 + elapsed) / stats.total_rounds as f64;
        stats.avg_clients_per_round = (stats.avg_clients_per_round * (stats.total_rounds - 1) as f64 + valid_updates.len() as f64) / stats.total_rounds as f64;

        Ok(SecureAggregationResult {
            aggregated_model,
            excluded_clients,
            privacy_budget_used: if self.config.enable_differential_privacy { 0.1 } else { 0.0 },
            security_score,
        })
    }

    /// Apply differential privacy
    async fn apply_differential_privacy(&self, model: Vec<f64>) -> Vec<f64> {
        // Simplified Laplace noise addition
        let epsilon = 1.0;
        let sensitivity = 1.0;
        let scale = sensitivity / epsilon;
        
        model.iter()
            .map(|_| {
                let u: f64 = rand::random();
                let noise = -scale * (1.0 - u).ln();
                // In production, use proper Laplace distribution
                noise
            })
            .collect()
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> Result<FederatedLearningStats, Box<dyn std::error::Error>> {
        let stats = self.statistics.read().await;
        Ok(stats.clone())
    }

    /// Get round history
    pub async fn get_round_history(&self) -> Result<Vec<AggregationRound>, Box<dyn std::error::Error>> {
        let history = self.round_history.read().await;
        Ok(history.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_registration() {
        let manager = FederatedSecurityManager::new(FederatedSecurityConfig::default());
        
        let credentials = ClientCredentials {
            client_id: "client1".to_string(),
            auth_token: "token123".to_string(),
            public_key: vec![1, 2, 3],
            reputation_score: 0.9,
            last_seen: 0,
        };
        
        manager.register_client(credentials).await.unwrap();
        
        let authenticated = manager.authenticate_client("client1", "token123").await.unwrap();
        assert!(authenticated);
    }

    #[tokio::test]
    async fn test_secure_aggregation() {
        let manager = FederatedSecurityManager::new(FederatedSecurityConfig::default());
        
        let updates = vec![
            ClientUpdate {
                client_id: "client1".to_string(),
                update_data: vec![0.1, 0.2, 0.3],
                num_samples: 100,
                timestamp: 0,
                signature: vec![],
            },
            ClientUpdate {
                client_id: "client2".to_string(),
                update_data: vec![0.2, 0.3, 0.4],
                num_samples: 200,
                timestamp: 0,
                signature: vec![],
            },
        ];
        
        let result = manager.secure_aggregate(updates).await.unwrap();
        assert!(!result.aggregated_model.is_empty());
    }
}