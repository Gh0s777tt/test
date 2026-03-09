// VantisOS - Network Integration Module
// AI-powered network stack optimization and management

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, HashSet};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

// ============================================================================
// Core Types
// ============================================================================

/// Network integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Enable traffic prediction
    pub enable_traffic_prediction: bool,
    /// Enable bandwidth optimization
    pub enable_bandwidth_optimization: bool,
    /// Enable connection pooling optimization
    pub enable_connection_pooling: bool,
    /// Enable load balancing optimization
    pub enable_load_balancing: bool,
    /// Enable anomaly detection
    pub enable_anomaly_detection: bool,
    /// Prediction window in seconds
    pub prediction_window_secs: u64,
    /// Monitoring interval in seconds
    pub monitoring_interval_secs: u64,
    /// Performance thresholds
    pub performance_threshold: NetworkPerformanceThreshold,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enable_traffic_prediction: true,
            enable_bandwidth_optimization: true,
            enable_connection_pooling: true,
            enable_load_balancing: true,
            enable_anomaly_detection: true,
            prediction_window_secs: 300,
            monitoring_interval_secs: 60,
            performance_threshold: NetworkPerformanceThreshold::default(),
        }
    }
}

/// Network performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformanceThreshold {
    /// Maximum acceptable latency in ms
    pub max_latency_ms: f64,
    /// Minimum acceptable throughput in Mbps
    pub min_throughput_mbps: f64,
    /// Maximum acceptable packet loss percentage
    pub max_packet_loss_percent: f64,
    /// Maximum acceptable jitter in ms
    pub max_jitter_ms: f64,
    /// Maximum acceptable connection error rate
    pub max_error_rate: f64,
    /// Maximum acceptable bandwidth utilization
    pub max_bandwidth_utilization: f64,
}

impl Default for NetworkPerformanceThreshold {
    fn default() -> Self {
        Self {
            max_latency_ms: 100.0,
            min_throughput_mbps: 100.0,
            max_packet_loss_percent: 1.0,
            max_jitter_ms: 30.0,
            max_error_rate: 0.01,
            max_bandwidth_utilization: 0.85,
        }
    }
}

/// Network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Timestamp of measurement
    pub timestamp: DateTime<Utc>,
    /// Interface metrics
    pub interfaces: Vec<InterfaceMetrics>,
    /// Total bytes received
    pub total_bytes_received: u64,
    /// Total bytes sent
    pub total_bytes_sent: u64,
    /// Active connections
    pub active_connections: u64,
    /// Connection errors
    pub connection_errors: u64,
    /// Average latency in ms
    pub avg_latency_ms: f64,
    /// Packet loss rate
    pub packet_loss_rate: f64,
    /// Jitter in ms
    pub jitter_ms: f64,
    /// DNS resolution time in ms
    pub dns_resolution_time_ms: f64,
    /// TCP retransmission rate
    pub tcp_retransmit_rate: f64,
    /// Bandwidth utilization
    pub bandwidth_utilization: f64,
    /// Queue depth
    pub queue_depth: u32,
}

/// Interface-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceMetrics {
    /// Interface name
    pub name: String,
    /// MAC address
    pub mac_address: String,
    /// IP addresses
    pub ip_addresses: Vec<String>,
    /// MTU
    pub mtu: u32,
    /// Speed in Mbps
    pub speed_mbps: u32,
    /// Duplex mode
    pub duplex: DuplexMode,
    /// Link status
    pub link_up: bool,
    /// RX bytes
    pub rx_bytes: u64,
    /// TX bytes
    pub tx_bytes: u64,
    /// RX packets
    pub rx_packets: u64,
    /// TX packets
    pub tx_packets: u64,
    /// RX errors
    pub rx_errors: u64,
    /// TX errors
    pub tx_errors: u64,
    /// RX dropped
    pub rx_dropped: u64,
    /// TX dropped
    pub tx_dropped: u64,
}

