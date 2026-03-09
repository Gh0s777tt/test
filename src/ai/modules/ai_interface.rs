<content>
//! AI Interface Module
//! Provides a unified API for all AI features in VantisOS.
//! This is the main entry point for interacting with the AI subsystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// AI Interface Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiInterfaceConfig {
    /// Enable AI features globally
    pub enable_ai: bool,
    
    /// Enable predictive features
    pub enable_prediction: bool,
    
    /// Enable optimization features
    pub enable_optimization: bool,
    
    /// Enable anomaly detection
    pub enable_anomaly_detection: bool,
    
    /// Enable smart caching
    pub enable_smart_caching: bool,
    
    /// Maximum concurrent AI operations
    pub max_concurrent_ops: usize,
    
    /// AI operation timeout in milliseconds
    pub operation_timeout_ms: u64,
    
    /// Enable result caching
    pub cache_results: bool,
    
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    
    /// Logging level (0-3)
    pub log_level: u8,
}

impl Default for AiInterfaceConfig {
    fn default() -> Self {
        Self {
            enable_ai: true,
            enable_prediction: true,
            enable_optimization: true,
            enable_anomaly_detection: true,
            enable_smart_caching: true,
            max_concurrent_ops: 50,
            operation_timeout_ms: 30000,
            cache_results: true,
            cache_ttl_seconds: 300,
            log_level: 1,
        }
    }
}

/// AI Feature identifiers
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum AiFeature {
    /// Predictive prefetching
    PredictivePrefetch,
    
    /// Smart caching
    SmartCache,
    
    /// Resource optimization
    ResourceOptimization,
    
    /// Anomaly detection
    AnomalyDetection,
    
    /// Adaptive scheduling
    AdaptiveScheduling,
    
    /// Pattern recognition
    PatternRecognition,
    
    /// Natural language processing
    NaturalLanguage,
    
    /// Computer vision
    ComputerVision,
    
    /// Decision support
    DecisionSupport,
    
    /// Automated remediation
    AutoRemediation,
}

/// AI Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    /// Request ID
    pub request_id: String,
    
    /// Request type
    pub request_type: AiRequestType,
    
    /// Target feature
    pub feature: AiFeature,
    
    /// Input data
    pub input: AiInput,
    
    /// Request parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Priority (1-10)
    pub priority: u8,
    
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Timeout in milliseconds
    pub timeout_ms: u64,
    
    /// Callback URL (optional)
    pub callback_url: Option<String>,
}

/// AI Request Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiRequestType {
    /// Synchronous request
    Sync,
    
    /// Asynchronous request
    Async,
    
    /// Streaming request
    Streaming,
    
    /// Batch request
    Batch,
}

/// AI Input data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiInput {
    /// Text input
    Text(String),
    
    /// Binary data
    Binary(Vec<u8>),
    
    /// Structured data
    Structured(serde_json::Value),
    
    /// File path
    FilePath(String),
    
    /// Multiple inputs
    Multiple(Vec<AiInput>),
    
    /// No input required
    None,
}

/// AI Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    /// Request ID this response is for
    pub request_id: String,
    
    /// Response status
    pub status: AiResponseStatus,
    
    /// Result data
    pub result: Option<AiResult>,
    
    /// Error message if failed
    pub error: Option<String>,
    
    /// Processing time in milliseconds
    pub processing_time_ms: f64,
    
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// AI Response Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AiResponseStatus {
    Success,
    PartialSuccess,
    Failed,
    Timeout,
    Cancelled,
    Pending,
}

/// AI Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiResult {
    /// Prediction result
    Prediction(PredictionResult),
    
    /// Optimization result
    Optimization(OptimizationResult),
    
    /// Anomaly detection result
    Anomaly(AnomalyResult),
    
    /// Classification result
    Classification(ClassificationResult),
    
    /// Recommendation result
    Recommendation(RecommendationResult),
    
    /// Analysis result
    Analysis(AnalysisResult),
    
    /// Raw data result
    Raw(serde_json::Value),
}

/// Prediction Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    /// Predicted value
    pub value: f64,
    
    /// Confidence interval
    pub confidence_interval: (f64, f64),
    
    /// Prediction model used
    pub model: String,
    
    /// Feature importance
    pub feature_importance: HashMap<String, f64>,
    
    /// Additional predictions
    pub alternatives: Vec<PredictionAlternative>,
}

