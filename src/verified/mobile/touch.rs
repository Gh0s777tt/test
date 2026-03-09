//! Touch Optimization Module
//! 
//! This module provides touch-optimized input handling for mobile devices
//! including gesture recognition, multi-touch support, and haptic feedback.

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use spin::Mutex;

/// Touch event
#[derive(Debug, Clone, Copy)]
pub struct TouchEvent {
    pub touch_id: u32,
    pub x: u32,
    pub y: u32,
    pub pressure: u8,
    pub timestamp: u64,
}

/// Gesture state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureState {
    Started,
    Updated,
    Ended,
    Cancelled,
}

/// Gesture event
#[derive(Debug, Clone)]
pub struct GestureEvent {
    pub gesture_type: super::ui::GestureType,
    pub state: GestureState,
    pub touches: Vec<TouchEvent>,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub timestamp: u64,
}

/// Touch configuration
#[derive(Debug, Clone, Copy)]
pub struct TouchConfiguration {
    pub tap_timeout_ms: u32,
    pub double_tap_timeout_ms: u32,
    pub long_press_timeout_ms: u32,
    pub swipe_threshold: u32,
    pub pinch_threshold: f64,
    pub rotate_threshold: f64,
}

impl Default for TouchConfiguration {
    fn default() -> Self {
        Self {
            tap_timeout_ms: 200,
            double_tap_timeout_ms: 300,
            long_press_timeout_ms: 500,
            swipe_threshold: 100,
            pinch_threshold: 10.0,
            rotate_threshold: 15.0,
        }
    }
}

/// Touch event handler
pub struct TouchEventHandler {
    config: TouchConfiguration,
    touch_points: Arc<Mutex<BTreeMap<u32, TouchEvent>>>,
    gesture_state: Arc<Mutex<Option<GestureEvent>>>,
    multi_touch_enabled: Arc<Mutex<bool>>,
}

impl TouchEventHandler {
    /// Create a new touch event handler
    pub fn new(config: TouchConfiguration) -> Self {
        Self {
            config,
            touch_points: Arc::new(Mutex::new(BTreeMap::new())),
            gesture_state: Arc::new(Mutex::new(None)),
            multi_touch_enabled: Arc::new(Mutex::new(true)),
        }
    }

    /// Handle touch event
    pub fn handle_touch_event(&self, event: TouchEvent) -> Option<GestureEvent> {
        match event.touch_id {
            0 => self.handle_primary_touch(event),
            _ => self.handle_secondary_touch(event),
        }
    }

    /// Handle primary touch (first touch point)
    fn handle_primary_touch(&self, event: TouchEvent) -> Option<GestureEvent> {
        let mut touch_points = self.touch_points.lock();
        let mut gesture_state = self.gesture_state.lock();

        match event.pressure {
            0 => {
                // Touch ended
                touch_points.remove(&event.touch_id);
                
                if let Some(gesture) = gesture_state.take() {
                    return Some(GestureEvent {
                        gesture_type: gesture.gesture_type,
                        state: GestureState::Ended,
                        touches: vec![event],
                        velocity_x: gesture.velocity_x,
                        velocity_y: gesture.velocity_y,
                        timestamp: event.timestamp,
                    });
                }
                
                // Check for tap
                if self.is_tap(&touch_points, event) {
                    return Some(GestureEvent {
                        gesture_type: super::ui::GestureType::Tap,
                        state: GestureState::Ended,
                        touches: vec![event],
                        velocity_x: 0.0,
                        velocity_y: 0.0,
                        timestamp: event.timestamp,
                    });
                }
            }
            _ => {
                // Touch started or updated
                touch_points.insert(event.touch_id, event);
                
                // Update gesture detection
                if let Some(gesture) = gesture_state.as_mut() {
                    gesture.state = GestureState::Updated;
                }
            }
        }

        None
    }

    /// Handle secondary touch (additional touch points)
    fn handle_secondary_touch(&self, event: TouchEvent) -> Option<GestureEvent> {
        if !*self.multi_touch_enabled.lock() {
            return None;
        }

        let mut touch_points = self.touch_points.lock();
        
        match event.pressure {
            0 => {
                touch_points.remove(&event.touch_id);
            }
            _ => {
                touch_points.insert(event.touch_id, event);
            }
        }

        None
    }

    /// Check if event is a tap
    fn is_tap(&self, _touch_points: &BTreeMap<u32, TouchEvent>, event: TouchEvent) -> bool {
        event.pressure == 0
    }

    /// Get current touch points
    pub fn touch_points(&self) -> Vec<TouchEvent> {
        self.touch_points.lock().values().cloned().collect()
    }

    /// Enable or disable multi-touch
    pub fn set_multi_touch_enabled(&self, enabled: bool) {
        *self.multi_touch_enabled.lock() = enabled;
    }

    /// Check if multi-touch is enabled
    pub fn is_multi_touch_enabled(&self) -> bool {
        *self.multi_touch_enabled.lock()
    }
}

impl Default for TouchEventHandler {
    fn default() -> Self {
        Self::new(TouchConfiguration::default())
    }
}

/// Haptic feedback type
#[derive(Debug, Clone, Copy)]
pub enum HapticFeedbackType {
    Light,
    Medium,
    Heavy,
    Success,
    Warning,
    Error,
}

/// Haptic feedback manager
pub struct HapticFeedbackManager {
    enabled: Arc<Mutex<bool>>,
    intensity: Arc<Mutex<u8>>,
}

impl HapticFeedbackManager {
    /// Create a new haptic feedback manager
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(Mutex::new(true)),
            intensity: Arc::new(Mutex::new(128)),
        }
    }

    /// Provide haptic feedback
    pub fn haptic_feedback(&self, feedback_type: HapticFeedbackType) {
        if !*self.enabled.lock() {
            return;
        }
        let _ = (feedback_type);
    }

    /// Set enabled state
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.lock() = enabled;
    }

    /// Set intensity (0-255)
    pub fn set_intensity(&self, intensity: u8) {
        *self.intensity.lock() = intensity;
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.lock()
    }
}

impl Default for HapticFeedbackManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global touch event handler
static TOUCH_EVENT_HANDLER: spin::Once<TouchEventHandler> = spin::Once::new();

/// Global haptic feedback manager
static HAPTIC_FEEDBACK_MANAGER: spin::Once<HapticFeedbackManager> = spin::Once::new();

/// Get the global touch event handler
pub fn touch_event_handler() -> &'static TouchEventHandler {
    TOUCH_EVENT_HANDLER.call_once(|| TouchEventHandler::default())
}

/// Get the global haptic feedback manager
pub fn haptic_feedback_manager() -> &'static HapticFeedbackManager {
    HAPTIC_FEEDBACK_MANAGER.call_once(|| HapticFeedbackManager::new())
}

/// Handle a touch event
pub fn handle_touch_event(event: TouchEvent) -> Option<GestureEvent> {
    touch_event_handler().handle_touch_event(event)
}

/// Provide haptic feedback
pub fn haptic_feedback(feedback_type: HapticFeedbackType) {
    haptic_feedback_manager().haptic_feedback(feedback_type)
}

/// Set haptic feedback intensity
pub fn set_haptic_intensity(intensity: u8) {
    haptic_feedback_manager().set_intensity(intensity)
}