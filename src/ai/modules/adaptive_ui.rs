//! Adaptive User Interface with Personalization
//! 
//! This module implements an intelligent, adaptive UI system that learns from
//! user behavior patterns and automatically adjusts the interface layout,
 styling, and functionality to provide a personalized user experience.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Adaptive UI errors
#[derive(Error, Debug)]
pub enum AdaptiveUiError {
    #[error("Invalid UI layout: {0}")]
    InvalidLayout(String),
    
    #[error("Personalization failed: {0}")]
    PersonalizationFailed(String),
    
    #[error("Theme not found: {0}")]
    ThemeNotFound(String),
    
    #[error("User profile not found")]
    UserProfileNotFound,
}

/// Configuration for adaptive UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveUiConfig {
    /// Enable automatic layout adaptation
    pub enable_layout_adaptation: bool,
    
    /// Enable theme adaptation
    pub enable_theme_adaptation: bool,
    
    /// Enable behavior learning
    pub enable_behavior_learning: bool,
    
    /// Learning rate for user preferences
    pub learning_rate: f64,
    
    /// Minimum samples before adaptation
    pub min_samples_for_adaptation: usize,
    
    /// Adaptation threshold (0-1)
    pub adaptation_threshold: f64,
    
    /// Enable accessibility features
    pub enable_accessibility: bool,
}

/// UI component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiComponent {
    /// Component identifier
    pub component_id: String,
    
    /// Component type
    pub component_type: ComponentType,
    
    /// Position (x, y)
    pub position: (f64, f64),
    
    /// Size (width, height)
    pub size: (f64, f64),
    
    /// Visibility
    pub visible: bool,
    
    /// Z-index for layering
    pub z_index: u32,
    
    /// Custom properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// Types of UI components
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ComponentType {
    Button,
    TextField,
    Label,
    Image,
    Container,
    Menu,
    StatusBar,
    Window,
    Panel,
    Icon,
    Slider,
    Checkbox,
    Dropdown,
}

/// UI layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiLayout {
    /// Layout identifier
    pub layout_id: String,
    
    /// Layout name
    pub name: String,
    
    /// Components in the layout
    pub components: Vec<UiComponent>,
    
    /// Layout type
    pub layout_type: LayoutType,
    
    /// Adaptation score
    pub adaptation_score: f64,
}

/// Layout types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LayoutType {
    Grid,
    Flex,
    Absolute,
    Dynamic,
    Adaptive,
}

/// UI theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiTheme {
    /// Theme identifier
    pub theme_id: String,
    
    /// Theme name
    pub name: String,
    
    /// Primary color
    pub primary_color: String,
    
    /// Secondary color
    pub secondary_color: String,
    
    /// Background color
    pub background_color: String,
    
    /// Text color
    pub text_color: String,
    
    /// Font family
    pub font_family: String,
    
    /// Font size
    pub font_size: u32,
    
    /// Border radius
    pub border_radius: u32,
    
    /// Spacing
    pub spacing: u32,
    
    /// Dark mode flag
    pub dark_mode: bool,
}

/// User interaction event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionEvent {
    /// Event ID
    pub event_id: String,
    
    /// Event type
    pub event_type: InteractionType,
    
    /// Component ID
    pub component_id: String,
    
    /// Timestamp
    pub timestamp: Instant,
    
    /// Duration of interaction
    pub duration: Option<Duration>,
    
    /// Position
    pub position: Option<(f64, f64)>,
    
    /// Additional data
    pub data: Option<serde_json::Value>,
}

/// Types of interactions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InteractionType {
    Click,
    Hover,
    Drag,
    Scroll,
    Type,
    Select,
    Focus,
    Blur,
}

/// User behavior pattern
#[derive(Debug, Clone)]
struct BehaviorPattern {
    /// Pattern type
    pattern_type: String,
    
    /// Frequency
    frequency: f64,
    
    /// Last seen
    last_seen: Instant,
    
    /// Confidence
    confidence: f64,
}

/// User profile for personalization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID
    pub user_id: String,
    
    /// Preferred layout
    pub preferred_layout: Option<String>,
    
    /// Preferred theme
    pub preferred_theme: Option<String>,
    
    /// Accessibility settings
    pub accessibility_settings: AccessibilitySettings,
    
    /// Behavior patterns
    pub behavior_patterns: HashMap<String, f64>,
    
    /// Custom preferences
    pub preferences: HashMap<String, String>,
}

