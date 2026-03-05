//! Secure Inference Module
//! 
//! Secure inference pipeline for AI models.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};

/// Secure inference configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureInferenceConfig {
    /// Enable input validation
    pub enable_input_validation: bool,
    /// Enable output sanitization
    pub enable_output_sanitization: bool,
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Enable rate limiting
    pub enable_rate_limiting: bool,
    /// Maximum input size in bytes
    pub max_input_size_bytes: u64,
    /// Maximum inference time in milliseconds
    pub max_inference_time_ms: u64,
}

impl Default for SecureInferenceConfig {
    fn default() -> Self {
        Self {
            enable_input_validation: true,
            enable_output_sanitization: true,
            enable_audit_logging: true,
            enable_rate_limiting: true,
            max_input_size_bytes: 10 * 1024 * 1024, // 10 MB
            max_inference_time_ms: 5000, // 5 seconds
        }
    }
}

/// Input validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputValidationResult {
    /// Is valid
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<String>,
    /// Sanitized input
    pub sanitized_input: Option<Vec<u8>>,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// Entry ID
    pub entry_id: String,
    /// Timestamp
    pub timestamp: u64,
    /// User ID
    pub user_id: String,
    /// Model ID
    pub model_id: String,
    /// Request hash
    pub request_hash: String,
    /// Response hash
    pub response_hash: String,
    /// Inference time in milliseconds
    pub inference_time_ms: u64,
    /// Status
    pub status: String,
}

/// Rate limit status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStatus {
    /// Is limited
    pub is_limited: bool,
    /// Remaining requests
    pub remaining_requests: u64,
    /// Reset time
    pub reset_time: u64,
    /// Current window requests
    pub current_window_requests: u64,
}

/// Inference session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceSession {
    /// Session ID
    pub session_id: String,
    /// User ID
    pub user_id: String,
    /// Model ID
    pub model_id: String,
    /// Created timestamp
    pub created_at: u64,
    /// Request count
    pub request_count: u64,
}

/// Secure inference statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureInferenceStats {
    /// Total inference requests
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Rejected requests
    pub rejected_requests: u64,
    /// Rate limited requests
    pub rate_limited_requests: u64,
    /// Average inference time in milliseconds
    pub avg_inference_time_ms: f64,
    /// Audit log entries count
    pub audit_entries_count: u64,
}

/// Secure inference pipeline
pub struct SecureInferencePipeline {
    config: SecureInferenceConfig,
    rate_limits: Arc<RwLock<HashMap<String, RateLimitStatus>>>,
    audit_log: Arc<RwLock<Vec<AuditLogEntry>>>,
    sessions: Arc<RwLock<HashMap<String, InferenceSession>>>,
    statistics: Arc<RwLock<SecureInferenceStats>>,
}

