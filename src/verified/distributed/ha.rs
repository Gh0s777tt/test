/// High Availability Module
/// 
/// This module provides high availability capabilities for VantisOS,
/// supporting failover, leader election, and health monitoring.
/// 
/// Features:
/// - Automatic failover
/// - Leader election
/// - Health monitoring and checks
/// - Graceful degradation
/// - State replication

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{DistributedError, NodeStatus};

/// High availability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighAvailability {
    /// HA name
    pub name: String,
    /// HA mode
    pub mode:HAMode,
    /// Failover configuration
    #[serde(rename = "failoverConfig")]
    pub failover_config: FailoverConfig,
    /// Leader election
    #[serde(rename = "leaderElection")]
    pub leader_election: LeaderElection,
    /// Health monitoring
    #[serde(rename = "healthMonitoring")]
    pub health_monitoring: HealthMonitoring,
    /// HA status
    pub status: HAStatus,
}

impl HighAvailability {
    /// Create a new HA configuration
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            mode:HAMode::ActivePassive,
            failover_config: FailoverConfig::default(),
            leader_election: LeaderElection::default(),
            health_monitoring: HealthMonitoring::default(),
            status: HAStatus::default(),
        }
    }

    /// Set HA mode
    pub fn set_mode(&mut self, mode:HAMode) -> &mut Self {
        self.mode = mode;
        self
    }

    /// Set failover configuration
    pub fn set_failover_config(&mut self, config: FailoverConfig) -> &mut Self {
        self.failover_config = config;
        self
    }

    /// Set leader election
    pub fn set_leader_election(&mut self, election: LeaderElection) -> &mut Self {
        self.leader_election = election;
        self
    }

    /// Set health monitoring
    pub fn set_health_monitoring(&mut self, monitoring: HealthMonitoring) -> &mut Self {
        self.health_monitoring = monitoring;
        self
    }

    /// Validate HA configuration
    pub fn validate(&self) -> Result<(), DistributedError> {
        if self.name.is_empty() {
            return Err(DistributedError::HAError("HA name is required".to_string()));
        }
        Ok(())
    }
}

/// HA mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HAMode {
    ActivePassive,
    ActiveActive,
    NPlusM,
}

/// Failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Enabled
    pub enabled: bool,
    /// Auto failover
    #[serde(rename = "autoFailover")]
    pub auto_failover: bool,
    /// Failover threshold
    #[serde(rename = "failoverThreshold")]
    pub failover_threshold: i32,
    /// Failover timeout in seconds
    #[serde(rename = "failoverTimeoutSeconds")]
    pub failover_timeout_seconds: i32,
    /// Pre-failover script
    #[serde(rename = "preFailoverScript")]
    pub pre_failover_script: Option<String>,
    /// Post-failover script
    #[serde(rename = "postFailoverScript")]
    pub post_failover_script: Option<String>,
    /// Grace period
    #[serde(rename = "gracePeriodSeconds")]
    pub grace_period_seconds: i32,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_failover: true,
            failover_threshold: 3,
            failover_timeout_seconds: 30,
            pre_failover_script: None,
            post_failover_script: None,
            grace_period_seconds: 5,
        }
    }
}

/// Leader election
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderElection {
    /// Enabled
    pub enabled: bool,
    /// Election algorithm
    #[serde(rename = "electionAlgorithm")]
    pub election_algorithm: ElectionAlgorithm,
    /// Term duration
    #[serde(rename = "termDurationSeconds")]
    pub term_duration_seconds: i32,
    /// Vote timeout
    #[serde(rename = "voteTimeoutSeconds")]
    pub vote_timeout_seconds: i32,
    /// Current leader
    #[serde(rename = "currentLeader")]
    pub current_leader: Option<String>,
    /// Current term
    #[serde(rename = "currentTerm")]
    pub current_term: i64,
    /// Voted for
    #[serde(rename = "votedFor")]
    pub voted_for: Option<String>,
}

impl Default for LeaderElection {
    fn default() -> Self {
        Self {
            enabled: true,
            election_algorithm: ElectionAlgorithm::Raft,
            term_duration_seconds: 300,
            vote_timeout_seconds: 10,
            current_leader: None,
            current_term: 0,
            voted_for: None,
        }
    }
}

