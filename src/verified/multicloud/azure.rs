//! Microsoft Azure Cloud Integration
//! 
//! This module provides comprehensive integration with Microsoft Azure services,
//! including virtual machines, storage accounts, networking, and Azure Kubernetes Service (AKS).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;
use tokio::sync::RwLock;
use std::sync::Arc;
use chrono::{DateTime, Utc};

use super::{CloudError, CloudResult, CloudResource, CloudCredentials};

// ============================================================================
// Azure Configuration Types
// ============================================================================

/// Azure cloud configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureConfig {
    /// Azure subscription ID
    pub subscription_id: String,
    /// Resource group name
    pub resource_group: String,
    /// Azure region (e.g., "eastus", "westeurope")
    pub region: String,
    /// Azure Active Directory tenant ID
    pub tenant_id: String,
    /// Client ID for service principal
    pub client_id: String,
    /// Client secret for service principal
    pub client_secret: Option<String>,
    /// Azure cloud environment
    pub environment: AzureEnvironment,
    /// Default tags for all resources
    pub default_tags: HashMap<String, String>,
}

impl Default for AzureConfig {
    fn default() -> Self {
        Self {
            subscription_id: String::new(),
            resource_group: String::new(),
            region: "eastus".to_string(),
            tenant_id: String::new(),
            client_id: String::new(),
            client_secret: None,
            environment: AzureEnvironment::Public,
            default_tags: HashMap::new(),
        }
    }
}

/// Azure cloud environments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AzureEnvironment {
    /// Azure public cloud
    Public,
    /// Azure Government cloud
    Government,
    /// Azure China cloud
    China,
    /// Azure Germany cloud
    Germany,
}

impl AzureEnvironment {
    /// Get the management endpoint for this environment
    pub fn management_endpoint(&self) -> &'static str {
        match self {
            AzureEnvironment::Public => "https://management.azure.com",
            AzureEnvironment::Government => "https://management.usgovcloudapi.net",
            AzureEnvironment::China => "https://management.chinacloudapi.cn",
            AzureEnvironment::Germany => "https://management.microsoftazure.de",
        }
    }

    /// Get the storage endpoint suffix for this environment
    pub fn storage_suffix(&self) -> &'static str {
        match self {
            AzureEnvironment::Public => "core.windows.net",
            AzureEnvironment::Government => "core.usgovcloudapi.net",
            AzureEnvironment::China => "core.chinacloudapi.cn",
            AzureEnvironment::Germany => "core.microsoftazure.de",
        }
    }
}

// ============================================================================
// Azure Credentials
// ============================================================================

/// Azure authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureCredentials {
    /// Tenant ID
    pub tenant_id: String,
    /// Client ID (Application ID)
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Subscription ID
    pub subscription_id: String,
    /// Access token (cached)
    pub access_token: Option<AzureToken>,
}

/// Azure OAuth2 access token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureToken {
    /// Access token value
    pub access_token: String,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Expires in seconds
    pub expires_in: i64,
    /// Token expiration time
    pub expires_on: DateTime<Utc>,
    /// Refresh token (if available)
    pub refresh_token: Option<String>,
}

impl AzureCredentials {
    /// Create new Azure credentials
    pub fn new(
        tenant_id: impl Into<String>,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        subscription_id: impl Into<String>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            subscription_id: subscription_id.into(),
            access_token: None,
        }
    }

    /// Check if the current token is expired
    pub fn is_token_expired(&self) -> bool {
        match &self.access_token {
            Some(token) => token.expires_on < Utc::now() + chrono::Duration::seconds(60),
            None => true,
        }
    }
}

impl CloudCredentials for AzureCredentials {
    fn provider(&self) -> super::CloudProvider {
        super::CloudProvider::Azure
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// ============================================================================
// Azure Virtual Machine
// ============================================================================

/// Azure Virtual Machine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineConfig {
    /// VM name
    pub name: String,
    /// Resource group
    pub resource_group: String,
    /// Azure region
    pub location: String,
    /// VM size (e.g., "Standard_D2s_v3")
    pub vm_size: VmSize,
    /// Operating system configuration
    pub os_profile: OsProfile,
    /// Network profile
    pub network_profile: NetworkProfile,
    /// Storage profile
    pub storage_profile: StorageProfile,
    /// Availability set (optional)
    pub availability_set: Option<String>,
    /// Managed identity
    pub identity: Option<ManagedIdentity>,
    /// Tags
    pub tags: HashMap<String, String>,
    /// Zones (for zonal deployment)
    pub zones: Vec<String>,
}

/// VM size specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmSize {
    /// Size name (e.g., "Standard_D2s_v3")
    pub name: String,
    /// Number of CPU cores
    pub number_of_cores: i32,
    /// Memory in MB
    pub memory_in_mb: i64,
    /// Max data disk count
    pub max_data_disk_count: i32,
    /// OS disk size in GB
    pub os_disk_size_in_mb: i64,
    /// Resource disk size in MB
    pub resource_disk_size_in_mb: i64,
}

impl VmSize {
    /// Common VM sizes
    pub fn standard_d2s_v3() -> Self {
        Self {
            name: "Standard_D2s_v3".to_string(),
            number_of_cores: 2,
            memory_in_mb: 8192,
            max_data_disk_count: 4,
            os_disk_size_in_mb: 1047552,
            resource_disk_size_in_mb: 16384,
        }
    }

    pub fn standard_d4s_v3() -> Self {
        Self {
            name: "Standard_D4s_v3".to_string(),
            number_of_cores: 4,
            memory_in_mb: 16384,
            max_data_disk_count: 8,
            os_disk_size_in_mb: 1047552,
            resource_disk_size_in_mb: 32768,
        }
    }

    pub fn standard_b2s() -> Self {
        Self {
            name: "Standard_B2s".to_string(),
            number_of_cores: 2,
            memory_in_mb: 4096,
            max_data_disk_count: 4,
            os_disk_size_in_mb: 1047552,
            resource_disk_size_in_mb: 8192,
        }
    }
}

