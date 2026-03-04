<content>
//! AI Gateway Module
//! Provides a gateway for AI services, handling authentication,
//! rate limiting, routing, and external service integration.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// AI Gateway Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Enable authentication
    pub enable_auth: bool,
    
    /// Enable rate limiting
    pub enable_rate_limiting: bool,
    
    /// Enable request routing
    pub enable_routing: bool,
    
    /// Enable request caching
    pub enable_caching: bool,
    
    /// Maximum requests per minute per client
    pub max_requests_per_minute: u32,
    
    /// Maximum concurrent connections
    pub max_connections: u32,
    
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    
    /// Enable load balancing
    pub enable_load_balancing: bool,
    
    /// Enable health checking
    pub enable_health_check: bool,
    
    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            enable_auth: true,
            enable_rate_limiting: true,
            enable_routing: true,
            enable_caching: true,
            max_requests_per_minute: 100,
            max_connections: 1000,
            request_timeout_ms: 30000,
            enable_load_balancing: true,
            enable_health_check: true,
            health_check_interval_seconds: 30,
        }
    }
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Endpoint ID
    pub id: String,
    
    /// Service name
    pub name: String,
    
    /// Service URL
    pub url: String,
    
    /// Service type
    pub service_type: ServiceType,
    
    /// Authentication credentials
    pub auth: Option<ServiceAuth>,
    
    /// Health status
    pub health: ServiceHealth,
    
    /// Load metrics
    pub load: ServiceLoad,
    
    /// Priority (1-10, 10 highest)
    pub priority: u8,
    
    /// Whether service is enabled
    pub enabled: bool,
}

/// Service types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    /// Prediction service
    Prediction,
    
    /// Optimization service
    Optimization,
    
    /// Anomaly detection service
    AnomalyDetection,
    
    /// Natural language service
    NaturalLanguage,
    
    /// Computer vision service
    ComputerVision,
    
    /// Machine learning service
    MachineLearning,
    
    /// Data processing service
    DataProcessing,
    
    /// Custom service
    Custom(String),
}

/// Service authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAuth {
    /// Auth type
    pub auth_type: AuthType,
    
    /// API key (if applicable)
    pub api_key: Option<String>,
    
    /// Token (if applicable)
    pub token: Option<String>,
    
    /// Username (if applicable)
    pub username: Option<String>,
    
    /// Password (if applicable)
    pub password: Option<String>,
}

/// Authentication types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    None,
    ApiKey,
    BearerToken,
    BasicAuth,
    OAuth2,
    Custom,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    /// Whether service is healthy
    pub healthy: bool,
    
    /// Last health check time
    pub last_check: chrono::DateTime<chrono::Utc>,
    
    /// Response time in ms
    pub response_time_ms: f64,
    
    /// Error count
    pub error_count: u64,
    
    /// Success count
    pub success_count: u64,
    
    /// Health score (0.0-1.0)
    pub score: f64,
}

impl Default for ServiceHealth {
    fn default() -> Self {
        Self {
            healthy: true,
            last_check: chrono::Utc::now(),
            response_time_ms: 0.0,
            error_count: 0,
            success_count: 0,
            score: 1.0,
        }
    }
}

/// Service load metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLoad {
    /// Current requests being processed
    pub current_requests: u32,
    
    /// Average response time in ms
    pub avg_response_time_ms: f64,
    
    /// CPU utilization (0.0-1.0)
    pub cpu_utilization: f64,
    
    /// Memory utilization (0.0-1.0)
    pub memory_utilization: f64,
    
    /// Queue depth
    pub queue_depth: u32,
}

impl Default for ServiceLoad {
    fn default() -> Self {
        Self {
            current_requests: 0,
            avg_response_time_ms: 0.0,
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            queue_depth: 0,
        }
    }
}

/// Gateway client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayClient {
    /// Client ID
    pub client_id: String,
    
    /// API key
    pub api_key: String,
    
    /// Client name
    pub name: String,
    
    /// Rate limit tier
    pub rate_limit_tier: RateLimitTier,
    
    /// Permissions
    pub permissions: Vec<Permission>,
    
    /// Request count
    pub request_count: u64,
    
    /// Last request time
    pub last_request: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Enabled status
    pub enabled: bool,
}

