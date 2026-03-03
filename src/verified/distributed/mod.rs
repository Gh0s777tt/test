/// Distributed Computing Module
/// 
/// This module provides distributed computing capabilities for VantisOS,
/// enabling storage, cluster management, high availability, and disaster recovery.
/// 
/// Features:
/// - Distributed storage
/// - Cluster management
/// - High availability
/// - Disaster recovery

pub mod storage;
pub mod cluster;
pub mod ha;
pub mod disaster;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Distributed error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributedError {
    /// Storage error
    StorageError(String),
    /// Cluster error
    ClusterError(String),
    /// High availability error
    HAError(String),
    /// Disaster recovery error
    DisasterRecoveryError(String),
    /// Replication error
    ReplicationError(String),
    /// Consistency error
    ConsistencyError(String),
    /// Quorum error
    QuorumError(String),
    /// Node unavailable
    NodeUnavailable(String),
    /// Timeout
    Timeout(String),
    /// Internal error
    InternalError(String),
}

impl std::fmt::Display for DistributedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistributedError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            DistributedError::ClusterError(msg) => write!(f, "Cluster error: {}", msg),
            DistributedError::HAError(msg) => write!(f, "HA error: {}", msg),
            DistributedError::DisasterRecoveryError(msg) => write!(f, "Disaster recovery error: {}", msg),
            DistributedError::ReplicationError(msg) => write!(f, "Replication error: {}", msg),
            DistributedError::ConsistencyError(msg) => write!(f, "Consistency error: {}", msg),
            DistributedError::QuorumError(msg) => write!(f, "Quorum error: {}", msg),
            DistributedError::NodeUnavailable(msg) => write!(f, "Node unavailable: {}", msg),
            DistributedError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            DistributedError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for DistributedError {}

/// Node status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeStatus {
    /// Node is ready
    Ready,
    /// Node is not ready
    NotReady,
    /// Node is unknown
    Unknown,
    /// Node is being terminated
    Terminating,
    /// Node is terminated
    Terminated,
}

/// Replication factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationFactor {
    /// Number of replicas
    pub replicas: i32,
    /// Consistency level
    #[serde(rename = "consistencyLevel")]
    pub consistency_level: ConsistencyLevel,
}

/// Consistency level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConsistencyLevel {
    /// Strong consistency
    Strong,
    /// Eventual consistency
    Eventual,
    /// Quorum consistency
    Quorum,
}

/// Quorum configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuorumConfig {
    /// Quorum size
    pub size: i32,
    /// Election timeout
    #[serde(rename = "electionTimeoutMs")]
    pub election_timeout_ms: i32,
    /// Heartbeat interval
    #[serde(rename = "heartbeatIntervalMs")]
    pub heartbeat_interval_ms: i32,
}

impl Default for QuorumConfig {
    fn default() -> Self {
        Self {
            size: 2,
            election_timeout_ms: 5000,
            heartbeat_interval_ms: 1000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distributed_error_display() {
        let err = DistributedError::StorageError("Storage full".to_string());
        assert!(err.to_string().contains("Storage error"));
    }

    #[test]
    fn test_consistency_level() {
        let level = ConsistencyLevel::Strong;
        assert_eq!(level, ConsistencyLevel::Strong);
    }
}