/// Prediction Alternative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionAlternative {
    pub value: f64,
    pub probability: f64,
    pub description: String,
}

/// Optimization Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// Original value
    pub original_value: f64,
    
    /// Optimized value
    pub optimized_value: f64,
    
    /// Improvement percentage
    pub improvement_percent: f64,
    
    /// Actions taken
    pub actions: Vec<OptimizationActionData>,
    
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Optimization Action Data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationActionData {
    pub action: String,
    pub parameter: String,
    pub old_value: f64,
    pub new_value: f64,
    pub impact: f64,
}

/// Anomaly Detection Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyResult {
    /// Whether anomaly detected
    pub anomaly_detected: bool,
    
    /// Anomaly score (0.0-1.0)
    pub anomaly_score: f64,
    
    /// Anomaly type
    pub anomaly_type: Option<String>,
    
    /// Severity (1-10)
    pub severity: u8,
    
    /// Affected components
    pub affected_components: Vec<String>,
    
    /// Root cause suggestions
    pub root_causes: Vec<String>,
    
    /// Remediation suggestions
    pub remediation: Vec<String>,
}

/// Classification Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    /// Predicted class
    pub predicted_class: String,
    
    /// Class probabilities
    pub probabilities: HashMap<String, f64>,
    
    /// Features used
    pub features: Vec<String>,
    
    /// Model confidence
    pub confidence: f64,
}

/// Recommendation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationResult {
    /// Recommendations
    pub recommendations: Vec<Recommendation>,
    
    /// Overall score
    pub overall_score: f64,
    
    /// Context used
    pub context: HashMap<String, String>,
}

/// Single Recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: u8,
    pub impact: f64,
    pub effort: f64,
    pub category: String,
}

/// Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Summary
    pub summary: String,
    
    /// Key findings
    pub findings: Vec<Finding>,
    
    /// Metrics
    pub metrics: HashMap<String, f64>,
    
    /// Trends
    pub trends: Vec<Trend>,
    
    /// Patterns detected
    pub patterns: Vec<String>,
}

/// Finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub description: String,
    pub significance: f64,
    pub category: String,
    pub evidence: Vec<String>,
}

/// Trend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub name: String,
    pub direction: TrendDirection,
    pub magnitude: f64,
    pub confidence: f64,
}

/// Trend Direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

/// AI Feature Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureStatus {
    /// Feature name
    pub feature: AiFeature,
    
    /// Whether feature is enabled
    pub enabled: bool,
    
    /// Feature version
    pub version: String,
    
    /// Feature health
    pub health: FeatureHealth,
    
    /// Usage statistics
    pub stats: FeatureStats,
    
    /// Last used
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
}

/// Feature Health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureHealth {
    Operational,
    Degraded,
    Unavailable,
    Maintenance,
}

/// Feature Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_latency_ms: f64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

/// AI Interface - Main entry point for AI features
pub struct AiInterface {
    config: AiInterfaceConfig,
    feature_registry: HashMap<AiFeature, FeatureStatus>,
    pending_requests: HashMap<String, AiRequest>,
    result_cache: HashMap<String, CachedResult>,
    operation_history: Vec<OperationRecord>,
}

/// Cached Result
#[derive(Debug, Clone)]
struct CachedResult {
    response: AiResponse,
    cached_at: chrono::DateTime<chrono::Utc>,
    ttl_seconds: u64,
}

/// Operation Record
#[derive(Debug, Clone)]
struct OperationRecord {
    request_id: String,
    feature: AiFeature,
    started_at: chrono::DateTime<chrono::Utc>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
    success: bool,
}

impl AiInterface {
    /// Create a new AI interface
    pub fn new(config: AiInterfaceConfig) -> Self {
        let mut feature_registry = HashMap::new();
        
        // Initialize all features
        for feature in [
            AiFeature::PredictivePrefetch,
            AiFeature::SmartCache,
            AiFeature::ResourceOptimization,
            AiFeature::AnomalyDetection,
            AiFeature::AdaptiveScheduling,
            AiFeature::PatternRecognition,
            AiFeature::NaturalLanguage,
            AiFeature::ComputerVision,
            AiFeature::DecisionSupport,
            AiFeature::AutoRemediation,
        ] {
            feature_registry.insert(feature.clone(), FeatureStatus {
                feature,
                enabled: true,
                version: "1.0.0".to_string(),
                health: FeatureHealth::Operational,
                stats: FeatureStats::default(),
                last_used: None,
            });
        }
        
        Self {
            config,
            feature_registry,
            pending_requests: HashMap::new(),
            result_cache: HashMap::new(),
            operation_history: Vec::new(),
        }
    }
    
