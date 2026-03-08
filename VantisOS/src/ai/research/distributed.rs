// Distributed Systems for VantisOS AI Research
// Federated learning, distributed optimization, and fault tolerance

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

/// Synchronization strategy for distributed training
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SyncStrategy {
    /// Synchronous - wait for all workers
    Sync,
    /// Asynchronous - don't wait
    Async,
    /// Hybrid - bounded staleness
    Hybrid,
}

/// Node configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node ID
    pub id: String,
    /// Node address
    pub address: String,
    /// Port
    pub port: u16,
    /// Number of GPUs
    pub num_gpus: usize,
    /// Memory capacity (GB)
    pub memory: usize,
    /// Node role
    pub role: NodeRole,
}

/// Node role in distributed system
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeRole {
    /// Parameter server
    ParameterServer,
    /// Worker node
    Worker,
    /// Coordinator node
    Coordinator,
}

/// Node manager for distributed systems
#[derive(Clone, Debug)]
pub struct NodeManager {
    nodes: HashMap<String, NodeConfig>,
    status: HashMap<String, NodeStatus>,
}

/// Node status
#[derive(Clone, Debug)]
pub struct NodeStatus {
    pub is_alive: bool,
    pub load: f64,
    pub last_heartbeat: u64,
    pub current_task: Option<String>,
}

impl NodeManager {
    /// Create a new node manager
    pub fn new() -> Self {
        NodeManager {
            nodes: HashMap::new(),
            status: HashMap::new(),
        }
    }

    /// Add a node
    pub fn add_node(&mut self, config: NodeConfig) -> Result<(), String> {
        if self.nodes.contains_key(&config.id) {
            return Err(format!("Node {} already exists", config.id));
        }

        self.nodes.insert(config.id.clone(), config);
        self.status.insert(config.id, NodeStatus {
            is_alive: true,
            load: 0.0,
            last_heartbeat: 0,
            current_task: None,
        });

        Ok(())
    }

    /// Remove a node
    pub fn remove_node(&mut self, id: &str) -> Result<(), String> {
        if self.nodes.remove(id).is_none() {
            return Err(format!("Node {} not found", id));
        }
        self.status.remove(id);
        Ok(())
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get all nodes
    pub fn list_nodes(&self) -> Vec<&NodeConfig> {
        self.nodes.values().collect()
    }

    /// Get workers
    pub fn get_workers(&self) -> Vec<&NodeConfig> {
        self.nodes.values()
            .filter(|n| n.role == NodeRole::Worker)
            .collect()
    }

    /// Select nodes for a task
    pub fn select_nodes(&self, count: usize) -> Vec<&NodeConfig> {
        self.nodes.values()
            .filter(|n| n.role == NodeRole::Worker)
            .take(count)
            .collect()
    }

    /// Update node status
    pub fn update_status(&mut self, id: &str, status: NodeStatus) -> Result<(), String> {
        if !self.nodes.contains_key(id) {
            return Err(format!("Node {} not found", id));
        }
        self.status.insert(id.to_string(), status);
        Ok(())
    }

    /// Get node status
    pub fn get_status(&self, id: &str) -> Option<&NodeStatus> {
        self.status.get(id)
    }

    /// Check node health
    pub fn check_health(&self, id: &str) -> bool {
        self.status.get(id).map(|s| s.is_alive).unwrap_or(false)
    }

    /// Balance load across nodes
    pub fn balance_load(&mut self) -> Result<(), String> {
        // Simple round-robin load balancing
        let nodes: Vec<_> = self.nodes.keys().cloned().collect();
        let num_nodes = nodes.len();
        
        if num_nodes == 0 {
            return Ok(());
        }

        for (i, id) in nodes.iter().enumerate() {
            if let Some(status) = self.status.get_mut(id) {
                status.load = 1.0 / num_nodes as f64;
            }
        }

        Ok(())
    }
}

impl Default for NodeManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Federated learning manager
#[derive(Clone, Debug)]
pub struct FederatedLearning {
    /// Number of clients
    num_clients: usize,
    /// Learning rate
    learning_rate: f64,
    /// Number of local epochs
    local_epochs: usize,
    /// Client fraction
    client_fraction: f64,
    /// Aggregation strategy
    aggregation: AggregationStrategy,
    /// Privacy settings
    privacy: PrivacySettings,
    /// Client weights
    client_weights: HashMap<String, f64>,
    /// Round counter
    current_round: usize,
}

/// Aggregation strategy for federated averaging
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AggregationStrategy {
    /// Simple average
    Average,
    /// Weighted by data size
    Weighted,
    /// Median
    Median,
    /// Trimmed mean
    TrimmedMean { trim_ratio: f64 },
}

/// Privacy settings for federated learning
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Enable differential privacy
    pub differential_privacy: bool,
    /// Privacy budget (epsilon)
    pub epsilon: f64,
    /// Delta parameter
    pub delta: f64,
    /// Clipping norm
    pub clipping_norm: f64,
    /// Enable secure aggregation
    pub secure_aggregation: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        PrivacySettings {
            differential_privacy: false,
            epsilon: 1.0,
            delta: 1e-5,
            clipping_norm: 1.0,
            secure_aggregation: false,
        }
    }
}

