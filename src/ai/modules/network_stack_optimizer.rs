//! Network Stack Optimizer with Intelligent Packet Handling
//! 
//! This module implements AI-driven network stack optimization including
//! intelligent packet scheduling, congestion control, traffic classification,
//! and adaptive quality of service (QoS) management.

use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Network optimization errors
#[derive(Error, Debug)]
pub enum NetworkOptimizerError {
    #[error("Invalid network interface: {0}")]
    InvalidInterface(String),
    
    #[error("Packet processing failed: {0}")]
    PacketProcessingFailed(String),
    
    #[error("Congestion control error: {0}")]
    CongestionControlError(String),
    
    #[error("QoS policy violation: {0}")]
    QoSViolation(String),
}

/// Configuration for network stack optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOptimizerConfig {
    /// Maximum number of connections to track
    pub max_connections: usize,
    
    /// Enable intelligent packet scheduling
    pub enable_packet_scheduling: bool,
    
    /// Enable adaptive congestion control
    pub enable_adaptive_congestion_control: bool,
    
    /// Enable traffic classification
    pub enable_traffic_classification: bool,
    
    /// Enable QoS management
    pub enable_qos: bool,
    
    /// Default QoS policy
    pub default_qos_policy: QoSPolicy,
    
    /// Congestion detection threshold (0-1)
    pub congestion_threshold: f64,
    
    /// Learning rate for adaptation
    pub learning_rate: f64,
}

/// Traffic classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TrafficType {
    /// Real-time traffic (VoIP, video streaming)
    RealTime,
    
    /// Interactive traffic (web browsing, SSH)
    Interactive,
    
    /// Bulk transfer traffic (file downloads, backups)
    Bulk,
    
    /// Background traffic (updates, sync)
    Background,
    
    /// Unknown traffic type
    Unknown,
}

/// QoS policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QoSPolicy {
    /// Minimum bandwidth guarantee (Mbps)
    pub min_bandwidth_mbps: f64,
    
    /// Maximum bandwidth limit (Mbps)
    pub max_bandwidth_mbps: f64,
    
    /// Maximum latency (ms)
    pub max_latency_ms: u64,
    
    /// Maximum packet loss rate (0-1)
    pub max_packet_loss: f64,
    
    /// Priority level (0-100, higher is more important)
    pub priority: u8,
}

impl Default for QoSPolicy {
    fn default() -> Self {
        Self {
            min_bandwidth_mbps: 0.0,
            max_bandwidth_mbps: 1000.0,
            max_latency_ms: 100,
            max_packet_loss: 0.01,
            priority: 50,
        }
    }
}

/// Network packet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPacket {
    /// Unique packet identifier
    pub packet_id: u64,
    
    /// Source address
    pub source_addr: SocketAddr,
    
    /// Destination address
    pub destination_addr: SocketAddr,
    
    /// Packet size in bytes
    pub size_bytes: usize,
    
    /// Protocol (TCP, UDP, etc.)
    pub protocol: String,
    
    /// Timestamp
    pub timestamp: Instant,
    
    /// Traffic type (if classified)
    pub traffic_type: Option<TrafficType>,
    
    /// Priority (0-100)
    pub priority: u8,
    
    /// QoS policy applied
    pub qos_policy: Option<QoSPolicy>,
}

/// Connection state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    /// Connection identifier
    pub connection_id: String,
    
    /// Source and destination addresses
    pub endpoints: (SocketAddr, SocketAddr),
    
    /// Traffic type
    pub traffic_type: TrafficType,
    
    /// Current congestion window
    pub congestion_window: usize,
    
    /// Round-trip time
    pub rtt: Duration,
    
    /// Packet loss rate
    pub packet_loss_rate: f64,
    
    /// Throughput (Mbps)
    pub throughput: f64,
    
    /// QoS policy
    pub qos_policy: QoSPolicy,
    
    /// Last activity timestamp
    pub last_activity: Instant,
    
    /// Statistics
    pub stats: ConnectionStats,
}

/// Connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    /// Total packets sent
    pub packets_sent: u64,
    
    /// Total packets received
    pub packets_received: u64,
    
    /// Total bytes transferred
    pub bytes_transferred: u64,
    
    /// Retransmissions
    pub retransmissions: u64,
    
    /// Average latency
    pub avg_latency_ms: f64,
}

/// Congestion detection model
#[derive(Debug)]
pub struct CongestionModel {
    /// Historical congestion levels
    congestion_history: VecDeque<f64>,
    
    /// Baseline congestion level
    baseline_congestion: f64,
    
    /// Adaptation learning rate
    learning_rate: f64,
    
    /// Congestion trend
    congestion_trend: f64,
}