/// Duplex mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DuplexMode {
    Half,
    Full,
    Unknown,
}

/// Traffic pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPattern {
    /// Pattern ID
    pub id: String,
    /// Source/destination pattern
    pub src_pattern: String,
    pub dst_pattern: String,
    /// Protocol
    pub protocol: Protocol,
    /// Port range
    pub port_range: Option<(u16, u16)>,
    /// Traffic frequency (requests per minute)
    pub frequency: f64,
    /// Average packet size
    pub avg_packet_size: u32,
    /// Traffic volume in bytes per second
    pub volume_bps: f64,
    /// Time pattern
    pub time_pattern: TrafficTimePattern,
    /// Predicted next activity
    pub predicted_activity: Option<DateTime<Utc>>,
    /// Prediction confidence
    pub prediction_confidence: f64,
    /// Pattern strength
    pub strength: f64,
    /// Last activity
    pub last_activity: DateTime<Utc>,
}

/// Protocol types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    Other(u8),
}

/// Traffic time pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficTimePattern {
    /// Peak hours
    pub peak_hours: Vec<u8>,
    /// Off-peak hours
    pub off_peak_hours: Vec<u8>,
    /// Active days
    pub active_days: Vec<u8>,
    /// Has weekly pattern
    pub has_weekly_pattern: bool,
    /// Periodicity in hours
    pub periodicity_hours: Option<u32>,
}

/// Connection pool entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolEntry {
    /// Connection ID
    pub id: String,
    /// Remote address
    pub remote_address: String,
    /// Remote port
    pub remote_port: u16,
    /// Local port
    pub local_port: u16,
    /// Protocol
    pub protocol: Protocol,
    /// State
    pub state: ConnectionState,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Last activity
    pub last_activity: DateTime<Utc>,
    /// Bytes sent
    pub bytes_sent: u64,
    /// Bytes received
    pub bytes_received: u64,
    /// Priority
    pub priority: ConnectionPriority,
    /// Keep-alive enabled
    pub keep_alive: bool,
    /// TTL in seconds
    pub ttl_secs: Option<u64>,
}

/// Connection states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    Connecting,
    Established,
    Closing,
    Closed,
    Error,
}

/// Connection priorities
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConnectionPriority {
    Low,
    Normal,
    High,
    Critical,
    RealTime,
}

/// Network optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOptimization {
    /// Optimization ID
    pub id: String,
    /// Optimization type
    pub optimization_type: NetworkOptimizationType,
    /// Affected endpoints
    pub endpoints: Vec<String>,
    /// Description
    pub description: String,
    /// Expected benefit
    pub expected_benefit: String,
    /// Priority
    pub priority: OptimizationPriority,
    /// Impact score
    pub impact_score: u32,
    /// Risk level
    pub risk: RiskLevel,
    /// Status
    pub status: OptimizationStatus,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Applied at
    pub applied_at: Option<DateTime<Utc>>,
}

/// Network optimization types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkOptimizationType {
    BandwidthAllocation,
    LoadBalancing,
    ConnectionPooling,
    TrafficShaping,
    QoSAdjustment,
    RouteOptimization,
    CacheOptimization,
    DnsOptimization,
    TcpTuning,
    BufferOptimization,
}

