// Haptic Language - Accessibility Feature Implementation
// VantisOS Haptic Communication System

use std::collections::HashMap;
use std::time::{Duration, Instant};

// ============================================================================
// Haptic Patterns
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternCategory {
    Notification,
    Status,
    Navigation,
    Input,
    Custom,
}

#[derive(Debug, Clone)]
pub struct Vibration {
    pub duration: Duration,
    pub intensity: f32,
    pub pause_after: Duration,
}

impl Vibration {
    pub fn new(duration: Duration, intensity: f32, pause_after: Duration) -> Self {
        Vibration {
            duration,
            intensity,
            pause_after,
        }
    }

    pub fn short(intensity: f32) -> Self {
        Vibration::new(Duration::from_millis(50), intensity, Duration::from_millis(50))
    }

    pub fn medium(intensity: f32) -> Self {
        Vibration::new(Duration::from_millis(100), intensity, Duration::from_millis(100))
    }

    pub fn long(intensity: f32) -> Self {
        Vibration::new(Duration::from_millis(200), intensity, Duration::from_millis(100))
    }
}

#[derive(Debug, Clone)]
pub struct HapticPattern {
    pub id: String,
    pub name: String,
    pub category: PatternCategory,
    pub vibrations: Vec<Vibration>,
    pub repeat: bool,
    pub repeat_count: u32,
    pub repeat_interval: Duration,
}

impl HapticPattern {
    pub fn new(id: String, name: String, category: PatternCategory) -> Self {
        HapticPattern {
            id,
            name,
            category,
            vibrations: Vec::new(),
            repeat: false,
            repeat_count: 0,
            repeat_interval: Duration::from_millis(500),
        }
    }

    pub fn add_vibration(&mut self, vibration: Vibration) {
        self.vibrations.push(vibration);
    }

    pub fn set_repeat(&mut self, repeat: bool, count: u32, interval: Duration) {
        self.repeat = repeat;
        self.repeat_count = count;
        self.repeat_interval = interval;
    }

    pub fn total_duration(&self) -> Duration {
        let pattern_duration: Duration = self.vibrations.iter()
            .map(|v| v.duration + v.pause_after)
            .sum();

        if self.repeat {
            pattern_duration * self.repeat_count as u32 + 
            self.repeat_interval * (self.repeat_count - 1) as u32
        } else {
            pattern_duration
        }
    }
}

// ============================================================================
// Tactile Communication
// ============================================================================

#[derive(Debug, Clone)]
pub struct CommunicationContext {
    pub current_location: String,
    pub current_action: String,
    pub current_object: String,
}

impl CommunicationContext {
    pub fn new() -> Self {
        CommunicationContext {
            current_location: String::new(),
            current_action: String::new(),
            current_object: String::new(),
        }
    }

    pub fn set_location(&mut self, location: String) {
        self.current_location = location;
    }

    pub fn set_action(&mut self, action: String) {
        self.current_action = action;
    }

    pub fn set_object(&mut self, object: String) {
        self.current_object = object;
    }
}

#[derive(Debug, Clone)]
pub struct TactileCommunication {
    pub patterns: HashMap<String, HapticPattern>,
    pub context: CommunicationContext,
    pub enabled: bool,
}

impl TactileCommunication {
    pub fn new() -> Self {
        let mut comm = TactileCommunication {
            patterns: HashMap::new(),
            context: CommunicationContext::new(),
            enabled: true,
        };

        comm.initialize_default_patterns();
        comm
    }

    fn initialize_default_patterns(&mut self) {
        // Simple communication patterns
        let yes = HapticPattern::new(
            String::from("yes"),
            String::from("Yes"),
            PatternCategory::Custom,
        );
        yes.add_vibration(Vibration::medium(0.8));
        self.patterns.insert(String::from("yes"), yes);

        let no = HapticPattern::new(
            String::from("no"),
            String::from("No"),
            PatternCategory::Custom,
        );
        no.add_vibration(Vibration::short(0.8));
        no.add_vibration(Vibration::short(0.8));
        self.patterns.insert(String::from("no"), no);

        let confirm = HapticPattern::new(
            String::from("confirm"),
            String::from("Confirm"),
            PatternCategory::Custom,
        );
        confirm.add_vibration(Vibration::long(1.0));
        self.patterns.insert(String::from("confirm"), confirm);

        let cancel = HapticPattern::new(
            String::from("cancel"),
            String::from("Cancel"),
            PatternCategory::Custom,
        );
        cancel.add_vibration(Vibration::short(0.3));
        self.patterns.insert(String::from("cancel"), cancel);
    }