    /// Create with default configuration
    pub fn default_interface() -> Self {
        Self::new(AiInterfaceConfig::default())
    }
    
    /// Process an AI request
    pub fn process_request(&mut self, request: AiRequest) -> Result<AiResponse, AiInterfaceError> {
        if !self.config.enable_ai {
            return Err(AiInterfaceError::AiDisabled);
        }
        
        // Check if feature is enabled
        let feature_status = self.feature_registry.get(&request.feature)
            .ok_or_else(|| AiInterfaceError::FeatureNotFound(request.feature.clone()))?;
        
        if !feature_status.enabled {
            return Err(AiInterfaceError::FeatureDisabled(request.feature.clone()));
        }
        
        let start_time = Instant::now();
        let request_id = request.request_id.clone();
        
        // Check cache
        if self.config.cache_results {
            let cache_key = self.generate_cache_key(&request);
            if let Some(cached) = self.check_cache(&cache_key) {
                return Ok(cached);
            }
        }
        
        // Process based on request type
        let result = match request.request_type {
            AiRequestType::Sync => self.process_sync(&request),
            AiRequestType::Async => self.process_async(&request),
            AiRequestType::Streaming => self.process_streaming(&request),
            AiRequestType::Batch => self.process_batch(&request),
        };
        
        let processing_time = start_time.elapsed().as_millis() as f64;
        
        // Update feature stats
        self.update_feature_stats(&request.feature, result.is_ok(), processing_time);
        
        // Record operation
        self.operation_history.push(OperationRecord {
            request_id: request_id.clone(),
            feature: request.feature.clone(),
            started_at: request.timestamp,
            completed_at: Some(chrono::Utc::now()),
            success: result.is_ok(),
        });
        
        result
    }
    
