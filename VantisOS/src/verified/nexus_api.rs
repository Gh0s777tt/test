// VantisOS Nexus API - REST and gRPC API Layer
// Copyright (c) 2025 VantisOS Foundation
// SPDX-License-Identifier: MIT

//! # Nexus API
//!
//! REST and gRPC API layer for Nexus Server. Provides HTTP/HTTPS endpoints
//! and gRPC services for node management, compliance monitoring, and analytics.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{NexusError, NexusConfig};
use super::engine::{NexusEngine, NodeInfo, NodeStatus, HealthCheck};
use super::compliance::{ComplianceEngine, ComplianceStatus};
use super::auth::{AuthManager, User, Role, Permission};
use super::analytics::{AnalyticsEngine, Metric, Alert};
use super::updates::{UpdateManager, UpdatePackage};

/// HTTP method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
        }
    }
}

/// API request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    /// Request ID
    pub request_id: Uuid,
    
    /// HTTP method
    pub method: HttpMethod,
    
    /// Request path
    pub path: String,
    
    /// Query parameters
    pub query_params: HashMap<String, String>,
    
    /// Request headers
    pub headers: HashMap<String, String>,
    
    /// Request body
    pub body: Option<String>,
    
    /// Authentication token
    pub auth_token: Option<String>,
    
    /// Request timestamp
    pub timestamp: u64,
}

impl ApiRequest {
    /// Create a new API request
    pub fn new(method: HttpMethod, path: String) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            method,
            path,
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body: None,
            auth_token: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Add query parameter
    pub fn with_query(mut self, key: String, value: String) -> Self {
        self.query_params.insert(key, value);
        self
    }
    
    /// Add header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    /// Set body
    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
    
    /// Set auth token
    pub fn with_auth(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }
}

/// API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    /// Request ID
    pub request_id: Uuid,
    
    /// HTTP status code
    pub status_code: u16,
    
    /// Response body
    pub body: Option<String>,
    
    /// Response headers
    pub headers: HashMap<String, String>,
    
    /// Response timestamp
    pub timestamp: u64,
    
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

impl ApiResponse {
    /// Create a new API response
    pub fn new(request_id: Uuid, status_code: u16) -> Self {
        Self {
            request_id,
            status_code,
            body: None,
            headers: HashMap::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            processing_time_ms: 0,
        }
    }
    
    /// Set body
    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
    
    /// Add header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    /// Set processing time
    pub fn with_processing_time(mut self, time_ms: u64) -> Self {
        self.processing_time_ms = time_ms;
        self
    }
    
    /// Create success response
    pub fn success(request_id: Uuid, body: String) -> Self {
        Self::new(request_id, 200)
            .with_body(body)
    }
    
    /// Create error response
    pub fn error(request_id: Uuid, status_code: u16, error: String) -> Self {
        Self::new(request_id, status_code)
            .with_body(error)
    }
    
    /// Create not found response
    pub fn not_found(request_id: Uuid) -> Self {
        Self::error(request_id, 404, "Not found".to_string())
    }
    
    /// Create unauthorized response
    pub fn unauthorized(request_id: Uuid) -> Self {
        Self::error(request_id, 401, "Unauthorized".to_string())
    }
    
    /// Create forbidden response
    pub fn forbidden(request_id: Uuid) -> Self {
        Self::error(request_id, 403, "Forbidden".to_string())
    }
    
    /// Create bad request response
    pub fn bad_request(request_id: Uuid, error: String) -> Self {
        Self::error(request_id, 400, error)
    }
    
    /// Create internal error response
    pub fn internal_error(request_id: Uuid, error: String) -> Self {
        Self::error(request_id, 500, error)
    }
}

/// API endpoint handler
pub type ApiHandler = Arc<dyn Fn(ApiRequest) -> Result<ApiResponse, NexusError> + Send + Sync>;

/// API route
#[derive(Debug, Clone)]
pub struct ApiRoute {
    /// HTTP method
    pub method: HttpMethod,
    
    /// Route path
    pub path: String,
    
    /// Handler function
    pub handler: ApiHandler,
    
    /// Required permissions
    pub required_permissions: Vec<Permission>,
    
    /// Rate limit (requests per minute)
    pub rate_limit: Option<usize>,
}

/// Nexus API layer
pub struct NexusApi {
    /// Server configuration
    config: NexusConfig,
    
    /// Core engine
    engine: Arc<NexusEngine>,
    
    /// Compliance engine
    compliance: Arc<ComplianceEngine>,
    
    /// Authentication manager
    auth: Arc<AuthManager>,
    
    /// Analytics engine
    analytics: Arc<AnalyticsEngine>,
    
    /// Update manager
    updates: Arc<UpdateManager>,
    
    /// Registered routes
    routes: Arc<RwLock<Vec<ApiRoute>>>,
    
    /// Running state
    running: Arc<RwLock<bool>>,
}

use std::sync::RwLock;

impl NexusApi {
    /// Create a new Nexus API instance
    pub fn new(
        config: NexusConfig,
        engine: Arc<NexusEngine>,
        compliance: Arc<ComplianceEngine>,
        auth: Arc<AuthManager>,
        analytics: Arc<AnalyticsEngine>,
        updates: Arc<UpdateManager>,
    ) -> Result<Self, NexusError> {
        let api = Self {
            config,
            engine,
            compliance,
            auth,
            analytics,
            updates,
            routes: Arc::new(RwLock::new(Vec::new())),
            running: Arc::new(RwLock::new(false)),
        };
        
        // Register default routes
        api.register_default_routes();
        
        Ok(api)
    }
    
