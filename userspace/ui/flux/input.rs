//! # Flux Input System
//! 
//! Unified input handling system supporting mouse, keyboard, touch, and gestures.
//! Provides a consistent interface for all input devices and high-level gesture recognition.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Input device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputDevice {
    Mouse,
    Keyboard,
    Touchscreen,
    Gamepad,
    Tablet,
}

/// Mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Forward,
    Back,
}

/// Keyboard key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    // Numbers
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    // Function keys
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    // Special keys
    Escape, Tab, CapsLock, LeftShift, RightShift, LeftCtrl, RightCtrl,
    LeftAlt, RightAlt, LeftMeta, RightMeta, Enter, Space, Backspace,
    // Navigation
    Up, Down, Left, Right, Home, End, PageUp, PageDown,
    // Insert/Delete
    Insert, Delete,
    // Print Screen
    PrintScreen, ScrollLock, Pause,
}

/// Modifier key state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
    pub caps_lock: bool,
}

impl Default for Modifiers {
    fn default() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
            caps_lock: false,
        }
    }
}

/// Touch point
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TouchPoint {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
}

impl TouchPoint {
    pub fn new(id: u32, x: f32, y: f32, pressure: f32) -> Self {
        Self { id, x, y, pressure }
    }
}

/// Mouse position
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MousePosition {
    pub x: f32,
    pub y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
}

impl MousePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            delta_x: 0.0,
            delta_y: 0.0,
        }
    }
}

/// Input event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEvent {
    MouseMove(MousePosition),
    MouseDown { button: MouseButton, position: MousePosition },
    MouseUp { button: MouseButton, position: MousePosition },
    MouseWheel { delta_x: f32, delta_y: f32 },
    
    KeyDown { key: KeyCode, modifiers: Modifiers },
    KeyUp { key: KeyCode, modifiers: Modifiers },
    KeyRepeat { key: KeyCode, modifiers: Modifiers },
    
    TouchStart { touches: Vec<TouchPoint> },
    TouchMove { touches: Vec<TouchPoint> },
    TouchEnd { touches: Vec<TouchPoint> },
    TouchCancel { touches: Vec<TouchPoint> },
    
    Gesture(GestureEvent),
}

impl InputEvent {
    /// Get the timestamp of when this event occurred
    pub fn timestamp(&self) -> Instant {
        Instant::now()
    }
}

/// Gesture type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GestureType {
    Tap,
    DoubleTap,
    LongPress,
    Pan,
    Pinch,
    Rotate,
    Swipe,
}

/// Swipe direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Gesture event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureEvent {
    pub gesture_type: GestureType,
    pub center_x: f32,
    pub center_y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub scale: f32,
    pub rotation: f32,
    pub velocity: f32,
    pub direction: Option<SwipeDirection>,
}

impl GestureEvent {
    pub fn new(gesture_type: GestureType, x: f32, y: f32) -> Self {
        Self {
            gesture_type,
            center_x: x,
            center_y: y,
            delta_x: 0.0,
            delta_y: 0.0,
            scale: 1.0,
            rotation: 0.0,
            velocity: 0.0,
            direction: None,
        }
    }
}

/// Input state
#[derive(Debug, Clone)]
pub struct InputState {
    pub mouse_position: MousePosition,
    pub mouse_buttons: HashMap<MouseButton, bool>,
    pub keys: HashMap<KeyCode, bool>,
    pub modifiers: Modifiers,
    pub touches: Vec<TouchPoint>,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            mouse_position: MousePosition::new(0.0, 0.0),
            mouse_buttons: HashMap::new(),
            keys: HashMap::new(),
            modifiers: Modifiers::default(),
            touches: Vec::new(),
        }
    }
}

impl InputState {
    /// Check if a mouse button is pressed
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.get(&button).copied().unwrap_or(false)
    }

    /// Check if a key is pressed
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys.get(&key).copied().unwrap_or(false)
    }

    /// Get number of active touches
    pub fn touch_count(&self) -> usize {
        self.touches.len()
    }
}