/// Optimization priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum OptimizationPriority {
    Low,
    Medium,
    High,
    Critical,
    Immediate,
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Optimization status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationStatus {
    Pending,
    Scheduled,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Network alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAlert {
    /// Alert ID
    pub id: String,
    /// Alert type
    pub alert_type: NetworkAlertType,
    /// Severity
    pub severity: AlertSeverity,
    /// Message
    pub message: String,
    /// Affected resource
    pub affected_resource: String,
    /// Current value
    pub current_value: f64,
    /// Threshold value
    pub threshold_value: f64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Acknowledged
    pub acknowledged: bool,
}

/// Network alert types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkAlertType {
    HighLatency,
    LowThroughput,
    HighPacketLoss,
    HighJitter,
    BandwidthSaturated,
    ConnectionErrors,
    DnsResolutionSlow,
    TcpRetransmissions,
    InterfaceDown,
    AnomalyDetected,
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Traffic prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPrediction {
    /// Prediction ID
    pub id: String,
    /// Prediction timestamp
    pub timestamp: DateTime<Utc>,
    /// Prediction horizon in seconds
    pub horizon_secs: u64,
    /// Predicted throughput in Mbps
    pub predicted_throughput: f64,
    /// Predicted latency in ms
    pub predicted_latency: f64,
    /// Predicted connection count
    pub predicted_connections: u64,
    /// Confidence level
    pub confidence: f64,
    /// Model used
    pub model_type: PredictionModel,
    /// Features used
    pub features: Vec<String>,
}

/// Prediction models
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionModel {
    ARIMA,
    ExponentialSmoothing,
    NeuralNetwork,
    Ensemble,
    Hybrid,
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Backend servers
    pub backends: Vec<BackendServer>,
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
    /// Health check timeout in ms
    pub health_check_timeout_ms: u64,
    /// Maximum connections per backend
    pub max_connections_per_backend: u32,
    /// Enable sticky sessions
    pub sticky_sessions: bool,
}

/// Backend server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendServer {
    /// Server ID
    pub id: String,
    /// Server address
    pub address: String,
    /// Server port
    pub port: u16,
    /// Weight
    pub weight: u32,
    /// Is healthy
    pub healthy: bool,
    /// Active connections
    pub active_connections: u32,
    /// Response time in ms
    pub response_time_ms: f64,
    /// Last health check
    pub last_health_check: DateTime<Utc>,
}

/// Load balancing algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    WeightedLeastConnections,
    IPHash,
    LeastResponseTime,
}

// ============================================================================
// Network Integration
// ============================================================================

/// Main network integration
pub struct NetworkIntegration {
    /// Configuration
    config: NetworkConfig,
    /// Current metrics
    metrics: Option<NetworkMetrics>,
    /// Metrics history
    metrics_history: BTreeMap<DateTime<Utc>, NetworkMetrics>,
    /// Traffic patterns
    traffic_patterns: HashMap<String, TrafficPattern>,
    /// Connection pool
    connection_pool: HashMap<String, ConnectionPoolEntry>,
    /// Pending optimizations
    optimizations: HashMap<String, NetworkOptimization>,
    /// Active alerts
    alerts: HashMap<String, NetworkAlert>,
    /// Traffic predictions
    predictions: Vec<TrafficPrediction>,
    /// Load balancer config
    load_balancer: Option<LoadBalancerConfig>,
    /// Statistics
    stats: NetworkStats,
}

/// Network statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkStats {
    /// Total connections managed
    pub total_connections: u64,
    /// Active connections
    pub active_connections: u64,
    /// Total bytes transferred
    pub total_bytes: u64,
    /// Average latency
    pub avg_latency_ms: f64,
    /// Cache hit ratio
    pub connection_pool_hit_ratio: f64,
    /// Optimizations suggested
    pub optimizations_suggested: u64,
    /// Optimizations applied
    pub optimizations_applied: u64,
    /// Alerts generated
    pub alerts_generated: u64,
    /// Prediction accuracy
    pub avg_prediction_accuracy: f64,
}

impl NetworkIntegration {
    /// Create a new network integration
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            metrics: None,
            metrics_history: BTreeMap::new(),
            traffic_patterns: HashMap::new(),
            connection_pool: HashMap::new(),
            optimizations: HashMap::new(),
            alerts: HashMap::new(),
            predictions: Vec::new(),
            load_balancer: None,
            stats: NetworkStats::default(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(NetworkConfig::default())
    }

