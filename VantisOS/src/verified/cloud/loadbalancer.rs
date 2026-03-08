/// Load Balancing Module
/// 
/// This module provides load balancing capabilities for cloud-native applications,
/// supporting multiple load balancing algorithms and health checking strategies.
/// 
/// Features:
/// - Multiple load balancing algorithms (Round Robin, Least Connections, IP Hash)
/// - Health checking and circuit breaking
/// - Session affinity
/// - Global and regional load balancing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{CloudError, CloudProvider};

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    /// Load balancer name
    pub name: String,
    /// Namespace
    pub namespace: String,
    /// Load balancer type
    #[serde(rename = "type")]
    pub type_field: LoadBalancerType,
    /// Algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check
    #[serde(rename = "healthCheck")]
    pub health_check: HealthCheck,
    /// Session affinity
    #[serde(rename = "sessionAffinity")]
    pub session_affinity: SessionAffinity,
    /// Backend servers
    pub backends: Vec<Backend>,
    /// Circuit breaker
    #[serde(rename = "circuitBreaker")]
    pub circuit_breaker: Option<CircuitBreaker>,
    /// Load balancer status
    pub status: LoadBalancerStatus,
}

impl LoadBalancer {
    /// Create a new load balancer
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: "default".to_string(),
            type_field: LoadBalancerType::Internal,
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_check: HealthCheck::default(),
            session_affinity: SessionAffinity::None,
            backends: Vec::new(),
            circuit_breaker: None,
            status: LoadBalancerStatus::Active,
        }
    }

    /// Set the namespace
    pub fn set_namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.namespace = namespace.into();
        self
    }

    /// Set load balancer type
    pub fn set_type(&mut self, lb_type: LoadBalancerType) -> &mut Self {
        self.type_field = lb_type;
        self
    }

    /// Set algorithm
    pub fn set_algorithm(&mut self, algorithm: LoadBalancingAlgorithm) -> &mut Self {
        self.algorithm = algorithm;
        self
    }

    /// Add a backend
    pub fn add_backend(&mut self, backend: Backend) -> &mut Self {
        self.backends.push(backend);
        self
    }

    /// Set session affinity
    pub fn set_session_affinity(&mut self, affinity: SessionAffinity) -> &mut Self {
        self.session_affinity = affinity;
        self
    }

    /// Set circuit breaker
    pub fn set_circuit_breaker(&mut self, circuit_breaker: CircuitBreaker) -> &mut Self {
        self.circuit_breaker = Some(circuit_breaker);
        self
    }

    /// Select a backend based on algorithm
    pub fn select_backend(&self, client_ip: Option<String>) -> Option<&Backend> {
        if self.backends.is_empty() {
            return None;
        }

        let healthy_backends: Vec<&Backend> = self.backends
            .iter()
            .filter(|b| b.healthy)
            .collect();

        if healthy_backends.is_empty() {
            return None;
        }

        match self.algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                let index = (self.status.total_requests as usize) % healthy_backends.len();
                healthy_backends.get(index).copied()
            }
            LoadBalancingAlgorithm::LeastConnections => {
                healthy_backends
                    .iter()
                    .min_by_key(|b| b.active_connections)
                    .copied()
            }
            LoadBalancingAlgorithm::IPHash => {
                if let Some(ip) = client_ip {
                    let hash = Self::hash_ip(&ip);
                    let index = (hash as usize) % healthy_backends.len();
                    healthy_backends.get(index).copied()
                } else {
                    healthy_backends.first().copied()
                }
            }
            LoadBalancingAlgorithm::WeightedRoundRobin => {
                // Simplified weighted selection
                let total_weight: i32 = healthy_backends
                    .iter()
                    .map(|b| b.weight.unwrap_or(1))
                    .sum();
                
                let mut target = (self.status.total_requests as i32) % total_weight;
                
                for backend in &healthy_backends {
                    let weight = backend.weight.unwrap_or(1);
                    if target < weight {
                        return Some(*backend);
                    }
                    target -= weight;
                }
                
                healthy_backends.first().copied()
            }
            LoadBalancingAlgorithm::Random => {
                let index = fastrand::usize(0..healthy_backends.len());
                healthy_backends.get(index).copied()
            }
        }
    }

    /// Hash IP address
    fn hash_ip(ip: &str) -> u64 {
        let mut hash: u64 = 5381;
        for byte in ip.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash
    }

    /// Increment request counter
    pub fn increment_requests(&mut self) {
        self.status.total_requests += 1;
    }

    /// Validate load balancer configuration
    pub fn validate(&self) -> Result<(), CloudError> {
        if self.name.is_empty() {
            return Err(CloudError::ValidationFailed("Load balancer name is required".to_string()));
        }
        if self.backends.is_empty() {
            return Err(CloudError::ValidationFailed("At least one backend is required".to_string()));
        }
        Ok(())
    }
}