/// Gesture recognizer configuration
#[derive(Debug, Clone)]
pub struct GestureConfig {
    pub tap_timeout: Duration,
    pub double_tap_timeout: Duration,
    pub long_press_timeout: Duration,
    pub swipe_threshold: f32,
    pub pinch_threshold: f32,
    pub rotate_threshold: f32,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            tap_timeout: Duration::from_millis(300),
            double_tap_timeout: Duration::from_millis(400),
            long_press_timeout: Duration::from_millis(500),
            swipe_threshold: 50.0,
            pinch_threshold: 10.0,
            rotate_threshold: 10.0,
        }
    }
}

/// Gesture recognizer state
#[derive(Debug, Clone)]
struct GestureRecognizerState {
    tap_start_time: Option<Instant>,
    tap_position: Option<(f32, f32)>,
    last_tap_time: Option<Instant>,
    
    pan_start_time: Option<Instant>,
    pan_start_position: Option<(f32, f32)>,
    
    pinch_start_distance: Option<f32>,
    pinch_start_scale: f32,
    
    rotate_start_angle: Option<f32>,
    
    long_press_active: bool,
}

impl Default for GestureRecognizerState {
    fn default() -> Self {
        Self {
            tap_start_time: None,
            tap_position: None,
            last_tap_time: None,
            pan_start_time: None,
            pan_start_position: None,
            pinch_start_distance: None,
            pinch_start_scale: 1.0,
            rotate_start_angle: None,
            long_press_active: false,
        }
    }
}

/// Gesture recognizer
pub struct GestureRecognizer {
    config: GestureConfig,
    state: GestureRecognizerState,
}

impl GestureRecognizer {
    pub fn new() -> Self {
        Self::with_config(GestureConfig::default())
    }

    pub fn with_config(config: GestureConfig) -> Self {
        Self {
            config,
            state: GestureRecognizerState::default(),
        }
    }

