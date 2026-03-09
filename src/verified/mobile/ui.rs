//! Mobile UI Framework Module
//! 
//! This module provides a mobile-optimized UI framework for VantisOS
//! including responsive layouts, touch-friendly controls, and animations.

use alloc::sync::Arc;
use spin::Mutex;

/// Screen orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenOrientation {
    Portrait,
    Landscape,
}

/// UI density
#[derive(Debug, Clone, Copy)]
pub enum UiDensity {
    Compact,
    Comfortable,
    Spacious,
}

/// Touch target size
#[derive(Debug, Clone, Copy)]
pub struct TouchTargetSize {
    pub width: u32,
    pub height: u32,
}

impl Default for TouchTargetSize {
    fn default() -> Self {
        Self {
            width: 48,
            height: 48,
        }
    }
}

/// Gesture type
#[derive(Debug, Clone, Copy)]
pub enum GestureType {
    Tap,
    DoubleTap,
    LongPress,
    SwipeLeft,
    SwipeRight,
    SwipeUp,
    SwipeDown,
    Pinch,
    Rotate,
}

/// Animation type
#[derive(Debug, Clone, Copy)]
pub enum AnimationType {
    Fade,
    Slide,
    Scale,
    Rotate,
    Bounce,
    Elastic,
}

/// Animation duration
#[derive(Debug, Clone, Copy)]
pub enum AnimationDuration {
    Short,
    Medium,
    Long,
}

impl AnimationDuration {
    /// Get duration in milliseconds
    pub fn ms(&self) -> u32 {
        match self {
            Self::Short => 150,
            Self::Medium => 300,
            Self::Long => 500,
        }
    }
}

/// UI component
#[derive(Debug, Clone)]
pub struct UiComponent {
    pub component_id: String,
    pub component_type: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub visible: bool,
    pub enabled: bool,
    pub touch_target_size: TouchTargetSize,
}

impl UiComponent {
    /// Create a new UI component
    pub fn new(
        component_id: impl Into<String>,
        component_type: impl Into<String>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            component_id: component_id.into(),
            component_type: component_type.into(),
            x,
            y,
            width,
            height,
            visible: true,
            enabled: true,
            touch_target_size: TouchTargetSize::default(),
        }
    }

    /// Check if point is within component
    pub fn contains_point(&self, x: u32, y: u32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set enabled state
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Set position
    pub fn set_position(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    /// Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

/// UI layout engine
pub struct UiLayoutEngine {
    orientation: Arc<Mutex<ScreenOrientation>>,
    density: Arc<Mutex<UiDensity>>,
    screen_width: Arc<Mutex<u32>>,
    screen_height: Arc<Mutex<u32>>,
}

impl UiLayoutEngine {
    /// Create a new UI layout engine
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            orientation: Arc::new(Mutex::new(ScreenOrientation::Portrait)),
            density: Arc::new(Mutex::new(UiDensity::Comfortable)),
            screen_width: Arc::new(Mutex::new(screen_width)),
            screen_height: Arc::new(Mutex::new(screen_height)),
        }
    }

    /// Set screen orientation
    pub fn set_orientation(&self, orientation: ScreenOrientation) {
        *self.orientation.lock() = orientation;
    }

    /// Set UI density
    pub fn set_density(&self, density: UiDensity) {
        *self.density.lock() = density;
    }

    /// Get screen orientation
    pub fn orientation(&self) -> ScreenOrientation {
        *self.orientation.lock()
    }

    /// Get UI density
    pub fn density(&self) -> UiDensity {
        *self.density.lock()
    }

    /// Get screen dimensions
    pub fn screen_dimensions(&self) -> (u32, u32) {
        (*self.screen_width.lock(), *self.screen_height.lock())
    }

    /// Calculate spacing based on density
    pub fn spacing(&self) -> u32 {
        match *self.density.lock() {
            UiDensity::Compact => 8,
            UiDensity::Comfortable => 16,
            UiDensity::Spacious => 24,
        }
    }

    /// Calculate padding based on density
    pub fn padding(&self) -> u32 {
        match *self.density.lock() {
            UiDensity::Compact => 12,
            UiDensity::Comfortable => 16,
            UiDensity::Spacious => 24,
        }
    }

    /// Calculate corner radius based on density
    pub fn corner_radius(&self) -> u32 {
        match *self.density.lock() {
            UiDensity::Compact => 4,
            UiDensity::Comfortable => 8,
            UiDensity::Spacious => 12,
        }
    }

    /// Update screen dimensions
    pub fn update_screen_dimensions(&self, width: u32, height: u32) {
        *self.screen_width.lock() = width;
        *self.screen_height.lock() = height;
    }
}

impl Default for UiLayoutEngine {
    fn default() -> Self {
        Self::new(375, 812) // iPhone X dimensions
    }
}

/// Animation manager
pub struct AnimationManager {
    active_animations: Arc<Mutex<alloc::vec::Vec<Animation>>>,
}

/// Animation
#[derive(Debug, Clone)]
struct Animation {
    animation_id: String,
    component_id: String,
    animation_type: AnimationType,
    duration_ms: u32,
    start_time: u64,
    completed: bool,
}

impl AnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self {
            active_animations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Start an animation
    pub fn start_animation(
        &self,
        component_id: impl Into<String>,
        animation_type: AnimationType,
        duration: AnimationDuration,
    ) {
        let animation = Animation {
            animation_id: alloc::format!("anim_{}", self.current_timestamp()),
            component_id: component_id.into(),
            animation_type,
            duration_ms: duration.ms(),
            start_time: self.current_timestamp(),
            completed: false,
        };

        self.active_animations.lock().push(animation);
    }

    /// Check if component has active animation
    pub fn has_active_animation(&self, component_id: &str) -> bool {
        self.active_animations
            .lock()
            .iter()
            .any(|a| !a.completed && a.component_id == component_id)
    }

    /// Cancel animation for component
    pub fn cancel_animation(&self, component_id: &str) {
        let mut animations = self.active_animations.lock();
        animations.retain(|a| a.component_id != component_id || a.completed);
    }

    /// Get current timestamp (placeholder)
    fn current_timestamp(&self) -> u64 {
        // In a real implementation, this would return actual time
        0
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global UI layout engine
static UI_LAYOUT_ENGINE: spin::Once<UiLayoutEngine> = spin::Once::new();

/// Get the global UI layout engine
pub fn ui_layout_engine() -> &'static UiLayoutEngine {
    UI_LAYOUT_ENGINE.call_once(|| UiLayoutEngine::default())
}

/// Global animation manager
static ANIMATION_MANAGER: spin::Once<AnimationManager> = spin::Once::new();

/// Get the global animation manager
pub fn animation_manager() -> &'static AnimationManager {
    ANIMATION_MANAGER.call_once(|| AnimationManager::default())
}

/// Set screen orientation
pub fn set_screen_orientation(orientation: ScreenOrientation) {
    ui_layout_engine().set_orientation(orientation)
}

/// Get screen dimensions
pub fn get_screen_dimensions() -> (u32, u32) {
    ui_layout_engine().screen_dimensions()
}

/// Update screen dimensions
pub fn update_screen_dimensions(width: u32, height: u32) {
    ui_layout_engine().update_screen_dimensions(width, height)
}

/// Calculate touch target size
pub fn touch_target_size() -> TouchTargetSize {
    TouchTargetSize::default()
}

/// Start UI animation
pub fn start_ui_animation(
    component_id: impl Into<String>,
    animation_type: AnimationType,
    duration: AnimationDuration,
) {
    animation_manager().start_animation(component_id, animation_type, duration)
}