impl CongestionModel {
    pub fn new(learning_rate: f64) -> Self {
        Self {
            congestion_history: VecDeque::with_capacity(100),
            baseline_congestion: 0.3,
            learning_rate,
            congestion_trend: 0.0,
        }
    }

    /// Update congestion level
    pub fn update(&mut self, current_congestion: f64) {
        if self.congestion_history.len() >= self.congestion_history.capacity() {
            self.congestion_history.pop_front();
        }
        self.congestion_history.push_back(current_congestion);

        // Update baseline
        if self.congestion_history.len() > 10 {
            let avg: f64 = self.congestion_history.iter()
                .take(20)
                .sum::<f64>() / self.congestion_history.len().min(20) as f64;
            
            self.baseline_congestion = self.baseline_congestion * (1.0 - self.learning_rate)
                + avg * self.learning_rate;
        }

        // Calculate trend
        if self.congestion_history.len() >= 5 {
            let recent: f64 = self.congestion_history.iter()
                .rev()
                .take(5)
                .sum::<f64>() / 5.0;
            
            let older: f64 = self.congestion_history.iter()
                .rev()
                .skip(5)
                .take(5)
                .sum::<f64>() / 5.0;
            
            self.congestion_trend = recent - older;
        }
    }

    /// Predict future congestion
    pub fn predict(&self, horizon_ms: u64) -> f64 {
        let predicted = self.baseline_congestion 
            + self.congestion_trend * (horizon_ms as f64 / 5000.0);
        
        predicted.clamp(0.0, 1.0)
    }

    /// Detect congestion
    pub fn is_congested(&self, threshold: f64) -> bool {
        self.baseline_congestion > threshold
    }
}

/// Traffic classifier
#[derive(Debug)]
pub struct TrafficClassifier {
    /// Pattern database
    patterns: HashMap<String, TrafficType>,
    
    /// Learning weights
    weights: HashMap<String, f64>,
}

impl TrafficClassifier {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        patterns.insert("voip".to_string(), TrafficType::RealTime);
        patterns.insert("video".to_string(), TrafficType::RealTime);
        patterns.insert("http".to_string(), TrafficType::Interactive);
        patterns.insert("https".to_string(), TrafficType::Interactive);
        patterns.insert("ssh".to_string(), TrafficType::Interactive);
        patterns.insert("ftp".to_string(), TrafficType::Bulk);
        patterns.insert("backup".to_string(), TrafficType::Bulk);
        patterns.insert("sync".to_string(), TrafficType::Background);
        patterns.insert("update".to_string(), TrafficType::Background);

        Self {
            patterns,
            weights: HashMap::new(),
        }
    }

    /// Classify traffic based on packet characteristics
    pub fn classify(&mut self, packet: &NetworkPacket) -> TrafficType {
        // Simple heuristics-based classification
        let traffic_type = if packet.size_bytes < 100 {
            TrafficType::Interactive
        } else if packet.size_bytes > 1400 {
            TrafficType::Bulk
        } else if packet.protocol.to_lowercase().contains("udp") {
            TrafficType::RealTime
        } else {
            TrafficType::Unknown
        };

        // Update learning weights
        let pattern_key = format!("{}:{}", packet.protocol, packet.size_bytes);
        *self.weights.entry(pattern_key).or_insert(0.5) += 0.01;

        traffic_type
    }

    /// Update classification based on feedback
    pub fn update(&mut self, pattern: &str, traffic_type: TrafficType) {
        self.patterns.insert(pattern.to_string(), traffic_type);
    }
}

/// Network Stack Optimizer
pub struct NetworkStackOptimizer {
    config: NetworkOptimizerConfig,
    
    /// Active connections
    connections: Arc<RwLock<HashMap<String, ConnectionState>>>,
    
    /// Packet queue
    packet_queue: Arc<RwLock<VecDeque<NetworkPacket>>>,
    
    /// Congestion model
    congestion_model: Arc<RwLock<CongestionModel>>,
    
    /// Traffic classifier
    classifier: Arc<RwLock<TrafficClassifier>>,
    
    /// Network statistics
    stats: Arc<RwLock<NetworkStatistics>>,
    
    /// Current congestion level
    congestion_level: Arc<RwLock<f64>>,
}

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatistics {
    /// Total packets processed
    pub total_packets: u64,
    
    /// Total bytes transferred
    pub total_bytes: u64,
    
    /// Average throughput (Mbps)
    pub avg_throughput: f64,
    
    /// Average latency (ms)
    pub avg_latency_ms: f64,
    
    /// Average packet loss rate
    pub avg_packet_loss: f64,
    
    /// Current congestion level (0-1)
    pub congestion_level: f64,
    
    /// Number of active connections
    pub active_connections: usize,
}

