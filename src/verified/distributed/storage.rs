/// Distributed Storage Module
/// 
/// This module provides distributed storage capabilities for VantisOS,
/// supporting multiple storage backends with replication and consistency guarantees.
/// 
/// Features:
/// - Distributed storage backends (Ceph, MinIO, S3)
/// - Data replication and sharding
/// - Consistency levels (Strong, Eventual, Quorum)
/// - Storage pool management
/// - Snapshot and backup

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{DistributedError, ReplicationFactor, ConsistencyLevel};

/// Distributed storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedStorage {
    /// Storage name
    pub name: String,
    /// Storage backend
    pub backend: StorageBackend,
    /// Replication configuration
    pub replication: ReplicationFactor,
    /// Storage pools
    pub pools: Vec<StoragePool>,
    /// Storage status
    pub status: StorageStatus,
}

impl DistributedStorage {
    /// Create a new distributed storage
    pub fn new(name: impl Into<String>, backend: StorageBackend) -> Self {
        Self {
            name: name.into(),
            backend,
            replication: ReplicationFactor {
                replicas: 3,
                consistency_level: ConsistencyLevel::Quorum,
            },
            pools: Vec::new(),
            status: StorageStatus::default(),
        }
    }

    /// Set replication factor
    pub fn set_replication(&mut self, replicas: i32, consistency: ConsistencyLevel) -> &mut Self {
        self.replication = ReplicationFactor {
            replicas,
            consistency_level: consistency,
        };
        self
    }

    /// Add a storage pool
    pub fn add_pool(&mut self, pool: StoragePool) -> &mut Self {
        self.pools.push(pool);
        self
    }

    /// Get total capacity
    pub fn get_total_capacity(&self) -> u64 {
        self.pools.iter().map(|p| p.capacity).sum()
    }

    /// Get available capacity
    pub fn get_available_capacity(&self) -> u64 {
        self.pools.iter().map(|p| p.available).sum()
    }

    /// Get used capacity
    pub fn get_used_capacity(&self) -> u64 {
        self.pools.iter().map(|p| p.capacity - p.available).sum()
    }

    /// Validate storage configuration
    pub fn validate(&self) -> Result<(), DistributedError> {
        if self.name.is_empty() {
            return Err(DistributedError::StorageError("Storage name is required".to_string()));
        }
        if self.replication.replicas < 1 {
            return Err(DistributedError::StorageError("Replication factor must be at least 1".to_string()));
        }
        Ok(())
    }
}

/// Storage backend type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StorageBackend {
    Ceph,
    MinIO,
    S3,
    GlusterFS,
    NFS,
    Custom(String),
}

/// Storage pool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePool {
    /// Pool name
    pub name: String,
    /// Total capacity in bytes
    pub capacity: u64,
    /// Available capacity in bytes
    pub available: u64,
    /// Storage class
    #[serde(rename = "storageClass")]
    pub storage_class: StorageClass,
    /// Compression enabled
    #[serde(rename = "compressionEnabled")]
    pub compression_enabled: bool,
    /// Encryption enabled
    #[serde(rename = "encryptionEnabled")]
    pub encryption_enabled: bool,
    /// Replication enabled
    #[serde(rename = "replicationEnabled")]
    pub replication_enabled: bool,
}

impl StoragePool {
    /// Create a new storage pool
    pub fn new(name: impl Into<String>, capacity: u64) -> Self {
        Self {
            name: name.into(),
            capacity,
            available: capacity,
            storage_class: StorageClass::Standard,
            compression_enabled: false,
            encryption_enabled: false,
            replication_enabled: true,
        }
    }

    /// Set storage class
    pub fn set_storage_class(&mut self, class: StorageClass) -> &mut Self {
        self.storage_class = class;
        self
    }

    /// Set compression
    pub fn set_compression(&mut self, enabled: bool) -> &mut Self {
        self.compression_enabled = enabled;
        self
    }

    /// Set encryption
    pub fn set_encryption(&mut self, enabled: bool) -> &mut Self {
        self.encryption_enabled = enabled;
        self
    }

    /// Get utilization percentage
    pub fn get_utilization(&self) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            ((self.capacity - self.available) as f64 / self.capacity as f64) * 100.0
        }
    }
}

/// Storage class
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StorageClass {
    Standard,
    Premium,
    Ultra,
    Cold,
    Archive,
}

/// Storage status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageStatus {
    /// Health status
    pub health: StorageHealth,
    /// Total objects
    #[serde(rename = "totalObjects")]
    pub total_objects: i64,
    /// Total size in bytes
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    /// Active connections
    #[serde(rename = "activeConnections")]
    pub active_connections: i32,
    /// Last backup time
    #[serde(rename = "lastBackupTime")]
    pub last_backup_time: Option<String>,
}

/// Storage health
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StorageHealth {
    Healthy,
    Degraded,
    Unhealthy,
    Maintenance,
}

impl Default for StorageHealth {
    fn default() -> Self {
        StorageHealth::Healthy
    }
}

/// Snapshot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    /// Snapshot name
    pub name: String,
    /// Source pool
    #[serde(rename = "sourcePool")]
    pub source_pool: String,
    /// Snapshot type
    #[serde(rename = "snapshotType")]
    pub snapshot_type: SnapshotType,
    /// Retention policy
    #[serde(rename = "retentionPolicy")]
    pub retention_policy: RetentionPolicy,
    /// Schedule
    pub schedule: Option<Schedule>,
}

