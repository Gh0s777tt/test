// VantisOS Nexus Server - Enterprise-grade Central Management Platform
// Copyright (c) 2025 VantisOS Foundation
// SPDX-License-Identifier: MIT

//! # Nexus Server
//!
//! Enterprise-grade centralized management, monitoring, and compliance platform
//! for VantisOS deployments. Provides REST/gRPC APIs, node management, compliance
//! monitoring, and real-time analytics.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Re-export modules
pub mod api;
pub mod engine;
pub mod compliance;
pub mod storage;
pub mod auth;
pub mod analytics;
pub mod updates;

use api::{NexusApi, ApiRequest, ApiResponse, HttpMethod};
use engine::{NexusEngine, NodeInfo, NodeStatus, HealthCheck};
use compliance::{ComplianceEngine, ComplianceFramework, ComplianceStatus};
use storage::{NexusStorage, StorageBackend, DatabaseConfig};
use auth::{AuthManager, OAuth2Provider, Role, Permission, User};
use analytics::{AnalyticsEngine, Metric, MetricType, Alert};
use updates::{UpdateManager, UpdatePackage, UpdateStatus};

/// Nexus Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusConfig {
    /// Server bind address
    pub bind_address: String,
    
    /// REST API port
    pub rest_port: u16,
    
    /// gRPC API port
    pub grpc_port: u16,
    
    /// Maximum concurrent connections
    pub max_connections: usize,
    
    /// Request timeout in seconds
    pub request_timeout: u64,
    
    /// Enable TLS
    pub tls_enabled: bool,
    
    /// TLS certificate path
    pub tls_cert_path: Option<String>,
    
    /// TLS key path
    pub tls_key_path: Option<String>,
    
    /// Database configuration
    pub database: DatabaseConfig,
    
    /// OAuth 2.0 providers
    pub oauth_providers: Vec<OAuth2Provider>,
    
    /// Enable analytics
    pub analytics_enabled: bool,
    
    /// Metrics retention period in days
    pub metrics_retention_days: u32,
    
    /// Update check interval in hours
    pub update_check_interval_hours: u32,
}

impl Default for NexusConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0".to_string(),
            rest_port: 8080,
            grpc_port: 9090,
            max_connections: 10000,
            request_timeout: 30,
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            database: DatabaseConfig::default(),
            oauth_providers: Vec::new(),
            analytics_enabled: true,
            metrics_retention_days: 90,
            update_check_interval_hours: 24,
        }
    }
}

/// Nexus Server main instance
pub struct NexusServer {
    /// Server configuration
    config: NexusConfig,
    
    /// Core engine
    engine: Arc<NexusEngine>,
    
    /// API layer
    api: Arc<NexusApi>,
    
    /// Compliance engine
    compliance: Arc<ComplianceEngine>,
    
    /// Storage backend
    storage: Arc<NexusStorage>,
    
    /// Authentication manager
    auth: Arc<AuthManager>,
    
    /// Analytics engine
    analytics: Arc<AnalyticsEngine>,
    
    /// Update manager
    updates: Arc<UpdateManager>,
    
    /// Server start time
    start_time: Instant,
    
    /// Running state
    running: Arc<RwLock<bool>>,
}

