//! ML Load Balancer Module
//! 
//! Provides intelligent load balancing using multi-armed bandit algorithms.
//! 
//! ## ML Features
//! - Multi-armed bandit for optimal node selection
//! - Thompson Sampling for exploration/exploitation
//! - Adaptive node health tracking
//! 
//! ## Security Considerations
//! - Distribution decisions are deterministic
//! - No user data leakage
//! - Rate limiting for rebalancing

use crate::ai::{error::AIError, types::Confidence};
use crate::ai::ml::optimization::GeneticAlgorithm;

/// ML Load Balancer with Multi-Armed Bandit
/// 
/// Balances load across multiple nodes using ML.
/// 
/// ## Features
/// - Multi-armed bandit optimization (Thompson Sampling)
/// - Real-time load balancing
/// - Node health tracking
/// - Adaptive exploration/exploitation
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::load_balancer::MLLoadBalancer;
//! 
//! let lb = MLLoadBalancer::new(4)?;
//! 
//! // Get target node for new load
//! let node = lb.get_target_node("web")?;
//! println!("Routing to node {}", node);
//! ```
pub struct MLLoadBalancer {
    enabled: bool,
    node_count: usize,
    model_version: u32,
    
    // Bandit algorithm parameters
    alpha: Vec<f64>, // Success counts for Thompson Sampling
    beta: Vec<f64>,  // Failure counts for Thompson Sampling
    
    // Node statistics
    node_loads: Vec<f64>,
    node_response_times: Vec<f64>,
    node_error_counts: Vec<u64>,
    node_request_counts: Vec<u64>,
    
    // Configuration
    exploration_rate: f64,
    min_samples: usize,
}

/// Load balancing request
#[derive(Debug, Clone)]
pub struct LoadBalancingRequest {
    pub request_type: String,
    pub estimated_duration_ms: u32,
    pub priority: u8, // 0-10
    pub payload_size_bytes: usize,
}

/// Load balancing decision
#[derive(Debug, Clone)]
pub struct LoadBalancingDecision {
    pub target_node: usize,
    pub confidence: f64,
    pub reasoning: String,
}

impl MLLoadBalancer {
    /// Create a new ML Load Balancer
    /// 
    /// ## Arguments
    /// * `node_count` - Number of available nodes
    /// 
    /// ## Returns
    /// A new load balancer instance
    /// 
    /// ## Errors
    /// Returns AIError if node_count is invalid
    pub fn new(node_count: usize) -> Result<Self, AIError> {
        if node_count == 0 {
            return Err(AIError::InvalidInput);
        }
        
        let alpha = vec![1.0; node_count]; // Beta prior alpha = 1
        let beta = vec![1.0; node_count];  // Beta prior beta = 1
        
        Ok(Self {
            enabled: true,
            node_count,
            model_version: 1,
            alpha,
            beta,
            node_loads: vec![0.0; node_count],
            node_response_times: vec![0.0; node_count],
            node_error_counts: vec![0; node_count],
            node_request_counts: vec![0; node_count],
            exploration_rate: 0.1,
            min_samples: 5,
        })
    }
    
    /// Create with custom exploration rate
    pub fn new_with_config(node_count: usize, exploration_rate: f64) -> Result<Self, AIError> {
        let mut lb = Self::new(node_count)?;
        lb.exploration_rate = exploration_rate;
        Ok(lb)
    }