impl SnapshotConfig {
    /// Create a new snapshot configuration
    pub fn new(name: impl Into<String>, source_pool: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source_pool: source_pool.into(),
            snapshot_type: SnapshotType::Incremental,
            retention_policy: RetentionPolicy::default(),
            schedule: None,
        }
    }

    /// Set snapshot type
    pub fn set_snapshot_type(&mut self, snapshot_type: SnapshotType) -> &mut Self {
        self.snapshot_type = snapshot_type;
        self
    }

    /// Set retention policy
    pub fn set_retention_policy(&mut self, policy: RetentionPolicy) -> &mut Self {
        self.retention_policy = policy;
        self
    }

    /// Set schedule
    pub fn set_schedule(&mut self, schedule: Schedule) -> &mut Self {
        self.schedule = Some(schedule);
        self
    }
}

/// Snapshot type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotType {
    Full,
    Incremental,
    Differential,
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Maximum snapshots to keep
    #[serde(rename = "maxSnapshots")]
    pub max_snapshots: i32,
    /// Retention duration
    #[serde(rename = "retentionDuration")]
    pub retention_duration: String,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_snapshots: 10,
            retention_duration: "7d".to_string(),
        }
    }
}

/// Schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    /// Cron expression
    pub cron: String,
    /// Timezone
    pub timezone: String,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Backup name
    pub name: String,
    /// Source pools
    #[serde(rename = "sourcePools")]
    pub source_pools: Vec<String>,
    /// Destination
    pub destination: BackupDestination,
    /// Backup type
    #[serde(rename = "backupType")]
    pub backup_type: BackupType,
    /// Compression
    pub compression: bool,
    /// Encryption
    pub encryption: bool,
    /// Schedule
    pub schedule: Option<Schedule>,
}

impl BackupConfig {
    /// Create a new backup configuration
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source_pools: Vec::new(),
            destination: BackupDestination::Local {
                path: "/backups".to_string(),
            },
            backup_type: BackupType::Full,
            compression: true,
            encryption: false,
            schedule: None,
        }
    }

    /// Add source pool
    pub fn add_source_pool(&mut self, pool: impl Into<String>) -> &mut Self {
        self.source_pools.push(pool.into());
        self
    }

    /// Set destination
    pub fn set_destination(&mut self, destination: BackupDestination) -> &mut Self {
        self.destination = destination;
        self
    }

    /// Set backup type
    pub fn set_backup_type(&mut self, backup_type: BackupType) -> &mut Self {
        self.backup_type = backup_type;
        self
    }

    /// Set compression
    pub fn set_compression(&mut self, enabled: bool) -> &mut Self {
        self.compression = enabled;
        self
    }

    /// Set encryption
    pub fn set_encryption(&mut self, enabled: bool) -> &mut Self {
        self.encryption = enabled;
        self
    }

    /// Set schedule
    pub fn set_schedule(&mut self, schedule: Schedule) -> &mut Self {
        self.schedule = Some(schedule);
        self
    }
}

/// Backup destination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackupDestination {
    Local {
        path: String,
    },
    S3 {
        bucket: String,
        region: String,
        prefix: String,
    },
    NFS {
        server: String,
        path: String,
    },
    Custom {
        destination: String,
    },
}

/// Backup type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

/// Storage object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageObject {
    /// Object key
    pub key: String,
    /// Object size
    pub size: u64,
    /// Content type
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// Checksum
    pub checksum: String,
    /// Creation time
    #[serde(rename = "creationTime")]
    pub creation_time: String,
    /// Modification time
    #[serde(rename = "modificationTime")]
    pub modification_time: String,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl StorageObject {
    /// Create a new storage object
    pub fn new(key: impl Into<String>, size: u64) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            key: key.into(),
            size,
            content_type: "application/octet-stream".to_string(),
            checksum: String::new(),
            creation_time: now.clone(),
            modification_time: now,
            metadata: HashMap::new(),
        }
    }

    /// Set content type
    pub fn set_content_type(&mut self, content_type: impl Into<String>) -> &mut Self {
        self.content_type = content_type.into();
        self
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Data shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataShard {
    /// Shard ID
    #[serde(rename = "shardId")]
    pub shard_id: String,
    /// Range
    pub range: ShardRange,
    /// Replicas
    pub replicas: Vec<ShardReplica>,
}

/// Shard range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardRange {
    /// Start key
    #[serde(rename = "startKey")]
    pub start_key: String,
    /// End key
    #[serde(rename = "endKey")]
    pub end_key: String,
}

/// Shard replica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardReplica {
    /// Node ID
    #[serde(rename = "nodeId")]
    pub node_id: String,
    /// Status
    pub status: NodeStatus,
    /// Size
    pub size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_pool_creation() {
        let pool = StoragePool::new("test-pool", 1024 * 1024 * 1024);
        assert_eq!(pool.name, "test-pool");
        assert_eq!(pool.capacity, 1024 * 1024 * 1024);
    }

    #[test]
    fn test_storage_pool_utilization() {
        let mut pool = StoragePool::new("test-pool", 1000);
        pool.available = 500;
        
        let utilization = pool.get_utilization();
        assert_eq!(utilization, 50.0);
    }

    #[test]
    fn test_distributed_storage_validation() {
        let storage = DistributedStorage::new("test", StorageBackend::Ceph);
        assert!(storage.validate().is_ok());
        
        let mut storage = DistributedStorage::new("", StorageBackend::Ceph);
        assert!(storage.validate().is_err());
    }

    #[test]
    fn test_snapshot_config() {
        let snapshot = SnapshotConfig::new("test-snapshot", "pool1");
        assert_eq!(snapshot.name, "test-snapshot");
        assert_eq!(snapshot.source_pool, "pool1");
    }

    #[test]
    fn test_backup_config() {
        let backup = BackupConfig::new("test-backup");
        assert_eq!(backup.name, "test-backup");
    }
}