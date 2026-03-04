// VantisOS - File System Integration Module
// AI-powered file system optimization and management

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap, HashSet};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;

// ============================================================================
// Core Types
// ============================================================================

/// File system integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemConfig {
    /// Enable AI-powered caching
    pub enable_smart_caching: bool,
    /// Enable predictive prefetching
    pub enable_predictive_prefetch: bool,
    /// Enable automatic defragmentation suggestions
    pub enable_defrag_suggestions: bool,
    /// Enable storage optimization
    pub enable_storage_optimization: bool,
    /// Maximum cache size in MB
    pub max_cache_size_mb: u64,
    /// Prefetch lookahead window
    pub prefetch_window: usize,
    /// Monitoring interval in seconds
    pub monitoring_interval_secs: u64,
    /// Enable anomaly detection
    pub enable_anomaly_detection: bool,
    /// Performance threshold for alerts
    pub performance_threshold: PerformanceThreshold,
}

impl Default for FileSystemConfig {
    fn default() -> Self {
        Self {
            enable_smart_caching: true,
            enable_predictive_prefetch: true,
            enable_defrag_suggestions: true,
            enable_storage_optimization: true,
            max_cache_size_mb: 1024,
            prefetch_window: 10,
            monitoring_interval_secs: 60,
            enable_anomaly_detection: true,
            performance_threshold: PerformanceThreshold::default(),
        }
    }
}

/// Performance thresholds for file system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThreshold {
    /// Maximum acceptable read latency in ms
    pub max_read_latency_ms: f64,
    /// Maximum acceptable write latency in ms
    pub max_write_latency_ms: f64,
    /// Minimum throughput in MB/s
    pub min_throughput_mbps: f64,
    /// Maximum acceptable fragmentation percentage
    pub max_fragmentation_percent: f64,
    /// Maximum acceptable disk usage percentage
    pub max_disk_usage_percent: f64,
    /// Maximum acceptable IOPS variance
    pub max_iops_variance: f64,
}

impl Default for PerformanceThreshold {
    fn default() -> Self {
        Self {
            max_read_latency_ms: 100.0,
            max_write_latency_ms: 150.0,
            min_throughput_mbps: 100.0,
            max_fragmentation_percent: 15.0,
            max_disk_usage_percent: 85.0,
            max_iops_variance: 0.3,
        }
    }
}

/// File system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemMetrics {
    /// Timestamp of measurement
    pub timestamp: DateTime<Utc>,
    /// Read operations per second
    pub read_iops: f64,
    /// Write operations per second
    pub write_iops: f64,
    /// Read throughput in MB/s
    pub read_throughput: f64,
    /// Write throughput in MB/s
    pub write_throughput: f64,
    /// Average read latency in ms
    pub read_latency_ms: f64,
    /// Average write latency in ms
    pub write_latency_ms: f64,
    /// Queue depth
    pub queue_depth: u32,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
    /// Disk usage percentage
    pub disk_usage_percent: f64,
    /// Free space in GB
    pub free_space_gb: f64,
    /// Fragmentation percentage
    pub fragmentation_percent: f64,
    /// Inode usage percentage
    pub inode_usage_percent: f64,
    /// Number of open files
    pub open_files: u64,
    /// Number of open directories
    pub open_directories: u64,
    /// Mount point information
    pub mount_points: Vec<MountPointInfo>,
}

/// Mount point information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountPointInfo {
    /// Mount path
    pub path: String,
    /// File system type
    pub fs_type: String,
    /// Total size in GB
    pub total_gb: f64,
    /// Used space in GB
    pub used_gb: f64,
    /// Available space in GB
    pub available_gb: f64,
    /// Mount options
    pub options: Vec<String>,
    /// Is read-only
    pub read_only: bool,
}

/// File access pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAccessPattern {
    /// Pattern ID
    pub id: String,
    /// File path pattern (can include wildcards)
    pub path_pattern: String,
    /// Access frequency (per hour)
    pub frequency: f64,
    /// Access type distribution
    pub access_types: AccessTypeDistribution,
    /// Time of day pattern
    pub time_pattern: TimePattern,
    /// Predicted next access time
    pub predicted_next_access: Option<DateTime<Utc>>,
    /// Confidence in prediction
    pub prediction_confidence: f64,
    /// Associated files
    pub related_files: Vec<String>,
    /// Last accessed
    pub last_accessed: DateTime<Utc>,
    /// Pattern strength (0-1)
    pub strength: f64,
}