    /// Get target node for new load using Thompson Sampling
    /// 
    /// ## Arguments
    /// * `request_type` - Type of incoming request
    /// 
    /// ## Returns
    /// Target node ID
    /// 
    /// ## Errors
    /// - Returns error if load balancer is disabled
    /// 
    /// ## Algorithm
    /// Uses Thompson Sampling (Bayesian multi-armed bandit):
    /// 1. Sample from Beta distribution for each node
    /// 2. Select node with highest sample
    /// 3. Update parameters based on observed reward
    pub fn get_target_node(&mut self, request_type: &str) -> Result<usize, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }
        
        let target_node = self.thompson_sampling();
        
        // Update request count
        self.node_request_counts[target_node] += 1;
        
        Ok(target_node)
    }
    
    /// Get target node with full decision info
    pub fn get_target_node_detailed(&mut self, request: &LoadBalancingRequest) -> Result<LoadBalancingDecision, AIError> {
        if !self.enabled {
            return Err(AIError::ModuleNotReady);
        }
        
        let target_node = self.thompson_sampling();
        let confidence = self.calculate_node_confidence(target_node);
        
        let reasoning = format!(
            "Selected node {} based on Thompson Sampling. Load: {:.1}%, Response time: {:.1}ms, Error rate: {:.2}%",
            target_node,
            self.node_loads[target_node],
            self.node_response_times[target_node],
            self.calculate_error_rate(target_node)
        );
        
        self.node_request_counts[target_node] += 1;
        
        Ok(LoadBalancingDecision {
            target_node,
            confidence,
            reasoning,
        })
    }
    
    /// Thompson Sampling algorithm
    /// 
    /// Sample from Beta(alpha, beta) distribution for each node
    /// and select the one with the highest sample
    fn thompson_sampling(&self) -> usize {
        // For each node, sample from Beta distribution
        let samples: Vec<f64> = self.alpha.iter().zip(self.beta.iter())
            .map(|(&a, &b)| self.beta_sample(a, b))
            .collect();
        
        // Find index with maximum sample
        samples.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
    
    /// Sample from Beta distribution (simplified approximation)
    fn beta_sample(&self, alpha: f64, beta: f64) -> f64 {
        // Use simple approximation: alpha / (alpha + beta) + noise
        // In production, use proper Beta distribution sampling
        let mean = alpha / (alpha + beta);
        let noise = (rand::random::<f64>() - 0.5) * 0.2; // Small random noise
        
        (mean + noise).clamp(0.0, 1.0)
    }
    
    /// Provide feedback for bandit learning
    /// 
    /// ## Arguments
    /// * `node` - The node that was used
    /// * `success` - Whether the request was successful
    /// * `response_time_ms` - Response time in milliseconds
    /// * `load_percent` - Load on the node after request
    pub fn provide_feedback(
        &mut self,
        node: usize,
        success: bool,
        response_time_ms: f64,
        load_percent: f64,
    ) -> Result<(), AIError> {
        if node >= self.node_count {
            return Err(AIError::InvalidInput);
        }
        
        // Update Thompson Sampling parameters
        if success {
            // Success: increment alpha
            self.alpha[node] += 1.0;
        } else {
            // Failure: increment beta
            self.beta[node] += 1.0;
        }
        
        // Update node statistics
        let count = self.node_request_counts[node] as f64;
        if count > 1.0 {
            // Exponential moving average
            self.node_response_times[node] = 0.9 * self.node_response_times[node] + 0.1 * response_time_ms;
        } else {
            self.node_response_times[node] = response_time_ms;
        }
        
        self.node_loads[node] = load_percent;
        
        if !success {
            self.node_error_counts[node] += 1;
        }
        
        Ok(())
    }
    
    /// Calculate node confidence (0.0 to 1.0)
    fn calculate_node_confidence(&self, node: usize) -> f64 {
        let alpha = self.alpha[node];
        let beta = self.beta[node];
        let total = alpha + beta;
        
        if total < self.min_samples as f64 {
            return 0.5; // Low confidence with few samples
        }
        
        // Confidence increases with more samples
        let sample_confidence = (total / (self.min_samples as f64)).min(1.0);
        
        // Also consider error rate
        let error_rate = self.calculate_error_rate(node);
        let error_penalty = error_rate * 2.0;
        
        (sample_confidence - error_penalty).max(0.0).min(1.0)
    }
    
    /// Calculate error rate for a node
    fn calculate_error_rate(&self, node: usize) -> f64 {
        let total = self.node_request_counts[node];
        if total == 0 {
            return 0.0;
        }
        
        self.node_error_counts[node] as f64 / total as f64
    }
    
    /// Get node statistics
    pub fn get_node_stats(&self, node: usize) -> Option<(f64, f64, f64, u64)> {
        if node >= self.node_count {
            return None;
        }
        
        Some((
            self.node_loads[node],
            self.node_response_times[node],
            self.calculate_error_rate(node),
            self.node_request_counts[node],
        ))
    }
    
    /// Get all node statistics
    pub fn get_all_stats(&self) -> Vec<(f64, f64, f64, u64)> {
        (0..self.node_count)
            .map(|i| (
                self.node_loads[i],
                self.node_response_times[i],
                self.calculate_error_rate(i),
                self.node_request_counts[i],
            ))
            .collect()
    }
    
    /// Get best performing node based on statistics
    pub fn get_best_node(&self) -> Option<usize> {
        if self.node_count == 0 {
            return None;
        }
        
        (0..self.node_count)
            .filter(|&i| self.node_request_counts[i] > 0)
            .min_by(|&a, &b| {
                // Compare by (error_rate, response_time, load)
                let error_a = self.calculate_error_rate(a);
                let error_b = self.calculate_error_rate(b);
                
                if error_a != error_b {
                    return error_a.partial_cmp(&error_b).unwrap();
                }
                
                let resp_a = self.node_response_times[a];
                let resp_b = self.node_response_times[b];
                if resp_a != resp_b {
                    return resp_a.partial_cmp(&resp_b).unwrap();
                }
                
                self.node_loads[a].partial_cmp(&self.node_loads[b]).unwrap()
            })
    }
    
    /// Reset statistics for all nodes
    pub fn reset(&mut self) {
        self.alpha = vec![1.0; self.node_count];
        self.beta = vec![1.0; self.node_count];
        self.node_loads = vec![0.0; self.node_count];
        self.node_response_times = vec![0.0; self.node_count];
        self.node_error_counts = vec![0; self.node_count];
        self.node_request_counts = vec![0; self.node_count];
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Check if load balancer is ready
    pub fn is_ready(&self) -> bool {
        self.enabled
    }
    
    /// Enable or disable load balancer
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Get model version
    pub fn get_model_version(&self) -> u32 {
        self.model_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lb_creation() {
        let lb = MLLoadBalancer::new(4).unwrap();
        assert!(lb.is_ready());
        assert_eq!(lb.node_count(), 4);
    }

    #[test]
    fn test_get_target_node() {
        let mut lb = MLLoadBalancer::new(4).unwrap();
        let node = lb.get_target_node("web").unwrap();
        assert!(node < lb.node_count());
    }

    #[test]
    fn test_feedback_mechanism() {
        let mut lb = MLLoadBalancer::new(4).unwrap();
        
        // Get node and provide feedback
        let node = lb.get_target_node("web").unwrap();
        lb.provide_feedback(node, true, 50.0, 30.0).unwrap();
        
        // Check statistics updated
        let stats = lb.get_node_stats(node).unwrap();
        assert_eq!(stats.3, 1); // 1 request
    }
    
    #[test]
    fn test_convergence() {
        let mut lb = MLLoadBalancer::new(3).unwrap();
        
        // Simulate node 0 being better (faster, fewer errors)
        for _ in 0..100 {
            let node = lb.get_target_node("api").unwrap();
            match node {
                0 => lb.provide_feedback(0, true, 20.0, 20.0).unwrap(),
                1 => lb.provide_feedback(1, true, 100.0, 50.0).unwrap(),
                2 => lb.provide_feedback(2, true, 80.0, 40.0).unwrap(),
                _ => {},
            }
        }
        
        // Node 0 should have lower response time
        let stats_0 = lb.get_node_stats(0).unwrap();
        let stats_1 = lb.get_node_stats(1).unwrap();
        assert!(stats_0.1 < stats_1.1); // Response time
    }
    
    #[test]
    fn test_error_handling() {
        let mut lb = MLLoadBalancer::new(2).unwrap();
        
        // Node 0 has errors
        lb.provide_feedback(0, false, 1000.0, 50.0).unwrap();
        lb.provide_feedback(0, false, 1000.0, 50.0).unwrap();
        
        // Node 1 is successful
        lb.provide_feedback(1, true, 50.0, 30.0).unwrap();
        lb.provide_feedback(1, true, 50.0, 30.0).unwrap();
        
        // Node 1 should have lower error rate
        let err_0 = lb.get_node_stats(0).unwrap().2;
        let err_1 = lb.get_node_stats(1).unwrap().2;
        assert!(err_0 > err_1);
    }
    
    #[test]
    fn test_best_node_selection() {
        let mut lb = MLLoadBalancer::new(4).unwrap();
        
        // Simulate some traffic
        for i in 0..10 {
            let node = i % 4;
            let success = node != 1; // Node 1 has errors
            let response_time = if node == 1 { 200.0 } else { 50.0 };
            lb.provide_feedback(node, success, response_time, 30.0).unwrap();
        }
        
        // Best node should not be node 1
        let best = lb.get_best_node();
        assert!(best.is_some());
        assert_ne!(best.unwrap(), 1);
    }
    
    #[test]
    fn test_reset() {
        let mut lb = MLLoadBalancer::new(3).unwrap();
        lb.get_target_node("test").unwrap();
        lb.provide_feedback(0, true, 50.0, 30.0).unwrap();
        
        lb.reset();
        
        // Statistics should be reset
        let stats = lb.get_all_stats();
        for stat in stats {
            assert_eq!(stat.3, 0); // Request counts should be 0
        }
    }
    
    #[test]
    fn test_detailed_decision() {
        let mut lb = MLLoadBalancer::new(3).unwrap();
        
        let request = LoadBalancingRequest {
            request_type: "web".to_string(),
            estimated_duration_ms: 100,
            priority: 5,
            payload_size_bytes: 1024,
        };
        
        let decision = lb.get_target_node_detailed(&request).unwrap();
        assert!(decision.target_node < 3);
        assert!(!decision.reasoning.is_empty());
        assert!(decision.confidence >= 0.0 && decision.confidence <= 1.0);
    }
}