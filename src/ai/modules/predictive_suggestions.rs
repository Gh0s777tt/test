//! Predictive Suggestions and Recommendations
//! 
//! This module implements an intelligent suggestion system that uses machine
//! learning to predict user needs and provide contextual recommendations based
//! on usage patterns, current context, and historical behavior.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Predictive suggestions errors
#[derive(Error, Debug)]
pub enum SuggestionsError {
    #[error("No suggestions available")]
    NoSuggestionsAvailable,
    
    #[error("Insufficient data for prediction")]
    InsufficientData,
    
    #[error("Invalid context: {0}")]
    InvalidContext(String),
}

/// Configuration for predictive suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionsConfig {
    /// Maximum number of suggestions to return
    pub max_suggestions: usize,
    
    /// Minimum confidence threshold
    pub min_confidence: f64,
    
    /// Enable real-time suggestions
    pub enable_real_time: bool,
    
    /// Learning rate for pattern updates
    pub learning_rate: f64,
    
    /// History size for patterns
    pub history_size: usize,
    
    /// Enable context-aware suggestions
    pub enable_context_aware: bool,
    
    /// Enable collaborative filtering
    pub enable_collaborative: bool,
}

/// Suggestion type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SuggestionType {
    Application,
    File,
    Command,
    Action,
    Setting,
    Content,
    Contact,
    Location,
}

/// Suggestion item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    /// Suggestion identifier
    pub suggestion_id: String,
    
    /// Suggestion type
    pub suggestion_type: SuggestionType,
    
    /// Suggestion title
    pub title: String,
    
    /// Suggestion description
    pub description: String,
    
    /// Confidence score (0-1)
    pub confidence: f64,
    
    /// Relevance score
    pub relevance: f64,
    
    /// Action to perform
    pub action: SuggestionAction,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Action to perform on suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionAction {
    Launch { target: String },
    Open { path: String },
    Execute { command: String },
    Navigate { url: String },
    Configure { setting: String, value: String },
    Custom { action_type: String, params: HashMap<String, String> },
}

/// User activity event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    /// Activity ID
    pub activity_id: String,
    
    /// Activity type
    pub activity_type: String,
    
    /// Activity timestamp
    pub timestamp: Instant,
    
    /// Context information
    pub context: ActivityContext,
    
    /// Duration
    pub duration: Duration,
    
    /// Success flag
    pub success: bool,
}

/// Activity context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityContext {
    /// Current application
    pub application: Option<String>,
    
    /// Current file
    pub file: Option<String>,
    
    /// Time of day
    pub time_of_day: String,
    
    /// Day of week
    pub day_of_week: String,
    
    /// Location (if available)
    pub location: Option<String>,
    
    /// Device type
    pub device_type: String,
    
    /// Additional context
    pub additional: HashMap<String, String>,
}

/// Usage pattern
#[derive(Debug, Clone)]
struct UsagePattern {
    /// Pattern key
    pattern_key: String,
    
    /// Frequency
    frequency: f64,
    
    /// Last occurrence
    last_occurrence: Instant,
    
    /// Confidence
    confidence: f64,
    
    /// Associated actions
    actions: Vec<String>,
}

/// Prediction model
#[derive(Debug)]
struct PredictionModel {
    /// Historical activities
    activities: VecDeque<UserActivity>,
    
    /// Usage patterns
    patterns: HashMap<String, UsagePattern>,
    
    /// Learning rate
    learning_rate: f64,
    
    /// Time-based patterns
    time_patterns: HashMap<String, Vec<String>>,
}

impl PredictionModel {
    pub fn new(learning_rate: f64, history_size: usize) -> Self {
        Self {
            activities: VecDeque::with_capacity(history_size),
            patterns: HashMap::new(),
            learning_rate,
            time_patterns: HashMap::new(),
        }
    }

    /// Add user activity to model
    pub fn add_activity(&mut self, activity: UserActivity) {
        if self.activities.len() >= self.activities.capacity() {
            self.activities.pop_front();
        }
        self.activities.push_back(activity.clone());

        // Update patterns
        self.update_patterns(&activity);
    }

    /// Update patterns based on activity
    fn update_patterns(&mut self, activity: &UserActivity) {
        // Create pattern key based on context
        let pattern_key = format!("{}:{}:{}",
            activity.activity_type,
            activity.context.time_of_day,
            activity.context.day_of_week
        );

        // Update or create pattern
        let pattern = self.patterns.entry(pattern_key.clone()).or_insert(UsagePattern {
            pattern_key: pattern_key.clone(),
            frequency: 0.0,
            last_occurrence: activity.timestamp,
            confidence: 0.0,
            actions: Vec::new(),
        });

        pattern.frequency += 1.0;
        pattern.last_occurrence = activity.timestamp;
        
        if !pattern.actions.contains(&activity.activity_type) {
            pattern.actions.push(activity.activity_type.clone());
        }

        pattern.confidence = (pattern.frequency / 10.0).min(1.0);

        // Update time-based patterns
        let time_key = format!("{}:{}", 
            activity.context.time_of_day,
            activity.context.day_of_week
        );
        
        self.time_patterns.entry(time_key.clone())
            .or_insert_with(Vec::new)
            .push(activity.activity_type.clone());
    }