/// Rate limit tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitTier {
    Free,
    Basic,
    Professional,
    Enterprise,
    Unlimited,
}

impl RateLimitTier {
    pub fn requests_per_minute(&self) -> u32 {
        match self {
            RateLimitTier::Free => 10,
            RateLimitTier::Basic => 60,
            RateLimitTier::Professional => 300,
            RateLimitTier::Enterprise => 1000,
            RateLimitTier::Unlimited => u32::MAX,
        }
    }
}

/// Permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    Predict,
    Optimize,
    DetectAnomalies,
    AnalyzePatterns,
    ProcessNaturalLanguage,
    ProcessVision,
    ManageServices,
    ViewMetrics,
    Admin,
}

/// Gateway request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayRequest {
    /// Request ID
    pub request_id: String,
    
    /// Client ID
    pub client_id: String,
    
    /// Target service type
    pub service_type: ServiceType,
    
    /// Request payload
    pub payload: serde_json::Value,
    
    /// Request headers
    pub headers: HashMap<String, String>,
    
    /// Priority
    pub priority: u8,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Timeout override
    pub timeout_ms: Option<u64>,
}

/// Gateway response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayResponse {
    /// Request ID
    pub request_id: String,
    
    /// Response status
    pub status: GatewayStatus,
    
    /// Response data
    pub data: Option<serde_json::Value>,
    
    /// Error message
    pub error: Option<String>,
    
    /// Processing time in ms
    pub processing_time_ms: f64,
    
    /// Service that handled the request
    pub service_id: Option<String>,
    
    /// Cache hit
    pub cache_hit: bool,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Gateway status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GatewayStatus {
    Success,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    RateLimited,
    Timeout,
    ServiceUnavailable,
    InternalError,
}

/// Rate limit record
#[derive(Debug, Clone)]
struct RateLimitRecord {
    requests: VecDeque<chrono::DateTime<chrono::Utc>>,
}

/// Cached response
#[derive(Debug, Clone)]
struct CachedResponse {
    response: GatewayResponse,
    cached_at: chrono::DateTime<chrono::Utc>,
    ttl_seconds: u64,
}

/// Routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Rule ID
    pub id: String,
    
    /// Rule name
    pub name: String,
    
    /// Condition
    pub condition: RoutingCondition,
    
    /// Target service IDs
    pub targets: Vec<String>,
    
    /// Load balancing strategy
    pub strategy: LoadBalancingStrategy,
    
    /// Priority
    pub priority: u8,
    
    /// Enabled
    pub enabled: bool,
}

/// Routing conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingCondition {
    All,
    ServiceType(ServiceType),
    ClientId(String),
    PriorityRange(u8, u8),
    Custom(String),
}

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    LeastResponseTime,
    Weighted,
    Random,
}

/// AI Gateway
pub struct AiGateway {
    config: GatewayConfig,
    services: HashMap<String, ServiceEndpoint>,
    clients: HashMap<String, GatewayClient>,
    rate_limits: HashMap<String, RateLimitRecord>,
    response_cache: HashMap<String, CachedResponse>,
    routing_rules: Vec<RoutingRule>,
    request_log: VecDeque<GatewayRequest>,
    metrics: GatewayMetrics,
}

/// Gateway metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub rate_limited_requests: u64,
    pub cached_responses: u64,
    pub avg_response_time_ms: f64,
    pub active_connections: u32,
    pub uptime_seconds: u64,
}

impl Default for GatewayMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            rate_limited_requests: 0,
            cached_responses: 0,
            avg_response_time_ms: 0.0,
            active_connections: 0,
            uptime_seconds: 0,
        }
    }
}