/// Load balancer type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LoadBalancerType {
    Internal,
    External,
    Global,
    Regional,
}

/// Load balancing algorithm
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    IPHash,
    WeightedRoundRobin,
    Random,
}

/// Health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Enabled
    pub enabled: bool,
    /// Path
    pub path: String,
    /// Port
    pub port: i32,
    /// Protocol
    pub protocol: String,
    /// Interval in seconds
    pub interval: i32,
    /// Timeout in seconds
    pub timeout: i32,
    /// Unhealthy threshold
    #[serde(rename = "unhealthyThreshold")]
    pub unhealthy_threshold: i32,
    /// Healthy threshold
    #[serde(rename = "healthyThreshold")]
    pub healthy_threshold: i32,
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self {
            enabled: true,
            path: "/health".to_string(),
            port: 80,
            protocol: "HTTP".to_string(),
            interval: 10,
            timeout: 5,
            unhealthy_threshold: 3,
            healthy_threshold: 2,
        }
    }
}

/// Session affinity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SessionAffinity {
    None,
    ClientIP,
    Cookie,
}

/// Backend server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backend {
    /// Backend name
    pub name: String,
    /// Address (IP or hostname)
    pub address: String,
    /// Port
    pub port: i32,
    /// Weight
    pub weight: Option<i32>,
    /// Healthy status
    pub healthy: bool,
    /// Active connections
    #[serde(rename = "activeConnections")]
    pub active_connections: i32,
    /// Total requests
    #[serde(rename = "totalRequests")]
    pub total_requests: u64,
}

impl Backend {
    /// Create a new backend
    pub fn new(name: impl Into<String>, address: impl Into<String>, port: i32) -> Self {
        Self {
            name: name.into(),
            address: address.into(),
            port,
            weight: Some(1),
            healthy: true,
            active_connections: 0,
            total_requests: 0,
        }
    }

    /// Set weight
    pub fn set_weight(&mut self, weight: i32) -> &mut Self {
        self.weight = Some(weight);
        self
    }

    /// Mark as healthy/unhealthy
    pub fn set_healthy(&mut self, healthy: bool) -> &mut Self {
        self.healthy = healthy;
        self
    }

    /// Increment active connections
    pub fn increment_connections(&mut self) {
        self.active_connections += 1;
        self.total_requests += 1;
    }

    /// Decrement active connections
    pub fn decrement_connections(&mut self) {
        if self.active_connections > 0 {
            self.active_connections -= 1;
        }
    }
}

