//! Google Cloud Platform (GCP) Integration
//! 
//! This module provides comprehensive integration with Google Cloud Platform services,
//! including Compute Engine, Cloud Storage, VPC networks, and Google Kubernetes Engine (GKE).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::sync::RwLock;
use std::sync::Arc;
use chrono::{DateTime, Utc};

use super::{CloudError, CloudResult, CloudResource, CloudCredentials};

// ============================================================================
// GCP Configuration Types
// ============================================================================

/// GCP cloud configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpConfig {
    /// GCP project ID
    pub project_id: String,
    /// GCP region (e.g., "us-central1", "europe-west4")
    pub region: String,
    /// GCP zone (e.g., "us-central1-a")
    pub zone: String,
    /// Authentication credentials
    pub credentials: GcpCredentials,
    /// Default labels for all resources
    pub default_labels: HashMap<String, String>,
    /// Network configuration
    pub network_config: GcpNetworkConfig,
}

impl Default for GcpConfig {
    fn default() -> Self {
        Self {
            project_id: String::new(),
            region: "us-central1".to_string(),
            zone: "us-central1-a".to_string(),
            credentials: GcpCredentials::default(),
            default_labels: HashMap::new(),
            network_config: GcpNetworkConfig::default(),
        }
    }
}

/// GCP network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpNetworkConfig {
    /// Default network name
    pub network: String,
    /// Default subnet region
    pub subnet_region: String,
    /// Use private IP access
    pub private_ip_access: bool,
}

impl Default for GcpNetworkConfig {
    fn default() -> Self {
        Self {
            network: "default".to_string(),
            subnet_region: "us-central1".to_string(),
            private_ip_access: false,
        }
    }
}

// ============================================================================
// GCP Credentials
// ============================================================================

/// GCP authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpCredentials {
    /// Service account email
    pub client_email: String,
    /// Client ID
    pub client_id: String,
    /// Private key (PEM format)
    pub private_key: String,
    /// Project ID
    pub project_id: String,
    /// Token URI
    pub token_uri: String,
    /// Access token (cached)
    pub access_token: Option<GcpToken>,
}

impl Default for GcpCredentials {
    fn default() -> Self {
        Self {
            client_email: String::new(),
            client_id: String::new(),
            private_key: String::new(),
            project_id: String::new(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            access_token: None,
        }
    }
}

/// GCP OAuth2 access token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcpToken {
    /// Access token value
    pub access_token: String,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Expires in seconds
    pub expires_in: i64,
    /// Token expiration time
    pub expiry: DateTime<Utc>,
    /// Refresh token (if available)
    pub refresh_token: Option<String>,
}

impl GcpCredentials {
    /// Create new GCP credentials
    pub fn new(
        client_email: impl Into<String>,
        client_id: impl Into<String>,
        private_key: impl Into<String>,
        project_id: impl Into<String>,
    ) -> Self {
        Self {
            client_email: client_email.into(),
            client_id: client_id.into(),
            private_key: private_key.into(),
            project_id: project_id.into(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            access_token: None,
        }
    }

    /// Check if the current token is expired
    pub fn is_token_expired(&self) -> bool {
        match &self.access_token {
            Some(token) => token.expiry < Utc::now() + chrono::Duration::seconds(60),
            None => true,
        }
    }

    /// Create from JSON (service account key format)
    pub fn from_json(json: serde_json::Value) -> CloudResult<Self> {
        Ok(Self {
            client_email: json["client_email"]
                .as_str()
                .ok_or_else(|| CloudError::AuthenticationFailed("Missing client_email".to_string()))?
                .to_string(),
            client_id: json["client_id"]
                .as_str()
                .ok_or_else(|| CloudError::AuthenticationFailed("Missing client_id".to_string()))?
                .to_string(),
            private_key: json["private_key"]
                .as_str()
                .ok_or_else(|| CloudError::AuthenticationFailed("Missing private_key".to_string()))?
                .to_string(),
            project_id: json["project_id"]
                .as_str()
                .ok_or_else(|| CloudError::AuthenticationFailed("Missing project_id".to_string()))?
                .to_string(),
            token_uri: json["token_uri"]
                .as_str()
                .unwrap_or("https://oauth2.googleapis.com/token")
                .to_string(),
            access_token: None,
        })
    }
}

impl CloudCredentials for GcpCredentials {
    fn provider(&self) -> super::CloudProvider {
        super::CloudProvider::Gcp
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// ============================================================================
// GCP Compute Engine
// ============================================================================

/// GCP Compute Instance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeInstanceConfig {
    /// Instance name
    pub name: String,
    /// Zone
    pub zone: String,
    /// Machine type (e.g., "e2-medium", "n1-standard-4")
    pub machine_type: MachineType,
    /// Boot disk configuration
    pub boot_disk: BootDiskConfig,
    /// Additional disks
    pub disks: Vec<AttachedDisk>,
    /// Network interfaces
    pub network_interfaces: Vec<NetworkInterface>,
    /// Service accounts
    pub service_accounts: Vec<ServiceAccount>,
    /// Metadata
    pub metadata: Metadata,
    /// Scheduling options
    pub scheduling: Scheduling,
    /// Enable secure boot
    pub shielded_instance_config: Option<ShieldedInstanceConfig>,
    /// Labels
    pub labels: HashMap<String, String>,
    /// Tags
    pub tags: Vec<String>,
}

/// Machine type specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineType {
    /// Machine type name
    pub name: String,
    /// Number of vCPUs
    pub guest_cpus: i32,
    /// Memory in MB
    pub memory_mb: i64,
    /// Maximum persistent disks
    pub maximum_persistent_disks: i32,
    /// Image family (for preemptible)
    pub is_shared_cpu: bool,
}

impl MachineType {
    /// E2 series - cost-effective
    pub fn e2_medium() -> Self {
        Self {
            name: "e2-medium".to_string(),
            guest_cpus: 2,
            memory_mb: 4096,
            maximum_persistent_disks: 128,
            is_shared_cpu: true,
        }
    }

    pub fn e2_standard_4() -> Self {
        Self {
            name: "e2-standard-4".to_string(),
            guest_cpus: 4,
            memory_mb: 16384,
            maximum_persistent_disks: 128,
            is_shared_cpu: false,
        }
    }