impl SecureInferencePipeline {
    /// Create a new secure inference pipeline
    pub fn new(config: SecureInferenceConfig) -> Self {
        Self {
            config,
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            audit_log: Arc::new(RwLock::new(Vec::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(SecureInferenceStats {
                total_requests: 0,
                successful_requests: 0,
                rejected_requests: 0,
                rate_limited_requests: 0,
                avg_inference_time_ms: 0.0,
                audit_entries_count: 0,
            })),
        }
    }

    /// Create inference session
    pub async fn create_session(&self, user_id: &str, model_id: &str) -> Result<InferenceSession, Box<dyn std::error::Error>> {
        let session = InferenceSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            model_id: model_id.to_string(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            request_count: 0,
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session.session_id.clone(), session.clone());

        Ok(session)
    }

    /// Validate input
    pub async fn validate_input(&self, input: &[u8]) -> InputValidationResult {
        let mut errors = Vec::new();
        let mut sanitized_input = input.to_vec();

        // Check size
        if input.len() as u64 > self.config.max_input_size_bytes {
            errors.push(format!(
                "Input size {} exceeds maximum {}",
                input.len(),
                self.config.max_input_size_bytes
            ));
        }

        // Check for null bytes
        if input.contains(&0) {
            errors.push("Input contains null bytes".to_string());
            sanitized_input.retain(|&b| b != 0);
        }

        // Check for control characters
        if input.iter().any(|&b| b < 32 && b != 10 && b != 13) {
            errors.push("Input contains control characters".to_string());
            sanitized_input.retain(|&b| b >= 32 || b == 10 || b == 13);
        }

        InputValidationResult {
            is_valid: errors.is_empty(),
            errors,
            sanitized_input: Some(sanitized_input),
        }
    }

    /// Sanitize output
    pub async fn sanitize_output(&self, output: &[u8]) -> Vec<u8> {
        let mut sanitized = output.to_vec();
        
        // Remove potentially sensitive information
        sanitized.retain(|&b| b != 0);
        
        sanitized
    }

    /// Check rate limit
    pub async fn check_rate_limit(&self, user_id: &str, max_requests: u64) -> RateLimitStatus {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut rate_limits = self.rate_limits.write().await;
        let entry = rate_limits.entry(user_id.to_string()).or_insert(RateLimitStatus {
            is_limited: false,
            remaining_requests: max_requests,
            reset_time: current_time + 60,
            current_window_requests: 0,
        });

        // Reset if window passed
        if current_time >= entry.reset_time {
            entry.remaining_requests = max_requests;
            entry.reset_time = current_time + 60;
            entry.current_window_requests = 0;
        }

        entry.current_window_requests += 1;
        entry.remaining_requests = entry.remaining_requests.saturating_sub(1);
        entry.is_limited = entry.remaining_requests == 0;

        let result = entry.clone();

        // Update stats if rate limited
        if result.is_limited {
            let mut stats = self.statistics.write().await;
            stats.rate_limited_requests += 1;
        }

        result
    }

    /// Log inference to audit log
    pub async fn log_inference(
        &self,
        user_id: &str,
        model_id: &str,
        request_hash: &str,
        response_hash: &str,
        inference_time_ms: u64,
        status: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let entry = AuditLogEntry {
            entry_id: uuid::Uuid::new_v4().to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
            user_id: user_id.to_string(),
            model_id: model_id.to_string(),
            request_hash: request_hash.to_string(),
            response_hash: response_hash.to_string(),
            inference_time_ms,
            status: status.to_string(),
        };

        let mut audit_log = self.audit_log.write().await;
        audit_log.push(entry);

        let mut stats = self.statistics.write().await;
        stats.audit_entries_count += 1;

        Ok(())
    }

    /// Run secure inference
    pub async fn run_inference(
        &self,
        session_id: &str,
        input: &[u8],
        inference_fn: impl Fn(&[u8]) -> Vec<u8>
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        let mut stats = self.statistics.write().await;
        stats.total_requests += 1;
        drop(stats);

        // Get session
        let sessions = self.sessions.read().await;
        let session = sessions.get(session_id).ok_or("Session not found")?.clone();
        drop(sessions);

        // Check rate limit
        if self.config.enable_rate_limiting {
            let rate_limit = self.check_rate_limit(&session.user_id, 100).await;
            if rate_limit.is_limited {
                let mut stats = self.statistics.write().await;
                stats.rejected_requests += 1;
                return Err("Rate limit exceeded".into());
            }
        }

        // Validate input
        if self.config.enable_input_validation {
            let validation = self.validate_input(input).await;
            if !validation.is_valid {
                let mut stats = self.statistics.write().await;
                stats.rejected_requests += 1;
                return Err(format!("Input validation failed: {:?}", validation.errors).into());
            }
        }

        // Run inference
        let output = inference_fn(input);
        let elapsed = start.elapsed().as_millis() as u64;

        // Check inference time
        if elapsed > self.config.max_inference_time_ms {
            let mut stats = self.statistics.write().await;
            stats.rejected_requests += 1;
            return Err("Inference timeout".into());
        }

        // Sanitize output
        let sanitized_output = if self.config.enable_output_sanitization {
            self.sanitize_output(&output).await
        } else {
            output
        };

        // Log to audit
        if self.config.enable_audit_logging {
            self.log_inference(
                &session.user_id,
                &session.model_id,
                &self.hash_data(input),
                &self.hash_data(&sanitized_output),
                elapsed,
                "success"
            ).await?;
        }

        // Update stats
        let mut stats = self.statistics.write().await;
        stats.successful_requests += 1;
        stats.avg_inference_time_ms = (stats.avg_inference_time_ms * (stats.successful_requests - 1) as f64 + elapsed as f64) / stats.successful_requests as f64;

        Ok(sanitized_output)
    }

    /// Hash data
    fn hash_data(&self, data: &[u8]) -> String {
        // Simplified hash - in production use proper SHA-256
        let mut hash = 0u64;
        for byte in data {
            hash = hash.wrapping_mul(31).wrapping_add(*byte as u64);
        }
        format!("{:x}", hash)
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> Result<SecureInferenceStats, Box<dyn std::error::Error>> {
        let stats = self.statistics.read().await;
        Ok(stats.clone())
    }

    /// Get audit log
    pub async fn get_audit_log(&self) -> Result<Vec<AuditLogEntry>, Box<dyn std::error::Error>> {
        let audit_log = self.audit_log.read().await;
        Ok(audit_log.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_input_validation() {
        let pipeline = SecureInferencePipeline::new(SecureInferenceConfig::default());
        
        let valid_input = vec![1, 2, 3, 4, 5];
        let result = pipeline.validate_input(&valid_input).await;
        assert!(result.is_valid);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let pipeline = SecureInferencePipeline::new(SecureInferenceConfig::default());
        
        for _ in 0..100 {
            let status = pipeline.check_rate_limit("user1", 100).await;
            assert!(!status.is_limited);
        }
        
        let status = pipeline.check_rate_limit("user1", 100).await;
        assert!(status.is_limited);
    }
}