/// Operating system profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsProfile {
    /// Computer name
    pub computer_name: String,
    /// Admin username
    pub admin_username: String,
    /// Admin password (for Windows)
    pub admin_password: Option<String>,
    /// SSH public key (for Linux)
    pub ssh_public_key: Option<SshPublicKey>,
    /// Custom data (cloud-init)
    pub custom_data: Option<String>,
    /// Windows configuration
    pub windows_configuration: Option<WindowsConfiguration>,
    /// Linux configuration
    pub linux_configuration: Option<LinuxConfiguration>,
}

/// SSH public key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshPublicKey {
    /// Key path on the VM
    pub path: String,
    /// Public key data
    pub key_data: String,
}

/// Windows configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsConfiguration {
    /// Enable automatic updates
    pub enable_automatic_updates: bool,
    /// Time zone
    pub time_zone: Option<String>,
    /// WinRM configuration
    pub win_rm: Option<WinRmConfiguration>,
}

/// WinRM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinRmConfiguration {
    /// List of listeners
    pub listeners: Vec<WinRmListener>,
}

/// WinRM listener
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinRmListener {
    /// Protocol (Http or Https)
    pub protocol: String,
    /// Certificate URL (for HTTPS)
    pub certificate_url: Option<String>,
}

/// Linux configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxConfiguration {
    /// Disable password authentication
    pub disable_password_authentication: bool,
    /// SSH configuration
    pub ssh: Option<SshConfiguration>,
    /// Provision VM agent
    pub provision_vm_agent: bool,
}

/// SSH configuration for Linux
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfiguration {
    /// Public keys
    pub public_keys: Vec<SshPublicKey>,
}

/// Network profile for VM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfile {
    /// Network interfaces
    pub network_interfaces: Vec<NetworkInterfaceReference>,
    /// Accelerated networking
    pub enable_accelerated_networking: bool,
}

/// Network interface reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceReference {
    /// Resource ID
    pub id: String,
    /// Primary network interface
    pub primary: bool,
}

/// Storage profile for VM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProfile {
    /// OS disk
    pub os_disk: OsDisk,
    /// Data disks
    pub data_disks: Vec<DataDisk>,
    /// Image reference
    pub image_reference: ImageReference,
}

/// OS disk configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsDisk {
    /// Disk name
    pub name: String,
    /// Disk size in GB
    pub disk_size_gb: i32,
    /// Storage account type
    pub storage_account_type: StorageAccountType,
    /// Caching type
    pub caching: CachingType,
    /// Create option
    pub create_option: DiskCreateOption,
    /// Managed disk ID (for attaching existing disk)
    pub managed_disk_id: Option<String>,
}

/// Data disk configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDisk {
    /// Lun (logical unit number)
    pub lun: i32,
    /// Disk name
    pub name: String,
    /// Disk size in GB
    pub disk_size_gb: i32,
    /// Storage account type
    pub storage_account_type: StorageAccountType,
    /// Caching type
    pub caching: CachingType,
    /// Create option
    pub create_option: DiskCreateOption,
}

/// Storage account types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageAccountType {
    StandardLrs,
    StandardSsdLrs,
    PremiumLrs,
    PremiumV2Lrs,
    UltraSsdLrs,
}

impl StorageAccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageAccountType::StandardLrs => "Standard_LRS",
            StorageAccountType::StandardSsdLrs => "StandardSSD_LRS",
            StorageAccountType::PremiumLrs => "Premium_LRS",
            StorageAccountType::PremiumV2Lrs => "PremiumV2_LRS",
            StorageAccountType::UltraSsdLrs => "UltraSSD_LRS",
        }
    }
}

/// Caching types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CachingType {
    None,
    ReadOnly,
    ReadWrite,
}

/// Disk create options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiskCreateOption {
    FromImage,
    Empty,
    Attach,
    Copy,
    Restore,
}

/// Image reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageReference {
    /// Publisher (e.g., "Canonical", "MicrosoftWindowsServer")
    pub publisher: String,
    /// Offer (e.g., "UbuntuServer", "WindowsServer")
    pub offer: String,
    /// SKU (e.g., "18.04-LTS", "2019-Datacenter")
    pub sku: String,
    /// Version (e.g., "latest", specific version)
    pub version: String,
}

impl ImageReference {
    /// Ubuntu 20.04 LTS
    pub fn ubuntu_20_04() -> Self {
        Self {
            publisher: "Canonical".to_string(),
            offer: "UbuntuServer".to_string(),
            sku: "20_04-lts".to_string(),
            version: "latest".to_string(),
        }
    }

    /// Ubuntu 22.04 LTS
    pub fn ubuntu_22_04() -> Self {
        Self {
            publisher: "Canonical".to_string(),
            offer: "UbuntuServer".to_string(),
            sku: "22_04-lts".to_string(),
            version: "latest".to_string(),
        }
    }

    /// Windows Server 2019
    pub fn windows_server_2019() -> Self {
        Self {
            publisher: "MicrosoftWindowsServer".to_string(),
            offer: "WindowsServer".to_string(),
            sku: "2019-Datacenter".to_string(),
            version: "latest".to_string(),
        }
    }

    /// Windows Server 2022
    pub fn windows_server_2022() -> Self {
        Self {
            publisher: "MicrosoftWindowsServer".to_string(),
            offer: "WindowsServer".to_string(),
            sku: "2022-Datacenter".to_string(),
            version: "latest".to_string(),
        }
    }
}

/// Managed identity for VM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedIdentity {
    /// Identity type
    pub identity_type: IdentityType,
    /// User assigned identities
    pub user_assigned_identities: Vec<String>,
    /// Principal ID (system assigned)
    pub principal_id: Option<String>,
    /// Tenant ID
    pub tenant_id: Option<String>,
}