    /// Predict next actions
    pub fn predict(&self, context: &ActivityContext, max_predictions: usize) -> Vec<(String, f64)> {
        let mut predictions: Vec<(String, f64)> = Vec::new();
        let pattern_key = format!("{}:{}:{}",
            context.application.as_deref().unwrap_or("unknown"),
            context.time_of_day,
            context.day_of_week
        );

        // Get matching patterns
        if let Some(pattern) = self.patterns.get(&pattern_key) {
            for action in &pattern.actions {
                predictions.push((
                    action.clone(),
                    pattern.confidence
                ));
            }
        }

        // Sort by confidence
        predictions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Limit predictions
        predictions.truncate(max_predictions);

        predictions
    }
}

/// Predictive Suggestions Engine
pub struct PredictiveSuggestionsEngine {
    config: SuggestionsConfig,
    
    /// Prediction model
    model: Arc<RwLock<PredictionModel>>,
    
    /// Current context
    current_context: Arc<RwLock<Option<ActivityContext>>>,
    
    /// Suggestion cache
    suggestion_cache: Arc<RwLock<VecDeque<Suggestion>>>,
    
    /// User preferences
    user_preferences: Arc<RwLock<HashMap<String, String>>>,
    
    /// Blocked suggestions
    blocked_suggestions: Arc<RwLock<HashSet<String>>>,
}