impl AiGateway {
    /// Create a new AI gateway
    pub fn new(config: GatewayConfig) -> Self {
        Self {
            config,
            services: HashMap::new(),
            clients: HashMap::new(),
            rate_limits: HashMap::new(),
            response_cache: HashMap::new(),
            routing_rules: Vec::new(),
            request_log: VecDeque::new(),
            metrics: GatewayMetrics::default(),
        }
    }
    
    /// Create with default configuration
    pub fn default_gateway() -> Self {
        Self::new(GatewayConfig::default())
    }
    
    /// Register a service
    pub fn register_service(&mut self, endpoint: ServiceEndpoint) {
        self.services.insert(endpoint.id.clone(), endpoint);
    }
    
    /// Unregister a service
    pub fn unregister_service(&mut self, service_id: &str) {
        self.services.remove(service_id);
    }
    
    /// Register a client
    pub fn register_client(&mut self, client: GatewayClient) {
        self.clients.insert(client.client_id.clone(), client);
    }
    
    /// Unregister a client
    pub fn unregister_client(&mut self, client_id: &str) {
        self.clients.remove(client_id);
        self.rate_limits.remove(client_id);
    }
    
    /// Process a gateway request
    pub fn process_request(&mut self, request: GatewayRequest) -> Result<GatewayResponse, GatewayError> {
        let start_time = Instant::now();
        let request_id = request.request_id.clone();
        
        // Update metrics
        self.metrics.total_requests += 1;
        
        // Validate client
        let client = self.validate_client(&request.client_id)?;
        
        // Check rate limit
        if self.config.enable_rate_limiting {
            self.check_rate_limit(&request.client_id, &client.rate_limit_tier)?;
        }
        
        // Check cache
        if self.config.enable_caching {
            let cache_key = self.generate_cache_key(&request);
            if let Some(cached) = self.get_cached_response(&cache_key) {
                self.metrics.cached_responses += 1;
                return Ok(cached);
            }
        }
        
        // Route request to service
        let service_id = self.route_request(&request)?;
        
        // Execute request
        let response = self.execute_request(&request, &service_id)?;
        
        // Update metrics
        let processing_time = start_time.elapsed().as_millis() as f64;
        self.update_metrics(&response, processing_time);
        
        // Cache response if successful
        if self.config.enable_caching && response.status == GatewayStatus::Success {
            let cache_key = self.generate_cache_key(&request);
            self.cache_response(cache_key, response.clone());
        }
        
        // Log request
        self.log_request(request);
        
        Ok(response)
    }
    
    /// Validate client
    fn validate_client(&self, client_id: &str) -> Result<&GatewayClient, GatewayError> {
        let client = self.clients.get(client_id)
            .ok_or_else(|| GatewayError::ClientNotFound(client_id.to_string()))?;
        
        if !client.enabled {
            return Err(GatewayError::ClientDisabled(client_id.to_string()));
        }
        
        Ok(client)
    }
    
    /// Check rate limit
    fn check_rate_limit(&mut self, client_id: &str, tier: &RateLimitTier) -> Result<(), GatewayError> {
        let now = chrono::Utc::now();
        let limit = tier.requests_per_minute();
        
        let record = self.rate_limits.entry(client_id.to_string())
            .or_insert(RateLimitRecord {
                requests: VecDeque::new(),
            });
        
        // Remove requests older than 1 minute
        let one_minute_ago = now - chrono::Duration::seconds(60);
        while let Some(&front) = record.requests.front() {
            if front < one_minute_ago {
                record.requests.pop_front();
            } else {
                break;
            }
        }
        
        // Check if limit exceeded
        if record.requests.len() >= limit as usize {
            self.metrics.rate_limited_requests += 1;
            return Err(GatewayError::RateLimitExceeded);
        }
        
        // Add current request
        record.requests.push_back(now);
        
        Ok(())
    }
    
