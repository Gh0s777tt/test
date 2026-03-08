// VantisOS - Feedback Collector Module
// Comprehensive feedback collection, aggregation, and analysis for optimization systems

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents different types of feedback sources
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FeedbackSource {
    UserExplicit,
    UserImplicit,
    SystemMetrics,
    AutomatedMonitoring,
    ExternalApi,
    AuditLog,
}

/// Represents feedback severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeedbackSeverity {
    Critical = 5,
    High = 4,
    Medium = 3,
    Low = 2,
    Info = 1,
}

/// Represents the sentiment of user feedback
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sentiment {
    VeryPositive,
    Positive,
    Neutral,
    Negative,
    VeryNegative,
}

/// Core feedback data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub id: Uuid,
    pub source: FeedbackSource,
    pub timestamp: DateTime<Utc>,
    pub severity: FeedbackSeverity,
    pub category: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
    pub entity_id: Option<String>,
    pub entity_type: Option<String>,
    pub sentiment: Option<Sentiment>,
    pub is_anonymous: bool,
    pub user_id: Option<String>,
    pub tags: Vec<String>,
    pub is_processed: bool,
    pub processing_notes: Vec<String>,
}

impl Feedback {
    pub fn new(source: FeedbackSource, category: String, message: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            timestamp: Utc::now(),
            severity: FeedbackSeverity::Medium,
            category,
            message,
            metadata: HashMap::new(),
            entity_id: None,
            entity_type: None,
            sentiment: None,
            is_anonymous: false,
            user_id: None,
            tags: Vec::new(),
            is_processed: false,
            processing_notes: Vec::new(),
        }
    }

    pub fn with_severity(mut self, severity: FeedbackSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_entity(mut self, entity_id: String, entity_type: String) -> Self {
        self.entity_id = Some(entity_id);
        self.entity_type = Some(entity_type);
        self
    }

    pub fn with_sentiment(mut self, sentiment: Sentiment) -> Self {
        self.sentiment = Some(sentiment);
        self
    }

    pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self.is_anonymous = false;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn mark_processed(&mut self, note: String) {
        self.is_processed = true;
        self.processing_notes.push(note);
    }
}

/// Aggregated feedback statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackStats {
    pub total_feedback: usize,
    pub processed_feedback: usize,
    pub unprocessed_feedback: usize,
    pub by_source: HashMap<FeedbackSource, usize>,
    pub by_severity: HashMap<FeedbackSeverity, usize>,
    pub by_category: HashMap<String, usize>,
    pub by_sentiment: HashMap<Sentiment, usize>,
    pub average_severity: f64,
    pub positive_ratio: f64,
    pub time_window: TimeWindow,
}

/// Feedback aggregation by time periods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackTrend {
    pub period: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub feedback_count: usize,
    pub average_severity: f64,
    pub sentiment_distribution: HashMap<Sentiment, f64>,
    pub top_categories: Vec<(String, usize)>,
    pub top_issues: Vec<(String, usize)>,
}

/// Time window for feedback analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeWindow {
    pub fn last_hours(hours: i64) -> Self {
        let end = Utc::now();
        let start = end - chrono::Duration::hours(hours);
        Self { start, end }
    }

    pub fn last_days(days: i64) -> Self {
        let end = Utc::now();
        let start = end - chrono::Duration::days(days);
        Self { start, end }
    }

    pub fn custom(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }
}

/// Feedback filter criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackFilter {
    pub sources: Option<Vec<FeedbackSource>>,
    pub severities: Option<Vec<FeedbackSeverity>>,
    pub categories: Option<Vec<String>>,
    pub sentiments: Option<Vec<Sentiment>>,
    pub tags: Option<Vec<String>>,
    pub user_ids: Option<Vec<String>>,
    pub entity_ids: Option<Vec<String>>,
    pub processed_only: Option<bool>,
    pub time_window: Option<TimeWindow>,
    pub min_severity: Option<FeedbackSeverity>,
}