/// Access type distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTypeDistribution {
    /// Read percentage
    pub read_percent: f64,
    /// Write percentage
    pub write_percent: f64,
    /// Sequential access percentage
    pub sequential_percent: f64,
    /// Random access percentage
    pub random_percent: f64,
}

/// Time pattern for access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePattern {
    /// Peak hours (0-23)
    pub peak_hours: Vec<u8>,
    /// Off-peak hours
    pub off_peak_hours: Vec<u8>,
    /// Day of week pattern (0=Monday, 6=Sunday)
    pub active_days: Vec<u8>,
    /// Seasonal pattern detected
    pub has_seasonal_pattern: bool,
    /// Pattern periodicity in hours
    pub periodicity_hours: Option<u32>,
}

/// Cache entry with AI metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartCacheEntry {
    /// File path
    pub path: String,
    /// Entry size in bytes
    pub size: u64,
    /// Access count
    pub access_count: u64,
    /// Last access time
    pub last_access: DateTime<Utc>,
    /// Predicted next access time
    pub predicted_access: Option<DateTime<Utc>>,
    /// Priority score (0-100)
    pub priority: u32,
    /// Cache type
    pub cache_type: CacheType,
    /// TTL in seconds
    pub ttl_secs: Option<u64>,
    /// Creation time
    pub created_at: DateTime<Utc>,
    /// Tags for categorization
    pub tags: HashSet<String>,
}

/// Types of cache entries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheType {
    /// Hot data (frequently accessed)
    Hot,
    /// Warm data (moderately accessed)
    Warm,
    /// Cold data (rarely accessed)
    Cold,
    /// Predictive (prefetched)
    Predictive,
    /// Pinned (never evict)
    Pinned,
}

/// Optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemOptimization {
    /// Optimization ID
    pub id: String,
    /// Optimization type
    pub optimization_type: FileSystemOptimizationType,
    /// Affected paths
    pub paths: Vec<String>,
    /// Description
    pub description: String,
    /// Expected benefit
    pub expected_benefit: String,
    /// Priority
    pub priority: OptimizationPriority,
    /// Impact score (0-100)
    pub impact_score: u32,
    /// Effort required
    pub effort: EffortLevel,
    /// Risk level
    pub risk: RiskLevel,
    /// Prerequisites
    pub prerequisites: Vec<String>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Applied at
    pub applied_at: Option<DateTime<Utc>>,
    /// Status
    pub status: OptimizationStatus,
}

/// Types of file system optimizations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileSystemOptimizationType {
    /// Defragmentation
    Defragmentation,
    /// Cache optimization
    CacheOptimization,
    /// Storage tiering
    StorageTiering,
    /// File relocation
    FileRelocation,
    /// Prefetch configuration
    PrefetchConfig,
    /// Access pattern optimization
    AccessPatternOptimization,
    /// Compression
    Compression,
    /// Deduplication
    Deduplication,
    /// Snapshot optimization
    SnapshotOptimization,
    /// Quota adjustment
    QuotaAdjustment,
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

/// Effort levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffortLevel {
    Trivial,
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Risk levels
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

/// File system alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemAlert {
    /// Alert ID
    pub id: String,
    /// Alert type
    pub alert_type: FileSystemAlertType,
    /// Severity
    pub severity: AlertSeverity,
    /// Message
    pub message: String,
    /// Affected path or resource
    pub affected_resource: String,
    /// Current value
    pub current_value: f64,
    /// Threshold value
    pub threshold_value: f64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Acknowledged
    pub acknowledged: bool,
    /// Resolution
    pub resolution: Option<String>,
}

/// Types of file system alerts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileSystemAlertType {
    HighReadLatency,
    HighWriteLatency,
    LowThroughput,
    HighDiskUsage,
    HighFragmentation,
    CacheMiss,
    DiskFull,
    InodeExhaustion,
    IoError,
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

// ============================================================================
// File System Integration
// ============================================================================

/// Main file system integration
pub struct FileSystemIntegration {
    /// Configuration
    config: FileSystemConfig,
    /// Current metrics
    metrics: Option<FileSystemMetrics>,
    /// Metrics history
    metrics_history: BTreeMap<DateTime<Utc>, FileSystemMetrics>,
    /// Access patterns
    access_patterns: HashMap<String, FileAccessPattern>,
    /// Cache entries
    cache: HashMap<String, SmartCacheEntry>,
    /// Pending optimizations
    optimizations: HashMap<String, FileSystemOptimization>,
    /// Active alerts
    alerts: HashMap<String, FileSystemAlert>,
    /// Statistics
    stats: IntegrationStats,
}