impl FederatedLearning {
    /// Create a new federated learning instance
    pub fn new(num_clients: usize, learning_rate: f64) -> Self {
        FederatedLearning {
            num_clients,
            learning_rate,
            local_epochs: 1,
            client_fraction: 1.0,
            aggregation: AggregationStrategy::Average,
            privacy: PrivacySettings::default(),
            client_weights: HashMap::new(),
            current_round: 0,
        }
    }

    /// Set local epochs
    pub fn local_epochs(mut self, epochs: usize) -> Self {
        self.local_epochs = epochs;
        self
    }

    /// Set client fraction
    pub fn client_fraction(mut self, fraction: f64) -> Self {
        self.client_fraction = fraction;
        self
    }

    /// Set aggregation strategy
    pub fn aggregation(mut self, strategy: AggregationStrategy) -> Self {
        self.aggregation = strategy;
        self
    }

    /// Enable differential privacy
    pub fn with_differential_privacy(mut self, epsilon: f64, delta: f64) -> Self {
        self.privacy.differential_privacy = true;
        self.privacy.epsilon = epsilon;
        self.privacy.delta = delta;
        self
    }

    /// Enable secure aggregation
    pub fn with_secure_aggregation(mut self) -> Self {
        self.privacy.secure_aggregation = true;
        self
    }

    /// Get number of clients
    pub fn num_clients(&self) -> usize {
        self.num_clients
    }

    /// Get current round
    pub fn current_round(&self) -> usize {
        self.current_round
    }

    /// Select clients for current round
    pub fn select_clients(&self) -> Vec<usize> {
        let num_selected = (self.num_clients as f64 * self.client_fraction).ceil() as usize;
        (0..self.num_clients).take(num_selected).collect()
    }

    /// Register a client
    pub fn register_client(&mut self, id: &str, weight: f64) {
        self.client_weights.insert(id.to_string(), weight);
    }

    /// Aggregate gradients from clients
    pub fn aggregate(&self, gradients: &[Vec<f64>]) -> Vec<f64> {
        if gradients.is_empty() {
            return vec![];
        }

        let size = gradients[0].len();
        
        match &self.aggregation {
            AggregationStrategy::Average => {
                let mut aggregated = vec![0.0; size];
                for grad in gradients {
                    for (i, &g) in grad.iter().enumerate() {
                        aggregated[i] += g;
                    }
                }
                for a in aggregated.iter_mut() {
                    *a /= gradients.len() as f64;
                }
                aggregated
            }
            AggregationStrategy::Weighted => {
                let mut aggregated = vec![0.0; size];
                let total_weight: f64 = gradients.len() as f64;
                
                for grad in gradients {
                    for (i, &g) in grad.iter().enumerate() {
                        aggregated[i] += g / total_weight;
                    }
                }
                aggregated
            }
            AggregationStrategy::Median => {
                let mut aggregated = vec![0.0; size];
                for i in 0..size {
                    let mut values: Vec<f64> = gradients.iter().map(|g| g[i]).collect();
                    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    aggregated[i] = values[values.len() / 2];
                }
                aggregated
            }
            AggregationStrategy::TrimmedMean { trim_ratio } => {
                let mut aggregated = vec![0.0; size];
                let trim_count = (gradients.len() as f64 * trim_ratio) as usize;
                
                for i in 0..size {
                    let mut values: Vec<f64> = gradients.iter().map(|g| g[i]).collect();
                    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let trimmed = &values[trim_count..values.len() - trim_count];
                    aggregated[i] = trimmed.iter().sum::<f64>() / trimmed.len() as f64;
                }
                aggregated
            }
        }
    }