    /// Register default API routes
    fn register_default_routes(&self) {
        let mut routes = self.routes.write().unwrap();
        
        // Health check endpoint
        routes.push(ApiRoute {
            method: HttpMethod::GET,
            path: "/health".to_string(),
            handler: Arc::new(|req| {
                Ok(ApiResponse::success(req.request_id, r#"{"status":"healthy"}"#.to_string()))
            }),
            required_permissions: vec![],
            rate_limit: None,
        });
        
        // Server info endpoint
        routes.push(ApiRoute {
            method: HttpMethod::GET,
            path: "/api/v1/server/info".to_string(),
            handler: Arc::new(|req| {
                let info = r#"{
                    "name": "VantisOS Nexus Server",
                    "version": "1.0.0",
                    "status": "running"
                }"#;
                Ok(ApiResponse::success(req.request_id, info.to_string()))
            }),
            required_permissions: vec![],
            rate_limit: None,
        });
        
        // Node list endpoint
        routes.push(ApiRoute {
            method: HttpMethod::GET,
            path: "/api/v1/nodes".to_string(),
            handler: Arc::new(|req| {
                // TODO: Implement node listing
                Ok(ApiResponse::success(req.request_id, r#"{"nodes":[]}"#.to_string()))
            }),
            required_permissions: vec![Permission::ReadNodes],
            rate_limit: Some(100),
        });
        
        // Compliance status endpoint
        routes.push(ApiRoute {
            method: HttpMethod::GET,
            path: "/api/v1/compliance/status".to_string(),
            handler: Arc::new(|req| {
                // TODO: Implement compliance status
                Ok(ApiResponse::success(req.request_id, r#"{"score":95.5}"#.to_string()))
            }),
            required_permissions: vec![Permission::ReadCompliance],
            rate_limit: Some(50),
        });
        
        // Metrics endpoint
        routes.push(ApiRoute {
            method: HttpMethod::GET,
            path: "/api/v1/metrics".to_string(),
            handler: Arc::new(|req| {
                // TODO: Implement metrics
                Ok(ApiResponse::success(req.request_id, r#"{"metrics":[]}"#.to_string()))
            }),
            required_permissions: vec![Permission::ReadMetrics],
            rate_limit: Some(200),
        });
    }
    
    /// Register a custom route
    pub fn register_route(&self, route: ApiRoute) {
        let mut routes = self.routes.write().unwrap();
        routes.push(route);
    }
    
    /// Handle an API request
    pub async fn handle_request(&self, request: ApiRequest) -> ApiResponse {
        let start_time = Instant::now();
        
        // Find matching route
        let routes = self.routes.read().unwrap();
        let route = routes.iter()
            .find(|r| r.method == request.method && self.match_path(&r.path, &request.path));
        
        let route = match route {
            Some(r) => r,
            None => {
                return ApiResponse::not_found(request.request_id);
            }
        };
        
        // Check authentication
        if let Some(ref token) = request.auth_token {
            match self.auth.validate_token(token) {
                Ok(user) => {
                    // Check permissions
                    for permission in &route.required_permissions {
                        if !user.has_permission(permission) {
                            return ApiResponse::forbidden(request.request_id);
                        }
                    }
                }
                Err(_) => {
                    return ApiResponse::unauthorized(request.request_id);
                }
            }
        } else if !route.required_permissions.is_empty() {
            return ApiResponse::unauthorized(request.request_id);
        }
        
        // Execute handler
        let response = match (route.handler)(request) {
            Ok(r) => r,
            Err(e) => ApiResponse::internal_error(request.request_id, e.to_string()),
        };
        
        // Add processing time
        let processing_time = start_time.elapsed().as_millis() as u64;
        response.with_processing_time(processing_time)
    }
    
    /// Match route path with request path
    fn match_path(&self, route_path: &str, request_path: &str) -> bool {
        // Simple exact match for now
        // TODO: Implement path parameter matching
        route_path == request_path
    }
    
    /// Start the API server
    pub async fn start(&self) -> Result<(), NexusError> {
        let mut running = self.running.write()
            .map_err(|_| NexusError::LockError)?;
        
        if *running {
            return Err(NexusError::AlreadyRunning);
        }
        
        *running = true;
        drop(running);
        
        // TODO: Start HTTP server
        // TODO: Start gRPC server
        
        log::info!("Nexus API started successfully");
        
        Ok(())
    }
    
    /// Stop the API server
    pub async fn stop(&self) -> Result<(), NexusError> {
        let mut running = self.running.write()
            .map_err(|_| NexusError::LockError)?;
        
        if !*running {
            return Err(NexusError::NotRunning);
        }
        
        *running = false;
        drop(running);
        
        // TODO: Stop HTTP server
        // TODO: Stop gRPC server
        
        log::info!("Nexus API stopped successfully");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api_request_creation() {
        let request = ApiRequest::new(HttpMethod::GET, "/test".to_string())
            .with_query("key".to_string(), "value".to_string())
            .with_header("Content-Type".to_string(), "application/json".to_string());
        
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path, "/test");
        assert_eq!(request.query_params.get("key"), Some(&"value".to_string()));
    }
    
    #[test]
    fn test_api_response_creation() {
        let request_id = Uuid::new_v4();
        let response = ApiResponse::success(request_id, r#"{"status":"ok"}"#.to_string());
        
        assert_eq!(response.status_code, 200);
        assert_eq!(response.body, Some(r#"{"status":"ok"}"#.to_string()));
    }
    
    #[test]
    fn test_error_responses() {
        let request_id = Uuid::new_v4();
        
        let not_found = ApiResponse::not_found(request_id);
        assert_eq!(not_found.status_code, 404);
        
        let unauthorized = ApiResponse::unauthorized(request_id);
        assert_eq!(unauthorized.status_code, 401);
        
        let forbidden = ApiResponse::forbidden(request_id);
        assert_eq!(forbidden.status_code, 403);
    }
}