/// Integration statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IntegrationStats {
    /// Total files monitored
    pub files_monitored: u64,
    /// Total cache hits
    pub cache_hits: u64,
    /// Total cache misses
    pub cache_misses: u64,
    /// Prefetch hits
    pub prefetch_hits: u64,
    /// Prefetch misses
    pub prefetch_misses: u64,
    /// Optimizations suggested
    pub optimizations_suggested: u64,
    /// Optimizations applied
    pub optimizations_applied: u64,
    /// Alerts generated
    pub alerts_generated: u64,
    /// Average prediction accuracy
    pub avg_prediction_accuracy: f64,
}

impl FileSystemIntegration {
    /// Create a new file system integration
    pub fn new(config: FileSystemConfig) -> Self {
        Self {
            config,
            metrics: None,
            metrics_history: BTreeMap::new(),
            access_patterns: HashMap::new(),
            cache: HashMap::new(),
            optimizations: HashMap::new(),
            alerts: HashMap::new(),
            stats: IntegrationStats::default(),
        }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(FileSystemConfig::default())
    }

    /// Update metrics
    pub fn update_metrics(&mut self, metrics: FileSystemMetrics) {
        // Check thresholds before storing
        self._check_thresholds(&metrics);
        
        // Store metrics
        let timestamp = metrics.timestamp;
        self.metrics_history.insert(timestamp, metrics.clone());
        self.metrics = Some(metrics);
        
        // Trim history if too large
        if self.metrics_history.len() > 1000 {
            let oldest = *self.metrics_history.keys().next().unwrap();
            self.metrics_history.remove(&oldest);
        }
        
        // Analyze patterns if enabled
        if self.config.enable_storage_optimization {
            self._analyze_patterns();
        }
    }

    /// Record file access
    pub fn record_access(
        &mut self,
        path: &str,
        access_type: FileAccess,
        size: u64,
        latency_ms: f64,
    ) {
        // Update or create access pattern
        let pattern = self.access_patterns.entry(path.to_string())
            .or_insert_with(|| FileAccessPattern {
                id: Uuid::new_v4().to_string(),
                path_pattern: path.to_string(),
                frequency: 0.0,
                access_types: AccessTypeDistribution {
                    read_percent: 0.0,
                    write_percent: 0.0,
                    sequential_percent: 0.0,
                    random_percent: 0.0,
                },
                time_pattern: TimePattern {
                    peak_hours: vec![],
                    off_peak_hours: vec![],
                    active_days: vec![],
                    has_seasonal_pattern: false,
                    periodicity_hours: None,
                },
                predicted_next_access: None,
                prediction_confidence: 0.0,
                related_files: vec![],
                last_accessed: Utc::now(),
                strength: 0.0,
            });
        
        // Update access statistics
        pattern.frequency += 1.0;
        pattern.last_accessed = Utc::now();
        
        // Update access type distribution
        match access_type {
            FileAccess::Read => {
                pattern.access_types.read_percent = 
                    (pattern.access_types.read_percent + 1.0) / 2.0;
            }
            FileAccess::Write => {
                pattern.access_types.write_percent = 
                    (pattern.access_types.write_percent + 1.0) / 2.0;
            }
        }
        
        // Update cache
        if self.config.enable_smart_caching {
            self._update_cache(path, size, latency_ms);
        }
        
        self.stats.files_monitored += 1;
    }

