/// Service Mesh Module
/// 
/// This module provides service mesh integration for cloud-native applications,
/// supporting Istio and Linkerd for service-to-service communication, traffic
/// management, and security.
/// 
/// Features:
/// - Service mesh provider support (Istio, Linkerd)
/// - Traffic management (routing, splitting, mirroring)
/// - Security features (mTLS, policies)
/// - Observability (metrics, logs, tracing)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::mod::{CloudError, CloudProvider};

/// Service mesh configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMesh {
    /// Service mesh name
    pub name: String,
    /// Namespace
    pub namespace: String,
    /// Provider
    pub provider: ServiceMeshProvider,
    /// Version
    pub version: Option<String>,
    /// Traffic management
    #[serde(rename = "trafficManagement")]
    pub traffic_management: Option<TrafficManagement>,
    /// Security configuration
    #[serde(rename = "securityConfig")]
    pub security_config: Option<SecurityConfig>,
    /// Observability configuration
    #[serde(rename = "observabilityConfig")]
    pub observability_config: Option<ObservabilityConfig>,
    /// Service mesh status
    pub status: ServiceMeshStatus,
}

impl ServiceMesh {
    /// Create a new service mesh
    pub fn new(name: impl Into<String>, provider: ServiceMeshProvider) -> Self {
        Self {
            name: name.into(),
            namespace: "istio-system".to_string(),
            provider,
            version: None,
            traffic_management: None,
            security_config: None,
            observability_config: None,
            status: ServiceMeshStatus::default(),
        }
    }

    /// Set the namespace
    pub fn set_namespace(&mut self, namespace: impl Into<String>) -> &mut Self {
        self.namespace = namespace.into();
        self
    }

    /// Set the provider
    pub fn set_provider(&mut self, provider: ServiceMeshProvider) -> &mut Self {
        self.provider = provider;
        self
    }

    /// Set traffic management
    pub fn set_traffic_management(&mut self, tm: TrafficManagement) -> &mut Self {
        self.traffic_management = Some(tm);
        self
    }

    /// Set security configuration
    pub fn set_security_config(&mut self, sc: SecurityConfig) -> &mut Self {
        self.security_config = Some(sc);
        self
    }

    /// Set observability configuration
    pub fn set_observability_config(&mut self, oc: ObservabilityConfig) -> &mut Self {
        self.observability_config = Some(oc);
        self
    }

    /// Validate service mesh configuration
    pub fn validate(&self) -> Result<(), CloudError> {
        if self.name.is_empty() {
            return Err(CloudError::ValidationFailed("Service mesh name is required".to_string()));
        }
        Ok(())
    }
}

/// Service mesh provider
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceMeshProvider {
    Istio,
    Linkerd,
    Consul,
    AppMesh,
    Custom(String),
}

/// Service mesh status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceMeshStatus {
    /// Health status
    pub healthy: bool,
    /// Version
    pub version: Option<String>,
    /// Control plane status
    #[serde(rename = "controlPlaneStatus")]
    pub control_plane_status: String,
    /// Data plane status
    #[serde(rename = "dataPlaneStatus")]
    pub data_plane_status: String,
    /// Connected services
    #[serde(rename = "connectedServices")]
    pub connected_services: i32,
}

/// Traffic management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficManagement {
    /// Virtual services
    #[serde(rename = "virtualServices")]
    pub virtual_services: Vec<VirtualService>,
    /// Destination rules
    #[serde(rename = "destinationRules")]
    pub destination_rules: Vec<DestinationRule>,
    /// Gateways
    pub gateways: Vec<Gateway>,
}

impl Default for TrafficManagement {
    fn default() -> Self {
        Self {
            virtual_services: Vec::new(),
            destination_rules: Vec::new(),
            gateways: Vec::new(),
        }
    }
}

/// Virtual service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualService {
    /// Virtual service name
    pub name: String,
    /// Hosts
    pub hosts: Vec<String>,
    /// Gateways
    pub gateways: Vec<String>,
    /// HTTP routes
    pub http: Vec<HTTPRoute>,
    /// TCP routes
    pub tcp: Vec<TCPRoute>,
}

impl VirtualService {
    /// Create a new virtual service
    pub fn new(name: impl Into<String>, hosts: Vec<String>) -> Self {
        Self {
            name: name.into(),
            hosts,
            gateways: Vec::new(),
            http: Vec::new(),
            tcp: Vec::new(),
        }
    }

    /// Add an HTTP route
    pub fn add_http_route(&mut self, route: HTTPRoute) -> &mut Self {
        self.http.push(route);
        self
    }

    /// Add a TCP route
    pub fn add_tcp_route(&mut self, route: TCPRoute) -> &mut Self {
        self.tcp.push(route);
        self
    }