/// Circuit breaker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreaker {
    /// Enabled
    pub enabled: bool,
    /// Failure threshold
    #[serde(rename = "failureThreshold")]
    pub failure_threshold: i32,
    /// Success threshold
    #[serde(rename = "successThreshold")]
    pub success_threshold: i32,
    /// Timeout in seconds
    pub timeout: i32,
    /// Half-open max requests
    #[serde(rename = "halfOpenMaxRequests")]
    pub half_open_max_requests: i32,
    /// Current state
    pub state: CircuitBreakerState,
    /// Failure count
    #[serde(rename = "failureCount")]
    pub failure_count: i32,
    /// Success count
    #[serde(rename = "successCount")]
    pub success_count: i32,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new() -> Self {
        Self {
            enabled: true,
            failure_threshold: 5,
            success_threshold: 2,
            timeout: 30,
            half_open_max_requests: 1,
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
        }
    }

    /// Check if request is allowed
    pub fn allow_request(&self) -> bool {
        if !self.enabled {
            return true;
        }
        
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => false,
            CircuitBreakerState::HalfOpen => {
                self.success_count < self.half_open_max_requests
            }
        }
    }

    /// Record success
    pub fn record_success(&mut self) {
        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count = 0;
            }
            CircuitBreakerState::HalfOpen => {
                self.success_count += 1;
                if self.success_count >= self.success_threshold {
                    self.state = CircuitBreakerState::Closed;
                    self.success_count = 0;
                }
            }
            CircuitBreakerState::Open => {}
        }
    }

    /// Record failure
    pub fn record_failure(&mut self) {
        match self.state {
            CircuitBreakerState::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.failure_threshold {
                    self.state = CircuitBreakerState::Open;
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.state = CircuitBreakerState::Open;
                self.success_count = 0;
            }
            CircuitBreakerState::Open => {}
        }
    }

    /// Reset circuit breaker
    pub fn reset(&mut self) {
        self.state = CircuitBreakerState::Closed;
        self.failure_count = 0;
        self.success_count = 0;
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new()
    }
}

/// Circuit breaker state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// Load balancer status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancerStatus {
    /// Status
    pub status: LoadBalancerHealthStatus,
    /// Total requests
    #[serde(rename = "totalRequests")]
    pub total_requests: u64,
    /// Active connections
    #[serde(rename = "activeConnections")]
    pub active_connections: i32,
    /// Last updated
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
}

/// Load balancer health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LoadBalancerHealthStatus {
    Active,
    Draining,
    Maintenance,
    Error,
}

impl Default for LoadBalancerHealthStatus {
    fn default() -> Self {
        LoadBalancerHealthStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_balancer_creation() {
        let lb = LoadBalancer::new("test-lb");
        assert_eq!(lb.name, "test-lb");
        assert_eq!(lb.type_field, LoadBalancerType::Internal);
    }

    #[test]
    fn test_backend_creation() {
        let backend = Backend::new("backend1", "192.168.1.1", 8080);
        assert_eq!(backend.name, "backend1");
        assert_eq!(backend.address, "192.168.1.1");
        assert_eq!(backend.port, 8080);
    }

    #[test]
    fn test_round_robin_selection() {
        let mut lb = LoadBalancer::new("test-lb");
        lb.add_backend(Backend::new("b1", "192.168.1.1", 8080));
        lb.add_backend(Backend::new("b2", "192.168.1.2", 8080));
        
        let b1 = lb.select_backend(None);
        lb.increment_requests();
        let b2 = lb.select_backend(None);
        
        // Should round-robin between backends
        assert!(b1.is_some() && b2.is_some());
        assert_ne!(b1.unwrap().name, b2.unwrap().name);
    }

    #[test]
    fn test_least_connections_selection() {
        let mut lb = LoadBalancer::new("test-lb");
        lb.set_algorithm(LoadBalancingAlgorithm::LeastConnections);
        
        let mut b1 = Backend::new("b1", "192.168.1.1", 8080);
        b1.active_connections = 10;
        
        let mut b2 = Backend::new("b2", "192.168.1.2", 8080);
        b2.active_connections = 1;
        
        lb.add_backend(b1);
        lb.add_backend(b2);
        
        let selected = lb.select_backend(None);
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().name, "b2");
    }

    #[test]
    fn test_circuit_breaker() {
        let mut cb = CircuitBreaker::new();
        
        assert!(cb.allow_request());
        
        for _ in 0..5 {
            cb.record_failure();
        }
        
        assert!(!cb.allow_request());
        assert_eq!(cb.state, CircuitBreakerState::Open);
    }

    #[test]
    fn test_load_balancer_validation() {
        let lb = LoadBalancer::new("test-lb");
        assert!(lb.validate().is_err()); // No backends
        
        let mut lb = LoadBalancer::new("test-lb");
        lb.add_backend(Backend::new("b1", "192.168.1.1", 8080));
        assert!(lb.validate().is_ok());
    }
}