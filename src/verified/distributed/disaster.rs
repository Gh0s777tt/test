/// Disaster Recovery Module
/// 
/// This module provides disaster recovery capabilities for VantisOS,
/// supporting backup, restore, and failover to secondary sites.
/// 
/// Features:
/// - Automated backups
/// - Point-in-time recovery
/// - Site failover
/// - Replication monitoring
/// - Recovery testing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::DistributedError;

/// Disaster recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecovery {
    /// DR name
    pub name: String,
    /// Backup configuration
    #[serde(rename = "backupConfig")]
    pub backup_config: BackupConfig,
    /// Recovery configuration
    #[serde(rename = "recoveryConfig")]
    pub recovery_config: RecoveryConfig,
    /// Failover configuration
    #[serde(rename = "failoverConfig")]
    pub failover_config: FailoverConfig,
    /// Replication monitoring
    #[serde(rename = "replicationMonitoring")]
    pub replication_monitoring: ReplicationMonitoring,
    /// DR status
    pub status: DisasterRecoveryStatus,
}

impl DisasterRecovery {
    /// Create a new DR configuration
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            backup_config: BackupConfig::default(),
            recovery_config: RecoveryConfig::default(),
            failover_config: FailoverConfig::default(),
            replication_monitoring: ReplicationMonitoring::default(),
            status: DisasterRecoveryStatus::default(),
        }
    }

    /// Set backup configuration
    pub fn set_backup_config(&mut self, config: BackupConfig) -> &mut Self {
        self.backup_config = config;
        self
    }

    /// Set recovery configuration
    pub fn set_recovery_config(&mut self, config: RecoveryConfig) -> &mut Self {
        self.recovery_config = config;
        self
    }

    /// Set failover configuration
    pub fn set_failover_config(&mut self, config: FailoverConfig) -> &mut Self {
        self.failover_config = config;
        self
    }

    /// Validate DR configuration
    pub fn validate(&self) -> Result<(), DistributedError> {
        if self.name.is_empty() {
            return Err(DistributedError::DisasterRecoveryError("DR name is required".to_string()));
        }
        Ok(())
    }
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enabled
    pub enabled: bool,
    /// Schedule
    pub schedule: BackupSchedule,
    /// Storage location
    #[serde(rename = "storageLocation")]
    pub storage_location: StorageLocation,
    /// Retention policy
    #[serde(rename = "retentionPolicy")]
    pub retention_policy: RetentionPolicy,
    /// Compression
    pub compression: bool,
    /// Encryption
    pub encryption: bool,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            schedule: BackupSchedule::default(),
            storage_location: StorageLocation::Local {
                path: "/backups".to_string(),
            },
            retention_policy: RetentionPolicy::default(),
            compression: true,
            encryption: false,
        }
    }
}

/// Backup schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSchedule {
    /// Type
    #[serde(rename = "scheduleType")]
    pub schedule_type: ScheduleType,
    /// Cron expression
    pub cron: String,
    /// Timezone
    pub timezone: String,
}

impl Default for BackupSchedule {
    fn default() -> Self {
        Self {
            schedule_type: ScheduleType::Automated,
            cron: "0 2 * * *".to_string(), // Daily at 2 AM
            timezone: "UTC".to_string(),
        }
    }
}

/// Schedule type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ScheduleType {
    Manual,
    Automated,
}

/// Storage location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    Local {
        path: String,
    },
    S3 {
        bucket: String,
        region: String,
        prefix: String,
    },
    Azure {
        container: String,
        storage_account: String,
    },
    GCS {
        bucket: String,
    },
    NFS {
        server: String,
        path: String,
    },
}

/// Retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Maximum backups to keep
    #[serde(rename = "maxBackups")]
    pub max_backups: i32,
    /// Retention duration
    #[serde(rename = "retentionDuration")]
    pub retention_duration: String,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            max_backups: 30,
            retention_duration: "30d".to_string(),
        }
    }
}

/// Recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryConfig {
    /// Recovery point objective in seconds (RPO)
    #[serde(rename = "rpoSeconds")]
    pub rpo_seconds: i32,
    /// Recovery time objective in seconds (RTO)
    #[serde(rename = "rtoSeconds")]
    pub rto_seconds: i32,
    /// Point in time recovery
    #[serde(rename = "pointInTimeRecovery")]
    pub point_in_time_recovery: bool,
    /// Verification enabled
    #[serde(rename = "verificationEnabled")]
    pub verification_enabled: bool,
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            rpo_seconds: 3600, // 1 hour
            rto_seconds: 1800, // 30 minutes
            point_in_time_recovery: true,
            verification_enabled: true,
        }
    }
}

/// Failover configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Enabled
    pub enabled: bool,
    /// Failover mode
    #[serde(rename = "failoverMode")]
    pub failover_mode: FailoverMode,
    /// Automatic failover
    #[serde(rename = "automaticFailover")]
    pub automatic_failover: bool,
    /// Primary site
    #[serde(rename = "primarySite")]
    pub primary_site: Site,
    /// Secondary site
    #[serde(rename = "secondarySite")]
    pub secondary_site: Site,
    /// Failover trigger
    #[serde(rename = "failoverTrigger")]
    pub failover_trigger: FailoverTrigger,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            failover_mode: FailoverMode::ActivePassive,
            automatic_failover: true,
            primary_site: Site {
                name: "primary".to_string(),
                location: "us-east-1".to_string(),
                status: SiteStatus::Active,
            },
            secondary_site: Site {
                name: "secondary".to_string(),
                location: "us-west-2".to_string(),
                status: SiteStatus::Standby,
            },
            failover_trigger: FailoverTrigger::default(),
        }
    }
}

/// Failover mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FailoverMode {
    ActivePassive,
    ActiveActive,
    GeoDNS,
}

/// Site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    /// Site name
    pub name: String,
    /// Location
    pub location: String,
    /// Status
    pub status: SiteStatus,
}

/// Site status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SiteStatus {
    Active,
    Standby,
    Offline,
    Maintenance,
}

/// Failover trigger
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverTrigger {
    /// Failure threshold
    #[serde(rename = "failureThreshold")]
    pub failure_threshold: i32,
    /// Trigger conditions
    pub conditions: Vec<TriggerCondition>,
}

impl Default for FailoverTrigger {
    fn default() -> Self {
        Self {
            failure_threshold: 3,
            conditions: vec![
                TriggerCondition {
                    condition_type: TriggerConditionType::HealthCheck,
                    threshold: 50.0,
                },
            ],
        }
    }
}

/// Trigger condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCondition {
    /// Condition type
    #[serde(rename = "conditionType")]
    pub condition_type: TriggerConditionType,
    /// Threshold
    pub threshold: f64,
}

/// Trigger condition type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TriggerConditionType {
    HealthCheck,
    Latency,
    ErrorRate,
    Manual,
}

/// Replication monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationMonitoring {
    /// Enabled
    pub enabled: bool,
    /// Check interval
    #[serde(rename = "checkIntervalSeconds")]
    pub check_interval_seconds: i32,
    /// Replication lag threshold
    #[serde(rename = "replicationLagThresholdMs")]
    pub replication_lag_threshold_ms: i32,
    /// Alert threshold
    #[serde(rename = "alertThresholdPercent")]
    pub alert_threshold_percent: f64,
    /// Replication status
    pub status: ReplicationStatus,
}

impl Default for ReplicationMonitoring {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval_seconds: 60,
            replication_lag_threshold_ms: 5000,
            alert_threshold_percent: 80.0,
            status: ReplicationStatus {
                lag_ms: 0,
                throughput_mb_s: 0.0,
                healthy: true,
                last_sync: None,
            },
        }
    }
}

/// Replication status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatus {
    /// Replication lag in milliseconds
    #[serde(rename = "lagMs")]
    pub lag_ms: i32,
    /// Throughput in MB/s
    #[serde(rename = "throughputMbS")]
    pub throughput_mb_s: f64,
    /// Healthy
    pub healthy: bool,
    /// Last sync
    #[serde(rename = "lastSync")]
    pub last_sync: Option<String>,
}