    /// Add a gateway
    pub fn add_gateway(&mut self, gateway: impl Into<String>) -> &mut Self {
        self.gateways.push(gateway.into());
        self
    }
}

/// HTTP route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPRoute {
    /// Match conditions
    pub match_conditions: Vec<HTTPMatchCondition>,
    /// Route destination
    pub route: Vec<DestinationWeight>,
    /// Fault injection
    #[serde(rename = "fault")]
    pub fault: Option<FaultInjection>,
    /// Timeout
    pub timeout: Option<String>,
    /// Retries
    pub retries: Option<RetryPolicy>,
}

/// HTTP match condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPMatchCondition {
    /// URI match
    pub uri: Option<Match>,
    /// Method match
    pub method: Option<String>,
    /// Headers match
    pub headers: Option<HashMap<String, Match>>,
    /// Query parameters match
    #[serde(rename = "queryParams")]
    pub query_params: Option<HashMap<String, Match>>,
}

/// Match rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    /// Exact match
    pub exact: Option<String>,
    /// Prefix match
    pub prefix: Option<String>,
    /// Regex match
    pub regex: Option<String>,
}

/// Destination weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationWeight {
    /// Destination
    pub destination: Destination,
    /// Weight
    pub weight: i32,
}

/// Destination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Destination {
    /// Host
    pub host: String,
    /// Subset
    pub subset: Option<String>,
    /// Port
    pub port: Option<PortSelector>,
}

/// Port selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortSelector {
    /// Number
    pub number: Option<i32>,
    /// Name
    pub name: Option<String>,
}

/// Fault injection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultInjection {
    /// Delay
    pub delay: Option<Delay>,
    /// Abort
    pub abort: Option<Abort>,
}

/// Delay fault injection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delay {
    /// Fixed delay
    #[serde(rename = "fixedDelay")]
    pub fixed_delay: Option<String>,
    /// Percentage
    pub percentage: Option<i32>,
}

/// Abort fault injection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Abort {
    /// HTTP status code
    #[serde(rename = "httpStatus")]
    pub http_status: Option<i32>,
    /// Percentage
    pub percentage: Option<i32>,
}

/// Retry policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Attempts
    pub attempts: i32,
    /// Per try timeout
    #[serde(rename = "perTryTimeout")]
    pub per_try_timeout: Option<String>,
}

/// TCP route
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCPRoute {
    /// Match conditions
    pub match_conditions: Vec<TCPMatchCondition>,
    /// Route destination
    pub route: Vec<DestinationWeight>,
}

/// TCP match condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCPMatchCondition {
    /// Ports
    pub ports: Option<Vec<i32>>,
    /// Source subnets
    #[serde(rename = "sourceSubnet")]
    pub source_subnet: Option<Vec<String>>,
}

/// Destination rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationRule {
    /// Destination rule name
    pub name: String,
    /// Host
    pub host: String,
    /// Traffic policy
    #[serde(rename = "trafficPolicy")]
    pub traffic_policy: Option<TrafficPolicy>,
    /// Subsets
    pub subsets: Vec<Subset>,
}

impl DestinationRule {
    /// Create a new destination rule
    pub fn new(name: impl Into<String>, host: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            host: host.into(),
            traffic_policy: None,
            subsets: Vec::new(),
        }
    }

    /// Set traffic policy
    pub fn set_traffic_policy(&mut self, tp: TrafficPolicy) -> &mut Self {
        self.traffic_policy = Some(tp);
        self
    }

    /// Add a subset
    pub fn add_subset(&mut self, subset: Subset) -> &mut Self {
        self.subsets.push(subset);
        self
    }
}

/// Traffic policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPolicy {
    /// Load balancer
    #[serde(rename = "loadBalancer")]
    pub load_balancer: Option<LoadBalancer>,
    /// Connection pool
    #[serde(rename = "connectionPool")]
    pub connection_pool: Option<ConnectionPoolSettings>,
    /// Circuit breaker
    #[serde(rename = "circuitBreaker")]
    pub circuit_breaker: Option<CircuitBreaker>,
    /// Outlier detection
    #[serde(rename = "outlierDetection")]
    pub outlier_detection: Option<OutlierDetection>,
}

/// Load balancer settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancer {
    /// Simple
    #[serde(rename = "simple")]
    pub simple: Option<String>,
    /// Consistent hash
    #[serde(rename = "consistentHash")]
    pub consistent_hash: Option<ConsistentHash>,
}

/// Consistent hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistentHash {
    /// HTTP header name
    #[serde(rename = "httpHeaderName")]
    pub http_header_name: Option<String>,
    /// HTTP cookie
    #[serde(rename = "httpCookie")]
    pub http_cookie: Option<HTTPCookie>,
    /// Use source IP
    #[serde(rename = "useSourceIp")]
    pub use_source_ip: Option<bool>,
}

