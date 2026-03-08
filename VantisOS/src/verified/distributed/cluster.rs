/// Cluster Management Module
/// 
/// This module provides cluster management capabilities for VantisOS,
/// supporting node management, service discovery, and cluster operations.
/// 
/// Features:
/// - Node lifecycle management
/// - Service discovery
/// - Cluster membership
/// - Load distribution
/// - Resource scheduling

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{DistributedError, NodeStatus, QuorumConfig};

/// Cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    /// Cluster name
    pub name: String,
    /// Cluster nodes
    pub nodes: Vec<Node>,
    /// Quorum configuration
    pub quorum: QuorumConfig,
    /// Service discovery
    #[serde(rename = "serviceDiscovery")]
    pub service_discovery: ServiceDiscovery,
    /// Cluster status
    pub status: ClusterStatus,
}

impl Cluster {
    /// Create a new cluster
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: Vec::new(),
            quorum: QuorumConfig::default(),
            service_discovery: ServiceDiscovery::default(),
            status: ClusterStatus::default(),
        }
    }

    /// Add a node
    pub fn add_node(&mut self, node: Node) -> &mut Self {
        self.nodes.push(node);
        self
    }

    /// Remove a node
    pub fn remove_node(&mut self, node_id: &str) -> &mut Self {
        self.nodes.retain(|n| n.id != node_id);
        self
    }

    /// Get node by ID
    pub fn get_node(&self, node_id: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == node_id)
    }

    /// Get mutable node by ID
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == node_id)
    }

    /// Get leader node
    pub fn get_leader(&self) -> Option<&Node> {
        self.nodes.iter().find(|n| n.is_leader)
    }

    /// Get healthy nodes
    pub fn get_healthy_nodes(&self) -> Vec<&Node> {
        self.nodes.iter().filter(|n| n.status == NodeStatus::Ready).collect()
    }

    /// Get node count
    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get healthy node count
    pub fn get_healthy_node_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.status == NodeStatus::Ready).count()
    }

    /// Check if quorum is maintained
    pub fn has_quorum(&self) -> bool {
        let healthy_count = self.get_healthy_node_count();
        let quorum_size = self.quorum.size as usize;
        healthy_count >= quorum_size
    }

    /// Validate cluster configuration
    pub fn validate(&self) -> Result<(), DistributedError> {
        if self.name.is_empty() {
            return Err(DistributedError::ClusterError("Cluster name is required".to_string()));
        }
        if self.nodes.is_empty() {
            return Err(DistributedError::ClusterError("Cluster must have at least one node".to_string()));
        }
        Ok(())
    }
}

/// Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Node ID
    pub id: String,
    /// Node name
    pub name: String,
    /// Node address
    pub address: String,
    /// Node port
    pub port: i32,
    /// Node status
    pub status: NodeStatus,
    /// Is leader
    #[serde(rename = "isLeader")]
    pub is_leader: bool,
    /// Labels
    pub labels: HashMap<String, String>,
    /// Resources
    pub resources: NodeResources,
    /// Last heartbeat
    #[serde(rename = "lastHeartbeat")]
    pub last_heartbeat: Option<String>,
}

impl Node {
    /// Create a new node
    pub fn new(id: impl Into<String>, name: impl Into<String>, address: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            address: address.into(),
            port: 8080,
            status: NodeStatus::Ready,
            is_leader: false,
            labels: HashMap::new(),
            resources: NodeResources::default(),
            last_heartbeat: None,
        }
    }

    /// Set port
    pub fn set_port(&mut self, port: i32) -> &mut Self {
        self.port = port;
        self
    }

    /// Set status
    pub fn set_status(&mut self, status: NodeStatus) -> &mut Self {
        self.status = status;
        self
    }

    /// Set as leader
    pub fn set_leader(&mut self, is_leader: bool) -> &mut Self {
        self.is_leader = is_leader;
        self
    }

    /// Add label
    pub fn add_label(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.labels.insert(key.into(), value.into());
        self
    }

    /// Get connection string
    pub fn get_connection_string(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

/// Node resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResources {
    /// CPU cores
    #[serde(rename = "cpuCores")]
    pub cpu_cores: i32,
    /// Memory in bytes
    pub memory: u64,
    /// Disk in bytes
    pub disk: u64,
    /// CPU used
    #[serde(rename = "cpuUsed")]
    pub cpu_used: f64,
    /// Memory used
    #[serde(rename = "memoryUsed")]
    pub memory_used: u64,
    /// Disk used
    #[serde(rename = "diskUsed")]
    pub disk_used: u64,
}

impl Default for NodeResources {
    fn default() -> Self {
        Self {
            cpu_cores: 4,
            memory: 16 * 1024 * 1024 * 1024, // 16 GB
            disk: 100 * 1024 * 1024 * 1024,  // 100 GB
            cpu_used: 0.0,
            memory_used: 0,
            disk_used: 0,
        }
    }
}

impl NodeResources {
    /// Get CPU utilization percentage
    pub fn get_cpu_utilization(&self) -> f64 {
        if self.cpu_cores == 0 {
            return 0.0;
        }
        (self.cpu_used / self.cpu_cores as f64) * 100.0
    }

    /// Get memory utilization percentage
    pub fn get_memory_utilization(&self) -> f64 {
        if self.memory == 0 {
            return 0.0;
        }
        (self.memory_used as f64 / self.memory as f64) * 100.0
    }

    /// Get disk utilization percentage
    pub fn get_disk_utilization(&self) -> f64 {
        if self.disk == 0 {
            return 0.0;
        }
        (self.disk_used as f64 / self.disk as f64) * 100.0
    }