    pub fn add_pattern(&mut self, pattern: HapticPattern) {
        self.patterns.insert(pattern.id.clone(), pattern);
    }

    pub fn get_pattern(&self, pattern_id: &str) -> Option<&HapticPattern> {
        self.patterns.get(pattern_id)
    }

    pub fn encode_message(&self, message: &str) -> Option<Vec<HapticPattern>> {
        // Simple message encoding
        match message.to_lowercase().as_str() {
            "yes" => self.patterns.get("yes").cloned().map(|p| vec![p]),
            "no" => self.patterns.get("no").cloned().map(|p| vec![p]),
            "confirm" => self.patterns.get("confirm").cloned().map(|p| vec![p]),
            "cancel" => self.patterns.get("cancel").cloned().map(|p| vec![p]),
            _ => None,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

// ============================================================================
// Notification System
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationPriority {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NotificationType {
    System,
    Application,
    Communication,
    Calendar,
    Custom,
}

#[derive(Debug, Clone)]
pub struct HapticNotification {
    pub id: String,
    pub notification_type: NotificationType,
    pub priority: NotificationPriority,
    pub pattern: HapticPattern,
    pub timestamp: Instant,
}

impl HapticNotification {
    pub fn new(
        id: String,
        notification_type: NotificationType,
        priority: NotificationPriority,
        pattern: HapticPattern,
    ) -> Self {
        HapticNotification {
            id,
            notification_type,
            priority,
            pattern,
            timestamp: Instant::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PriorityRules {
    pub critical_intensity: f32,
    pub high_intensity: f32,
    pub medium_intensity: f32,
    pub low_intensity: f32,
    pub informational_intensity: f32,
}

impl PriorityRules {
    pub fn new() -> Self {
        PriorityRules {
            critical_intensity: 1.0,
            high_intensity: 0.8,
            medium_intensity: 0.6,
            low_intensity: 0.4,
            informational_intensity: 0.2,
        }
    }

    pub fn get_intensity(&self, priority: NotificationPriority) -> f32 {
        match priority {
            NotificationPriority::Critical => self.critical_intensity,
            NotificationPriority::High => self.high_intensity,
            NotificationPriority::Medium => self.medium_intensity,
            NotificationPriority::Low => self.low_intensity,
            NotificationPriority::Informational => self.informational_intensity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NotificationSystem {
    pub notifications: Vec<HapticNotification>,
    pub priority_rules: PriorityRules,
    pub do_not_disturb: bool,
}

impl NotificationSystem {
    pub fn new() -> Self {
        let mut system = NotificationSystem {
            notifications: Vec::new(),
            priority_rules: PriorityRules::new(),
            do_not_disturb: false,
        };

        system.initialize_default_notifications();
        system
    }

    fn initialize_default_notifications(&mut self) {
        // System notifications
        let system_startup = HapticPattern::new(
            String::from("system_startup"),
            String::from("System Startup"),
            PatternCategory::Notification,
        );
        system_startup.add_vibration(Vibration::short(0.5));
        system_startup.add_vibration(Vibration::medium(0.7));
        system_startup.add_vibration(Vibration::long(0.9));

        let system_error = HapticPattern::new(
            String::from("system_error"),
            String::from("System Error"),
            PatternCategory::Status,
        );
        system_error.add_vibration(Vibration::short(1.0));
        system_error.add_vibration(Vibration::short(1.0));
        system_error.add_vibration(Vibration::short(1.0));

        // Communication notifications
        let new_message = HapticPattern::new(
            String::from("new_message"),
            String::from("New Message"),
            PatternCategory::Notification,
        );
        new_message.add_vibration(Vibration::short(0.7));
        new_message.add_vibration(Vibration::short(0.7));
        new_message.add_vibration(Vibration::long(0.9));

        let new_call = HapticPattern::new(
            String::from("new_call"),
            String::from("New Call"),
            PatternCategory::Notification,
        );
        new_call.add_vibration(Vibration::long(0.9));
        new_call.add_vibration(Vibration::long(0.9));
        new_call.add_vibration(Vibration::long(0.9));
        new_call.set_repeat(true, 3, Duration::from_millis(500));
    }

    pub fn add_notification(&mut self, notification: HapticNotification) {
        if !self.do_not_disturb || notification.priority == NotificationPriority::Critical {
            self.notifications.push(notification);
        }
    }

    pub fn get_next_notification(&mut self) -> Option<HapticNotification> {
        if self.notifications.is_empty() {
            return None;
        }

        // Sort by priority and timestamp
        self.notifications.sort_by(|a, b| {
            b.priority.cmp(&a.priority)
                .then_with(|| a.timestamp.cmp(&b.timestamp))
        });

        self.notifications.remove(0)
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub fn set_do_not_disturb(&mut self, enabled: bool) {
        self.do_not_disturb = enabled;
    }

    pub fn notification_count(&self) -> usize {
        self.notifications.len()
    }
}

// ============================================================================
// Spatial Haptics
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceType {
    Object,
    NavigationPoint,
    Boundary,
    Target,
}

#[derive(Debug, Clone)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position3D { x, y, z }
    }

    pub fn distance_to(&self, other: &Position3D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug, Clone)]
pub struct FeedbackSource {
    pub id: String,
    pub source_type: SourceType,
    pub position: Position3D,
    pub intensity: f32,
}

impl FeedbackSource {
    pub fn new(id: String, source_type: SourceType, position: Position3D, intensity: f32) -> Self {
        FeedbackSource {
            id,
            source_type,
            position,
            intensity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpatialMapping {
    pub user_position: Position3D,
    pub orientation: f32, // in radians
    pub max_distance: f32,
}

impl SpatialMapping {
    pub fn new() -> Self {
        SpatialMapping {
            user_position: Position3D::new(0.0, 0.0, 0.0),
            orientation: 0.0,
            max_distance: 10.0,
        }
    }

    pub fn set_user_position(&mut self, position: Position3D) {
        self.user_position = position;
    }

    pub fn set_orientation(&mut self, orientation: f32) {
        self.orientation = orientation;
    }

    pub fn calculate_direction(&self, target: &Position3D) -> f32 {
        let dx = target.x - self.user_position.x;
        let dy = target.y - self.user_position.y;
        (dy.atan2(dx) - self.orientation + std::f32::consts::PI * 2.0) % (std::f32::consts::PI * 2.0)
    }

    pub fn calculate_distance(&self, target: &Position3D) -> f32 {
        self.user_position.distance_to(target)
    }

    pub fn calculate_intensity(&self, target: &Position3D) -> f32 {
        let distance = self.calculate_distance(target);
        if distance >= self.max_distance {
            0.0
        } else {
            1.0 - (distance / self.max_distance)
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpatialHaptics {
    pub enabled: bool,
    pub feedback_sources: Vec<FeedbackSource>,
    pub spatial_mapping: SpatialMapping,
}

impl SpatialHaptics {
    pub fn new() -> Self {
        SpatialHaptics {
            enabled: true,
            feedback_sources: Vec::new(),
            spatial_mapping: SpatialMapping::new(),
        }
    }

    pub fn add_feedback_source(&mut self, source: FeedbackSource) {
        self.feedback_sources.push(source);
    }

    pub fn remove_feedback_source(&mut self, source_id: &str) {
        self.feedback_sources.retain(|s| s.id != source_id);
    }

    pub fn get_spatial_feedback(&self) -> Option<(f32, f32)> {
        if !self.enabled || self.feedback_sources.is_empty() {
            return None;
        }

        // Find the closest source
        let closest = self.feedback_sources.iter()
            .min_by(|a, b| {
                let dist_a = self.spatial_mapping.calculate_distance(&a.position);
                let dist_b = self.spatial_mapping.calculate_distance(&b.position);
                dist_a.partial_cmp(&dist_b).unwrap()
            });

        if let Some(source) = closest {
            let direction = self.spatial_mapping.calculate_direction(&source.position);
            let intensity = self.spatial_mapping.calculate_intensity(&source.position) * source.intensity;
            Some((direction, intensity))
        } else {
            None
        }
    }

    pub fn clear_feedback_sources(&mut self) {
        self.feedback_sources.clear();
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

// ============================================================================
// Intensity Controller
// ============================================================================

#[derive(Debug, Clone)]
pub struct IntensityProfile {
    pub name: String,
    pub global_intensity: f32,
    pub category_intensities: HashMap<PatternCategory, f32>,
}

impl IntensityProfile {
    pub fn new(name: String, global_intensity: f32) -> Self {
        IntensityProfile {
            name,
            global_intensity,
            category_intensities: HashMap::new(),
        }
    }

    pub fn set_category_intensity(&mut self, category: PatternCategory, intensity: f32) {
        self.category_intensities.insert(category, intensity);
    }
}

#[derive(Debug, Clone)]
pub struct IntensityController {
    pub global_intensity: f32,
    pub category_intensities: HashMap<PatternCategory, f32>,
    pub pattern_intensities: HashMap<String, f32>,
    pub profiles: Vec<IntensityProfile>,
    pub active_profile: Option<String>,
}

impl IntensityController {
    pub fn new() -> Self {
        let mut controller = IntensityController {
            global_intensity: 0.5,
            category_intensities: HashMap::new(),
            pattern_intensities: HashMap::new(),
            profiles: Vec::new(),
            active_profile: None,
        };

        controller.initialize_default_profiles();
        controller
    }

    fn initialize_default_profiles(&mut self) {
        let gentle = IntensityProfile::new(String::from("Gentle"), 0.25);
        self.profiles.push(gentle);

        let normal = IntensityProfile::new(String::from("Normal"), 0.5);
        self.profiles.push(normal);

        let strong = IntensityProfile::new(String::from("Strong"), 0.75);
        self.profiles.push(strong);
    }

    pub fn set_global_intensity(&mut self, intensity: f32) {
        self.global_intensity = intensity.clamp(0.0, 1.0);
    }

    pub fn set_category_intensity(&mut self, category: PatternCategory, intensity: f32) {
        self.category_intensities.insert(category, intensity.clamp(0.0, 1.0));
    }

    pub fn set_pattern_intensity(&mut self, pattern_id: String, intensity: f32) {
        self.pattern_intensities.insert(pattern_id, intensity.clamp(0.0, 1.0));
    }

    pub fn get_effective_intensity(&self, pattern_id: &str, category: &PatternCategory) -> f32 {
        let pattern_intensity = self.pattern_intensities.get(pattern_id);
        let category_intensity = self.category_intensities.get(category);
        
        let intensity = pattern_intensity
            .or(category_intensity)
            .unwrap_or(&self.global_intensity);

        intensity.min(&self.global_intensity).to_owned()
    }

    pub fn add_profile(&mut self, profile: IntensityProfile) {
        self.profiles.push(profile);
    }

    pub fn set_active_profile(&mut self, profile_name: &str) -> Result<(), String> {
        if let Some(profile) = self.profiles.iter().find(|p| p.name == profile_name) {
            self.global_intensity = profile.global_intensity;
            self.category_intensities = profile.category_intensities.clone();
            self.active_profile = Some(profile_name.to_string());
            Ok(())
        } else {
            Err(format!("Profile '{}' not found", profile_name))
        }
    }

    pub fn get_active_profile(&self) -> Option<&String> {
        self.active_profile.as_ref()
    }
}

// ============================================================================
// Haptic Themes
// ============================================================================

#[derive(Debug, Clone)]
pub struct CategorySettings {
    pub enabled: bool,
    pub intensity_multiplier: f32,
    pub duration_multiplier: f32,
}

impl CategorySettings {
    pub fn new() -> Self {
        CategorySettings {
            enabled: true,
            intensity_multiplier: 1.0,
            duration_multiplier: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HapticTheme {
    pub id: String,
    pub name: String,
    pub description: String,
    pub patterns: HashMap<String, HapticPattern>,
    pub intensity_profile: IntensityProfile,
    pub category_settings: HashMap<PatternCategory, CategorySettings>,
}

impl HapticTheme {
    pub fn new(id: String, name: String, description: String) -> Self {
        HapticTheme {
            id,
            name,
            description,
            patterns: HashMap::new(),
            intensity_profile: IntensityProfile::new(name.clone(), 0.5),
            category_settings: HashMap::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: HapticPattern) {
        self.patterns.insert(pattern.id.clone(), pattern);
    }

    pub fn set_category_settings(&mut self, category: PatternCategory, settings: CategorySettings) {
        self.category_settings.insert(category, settings);
    }
}

#[derive(Debug, Clone)]
pub struct ThemeManager {
    pub themes: Vec<HapticTheme>,
    pub active_theme: Option<String>,
    pub custom_themes: Vec<HapticTheme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = ThemeManager {
            themes: Vec::new(),
            active_theme: None,
            custom_themes: Vec::new(),
        };

        manager.initialize_default_themes();
        manager
    }

    fn initialize_default_themes(&mut self) {
        let default = HapticTheme::new(
            String::from("default"),
            String::from("Default"),
            String::from("Balanced intensity and standard patterns"),
        );
        self.themes.push(default);

        let gentle = HapticTheme::new(
            String::from("gentle"),
            String::from("Gentle"),
            String::from("Low intensity and soft patterns"),
        );
        gentle.intensity_profile.global_intensity = 0.25;
        self.themes.push(gentle);

        let strong = HapticTheme::new(
            String::from("strong"),
            String::from("Strong"),
            String::from("High intensity and strong patterns"),
        );
        strong.intensity_profile.global_intensity = 0.75;
        self.themes.push(strong);
    }

    pub fn add_theme(&mut self, theme: HapticTheme) {
        self.themes.push(theme);
    }

    pub fn add_custom_theme(&mut self, theme: HapticTheme) {
        self.custom_themes.push(theme);
    }

    pub fn get_theme(&self, theme_id: &str) -> Option<&HapticTheme> {
        self.themes.iter()
            .chain(self.custom_themes.iter())
            .find(|t| t.id == theme_id)
    }

    pub fn set_active_theme(&mut self, theme_id: &str) -> Result<(), String> {
        if self.get_theme(theme_id).is_some() {
            self.active_theme = Some(theme_id.to_string());
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_id))
        }
    }

    pub fn get_active_theme(&self) -> Option<&HapticTheme> {
        if let Some(ref theme_id) = self.active_theme {
            self.get_theme(theme_id)
        } else {
            self.themes.first()
        }
    }
}

// ============================================================================
// Haptic Engine
// ============================================================================

#[derive(Debug, Clone)]
pub struct HapticEngine {
    pub tactile_communication: TactileCommunication,
    pub notification_system: NotificationSystem,
    pub spatial_haptics: SpatialHaptics,
    pub intensity_controller: IntensityController,
    pub theme_manager: ThemeManager,
    pub enabled: bool,
    pub playing: bool,
}

impl HapticEngine {
    pub fn new() -> Self {
        HapticEngine {
            tactile_communication: TactileCommunication::new(),
            notification_system: NotificationSystem::new(),
            spatial_haptics: SpatialHaptics::new(),
            intensity_controller: IntensityController::new(),
            theme_manager: ThemeManager::new(),
            enabled: true,
            playing: false,
        }
    }

    pub fn play_pattern(&mut self, pattern_id: &str) -> Result<(), String> {
        if !self.enabled {
            return Err("Haptic engine is disabled".to_string());
        }

        // Get pattern from communication or theme
        let pattern = self.tactile_communication.get_pattern(pattern_id)
            .or_else(|| {
                self.theme_manager.get_active_theme()
                    .and_then(|theme| theme.patterns.get(pattern_id))
            });

        if let Some(pattern) = pattern {
            self.play_haptic_pattern(pattern)?;
            Ok(())
        } else {
            Err(format!("Pattern '{}' not found", pattern_id))
        }
    }

    fn play_haptic_pattern(&self, pattern: &HapticPattern) -> Result<(), String> {
        // Apply intensity
        let intensity = self.intensity_controller.get_effective_intensity(
            &pattern.id,
            &pattern.category,
        );

        // Apply theme settings
        let effective_pattern = self.apply_theme_settings(pattern);

        // Play vibrations
        for vibration in &effective_pattern.vibrations {
            let effective_intensity = vibration.intensity * intensity;
            self.vibrate(vibration.duration, effective_intensity);
            std::thread::sleep(vibration.pause_after);
        }

        Ok(())
    }

    fn apply_theme_settings(&self, pattern: &HapticPattern) -> HapticPattern {
        if let Some(theme) = self.theme_manager.get_active_theme() {
            let mut effective_pattern = pattern.clone();

            if let Some(settings) = theme.category_settings.get(&pattern.category) {
                if !settings.enabled {
                    effective_pattern.vibrations.clear();
                } else {
                    for vibration in &mut effective_pattern.vibrations {
                        vibration.intensity *= settings.intensity_multiplier;
                        vibration.duration = vibration.duration.mul_f32(settings.duration_multiplier);
                    }
                }
            }

            effective_pattern
        } else {
            pattern.clone()
        }
    }

    fn vibrate(&self, duration: Duration, intensity: f32) {
        // Implementation would control vibration motor
        // This is a placeholder
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }

    pub fn play_notification(&mut self) -> Result<(), String> {
        if let Some(notification) = self.notification_system.get_next_notification() {
            self.play_haptic_pattern(&notification.pattern)?;
        }
        Ok(())
    }

    pub fn play_spatial_feedback(&self) -> Result<(), String> {
        if let Some((direction, intensity)) = self.spatial_haptics.get_spatial_feedback() {
            // Convert direction to vibration pattern
            let pattern = self.direction_to_pattern(direction, intensity);
            self.play_haptic_pattern(&pattern)?;
        }
        Ok(())
    }

    fn direction_to_pattern(&self, direction: f32, intensity: f32) -> HapticPattern {
        let mut pattern = HapticPattern::new(
            String::from("spatial"),
            String::from("Spatial Feedback"),
            PatternCategory::Custom,
        );

        // Simple directional feedback
        if direction < std::f32::consts::PI / 4.0 || direction > 7.0 * std::f32::consts::PI / 4.0 {
            // Right
            pattern.add_vibration(Vibration::medium(intensity));
        } else if direction < 3.0 * std::f32::consts::PI / 4.0 {
            // Down
            pattern.add_vibration(Vibration::short(intensity));
            pattern.add_vibration(Vibration::short(intensity));
        } else if direction < 5.0 * std::f32::consts::PI / 4.0 {
            // Left
            pattern.add_vibration(Vibration::long(intensity));
        } else {
            // Up
            pattern.add_vibration(Vibration::short(intensity));
            pattern.add_vibration(Vibration::medium(intensity));
        }

        pattern
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
        self.stop();
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn get_status(&self) -> HapticStatus {
        HapticStatus {
            enabled: self.enabled,
            playing: self.playing,
            active_theme: self.theme_manager.active_theme.clone(),
            global_intensity: self.intensity_controller.global_intensity,
            notification_count: self.notification_system.notification_count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HapticStatus {
    pub enabled: bool,
    pub playing: bool,
    pub active_theme: Option<String>,
    pub global_intensity: f32,
    pub notification_count: usize,
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibration() {
        let vibration = Vibration::short(0.5);
        assert_eq!(vibration.duration, Duration::from_millis(50));
        assert_eq!(vibration.intensity, 0.5);
    }

    #[test]
    fn test_haptic_pattern() {
        let mut pattern = HapticPattern::new(
            String::from("test"),
            String::from("Test Pattern"),
            PatternCategory::Custom,
        );
        pattern.add_vibration(Vibration::short(0.5));
        pattern.add_vibration(Vibration::medium(0.7));
        assert_eq!(pattern.vibrations.len(), 2);
    }

    #[test]
    fn test_tactile_communication() {
        let comm = TactileCommunication::new();
        assert!(comm.enabled);
        assert!(comm.get_pattern("yes").is_some());
    }

    #[test]
    fn test_notification_system() {
        let system = NotificationSystem::new();
        assert_eq!(system.notification_count(), 0);
        assert!(!system.do_not_disturb);
    }

    #[test]
    fn test_spatial_haptics() {
        let mut spatial = SpatialHaptics::new();
        let source = FeedbackSource::new(
            String::from("test"),
            SourceType::Object,
            Position3D::new(1.0, 0.0, 0.0),
            0.8,
        );
        spatial.add_feedback_source(source);
        assert_eq!(spatial.feedback_sources.len(), 1);
    }

    #[test]
    fn test_intensity_controller() {
        let controller = IntensityController::new();
        assert_eq!(controller.global_intensity, 0.5);
        controller.set_global_intensity(0.8);
        assert_eq!(controller.global_intensity, 0.8);
    }

    #[test]
    fn test_theme_manager() {
        let manager = ThemeManager::new();
        assert!(manager.get_theme("default").is_some());
        assert!(manager.set_active_theme("gentle").is_ok());
    }

    #[test]
    fn test_haptic_engine() {
        let engine = HapticEngine::new();
        assert!(engine.enabled);
        assert!(!engine.playing);
    }
}