/// HTTP cookie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPCookie {
    /// Name
    pub name: Option<String>,
    /// TTL
    pub ttl: Option<String>,
}

/// Connection pool settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolSettings {
    /// TCP settings
    pub tcp: Option<TCPSettings>,
    /// HTTP settings
    pub http: Option<HTTPSettings>,
}

/// TCP settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCPSettings {
    /// Max connections
    #[serde(rename = "maxConnections")]
    pub max_connections: Option<i32>,
    /// Connect timeout
    #[serde(rename = "connectTimeout")]
    pub connect_timeout: Option<String>,
}

/// HTTP settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPSettings {
    /// HTTP/1.1 settings
    #[serde(rename = "http1MaxPendingRequests")]
    pub http1_max_pending_requests: Option<i32>,
    /// HTTP/2 max requests
    #[serde(rename = "http2MaxRequests")]
    pub http2_max_requests: Option<i32>,
    /// Max requests per connection
    #[serde(rename = "maxRequestsPerConnection")]
    pub max_requests_per_connection: Option<i32>,
    /// Idle timeout
    #[serde(rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
}

/// Circuit breaker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreaker {
    /// Consecutive errors
    #[serde(rename = "consecutiveErrors")]
    pub consecutive_errors: Option<i32>,
    /// Interval
    pub interval: Option<String>,
    /// Base ejection time
    #[serde(rename = "baseEjectionTime")]
    pub base_ejection_time: Option<String>,
    /// Max ejection percent
    #[serde(rename = "maxEjectionPercent")]
    pub max_ejection_percent: Option<i32>,
}

/// Outlier detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlierDetection {
    /// Consecutive errors
    #[serde(rename = "consecutiveErrors")]
    pub consecutive_errors: Option<i32>,
    /// Interval
    pub interval: Option<String>,
    /// Base ejection time
    #[serde(rename = "baseEjectionTime")]
    pub base_ejection_time: Option<String>,
    /// Success rate minimum hosts
    #[serde(rename = "successRateMinimumHosts")]
    pub success_rate_minimum_hosts: Option<i32>,
    /// Success rate standard deviation
    #[serde(rename = "successRateStdevFactor")]
    pub success_rate_stdev_factor: Option<i32>,
}

/// Subset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subset {
    /// Name
    pub name: String,
    /// Labels
    pub labels: HashMap<String, String>,
}

/// Gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gateway {
    /// Gateway name
    pub name: String,
    /// Selector
    pub selector: HashMap<String, String>,
    /// Servers
    pub servers: Vec<Server>,
}

impl Gateway {
    /// Create a new gateway
    pub fn new(name: impl Into<String>, selector: HashMap<String, String>) -> Self {
        Self {
            name: name.into(),
            selector,
            servers: Vec::new(),
        }
    }

    /// Add a server
    pub fn add_server(&mut self, server: Server) -> &mut Self {
        self.servers.push(server);
        self
    }
}

/// Server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// Port
    pub port: Port,
    /// Hosts
    pub hosts: Vec<String>,
    /// TLS
    pub tls: Option<TLSOptions>,
}

/// Port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    /// Number
    pub number: i32,
    /// Name
    pub name: String,
    /// Protocol
    pub protocol: String,
}

/// TLS options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TLSOptions {
    /// Mode
    #[serde(rename = "mode")]
    pub mode: String,
    /// Server name
    #[serde(rename = "serverName")]
    pub server_name: Option<String>,
    /// Credential name
    #[serde(rename = "credentialName")]
    pub credential_name: Option<String>,
    /// Min protocol version
    #[serde(rename = "minProtocolVersion")]
    pub min_protocol_version: Option<String>,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// mTLS mode
    #[serde(rename = "mtlsMode")]
    pub mtls_mode: MTLSMode,
    /// Authentication policies
    #[serde(rename = "authPolicies")]
    pub auth_policies: Vec<AuthenticationPolicy>,
    /// Authorization policies
    #[serde(rename = "authorizationPolicies")]
    pub authorization_policies: Vec<AuthorizationPolicy>,
    /// Request authentication
    #[serde(rename = "requestAuthentication")]
    pub request_authentication: Vec<RequestAuthentication>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            mtls_mode: MTLSMode::Permissive,
            auth_policies: Vec::new(),
            authorization_policies: Vec::new(),
            request_authentication: Vec::new(),
        }
    }
}

/// mTLS mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MTLSMode {
    Strict,
    Permissive,
    Disable,
}