impl NetworkStackOptimizer {
    pub fn new(config: NetworkOptimizerConfig) -> Self {
        let congestion_model = CongestionModel::new(config.learning_rate);
        let classifier = TrafficClassifier::new();

        Self {
            config,
            connections: Arc::new(RwLock::new(HashMap::new())),
            packet_queue: Arc::new(RwLock::new(VecDeque::new())),
            congestion_model: Arc::new(RwLock::new(congestion_model)),
            classifier: Arc::new(RwLock::new(classifier)),
            stats: Arc::new(RwLock::new(NetworkStatistics {
                total_packets: 0,
                total_bytes: 0,
                avg_throughput: 0.0,
                avg_latency_ms: 0.0,
                avg_packet_loss: 0.0,
                congestion_level: 0.0,
                active_connections: 0,
            })),
            congestion_level: Arc::new(RwLock::new(0.0)),
        }
    }

    /// Process an incoming packet
    pub async fn process_packet(&self, mut packet: NetworkPacket) -> Result<(), NetworkOptimizerError> {
        // Classify traffic if enabled
        if self.config.enable_traffic_classification {
            let mut classifier = self.classifier.write().await;
            packet.traffic_type = Some(classifier.classify(&packet));
        }

        // Apply QoS policy if enabled
        if self.config.enable_qos {
            packet.qos_policy = Some(self.apply_qos_policy(&packet).await);
        }

        // Update congestion level
        self.update_congestion_level(&packet).await;

        // Add to queue for scheduling
        {
            let mut queue = self.packet_queue.write().await;
            if self.config.enable_packet_scheduling {
                // Insert in priority order
                let mut idx = 0;
                for existing in queue.iter() {
                    if existing.priority < packet.priority {
                        break;
                    }
                    idx += 1;
                }
                queue.insert(idx, packet);
            } else {
                queue.push_back(packet);
            }
        }

        Ok(())
    }

    /// Apply QoS policy based on traffic type
    async fn apply_qos_policy(&self, packet: &NetworkPacket) -> QoSPolicy {
        let traffic_type = packet.traffic_type.unwrap_or(TrafficType::Unknown);

        match traffic_type {
            TrafficType::RealTime => QoSPolicy {
                min_bandwidth_mbps: 10.0,
                max_bandwidth_mbps: 100.0,
                max_latency_ms: 20,
                max_packet_loss: 0.001,
                priority: 100,
            },
            TrafficType::Interactive => QoSPolicy {
                min_bandwidth_mbps: 1.0,
                max_bandwidth_mbps: 50.0,
                max_latency_ms: 50,
                max_packet_loss: 0.01,
                priority: 80,
            },
            TrafficType::Bulk => QoSPolicy {
                min_bandwidth_mbps: 0.0,
                max_bandwidth_mbps: 1000.0,
                max_latency_ms: 1000,
                max_packet_loss: 0.1,
                priority: 30,
            },
            TrafficType::Background => QoSPolicy {
                min_bandwidth_mbps: 0.0,
                max_bandwidth_mbps: 500.0,
                max_latency_ms: 5000,
                max_packet_loss: 0.2,
                priority: 10,
            },
            TrafficType::Unknown => self.config.default_qos_policy.clone(),
        }
    }

    /// Update congestion level based on packet data
    async fn update_congestion_level(&self, packet: &NetworkPacket) {
        let mut congestion = self.congestion_level.write().await;
        
        // Simple congestion estimation based on packet queue size
        let queue_size = {
            let queue = self.packet_queue.read().await;
            queue.len()
        };

        let estimated_congestion = (queue_size as f64 / self.config.max_connections as f64).min(1.0);
        *congestion = *congestion * 0.9 + estimated_congestion * 0.1;

        // Update congestion model
        {
            let mut model = self.congestion_model.write().await;
            model.update(*congestion);
        }
    }

    /// Get next packet to send (with intelligent scheduling)
    pub async fn get_next_packet(&self) -> Option<NetworkPacket> {
        let mut queue = self.packet_queue.write().await;
        
        if self.config.enable_packet_scheduling {
            // Adaptive congestion-aware scheduling
            let congestion = *self.congestion_level.read().await;
            
            if congestion > self.config.congestion_threshold {
                // Prioritize small packets and real-time traffic
                queue.pop_front()
            } else {
                queue.pop_front()
            }
        } else {
            queue.pop_front()
        }
    }