    /// N1 series - general purpose
    pub fn n1_standard_4() -> Self {
        Self {
            name: "n1-standard-4".to_string(),
            guest_cpus: 4,
            memory_mb: 15360,
            maximum_persistent_disks: 128,
            is_shared_cpu: false,
        }
    }

    /// N2 series - improved performance
    pub fn n2_standard_4() -> Self {
        Self {
            name: "n2-standard-4".to_string(),
            guest_cpus: 4,
            memory_mb: 16384,
            maximum_persistent_disks: 128,
            is_shared_cpu: false,
        }
    }

    /// C2 series - compute-optimized
    pub fn c2_standard_4() -> Self {
        Self {
            name: "c2-standard-4".to_string(),
            guest_cpus: 4,
            memory_mb: 16384,
            maximum_persistent_disks: 128,
            is_shared_cpu: false,
        }
    }

    /// T2A series - Arm-based
    pub fn t2a_standard_4() -> Self {
        Self {
            name: "t2a-standard-4".to_string(),
            guest_cpus: 4,
            memory_mb: 16384,
            maximum_persistent_disks: 128,
            is_shared_cpu: false,
        }
    }
}

/// Boot disk configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootDiskConfig {
    /// Initialize parameters
    pub initialize_params: DiskInitializeParams,
    /// Auto-delete
    pub auto_delete: bool,
    /// Boot flag
    pub boot: bool,
}

/// Disk initialization parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInitializeParams {
    /// Disk name
    pub disk_name: String,
    /// Source image
    pub source_image: String,
    /// Disk type
    pub disk_type: DiskType,
    /// Disk size in GB
    pub disk_size_gb: i64,
}

impl DiskInitializeParams {
    /// Ubuntu 20.04 LTS
    pub fn ubuntu_20_04() -> Self {
        Self {
            disk_name: String::new(),
            source_image: "projects/ubuntu-os-cloud/global/images/ubuntu-2004-lts-amd64-v20230801".to_string(),
            disk_type: DiskType::PdStandard,
            disk_size_gb: 20,
        }
    }

    /// Ubuntu 22.04 LTS
    pub fn ubuntu_22_04() -> Self {
        Self {
            disk_name: String::new(),
            source_image: "projects/ubuntu-os-cloud/global/images/ubuntu-2204-jammy-v20230804".to_string(),
            disk_type: DiskType::PdStandard,
            disk_size_gb: 20,
        }
    }

    /// Debian 11
    pub fn debian_11() -> Self {
        Self {
            disk_name: String::new(),
            source_image: "projects/debian-cloud/global/images/debian-11-bullseye-v20230814".to_string(),
            disk_type: DiskType::PdStandard,
            disk_size_gb: 20,
        }
    }

    /// Windows Server 2019
    pub fn windows_server_2019() -> Self {
        Self {
            disk_name: String::new(),
            source_image: "projects/windows-cloud/global/images/windows-server-2019-dc-v20230814".to_string(),
            disk_type: DiskType::PdStandard,
            disk_size_gb: 50,
        }
    }

    /// Windows Server 2022
    pub fn windows_server_2022() -> Self {
        Self {
            disk_name: String::new(),
            source_image: "projects/windows-cloud/global/images/windows-server-2022-dc-v20230814".to_string(),
            disk_type: DiskType::PdStandard,
            disk_size_gb: 50,
        }
    }
}

/// Disk types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiskType {
    PdStandard,  // Standard persistent disk
    PdSsd,       // SSD persistent disk
    PdBalanced,  // Balanced persistent disk
    PdExtreme,   // Extreme persistent disk
}

impl DiskType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiskType::PdStandard => "pd-standard",
            DiskType::PdSsd => "pd-ssd",
            DiskType::PdBalanced => "pd-balanced",
            DiskType::PdExtreme => "pd-extreme",
        }
    }
}

/// Attached disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachedDisk {
    /// Disk type
    pub kind: DiskKind,
    /// Mode (READ_ONLY or READ_WRITE)
    pub mode: DiskMode,
    /// Source
    pub source: String,
    /// Device name
    pub device_name: String,
    /// Auto-delete
    pub auto_delete: bool,
    /// Initialize params (for new disks)
    pub initialize_params: Option<DiskInitializeParams>,
}

/// Disk kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiskKind {
    Persistent,
    Scratch,
}

impl DiskKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiskKind::Persistent => "PERSISTENT",
            DiskKind::Scratch => "SCRATCH",
        }
    }
}

/// Disk mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiskMode {
    ReadOnly,
    ReadWrite,
}

impl DiskMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            DiskMode::ReadOnly => "READ_ONLY",
            DiskMode::ReadWrite => "READ_WRITE",
        }
    }
}

/// Network interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// Network URL
    pub network: String,
    /// Subnetwork URL
    pub subnetwork: String,
    /// Access config (external IPs)
    pub access_configs: Vec<AccessConfig>,
    /// Alias IP ranges
    pub alias_ip_ranges: Vec<AliasIpRange>,
    /// Stack type
    pub stack_type: NetworkStackType,
}

/// Access configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessConfig {
    /// Type (ONE_TO_ONE_NAT)
    pub kind: String,
    /// Network tier
    pub network_tier: NetworkTier,
    /// Name
    pub name: String,
    /// NAT IP address
    pub nat_ip: Option<String>,
}

/// Network tier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkTier {
    Premium,
    Standard,
}

impl NetworkTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            NetworkTier::Premium => "PREMIUM",
            NetworkTier::Standard => "STANDARD",
        }
    }
}

/// Alias IP range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasIpRange {
    /// IP CIDR range
    pub ip_cidr_range: String,
    /// Subnetwork range name
    pub subnetwork_range_name: String,
}

/// Network stack type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkStackType {
    Ipv4,
    Ipv4Only,
    Ipv4Ipv6,
}

impl NetworkStackType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NetworkStackType::Ipv4 => "IPV4_ONLY",
            NetworkStackType::Ipv4Only => "IPV4_ONLY",
            NetworkStackType::Ipv4Ipv6 => "IPV4_IPV6",
        }
    }
}

/// Service account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAccount {
    /// Email
    pub email: String,
    /// Scopes
    pub scopes: Vec<String>,
}