impl PredictiveSuggestionsEngine {
    pub fn new(config: SuggestionsConfig) -> Self {
        let model = PredictionModel::new(config.learning_rate, config.history_size);

        Self {
            config,
            model: Arc::new(RwLock::new(model)),
            current_context: Arc::new(RwLock::new(None)),
            suggestion_cache: Arc::new(RwLock::new(VecDeque::with_capacity(100))),
            user_preferences: Arc::new(RwLock::new(HashMap::new())),
            blocked_suggestions: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Record user activity
    pub async fn record_activity(&self, activity: UserActivity) {
        // Update context
        {
            let mut context = self.current_context.write().await;
            *context = Some(activity.context.clone());
        }

        // Add to model
        {
            let mut model = self.model.write().await;
            model.add_activity(activity.clone());
        }

        // Update suggestions if real-time is enabled
        if self.config.enable_real_time {
            if let Err(e) = self.update_suggestions().await {
                log::error!("Failed to update suggestions: {}", e);
            }
        }
    }

    /// Get suggestions based on current context
    pub async fn get_suggestions(&self) -> Result<Vec<Suggestion>, SuggestionsError> {
        let context = {
            let ctx = self.current_context.read().await;
            ctx.clone().ok_or(SuggestionsError::InvalidContext(
                "No current context".to_string()
            ))?
        };

        let predictions = {
            let model = self.model.read().await;
            model.predict(&context, self.config.max_suggestions)
        };

        if predictions.is_empty() {
            return Err(SuggestionsError::NoSuggestionsAvailable);
        }

        // Convert predictions to suggestions
        let suggestions: Vec<Suggestion> = predictions.into_iter()
            .enumerate()
            .filter_map(|(idx, (action, confidence))| {
                if confidence >= self.config.min_confidence {
                    Some(self.create_suggestion(action, confidence, idx, &context))
                } else {
                    None
                }
            })
            .collect();

        if suggestions.is_empty() {
            return Err(SuggestionsError::NoSuggestionsAvailable);
        }

        Ok(suggestions)
    }

    /// Create suggestion from prediction
    fn create_suggestion(&self, action: String, confidence: f64, idx: usize, context: &ActivityContext) -> Suggestion {
        let (suggestion_type, title, description, action_enum) = match action.as_str() {
            "launch_app" => {
                let app = context.application.as_deref().unwrap_or("Application");
                (SuggestionType::Application,
                 format!("Launch {}", app),
                 format!("Open {} based on your usage patterns", app),
                 SuggestionAction::Launch { 
                     target: app.to_string() 
                 })
            },
            "open_file" => {
                let file = context.file.as_deref().unwrap_or("File");
                (SuggestionType::File,
                 format!("Open {}", file),
                 "Quick access to recent file".to_string(),
                 SuggestionAction::Open { 
                     path: file.to_string() 
                 })
            },
            "execute_command" => {
                (SuggestionType::Command,
                 "Quick Command".to_string(),
                 "Execute frequently used command".to_string(),
                 SuggestionAction::Execute { 
                     command: "command".to_string() 
                 })
            },
            _ => {
                (SuggestionType::Action,
                 action.clone(),
                 "Suggested action".to_string(),
                 SuggestionAction::Custom { 
                     action_type: action.clone(),
                     params: HashMap::new(),
                 })
            }
        };

        Suggestion {
            suggestion_id: format!("suggestion_{}", idx),
            suggestion_type,
            title,
            description,
            confidence,
            relevance: confidence,
            action: action_enum,
            metadata: HashMap::new(),
        }
    }

    /// Update suggestion cache
    async fn update_suggestions(&self) -> Result<(), SuggestionsError> {
        let suggestions = self.get_suggestions().await?;
        
        let mut cache = self.suggestion_cache.write().await;
        cache.clear();
        
        for suggestion in suggestions {
            cache.push_back(suggestion);
        }

        Ok(())
    }

    /// Reject a suggestion
    pub async fn reject_suggestion(&self, suggestion_id: &str) {
        let mut blocked = self.blocked_suggestions.write().await;
        blocked.insert(suggestion_id.to_string());
        
        // Remove from cache
        let mut cache = self.suggestion_cache.write().await;
        cache.retain(|s| s.suggestion_id != suggestion_id);
    }

    /// Accept a suggestion
    pub async fn accept_suggestion(&self, suggestion_id: &str) -> Result<(), SuggestionsError> {
        let suggestion = {
            let cache = self.suggestion_cache.read().await;
            cache.iter()
                .find(|s| s.suggestion_id == suggestion_id)
                .cloned()
                .ok_or(SuggestionsError::NoSuggestionsAvailable)?
        };

        // Execute the suggestion action
        log::info!("Executing suggestion: {}", suggestion.title);
        
        // In a real implementation, this would execute the action
        match &suggestion.action {
            SuggestionAction::Launch { target } => {
                log::info!("Launching application: {}", target);
            },
            SuggestionAction::Open { path } => {
                log::info!("Opening file: {}", path);
            },
            SuggestionAction::Execute { command } => {
                log::info!("Executing command: {}", command);
            },
            SuggestionAction::Navigate { url } => {
                log::info!("Navigating to: {}", url);
            },
            SuggestionAction::Configure { setting, value } => {
                log::info!("Configuring: {} = {}", setting, value);
            },
            SuggestionAction::Custom { action_type, params } => {
                log::info!("Custom action: {:?}, params: {:?}", action_type, params);
            },
        }

        Ok(())
    }

    /// Update user preferences
    pub async fn update_preferences(&self, key: String, value: String) {
        let mut prefs = self.user_preferences.write().await;
        prefs.insert(key, value);
    }

    /// Get prediction accuracy
    pub async fn get_accuracy(&self) -> f64 {
        let model = self.model.read().await;
        let total_activities = model.activities.len();
        
        if total_activities < 10 {
            return 0.0;
        }

        // Calculate accuracy based on pattern confidence
        let total_confidence: f64 = model.patterns.values()
            .map(|p| p.confidence)
            .sum();

        total_confidence / model.patterns.len().max(1) as f64
    }

    /// Get statistics
    pub async fn get_statistics(&self) -> SuggestionsStatistics {
        let model = self.model.read().await;
        let cache = self.suggestion_cache.read().await;

        SuggestionsStatistics {
            total_activities: model.activities.len(),
            total_patterns: model.patterns.len(),
            cached_suggestions: cache.len(),
            accuracy: self.get_accuracy().await,
        }
    }
}

/// Suggestions statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionsStatistics {
    pub total_activities: usize,
    pub total_patterns: usize,
    pub cached_suggestions: usize,
    pub accuracy: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestions_config() {
        let config = SuggestionsConfig {
            max_suggestions: 10,
            min_confidence: 0.5,
            enable_real_time: true,
            learning_rate: 0.1,
            history_size: 100,
            enable_context_aware: true,
            enable_collaborative: false,
        };

        assert_eq!(config.max_suggestions, 10);
        assert!(config.enable_real_time);
    }

    #[tokio::test]
    async fn test_suggestions_engine_creation() {
        let config = SuggestionsConfig {
            max_suggestions: 10,
            min_confidence: 0.5,
            enable_real_time: true,
            learning_rate: 0.1,
            history_size: 100,
            enable_context_aware: true,
            enable_collaborative: false,
        };

        let engine = PredictiveSuggestionsEngine::new(config);
        let stats = engine.get_statistics().await;
        
        assert_eq!(stats.total_activities, 0);
        assert_eq!(stats.total_patterns, 0);
    }

    #[tokio::test]
    async fn test_activity_recording() {
        let config = SuggestionsConfig {
            max_suggestions: 10,
            min_confidence: 0.5,
            enable_real_time: true,
            learning_rate: 0.1,
            history_size: 100,
            enable_context_aware: true,
            enable_collaborative: false,
        };

        let engine = PredictiveSuggestionsEngine::new(config);
        
        let activity = UserActivity {
            activity_id: "1".to_string(),
            activity_type: "launch_app".to_string(),
            timestamp: Instant::now(),
            context: ActivityContext {
                application: Some("Browser".to_string()),
                file: None,
                time_of_day: "morning".to_string(),
                day_of_week: "Monday".to_string(),
                location: None,
                device_type: "desktop".to_string(),
                additional: HashMap::new(),
            },
            duration: Duration::from_secs(300),
            success: true,
        };

        engine.record_activity(activity).await;
        
        let stats = engine.get_statistics().await;
        assert_eq!(stats.total_activities, 1);
    }
}