    /// Process input events and recognize gestures
    pub fn process(&mut self, event: &InputEvent) -> Vec<GestureEvent> {
        let mut gestures = Vec::new();

        match event {
            InputEvent::TouchStart { touches } => {
                if touches.len() == 1 {
                    self.state.tap_start_time = Some(Instant::now());
                    self.state.tap_position = Some((touches[0].x, touches[0].y));
                    self.state.pan_start_time = Some(Instant::now());
                    self.state.pan_start_position = Some((touches[0].x, touches[0].y));
                } else if touches.len() == 2 {
                    let distance = Self::calculate_distance(touches[0], touches[1]);
                    self.state.pinch_start_distance = Some(distance);
                    self.state.rotate_start_angle = Some(Self::calculate_angle(touches[0], touches[1]));
                }
            }

            InputEvent::TouchMove { touches } => {
                if touches.len() == 1 {
                    if let Some(start) = self.state.pan_start_position {
                        let dx = touches[0].x - start.0;
                        let dy = touches[0].y - start.1;
                        
                        if dx.abs() > self.config.swipe_threshold || dy.abs() > self.config.swipe_threshold {
                            let mut gesture = GestureEvent::new(GestureType::Pan, touches[0].x, touches[0].y);
                            gesture.delta_x = dx;
                            gesture.delta_y = dy;
                            gestures.push(gesture);
                        }
                    }
                } else if touches.len() == 2 {
                    if let Some(start_dist) = self.state.pinch_start_distance {
                        let current_dist = Self::calculate_distance(touches[0], touches[1]);
                        let scale = current_dist / start_dist;
                        
                        if (scale - 1.0).abs() > 0.1 {
                            let center_x = (touches[0].x + touches[1].x) / 2.0;
                            let center_y = (touches[0].y + touches[1].y) / 2;
                            let mut gesture = GestureEvent::new(GestureType::Pinch, center_x, center_y);
                            gesture.scale = scale;
                            gestures.push(gesture);
                        }
                    }

                    if let Some(start_angle) = self.state.rotate_start_angle {
                        let current_angle = Self::calculate_angle(touches[0], touches[1]);
                        let rotation = current_angle - start_angle;
                        
                        if rotation.abs() > self.config.rotate_threshold {
                            let center_x = (touches[0].x + touches[1].x) / 2.0;
                            let center_y = (touches[0].y + touches[1].y) / 2.0;
                            let mut gesture = GestureEvent::new(GestureType::Rotate, center_x, center_y);
                            gesture.rotation = rotation.to_degrees();
                            gestures.push(gesture);
                        }
                    }
                }
            }

            InputEvent::TouchEnd { touches } => {
                if let Some(start_time) = self.state.tap_start_time {
                    let duration = start_time.elapsed();
                    
                    if duration < self.config.tap_timeout {
                        if let Some(pos) = self.state.tap_position {
                            let now = Instant::now();
                            if let Some(last_tap) = self.state.last_tap_time {
                                if now.duration_since(last_tap) < self.config.double_tap_timeout {
                                    let mut gesture = GestureEvent::new(GestureType::DoubleTap, pos.0, pos.1);
                                    gestures.push(gesture);
                                } else {
                                    let mut gesture = GestureEvent::new(GestureType::Tap, pos.0, pos.1);
                                    gestures.push(gesture);
                                }
                            } else {
                                let mut gesture = GestureEvent::new(GestureType::Tap, pos.0, pos.1);
                                gestures.push(gesture);
                            }
                            self.state.last_tap_time = Some(now);
                        }
                    }
                }

                // Check for swipe
                if let Some(start) = self.state.pan_start_position {
                    if let Some(start_time) = self.state.pan_start_time {
                        let duration = start_time.elapsed();
                        let dx = touches[0].x - start.0;
                        let dy = touches[0].y - start.1;
                        let distance = (dx * dx + dy * dy).sqrt();
                        
                        if distance > self.config.swipe_threshold {
                            let velocity = distance / duration.as_secs_f32();
                            let direction = if dx.abs() > dy.abs() {
                                if dx > 0.0 {
                                    Some(SwipeDirection::Right)
                                } else {
                                    Some(SwipeDirection::Left)
                                }
                            } else {
                                if dy > 0.0 {
                                    Some(SwipeDirection::Down)
                                } else {
                                    Some(SwipeDirection::Up)
                                }
                            };

                            let mut gesture = GestureEvent::new(GestureType::Swipe, touches[0].x, touches[0].y);
                            gesture.delta_x = dx;
                            gesture.delta_y = dy;
                            gesture.velocity = velocity;
                            gesture.direction = direction;
                            gestures.push(gesture);
                        }
                    }
                }

                // Reset state
                self.state.tap_start_time = None;
                self.state.tap_position = None;
                self.state.pan_start_time = None;
                self.state.pan_start_position = None;
                self.state.pinch_start_distance = None;
                self.state.rotate_start_angle = None;
                self.state.long_press_active = false;
            }

            InputEvent::TouchCancel { .. } => {
                // Reset state
                self.state.tap_start_time = None;
                self.state.tap_position = None;
                self.state.pan_start_time = None;
                self.state.pan_start_position = None;
                self.state.pinch_start_distance = None;
                self.state.rotate_start_angle = None;
                self.state.long_press_active = false;
            }

            _ => {}
        }

        gestures
    }

    fn calculate_distance(touch1: TouchPoint, touch2: TouchPoint) -> f32 {
        let dx = touch2.x - touch1.x;
        let dy = touch2.y - touch1.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn calculate_angle(touch1: TouchPoint, touch2: TouchPoint) -> f32 {
        let dx = touch2.x - touch1.x;
        let dy = touch2.y - touch1.y;
        dy.atan2(dx)
    }
}

impl Default for GestureRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Main input manager
pub struct InputManager {
    state: InputState,
    gesture_recognizer: GestureRecognizer,
    event_buffer: Vec<InputEvent>,
    max_buffer_size: usize,
}

impl InputManager {
    pub fn new() -> Self {
        Self::with_config(GestureConfig::default(), 1000)
    }