    /// Route request to appropriate service
    fn route_request(&self, request: &GatewayRequest) -> Result<String, GatewayError> {
        if !self.config.enable_routing {
            // Return first available service
            return self.services.keys().next()
                .cloned()
                .ok_or(GatewayError::NoServiceAvailable);
        }
        
        // Find matching routing rules
        let matching_rules: Vec<_> = self.routing_rules.iter()
            .filter(|r| r.enabled && self.matches_condition(&r.condition, request))
            .collect();
        
        if matching_rules.is_empty() {
            // Default routing - find service by type
            let service = self.services.values()
                .filter(|s| s.enabled && self.service_matches_type(s, &request.service_type))
                .min_by_key(|s| s.priority)
                .ok_or(GatewayError::NoServiceAvailable)?;
            
            return Ok(service.id.clone());
        }
        
        // Apply highest priority rule
        let rule = matching_rules.iter()
            .max_by_key(|r| r.priority)
            .unwrap();
        
        // Select target based on strategy
        let target_id = self.select_target(&rule.targets, &rule.strategy)?;
        
        Ok(target_id)
    }
    
    /// Check if condition matches request
    fn matches_condition(&self, condition: &RoutingCondition, request: &GatewayRequest) -> bool {
        match condition {
            RoutingCondition::All => true,
            RoutingCondition::ServiceType(st) => std::mem::discriminant(st) == std::mem::discriminant(&request.service_type),
            RoutingCondition::ClientId(id) => request.client_id == *id,
            RoutingCondition::PriorityRange(min, max) => request.priority >= *min && request.priority <= *max,
            RoutingCondition::Custom(expr) => self.evaluate_custom_condition(expr, request),
        }
    }
    
    /// Evaluate custom condition
    fn evaluate_custom_condition(&self, _expr: &str, _request: &GatewayRequest) -> bool {
        // Simplified - would implement expression parser
        true
    }
    
    /// Check if service matches type
    fn service_matches_type(&self, service: &ServiceEndpoint, service_type: &ServiceType) -> bool {
        std::mem::discriminant(&service.service_type) == std::mem::discriminant(service_type)
    }
    
    /// Select target based on strategy
    fn select_target(&self, targets: &[String], strategy: &LoadBalancingStrategy) -> Result<String, GatewayError> {
        if targets.is_empty() {
            return Err(GatewayError::NoServiceAvailable);
        }
        
        let available_targets: Vec<_> = targets.iter()
            .filter(|id| self.services.get(*id).map(|s| s.enabled && s.health.healthy).unwrap_or(false))
            .collect();
        
        if available_targets.is_empty() {
            return Err(GatewayError::NoServiceAvailable);
        }
        
        match strategy {
            LoadBalancingStrategy::RoundRobin => {
                // Simple round-robin using first available
                Ok(available_targets[0].clone())
            }
            LoadBalancingStrategy::LeastConnections => {
                let min_load = available_targets.iter()
                    .filter_map(|id| self.services.get(*id))
                    .min_by_key(|s| s.load.current_requests);
                Ok(min_load.unwrap().id.clone())
            }
            LoadBalancingStrategy::LeastResponseTime => {
                let fastest = available_targets.iter()
                    .filter_map(|id| self.services.get(*id))
                    .min_by(|a, b| a.load.avg_response_time_ms.partial_cmp(&b.load.avg_response_time_ms).unwrap());
                Ok(fastest.unwrap().id.clone())
            }
            LoadBalancingStrategy::Random => {
                let idx = rand::random::<usize>() % available_targets.len();
                Ok(available_targets[idx].clone())
            }
            LoadBalancingStrategy::Weighted => {
                // Use health score as weight
                let best = available_targets.iter()
                    .filter_map(|id| self.services.get(*id))
                    .max_by(|a, b| a.health.score.partial_cmp(&b.health.score).unwrap());
                Ok(best.unwrap().id.clone())
            }
        }
    }
    