/// Identity types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityType {
    SystemAssigned,
    UserAssigned,
    SystemAssignedUserAssigned,
    None,
}

// ============================================================================
// Azure Storage Account
// ============================================================================

/// Azure Storage Account configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountConfig {
    /// Storage account name (must be globally unique)
    pub name: String,
    /// Resource group
    pub resource_group: String,
    /// Location
    pub location: String,
    /// Storage account kind
    pub kind: StorageAccountKind,
    /// SKU
    pub sku: StorageSku,
    /// Access tier
    pub access_tier: AccessTier,
    /// Enable HTTPS only
    pub enable_https_traffic_only: bool,
    /// Minimum TLS version
    pub minimum_tls_version: TlsVersion,
    /// Allow blob public access
    pub allow_blob_public_access: bool,
    /// Network rule set
    pub network_rule_set: Option<NetworkRuleSet>,
    /// Encryption settings
    pub encryption: EncryptionSettings,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Storage account kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageAccountKind {
    StorageV2,
    Storage,
    BlobStorage,
    BlockBlobStorage,
    FileStorage,
}

impl StorageAccountKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageAccountKind::StorageV2 => "StorageV2",
            StorageAccountKind::Storage => "Storage",
            StorageAccountKind::BlobStorage => "BlobStorage",
            StorageAccountKind::BlockBlobStorage => "BlockBlobStorage",
            StorageAccountKind::FileStorage => "FileStorage",
        }
    }
}

/// Storage SKU
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageSku {
    pub name: StorageSkuName,
    pub tier: StorageTier,
}

/// Storage SKU names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageSkuName {
    StandardLrs,
    StandardGrs,
    StandardRagrs,
    StandardZrs,
    PremiumLrs,
    PremiumZrs,
}

impl StorageSkuName {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageSkuName::StandardLrs => "Standard_LRS",
            StorageSkuName::StandardGrs => "Standard_GRS",
            StorageSkuName::StandardRagrs => "Standard_RAGRS",
            StorageSkuName::StandardZrs => "Standard_ZRS",
            StorageSkuName::PremiumLrs => "Premium_LRS",
            StorageSkuName::PremiumZrs => "Premium_ZRS",
        }
    }
}

/// Storage tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageTier {
    Standard,
    Premium,
}

/// Access tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessTier {
    Hot,
    Cool,
    Archive,
}

impl AccessTier {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccessTier::Hot => "Hot",
            AccessTier::Cool => "Cool",
            AccessTier::Archive => "Archive",
        }
    }
}

/// TLS versions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TlsVersion {
    Tls1_0,
    Tls1_1,
    Tls1_2,
}

impl TlsVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            TlsVersion::Tls1_0 => "TLS1_0",
            TlsVersion::Tls1_1 => "TLS1_1",
            TlsVersion::Tls1_2 => "TLS1_2",
        }
    }
}

/// Network rule set for storage account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    /// Default action
    pub default_action: DefaultAction,
    /// IP rules
    pub ip_rules: Vec<IpRule>,
    /// Virtual network rules
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    /// Bypass services
    pub bypass: Vec<Bypass>,
}

/// Default actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefaultAction {
    Allow,
    Deny,
}

/// IP rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRule {
    /// IP address or CIDR
    pub ip_address_or_range: String,
    /// Action
    pub action: String,
}

/// Virtual network rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    /// Virtual network resource ID
    pub virtual_network_resource_id: String,
    /// Action
    pub action: String,
}

/// Bypass services
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bypass {
    AzureServices,
    Logging,
    Metrics,
}

/// Encryption settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionSettings {
    /// Key source
    pub key_source: KeySource,
    /// Key vault properties (if using customer-managed keys)
    pub key_vault_properties: Option<KeyVaultProperties>,
    /// Services to encrypt
    pub services: EncryptionServices,
}

/// Key sources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeySource {
    MicrosoftStorage,
    MicrosoftKeyVault,
}

/// Key vault properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    pub key_identifier: String,
}

/// Encryption services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionServices {
    pub blob: EncryptionService,
    pub file: EncryptionService,
    pub table: EncryptionService,
    pub queue: EncryptionService,
}

/// Encryption service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionService {
    pub enabled: bool,
    pub key_type: Option<KeyType>,
}

/// Key types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyType {
    Service,
    Account,
}

// ============================================================================
// Azure Virtual Network
// ============================================================================

/// Azure Virtual Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNetworkConfig {
    /// VNet name
    pub name: String,
    /// Resource group
    pub resource_group: String,
    /// Location
    pub location: String,
    /// Address space (CIDR blocks)
    pub address_space: Vec<String>,
    /// Subnets
    pub subnets: Vec<SubnetConfig>,
    /// DNS servers
    pub dns_servers: Vec<String>,
    /// DDoS protection
    pub enable_ddos_protection: bool,
    /// VNet peering
    pub peerings: Vec<VNetPeering>,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Subnet configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetConfig {
    /// Subnet name
    pub name: String,
    /// Address prefix (CIDR)
    pub address_prefix: String,
    /// Network security group
    pub network_security_group: Option<String>,
    /// Route table
    pub route_table: Option<String>,
    /// Service endpoints
    pub service_endpoints: Vec<ServiceEndpoint>,
    /// Private endpoints
    pub private_endpoints: Vec<String>,
    /// Enable private link service network policies
    pub private_link_service_network_policies: bool,
    /// NAT gateway
    pub nat_gateway: Option<String>,
}

/// Service endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Service name (e.g., "Microsoft.Storage")
    pub service: String,
    /// Locations
    pub locations: Vec<String>,
}

/// VNet peering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VNetPeering {
    /// Peering name
    pub name: String,
    /// Remote VNet resource ID
    pub remote_virtual_network: String,
    /// Allow forwarded traffic
    pub allow_forwarded_traffic: bool,
    /// Allow gateway transit
    pub allow_gateway_transit: bool,
    /// Allow virtual network access
    pub allow_virtual_network_access: bool,
    /// Use remote gateways
    pub use_remote_gateways: bool,
}