impl ServiceAccount {
    /// Default compute scope
    pub fn default_scopes() -> Vec<String> {
        vec![
            "https://www.googleapis.com/auth/compute".to_string(),
            "https://www.googleapis.com/auth/devstorage.read_only".to_string(),
            "https://www.googleapis.com/auth/logging.write".to_string(),
            "https://www.googleapis.com/auth/monitoring.write".to_string(),
        ]
    }

    /// Full access scopes
    pub fn full_access_scopes() -> Vec<String> {
        vec![
            "https://www.googleapis.com/auth/compute".to_string(),
            "https://www.googleapis.com/auth/devstorage.read_write".to_string(),
            "https://www.googleapis.com/auth/logging.write".to_string(),
            "https://www.googleapis.com/auth/monitoring.write".to_string(),
            "https://www.googleapis.com/auth/servicecontrol".to_string(),
            "https://www.googleapis.com/auth/service.management.readonly".to_string(),
            "https://www.googleapis.com/auth/trace.append".to_string(),
        ]
    }
}

/// Instance metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Key-value pairs
    pub items: Vec<MetadataItem>,
}

/// Metadata item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataItem {
    /// Key
    pub key: String,
    /// Value
    pub value: String,
}

impl Metadata {
    /// Create metadata with SSH key
    pub fn with_ssh_key(username: &str, public_key: &str) -> Self {
        Self {
            items: vec![MetadataItem {
                key: "ssh-keys".to_string(),
                value: format!("{}:{}", username, public_key),
            }],
        }
    }

    /// Create metadata with startup script
    pub fn with_startup_script(script: &str) -> Self {
        Self {
            items: vec![MetadataItem {
                key: "startup-script".to_string(),
                value: script.to_string(),
            }],
        }
    }
}

/// Scheduling options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scheduling {
    /// Preemptible VM
    pub preemptible: bool,
    /// Automatic restart
    pub automatic_restart: bool,
    /// On host maintenance
    pub on_host_maintenance: OnHostMaintenance,
    /// Node affinity
    pub node_affinities: Vec<NodeAffinity>,
}

/// On host maintenance behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnHostMaintenance {
    Migrate,
    Terminate,
}

impl OnHostMaintenance {
    pub fn as_str(&self) -> &'static str {
        match self {
            OnHostMaintenance::Migrate => "MIGRATE",
            OnHostMaintenance::Terminate => "TERMINATE",
        }
    }
}

/// Node affinity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAffinity {
    /// Key
    pub key: String,
    /// Operator
    pub operator: String,
    /// Values
    pub values: Vec<String>,
}

/// Shielded instance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldedInstanceConfig {
    /// Enable secure boot
    pub enable_secure_boot: bool,
    /// Enable vTPM
    pub enable_vtpm: bool,
    /// Enable integrity monitoring
    pub enable_integrity_monitoring: bool,
}

// ============================================================================
// GCP Cloud Storage
// ============================================================================

/// GCP Storage Bucket configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBucketConfig {
    /// Bucket name (must be globally unique)
    pub name: String,
    /// Location (region or multi-region)
    pub location: String,
    /// Storage class
    pub storage_class: StorageClass,
    /// Uniform bucket-level access
    pub uniform_bucket_level_access: bool,
    /// Public access prevention
    pub public_access_prevention: PublicAccessPrevention,
    /// Versioning enabled
    pub versioning: Option<VersioningConfig>,
    /// Lifecycle configuration
    pub lifecycle: Option<LifecycleConfig>,
    /// Logging configuration
    pub logging: Option<LoggingConfig>,
    /// Labels
    pub labels: HashMap<String, String>,
}

/// Storage classes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageClass {
    Standard,
    Nearline,
    Coldline,
    Archive,
}

impl StorageClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageClass::Standard => "STANDARD",
            StorageClass::Nearline => "NEARLINE",
            StorageClass::Coldline => "COLDLINE",
            StorageClass::Archive => "ARCHIVE",
        }
    }
}

/// Public access prevention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicAccessPrevention {
    Inherited,
    Enforced,
}

impl PublicAccessPrevention {
    pub fn as_str(&self) -> &'static str {
        match self {
            PublicAccessPrevention::Inherited => "inherited",
            PublicAccessPrevention::Enforced => "enforced",
        }
    }
}

/// Versioning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersioningConfig {
    /// Enabled
    pub enabled: bool,
}

/// Lifecycle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleConfig {
    /// Rules
    pub rules: Vec<LifecycleRule>,
}

/// Lifecycle rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleRule {
    /// Action
    pub action: LifecycleAction,
    /// Condition
    pub condition: LifecycleCondition,
}

/// Lifecycle action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleAction {
    /// Type (Delete, SetStorageClass)
    pub action_type: String,
    /// Storage class (for SetStorageClass)
    pub storage_class: Option<String>,
}

/// Lifecycle condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleCondition {
    /// Age in days
    pub age: Option<i32>,
    /// Created before
    pub created_before: Option<String>,
    /// Number of newer versions
    pub num_newer_versions: Option<i32>,
    /// Is live
    pub is_live: Option<bool>,
    /// Matches storage class
    pub matches_storage_class: Vec<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log bucket
    pub log_bucket: String,
    /// Log object prefix
    pub log_object_prefix: Option<String>,
}

// ============================================================================
// GCP VPC Networks
// ============================================================================

/// VPC network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcNetworkConfig {
    /// Network name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Auto create subnetworks
    pub auto_create_subnetworks: bool,
    /// Subnetworks (when auto_create_subnetworks is false)
    pub subnetworks: Vec<Subnetwork>,
    /// Routing configuration
    pub routing_config: Option<RoutingConfig>,
    /// MTU
    pub mtu: Option<i32>,
}

/// Subnetwork configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subnetwork {
    /// Name
    pub name: String,
    /// Region
    pub region: String,
    /// IP CIDR range
    pub ip_cidr_range: String,
    /// Description
    pub description: Option<String>,
    /// Private IP Google Access
    pub private_ip_google_access: bool,
    /// Enable flow logs
    pub enable_flow_logs: bool,
    /// Secondary IP ranges
    pub secondary_ip_ranges: Vec<SecondaryIpRange>,
}

/// Secondary IP range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryIpRange {
    /// Range name
    pub range_name: String,
    /// IP CIDR range
    pub ip_cidr_range: String,
}