    /// Run one federated round
    pub fn run_round(&mut self, client_gradients: &[Vec<f64>]) -> Result<Vec<f64>, String> {
        if client_gradients.is_empty() {
            return Err("No client gradients provided".to_string());
        }

        // Apply differential privacy if enabled
        let gradients = if self.privacy.differential_privacy {
            self.apply_differential_privacy(client_gradients)
        } else {
            client_gradients.to_vec()
        };

        // Aggregate gradients
        let aggregated = self.aggregate(&gradients);

        self.current_round += 1;
        Ok(aggregated)
    }

    /// Apply differential privacy
    fn apply_differential_privacy(&self, gradients: &[Vec<f64>]) -> Vec<Vec<f64>> {
        use rand::Rng;

        gradients.iter().map(|grad| {
            // Clip gradients
            let norm: f64 = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            let scale = if norm > self.privacy.clipping_norm {
                self.privacy.clipping_norm / norm
            } else {
                1.0
            };

            // Add noise
            let mut rng = rand::thread_rng();
            let noise_scale = self.privacy.epsilon.sqrt() / self.privacy.clipping_norm;
            
            grad.iter().map(|&g| {
                let noise = rng.gen::<f64>() * noise_scale;
                g * scale + noise
            }).collect()
        }).collect()
    }
}

/// Distributed optimizer
#[derive(Clone, Debug)]
pub struct DistributedOptimizer {
    /// Base optimizer type
    optimizer_type: String,
    /// Learning rate
    learning_rate: f64,
    /// Number of workers
    num_workers: usize,
    /// Synchronization strategy
    sync_strategy: SyncStrategy,
    /// Staleness threshold for async
    staleness_threshold: usize,
    /// Compression enabled
    compression: bool,
    /// Compression ratio
    compression_ratio: f64,
}

impl DistributedOptimizer {
    /// Create a new distributed optimizer
    pub fn new(optimizer_type: &str, learning_rate: f64, num_workers: usize) -> Self {
        DistributedOptimizer {
            optimizer_type: optimizer_type.to_string(),
            learning_rate,
            num_workers,
            sync_strategy: SyncStrategy::Sync,
            staleness_threshold: 10,
            compression: false,
            compression_ratio: 0.1,
        }
    }

    /// Set sync strategy
    pub fn sync_strategy(mut self, strategy: SyncStrategy) -> Self {
        self.sync_strategy = strategy;
        self
    }

    /// Enable compression
    pub fn with_compression(mut self, ratio: f64) -> Self {
        self.compression = true;
        self.compression_ratio = ratio;
        self
    }

    /// Get learning rate
    pub fn learning_rate(&self) -> f64 {
        self.learning_rate
    }

    /// Get number of workers
    pub fn num_workers(&self) -> usize {
        self.num_workers
    }

    /// Synchronize gradients across workers
    pub fn synchronize(&self, gradients: &[Vec<f64>]) -> Vec<f64> {
        if gradients.is_empty() {
            return vec![];
        }

        match self.sync_strategy {
            SyncStrategy::Sync => self.all_reduce(gradients),
            SyncStrategy::Async => self.stale_aggregate(gradients),
            SyncStrategy::Hybrid => self.bounded_staleness(gradients),
        }
    }