    /// Get optimization suggestions
    pub fn get_optimizations(&self) -> Vec<&FileSystemOptimization> {
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
    pub fn get_metrics(&self) -> Option<&FileSystemMetrics> {
        self.metrics.as_ref()
    }

    /// Get metrics history
    pub fn get_metrics_history(&self, hours: i64) -> Vec<&FileSystemMetrics> {
        let cutoff = Utc::now() - Duration::hours(hours);
        self.metrics_history.values()
            .filter(|m| m.timestamp >= cutoff)
            .collect()
    }

    /// Get access patterns
    pub fn get_access_patterns(&self) -> Vec<&FileAccessPattern> {
        self.access_patterns.values().collect()
    }

    /// Get cache status
    pub fn get_cache_status(&self) -> CacheStatus {
        let total_size: u64 = self.cache.values().map(|e| e.size).sum();
        let entries = self.cache.len();
        
        let mut by_type = HashMap::new();
        for entry in self.cache.values() {
            *by_type.entry(entry.cache_type).or_insert(0) += 1;
        }
        
        CacheStatus {
            total_entries: entries,
            total_size_mb: total_size as f64 / (1024.0 * 1024.0),
            max_size_mb: self.config.max_cache_size_mb as f64,
            utilization: (total_size as f64 / (self.config.max_cache_size_mb as f64 * 1024.0 * 1024.0)).min(1.0),
            entries_by_type: by_type,
            hit_ratio: if self.stats.cache_hits + self.stats.cache_misses > 0 {
                self.stats.cache_hits as f64 / (self.stats.cache_hits + self.stats.cache_misses) as f64
            } else {
                0.0
            },
        }
    }

    /// Get active alerts
    pub fn get_alerts(&self) -> Vec<&FileSystemAlert> {
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

    /// Predict next accesses
    pub fn predict_next_accesses(&self) -> Vec<(&String, DateTime<Utc>, f64)> {
        self.access_patterns.values()
            .filter_map(|p| {
                p.predicted_next_access.map(|t| {
                    (&p.path_pattern, t, p.prediction_confidence)
                })
            })
            .collect()
    }

    /// Get statistics
    pub fn get_stats(&self) -> &IntegrationStats {
        &self.stats
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Remove old metrics
    pub fn cleanup_old_metrics(&mut self, days: i64) -> usize {
        let cutoff = Utc::now() - Duration::days(days);
        let initial_len = self.metrics_history.len();
        
        self.metrics_history.retain(|t, _| *t >= cutoff);
        
        initial_len - self.metrics_history.len()
    }

    // Private methods

    fn _check_thresholds(&mut self, metrics: &FileSystemMetrics) {
        let thresholds = &self.config.performance_threshold;
        
        // Check read latency
        if metrics.read_latency_ms > thresholds.max_read_latency_ms {
            self._create_alert(
                FileSystemAlertType::HighReadLatency,
                AlertSeverity::Warning,
                format!("Read latency {}ms exceeds threshold {}ms", 
                       metrics.read_latency_ms, thresholds.max_read_latency_ms),
                "system",
                metrics.read_latency_ms,
                thresholds.max_read_latency_ms,
            );
        }
        
        // Check write latency
        if metrics.write_latency_ms > thresholds.max_write_latency_ms {
            self._create_alert(
                FileSystemAlertType::HighWriteLatency,
                AlertSeverity::Warning,
                format!("Write latency {}ms exceeds threshold {}ms", 
                       metrics.write_latency_ms, thresholds.max_write_latency_ms),
                "system",
                metrics.write_latency_ms,
                thresholds.max_write_latency_ms,
            );
        }
        
        // Check disk usage
        if metrics.disk_usage_percent > thresholds.max_disk_usage_percent {
            self._create_alert(
                FileSystemAlertType::HighDiskUsage,
                AlertSeverity::Error,
                format!("Disk usage {:.1}% exceeds threshold {:.1}%", 
                       metrics.disk_usage_percent, thresholds.max_disk_usage_percent),
                "system",
                metrics.disk_usage_percent,
                thresholds.max_disk_usage_percent,
            );
        }
        
        // Check fragmentation
        if metrics.fragmentation_percent > thresholds.max_fragmentation_percent {
            self._create_optimization(
                FileSystemOptimizationType::Defragmentation,
                vec!["/".to_string()],
                format!("Fragmentation {:.1}% exceeds threshold {:.1}%", 
                       metrics.fragmentation_percent, thresholds.max_fragmentation_percent),
                OptimizationPriority::Medium,
            );
        }
    }

    fn _create_alert(
        &mut self,
        alert_type: FileSystemAlertType,
        severity: AlertSeverity,
        message: String,
        affected_resource: String,
        current_value: f64,
        threshold_value: f64,
    ) {
        let alert = FileSystemAlert {
            id: Uuid::new_v4().to_string(),
            alert_type,
            severity,
            message,
            affected_resource,
            current_value,
            threshold_value,
            timestamp: Utc::now(),
            acknowledged: false,
            resolution: None,
        };
        
        self.alerts.insert(alert.id.clone(), alert);
        self.stats.alerts_generated += 1;
    }

    fn _create_optimization(
        &mut self,
        optimization_type: FileSystemOptimizationType,
        paths: Vec<String>,
        description: String,
        priority: OptimizationPriority,
    ) {
        let optimization = FileSystemOptimization {
            id: Uuid::new_v4().to_string(),
            optimization_type,
            paths,
            description,
            expected_benefit: "Improved performance".to_string(),
            priority,
            impact_score: 50,
            effort: EffortLevel::Medium,
            risk: RiskLevel::Low,
            prerequisites: vec![],
            created_at: Utc::now(),
            applied_at: None,
            status: OptimizationStatus::Pending,
        };
        
        self.optimizations.insert(optimization.id.clone(), optimization);
        self.stats.optimizations_suggested += 1;
    }

    fn _analyze_patterns(&mut self) {
        // Analyze access patterns and generate predictions
        for pattern in self.access_patterns.values_mut() {
            // Simple prediction based on frequency
            if pattern.frequency > 0.0 {
                let interval = 3600.0 / pattern.frequency; // Average interval in seconds
                pattern.predicted_next_access = Some(Utc::now() + Duration::seconds(interval as i64));
                pattern.prediction_confidence = (pattern.frequency / 10.0).min(1.0);
                pattern.strength = pattern.frequency.min(100.0) / 100.0;
            }
        }
    }

    fn _update_cache(&mut self, path: &str, size: u64, latency_ms: f64) {
        // Update or create cache entry
        let entry = self.cache.entry(path.to_string())
            .or_insert_with(|| SmartCacheEntry {
                path: path.to_string(),
                size,
                access_count: 0,
                last_access: Utc::now(),
                predicted_access: None,
                priority: 50,
                cache_type: CacheType::Warm,
                ttl_secs: Some(3600),
                created_at: Utc::now(),
                tags: HashSet::new(),
            });
        
        entry.access_count += 1;
        entry.last_access = Utc::now();
        
        // Update priority based on access count and latency
        if entry.access_count > 10 {
            entry.cache_type = CacheType::Hot;
            entry.priority = 90;
        } else if entry.access_count > 5 {
            entry.cache_type = CacheType::Warm;
            entry.priority = 70;
        }
        
        // Track cache hit/miss
        if latency_ms < 10.0 {
            self.stats.cache_hits += 1;
        } else {
            self.stats.cache_misses += 1;
        }
    }
}

/// File access types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAccess {
    Read,
    Write,
}

/// Cache status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatus {
    pub total_entries: usize,
    pub total_size_mb: f64,
    pub max_size_mb: f64,
    pub utilization: f64,
    pub entries_by_type: HashMap<CacheType, usize>,
    pub hit_ratio: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_creation() {
        let integration = FileSystemIntegration::with_defaults();
        assert!(integration.metrics.is_none());
        assert!(integration.access_patterns.is_empty());
    }

    #[test]
    fn test_record_access() {
        let mut integration = FileSystemIntegration::with_defaults();
        
        integration.record_access("/test/file.txt", FileAccess::Read, 1024, 5.0);
        
        assert_eq!(integration.access_patterns.len(), 1);
        assert!(integration.access_patterns.contains_key("/test/file.txt"));
    }

    #[test]
    fn test_metrics_update() {
        let mut integration = FileSystemIntegration::with_defaults();
        
        let metrics = FileSystemMetrics {
            timestamp: Utc::now(),
            read_iops: 1000.0,
            write_iops: 500.0,
            read_throughput: 100.0,
            write_throughput: 50.0,
            read_latency_ms: 5.0,
            write_latency_ms: 10.0,
            queue_depth: 5,
            cache_hit_ratio: 0.95,
            disk_usage_percent: 50.0,
            free_space_gb: 500.0,
            fragmentation_percent: 5.0,
            inode_usage_percent: 30.0,
            open_files: 100,
            open_directories: 20,
            mount_points: vec![],
        };
        
        integration.update_metrics(metrics);
        
        assert!(integration.metrics.is_some());
        assert_eq!(integration.metrics_history.len(), 1);
    }

    #[test]
    fn test_threshold_alert() {
        let mut integration = FileSystemIntegration::with_defaults();
        
        let metrics = FileSystemMetrics {
            timestamp: Utc::now(),
            read_iops: 1000.0,
            write_iops: 500.0,
            read_throughput: 100.0,
            write_throughput: 50.0,
            read_latency_ms: 150.0, // Exceeds threshold
            write_latency_ms: 10.0,
            queue_depth: 5,
            cache_hit_ratio: 0.95,
            disk_usage_percent: 50.0,
            free_space_gb: 500.0,
            fragmentation_percent: 5.0,
            inode_usage_percent: 30.0,
            open_files: 100,
            open_directories: 20,
            mount_points: vec![],
        };
        
        integration.update_metrics(metrics);
        
        let alerts = integration.get_alerts();
        assert!(!alerts.is_empty());
    }

    #[test]
    fn test_cache_status() {
        let mut integration = FileSystemIntegration::with_defaults();
        
        integration.record_access("/test/file1.txt", FileAccess::Read, 1024, 5.0);
        integration.record_access("/test/file2.txt", FileAccess::Read, 2048, 8.0);
        
        let status = integration.get_cache_status();
        assert_eq!(status.total_entries, 2);
    }
}