/// Routing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// BGP ASN (if using Cloud Router)
    pub bgp: Option<BgpConfig>,
}

/// BGP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BgpConfig {
    /// ASN
    pub asn: i64,
}

/// Firewall rule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRuleConfig {
    /// Rule name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Network
    pub network: String,
    /// Direction (INGRESS or EGRESS)
    pub direction: FirewallDirection,
    /// Priority (0-65535, lower is higher priority)
    pub priority: i32,
    /// Action (ALLOW or DENY)
    pub action: FirewallAction,
    /// Source ranges (for INGRESS)
    pub source_ranges: Vec<String>,
    /// Source tags
    pub source_tags: Vec<String>,
    /// Source service accounts
    pub source_service_accounts: Vec<String>,
    /// Destination ranges (for EGRESS)
    pub destination_ranges: Vec<String>,
    /// Allowed (for ALLOW action)
    pub allowed: Vec<FirewallAllowed>,
    /// Denied (for DENY action)
    pub denied: Vec<FirewallDenied>,
    /// Target tags
    pub target_tags: Vec<String>,
    /// Target service accounts
    pub target_service_accounts: Vec<String>,
}

/// Firewall direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FirewallDirection {
    Ingress,
    Egress,
}

impl FirewallDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            FirewallDirection::Ingress => "INGRESS",
            FirewallDirection::Egress => "EGRESS",
        }
    }
}

/// Firewall action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FirewallAction {
    Allow,
    Deny,
}

impl FirewallAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            FirewallAction::Allow => "ALLOW",
            FirewallAction::Deny => "DENY",
        }
    }
}

/// Firewall allowed rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallAllowed {
    /// IP protocol (tcp, udp, icmp, etc.)
    pub ip_protocol: String,
    /// Ports
    pub ports: Vec<String>,
}

/// Firewall denied rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallDenied {
    /// IP protocol
    pub ip_protocol: String,
    /// Ports
    pub ports: Vec<String>,
}

// ============================================================================
// Google Kubernetes Engine (GKE)
// ============================================================================

/// GKE cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GkeClusterConfig {
    /// Cluster name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Location (region or zone)
    pub location: String,
    /// Initial node count (for zonal clusters)
    pub initial_node_count: i32,
    /// Node pools
    pub node_pools: Vec<NodePoolConfig>,
    /// Network configuration
    pub network_config: GkeNetworkConfig,
    /// IP allocation policy
    pub ip_allocation_policy: Option<IpAllocationPolicy>,
    /// Master auth
    pub master_auth: Option<MasterAuth>,
    /// Add-ons
    pub addons_config: Option<AddonsConfig>,
    /// Maintenance policy
    pub maintenance_policy: Option<MaintenancePolicy>,
    /// Release channel
    pub release_channel: Option<ReleaseChannel>,
    /// Private cluster config
    pub private_cluster_config: Option<PrivateClusterConfig>,
    /// Database encryption
    pub database_encryption: Option<DatabaseEncryption>,
    /// Workload identity config
    pub workload_identity_config: Option<WorkloadIdentityConfig>,
    /// Labels
    pub labels: HashMap<String, String>,
}

/// Node pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePoolConfig {
    /// Pool name
    pub name: String,
    /// Node version
    pub version: Option<String>,
    /// Initial node count
    pub initial_node_count: i32,
    /// Machine type
    pub machine_type: String,
    /// Disk size in GB
    pub disk_size_gb: i32,
    /// Disk type
    pub disk_type: String,
    /// Image type
    pub image_type: NodeImageType,
    /// Autoscaling
    pub autoscaling: Option<AutoscalingConfig>,
    /// Management
    pub management: Option<NodeManagement>,
    /// Config (scheduling, upgrade settings)
    pub config: Option<NodeConfig>,
    /// Labels
    pub labels: HashMap<String, String>,
    /// Taints
    pub taints: Vec<NodeTaint>,
    /// Tags
    pub tags: Vec<String>,
    /// Accelerators
    pub accelerators: Vec<AcceleratorConfig>,
}

/// Node image types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeImageType {
    ContainerOptimized,
    CosContainerd,
    Ubuntu,
    UbuntuContainerd,
}

impl NodeImageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeImageType::ContainerOptimized => "COS",
            NodeImageType::CosContainerd => "cos_containerd",
            NodeImageType::Ubuntu => "UBUNTU",
            NodeImageType::UbuntuContainerd => "ubuntu_containerd",
        }
    }
}

/// Autoscaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoscalingConfig {
    /// Enable autoscaling
    pub enabled: bool,
    /// Min node count
    pub min_node_count: i32,
    /// Max node count
    pub max_node_count: i32,
}

/// Node management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeManagement {
    /// Auto-repair
    pub auto_repair: bool,
    /// Auto-upgrade
    pub auto_upgrade: bool,
    /// Upgrade options
    pub upgrade_options: Option<UpgradeOptions>,
}

/// Upgrade options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeOptions {
    /// Auto upgrade start time
    pub auto_upgrade_start_time: Option<String>,
    /// Description
    pub description: Option<String>,
}

/// Node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Preemptible
    pub preemptible: bool,
    /// Machine type (image config)
    pub image_type: Option<String>,
    /// Disk type
    pub disk_type: Option<String>,
    /// Labels
    pub labels: Option<HashMap<String, String>>,
    /// Opaque metadata
    pub metadata: Option<HashMap<String, String>>,
    /// Shielded instance config
    pub shielded_instance_config: Option<ShieldedInstanceConfig>,
}

/// Node taint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTaint {
    /// Key
    pub key: String,
    /// Value
    pub value: String,
    /// Effect
    pub effect: TaintEffect,
}

/// Taint effects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaintEffect {
    NoSchedule,
    PreferNoSchedule,
    NoExecute,
}

impl TaintEffect {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaintEffect::NoSchedule => "NO_SCHEDULE",
            TaintEffect::PreferNoSchedule => "PREFER_NO_SCHEDULE",
            TaintEffect::NoExecute => "NO_EXECUTE",
        }
    }
}

/// Accelerator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceleratorConfig {
    /// Accelerator type (e.g., "nvidia-tesla-k80")
    pub accelerator_type: String,
    /// Accelerator count
    pub accelerator_count: i32,
}