impl NexusServer {
    /// Create a new Nexus Server instance
    pub fn new(config: NexusConfig) -> Result<Self, NexusError> {
        let start_time = Instant::now();
        
        // Initialize storage
        let storage = Arc::new(NexusStorage::new(config.database.clone())?);
        
        // Initialize authentication
        let auth = Arc::new(AuthManager::new(
            storage.clone(),
            config.oauth_providers.clone(),
        )?);
        
        // Initialize core engine
        let engine = Arc::new(NexusEngine::new(
            storage.clone(),
            auth.clone(),
        )?);
        
        // Initialize compliance engine
        let compliance = Arc::new(ComplianceEngine::new(
            storage.clone(),
            engine.clone(),
        )?);
        
        // Initialize analytics engine
        let analytics = Arc::new(AnalyticsEngine::new(
            storage.clone(),
            config.metrics_retention_days,
        )?);
        
        // Initialize update manager
        let updates = Arc::new(UpdateManager::new(
            storage.clone(),
            auth.clone(),
            config.update_check_interval_hours,
        )?);
        
        // Initialize API layer
        let api = Arc::new(NexusApi::new(
            config.clone(),
            engine.clone(),
            compliance.clone(),
            auth.clone(),
            analytics.clone(),
            updates.clone(),
        )?);
        
        Ok(Self {
            config,
            engine,
            api,
            compliance,
            storage,
            auth,
            analytics,
            updates,
            start_time,
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Start the Nexus Server
    pub async fn start(&self) -> Result<(), NexusError> {
        let mut running = self.running.write()
            .map_err(|_| NexusError::LockError)?;
        
        if *running {
            return Err(NexusError::AlreadyRunning);
        }
        
        *running = true;
        drop(running);
        
        // Start storage
        self.storage.start().await?;
        
        // Start compliance monitoring
        self.compliance.start().await?;
        
        // Start analytics collection
        if self.config.analytics_enabled {
            self.analytics.start().await?;
        }
        
        // Start update manager
        self.updates.start().await?;
        
        // Start API servers
        self.api.start().await?;
        
        log::info!("Nexus Server started successfully");
        log::info!("REST API: http://{}:{}", self.config.bind_address, self.config.rest_port);
        log::info!("gRPC API: {}://{}:{}", 
            if self.config.tls_enabled { "grpcs" } else { "grpc" },
            self.config.bind_address, 
            self.config.grpc_port
        );
        
        Ok(())
    }
    
    /// Stop the Nexus Server
    pub async fn stop(&self) -> Result<(), NexusError> {
        let mut running = self.running.write()
            .map_err(|_| NexusError::LockError)?;
        
        if !*running {
            return Err(NexusError::NotRunning);
        }
        
        *running = false;
        drop(running);
        
        // Stop API servers
        self.api.stop().await?;
        
        // Stop update manager
        self.updates.stop().await?;
        
        // Stop analytics collection
        if self.config.analytics_enabled {
            self.analytics.stop().await?;
        }
        
        // Stop compliance monitoring
        self.compliance.stop().await?;
        
        // Stop storage
        self.storage.stop().await?;
        
        log::info!("Nexus Server stopped successfully");
        
        Ok(())
    }
    
    /// Check if server is running
    pub fn is_running(&self) -> bool {
        *self.running.read().unwrap_or_else(|_| Arc::new(RwLock::new(false)))
    }
    
    /// Get server uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Get server statistics
    pub fn get_stats(&self) -> ServerStats {
        let node_count = self.engine.get_node_count();
        let active_nodes = self.engine.get_active_node_count();
        let compliance_score = self.compliance.get_overall_compliance_score();
        let metrics_count = self.analytics.get_metrics_count();
        
        ServerStats {
            uptime: self.uptime(),
            node_count,
            active_nodes,
            compliance_score,
            metrics_count,
            start_time: self.start_time,
        }
    }
    
    /// Get engine reference
    pub fn engine(&self) -> &Arc<NexusEngine> {
        &self.engine
    }
    
    /// Get API reference
    pub fn api(&self) -> &Arc<NexusApi> {
        &self.api
    }
    
    /// Get compliance engine reference
    pub fn compliance(&self) -> &Arc<ComplianceEngine> {
        &self.compliance
    }
    
    /// Get storage reference
    pub fn storage(&self) -> &Arc<NexusStorage> {
        &self.storage
    }
    
    /// Get auth manager reference
    pub fn auth(&self) -> &Arc<AuthManager> {
        &self.auth
    }
    
    /// Get analytics engine reference
    pub fn analytics(&self) -> &Arc<AnalyticsEngine> {
        &self.analytics
    }
    
    /// Get update manager reference
    pub fn updates(&self) -> &Arc<UpdateManager> {
        &self.updates
    }
}

/// Server statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStats {
    /// Server uptime
    pub uptime: Duration,
    
    /// Total number of nodes
    pub node_count: usize,
    
    /// Number of active nodes
    pub active_nodes: usize,
    
    /// Overall compliance score (0-100)
    pub compliance_score: f64,
    
    /// Total metrics collected
    pub metrics_count: usize,
    
    /// Server start time
    pub start_time: Instant,
}

/// Nexus Server errors
#[derive(Debug, thiserror::Error)]
pub enum NexusError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Compliance error: {0}")]
    ComplianceError(String),
    
    #[error("Update error: {0}")]
    UpdateError(String),
    
    #[error("Server already running")]
    AlreadyRunning,
    
    #[error("Server not running")]
    NotRunning,
    
    #[error("Lock error")]
    LockError,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = NexusConfig::default();
        assert_eq!(config.rest_port, 8080);
        assert_eq!(config.grpc_port, 9090);
        assert_eq!(config.max_connections, 10000);
    }
    
    #[test]
    fn test_server_stats() {
        let start_time = Instant::now();
        let stats = ServerStats {
            uptime: Duration::from_secs(60),
            node_count: 10,
            active_nodes: 8,
            compliance_score: 95.5,
            metrics_count: 1000,
            start_time,
        };
        
        assert_eq!(stats.node_count, 10);
        assert_eq!(stats.active_nodes, 8);
        assert_eq!(stats.compliance_score, 95.5);
    }
}