impl Default for FeedbackFilter {
    fn default() -> Self {
        Self {
            sources: None,
            severities: None,
            categories: None,
            sentiments: None,
            tags: None,
            user_ids: None,
            entity_ids: None,
            processed_only: None,
            time_window: None,
            min_severity: None,
        }
    }
}

/// Feedback aggregation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackCollectorConfig {
    pub max_feedback_history: usize,
    pub auto_expire_days: i64,
    pub enable_sentiment_analysis: bool,
    pub enable_trend_analysis: bool,
    pub min_samples_for_trend: usize,
    pub cache_duration_seconds: u64,
    pub retention_policy_days: i64,
    pub auto_tag_patterns: HashMap<String, Vec<String>>,
}

impl Default for FeedbackCollectorConfig {
    fn default() -> Self {
        Self {
            max_feedback_history: 100000,
            auto_expire_days: 90,
            enable_sentiment_analysis: true,
            enable_trend_analysis: true,
            min_samples_for_trend: 10,
            cache_duration_seconds: 300,
            retention_policy_days: 365,
            auto_tag_patterns: HashMap::new(),
        }
    }
}

/// Collection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectorStats {
    pub total_collected: usize,
    pub total_processed: usize,
    pub collection_rate_per_minute: f64,
    pub average_processing_time_ms: f64,
    pub last_collection_time: Option<DateTime<Utc>>,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

/// Main feedback collector
pub struct FeedbackCollector {
    feedback: Arc<RwLock<Vec<Feedback>>>,
    feedback_index: Arc<RwLock<HashMap<String, Vec<Uuid>>>>, // category -> feedback IDs
    config: FeedbackCollectorConfig,
    stats: Arc<RwLock<CollectorStats>>,
}