/// GKE network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GkeNetworkConfig {
    /// Network
    pub network: String,
    /// Subnetwork
    pub subnetwork: String,
    /// Cluster IPv4 CIDR
    pub cluster_ipv4_cidr: Option<String>,
    /// Services IPv4 CIDR
    pub services_ipv4_cidr: Option<String>,
    /// Private IPv4 CIDR
    pub private_ipv4_cidr: Option<String>,
}

/// IP allocation policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAllocationPolicy {
    /// Cluster IPv4 CIDR block
    pub cluster_ipv4_cidr_block: Option<String>,
    /// Services IPv4 CIDR block
    pub services_ipv4_cidr_block: Option<String>,
    /// Use IP aliases
    pub use_ip_aliases: bool,
}

/// Master authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterAuth {
    /// Cluster CA certificate
    pub cluster_ca_certificate: Option<String>,
    /// Username
    pub username: Option<String>,
    /// Password
    pub password: Option<String>,
    /// Client certificate config
    pub client_certificate_config: Option<ClientCertificateConfig>,
}

/// Client certificate configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCertificateConfig {
    /// Issue client certificate
    pub issue_client_certificate: bool,
}

/// Add-ons configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonsConfig {
    /// HTTP load balancing
    pub http_load_balancing: Option<bool>,
    /// Horizontal Pod Autoscaling
    pub horizontal_pod_autoscaling: Option<bool>,
    /// Network policy
    pub network_policy_config: Option<NetworkPolicyConfig>,
    /// Cloud Run
    pub cloud_run_config: Option<CloudRunConfig>,
    /// DNS cache
    pub dns_cache_config: Option<DnsCacheConfig>,
    /// Config Connector
    pub config_connector_config: Option<ConfigConnectorConfig>,
    /// GKE Backup
    pub backup_agent_config: Option<bool>,
}

/// Network policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicyConfig {
    /// Disabled
    pub disabled: bool,
}

/// Cloud Run configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudRunConfig {
    /// Disabled
    pub disabled: bool,
    /// Load balancer type
    pub load_balancer_type: Option<String>,
}

/// DNS cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsCacheConfig {
    /// Enabled
    pub enabled: bool,
}

/// Config Connector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigConnectorConfig {
    /// Enabled
    pub enabled: bool,
}

/// Maintenance policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenancePolicy {
    /// Window
    pub window: Option<MaintenanceWindow>,
}

/// Maintenance window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// Daily maintenance window
    pub daily_maintenance_window: Option<DailyMaintenanceWindow>,
    /// Recurring window
    pub recurring_window: Option<RecurringTimeWindow>,
}

/// Daily maintenance window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyMaintenanceWindow {
    /// Start time (HH:MM format)
    pub start_time: String,
}

/// Recurring time window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringTimeWindow {
    /// Window
    pub window: String,
    /// Recurrence: <day_of_week> or <day_of_month>
    pub recurrence: String,
}

/// Release channels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseChannel {
    Rapid,
    Regular,
    Stable,
    Extended,
}

impl ReleaseChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReleaseChannel::Rapid => "RAPID",
            ReleaseChannel::Regular => "REGULAR",
            ReleaseChannel::Stable => "STABLE",
            ReleaseChannel::Extended => "EXTENDED",
        }
    }
}

/// Private cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateClusterConfig {
    /// Enable private nodes
    pub enable_private_nodes: bool,
    /// Enable private endpoint
    pub enable_private_endpoint: bool,
    /// Master IPv4 CIDR block
    pub master_ipv4_cidr_block: String,
    /// Private endpoint subnetwork
    pub private_endpoint_subnetwork: Option<String>,
}

/// Database encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseEncryption {
    /// State
    pub state: EncryptionState,
    /// Key name (if using CMEK)
    pub key_name: Option<String>,
}

/// Encryption state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionState {
    Decrypted,
    Encrypted,
}

impl EncryptionState {
    pub fn as_str(&self) -> &'static str {
        match self {
            EncryptionState::Decrypted => "DECRYPTED",
            EncryptionState::Encrypted => "ENCRYPTED",
        }
    }
}

/// Workload identity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadIdentityConfig {
    /// Workload pool
    pub workload_pool: String,
    /// Identity namespace
    pub identity_namespace: String,
}

// ============================================================================
// GCP Client
// ============================================================================

/// GCP API client
pub struct GcpClient {
    /// Configuration
    config: GcpConfig,
    /// Credentials
    credentials: GcpCredentials,
    /// HTTP client
    http_client: reqwest::Client,
    /// Cached token
    token: Arc<RwLock<Option<GcpToken>>>,
    /// Base API URL
    base_url: String,
}

impl GcpClient {
    /// Create a new GCP client
    pub fn new(config: GcpConfig, credentials: GcpCredentials) -> Self {
        Self {
            config,
            credentials,
            http_client: reqwest::Client::new(),
            token: Arc::new(RwLock::new(None)),
            base_url: "https://compute.googleapis.com/compute/v1".to_string(),
        }
    }

    /// Get the projects URL
    pub fn projects_url(&self) -> String {
        format!("{}/projects/{}", self.base_url, self.config.project_id)
    }

    /// Get the zones URL
    pub fn zones_url(&self) -> String {
        format!("{}/zones", self.projects_url())
    }

    /// Acquire an access token using JWT
    pub async fn acquire_token(&self) -> CloudResult<GcpToken> {
        // Create JWT claim set
        let now = Utc::now().timestamp();
        let expiry = now + 3600;

        let header = serde_json::json!({
            "alg": "RS256",
            "typ": "JWT"
        });

        let claim = serde_json::json!({
            "iss": self.credentials.client_email,
            "scope": "https://www.googleapis.com/auth/cloud-platform",
            "aud": self.credentials.token_uri,
            "exp": expiry,
            "iat": now,
        });

        // For simplicity, we'll use a mock implementation
        // In production, you'd use a proper JWT library
        let jwt = format!("{}.{}", 
            serde_json::to_string(&header).unwrap(),
            serde_json::to_string(&claim).unwrap()
        );

        let response = self
            .http_client
            .post(&self.credentials.token_uri)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .map_err(|e| CloudError::AuthenticationFailed(format!("Token request failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::AuthenticationFailed(format!(
                "Token request failed: {}",
                error_text
            )));
        }