/// Election algorithm
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ElectionAlgorithm {
    Raft,
    Paxos,
    Bully,
    Zookeeper,
    Etcd,
}

impl LeaderElection {
    /// Request vote
    pub fn request_vote(&mut self, candidate_id: &str, term: i64) -> bool {
        if term > self.current_term {
            self.current_term = term;
            self.voted_for = Some(candidate_id.to_string());
            self.current_leader = None;
            true
        } else if term == self.current_term {
            if self.voted_for.as_ref().map_or(false, |v| v == candidate_id) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Become leader
    pub fn become_leader(&mut self, leader_id: &str) {
        self.current_leader = Some(leader_id.to_string());
    }

    /// Step down
    pub fn step_down(&mut self) {
        self.current_leader = None;
    }
}

/// Health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoring {
    /// Enabled
    pub enabled: bool,
    /// Check interval
    #[serde(rename = "checkIntervalSeconds")]
    pub check_interval_seconds: i32,
    /// Check timeout
    #[serde(rename = "checkTimeoutSeconds")]
    pub check_timeout_seconds: i32,
    /// Unhealthy threshold
    #[serde(rename = "unhealthyThreshold")]
    pub unhealthy_threshold: i32,
    /// Healthy threshold
    #[serde(rename = "healthyThreshold")]
    pub healthy_threshold: i32,
    /// Health checks
    #[serde(rename = "healthChecks")]
    pub health_checks: Vec<HealthCheck>,
    /// Node health status
    #[serde(rename = "nodeHealthStatus")]
    pub node_health_status: HashMap<String, NodeHealth>,
}

impl Default for HealthMonitoring {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval_seconds: 10,
            check_timeout_seconds: 5,
            unhealthy_threshold: 3,
            healthy_threshold: 2,
            health_checks: Vec::new(),
            node_health_status: HashMap::new(),
        }
    }
}

impl HealthMonitoring {
    /// Add health check
    pub fn add_health_check(&mut self, check: HealthCheck) -> &mut Self {
        self.health_checks.push(check);
        self
    }

    /// Check node health
    pub fn check_node_health(&mut self, node_id: &str, is_healthy: bool) {
        let node_health = self.node_health_status.entry(node_id.to_string()).or_insert_with(NodeHealth::default);
        
        if is_healthy {
            node_health.consecutive_failures = 0;
            node_health.consecutive_successes += 1;
            
            if node_health.consecutive_successes >= self.healthy_threshold {
                node_health.status = HealthStatus::Healthy;
            }
        } else {
            node_health.consecutive_successes = 0;
            node_health.consecutive_failures += 1;
            
            if node_health.consecutive_failures >= self.unhealthy_threshold {
                node_health.status = HealthStatus::Unhealthy;
            }
        }
        
        node_health.last_check = Some(chrono::Utc::now().to_rfc3339());
    }

    /// Get node health status
    pub fn get_node_health(&self, node_id: &str) -> Option<&NodeHealth> {
        self.node_health_status.get(node_id)
    }

    /// Check if node is healthy
    pub fn is_node_healthy(&self, node_id: &str) -> bool {
        self.node_health_status.get(node_id)
            .map_or(false, |h| h.status == HealthStatus::Healthy)
    }
}

/// Health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Check ID
    pub id: String,
    /// Check name
    pub name: String,
    /// Check type
    #[serde(rename = "checkType")]
    pub check_type: HealthCheckType,
    /// Check endpoint
    pub endpoint: String,
    /// Interval
    pub interval: i32,
    /// Timeout
    pub timeout: i32,
}

/// Health check type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthCheckType {
    HTTP,
    TCP,
    ICMP,
    Exec,
    Custom,
}

/// Node health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHealth {
    /// Health status
    pub status: HealthStatus,
    /// Consecutive successes
    #[serde(rename = "consecutiveSuccesses")]
    pub consecutive_successes: i32,
    /// Consecutive failures
    #[serde(rename = "consecutiveFailures")]
    pub consecutive_failures: i32,
    /// Last check
    #[serde(rename = "lastCheck")]
    pub last_check: Option<String>,
}