    pub fn with_config(gesture_config: GestureConfig, max_buffer_size: usize) -> Self {
        Self {
            state: InputState::default(),
            gesture_recognizer: GestureRecognizer::with_config(gesture_config),
            event_buffer: Vec::new(),
            max_buffer_size,
        }
    }

    /// Process a new input event
    pub fn process_event(&mut self, event: InputEvent) -> Vec<InputEvent> {
        let mut result = Vec::new();

        // Update state
        match &event {
            InputEvent::MouseMove(pos) => {
                self.state.mouse_position = *pos;
            }
            InputEvent::MouseDown { button, position } => {
                self.state.mouse_buttons.insert(*button, true);
                self.state.mouse_position = *position;
            }
            InputEvent::MouseUp { button, position } => {
                self.state.mouse_buttons.insert(*button, false);
                self.state.mouse_position = *position;
            }
            InputEvent::KeyDown { key, modifiers } => {
                self.state.keys.insert(*key, true);
                self.state.modifiers = *modifiers;
            }
            InputEvent::KeyUp { key, modifiers } => {
                self.state.keys.insert(*key, false);
                self.state.modifiers = *modifiers;
            }
            InputEvent::TouchStart { touches } => {
                self.state.touches = touches.clone();
            }
            InputEvent::TouchMove { touches } => {
                self.state.touches = touches.clone();
            }
            InputEvent::TouchEnd { touches } => {
                self.state.touches = touches.clone();
            }
            InputEvent::TouchCancel { touches } => {
                self.state.touches = touches.clone();
            }
            _ => {}
        }

        // Recognize gestures
        let gestures = self.gesture_recognizer.process(&event);
        for gesture in gestures {
            let gesture_event = InputEvent::Gesture(gesture);
            result.push(gesture_event.clone());
            self.event_buffer.push(gesture_event);
        }

        // Add event to buffer
        self.event_buffer.push(event.clone());
        if self.event_buffer.len() > self.max_buffer_size {
            self.event_buffer.drain(0..self.event_buffer.len() - self.max_buffer_size);
        }

        result.push(event);
        result
    }

    /// Get current input state
    pub fn get_state(&self) -> &InputState {
        &self.state
    }

    /// Get event history
    pub fn get_event_history(&self) -> &[InputEvent] {
        &self.event_buffer
    }

    /// Clear event buffer
    pub fn clear_buffer(&mut self) {
        self.event_buffer.clear();
    }

    /// Check if any mouse button is pressed
    pub fn is_any_mouse_button_pressed(&self) -> bool {
        self.state.mouse_buttons.values().any(|&pressed| pressed)
    }

    /// Check if any modifier key is pressed
    pub fn is_any_modifier_pressed(&self) -> bool {
        self.state.modifiers.shift || self.state.modifiers.ctrl 
            || self.state.modifiers.alt || self.state.modifiers.meta
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state_creation() {
        let state = InputState::default();
        assert!(!state.is_mouse_pressed(MouseButton::Left));
        assert!(!state.is_key_pressed(KeyCode::A));
    }

    #[test]
    fn test_touch_point_creation() {
        let touch = TouchPoint::new(1, 100.0, 200.0, 0.5);
        assert_eq!(touch.id, 1);
        assert_eq!(touch.x, 100.0);
        assert_eq!(touch.y, 200.0);
    }

    #[test]
    fn test_gesture_recognizer_creation() {
        let recognizer = GestureRecognizer::new();
        assert_eq!(recognizer.config.tap_timeout, Duration::from_millis(300));
    }

    #[test]
    fn test_input_manager_creation() {
        let manager = InputManager::new();
        assert_eq!(manager.get_state().touch_count(), 0);
    }

    #[test]
    fn test_mouse_position() {
        let pos = MousePosition::new(100.0, 200.0);
        assert_eq!(pos.x, 100.0);
        assert_eq!(pos.y, 200.0);
        assert_eq!(pos.delta_x, 0.0);
    }

    #[test]
    fn test_modifiers_default() {
        let modifiers = Modifiers::default();
        assert!(!modifiers.shift);
        assert!(!modifiers.ctrl);
    }
}