impl FeedbackCollector {
    pub fn new(config: FeedbackCollectorConfig) -> Self {
        Self {
            feedback: Arc::new(RwLock::new(Vec::new())),
            feedback_index: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(CollectorStats {
                total_collected: 0,
                total_processed: 0,
                collection_rate_per_minute: 0.0,
                average_processing_time_ms: 0.0,
                last_collection_time: None,
                cache_hits: 0,
                cache_misses: 0,
            })),
        }
    }

    /// Submit feedback to the collector
    pub async fn submit_feedback(&self, feedback: Feedback) -> Result<Uuid, String> {
        let id = feedback.id;
        let category = feedback.category.clone();
        
        // Add to feedback list
        {
            let mut feedback_store = self.feedback.write().await;
            feedback_store.push(feedback.clone());
            
            // Enforce history limit
            if feedback_store.len() > self.config.max_feedback_history {
                feedback_store.remove(0);
            }
        }
        
        // Update index
        {
            let mut index = self.feedback_index.write().await;
            index.entry(category).or_insert_with(Vec::new).push(id);
        }
        
        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_collected += 1;
            stats.last_collection_time = Some(Utc::now());
            
            // Calculate collection rate
            if let Some(last_time) = stats.last_collection_time {
                let duration = Utc::now().signed_duration_since(last_time);
                if duration.num_seconds() > 0 {
                    stats.collection_rate_per_minute = 
                        (stats.total_collected as f64) / (duration.num_seconds() as f64 / 60.0);
                }
            }
        }
        
        // Auto-tag if enabled
        if !self.config.auto_tag_patterns.is_empty() {
            self._auto_tag(&id).await;
        }
        
        Ok(id)
    }

    /// Submit batch feedback
    pub async fn submit_batch(&self, feedback_items: Vec<Feedback>) -> Result<Vec<Uuid>, String> {
        let mut ids = Vec::new();
        
        for feedback in feedback_items {
            let id = self.submit_feedback(feedback).await?;
            ids.push(id);
        }
        
        Ok(ids)
    }

    /// Query feedback with filters
    pub async fn query_feedback(&self, filter: &FeedbackFilter) -> Result<Vec<Feedback>, String> {
        let feedback_store = self.feedback.read().await;
        
        let results: Vec<Feedback> = feedback_store
            .iter()
            .filter(|fb| self._matches_filter(fb, filter))
            .cloned()
            .collect();
        
        Ok(results)
    }

    /// Get feedback by ID
    pub async fn get_feedback(&self, id: Uuid) -> Option<Feedback> {
        let feedback_store = self.feedback.read().await;
        feedback_store.iter().find(|fb| fb.id == id).cloned()
    }

    /// Get feedback by category
    pub async fn get_feedback_by_category(&self, category: &str) -> Result<Vec<Feedback>, String> {
        let filter = FeedbackFilter {
            categories: Some(vec![category.to_string()]),
            ..Default::default()
        };
        self.query_feedback(&filter).await
    }

    /// Get feedback for an entity
    pub async fn get_entity_feedback(&self, entity_id: &str, entity_type: &str) -> Result<Vec<Feedback>, String> {
        let filter = FeedbackFilter {
            entity_ids: Some(vec![entity_id.to_string()]),
            ..Default::default()
        };
        self.query_feedback(&filter).await
    }

    /// Get unprocessed feedback
    pub async fn get_unprocessed(&self) -> Result<Vec<Feedback>, String> {
        let filter = FeedbackFilter {
            processed_only: Some(false),
            ..Default::default()
        };
        self.query_feedback(&filter).await
    }

    /// Mark feedback as processed
    pub async fn mark_processed(&self, id: Uuid, note: String) -> Result<(), String> {
        let mut feedback_store = self.feedback.write().await;
        
        if let Some(feedback) = feedback_store.iter_mut().find(|fb| fb.id == id) {
            feedback.mark_processed(note.clone());
            
            let mut stats = self.stats.write().await;
            stats.total_processed += 1;
            
            Ok(())
        } else {
            Err(format!("Feedback with id {} not found", id))
        }
    }

    /// Get feedback statistics
    pub async fn get_stats(&self, time_window: Option<TimeWindow>) -> Result<FeedbackStats, String> {
        let filter = FeedbackFilter {
            time_window: time_window.clone(),
            ..Default::default()
        };
        
        let feedback_list = self.query_feedback(&filter).await?;
        
        let total_feedback = feedback_list.len();
        let processed_feedback = feedback_list.iter().filter(|fb| fb.is_processed).count();
        let unprocessed_feedback = total_feedback - processed_feedback;
        
        let mut by_source = HashMap::new();
        let mut by_severity = HashMap::new();
        let mut by_category = HashMap::new();
        let mut by_sentiment = HashMap::new();
        let mut severity_sum = 0i32;
        let mut positive_count = 0usize;
        let mut total_sentiment = 0usize;
        
        for fb in &feedback_list {
            *by_source.entry(fb.source.clone()).or_insert(0) += 1;
            *by_severity.entry(fb.severity).or_insert(0) += 1;
            *by_category.entry(fb.category.clone()).or_insert(0) += 1;
            
            severity_sum += fb.severity as i32;
            
            if let Some(sentiment) = fb.sentiment {
                *by_sentiment.entry(sentiment).or_insert(0) += 1;
                total_sentiment += 1;
                if matches!(sentiment, Sentiment::Positive | Sentiment::VeryPositive) {
                    positive_count += 1;
                }
            }
        }
        
        let average_severity = if total_feedback > 0 {
            severity_sum as f64 / total_feedback as f64
        } else {
            0.0
        };
        
        let positive_ratio = if total_sentiment > 0 {
            positive_count as f64 / total_sentiment as f64
        } else {
            0.0
        };
        
        Ok(FeedbackStats {
            total_feedback,
            processed_feedback,
            unprocessed_feedback,
            by_source,
            by_severity,
            by_category,
            by_sentiment,
            average_severity,
            positive_ratio,
            time_window: time_window.unwrap_or(TimeWindow::last_days(30)),
        })
    }

    /// Analyze feedback trends
    pub async fn analyze_trends(&self, days: i64) -> Result<Vec<FeedbackTrend>, String> {
        if !self.config.enable_trend_analysis {
            return Err("Trend analysis is disabled".to_string());
        }
        
        let feedback_store = self.feedback.read().await;
        let end_time = Utc::now();
        let start_time = end_time - chrono::Duration::days(days);
        
        // Group by day
        let mut daily_feedback: HashMap<String, Vec<Feedback>> = HashMap::new();
        
        for fb in feedback_store.iter() {
            if fb.timestamp >= start_time && fb.timestamp <= end_time {
                let date_key = fb.timestamp.format("%Y-%m-%d").to_string();
                daily_feedback.entry(date_key).or_insert_with(Vec::new).push(fb.clone());
            }
        }
        
        let mut trends = Vec::new();
        
        for (date, feedbacks) in daily_feedback.iter() {
            if feedbacks.len() < self.config.min_samples_for_trend {
                continue;
            }
            
            let period_start = feedbacks.iter().map(|fb| fb.timestamp).min().unwrap();
            let period_end = feedbacks.iter().map(|fb| fb.timestamp).max().unwrap();
            
            let total_severity: i32 = feedbacks.iter().map(|fb| fb.severity as i32).sum();
            let average_severity = total_severity as f64 / feedbacks.len() as f64;
            
            let mut sentiment_distribution: HashMap<Sentiment, f64> = HashMap::new();
            let mut total_with_sentiment = 0usize;
            
            for fb in feedbacks {
                if let Some(sentiment) = fb.sentiment {
                    *sentiment_distribution.entry(sentiment).or_insert(0.0) += 1.0;
                    total_with_sentiment += 1;
                }
            }
            
            // Normalize sentiment distribution
            for count in sentiment_distribution.values_mut() {
                *count /= total_with_sentiment as f64;
            }
            
            // Get top categories
            let mut category_counts: HashMap<String, usize> = HashMap::new();
            for fb in feedbacks {
                *category_counts.entry(fb.category.clone()).or_insert(0) += 1;
            }
            
            let mut top_categories: Vec<(String, usize)> = category_counts.into_iter().collect();
            top_categories.sort_by(|a, b| b.1.cmp(&a.1));
            top_categories.truncate(5);
            
            // Get top issues (high severity feedback)
            let high_severity: Vec<&Feedback> = feedbacks
                .iter()
                .filter(|fb| fb.severity >= FeedbackSeverity::High)
                .collect();
            
            let mut issue_counts: HashMap<String, usize> = HashMap::new();
            for fb in high_severity {
                *issue_counts.entry(fb.message.clone()).or_insert(0) += 1;
            }
            
            let mut top_issues: Vec<(String, usize)> = issue_counts.into_iter().collect();
            top_issues.sort_by(|a, b| b.1.cmp(&a.1));
            top_issues.truncate(5);
            
            trends.push(FeedbackTrend {
                period: date.clone(),
                start_time: period_start,
                end_time: period_end,
                feedback_count: feedbacks.len(),
                average_severity,
                sentiment_distribution,
                top_categories,
                top_issues,
            });
        }
        
        trends.sort_by(|a, b| a.start_time.cmp(&b.start_time));
        
        Ok(trends)
    }

    /// Get collection statistics
    pub async fn get_collector_stats(&self) -> CollectorStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Expire old feedback
    pub async fn expire_old_feedback(&self) -> Result<usize, String> {
        let cutoff = Utc::now() - chrono::Duration::days(self.config.auto_expire_days);
        
        let mut feedback_store = self.feedback.write().await;
        let initial_len = feedback_store.len();
        
        feedback_store.retain(|fb| fb.timestamp > cutoff);
        
        let expired_count = initial_len - feedback_store.len();
        
        // Update index
        let mut index = self.feedback_index.write().await;
        index.clear();
        
        for fb in feedback_store.iter() {
            index.entry(fb.category.clone())
                 .or_insert_with(Vec::new)
                 .push(fb.id);
        }
        
        Ok(expired_count)
    }

    /// Export feedback to JSON
    pub async fn export_feedback(&self, filter: Option<FeedbackFilter>) -> Result<String, String> {
        let filter = filter.unwrap_or_default();
        let feedback_list = self.query_feedback(&filter).await?;
        
        serde_json::to_string_pretty(&feedback_list)
            .map_err(|e| format!("Failed to serialize feedback: {}", e))
    }

    /// Import feedback from JSON
    pub async fn import_feedback(&self, json_data: &str) -> Result<usize, String> {
        let feedback_items: Vec<Feedback> = serde_json::from_str(json_data)
            .map_err(|e| format!("Failed to deserialize feedback: {}", e))?;
        
        self.submit_batch(feedback_items).await?;
        
        Ok(feedback_items.len())
    }

    /// Clear all feedback
    pub async fn clear_all(&self) -> Result<(), String> {
        let mut feedback_store = self.feedback.write().await;
        feedback_store.clear();
        
        let mut index = self.feedback_index.write().await;
        index.clear();
        
        let mut stats = self.stats.write().await;
        stats.total_collected = 0;
        stats.total_processed = 0;
        stats.collection_rate_per_minute = 0.0;
        stats.average_processing_time_ms = 0.0;
        stats.last_collection_time = None;
        
        Ok(())
    }

    // Private helper methods

    fn _matches_filter(&self, feedback: &Feedback, filter: &FeedbackFilter) -> bool {
        if let Some(sources) = &filter.sources {
            if !sources.contains(&feedback.source) {
                return false;
            }
        }
        
        if let Some(severities) = &filter.severities {
            if !severities.contains(&feedback.severity) {
                return false;
            }
        }
        
        if let Some(categories) = &filter.categories {
            if !categories.contains(&feedback.category) {
                return false;
            }
        }
        
        if let Some(sentiments) = &filter.sentiments {
            if !feedback.sentiment.as_ref().map_or(false, |s| sentiments.contains(s)) {
                return false;
            }
        }
        
        if let Some(tags) = &filter.tags {
            if !tags.iter().any(|tag| feedback.tags.contains(tag)) {
                return false;
            }
        }
        
        if let Some(user_ids) = &filter.user_ids {
            if !feedback.user_id.as_ref().map_or(false, |uid| user_ids.contains(uid)) {
                return false;
            }
        }
        
        if let Some(entity_ids) = &filter.entity_ids {
            if !feedback.entity_id.as_ref().map_or(false, |eid| entity_ids.contains(eid)) {
                return false;
            }
        }
        
        if let Some(processed_only) = filter.processed_only {
            if feedback.is_processed != processed_only {
                return false;
            }
        }
        
        if let Some(window) = &filter.time_window {
            if feedback.timestamp < window.start || feedback.timestamp > window.end {
                return false;
            }
        }
        
        if let Some(min_severity) = filter.min_severity {
            if feedback.severity < min_severity {
                return false;
            }
        }
        
        true
    }

    async fn _auto_tag(&self, feedback_id: &Uuid) {
        // Get feedback
        let feedback_opt = {
            let feedback_store = self.feedback.read().await;
            feedback_store.iter().find(|fb| fb.id == *feedback_id).cloned()
        };
        
        if let Some(mut feedback) = feedback_opt {
            // Check auto-tag patterns
            let mut new_tags = Vec::new();
            
            for (tag, patterns) in &self.config.auto_tag_patterns {
                for pattern in patterns {
                    if feedback.message.contains(pattern) || 
                       feedback.category.contains(pattern) {
                        new_tags.push(tag.clone());
                        break;
                    }
                }
            }
            
            // Update feedback with new tags
            if !new_tags.is_empty() {
                let mut feedback_store = self.feedback.write().await;
                if let Some(fb) = feedback_store.iter_mut().find(|fb| fb.id == *feedback_id) {
                    fb.tags.extend(new_tags);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_feedback_creation() {
        let feedback = Feedback::new(
            FeedbackSource::UserExplicit,
            "performance".to_string(),
            "System is slow".to_string()
        )
        .with_severity(FeedbackSeverity::High)
        .with_sentiment(Sentiment::Negative)
        .with_user("user123".to_string())
        .with_tags(vec!["performance".to_string(), "urgent".to_string()]);
        
        assert_eq!(feedback.severity, FeedbackSeverity::High);
        assert_eq!(feedback.sentiment, Some(Sentiment::Negative));
        assert_eq!(feedback.user_id, Some("user123".to_string()));
        assert_eq!(feedback.tags.len(), 2);
        assert!(!feedback.is_processed);
    }

    #[tokio::test]
    async fn test_submit_feedback() {
        let collector = FeedbackCollector::new(FeedbackCollectorConfig::default());
        
        let feedback = Feedback::new(
            FeedbackSource::UserExplicit,
            "bug".to_string(),
            "Found a bug".to_string()
        );
        
        let id = collector.submit_feedback(feedback).await.unwrap();
        
        let retrieved = collector.get_feedback(id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().category, "bug");
    }

    #[tokio::test]
    async fn test_query_feedback() {
        let collector = FeedbackCollector::new(FeedbackCollectorConfig::default());
        
        // Submit multiple feedback items
        for i in 0..5 {
            let feedback = Feedback::new(
                FeedbackSource::UserExplicit,
                if i % 2 == 0 { "bug" } else { "feature" },
                format!("Feedback {}", i)
            ).with_severity(if i % 3 == 0 { FeedbackSeverity::High } else { FeedbackSeverity::Medium });
            
            collector.submit_feedback(feedback).await.unwrap();
        }
        
        // Query for bugs only
        let filter = FeedbackFilter {
            categories: Some(vec!["bug".to_string()]),
            ..Default::default()
        };
        
        let results = collector.query_feedback(&filter).await.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_feedback_stats() {
        let collector = FeedbackCollector::new(FeedbackCollectorConfig::default());
        
        for i in 0..10 {
            let feedback = Feedback::new(
                FeedbackSource::UserExplicit,
                "test".to_string(),
                format!("Message {}", i)
            ).with_severity(match i {
                0..=2 => FeedbackSeverity::High,
                3..=6 => FeedbackSeverity::Medium,
                _ => FeedbackSeverity::Low,
            }).with_sentiment(match i {
                0..=3 => Sentiment::Negative,
                4..=7 => Sentiment::Neutral,
                _ => Sentiment::Positive,
            });
            
            collector.submit_feedback(feedback).await.unwrap();
        }
        
        let stats = collector.get_stats(None).await.unwrap();
        assert_eq!(stats.total_feedback, 10);
        assert_eq!(stats.by_category.get("test"), Some(&10));
        assert!(stats.average_severity > 0.0);
        assert!(stats.positive_ratio > 0.0);
    }

    #[tokio::test]
    async fn test_mark_processed() {
        let collector = FeedbackCollector::new(FeedbackCollectorConfig::default());
        
        let feedback = Feedback::new(
            FeedbackSource::UserExplicit,
            "bug".to_string(),
            "Bug report".to_string()
        );
        
        let id = collector.submit_feedback(feedback).await.unwrap();
        
        assert!(!collector.get_feedback(id).await.unwrap().is_processed);
        
        collector.mark_processed(id, "Fixed".to_string()).await.unwrap();
        
        assert!(collector.get_feedback(id).await.unwrap().is_processed);
    }

    #[tokio::test]
    async fn test_time_window_filter() {
        let collector = FeedbackCollector::new(FeedbackCollectorConfig::default());
        
        // Submit feedback at different times
        let now = Utc::now();
        
        // Recent feedback
        let mut recent_feedback = Feedback::new(
            FeedbackSource::UserExplicit,
            "test".to_string(),
            "Recent".to_string()
        );
        recent_feedback.timestamp = now - chrono::Duration::minutes(30);
        collector.submit_feedback(recent_feedback).await.unwrap();
        
        // Old feedback
        let mut old_feedback = Feedback::new(
            FeedbackSource::UserExplicit,
            "test".to_string(),
            "Old".to_string()
        );
        old_feedback.timestamp = now - chrono::Duration::days(2);
        collector.submit_feedback(old_feedback).await.unwrap();
        
        // Query for last hour
        let filter = FeedbackFilter {
            time_window: Some(TimeWindow::last_hours(1)),
            ..Default::default()
        };
        
        let results = collector.query_feedback(&filter).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].message, "Recent");
    }
}