    /// Check if has capacity
    pub fn has_capacity(&self, cpu: f64, memory: u64, disk: u64) -> bool {
        let available_cpu = self.cpu_cores as f64 - self.cpu_used;
        let available_memory = self.memory - self.memory_used;
        let available_disk = self.disk - self.disk_used;

        available_cpu >= cpu && available_memory >= memory && available_disk >= disk
    }
}

/// Service discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscovery {
    /// Discovery mode
    pub mode: DiscoveryMode,
    /// Services
    pub services: Vec<Service>,
    /// DNS configuration
    #[serde(rename = "dnsConfig")]
    pub dns_config: Option<DNSConfig>,
}

impl Default for ServiceDiscovery {
    fn default() -> Self {
        Self {
            mode: DiscoveryMode::DNS,
            services: Vec::new(),
            dns_config: None,
        }
    }
}

/// Discovery mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DiscoveryMode {
    DNS,
    Consul,
    Etcd,
    Custom,
}

/// Service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Service ID
    pub id: String,
    /// Service name
    pub name: String,
    /// Service address
    pub address: String,
    /// Service port
    pub port: i32,
    /// Service tags
    pub tags: Vec<String>,
    /// Health check
    #[serde(rename = "healthCheck")]
    pub health_check: Option<ServiceHealthCheck>,
    /// Status
    pub status: ServiceStatus,
}

impl Service {
    /// Create a new service
    pub fn new(name: impl Into<String>, address: impl Into<String>, port: i32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            address: address.into(),
            port,
            tags: Vec::new(),
            health_check: None,
            status: ServiceStatus::Passing,
        }
    }

    /// Add tag
    pub fn add_tag(&mut self, tag: impl Into<String>) -> &mut Self {
        self.tags.push(tag.into());
        self
    }

    /// Set health check
    pub fn set_health_check(&mut self, check: ServiceHealthCheck) -> &mut Self {
        self.health_check = Some(check);
        self
    }

    /// Get connection string
    pub fn get_connection_string(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

/// Service health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthCheck {
    /// Check type
    #[serde(rename = "checkType")]
    pub check_type: HealthCheckType,
    /// Interval
    pub interval: String,
    /// Timeout
    pub timeout: String,
}

/// Health check type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthCheckType {
    HTTP,
    TCP,
    GRPC,
    Exec,
}

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Passing,
    Warning,
    Critical,
    Maintenance,
}

/// DNS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSConfig {
    /// DNS domain
    pub domain: String,
    /// DNS servers
    pub servers: Vec<String>,
    /// Cache TTL
    #[serde(rename = "cacheTTL")]
    pub cache_ttl: i32,
}

/// Cluster status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterStatus {
    /// Health status
    pub health: ClusterHealth,
    /// Total nodes
    #[serde(rename = "totalNodes")]
    pub total_nodes: i32,
    /// Healthy nodes
    #[serde(rename = "healthyNodes")]
    pub healthy_nodes: i32,
    /// Leader
    pub leader: Option<String>,
    /// Version
    pub version: Option<String>,
    /// Last election
    #[serde(rename = "lastElection")]
    pub last_election: Option<String>,
}

/// Cluster health
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ClusterHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Maintenance,
}

impl Default for ClusterHealth {
    fn default() -> Self {
        ClusterHealth::Healthy
    }
}

/// Cluster operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterOperation {
    /// Operation ID
    pub id: String,
    /// Operation type
    #[serde(rename = "operationType")]
    pub operation_type: ClusterOperationType,
    /// Target nodes
    #[serde(rename = "targetNodes")]
    pub target_nodes: Vec<String>,
    /// Status
    pub status: OperationStatus,
    /// Progress
    pub progress: f64,
    /// Start time
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// End time
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
}

/// Cluster operation type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ClusterOperationType {
    AddNode,
    RemoveNode,
    UpgradeNode,
    DrainNode,
    CordonNode,
    UncordonNode,
}

/// Operation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OperationStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cluster_creation() {
        let cluster = Cluster::new("test-cluster");
        assert_eq!(cluster.name, "test-cluster");
        assert_eq!(cluster.get_node_count(), 0);
    }

    #[test]
    fn test_node_creation() {
        let node = Node::new("node-1", "node-1", "192.168.1.1");
        assert_eq!(node.id, "node-1");
        assert_eq!(node.name, "node-1");
        assert_eq!(node.address, "192.168.1.1");
    }

    #[test]
    fn test_cluster_add_node() {
        let mut cluster = Cluster::new("test-cluster");
        let node = Node::new("node-1", "node-1", "192.168.1.1");
        cluster.add_node(node);
        
        assert_eq!(cluster.get_node_count(), 1);
    }

    #[test]
    fn test_cluster_quorum() {
        let mut cluster = Cluster::new("test-cluster");
        cluster.quorum.size = 2;
        
        let mut node1 = Node::new("node-1", "node-1", "192.168.1.1");
        node1.status = NodeStatus::Ready;
        
        let mut node2 = Node::new("node-2", "node-2", "192.168.1.2");
        node2.status = NodeStatus::Ready;
        
        cluster.add_node(node1);
        cluster.add_node(node2);
        
        assert!(cluster.has_quorum());
    }

    #[test]
    fn test_node_resources() {
        let resources = NodeResources::default();
        assert!(resources.has_capacity(1.0, 1024 * 1024, 1024 * 1024));
    }

    #[test]
    fn test_service_creation() {
        let service = Service::new("test-service", "192.168.1.1", 8080);
        assert_eq!(service.name, "test-service");
        assert_eq!(service.port, 8080);
    }

    #[test]
    fn test_cluster_validation() {
        let mut cluster = Cluster::new("test-cluster");
        assert!(cluster.validate().is_err()); // No nodes
        
        cluster.add_node(Node::new("node-1", "node-1", "192.168.1.1"));
        assert!(cluster.validate().is_ok());
    }
}