    /// Execute request on service
    fn execute_request(&mut self, request: &GatewayRequest, service_id: &str) -> Result<GatewayResponse, GatewayError> {
        let service = self.services.get_mut(service_id)
            .ok_or_else(|| GatewayError::ServiceNotFound(service_id.to_string()))?;
        
        // Simulate request execution
        service.load.current_requests += 1;
        
        let response = GatewayResponse {
            request_id: request.request_id.clone(),
            status: GatewayStatus::Success,
            data: Some(serde_json::json!({
                "result": "processed",
                "service": service.name
            })),
            error: None,
            processing_time_ms: 10.0,
            service_id: Some(service_id.to_string()),
            cache_hit: false,
            timestamp: chrono::Utc::now(),
        };
        
        // Update service metrics
        service.health.success_count += 1;
        service.load.current_requests -= 1;
        
        Ok(response)
    }
    
    /// Generate cache key
    fn generate_cache_key(&self, request: &GatewayRequest) -> String {
        format!("{:?}_{}_{}", request.service_type, request.client_id, request.payload)
    }
    
    /// Get cached response
    fn get_cached_response(&mut self, cache_key: &str) -> Option<GatewayResponse> {
        if let Some(cached) = self.response_cache.get(cache_key) {
            let now = chrono::Utc::now();
            let age = now.signed_duration_since(cached.cached_at).num_seconds() as u64;
            if age < cached.ttl_seconds {
                let mut response = cached.response.clone();
                response.cache_hit = true;
                return Some(response);
            } else {
                self.response_cache.remove(cache_key);
            }
        }
        None
    }
    
    /// Cache a response
    fn cache_response(&mut self, cache_key: String, response: GatewayResponse) {
        self.response_cache.insert(cache_key, CachedResponse {
            response,
            cached_at: chrono::Utc::now(),
            ttl_seconds: 300, // 5 minutes default
        });
    }
    
    /// Update metrics
    fn update_metrics(&mut self, response: &GatewayResponse, processing_time: f64) {
        if response.status == GatewayStatus::Success {
            self.metrics.successful_requests += 1;
        } else {
            self.metrics.failed_requests += 1;
        }
        
        let total = self.metrics.total_requests as f64;
        self.metrics.avg_response_time_ms = 
            (self.metrics.avg_response_time_ms * (total - 1.0) + processing_time) / total;
    }
    
    /// Log request
    fn log_request(&mut self, request: GatewayRequest) {
        self.request_log.push_back(request);
        
        // Keep limited history
        while self.request_log.len() > 10000 {
            self.request_log.pop_front();
        }
    }
    
    /// Add routing rule
    pub fn add_routing_rule(&mut self, rule: RoutingRule) {
        self.routing_rules.push(rule);
    }
    
    /// Remove routing rule
    pub fn remove_routing_rule(&mut self, rule_id: &str) {
        self.routing_rules.retain(|r| r.id != rule_id);
    }
    
    /// Check service health
    pub fn check_service_health(&mut self, service_id: &str) -> Result<ServiceHealth, GatewayError> {
        let service = self.services.get_mut(service_id)
            .ok_or_else(|| GatewayError::ServiceNotFound(service_id.to_string()))?;
        
        // Simulate health check
        let start = Instant::now();
        let healthy = true; // Would actually ping service
        
        service.health.healthy = healthy;
        service.health.last_check = chrono::Utc::now();
        service.health.response_time_ms = start.elapsed().as_millis() as f64;
        service.health.score = if healthy { 1.0 } else { 0.0 };
        
        Ok(service.health.clone())
    }
    
    /// Check all services health
    pub fn check_all_services(&mut self) -> HashMap<String, ServiceHealth> {
        let service_ids: Vec<String> = self.services.keys().cloned().collect();
        
        let mut results = HashMap::new();
        for id in service_ids {
            if let Ok(health) = self.check_service_health(&id) {
                results.insert(id, health);
            }
        }
        results
    }
    
    /// Get gateway metrics
    pub fn get_metrics(&self) -> GatewayMetrics {
        self.metrics.clone()
    }
    
    /// Get service statistics
    pub fn get_service_stats(&self) -> Vec<ServiceStats> {
        self.services.values().map(|s| ServiceStats {
            id: s.id.clone(),
            name: s.name.clone(),
            healthy: s.health.healthy,
            total_requests: s.health.success_count + s.health.error_count,
            success_rate: if s.health.success_count + s.health.error_count > 0 {
                s.health.success_count as f64 / (s.health.success_count + s.health.error_count) as f64
            } else {
                0.0
            },
            avg_response_time_ms: s.load.avg_response_time_ms,
            current_load: s.load.current_requests,
        }).collect()
    }
    