// ============================================================================
// Azure Network Security Group
// ============================================================================

/// Network Security Group configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityGroupConfig {
    /// NSG name
    pub name: String,
    /// Resource group
    pub resource_group: String,
    /// Location
    pub location: String,
    /// Security rules
    pub security_rules: Vec<SecurityRule>,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Security rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    /// Rule name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Protocol
    pub protocol: SecurityRuleProtocol,
    /// Source port range
    pub source_port_range: Option<String>,
    /// Source port ranges
    pub source_port_ranges: Vec<String>,
    /// Destination port range
    pub destination_port_range: Option<String>,
    /// Destination port ranges
    pub destination_port_ranges: Vec<String>,
    /// Source address prefix
    pub source_address_prefix: Option<String>,
    /// Source address prefixes
    pub source_address_prefixes: Vec<String>,
    /// Destination address prefix
    pub destination_address_prefix: Option<String>,
    /// Destination address prefixes
    pub destination_address_prefixes: Vec<String>,
    /// Access (Allow/Deny)
    pub access: SecurityRuleAccess,
    /// Priority (100-4096)
    pub priority: i32,
    /// Direction (Inbound/Outbound)
    pub direction: SecurityRuleDirection,
}

/// Security rule protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityRuleProtocol {
    Tcp,
    Udp,
    Icmp,
    Esp,
    Ah,
    Asterisk,
}

impl SecurityRuleProtocol {
    pub fn as_str(&self) -> &'static str {
        match self {
            SecurityRuleProtocol::Tcp => "Tcp",
            SecurityRuleProtocol::Udp => "Udp",
            SecurityRuleProtocol::Icmp => "Icmp",
            SecurityRuleProtocol::Esp => "Esp",
            SecurityRuleProtocol::Ah => "Ah",
            SecurityRuleProtocol::Asterisk => "*",
        }
    }
}

/// Security rule access
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityRuleAccess {
    Allow,
    Deny,
}

/// Security rule direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityRuleDirection {
    Inbound,
    Outbound,
}

// ============================================================================
// Azure Kubernetes Service (AKS)
// ============================================================================

/// Azure Kubernetes Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AksClusterConfig {
    /// Cluster name
    pub name: String,
    /// Resource group
    pub resource_group: String,
    /// Location
    pub location: String,
    /// DNS prefix
    pub dns_prefix: String,
    /// Kubernetes version
    pub kubernetes_version: String,
    /// Node pools
    pub agent_pools: Vec<AgentPoolConfig>,
    /// Network profile
    pub network_profile: AksNetworkProfile,
    /// AAD profile
    pub aad_profile: Option<AadProfile>,
    /// Add-on profiles
    pub addons: AksAddons,
    /// Auto-scaler profile
    pub auto_scaler_profile: Option<AksAutoScalerProfile>,
    /// API server access profile
    pub api_server_access_profile: Option<ApiServerAccessProfile>,
    /// Identity profile
    pub identity_profile: Option<AksIdentityProfile>,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Agent pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPoolConfig {
    /// Pool name
    pub name: String,
    /// VM size
    pub vm_size: String,
    /// OS disk size in GB
    pub os_disk_size_gb: i32,
    /// Node count
    pub count: i32,
    /// Min count (for auto-scaling)
    pub min_count: Option<i32>,
    /// Max count (for auto-scaling)
    pub max_count: Option<i32>,
    /// Enable auto-scaling
    pub enable_auto_scaling: bool,
    /// OS type (Linux/Windows)
    pub os_type: OsType,
    /// Max pods per node
    pub max_pods: Option<i32>,
    /// VNet subnet ID
    pub vnet_subnet_id: Option<String>,
    /// Availability zones
    pub availability_zones: Vec<String>,
    /// Node labels
    pub node_labels: HashMap<String, String>,
    /// Node taints
    pub node_taints: Vec<String>,
    /// Mode (System/User)
    pub mode: AgentPoolMode,
    /// Enable public IP
    pub enable_node_public_ip: bool,
    /// Scale down mode
    pub scale_down_mode: ScaleDownMode,
    /// Spot VM configuration
    pub spot_configuration: Option<SpotVmConfiguration>,
}

/// OS types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OsType {
    Linux,
    Windows,
}

/// Agent pool modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentPoolMode {
    System,
    User,
}

/// Scale down modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleDownMode {
    Delete,
    Deallocate,
}

/// Spot VM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotVmConfiguration {
    /// Max price
    pub max_price: f64,
    /// Eviction policy
    pub eviction_policy: SpotEvictionPolicy,
}

/// Spot eviction policies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotEvictionPolicy {
    Delete,
    Deallocate,
}

/// AKS network profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AksNetworkProfile {
    /// Network plugin (azure/kubenet)
    pub network_plugin: NetworkPlugin,
    /// Network policy (calico/azure)
    pub network_policy: Option<NetworkPolicy>,
    /// Pod CIDR
    pub pod_cidr: Option<String>,
    /// Service CIDR
    pub service_cidr: Option<String>,
    /// DNS service IP
    pub dns_service_ip: Option<String>,
    /// Docker bridge CIDR
    pub docker_bridge_cidr: Option<String>,
    /// Outbound type
    pub outbound_type: OutboundType,
    /// Load balancer SKU
    pub load_balancer_sku: LoadBalancerSku,
}

/// Network plugins
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPlugin {
    Kubenet,
    Azure,
    None,
}

/// Network policies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkPolicy {
    Calico,
    Azure,
}

/// Outbound types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutboundType {
    LoadBalancer,
    UserDefinedRouting,
    ManagedNATGateway,
    UserAssignedNATGateway,
}

/// Load balancer SKUs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancerSku {
    Basic,
    Standard,
}