/// Disaster recovery status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisasterRecoveryStatus {
    /// Current status
    pub status: DRStatus,
    /// Last backup time
    #[serde(rename = "lastBackupTime")]
    pub last_backup_time: Option<String>,
    /// Next backup time
    #[serde(rename = "nextBackupTime")]
    pub next_backup_time: Option<String>,
    /// Last successful backup
    #[serde(rename = "lastSuccessfulBackup")]
    pub last_successful_backup: Option<String>,
    /// Active site
    #[serde(rename = "activeSite")]
    pub active_site: Option<String>,
}

/// DR status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DRStatus {
    Active,
    BackingUp,
    Restoring,
    FailingOver,
    Error,
}

impl Default for DRStatus {
    fn default() -> Self {
        DRStatus::Active
    }
}

/// Backup job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJob {
    /// Job ID
    pub id: String,
    /// Job type
    #[serde(rename = "jobType")]
    pub job_type: BackupJobType,
    /// Status
    pub status: JobStatus,
    /// Progress
    pub progress: f64,
    /// Start time
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// End time
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    /// Size in bytes
    pub size: u64,
    /// Error message
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

/// Backup job type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BackupJobType {
    Full,
    Incremental,
    Differential,
}

/// Job status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Recovery job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryJob {
    /// Job ID
    pub id: String,
    /// Source backup
    #[serde(rename = "sourceBackup")]
    pub source_backup: String,
    /// Target site
    #[serde(rename = "targetSite")]
    pub target_site: String,
    /// Status
    pub status: JobStatus,
    /// Progress
    pub progress: f64,
    /// Point in time
    #[serde(rename = "pointInTime")]
    pub point_in_time: Option<String>,
    /// Start time
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// End time
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    /// Error message
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
}

/// DR test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryTest {
    /// Test ID
    pub id: String,
    /// Test name
    pub name: String,
    /// Test type
    #[serde(rename = "testType")]
    pub test_type: TestType,
    /// Status
    pub status: TestStatus,
    /// Results
    pub results: Option<TestResults>,
    /// Start time
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    /// End time
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
}

/// Test type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TestType {
    Backup,
    Restore,
    Failover,
    Replication,
}

/// Test status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TestStatus {
    Scheduled,
    Running,
    Passed,
    Failed,
    Cancelled,
}

/// Test results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    /// Passed
    pub passed: bool,
    /// Duration in seconds
    pub duration_seconds: i32,
    /// Details
    pub details: HashMap<String, String>,
    /// Recovery time
    #[serde(rename = "recoveryTimeSeconds")]
    pub recovery_time_seconds: Option<i32>,
    /// Data integrity
    #[serde(rename = "dataIntegrityCheck")]
    pub data_integrity_check: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disaster_recovery_creation() {
        let dr = DisasterRecovery::new("test-dr");
        assert_eq!(dr.name, "test-dr");
    }

    #[test]
    fn test_backup_schedule() {
        let schedule = BackupSchedule::default();
        assert_eq!(schedule.cron, "0 2 * * *");
        assert_eq!(schedule.timezone, "UTC");
    }

    #[test]
    fn test_failover_config() {
        let config = FailoverConfig::default();
        assert!(config.enabled);
        assert!(config.automatic_failover);
        assert_eq!(config.failover_mode, FailoverMode::ActivePassive);
    }

    #[test]
    fn test_recovery_config() {
        let config = RecoveryConfig::default();
        assert_eq!(config.rpo_seconds, 3600);
        assert_eq!(config.rto_seconds, 1800);
    }

    #[test]
    fn test_replication_monitoring() {
        let monitoring = ReplicationMonitoring::default();
        assert!(monitoring.enabled);
        assert_eq!(monitoring.check_interval_seconds, 60);
    }

    #[test]
    fn test_dr_validation() {
        let dr = DisasterRecovery::new("test-dr");
        assert!(dr.validate().is_ok());
        
        let dr = DisasterRecovery::new("");
        assert!(dr.validate().is_err());
    }
}