/// Accessibility settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    /// High contrast mode
    pub high_contrast: bool,
    
    /// Large text mode
    pub large_text: bool,
    
    /// Reduced motion
    pub reduced_motion: bool,
    
    /// Screen reader enabled
    pub screen_reader: bool,
    
    /// Keyboard navigation only
    pub keyboard_only: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            high_contrast: false,
            large_text: false,
            reduced_motion: false,
            screen_reader: false,
            keyboard_only: false,
        }
    }
}

/// Adaptive UI Manager
pub struct AdaptiveUiManager {
    config: AdaptiveUiConfig,
    
    /// Available layouts
    layouts: Arc<RwLock<HashMap<String, UiLayout>>>,
    
    /// Available themes
    themes: Arc<RwLock<HashMap<String, UiTheme>>>,
    
    /// Current layout
    current_layout: Arc<RwLock<String>>,
    
    /// Current theme
    current_theme: Arc<RwLock<String>>,
    
    /// User profiles
    user_profiles: Arc<RwLock<HashMap<String, UserProfile>>>,
    
    /// Interaction history
    interaction_history: Arc<RwLock<VecDeque<InteractionEvent>>>,
    
    /// Behavior patterns
    behavior_patterns: Arc<RwLock<HashMap<String, BehaviorPattern>>>,
    
    /// Current user
    current_user: Arc<RwLock<Option<String>>>,
}

impl AdaptiveUiManager {
    pub fn new(config: AdaptiveUiConfig) -> Self {
        Self {
            config,
            layouts: Arc::new(RwLock::new(HashMap::new())),
            themes: Arc::new(RwLock::new(HashMap::new())),
            current_layout: Arc::new(RwLock::new(String::new())),
            current_theme: Arc::new(RwLock::new(String::new())),
            user_profiles: Arc::new(RwLock::new(HashMap::new())),
            interaction_history: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            behavior_patterns: Arc::new(RwLock::new(HashMap::new())),
            current_user: Arc::new(RwLock::new(None)),
        }
    }

    /// Register a UI layout
    pub async fn register_layout(&self, layout: UiLayout) {
        let mut layouts = self.layouts.write().await;
        layouts.insert(layout.layout_id.clone(), layout);
    }

    /// Register a UI theme
    pub async fn register_theme(&self, theme: UiTheme) {
        let mut themes = self.themes.write().await;
        themes.insert(theme.theme_id.clone(), theme);
    }

    /// Set current user
    pub async fn set_user(&self, user_id: String) {
        let mut current_user = self.current_user.write().await;
        *current_user = Some(user_id.clone());
        
        // Load user profile if exists
        let profiles = self.user_profiles.read().await;
        if let Some(profile) = profiles.get(&user_id) {
            if let Some(layout_id) = &profile.preferred_layout {
                let mut current_layout = self.current_layout.write().await;
                *current_layout = layout_id.clone();
            }
            
            if let Some(theme_id) = &profile.preferred_theme {
                let mut current_theme = self.current_theme.write().await;
                *current_theme = theme_id.clone();
            }
        }
    }

    /// Record user interaction
    pub async fn record_interaction(&self, event: InteractionEvent) {
        // Add to history
        {
            let mut history = self.interaction_history.write().await;
            history.push_back(event.clone());
            
            if history.len() > 1000 {
                history.pop_front();
            }
        }

        // Update behavior patterns if learning is enabled
        if self.config.enable_behavior_learning {
            self.update_behavior_patterns(&event).await;
        }

        // Trigger adaptation if enough data
        if self.config.enable_layout_adaptation {
            let history_len = self.interaction_history.read().await.len();
            if history_len >= self.config.min_samples_for_adaptation {
                if let Err(e) = self.adapt_layout().await {
                    log::error!("Layout adaptation failed: {}", e);
                }
            }
        }
    }

    /// Update behavior patterns from interaction
    async fn update_behavior_patterns(&self, event: &InteractionEvent) {
        let mut patterns = self.behavior_patterns.write().await;
        
        let pattern_key = format!("{:?}:{:?}:{}", 
            event.event_type, 
            event.component_id,
            event.position.map(|p| format!("{:.0},{:.0}", p.0, p.1)).unwrap_or_default()
        );

        let pattern = patterns.entry(pattern_key).or_insert(BehaviorPattern {
            pattern_type: format!("{:?}", event.event_type),
            frequency: 0.0,
            last_seen: Instant::now(),
            confidence: 0.0,
        });

        pattern.frequency += 1.0;
        pattern.last_seen = event.timestamp;
        pattern.confidence = (pattern.frequency / 10.0).min(1.0);
    }