    /// Get recent requests
    pub fn get_recent_requests(&self, count: usize) -> Vec<&GatewayRequest> {
        self.request_log.iter().rev().take(count).collect()
    }
    
    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.response_cache.clear();
    }
}

/// Service statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStats {
    pub id: String,
    pub name: String,
    pub healthy: bool,
    pub total_requests: u64,
    pub success_rate: f64,
    pub avg_response_time_ms: f64,
    pub current_load: u32,
}

/// Gateway Error
#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("Client not found: {0}")]
    ClientNotFound(String),
    
    #[error("Client disabled: {0}")]
    ClientDisabled(String),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Service not found: {0}")]
    ServiceNotFound(String),
    
    #[error("No service available")]
    NoServiceAvailable,
    
    #[error("Authentication failed")]
    AuthFailed,
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("Request timeout")]
    Timeout,
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_creation() {
        let config = GatewayConfig::default();
        let gateway = AiGateway::new(config);
        
        assert!(gateway.config.enable_auth);
        assert!(gateway.services.is_empty());
    }

    #[test]
    fn test_service_registration() {
        let mut gateway = AiGateway::default_gateway();
        
        let service = ServiceEndpoint {
            id: "svc_001".to_string(),
            name: "Prediction Service".to_string(),
            url: "http://localhost:8080".to_string(),
            service_type: ServiceType::Prediction,
            auth: None,
            health: ServiceHealth::default(),
            load: ServiceLoad::default(),
            priority: 5,
            enabled: true,
        };
        
        gateway.register_service(service);
        assert!(gateway.services.contains_key("svc_001"));
    }

    #[test]
    fn test_client_registration() {
        let mut gateway = AiGateway::default_gateway();
        
        let client = GatewayClient {
            client_id: "client_001".to_string(),
            api_key: "key_123".to_string(),
            name: "Test Client".to_string(),
            rate_limit_tier: RateLimitTier::Basic,
            permissions: vec![Permission::Predict],
            request_count: 0,
            last_request: None,
            enabled: true,
        };
        
        gateway.register_client(client);
        assert!(gateway.clients.contains_key("client_001"));
    }

    #[test]
    fn test_rate_limiting() {
        let mut gateway = AiGateway::default_gateway();
        gateway.config.enable_rate_limiting = true;
        
        let client = GatewayClient {
            client_id: "client_002".to_string(),
            api_key: "key_456".to_string(),
            name: "Free Client".to_string(),
            rate_limit_tier: RateLimitTier::Free,
            permissions: vec![Permission::Predict],
            request_count: 0,
            last_request: None,
            enabled: true,
        };
        
        gateway.register_client(client);
        
        // Free tier allows 10 requests per minute
        for _ in 0..15 {
            let result = gateway.check_rate_limit("client_002", &RateLimitTier::Free);
            if result.is_err() {
                assert!(matches!(result, Err(GatewayError::RateLimitExceeded)));
                return;
            }
        }
        
        // Should have been rate limited
        panic!("Expected rate limit to be exceeded");
    }

    #[test]
    fn test_request_processing() {
        let mut gateway = AiGateway::default_gateway();
        
        // Register client
        gateway.register_client(GatewayClient {
            client_id: "client_003".to_string(),
            api_key: "key_789".to_string(),
            name: "Test Client".to_string(),
            rate_limit_tier: RateLimitTier::Professional,
            permissions: vec![Permission::Predict, Permission::Optimize],
            request_count: 0,
            last_request: None,
            enabled: true,
        });
        
        // Register service
        gateway.register_service(ServiceEndpoint {
            id: "svc_002".to_string(),
            name: "ML Service".to_string(),
            url: "http://localhost:8081".to_string(),
            service_type: ServiceType::Prediction,
            auth: None,
            health: ServiceHealth::default(),
            load: ServiceLoad::default(),
            priority: 5,
            enabled: true,
        });
        
        let request = GatewayRequest {
            request_id: "req_001".to_string(),
            client_id: "client_003".to_string(),
            service_type: ServiceType::Prediction,
            payload: serde_json::json!({"input": "test"}),
            headers: HashMap::new(),
            priority: 5,
            timestamp: chrono::Utc::now(),
            timeout_ms: None,
        };
        
        let response = gateway.process_request(request).unwrap();
        
        assert_eq!(response.status, GatewayStatus::Success);
        assert!(response.data.is_some());
    }

    #[test]
    fn test_routing_rules() {
        let mut gateway = AiGateway::default_gateway();
        
        gateway.register_service(ServiceEndpoint {
            id: "svc_a".to_string(),
            name: "Service A".to_string(),
            url: "http://a".to_string(),
            service_type: ServiceType::Prediction,
            auth: None,
            health: ServiceHealth::default(),
            load: ServiceLoad::default(),
            priority: 5,
            enabled: true,
        });
        
        gateway.register_service(ServiceEndpoint {
            id: "svc_b".to_string(),
            name: "Service B".to_string(),
            url: "http://b".to_string(),
            service_type: ServiceType::Prediction,
            auth: None,
            health: ServiceHealth::default(),
            load: ServiceLoad::default(),
            priority: 3,
            enabled: true,
        });
        
        let rule = RoutingRule {
            id: "rule_001".to_string(),
            name: "High Priority Routing".to_string(),
            condition: RoutingCondition::PriorityRange(8, 10),
            targets: vec!["svc_a".to_string()],
            strategy: LoadBalancingStrategy::RoundRobin,
            priority: 10,
            enabled: true,
        };
        
        gateway.add_routing_rule(rule);
        assert_eq!(gateway.routing_rules.len(), 1);
        
        gateway.remove_routing_rule("rule_001");
        assert_eq!(gateway.routing_rules.len(), 0);
    }

    #[test]
    fn test_health_check() {
        let mut gateway = AiGateway::default_gateway();
        
        gateway.register_service(ServiceEndpoint {
            id: "svc_health".to_string(),
            name: "Health Test Service".to_string(),
            url: "http://health".to_string(),
            service_type: ServiceType::Prediction,
            auth: None,
            health: ServiceHealth::default(),
            load: ServiceLoad::default(),
            priority: 5,
            enabled: true,
        });
        
        let health = gateway.check_service_health("svc_health").unwrap();
        assert!(health.healthy);
    }

    #[test]
    fn test_caching() {
        let mut gateway = AiGateway::default_gateway();
        gateway.config.enable_caching = true;
        
        gateway.register_client(GatewayClient {
            client_id: "client_cache".to_string(),
            api_key: "key_cache".to_string(),
            name: "Cache Client".to_string(),
            rate_limit_tier: RateLimitTier::Basic,
            permissions: vec![Permission::Predict],
            request_count: 0,
            last_request: None,
            enabled: true,
        });
        
        gateway.register_service(ServiceEndpoint {
            id: "svc_cache".to_string(),
            name: "Cache Service".to_string(),
            url: "http://cache".to_string(),
            service_type: ServiceType::Prediction,
            auth: None,
            health: ServiceHealth::default(),
            load: ServiceLoad::default(),
            priority: 5,
            enabled: true,
        });
        
        let request = GatewayRequest {
            request_id: "req_cache".to_string(),
            client_id: "client_cache".to_string(),
            service_type: ServiceType::Prediction,
            payload: serde_json::json!({"test": "data"}),
            headers: HashMap::new(),
            priority: 5,
            timestamp: chrono::Utc::now(),
            timeout_ms: None,
        };
        
        // First request
        let response1 = gateway.process_request(request.clone()).unwrap();
        assert!(!response1.cache_hit);
        
        // Second request (should hit cache)
        let response2 = gateway.process_request(request).unwrap();
        assert!(response2.cache_hit);
    }
}
</content>