    /// All-reduce operation
    fn all_reduce(&self, gradients: &[Vec<f64>]) -> Vec<f64> {
        if gradients.is_empty() {
            return vec![];
        }

        let size = gradients[0].len();
        let mut result = vec![0.0; size];

        for grad in gradients {
            for (i, &g) in grad.iter().enumerate() {
                result[i] += g;
            }
        }

        for r in result.iter_mut() {
            *r /= gradients.len() as f64;
        }

        result
    }

    /// Stale aggregation for async
    fn stale_aggregate(&self, gradients: &[Vec<f64>]) -> Vec<f64> {
        // Use the most recent gradient
        gradients.last().cloned().unwrap_or_default()
    }

    /// Bounded staleness aggregation
    fn bounded_staleness(&self, gradients: &[Vec<f64>]) -> Vec<f64> {
        // Average of recent gradients
        let start = if gradients.len() > self.staleness_threshold {
            gradients.len() - self.staleness_threshold
        } else {
            0
        };

        let recent = &gradients[start..];
        self.all_reduce(recent)
    }

    /// Compress gradients
    pub fn compress(&self, gradient: &[f64]) -> Vec<(usize, f64)> {
        if !self.compression {
            return gradient.iter().enumerate().map(|(i, &g)| (i, g)).collect();
        }

        // Top-k compression
        let mut indexed: Vec<_> = gradient.iter().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        let k = (gradient.len() as f64 * self.compression_ratio) as usize;
        indexed.into_iter().take(k).map(|(i, &g)| (i, g)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_manager() {
        let mut manager = NodeManager::new();
        
        let config = NodeConfig {
            id: "node1".to_string(),
            address: "127.0.0.1".to_string(),
            port: 8080,
            num_gpus: 4,
            memory: 32,
            role: NodeRole::Worker,
        };

        assert!(manager.add_node(config).is_ok());
        assert_eq!(manager.node_count(), 1);
    }

    #[test]
    fn test_federated_learning() {
        let fl = FederatedLearning::new(10, 0.01)
            .local_epochs(5)
            .client_fraction(0.5);

        assert_eq!(fl.num_clients(), 10);
        assert_eq!(fl.local_epochs, 5);
    }

    #[test]
    fn test_client_selection() {
        let fl = FederatedLearning::new(10, 0.01).client_fraction(0.5);
        let selected = fl.select_clients();
        assert_eq!(selected.len(), 5);
    }

    #[test]
    fn test_gradient_aggregation() {
        let fl = FederatedLearning::new(3, 0.01);
        let gradients = vec![
            vec![1.0, 2.0, 3.0],
            vec![2.0, 3.0, 4.0],
            vec![3.0, 4.0, 5.0],
        ];

        let aggregated = fl.aggregate(&gradients);
        assert!((aggregated[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_distributed_optimizer() {
        let opt = DistributedOptimizer::new("adam", 0.001, 4)
            .sync_strategy(SyncStrategy::Sync);

        assert_eq!(opt.learning_rate(), 0.001);
        assert_eq!(opt.num_workers(), 4);
    }

    #[test]
    fn test_gradient_synchronization() {
        let opt = DistributedOptimizer::new("adam", 0.001, 2);
        let gradients = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ];

        let sync = opt.synchronize(&gradients);
        assert!((sync[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_differential_privacy() {
        let fl = FederatedLearning::new(3, 0.01)
            .with_differential_privacy(1.0, 1e-5);

        assert!(fl.privacy.differential_privacy);
        assert_eq!(fl.privacy.epsilon, 1.0);
    }

    #[test]
    fn test_gradient_compression() {
        let opt = DistributedOptimizer::new("adam", 0.001, 2)
            .with_compression(0.5);

        let gradient = vec![1.0, 2.0, 3.0, 4.0];
        let compressed = opt.compress(&gradient);
        assert_eq!(compressed.len(), 2); // 50% compression
    }
}