    /// Update network metrics
    pub fn update_metrics(&mut self, metrics: NetworkMetrics) {
        // Check thresholds
        self._check_thresholds(&metrics);
        
        // Store metrics
        let timestamp = metrics.timestamp;
        self.metrics_history.insert(timestamp, metrics.clone());
        self.metrics = Some(metrics);
        
        // Trim history
        if self.metrics_history.len() > 1000 {
            let oldest = *self.metrics_history.keys().next().unwrap();
            self.metrics_history.remove(&oldest);
        }
        
        // Analyze patterns
        if self.config.enable_traffic_prediction {
            self._analyze_traffic_patterns();
        }
        
        // Update predictions
        self._update_predictions();
    }

    /// Record network activity
    pub fn record_activity(
        &mut self,
        src: &str,
        dst: &str,
        protocol: Protocol,
        bytes: u64,
        latency_ms: f64,
    ) {
        // Create or update traffic pattern
        let pattern_key = format!("{}:{}:{:?}", src, dst, protocol);
        
        let pattern = self.traffic_patterns.entry(pattern_key.clone())
            .or_insert_with(|| TrafficPattern {
                id: Uuid::new_v4().to_string(),
                src_pattern: src.to_string(),
                dst_pattern: dst.to_string(),
                protocol,
                port_range: None,
                frequency: 0.0,
                avg_packet_size: 0,
                volume_bps: 0.0,
                time_pattern: TrafficTimePattern {
                    peak_hours: vec![],
                    off_peak_hours: vec![],
                    active_days: vec![],
                    has_weekly_pattern: false,
                    periodicity_hours: None,
                },
                predicted_activity: None,
                prediction_confidence: 0.0,
                strength: 0.0,
                last_activity: Utc::now(),
            });
        
        pattern.frequency += 1.0;
        pattern.avg_packet_size = ((pattern.avg_packet_size as u64 + bytes) / 2) as u32;
        pattern.last_activity = Utc::now();
        
        // Update stats
        self.stats.total_bytes += bytes;
        self.stats.total_connections += 1;
    }

    /// Get optimization suggestions
    pub fn get_optimizations(&self) -> Vec<&NetworkOptimization> {
        self.optimizations.values().collect()
    }

