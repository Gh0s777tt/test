//! AI Decision Audit Trail Module
//!
//! This module provides comprehensive audit logging for AI system decisions,
//! ensuring traceability and accountability for automated decisions.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Audit entry for AI decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique entry ID
    pub id: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Decision ID
    pub decision_id: String,
    /// User ID (if applicable)
    pub user_id: Option<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// Model name
    pub model_name: String,
    /// Model version
    pub model_version: String,
    /// Decision type
    pub decision_type: DecisionType,
    /// Input data (hashed)
    pub input_hash: String,
    /// Output/prediction
    pub output: serde_json::Value,
    /// Confidence score
    pub confidence: f64,
    /// Decision rationale
    pub rationale: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// IP address
    pub ip_address: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Explanation ID (if available)
    pub explanation_id: Option<String>,
    /// Review status
    pub review_status: ReviewStatus,
    /// Reviewed by
    pub reviewed_by: Option<String>,
    /// Review timestamp
    pub review_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// Review notes
    pub review_notes: Option<String>,
}

/// Decision types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DecisionType {
    /// Classification decision
    Classification,
    /// Regression prediction
    Regression,
    /// Recommendation
    Recommendation,
    /// Automation trigger
    AutomationTrigger,
    /// Content moderation
    ContentModeration,
    /// Risk assessment
    RiskAssessment,
    /// Anomaly detection
    AnomalyDetection,
    /// Natural language generation
    TextGeneration,
    /// Voice command processing
    VoiceCommand,
}

/// Review status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReviewStatus {
    Pending,
    Approved,
    Rejected,
    FlaggedForReview,
    ManuallyOverridden,
}

/// Audit query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditQuery {
    /// Filter by decision ID
    pub decision_id: Option<String>,
    /// Filter by user ID
    pub user_id: Option<String>,
    /// Filter by model name
    pub model_name: Option<String>,
    /// Filter by decision type
    pub decision_type: Option<DecisionType>,
    /// Filter by review status
    pub review_status: Option<ReviewStatus>,
    /// Start timestamp
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    /// End timestamp
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Minimum confidence
    pub min_confidence: Option<f64>,
    /// Maximum confidence
    pub max_confidence: Option<f64>,
    /// Limit results
    pub limit: Option<usize>,
    /// Offset
    pub offset: Option<usize>,
}

/// Audit statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditStats {
    /// Total entries
    pub total_entries: usize,
    /// Entries by decision type
    pub by_decision_type: HashMap<DecisionType, usize>,
    /// Entries by model
    pub by_model: HashMap<String, usize>,
    /// Entries by review status
    pub by_review_status: HashMap<ReviewStatus, usize>,
    /// Average confidence
    pub avg_confidence: f64,
    /// Entries requiring review
    pub pending_review: usize,
}

/// Audit trail manager
pub struct AuditTrailManager {
    entries: Arc<RwLock<Vec<AuditEntry>>>,
    retention_days: u64,
    statistics: Arc<RwLock<AuditStats>>,
}