/// Authentication policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationPolicy {
    /// Policy name
    pub name: String,
    /// Selector
    pub selector: HashMap<String, String>,
    /// Origins
    pub origins: Vec<JwtOrigin>,
}

impl AuthenticationPolicy {
    /// Create a new authentication policy
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            selector: HashMap::new(),
            origins: Vec::new(),
        }
    }

    /// Add a JWT origin
    pub fn add_jwt_origin(&mut self, origin: JwtOrigin) -> &mut Self {
        self.origins.push(origin);
        self
    }
}

/// JWT origin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtOrigin {
    /// Issuer
    pub issuer: String,
    /// Jwks
    pub jwks: String,
}

/// Authorization policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationPolicy {
    /// Policy name
    pub name: String,
    /// Selector
    pub selector: HashMap<String, String>,
    /// Rules
    pub rules: Vec<AuthorizationRule>,
}

impl AuthorizationPolicy {
    /// Create a new authorization policy
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            selector: HashMap::new(),
            rules: Vec::new(),
        }
    }

    /// Add a rule
    pub fn add_rule(&mut self, rule: AuthorizationRule) -> &mut Self {
        self.rules.push(rule);
        self
    }
}

/// Authorization rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRule {
    /// From
    pub from: Vec<From>,
    /// To
    pub to: Vec<To>,
    /// When
    #[serde(rename = "when")]
    pub when_clause: Vec<Condition>,
}

/// From clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct From {
    /// Source
    pub source: Option<Source>,
}

/// To clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct To {
    /// Operation
    pub operation: Option<Operation>,
}

/// Source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// Principals
    pub principals: Option<Vec<String>>,
    /// Not principals
    #[serde(rename = "notPrincipals")]
    pub not_principals: Option<Vec<String>>,
    /// Request principals
    #[serde(rename = "requestPrincipals")]
    pub request_principals: Option<Vec<String>>,
}

/// Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// Hosts
    pub hosts: Option<Vec<String>>,
    /// Methods
    pub methods: Option<Vec<String>>,
    /// Ports
    pub ports: Option<Vec<i32>>,
}

/// Condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// Key
    pub key: String,
    /// Values
    pub values: Vec<String>,
}

/// Request authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestAuthentication {
    /// Name
    pub name: String,
    /// Selector
    pub selector: HashMap<String, String>,
    /// JWT rules
    #[serde(rename = "jwtRules")]
    pub jwt_rules: Vec<JwtRule>,
}

impl RequestAuthentication {
    /// Create a new request authentication
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            selector: HashMap::new(),
            jwt_rules: Vec::new(),
        }
    }

    /// Add a JWT rule
    pub fn add_jwt_rule(&mut self, rule: JwtRule) -> &mut Self {
        self.jwt_rules.push(rule);
        self
    }
}

/// JWT rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtRule {
    /// Issuer
    pub issuer: String,
    /// Jwks
    pub jwks: String,
}

/// Observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Metrics enabled
    #[serde(rename = "metricsEnabled")]
    pub metrics_enabled: bool,
    /// Logs enabled
    #[serde(rename = "logsEnabled")]
    pub logs_enabled: bool,
    /// Tracing enabled
    #[serde(rename = "tracingEnabled")]
    pub tracing_enabled: bool,
    /// Access log policy
    #[serde(rename = "accessLogPolicy")]
    pub access_log_policy: Option<AccessLogPolicy>,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            logs_enabled: true,
            tracing_enabled: true,
            access_log_policy: None,
        }
    }
}

/// Access log policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLogPolicy {
    /// Format
    pub format: String,
    /// Filter
    pub filter: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_mesh_creation() {
        let sm = ServiceMesh::new("test-mesh", ServiceMeshProvider::Istio);
        assert_eq!(sm.name, "test-mesh");
        assert_eq!(sm.provider, ServiceMeshProvider::Istio);
    }

    #[test]
    fn test_virtual_service_creation() {
        let vs = VirtualService::new("test-vs", vec!["example.com".to_string()]);
        assert_eq!(vs.name, "test-vs");
        assert_eq!(vs.hosts, vec!["example.com".to_string()]);
    }

    #[test]
    fn test_gateway_creation() {
        let mut selector = HashMap::new();
        selector.insert("istio".to_string(), "ingressgateway".to_string());
        let gw = Gateway::new("test-gw", selector);
        assert_eq!(gw.name, "test-gw");
    }

    #[test]
    fn test_service_mesh_validation() {
        let sm = ServiceMesh::new("test-mesh", ServiceMeshProvider::Istio);
        assert!(sm.validate().is_ok());
        
        let sm = ServiceMesh::new("", ServiceMeshProvider::Istio);
        assert!(sm.validate().is_err());
    }
}