/// AAD profile for AKS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AadProfile {
    /// Managed AAD
    pub managed: bool,
    /// Enable Azure RBAC
    pub enable_azure_rbac: bool,
    /// Admin group object IDs
    pub admin_group_object_ids: Vec<String>,
    /// Tenant ID
    pub tenant_id: Option<String>,
}

/// AKS add-ons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AksAddons {
    /// Enable HTTP application routing
    pub http_application_routing: bool,
    /// Enable monitoring (Azure Monitor for containers)
    pub monitoring: Option<MonitoringProfile>,
    /// Enable Azure Policy
    pub azure_policy: bool,
    /// Enable GitOps
    pub gitops: bool,
    /// Enable Open Service Mesh
    pub open_service_mesh: bool,
    /// Enable Azure Key Vault secrets provider
    pub azure_keyvault_secrets_provider: bool,
    /// Enable Keda
    pub keda: bool,
    /// Enable Virtual node
    pub virtual_node: bool,
}

/// Monitoring profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringProfile {
    /// Log analytics workspace resource ID
    pub log_analytics_workspace_resource_id: Option<String>,
    /// Enable metrics
    pub enabled: bool,
}

/// AKS auto-scaler profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AksAutoScalerProfile {
    /// Balance similar node groups
    pub balance_similar_node_groups: bool,
    /// Expander
    pub expander: ExpanderType,
    /// Max empty bulk delete
    pub max_empty_bulk_delete: i32,
    /// Max graceful termination sec
    pub max_graceful_termination_sec: i32,
    /// Max node provision time
    pub max_node_provision_time: String,
    /// Max total unready percentage
    pub max_total_unready_percentage: i32,
    /// New pod scale up delay
    pub new_pod_scale_up_delay: String,
    /// Ok total unready count
    pub ok_total_unready_count: i32,
    /// Scan interval
    pub scan_interval: String,
    /// Scale down delay after add
    pub scale_down_delay_after_add: String,
    /// Scale down delay after delete
    pub scale_down_delay_after_delete: String,
    /// Scale down delay after failure
    pub scale_down_delay_after_failure: String,
    /// Scale down unneeded time
    pub scale_down_unneeded_time: String,
    /// Scale down unready time
    pub scale_down_unready_time: String,
    /// Scale down utilization threshold
    pub scale_down_utilization_threshold: f64,
    /// Skip nodes with system pods
    pub skip_nodes_with_system_pods: bool,
}

/// Expander types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpanderType {
    Random,
    LeastWaste,
    MostPods,
    Priority,
}

/// API server access profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServerAccessProfile {
    /// Authorized IP ranges
    pub authorized_ip_ranges: Vec<String>,
    /// Enable private cluster
    pub enable_private_cluster: bool,
    /// Private DNS zone
    pub private_dns_zone: Option<String>,
    /// Enable private cluster public FQDN
    pub enable_private_cluster_public_fqdn: bool,
}

/// AKS identity profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AksIdentityProfile {
    /// Control plane identity
    pub control_plane_identity: Option<UserAssignedIdentity>,
    /// Kubelet identity
    pub kubelet_identity: Option<UserAssignedIdentity>,
}

/// User assigned identity reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAssignedIdentity {
    /// Resource ID
    pub resource_id: String,
    /// Client ID
    pub client_id: String,
    /// Object ID
    pub object_id: String,
}

// ============================================================================
// Azure Client
// ============================================================================

/// Azure API client
pub struct AzureClient {
    /// Configuration
    config: AzureConfig,
    /// Credentials
    credentials: AzureCredentials,
    /// HTTP client
    http_client: reqwest::Client,
    /// Cached token
    token: Arc<RwLock<Option<AzureToken>>>,
}

impl AzureClient {
    /// Create a new Azure client
    pub fn new(config: AzureConfig, credentials: AzureCredentials) -> Self {
        Self {
            config,
            credentials,
            http_client: reqwest::Client::new(),
            token: Arc::new(RwLock::new(None)),
        }
    }