        let token_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudError::AuthenticationFailed(format!("Failed to parse token: {}", e)))?;

        let expires_on = Utc::now()
            + chrono::Duration::seconds(
                token_response["expires_in"]
                    .as_str()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600),
            );

        let token = GcpToken {
            access_token: token_response["access_token"]
                .as_str()
                .ok_or_else(|| CloudError::AuthenticationFailed("No access token in response".to_string()))?
                .to_string(),
            token_type: token_response["token_type"]
                .as_str()
                .unwrap_or("Bearer")
                .to_string(),
            expires_in: token_response["expires_in"]
                .as_str()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600),
            expiry: expires_on,
            refresh_token: None,
        };

        Ok(token)
    }

    /// Ensure we have a valid token
    pub async fn ensure_token(&self) -> CloudResult<String> {
        let mut token_guard = self.token.write().await;

        let needs_refresh = match token_guard.as_ref() {
            None => true,
            Some(token) => token.expiry < Utc::now() + chrono::Duration::seconds(60),
        };

        if needs_refresh {
            let new_token = self.acquire_token().await?;
            let access_token = new_token.access_token.clone();
            *token_guard = Some(new_token);
            Ok(access_token)
        } else {
            Ok(token_guard.as_ref().unwrap().access_token.clone())
        }
    }

    /// Create a compute instance
    pub async fn create_compute_instance(
        &self,
        config: ComputeInstanceConfig,
    ) -> CloudResult<CloudResource> {
        let access_token = self.ensure_token().await?;

        let url = format!(
            "{}/zones/{}/instances?alt=json",
            self.projects_url(),
            config.zone
        );

        let instance_body = serde_json::json!({
            "name": config.name,
            "machineType": format!("{}/machineTypes/{}", self.zones_url(), config.machine_type.name),
            "disks": vec![
                serde_json::json!({
                    "boot": config.boot_disk.boot,
                    "autoDelete": config.boot_disk.auto_delete,
                    "initializeParams": {
                        "sourceImage": config.boot_disk.initialize_params.source_image,
                        "diskType": format!("{}/zones/{}/diskTypes/{}", 
                            self.projects_url(), config.zone, config.boot_disk.initialize_params.disk_type.as_str()),
                        "diskSizeGb": config.boot_disk.initialize_params.disk_size_gb
                    }
                }),
            ],
            "networkInterfaces": config.network_interfaces.iter().map(|nic| {
                serde_json::json!({
                    "network": nic.network,
                    "subnetwork": nic.subnetwork,
                    "accessConfigs": nic.access_configs.iter().map(|ac| {
                        serde_json::json!({
                            "type": ac.kind,
                            "name": ac.name,
                            "networkTier": ac.network_tier.as_str(),
                        })
                    }).collect::<Vec<_>>()
                })
            }).collect::<Vec<_>>(),
            "serviceAccounts": config.service_accounts.iter().map(|sa| {
                serde_json::json!({
                    "email": sa.email,
                    "scopes": sa.scopes
                })
            }).collect::<Vec<_>>(),
            "metadata": {
                "items": config.metadata.items
            },
            "scheduling": {
                "preemptible": config.scheduling.preemptible,
                "automaticRestart": config.scheduling.automatic_restart,
                "onHostMaintenance": config.scheduling.on_host_maintenance.as_str()
            },
            "labels": config.labels,
            "tags": {
                "items": config.tags
            }
        });

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&access_token)
            .json(&instance_body)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("Instance creation failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::ApiError(format!(
                "Instance creation failed: {}",
                error_text
            )));
        }

        let instance: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to parse response: {}", e)))?;

        Ok(CloudResource {
            id: instance["id"].as_str().unwrap_or_default().to_string(),
            name: instance["name"].as_str().unwrap_or_default().to_string(),
            resource_type: "compute.instances".to_string(),
            provider: super::CloudProvider::Gcp,
            location: Some(instance["zone"].as_str().unwrap_or_default().to_string()),
            tags: HashMap::new(),
            properties: instance.as_object().cloned().unwrap_or_default(),
        })
    }

    /// Create a storage bucket
    pub async fn create_storage_bucket(&self, config: StorageBucketConfig) -> CloudResult<CloudResource> {
        let access_token = self.ensure_token().await?;

        let url = format!(
            "https://storage.googleapis.com/storage/v1/b?project={}",
            self.config.project_id
        );

        let bucket_body = serde_json::json!({
            "name": config.name,
            "location": config.location,
            "storageClass": config.storage_class.as_str(),
            "iamConfiguration": {
                "uniformBucketLevelAccess": {
                    "enabled": config.uniform_bucket_level_access
                },
                "publicAccessPrevention": config.public_access_prevention.as_str()
            },
            "labels": config.labels
        });

        if let Some(versioning) = config.versioning {
            bucket_body["versioning"] = serde_json::json!({
                "enabled": versioning.enabled
            });
        }

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&access_token)
            .json(&bucket_body)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("Bucket creation failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::ApiError(format!(
                "Bucket creation failed: {}",
                error_text
            )));
        }

        let bucket: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to parse response: {}", e)))?;

        Ok(CloudResource {
            id: bucket["id"].as_str().unwrap_or_default().to_string(),
            name: bucket["name"].as_str().unwrap_or_default().to_string(),
            resource_type: "storage.buckets".to_string(),
            provider: super::CloudProvider::Gcp,
            location: Some(bucket["location"].as_str().unwrap_or_default().to_string()),
            tags: HashMap::new(),
            properties: bucket.as_object().cloned().unwrap_or_default(),
        })
    }

    /// Create a GKE cluster
    pub async fn create_gke_cluster(&self, config: GkeClusterConfig) -> CloudResult<CloudResource> {
        let access_token = self.ensure_token().await?;

        let url = format!(
            "https://container.googleapis.com/v1/projects/{}/locations/{}/clusters?alt=json",
            self.config.project_id,
            config.location
        );

        let cluster_body = serde_json::json!({
            "name": config.name,
            "description": config.description,
            "initialNodeCount": config.initial_node_count,
            "networkConfig": {
                "network": config.network_config.network,
                "subnetwork": config.network_config.subnetwork,
            },
            "nodePools": config.node_pools.iter().map(|pool| {
                serde_json::json!({
                    "name": pool.name,
                    "initialNodeCount": pool.initial_node_count,
                    "config": {
                        "machineType": pool.machine_type,
                        "diskSizeGb": pool.disk_size_gb,
                        "diskType": pool.disk_type,
                        "imageType": pool.image_type.as_str(),
                        "preemptible": pool.config.as_ref().map(|c| c.preemptible).unwrap_or(false),
                    },
                    "autoscaling": pool.autoscaling.as_ref().map(|a| {
                        serde_json::json!({
                            "enabled": a.enabled,
                            "minNodeCount": a.min_node_count,
                            "maxNodeCount": a.max_node_count
                        })
                    }),
                    "management": pool.management.as_ref().map(|m| {
                        serde_json::json!({
                            "autoRepair": m.auto_repair,
                            "autoUpgrade": m.auto_upgrade
                        })
                    }),
                })
            }).collect::<Vec<_>>(),
            "labels": config.labels
        });

        let response = self
            .http_client
            .post(&url)
            .bearer_auth(&access_token)
            .json(&cluster_body)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("GKE creation failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::ApiError(format!(
                "GKE creation failed: {}",
                error_text
            )));
        }

        let cluster: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to parse response: {}", e)))?;

        Ok(CloudResource {
            id: cluster["id"].as_str().unwrap_or_default().to_string(),
            name: cluster["name"].as_str().unwrap_or_default().to_string(),
            resource_type: "container.clusters".to_string(),
            provider: super::CloudProvider::Gcp,
            location: Some(cluster["location"].as_str().unwrap_or_default().to_string()),
            tags: HashMap::new(),
            properties: cluster.as_object().cloned().unwrap_or_default(),
        })
    }

    /// List instances in a zone
    pub async fn list_instances(&self, zone: &str) -> CloudResult<Vec<CloudResource>> {
        let access_token = self.ensure_token().await?;

        let url = format!(
            "{}/zones/{}/instances?alt=json",
            self.projects_url(),
            zone
        );

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&access_token)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to list instances: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::ApiError(format!(
                "Failed to list instances: {}",
                error_text
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to parse response: {}", e)))?;

        let instances = result["items"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|i| {
                        Some(CloudResource {
                            id: i["id"].as_str()?.to_string(),
                            name: i["name"].as_str()?.to_string(),
                            resource_type: "compute.instances".to_string(),
                            provider: super::CloudProvider::Gcp,
                            location: Some(i["zone"].as_str()?.to_string()),
                            tags: HashMap::new(),
                            properties: i.as_object().cloned().unwrap_or_default(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(instances)
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_url: &str) -> CloudResult<()> {
        let access_token = self.ensure_token().await?;

        let response = self
            .http_client
            .delete(resource_url)
            .bearer_auth(&access_token)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to delete resource: {}", e)))?;

        if !response.status().is_success() && response.status().as_u16() != 204 {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::ApiError(format!(
                "Failed to delete resource: {}",
                error_text
            )));
        }

        Ok(())
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcp_config_default() {
        let config = GcpConfig::default();
        assert_eq!(config.region, "us-central1");
        assert_eq!(config.zone, "us-central1-a");
    }

    #[test]
    fn test_gcp_credentials() {
        let creds = GcpCredentials::new(
            "test@example.com",
            "123456789",
            "private-key",
            "my-project",
        );

        assert_eq!(creds.client_email, "test@example.com");
        assert!(creds.is_token_expired());
    }

    #[test]
    fn test_machine_types() {
        let e2 = MachineType::e2_medium();
        assert_eq!(e2.name, "e2-medium");
        assert!(e2.is_shared_cpu);

        let n1 = MachineType::n1_standard_4();
        assert_eq!(n1.name, "n1-standard-4");
        assert_eq!(n1.guest_cpus, 4);

        let n2 = MachineType::n2_standard_4();
        assert_eq!(n2.name, "n2-standard-4");

        let c2 = MachineType::c2_standard_4();
        assert_eq!(c2.name, "c2-standard-4");
    }

    #[test]
    fn test_disk_initialize_params() {
        let ubuntu = DiskInitializeParams::ubuntu_22_04();
        assert!(ubuntu.source_image.contains("ubuntu"));
        assert_eq!(ubuntu.disk_type, DiskType::PdStandard);

        let windows = DiskInitializeParams::windows_server_2022();
        assert!(windows.source_image.contains("windows"));
    }

    #[test]
    fn test_disk_types() {
        assert_eq!(DiskType::PdStandard.as_str(), "pd-standard");
        assert_eq!(DiskType::PdSsd.as_str(), "pd-ssd");
        assert_eq!(DiskType::PdBalanced.as_str(), "pd-balanced");
    }

    #[test]
    fn test_storage_class() {
        assert_eq!(StorageClass::Standard.as_str(), "STANDARD");
        assert_eq!(StorageClass::Nearline.as_str(), "NEARLINE");
        assert_eq!(StorageClass::Coldline.as_str(), "COLDLINE");
        assert_eq!(StorageClass::Archive.as_str(), "ARCHIVE");
    }

    #[test]
    fn test_service_account_scopes() {
        let scopes = ServiceAccount::default_scopes();
        assert!(!scopes.is_empty());

        let full_scopes = ServiceAccount::full_access_scopes();
        assert!(full_scopes.len() > scopes.len());
    }

    #[test]
    fn test_metadata() {
        let ssh_meta = Metadata::with_ssh_key("ubuntu", "ssh-rsa AAAAB...");
        assert_eq!(ssh_meta.items.len(), 1);
        assert_eq!(ssh_meta.items[0].key, "ssh-keys");

        let startup_meta = Metadata::with_startup_script("#!/bin/bash\necho Hello");
        assert_eq!(startup_meta.items[0].key, "startup-script");
    }

    #[test]
    fn test_firewall_direction() {
        assert_eq!(FirewallDirection::Ingress.as_str(), "INGRESS");
        assert_eq!(FirewallDirection::Egress.as_str(), "EGRESS");
    }

    #[test]
    fn test_release_channel() {
        assert_eq!(ReleaseChannel::Rapid.as_str(), "RAPID");
        assert_eq!(ReleaseChannel::Regular.as_str(), "REGULAR");
        assert_eq!(ReleaseChannel::Stable.as_str(), "STABLE");
        assert_eq!(ReleaseChannel::Extended.as_str(), "EXTENDED");
    }

    #[test]
    fn test_node_image_type() {
        assert_eq!(NodeImageType::ContainerOptimized.as_str(), "COS");
        assert_eq!(NodeImageType::CosContainerd.as_str(), "cos_containerd");
        assert_eq!(NodeImageType::Ubuntu.as_str(), "UBUNTU");
    }

    #[test]
    fn test_compute_instance_config() {
        let config = ComputeInstanceConfig {
            name: "test-instance".to_string(),
            zone: "us-central1-a".to_string(),
            machine_type: MachineType::e2_medium(),
            boot_disk: BootDiskConfig {
                initialize_params: DiskInitializeParams::ubuntu_22_04(),
                auto_delete: true,
                boot: true,
            },
            disks: vec![],
            network_interfaces: vec![NetworkInterface {
                network: "projects/my-project/global/networks/default".to_string(),
                subnetwork: "projects/my-project/regions/us-central1/subnetworks/default".to_string(),
                access_configs: vec![AccessConfig {
                    kind: "ONE_TO_ONE_NAT".to_string(),
                    network_tier: NetworkTier::Premium,
                    name: "External NAT".to_string(),
                    nat_ip: None,
                }],
                alias_ip_ranges: vec![],
                stack_type: NetworkStackType::Ipv4,
            }],
            service_accounts: vec![ServiceAccount {
                email: "default".to_string(),
                scopes: ServiceAccount::default_scopes(),
            }],
            metadata: Metadata::with_ssh_key("ubuntu", "ssh-rsa AAAAB..."),
            scheduling: Scheduling {
                preemptible: false,
                automatic_restart: true,
                on_host_maintenance: OnHostMaintenance::Migrate,
                node_affinities: vec![],
            },
            shielded_instance_config: None,
            labels: HashMap::new(),
            tags: vec!["http-server".to_string(), "https-server".to_string()],
        };

        assert_eq!(config.name, "test-instance");
        assert_eq!(config.machine_type.name, "e2-medium");
        assert_eq!(config.scheduling.automatic_restart, true);
    }

    #[test]
    fn test_storage_bucket_config() {
        let config = StorageBucketConfig {
            name: "my-unique-bucket".to_string(),
            location: "US".to_string(),
            storage_class: StorageClass::Standard,
            uniform_bucket_level_access: true,
            public_access_prevention: PublicAccessPrevention::Enforced,
            versioning: None,
            lifecycle: None,
            logging: None,
            labels: HashMap::new(),
        };

        assert_eq!(config.storage_class, StorageClass::Standard);
        assert!(config.uniform_bucket_level_access);
    }

    #[test]
    fn test_firewall_rule_config() {
        let config = FirewallRuleConfig {
            name: "allow-http".to_string(),
            description: Some("Allow HTTP traffic".to_string()),
            network: "projects/my-project/global/networks/default".to_string(),
            direction: FirewallDirection::Ingress,
            priority: 1000,
            action: FirewallAction::Allow,
            source_ranges: vec!["0.0.0.0/0".to_string()],
            source_tags: vec![],
            source_service_accounts: vec![],
            destination_ranges: vec![],
            allowed: vec![FirewallAllowed {
                ip_protocol: "tcp".to_string(),
                ports: vec!["80".to_string(), "443".to_string()],
            }],
            denied: vec![],
            target_tags: vec!["http-server".to_string()],
            target_service_accounts: vec![],
        };

        assert_eq!(config.name, "allow-http");
        assert_eq!(config.action, FirewallAction::Allow);
        assert_eq!(config.allowed.len(), 1);
    }

    #[test]
    fn test_gke_cluster_config() {
        let config = GkeClusterConfig {
            name: "test-gke".to_string(),
            description: Some("Test GKE cluster".to_string()),
            location: "us-central1".to_string(),
            initial_node_count: 3,
            node_pools: vec![NodePoolConfig {
                name: "default-pool".to_string(),
                version: Some("1.28.0-gke.100".to_string()),
                initial_node_count: 3,
                machine_type: "e2-medium".to_string(),
                disk_size_gb: 100,
                disk_type: "pd-standard".to_string(),
                image_type: NodeImageType::CosContainerd,
                autoscaling: Some(AutoscalingConfig {
                    enabled: true,
                    min_node_count: 1,
                    max_node_count: 10,
                }),
                management: Some(NodeManagement {
                    auto_repair: true,
                    auto_upgrade: true,
                    upgrade_options: None,
                }),
                config: None,
                labels: HashMap::new(),
                taints: vec![],
                tags: vec![],
                accelerators: vec![],
            }],
            network_config: GkeNetworkConfig {
                network: "projects/my-project/global/networks/default".to_string(),
                subnetwork: "projects/my-project/regions/us-central1/subnetworks/default".to_string(),
                cluster_ipv4_cidr: Some("10.0.0.0/17".to_string()),
                services_ipv4_cidr: Some("10.0.128.0/17".to_string()),
                private_ipv4_cidr: None,
            },
            ip_allocation_policy: Some(IpAllocationPolicy {
                cluster_ipv4_cidr_block: Some("10.0.0.0/17".to_string()),
                services_ipv4_cidr_block: Some("10.0.128.0/17".to_string()),
                use_ip_aliases: true,
            }),
            master_auth: None,
            addons_config: None,
            maintenance_policy: None,
            release_channel: Some(ReleaseChannel::Regular),
            private_cluster_config: None,
            database_encryption: None,
            workload_identity_config: None,
            labels: HashMap::new(),
        };

        assert_eq!(config.name, "test-gke");
        assert_eq!(config.node_pools.len(), 1);
        assert!(config.node_pools[0].autoscaling.as_ref().unwrap().enabled);
    }

    #[test]
    fn test_taint_effect() {
        assert_eq!(TaintEffect::NoSchedule.as_str(), "NO_SCHEDULE");
        assert_eq!(TaintEffect::PreferNoSchedule.as_str(), "PREFER_NO_SCHEDULE");
        assert_eq!(TaintEffect::NoExecute.as_str(), "NO_EXECUTE");
    }

    #[test]
    fn test_network_tier() {
        assert_eq!(NetworkTier::Premium.as_str(), "PREMIUM");
        assert_eq!(NetworkTier::Standard.as_str(), "STANDARD");
    }
}