    /// Process synchronous request
    fn process_sync(&mut self, request: &AiRequest) -> Result<AiResponse, AiInterfaceError> {
        let result = self.execute_feature(&request.feature, &request.input, &request.parameters)?;
        
        Ok(AiResponse {
            request_id: request.request_id.clone(),
            status: AiResponseStatus::Success,
            result: Some(result),
            error: None,
            processing_time_ms: 0.0,
            confidence: 0.85,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Process asynchronous request
    fn process_async(&mut self, request: &AiRequest) -> Result<AiResponse, AiInterfaceError> {
        // Store as pending
        self.pending_requests.insert(request.request_id.clone(), request.clone());
        
        Ok(AiResponse {
            request_id: request.request_id.clone(),
            status: AiResponseStatus::Pending,
            result: None,
            error: None,
            processing_time_ms: 0.0,
            confidence: 0.0,
            metadata: [("status".to_string(), "queued".to_string())].into_iter().collect(),
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Process streaming request
    fn process_streaming(&mut self, request: &AiRequest) -> Result<AiResponse, AiInterfaceError> {
        // Initial response for streaming
        Ok(AiResponse {
            request_id: request.request_id.clone(),
            status: AiResponseStatus::Pending,
            result: None,
            error: None,
            processing_time_ms: 0.0,
            confidence: 0.0,
            metadata: [("streaming".to_string(), "true".to_string())].into_iter().collect(),
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Process batch request
    fn process_batch(&mut self, request: &AiRequest) -> Result<AiResponse, AiInterfaceError> {
        let result = self.execute_feature(&request.feature, &request.input, &request.parameters)?;
        
        Ok(AiResponse {
            request_id: request.request_id.clone(),
            status: AiResponseStatus::Success,
            result: Some(result),
            error: None,
            processing_time_ms: 0.0,
            confidence: 0.85,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
        })
    }
    
    /// Execute a feature
    fn execute_feature(
        &self,
        feature: &AiFeature,
        input: &AiInput,
        params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        match feature {
            AiFeature::PredictivePrefetch => self.execute_prediction(input, params),
            AiFeature::SmartCache => self.execute_caching(input, params),
            AiFeature::ResourceOptimization => self.execute_optimization(input, params),
            AiFeature::AnomalyDetection => self.execute_anomaly_detection(input, params),
            AiFeature::AdaptiveScheduling => self.execute_scheduling(input, params),
            AiFeature::PatternRecognition => self.execute_pattern_recognition(input, params),
            AiFeature::NaturalLanguage => self.execute_nlp(input, params),
            AiFeature::ComputerVision => self.execute_vision(input, params),
            AiFeature::DecisionSupport => self.execute_decision_support(input, params),
            AiFeature::AutoRemediation => self.execute_auto_remediation(input, params),
        }
    }
    
    /// Execute prediction feature
    fn execute_prediction(
        &self,
        input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        // Simulated prediction result
        Ok(AiResult::Prediction(PredictionResult {
            value: 0.85,
            confidence_interval: (0.75, 0.95),
            model: "ensemble_v2".to_string(),
            feature_importance: [("feature_1".to_string(), 0.4), ("feature_2".to_string(), 0.3)].into_iter().collect(),
            alternatives: vec![
                PredictionAlternative {
                    value: 0.70,
                    probability: 0.15,
                    description: "Conservative estimate".to_string(),
                },
            ],
        }))
    }
    
    /// Execute caching feature
    fn execute_caching(
        &self,
        _input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        Ok(AiResult::Raw(serde_json::json!({
            "cache_hit": true,
            "cache_size_mb": 256,
            "hit_rate": 0.87
        })))
    }
    
    /// Execute optimization feature
    fn execute_optimization(
        &self,
        _input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        Ok(AiResult::Optimization(OptimizationResult {
            original_value: 100.0,
            optimized_value: 75.0,
            improvement_percent: 25.0,
            actions: vec![
                OptimizationActionData {
                    action: "adjust".to_string(),
                    parameter: "buffer_size".to_string(),
                    old_value: 1024.0,
                    new_value: 2048.0,
                    impact: 0.3,
                },
            ],
            recommendations: vec!["Increase buffer pool".to_string()],
        }))
    }
    
    /// Execute anomaly detection feature
    fn execute_anomaly_detection(
        &self,
        _input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        Ok(AiResult::Anomaly(AnomalyResult {
            anomaly_detected: false,
            anomaly_score: 0.12,
            anomaly_type: None,
            severity: 0,
            affected_components: vec![],
            root_causes: vec![],
            remediation: vec![],
        }))
    }
    
    /// Execute scheduling feature
    fn execute_scheduling(
        &self,
        _input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        Ok(AiResult::Raw(serde_json::json!({
            "scheduled_tasks": 5,
            "optimized_slots": 3,
            "efficiency_gain": 0.15
        })))
    }
    
    /// Execute pattern recognition feature
    fn execute_pattern_recognition(
        &self,
        _input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        Ok(AiResult::Analysis(AnalysisResult {
            summary: "Pattern analysis completed".to_string(),
            findings: vec![
                Finding {
                    description: "Cyclic pattern detected".to_string(),
                    significance: 0.85,
                    category: "temporal".to_string(),
                    evidence: vec!["peak_hours".to_string()],
                },
            ],
            metrics: [("pattern_strength".to_string(), 0.78)].into_iter().collect(),
            trends: vec![
                Trend {
                    name: "load_pattern".to_string(),
                    direction: TrendDirection::Increasing,
                    magnitude: 0.15,
                    confidence: 0.92,
                },
            ],
            patterns: vec!["diurnal_cycle".to_string()],
        }))
    }
    
    /// Execute NLP feature
    fn execute_nlp(
        &self,
        input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        let text = match input {
            AiInput::Text(t) => t.clone(),
            _ => "No text input".to_string(),
        };
        
        Ok(AiResult::Classification(ClassificationResult {
            predicted_class: "query".to_string(),
            probabilities: [("query".to_string(), 0.8), ("command".to_string(), 0.2)].into_iter().collect(),
            features: vec!["tokens".to_string(), "sentiment".to_string()],
            confidence: 0.8,
        }))
    }
    
    /// Execute vision feature
    fn execute_vision(
        &self,
        _input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        Ok(AiResult::Raw(serde_json::json!({
            "objects_detected": 3,
            "confidence": 0.92,
            "labels": ["screen", "keyboard", "mouse"]
        })))
    }
    
    /// Execute decision support feature
    fn execute_decision_support(
        &self,
        _input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        Ok(AiResult::Recommendation(RecommendationResult {
            recommendations: vec![
                Recommendation {
                    id: "rec_001".to_string(),
                    title: "Optimize memory allocation".to_string(),
                    description: "Increase heap size for better performance".to_string(),
                    priority: 7,
                    impact: 0.25,
                    effort: 0.1,
                    category: "performance".to_string(),
                },
            ],
            overall_score: 0.82,
            context: HashMap::new(),
        }))
    }
    
    /// Execute auto remediation feature
    fn execute_auto_remediation(
        &self,
        _input: &AiInput,
        _params: &HashMap<String, serde_json::Value>,
    ) -> Result<AiResult, AiInterfaceError> {
        Ok(AiResult::Raw(serde_json::json!({
            "remediation_applied": true,
            "actions_taken": ["restart_service", "clear_cache"],
            "success": true
        })))
    }
    
    /// Generate cache key for request
    fn generate_cache_key(&self, request: &AiRequest) -> String {
        format!("{:?}_{}", request.feature, request.request_id)
    }
    
    /// Check cache for result
    fn check_cache(&mut self, cache_key: &str) -> Option<AiResponse> {
        if let Some(cached) = self.result_cache.get(cache_key) {
            let now = chrono::Utc::now();
            let age = now.signed_duration_since(cached.cached_at).num_seconds() as u64;
            if age < cached.ttl_seconds {
                return Some(cached.response.clone());
            } else {
                self.result_cache.remove(cache_key);
            }
        }
        None
    }
    
    /// Update feature statistics
    fn update_feature_stats(&mut self, feature: &AiFeature, success: bool, latency: f64) {
        if let Some(status) = self.feature_registry.get_mut(feature) {
            status.stats.total_requests += 1;
            if success {
                status.stats.successful_requests += 1;
            } else {
                status.stats.failed_requests += 1;
            }
            status.stats.avg_latency_ms = 
                (status.stats.avg_latency_ms * (status.stats.total_requests - 1) as f64 + latency) 
                / status.stats.total_requests as f64;
            status.last_used = Some(chrono::Utc::now());
        }
    }
    
    /// Enable a feature
    pub fn enable_feature(&mut self, feature: &AiFeature) -> Result<(), AiInterfaceError> {
        if let Some(status) = self.feature_registry.get_mut(feature) {
            status.enabled = true;
            Ok(())
        } else {
            Err(AiInterfaceError::FeatureNotFound(feature.clone()))
        }
    }
    
    /// Disable a feature
    pub fn disable_feature(&mut self, feature: &AiFeature) -> Result<(), AiInterfaceError> {
        if let Some(status) = self.feature_registry.get_mut(feature) {
            status.enabled = false;
            Ok(())
        } else {
            Err(AiInterfaceError::FeatureNotFound(feature.clone()))
        }
    }
    
    /// Get feature status
    pub fn get_feature_status(&self, feature: &AiFeature) -> Option<&FeatureStatus> {
        self.feature_registry.get(feature)
    }
    
    /// Get all features
    pub fn get_all_features(&self) -> Vec<&FeatureStatus> {
        self.feature_registry.values().collect()
    }
    
    /// Cancel a pending request
    pub fn cancel_request(&mut self, request_id: &str) -> Result<(), AiInterfaceError> {
        if self.pending_requests.remove(request_id).is_some() {
            Ok(())
        } else {
            Err(AiInterfaceError::RequestNotFound(request_id.to_string()))
        }
    }
    
    /// Get pending requests
    pub fn get_pending_requests(&self) -> Vec<&AiRequest> {
        self.pending_requests.values().collect()
    }
    
    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.result_cache.clear();
    }
    
    /// Get operation history
    pub fn get_operation_history(&self, count: usize) -> Vec<&OperationRecord> {
        self.operation_history.iter().rev().take(count).collect()
    }
}

impl Default for FeatureStats {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_latency_ms: 0.0,
            cache_hits: 0,
            cache_misses: 0,
        }
    }
}

/// AI Interface Error
#[derive(Debug, thiserror::Error)]
pub enum AiInterfaceError {
    #[error("AI features are disabled")]
    AiDisabled,
    
    #[error("Feature not found: {0:?}")]
    FeatureNotFound(AiFeature),
    
    #[error("Feature disabled: {0:?}")]
    FeatureDisabled(AiFeature),
    
    #[error("Request not found: {0}")]
    RequestNotFound(String),
    
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    #[error("Timeout exceeded")]
    Timeout,
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_interface_creation() {
        let config = AiInterfaceConfig::default();
        let interface = AiInterface::new(config);
        
        assert!(interface.config.enable_ai);
        assert_eq!(interface.feature_registry.len(), 10);
    }

    #[test]
    fn test_process_sync_request() {
        let mut interface = AiInterface::default_interface();
        
        let request = AiRequest {
            request_id: "req_001".to_string(),
            request_type: AiRequestType::Sync,
            feature: AiFeature::PredictivePrefetch,
            input: AiInput::None,
            parameters: HashMap::new(),
            priority: 5,
            timestamp: chrono::Utc::now(),
            timeout_ms: 5000,
            callback_url: None,
        };
        
        let response = interface.process_request(request).unwrap();
        
        assert_eq!(response.status, AiResponseStatus::Success);
        assert!(response.result.is_some());
    }

    #[test]
    fn test_async_request() {
        let mut interface = AiInterface::default_interface();
        
        let request = AiRequest {
            request_id: "req_002".to_string(),
            request_type: AiRequestType::Async,
            feature: AiFeature::ResourceOptimization,
            input: AiInput::None,
            parameters: HashMap::new(),
            priority: 5,
            timestamp: chrono::Utc::now(),
            timeout_ms: 5000,
            callback_url: None,
        };
        
        let response = interface.process_request(request).unwrap();
        
        assert_eq!(response.status, AiResponseStatus::Pending);
    }

    #[test]
    fn test_feature_enable_disable() {
        let mut interface = AiInterface::default_interface();
        
        let feature = AiFeature::AnomalyDetection;
        
        interface.disable_feature(&feature).unwrap();
        let status = interface.get_feature_status(&feature).unwrap();
        assert!(!status.enabled);
        
        interface.enable_feature(&feature).unwrap();
        let status = interface.get_feature_status(&feature).unwrap();
        assert!(status.enabled);
    }

    #[test]
    fn test_disabled_feature_error() {
        let mut interface = AiInterface::default_interface();
        
        let feature = AiFeature::ComputerVision;
        interface.disable_feature(&feature).unwrap();
        
        let request = AiRequest {
            request_id: "req_003".to_string(),
            request_type: AiRequestType::Sync,
            feature: feature.clone(),
            input: AiInput::None,
            parameters: HashMap::new(),
            priority: 5,
            timestamp: chrono::Utc::now(),
            timeout_ms: 5000,
            callback_url: None,
        };
        
        let result = interface.process_request(request);
        assert!(result.is_err());
    }

    #[test]
    fn test_feature_stats_update() {
        let mut interface = AiInterface::default_interface();
        
        let request = AiRequest {
            request_id: "req_004".to_string(),
            request_type: AiRequestType::Sync,
            feature: AiFeature::SmartCache,
            input: AiInput::None,
            parameters: HashMap::new(),
            priority: 5,
            timestamp: chrono::Utc::now(),
            timeout_ms: 5000,
            callback_url: None,
        };
        
        interface.process_request(request).unwrap();
        
        let status = interface.get_feature_status(&AiFeature::SmartCache).unwrap();
        assert_eq!(status.stats.total_requests, 1);
        assert_eq!(status.stats.successful_requests, 1);
    }

    #[test]
    fn test_operation_history() {
        let mut interface = AiInterface::default_interface();
        
        for i in 0..5 {
            let request = AiRequest {
                request_id: format!("req_{}", i),
                request_type: AiRequestType::Sync,
                feature: AiFeature::PatternRecognition,
                input: AiInput::None,
                parameters: HashMap::new(),
                priority: 5,
                timestamp: chrono::Utc::now(),
                timeout_ms: 5000,
                callback_url: None,
            };
            
            interface.process_request(request).unwrap();
        }
        
        let history = interface.get_operation_history(3);
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_cancel_request() {
        let mut interface = AiInterface::default_interface();
        
        let request = AiRequest {
            request_id: "req_cancel".to_string(),
            request_type: AiRequestType::Async,
            feature: AiFeature::AdaptiveScheduling,
            input: AiInput::None,
            parameters: HashMap::new(),
            priority: 5,
            timestamp: chrono::Utc::now(),
            timeout_ms: 5000,
            callback_url: None,
        };
        
        interface.process_request(request).unwrap();
        
        let result = interface.cancel_request("req_cancel");
        assert!(result.is_ok());
        
        let pending = interface.get_pending_requests();
        assert!(pending.is_empty());
    }
}
</content>