    /// Get the management endpoint URL
    pub fn management_url(&self) -> String {
        format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers",
            self.config.environment.management_endpoint(),
            self.config.subscription_id,
            self.config.resource_group
        )
    }

    /// Acquire an access token
    pub async fn acquire_token(&self) -> CloudResult<AzureToken> {
        let token_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/token",
            self.credentials.tenant_id
        );

        let response = self
            .http_client
            .post(&token_url)
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", &self.credentials.client_id),
                ("client_secret", &self.credentials.client_secret),
                ("resource", "https://management.azure.com/"),
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

        let token = AzureToken {
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
            expires_on,
            refresh_token: None,
        };

        Ok(token)
    }

    /// Ensure we have a valid token
    pub async fn ensure_token(&self) -> CloudResult<String> {
        let mut token_guard = self.token.write().await;

        let needs_refresh = match token_guard.as_ref() {
            None => true,
            Some(token) => token.expires_on < Utc::now() + chrono::Duration::seconds(60),
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

    /// Create a virtual machine
    pub async fn create_virtual_machine(
        &self,
        config: VirtualMachineConfig,
    ) -> CloudResult<CloudResource> {
        let access_token = self.ensure_token().await?;

        let url = format!(
            "{}/Microsoft.Compute/virtualMachines/{}?api-version=2023-03-01",
            self.management_url(),
            config.name
        );

        let vm_body = serde_json::json!({
            "location": config.location,
            "tags": config.tags,
            "properties": {
                "hardwareProfile": {
                    "vmSize": config.vm_size.name
                },
                "osProfile": {
                    "computerName": config.os_profile.computer_name,
                    "adminUsername": config.os_profile.admin_username,
                },
                "storageProfile": {
                    "osDisk": {
                        "name": config.storage_profile.os_disk.name,
                        "diskSizeGB": config.storage_profile.os_disk.disk_size_gb,
                        "caching": "ReadWrite",
                        "createOption": "FromImage",
                        "managedDisk": {
                            "storageAccountType": config.storage_profile.os_disk.storage_account_type.as_str()
                        }
                    },
                    "imageReference": {
                        "publisher": config.storage_profile.image_reference.publisher,
                        "offer": config.storage_profile.image_reference.offer,
                        "sku": config.storage_profile.image_reference.sku,
                        "version": config.storage_profile.image_reference.version
                    }
                },
                "networkProfile": {
                    "networkInterfaces": config.network_profile.network_interfaces.iter().map(|nic| {
                        serde_json::json!({
                            "id": nic.id,
                            "properties": {
                                "primary": nic.primary
                            }
                        })
                    }).collect::<Vec<_>>()
                }
            }
        });

        let response = self
            .http_client
            .put(&url)
            .bearer_auth(&access_token)
            .json(&vm_body)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("VM creation failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::ApiError(format!(
                "VM creation failed: {}",
                error_text
            )));
        }

        let vm: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to parse response: {}", e)))?;

        Ok(CloudResource {
            id: vm["id"].as_str().unwrap_or_default().to_string(),
            name: vm["name"].as_str().unwrap_or_default().to_string(),
            resource_type: "Microsoft.Compute/virtualMachines".to_string(),
            provider: super::CloudProvider::Azure,
            location: Some(vm["location"].as_str().unwrap_or_default().to_string()),
            tags: HashMap::new(),
            properties: vm.as_object().cloned().unwrap_or_default(),
        })
    }

    /// Create a storage account
    pub async fn create_storage_account(
        &self,
        config: StorageAccountConfig,
    ) -> CloudResult<CloudResource> {
        let access_token = self.ensure_token().await?;

        let url = format!(
            "{}/Microsoft.Storage/storageAccounts/{}?api-version=2023-01-01",
            self.management_url(),
            config.name
        );

        let storage_body = serde_json::json!({
            "location": config.location,
            "kind": config.kind.as_str(),
            "sku": {
                "name": config.sku.name.as_str(),
                "tier": match config.sku.tier {
                    StorageTier::Standard => "Standard",
                    StorageTier::Premium => "Premium",
                }
            },
            "properties": {
                "accessTier": config.access_tier.as_str(),
                "supportsHttpsTrafficOnly": config.enable_https_traffic_only,
                "minimumTlsVersion": config.minimum_tls_version.as_str(),
                "allowBlobPublicAccess": config.allow_blob_public_access,
            },
            "tags": config.tags
        });

        let response = self
            .http_client
            .put(&url)
            .bearer_auth(&access_token)
            .json(&storage_body)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("Storage creation failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::ApiError(format!(
                "Storage creation failed: {}",
                error_text
            )));
        }

        let storage: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to parse response: {}", e)))?;

        Ok(CloudResource {
            id: storage["id"].as_str().unwrap_or_default().to_string(),
            name: storage["name"].as_str().unwrap_or_default().to_string(),
            resource_type: "Microsoft.Storage/storageAccounts".to_string(),
            provider: super::CloudProvider::Azure,
            location: Some(storage["location"].as_str().unwrap_or_default().to_string()),
            tags: HashMap::new(),
            properties: storage.as_object().cloned().unwrap_or_default(),
        })
    }

    /// Create an AKS cluster
    pub async fn create_aks_cluster(&self, config: AksClusterConfig) -> CloudResult<CloudResource> {
        let access_token = self.ensure_token().await?;

        let url = format!(
            "{}/Microsoft.ContainerService/managedClusters/{}?api-version=2023-05-01",
            self.management_url(),
            config.name
        );

        let aks_body = serde_json::json!({
            "location": config.location,
            "properties": {
                "kubernetesVersion": config.kubernetes_version,
                "dnsPrefix": config.dns_prefix,
                "agentPoolProfiles": config.agent_pools.iter().map(|pool| {
                    serde_json::json!({
                        "name": pool.name,
                        "vmSize": pool.vm_size,
                        "osDiskSizeGB": pool.os_disk_size_gb,
                        "count": pool.count,
                        "enableAutoScaling": pool.enable_auto_scaling,
                        "minCount": pool.min_count,
                        "maxCount": pool.max_count,
                        "osType": match pool.os_type {
                            OsType::Linux => "Linux",
                            OsType::Windows => "Windows",
                        },
                        "mode": match pool.mode {
                            AgentPoolMode::System => "System",
                            AgentPoolMode::User => "User",
                        },
                        "availabilityZones": pool.availability_zones,
                        "nodeLabels": pool.node_labels,
                        "nodeTaints": pool.node_taints,
                    })
                }).collect::<Vec<_>>(),
                "networkProfile": {
                    "networkPlugin": match config.network_profile.network_plugin {
                        NetworkPlugin::Kubenet => "kubenet",
                        NetworkPlugin::Azure => "azure",
                        NetworkPlugin::None => "none",
                    },
                    "networkPolicy": config.network_profile.network_policy.map(|p| match p {
                        NetworkPolicy::Calico => "calico",
                        NetworkPolicy::Azure => "azure",
                    }),
                    "loadBalancerSku": match config.network_profile.load_balancer_sku {
                        LoadBalancerSku::Basic => "Basic",
                        LoadBalancerSku::Standard => "Standard",
                    },
                    "outboundType": match config.network_profile.outbound_type {
                        OutboundType::LoadBalancer => "loadBalancer",
                        OutboundType::UserDefinedRouting => "userDefinedRouting",
                        OutboundType::ManagedNATGateway => "managedNATGateway",
                        OutboundType::UserAssignedNATGateway => "userAssignedNATGateway",
                    },
                },
            },
            "tags": config.tags
        });

        let response = self
            .http_client
            .put(&url)
            .bearer_auth(&access_token)
            .json(&aks_body)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("AKS creation failed: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::ApiError(format!(
                "AKS creation failed: {}",
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
            resource_type: "Microsoft.ContainerService/managedClusters".to_string(),
            provider: super::CloudProvider::Azure,
            location: Some(cluster["location"].as_str().unwrap_or_default().to_string()),
            tags: HashMap::new(),
            properties: cluster.as_object().cloned().unwrap_or_default(),
        })
    }

    /// List resource groups
    pub async fn list_resource_groups(&self) -> CloudResult<Vec<CloudResource>> {
        let access_token = self.ensure_token().await?;

        let url = format!(
            "{}/subscriptions/{}/resourcegroups?api-version=2022-09-01",
            self.config.environment.management_endpoint(),
            self.config.subscription_id
        );

        let response = self
            .http_client
            .get(&url)
            .bearer_auth(&access_token)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to list resource groups: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudError::ApiError(format!(
                "Failed to list resource groups: {}",
                error_text
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to parse response: {}", e)))?;

        let groups = result["value"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|g| {
                        Some(CloudResource {
                            id: g["id"].as_str()?.to_string(),
                            name: g["name"].as_str()?.to_string(),
                            resource_type: "Microsoft.Resources/resourceGroups".to_string(),
                            provider: super::CloudProvider::Azure,
                            location: Some(g["location"].as_str()?.to_string()),
                            tags: HashMap::new(),
                            properties: g.as_object().cloned().unwrap_or_default(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(groups)
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_id: &str) -> CloudResult<()> {
        let access_token = self.ensure_token().await?;

        let url = format!("{}?api-version=2023-03-01", resource_id);

        let response = self
            .http_client
            .delete(&url)
            .bearer_auth(&access_token)
            .send()
            .await
            .map_err(|e| CloudError::ApiError(format!("Failed to delete resource: {}", e)))?;

        if !response.status().is_success() && response.status().as_u16() != 202 {
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
    fn test_azure_config_default() {
        let config = AzureConfig::default();
        assert_eq!(config.region, "eastus");
        assert_eq!(config.environment, AzureEnvironment::Public);
    }

    #[test]
    fn test_azure_environment_endpoints() {
        let public = AzureEnvironment::Public;
        assert_eq!(
            public.management_endpoint(),
            "https://management.azure.com"
        );
        assert_eq!(public.storage_suffix(), "core.windows.net");

        let gov = AzureEnvironment::Government;
        assert_eq!(
            gov.management_endpoint(),
            "https://management.usgovcloudapi.net"
        );
    }

    #[test]
    fn test_azure_credentials() {
        let creds = AzureCredentials::new(
            "tenant-123",
            "client-456",
            "secret-789",
            "sub-abc",
        );

        assert_eq!(creds.tenant_id, "tenant-123");
        assert_eq!(creds.client_id, "client-456");
        assert!(creds.is_token_expired());
    }

    #[test]
    fn test_vm_size() {
        let size = VmSize::standard_d2s_v3();
        assert_eq!(size.name, "Standard_D2s_v3");
        assert_eq!(size.number_of_cores, 2);
        assert_eq!(size.memory_in_mb, 8192);
    }

    #[test]
    fn test_image_reference() {
        let ubuntu = ImageReference::ubuntu_22_04();
        assert_eq!(ubuntu.publisher, "Canonical");
        assert_eq!(ubuntu.offer, "UbuntuServer");
        assert_eq!(ubuntu.sku, "22_04-lts");

        let windows = ImageReference::windows_server_2022();
        assert_eq!(windows.publisher, "MicrosoftWindowsServer");
        assert_eq!(windows.sku, "2022-Datacenter");
    }

    #[test]
    fn test_storage_account_type() {
        assert_eq!(StorageAccountType::StandardLrs.as_str(), "Standard_LRS");
        assert_eq!(StorageAccountType::PremiumLrs.as_str(), "Premium_LRS");
    }

    #[test]
    fn test_storage_account_kind() {
        assert_eq!(StorageAccountKind::StorageV2.as_str(), "StorageV2");
        assert_eq!(StorageAccountKind::BlobStorage.as_str(), "BlobStorage");
    }

    #[test]
    fn test_access_tier() {
        assert_eq!(AccessTier::Hot.as_str(), "Hot");
        assert_eq!(AccessTier::Cool.as_str(), "Cool");
    }

    #[test]
    fn test_security_rule_protocol() {
        assert_eq!(SecurityRuleProtocol::Tcp.as_str(), "Tcp");
        assert_eq!(SecurityRuleProtocol::Udp.as_str(), "Udp");
        assert_eq!(SecurityRuleProtocol::Asterisk.as_str(), "*");
    }

    #[test]
    fn test_network_plugin() {
        let azure = NetworkPlugin::Azure;
        let kubenet = NetworkPlugin::Kubenet;

        // These are used in JSON serialization
        match azure {
            NetworkPlugin::Azure => assert!(true),
            _ => panic!("Expected Azure"),
        }
        match kubenet {
            NetworkPlugin::Kubenet => assert!(true),
            _ => panic!("Expected Kubenet"),
        }
    }

    #[test]
    fn test_virtual_machine_config_serialization() {
        let config = VirtualMachineConfig {
            name: "test-vm".to_string(),
            resource_group: "test-rg".to_string(),
            location: "eastus".to_string(),
            vm_size: VmSize::standard_d2s_v3(),
            os_profile: OsProfile {
                computer_name: "testvm".to_string(),
                admin_username: "adminuser".to_string(),
                admin_password: None,
                ssh_public_key: None,
                custom_data: None,
                windows_configuration: None,
                linux_configuration: Some(LinuxConfiguration {
                    disable_password_authentication: true,
                    ssh: None,
                    provision_vm_agent: true,
                }),
            },
            network_profile: NetworkProfile {
                network_interfaces: vec![],
                enable_accelerated_networking: false,
            },
            storage_profile: StorageProfile {
                os_disk: OsDisk {
                    name: "test-os-disk".to_string(),
                    disk_size_gb: 128,
                    storage_account_type: StorageAccountType::PremiumLrs,
                    caching: CachingType::ReadWrite,
                    create_option: DiskCreateOption::FromImage,
                    managed_disk_id: None,
                },
                data_disks: vec![],
                image_reference: ImageReference::ubuntu_22_04(),
            },
            availability_set: None,
            identity: None,
            tags: HashMap::new(),
            zones: vec![],
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("test-vm"));
        assert!(json.contains("Standard_D2s_v3"));
    }

    #[test]
    fn test_aks_cluster_config() {
        let config = AksClusterConfig {
            name: "test-aks".to_string(),
            resource_group: "test-rg".to_string(),
            location: "eastus".to_string(),
            dns_prefix: "testaks".to_string(),
            kubernetes_version: "1.28".to_string(),
            agent_pools: vec![AgentPoolConfig {
                name: "agentpool".to_string(),
                vm_size: "Standard_D2s_v3".to_string(),
                os_disk_size_gb: 128,
                count: 3,
                min_count: Some(1),
                max_count: Some(10),
                enable_auto_scaling: true,
                os_type: OsType::Linux,
                max_pods: Some(30),
                vnet_subnet_id: None,
                availability_zones: vec!["1".to_string(), "2".to_string(), "3".to_string()],
                node_labels: HashMap::new(),
                node_taints: vec![],
                mode: AgentPoolMode::System,
                enable_node_public_ip: false,
                scale_down_mode: ScaleDownMode::Delete,
                spot_configuration: None,
            }],
            network_profile: AksNetworkProfile {
                network_plugin: NetworkPlugin::Azure,
                network_policy: Some(NetworkPolicy::Azure),
                pod_cidr: None,
                service_cidr: Some("10.0.0.0/16".to_string()),
                dns_service_ip: Some("10.0.0.10".to_string()),
                docker_bridge_cidr: None,
                outbound_type: OutboundType::LoadBalancer,
                load_balancer_sku: LoadBalancerSku::Standard,
            },
            aad_profile: Some(AadProfile {
                managed: true,
                enable_azure_rbac: true,
                admin_group_object_ids: vec![],
                tenant_id: None,
            }),
            addons: AksAddons {
                http_application_routing: true,
                monitoring: Some(MonitoringProfile {
                    log_analytics_workspace_resource_id: None,
                    enabled: true,
                }),
                azure_policy: true,
                gitops: false,
                open_service_mesh: false,
                azure_keyvault_secrets_provider: true,
                keda: false,
                virtual_node: false,
            },
            auto_scaler_profile: None,
            api_server_access_profile: None,
            identity_profile: None,
            tags: HashMap::new(),
        };

        assert_eq!(config.name, "test-aks");
        assert_eq!(config.agent_pools.len(), 1);
        assert!(config.agent_pools[0].enable_auto_scaling);
    }

    #[test]
    fn test_storage_account_config() {
        let config = StorageAccountConfig {
            name: "teststorageacct".to_string(),
            resource_group: "test-rg".to_string(),
            location: "eastus".to_string(),
            kind: StorageAccountKind::StorageV2,
            sku: StorageSku {
                name: StorageSkuName::StandardLrs,
                tier: StorageTier::Standard,
            },
            access_tier: AccessTier::Hot,
            enable_https_traffic_only: true,
            minimum_tls_version: TlsVersion::Tls1_2,
            allow_blob_public_access: false,
            network_rule_set: None,
            encryption: EncryptionSettings {
                key_source: KeySource::MicrosoftStorage,
                key_vault_properties: None,
                services: EncryptionServices {
                    blob: EncryptionService {
                        enabled: true,
                        key_type: Some(KeyType::Account),
                    },
                    file: EncryptionService {
                        enabled: true,
                        key_type: None,
                    },
                    table: EncryptionService {
                        enabled: true,
                        key_type: None,
                    },
                    queue: EncryptionService {
                        enabled: true,
                        key_type: None,
                    },
                },
            },
            tags: HashMap::new(),
        };

        assert_eq!(config.name, "teststorageacct");
        assert!(config.enable_https_traffic_only);
    }

    #[test]
    fn test_virtual_network_config() {
        let config = VirtualNetworkConfig {
            name: "test-vnet".to_string(),
            resource_group: "test-rg".to_string(),
            location: "eastus".to_string(),
            address_space: vec!["10.0.0.0/16".to_string()],
            subnets: vec![
                SubnetConfig {
                    name: "default".to_string(),
                    address_prefix: "10.0.0.0/24".to_string(),
                    network_security_group: None,
                    route_table: None,
                    service_endpoints: vec![],
                    private_endpoints: vec![],
                    private_link_service_network_policies: false,
                    nat_gateway: None,
                },
            ],
            dns_servers: vec![],
            enable_ddos_protection: false,
            peerings: vec![],
            tags: HashMap::new(),
        };

        assert_eq!(config.name, "test-vnet");
        assert_eq!(config.subnets.len(), 1);
    }

    #[test]
    fn test_network_security_group() {
        let config = NetworkSecurityGroupConfig {
            name: "test-nsg".to_string(),
            resource_group: "test-rg".to_string(),
            location: "eastus".to_string(),
            security_rules: vec![
                SecurityRule {
                    name: "AllowSSH".to_string(),
                    description: Some("Allow SSH access".to_string()),
                    protocol: SecurityRuleProtocol::Tcp,
                    source_port_range: Some("*".to_string()),
                    source_port_ranges: vec![],
                    destination_port_range: Some("22".to_string()),
                    destination_port_ranges: vec![],
                    source_address_prefix: Some("*".to_string()),
                    source_address_prefixes: vec![],
                    destination_address_prefix: Some("*".to_string()),
                    destination_address_prefixes: vec![],
                    access: SecurityRuleAccess::Allow,
                    priority: 100,
                    direction: SecurityRuleDirection::Inbound,
                },
            ],
            tags: HashMap::new(),
        };

        assert_eq!(config.security_rules.len(), 1);
        assert_eq!(config.security_rules[0].priority, 100);
    }
}