    /// Adapt layout based on user behavior
    async fn adapt_layout(&self) -> Result<(), AdaptiveUiError> {
        let current_layout_id = self.current_layout.read().await.clone();
        if current_layout_id.is_empty() {
            return Err(AdaptiveUiError::InvalidLayout(
                "No current layout set".to_string()
            ));
        }

        let patterns = self.behavior_patterns.read().await;
        let mut layout = {
            let layouts = self.layouts.read().await;
            layouts.get(&current_layout_id)
                .cloned()
                .ok_or_else(|| AdaptiveUiError::InvalidLayout(
                    "Layout not found".to_string()
                ))?
        };

        // Analyze patterns and adjust component positions
        let mut component_adjustments: HashMap<String, (f64, f64)> = HashMap::new();

        for (key, pattern) in patterns.iter() {
            if pattern.confidence < self.config.adaptation_threshold {
                continue;
            }

            // Extract component ID from pattern key
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() >= 2 {
                let component_id = parts[1].to_string();
                
                // Calculate optimal position based on interaction frequency
                if let Some(event) = self.interaction_history.read().await
                    .iter()
                    .find(|e| e.component_id == component_id)
                {
                    if let Some(pos) = event.position {
                        let current_pos = layout.components
                            .iter()
                            .find(|c| c.component_id == component_id)
                            .map(|c| c.position);
                        
                        if let Some(current_pos) = current_pos {
                            // Move component closer to interaction position
                            let adjustment_factor = 0.1 * pattern.confidence;
                            let new_pos = (
                                current_pos.0 + (pos.0 - current_pos.0) * adjustment_factor,
                                current_pos.1 + (pos.1 - current_pos.1) * adjustment_factor,
                            );
                            component_adjustments.insert(component_id, new_pos);
                        }
                    }
                }
            }
        }

        // Apply adjustments
        for component in &mut layout.components {
            if let Some(new_pos) = component_adjustments.get(&component.component_id) {
                component.position = *new_pos;
            }
        }

        layout.adaptation_score = layout.adaptation_score * 0.9 + 0.1;

        // Save adapted layout
        {
            let mut layouts = self.layouts.write().await;
            layouts.insert(layout.layout_id.clone(), layout);
        }

        Ok(())
    }

    /// Adapt theme based on user preferences
    pub async fn adapt_theme(&self) -> Result<(), AdaptiveUiError> {
        if !self.config.enable_theme_adaptation {
            return Ok(());
        }

        let user_id = {
            let current_user = self.current_user.read().await;
            current_user.clone().ok_or(AdaptiveUiError::UserProfileNotFound)?
        };

        let profiles = self.user_profiles.read().await;
        let profile = profiles.get(&user_id)
            .ok_or(AdaptiveUiError::UserProfileNotFound)?;

        let accessibility = &profile.accessibility_settings;

        // Create adapted theme based on accessibility settings
        let current_theme_id = self.current_theme.read().await.clone();
        let base_theme = {
            let themes = self.themes.read().await;
            themes.get(&current_theme_id)
                .cloned()
                .ok_or_else(|| AdaptiveUiError::ThemeNotFound(current_theme_id))?
        };

        let mut adapted_theme = base_theme.clone();
        
        if accessibility.high_contrast {
            adapted_theme.primary_color = "#000000".to_string();
            adapted_theme.secondary_color = "#FFFFFF".to_string();
            adapted_theme.background_color = "#FFFFFF".to_string();
            adapted_theme.text_color = "#000000".to_string();
        }

        if accessibility.large_text {
            adapted_theme.font_size = (adapted_theme.font_size as f64 * 1.2) as u32;
        }

        if accessibility.dark_mode {
            adapted_theme.background_color = "#1a1a1a".to_string();
            adapted_theme.text_color = "#ffffff".to_string();
        }

        // Save adapted theme
        {
            let mut themes = self.themes.write().await;
            let adapted_theme_id = format!("{}_adapted", base_theme.theme_id);
            themes.insert(adapted_theme_id, adapted_theme);
        }

        Ok(())
    }

    /// Get current layout
    pub async fn get_current_layout(&self) -> Option<UiLayout> {
        let current_layout_id = self.current_layout.read().await.clone();
        let layouts = self.layouts.read().await;
        layouts.get(&current_layout_id).cloned()
    }

    /// Get current theme
    pub async fn get_current_theme(&self) -> Option<UiTheme> {
        let current_theme_id = self.current_theme.read().await.clone();
        let themes = self.themes.read().await;
        themes.get(&current_theme_id).cloned()
    }

    /// Get user profile
    pub async fn get_user_profile(&self, user_id: &str) -> Option<UserProfile> {
        let profiles = self.user_profiles.read().await;
        profiles.get(user_id).cloned()
    }