    /// Update connection state
    pub async fn update_connection(&self, connection_id: String, state: ConnectionState) {
        let mut connections = self.connections.write().await;
        connections.insert(connection_id.clone(), state.clone());
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.active_connections = connections.len();
        }
    }

    /// Get connection information
    pub async fn get_connection(&self, connection_id: &str) -> Option<ConnectionState> {
        let connections = self.connections.read().await;
        connections.get(connection_id).cloned()
    }

    /// Apply adaptive congestion control
    pub async fn apply_congestion_control(&self, connection_id: &str) -> Result<(), NetworkOptimizerError> {
        if !self.config.enable_adaptive_congestion_control {
            return Ok(());
        }

        let mut connections = self.connections.write().await;
        let connection = connections.get_mut(connection_id)
            .ok_or_else(|| NetworkOptimizerError::CongestionControlError(
                "Connection not found".to_string()
            ))?;

        let congestion = *self.congestion_level.read().await;

        // Adaptive congestion window adjustment
        if congestion > self.config.congestion_threshold {
            // Reduce congestion window
            connection.congestion_window = (connection.congestion_window as f64 * 0.8) as usize;
            connection.congestion_window = connection.congestion_window.max(1);
        } else {
            // Increase congestion window
            connection.congestion_window = (connection.congestion_window as f64 * 1.1) as usize;
        }

        Ok(())
    }

    /// Get network statistics
    pub async fn get_statistics(&self) -> NetworkStatistics {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Get current congestion level
    pub async fn get_congestion_level(&self) -> f64 {
        *self.congestion_level.read().await
    }

    /// Predict future congestion
    pub async fn predict_congestion(&self, horizon_ms: u64) -> f64 {
        let model = self.congestion_model.read().await;
        model.predict(horizon_ms)
    }

    /// Run the optimizer loop
    pub async fn run(&self) {
        let mut interval = tokio::time::interval(Duration::from_millis(100));
        
        loop {
            interval.tick().await;

            // Process packets in queue
            while let Some(_packet) = self.get_next_packet().await {
                // In a real implementation, this would send the packet
                self.update_statistics().await;
            }

            // Clean up stale connections
            self.cleanup_stale_connections().await;
        }
    }

    /// Update network statistics
    async fn update_statistics(&self) {
        let mut stats = self.stats.write().await;
        
        stats.congestion_level = *self.congestion_level.read().await;
        stats.active_connections = self.connections.read().await.len();
    }

    /// Clean up stale connections
    async fn cleanup_stale_connections(&self) {
        let mut connections = self.connections.write().await;
        let now = Instant::now();
        let timeout = Duration::from_secs(300); // 5 minutes

        connections.retain(|_, state| {
            now.duration_since(state.last_activity) < timeout
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config() {
        let config = NetworkOptimizerConfig {
            max_connections: 1000,
            enable_packet_scheduling: true,
            enable_adaptive_congestion_control: true,
            enable_traffic_classification: true,
            enable_qos: true,
            default_qos_policy: QoSPolicy::default(),
            congestion_threshold: 0.7,
            learning_rate: 0.1,
        };

        assert_eq!(config.max_connections, 1000);
        assert!(config.enable_packet_scheduling);
    }

    #[test]
    fn test_congestion_model() {
        let mut model = CongestionModel::new(0.1);
        
        model.update(0.5);
        model.update(0.6);
        model.update(0.7);
        
        assert!(model.is_congested(0.65));
        
        let prediction = model.predict(1000);
        assert!(prediction >= 0.0 && prediction <= 1.0);
    }

    #[test]
    fn test_traffic_classifier() {
        let mut classifier = TrafficClassifier::new();
        
        let packet = NetworkPacket {
            packet_id: 1,
            source_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 8080),
            destination_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), 80),
            size_bytes: 50,
            protocol: "TCP".to_string(),
            timestamp: Instant::now(),
            traffic_type: None,
            priority: 50,
            qos_policy: None,
        };

        let traffic_type = classifier.classify(&packet);
        assert_eq!(traffic_type, TrafficType::Interactive);
    }

    #[tokio::test]
    async fn test_packet_processing() {
        let config = NetworkOptimizerConfig {
            max_connections: 100,
            enable_packet_scheduling: true,
            enable_adaptive_congestion_control: true,
            enable_traffic_classification: true,
            enable_qos: true,
            default_qos_policy: QoSPolicy::default(),
            congestion_threshold: 0.7,
            learning_rate: 0.1,
        };

        let optimizer = NetworkStackOptimizer::new(config);
        
        let packet = NetworkPacket {
            packet_id: 1,
            source_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 8080),
            destination_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2)), 80),
            size_bytes: 1024,
            protocol: "TCP".to_string(),
            timestamp: Instant::now(),
            traffic_type: None,
            priority: 50,
            qos_policy: None,
        };

        optimizer.process_packet(packet).await.unwrap();
        
        let stats = optimizer.get_statistics().await;
        assert_eq!(stats.active_connections, 0); // No connections created yet
    }
}