    /// Apply optimization
    pub fn apply_optimization(&mut self, optimization_id: &str) -> Result<(), String> {
        let optimization = self.optimizations.get_mut(optimization_id)
            .ok_or_else(|| format!("Optimization {} not found", optimization_id))?;
        
        optimization.status = OptimizationStatus::Completed;
        optimization.applied_at = Some(Utc::now());
        
        self.stats.optimizations_applied += 1;
        
        Ok(())
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> Option<&NetworkMetrics> {
        self.metrics.as_ref()
    }

    /// Get traffic patterns
    pub fn get_traffic_patterns(&self) -> Vec<&TrafficPattern> {
        self.traffic_patterns.values().collect()
    }

    /// Get traffic predictions
    pub fn get_predictions(&self) -> &[TrafficPrediction] {
        &self.predictions
    }

    /// Get active alerts
    pub fn get_alerts(&self) -> Vec<&NetworkAlert> {
        self.alerts.values()
            .filter(|a| !a.acknowledged)
            .collect()
    }

    /// Acknowledge alert
    pub fn acknowledge_alert(&mut self, alert_id: &str) -> Result<(), String> {
        let alert = self.alerts.get_mut(alert_id)
            .ok_or_else(|| format!("Alert {} not found", alert_id))?;
        
        alert.acknowledged = true;
        Ok(())
    }

    /// Get statistics
    pub fn get_stats(&self) -> &NetworkStats {
        &self.stats
    }

    /// Configure load balancer
    pub fn configure_load_balancer(&mut self, config: LoadBalancerConfig) {
        self.load_balancer = Some(config);
    }

    /// Get next backend server (load balancing)
    pub fn get_next_backend(&self) -> Option<&BackendServer> {
        if let Some(lb) = &self.load_balancer {
            let healthy_backends: Vec<_> = lb.backends.iter()
                .filter(|b| b.healthy)
                .collect();
            
            if healthy_backends.is_empty() {
                return None;
            }
            
            match lb.algorithm {
                LoadBalancingAlgorithm::RoundRobin => {
                    // Simple round-robin (would track index in real implementation)
                    healthy_backends.first().copied()
                }
                LoadBalancingAlgorithm::LeastConnections => {
                    healthy_backends.into_iter()
                        .min_by_key(|b| b.active_connections)
                }
                LoadBalancingAlgorithm::LeastResponseTime => {
                    healthy_backends.into_iter()
                        .min_by(|a, b| a.response_time_ms.partial_cmp(&b.response_time_ms).unwrap())
                }
                _ => healthy_backends.first().copied(),
            }
        } else {
            None
        }
    }

    // Private methods

    fn _check_thresholds(&mut self, metrics: &NetworkMetrics) {
        let thresholds = &self.config.performance_threshold;
        
        // Check latency
        if metrics.avg_latency_ms > thresholds.max_latency_ms {
            self._create_alert(
                NetworkAlertType::HighLatency,
                AlertSeverity::Warning,
                format!("Network latency {}ms exceeds threshold {}ms",
                       metrics.avg_latency_ms, thresholds.max_latency_ms),
                "network",
                metrics.avg_latency_ms,
                thresholds.max_latency_ms,
            );
        }
        
        // Check packet loss
        if metrics.packet_loss_rate * 100.0 > thresholds.max_packet_loss_percent {
            self._create_alert(
                NetworkAlertType::HighPacketLoss,
                AlertSeverity::Warning,
                format!("Packet loss {:.2}% exceeds threshold {:.2}%",
                       metrics.packet_loss_rate * 100.0, thresholds.max_packet_loss_percent),
                "network",
                metrics.packet_loss_rate * 100.0,
                thresholds.max_packet_loss_percent,
            );
        }
        
        // Check bandwidth utilization
        if metrics.bandwidth_utilization > thresholds.max_bandwidth_utilization {
            self._create_optimization(
                NetworkOptimizationType::BandwidthAllocation,
                vec!["all".to_string()],
                "High bandwidth utilization detected",
                OptimizationPriority::Medium,
            );
        }
    }

    fn _create_alert(
        &mut self,
        alert_type: NetworkAlertType,
        severity: AlertSeverity,
        message: String,
        affected_resource: String,
        current_value: f64,
        threshold_value: f64,
    ) {
        let alert = NetworkAlert {
            id: Uuid::new_v4().to_string(),
            alert_type,
            severity,
            message,
            affected_resource,
            current_value,
            threshold_value,
            timestamp: Utc::now(),
            acknowledged: false,
        };
        
        self.alerts.insert(alert.id.clone(), alert);
        self.stats.alerts_generated += 1;
    }

    fn _create_optimization(
        &mut self,
        optimization_type: NetworkOptimizationType,
        endpoints: Vec<String>,
        description: String,
        priority: OptimizationPriority,
    ) {
        let optimization = NetworkOptimization {
            id: Uuid::new_v4().to_string(),
            optimization_type,
            endpoints,
            description,
            expected_benefit: "Improved network performance".to_string(),
            priority,
            impact_score: 50,
            risk: RiskLevel::Low,
            status: OptimizationStatus::Pending,
            created_at: Utc::now(),
            applied_at: None,
        };
        
        self.optimizations.insert(optimization.id.clone(), optimization);
        self.stats.optimizations_suggested += 1;
    }

    fn _analyze_traffic_patterns(&mut self) {
        for pattern in self.traffic_patterns.values_mut() {
            if pattern.frequency > 0.0 {
                let interval = 60.0 / pattern.frequency;
                pattern.predicted_activity = Some(Utc::now() + Duration::seconds(interval as i64));
                pattern.prediction_confidence = (pattern.frequency / 10.0).min(1.0);
                pattern.strength = pattern.frequency.min(100.0) / 100.0;
            }
        }
    }

    fn _update_predictions(&mut self) {
        if self.metrics_history.len() < 10 {
            return;
        }
        
        // Simple prediction based on recent trends
        let recent: Vec<_> = self.metrics_history.values().rev().take(10).collect();
        
        let avg_throughput: f64 = recent.iter()
            .map(|m| (m.total_bytes_received + m.total_bytes_sent) as f64)
            .sum::<f64>() / recent.len() as f64;
        
        let avg_latency: f64 = recent.iter()
            .map(|m| m.avg_latency_ms)
            .sum::<f64>() / recent.len() as f64;
        
        let avg_connections: f64 = recent.iter()
            .map(|m| m.active_connections as f64)
            .sum::<f64>() / recent.len() as f64;
        
        let prediction = TrafficPrediction {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            horizon_secs: self.config.prediction_window_secs,
            predicted_throughput: avg_throughput,
            predicted_latency: avg_latency,
            predicted_connections: avg_connections as u64,
            confidence: 0.7,
            model_type: PredictionModel::Ensemble,
            features: vec!["throughput".to_string(), "latency".to_string(), "connections".to_string()],
        };
        
        self.predictions.push(prediction);
        
        // Keep only recent predictions
        if self.predictions.len() > 100 {
            self.predictions.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_creation() {
        let integration = NetworkIntegration::with_defaults();
        assert!(integration.metrics.is_none());
    }

    #[test]
    fn test_record_activity() {
        let mut integration = NetworkIntegration::with_defaults();
        
        integration.record_activity("192.168.1.1", "10.0.0.1", Protocol::TCP, 1024, 5.0);
        
        assert_eq!(integration.traffic_patterns.len(), 1);
    }

    #[test]
    fn test_metrics_update() {
        let mut integration = NetworkIntegration::with_defaults();
        
        let metrics = NetworkMetrics {
            timestamp: Utc::now(),
            interfaces: vec![],
            total_bytes_received: 1000000,
            total_bytes_sent: 500000,
            active_connections: 100,
            connection_errors: 0,
            avg_latency_ms: 5.0,
            packet_loss_rate: 0.001,
            jitter_ms: 2.0,
            dns_resolution_time_ms: 10.0,
            tcp_retransmit_rate: 0.01,
            bandwidth_utilization: 0.5,
            queue_depth: 5,
        };
        
        integration.update_metrics(metrics);
        
        assert!(integration.metrics.is_some());
    }

    #[test]
    fn test_load_balancing() {
        let mut integration = NetworkIntegration::with_defaults();
        
        let lb_config = LoadBalancerConfig {
            backends: vec![
                BackendServer {
                    id: "backend1".to_string(),
                    address: "10.0.0.1".to_string(),
                    port: 8080,
                    weight: 1,
                    healthy: true,
                    active_connections: 5,
                    response_time_ms: 10.0,
                    last_health_check: Utc::now(),
                },
                BackendServer {
                    id: "backend2".to_string(),
                    address: "10.0.0.2".to_string(),
                    port: 8080,
                    weight: 1,
                    healthy: true,
                    active_connections: 10,
                    response_time_ms: 15.0,
                    last_health_check: Utc::now(),
                },
            ],
            algorithm: LoadBalancingAlgorithm::LeastConnections,
            health_check_interval_secs: 30,
            health_check_timeout_ms: 5000,
            max_connections_per_backend: 100,
            sticky_sessions: false,
        };
        
        integration.configure_load_balancer(lb_config);
        
        let backend = integration.get_next_backend();
        assert!(backend.is_some());
        assert_eq!(backend.unwrap().id, "backend1"); // Has fewer connections
    }
}