    /// Update user profile
    pub async fn update_user_profile(&self, user_id: String, profile: UserProfile) {
        let mut profiles = self.user_profiles.write().await;
        profiles.insert(user_id.clone(), profile);
    }

    /// Get behavior patterns
    pub async fn get_behavior_patterns(&self) -> HashMap<String, BehaviorPattern> {
        let patterns = self.behavior_patterns.read().await;
        patterns.clone()
    }

    /// Get interaction statistics
    pub async fn get_interaction_stats(&self) -> InteractionStats {
        let history = self.interaction_history.read().await;
        
        let total_interactions = history.len();
        let mut event_counts: HashMap<InteractionType, u64> = HashMap::new();
        
        for event in history.iter() {
            *event_counts.entry(event.event_type).or_insert(0) += 1;
        }

        InteractionStats {
            total_interactions,
            event_counts,
            most_active_component: self.find_most_active_component().await,
            average_interaction_duration: self.calculate_average_duration().await,
        }
    }

    async fn find_most_active_component(&self) -> Option<String> {
        let history = self.interaction_history.read().await;
        let mut component_counts: HashMap<String, u64> = HashMap::new();
        
        for event in history.iter() {
            *component_counts.entry(event.component_id.clone()).or_insert(0) += 1;
        }

        component_counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(component, _)| component)
    }

    async fn calculate_average_duration(&self) -> Option<Duration> {
        let history = self.interaction_history.read().await;
        let durations: Vec<_> = history.iter()
            .filter_map(|e| e.duration)
            .collect();

        if durations.is_empty() {
            return None;
        }

        let total_ms: u64 = durations.iter()
            .map(|d| d.as_millis() as u64)
            .sum();

        Some(Duration::from_millis(total_ms / durations.len() as u64))
    }
}

/// Interaction statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionStats {
    pub total_interactions: usize,
    pub event_counts: HashMap<InteractionType, u64>,
    pub most_active_component: Option<String>,
    pub average_interaction_duration: Option<Duration>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_ui_config() {
        let config = AdaptiveUiConfig {
            enable_layout_adaptation: true,
            enable_theme_adaptation: true,
            enable_behavior_learning: true,
            learning_rate: 0.1,
            min_samples_for_adaptation: 10,
            adaptation_threshold: 0.7,
            enable_accessibility: true,
        };

        assert!(config.enable_layout_adaptation);
        assert_eq!(config.min_samples_for_adaptation, 10);
    }

    #[test]
    fn test_ui_component_creation() {
        let component = UiComponent {
            component_id: "button1".to_string(),
            component_type: ComponentType::Button,
            position: (100.0, 100.0),
            size: (200.0, 50.0),
            visible: true,
            z_index: 1,
            properties: HashMap::new(),
        };

        assert_eq!(component.component_id, "button1");
        assert_eq!(component.component_type, ComponentType::Button);
    }

    #[tokio::test]
    async fn test_adaptive_ui_manager_creation() {
        let config = AdaptiveUiConfig {
            enable_layout_adaptation: true,
            enable_theme_adaptation: true,
            enable_behavior_learning: true,
            learning_rate: 0.1,
            min_samples_for_adaptation: 10,
            adaptation_threshold: 0.7,
            enable_accessibility: true,
        };

        let manager = AdaptiveUiManager::new(config);
        
        let layout = UiLayout {
            layout_id: "default".to_string(),
            name: "Default Layout".to_string(),
            components: vec![],
            layout_type: LayoutType::Grid,
            adaptation_score: 0.0,
        };

        manager.register_layout(layout).await;
        
        let current_layout = manager.get_current_layout().await;
        assert!(current_layout.is_some());
    }

    #[tokio::test]
    async fn test_user_interaction() {
        let config = AdaptiveUiConfig {
            enable_layout_adaptation: true,
            enable_theme_adaptation: true,
            enable_behavior_learning: true,
            learning_rate: 0.1,
            min_samples_for_adaptation: 10,
            adaptation_threshold: 0.7,
            enable_accessibility: true,
        };

        let manager = AdaptiveUiManager::new(config);
        
        let event = InteractionEvent {
            event_id: "1".to_string(),
            event_type: InteractionType::Click,
            component_id: "button1".to_string(),
            timestamp: Instant::now(),
            duration: Some(Duration::from_millis(100)),
            position: Some((100.0, 100.0)),
            data: None,
        };

        manager.record_interaction(event).await;
        
        let stats = manager.get_interaction_stats().await;
        assert_eq!(stats.total_interactions, 1);
    }
}