impl AuditTrailManager {
    /// Create a new audit trail manager
    pub fn new(retention_days: u64) -> Result<Self> {
        info!("Initializing Audit Trail Manager with {} day retention", retention_days);

        Ok(Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            retention_days,
            statistics: Arc::new(RwLock::new(AuditStats {
                total_entries: 0,
                by_decision_type: HashMap::new(),
                by_model: HashMap::new(),
                by_review_status: HashMap::new(),
                avg_confidence: 0.0,
                pending_review: 0,
            })),
        })
    }

    /// Log an AI decision
    pub async fn log_decision(&self, mut entry: AuditEntry) -> Result<()> {
        entry.id = uuid::Uuid::new_v4().to_string();
        entry.timestamp = chrono::Utc::now();
        
        // Set default review status
        if entry.review_status == ReviewStatus::Pending {
            entry.review_status = ReviewStatus::FlaggedForReview;
        }

        // Calculate input hash if not provided
        if entry.input_hash.is_empty() {
            entry.input_hash = self.calculate_hash(&entry.output.to_string())?;
        }

        // Store entry
        {
            let mut entries = self.entries.write().await;
            entries.push(entry.clone());
        }

        // Update statistics
        self.update_statistics(&entry).await;

        debug!("Logged audit entry for decision: {}", entry.decision_id);
        Ok(())
    }

    /// Calculate hash for input data
    fn calculate_hash(&self, data: &str) -> Result<String> {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        data.hash(&mut hasher);
        Ok(format!("{:x}", hasher.finish()))
    }

    /// Update statistics
    async fn update_statistics(&self, entry: &AuditEntry) {
        let mut stats = self.statistics.write().await;
        
        stats.total_entries += 1;
        *stats.by_decision_type.entry(entry.decision_type).or_insert(0) += 1;
        *stats.by_model.entry(entry.model_name.clone()).or_insert(0) += 1;
        *stats.by_review_status.entry(entry.review_status).or_insert(0) += 1;
        
        stats.avg_confidence = (stats.avg_confidence * (stats.total_entries - 1) as f64 + entry.confidence)
            / stats.total_entries as f64;
        
        if entry.review_status == ReviewStatus::Pending || entry.review_status == ReviewStatus::FlaggedForReview {
            stats.pending_review += 1;
        }
    }

    /// Query audit entries
    pub async fn query(&self, query: AuditQuery) -> Result<Vec<AuditEntry>> {
        let entries = self.entries.read().await;

        let filtered: Vec<_> = entries
            .iter()
            .filter(|entry| {
                if let Some(ref id) = query.decision_id {
                    if &entry.decision_id != id {
                        return false;
                    }
                }
                if let Some(ref uid) = query.user_id {
                    if entry.user_id.as_ref() != Some(uid) {
                        return false;
                    }
                }
                if let Some(ref model) = query.model_name {
                    if &entry.model_name != model {
                        return false;
                    }
                }
                if let Some(dt) = query.decision_type {
                    if entry.decision_type != dt {
                        return false;
                    }
                }
                if let Some(status) = query.review_status {
                    if entry.review_status != status {
                        return false;
                    }
                }
                if let Some(start) = query.start_time {
                    if entry.timestamp < start {
                        return false;
                    }
                }
                if let Some(end) = query.end_time {
                    if entry.timestamp > end {
                        return false;
                    }
                }
                if let Some(min_conf) = query.min_confidence {
                    if entry.confidence < min_conf {
                        return false;
                    }
                }
                if let Some(max_conf) = query.max_confidence {
                    if entry.confidence > max_conf {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Apply offset and limit
        let offset = query.offset.unwrap_or(0);
        let limit = query.limit.unwrap_or(filtered.len());

        Ok(filtered
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect())
    }

    /// Get entry by ID
    pub async fn get_entry(&self, id: &str) -> Option<AuditEntry> {
        let entries = self.entries.read().await;
        entries.iter().find(|e| e.id == id).cloned()
    }

    /// Get entries for a decision
    pub async fn get_decision_entries(&self, decision_id: &str) -> Vec<AuditEntry> {
        let query = AuditQuery {
            decision_id: Some(decision_id.to_string()),
            limit: None,
            ..Default::default()
        };
        self.query(query).await.unwrap_or_default()
    }

    /// Review an entry
    pub async fn review_entry(
        &self,
        entry_id: &str,
        reviewer: &str,
        status: ReviewStatus,
        notes: Option<String>,
    ) -> Result<()> {
        let mut entries = self.entries.write().await;
        
        if let Some(entry) = entries.iter_mut().find(|e| e.id == entry_id) {
            entry.review_status = status;
            entry.reviewed_by = Some(reviewer.to_string());
            entry.review_timestamp = Some(chrono::Utc::now());
            entry.review_notes = notes;

            // Update pending count in stats
            if status != ReviewStatus::Pending && status != ReviewStatus::FlaggedForReview {
                let mut stats = self.statistics.write().await;
                if stats.pending_review > 0 {
                    stats.pending_review -= 1;
                }
            }

            info!("Reviewed entry {}: {:?} by {}", entry_id, status, reviewer);
            return Ok(());
        }

        Err(anyhow::anyhow!("Entry not found: {}", entry_id))
    }

    /// Mark entry as manually overridden
    pub async fn mark_overridden(
        &self,
        entry_id: &str,
        reviewer: &str,
        reason: String,
    ) -> Result<()> {
        self.review_entry(
            entry_id,
            reviewer,
            ReviewStatus::ManuallyOverridden,
            Some(reason),
        )
        .await
    }

    /// Flag entry for review
    pub async fn flag_for_review(&self, entry_id: &str, reason: String) -> Result<()> {
        let mut entries = self.entries.write().await;
        
        if let Some(entry) = entries.iter_mut().find(|e| e.id == entry_id) {
            entry.review_status = ReviewStatus::FlaggedForReview;
            entry.review_notes = Some(reason);

            info!("Flagged entry {} for review", entry_id);
            return Ok(());
        }

        Err(anyhow::anyhow!("Entry not found: {}", entry_id))
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> AuditStats {
        self.statistics.read().await.clone()
    }

    /// Get recent entries
    pub async fn get_recent_entries(&self, limit: usize) -> Vec<AuditEntry> {
        let entries = self.entries.read().await;
        entries.iter().rev().take(limit).cloned().collect()
    }

    /// Get entries pending review
    pub async fn get_pending_review(&self) -> Vec<AuditEntry> {
        let query = AuditQuery {
            review_status: Some(ReviewStatus::Pending),
            limit: None,
            ..Default::default()
        };
        self.query(query).await.unwrap_or_default()
    }

    /// Clean up old entries based on retention policy
    pub async fn cleanup_old_entries(&self) -> Result<usize> {
        let cutoff = chrono::Utc::now() - chrono::Duration::days(self.retention_days as i64);
        
        let mut entries = self.entries.write().await;
        let initial_len = entries.len();
        
        entries.retain(|e| e.timestamp > cutoff);
        
        let removed = initial_len - entries.len();
        
        if removed > 0 {
            info!("Cleaned up {} old audit entries", removed);
        }

        Ok(removed)
    }

    /// Export audit trail
    pub async fn export_audit_trail(&self, format: ExportFormat) -> Result<String> {
        let entries = self.entries.read().await;
        
        match format {
            ExportFormat::Json => {
                Ok(serde_json::to_string_pretty(&*entries)?)
            }
            ExportFormat::Csv => {
                let mut csv = String::from(
                    "id,timestamp,decision_id,user_id,model_name,decision_type,confidence,review_status\n"
                );
                for entry in entries.iter() {
                    csv.push_str(&format!(
                        "{},{},{},{},{},{:?},{},{:?}\n",
                        entry.id,
                        entry.timestamp.to_rfc3339(),
                        entry.decision_id,
                        entry.user_id.as_deref().unwrap_or(""),
                        entry.model_name,
                        entry.decision_type,
                        entry.confidence,
                        entry.review_status
                    ));
                }
                Ok(csv)
            }
            ExportFormat::Html => {
                let mut html = String::from("<html><body><h1>AI Decision Audit Trail</h1>");
                html.push_str("<table border='1'><tr><th>ID</th><th>Timestamp</th><th>Decision ID</th><th>Model</th><th>Type</th><th>Confidence</th><th>Status</th></tr>");
                for entry in entries.iter() {
                    html.push_str(&format!(
                        "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{:?}</td><td>{:.2}</td><td>{:?}</td></tr>",
                        entry.id,
                        entry.timestamp.to_rfc3339(),
                        entry.decision_id,
                        entry.model_name,
                        entry.decision_type,
                        entry.confidence,
                        entry.review_status
                    ));
                }
                html.push_str("</table></body></html>");
                Ok(html)
            }
        }
    }
}

impl Default for AuditQuery {
    fn default() -> Self {
        Self {
            decision_id: None,
            user_id: None,
            model_name: None,
            decision_type: None,
            review_status: None,
            start_time: None,
            end_time: None,
            min_confidence: None,
            max_confidence: None,
            limit: None,
            offset: None,
        }
    }
}

/// Export format for audit trail
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Csv,
    Html,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audit_trail_creation() {
        let manager = AuditTrailManager::new(365);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_log_decision() {
        let manager = AuditTrailManager::new(365).unwrap();

        let entry = AuditEntry {
            id: String::new(),
            timestamp: chrono::Utc::now(),
            decision_id: "decision-1".to_string(),
            user_id: Some("user-123".to_string()),
            session_id: Some("session-456".to_string()),
            model_name: "test_model".to_string(),
            model_version: "1.0.0".to_string(),
            decision_type: DecisionType::Classification,
            input_hash: String::new(),
            output: serde_json::json!({"prediction": "positive"}),
            confidence: 0.95,
            rationale: "High confidence based on input features".to_string(),
            metadata: HashMap::new(),
            ip_address: None,
            user_agent: None,
            explanation_id: None,
            review_status: ReviewStatus::Pending,
            reviewed_by: None,
            review_timestamp: None,
            review_notes: None,
        };

        let result = manager.log_decision(entry).await;
        assert!(result.is_ok());

        let stats = manager.get_statistics().await;
        assert_eq!(stats.total_entries, 1);
    }

    #[tokio::test]
    async fn test_query_audit_trail() {
        let manager = AuditTrailManager::new(365).unwrap();

        let entry = AuditEntry {
            id: String::new(),
            timestamp: chrono::Utc::now(),
            decision_id: "decision-1".to_string(),
            user_id: Some("user-123".to_string()),
            session_id: None,
            model_name: "test_model".to_string(),
            model_version: "1.0.0".to_string(),
            decision_type: DecisionType::Classification,
            input_hash: String::new(),
            output: serde_json::json!({"prediction": "positive"}),
            confidence: 0.95,
            rationale: "Test".to_string(),
            metadata: HashMap::new(),
            ip_address: None,
            user_agent: None,
            explanation_id: None,
            review_status: ReviewStatus::Pending,
            reviewed_by: None,
            review_timestamp: None,
            review_notes: None,
        };

        manager.log_decision(entry).await.unwrap();

        let query = AuditQuery {
            decision_id: Some("decision-1".to_string()),
            ..Default::default()
        };

        let results = manager.query(query).await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_review_entry() {
        let manager = AuditTrailManager::new(365).unwrap();

        let entry = AuditEntry {
            id: String::new(),
            timestamp: chrono::Utc::now(),
            decision_id: "decision-1".to_string(),
            user_id: None,
            session_id: None,
            model_name: "test_model".to_string(),
            model_version: "1.0.0".to_string(),
            decision_type: DecisionType::Classification,
            input_hash: String::new(),
            output: serde_json::json!({"prediction": "positive"}),
            confidence: 0.95,
            rationale: "Test".to_string(),
            metadata: HashMap::new(),
            ip_address: None,
            user_agent: None,
            explanation_id: None,
            review_status: ReviewStatus::Pending,
            reviewed_by: None,
            review_timestamp: None,
            review_notes: None,
        };

        manager.log_decision(entry).await.unwrap();

        let entries = manager.get_recent_entries(1).await;
        let entry_id = &entries[0].id;

        manager.review_entry(entry_id, "reviewer", ReviewStatus::Approved, None).await.unwrap();

        let updated = manager.get_entry(entry_id).await.unwrap();
        assert_eq!(updated.review_status, ReviewStatus::Approved);
        assert_eq!(updated.reviewed_by, Some("reviewer".to_string()));
    }
}