impl Default for NodeHealth {
    fn default() -> Self {
        Self {
            status: HealthStatus::Unknown,
            consecutive_successes: 0,
            consecutive_failures: 0,
            last_check: None,
        }
    }
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Degraded,
    Unknown,
}

/// HA status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HAStatus {
    /// Current mode
    pub current_mode: HAMode,
    /// Active node
    #[serde(rename = "activeNode")]
    pub active_node: Option<String>,
    /// Standby nodes
    #[serde(rename = "standbyNodes")]
    pub standby_nodes: Vec<String>,
    /// Is failing over
    #[serde(rename = "isFailingOver")]
    pub is_failing_over: bool,
    /// Last failover time
    #[serde(rename = "lastFailoverTime")]
    pub last_failover_time: Option<String>,
}

/// State replication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateReplication {
    /// Replication mode
    #[serde(rename = "replicationMode")]
    pub replication_mode: ReplicationMode,
    /// Sync interval
    #[serde(rename = "syncIntervalSeconds")]
    pub sync_interval_seconds: i32,
    /// Replication lag
    #[serde(rename = "replicationLagMs")]
    pub replication_lag_ms: i32,
    /// Replication status
    pub status: ReplicationStatus,
}

/// Replication mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReplicationMode {
    Synchronous,
    Asynchronous,
    SemiSynchronous,
}

/// Replication status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ReplicationStatus {
    Syncing,
    Synced,
    Lagging,
    Error,
}

impl Default for StateReplication {
    fn default() -> Self {
        Self {
            replication_mode: ReplicationMode::SemiSynchronous,
            sync_interval_seconds: 5,
            replication_lag_ms: 0,
            status: ReplicationStatus::Synced,
        }
    }
}

/// Graceful degradation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GracefulDegradation {
    /// Enabled
    pub enabled: bool,
    /// Degradation levels
    #[serde(rename = "degradationLevels")]
    pub degradation_levels: Vec<DegradationLevel>,
    /// Current level
    #[serde(rename = "currentLevel")]
    pub current_level: i32,
    /// Triggers
    pub triggers: Vec<DegradationTrigger>,
}

/// Degradation level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationLevel {
    /// Level number
    pub level: i32,
    /// Name
    pub name: String,
    /// Description
    pub description: String,
    /// Actions
    pub actions: Vec<DegradationAction>,
}

/// Degradation action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationAction {
    /// Action type
    #[serde(rename = "actionType")]
    pub action_type: String,
    /// Parameters
    pub parameters: HashMap<String, String>,
}

/// Degradation trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationTrigger {
    /// Trigger condition
    pub condition: String,
    /// Threshold
    pub threshold: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ha_creation() {
        let ha = HighAvailability::new("test-ha");
        assert_eq!(ha.name, "test-ha");
        assert_eq!(ha.mode, HAMode::ActivePassive);
    }

    #[test]
    fn test_leader_election() {
        let mut election = LeaderElection::default();
        
        // Request vote
        let granted = election.request_vote("candidate-1", 1);
        assert!(granted);
        
        // Become leader
        election.become_leader("candidate-1");
        assert_eq!(election.current_leader, Some("candidate-1".to_string()));
    }

    #[test]
    fn test_health_monitoring() {
        let mut monitoring = HealthMonitoring::default();
        
        // Check healthy
        monitoring.check_node_health("node-1", true);
        monitoring.check_node_health("node-1", true);
        
        assert!(monitoring.is_node_healthy("node-1"));
        
        // Check unhealthy
        monitoring.check_node_health("node-1", false);
        monitoring.check_node_health("node-1", false);
        monitoring.check_node_health("node-1", false);
        
        assert!(!monitoring.is_node_healthy("node-1"));
    }

    #[test]
    fn test_failover_config() {
        let config = FailoverConfig::default();
        assert!(config.enabled);
        assert!(config.auto_failover);
        assert_eq!(config.failover_threshold, 3);
    }

    #[test]
    fn test_ha_validation() {
        let ha = HighAvailability::new("test-ha");
        assert!(ha.validate().is_ok());
        
        let ha = HighAvailability::new("");
        assert!(ha.validate().is_err());
    }
}