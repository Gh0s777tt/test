// Touch Gestures for VantisOS v0.6.0 ARM64 kernel
// Touch UI Framework - Touch Gestures

use super::touch_event::{TouchEvent, TouchPoint, GestureEvent, GestureType, GestureDirection};
use super::framework::UIElementId;

// Gesture recognizer
pub struct GestureRecognizer {
    tap_threshold: u32,        // Maximum movement for tap (pixels)
    tap_timeout: u64,          // Maximum time for tap (milliseconds)
    double_tap_timeout: u64,  // Maximum time between taps (milliseconds)
    long_press_timeout: u64,   // Minimum time for long press (milliseconds)
    swipe_threshold: u32,      // Minimum movement for swipe (pixels)
    swipe_timeout: u64,        // Maximum time for swipe (milliseconds)
    pinch_threshold: f32,      // Minimum distance change for pinch (pixels)
    zoom_threshold: f32,       // Minimum distance change for zoom (pixels)
}

impl GestureRecognizer {
    pub const fn new() -> Self {
        GestureRecognizer {
            tap_threshold: 20,
            tap_timeout: 300,
            double_tap_timeout: 500,
            long_press_timeout: 500,
            swipe_threshold: 50,
            swipe_timeout: 500,
            pinch_threshold: 20.0,
            zoom_threshold: 20.0,
        }
    }

    pub fn set_tap_threshold(&mut self, threshold: u32) {
        self.tap_threshold = threshold;
    }

    pub fn set_tap_timeout(&mut self, timeout: u64) {
        self.tap_timeout = timeout;
    }

    pub fn set_double_tap_timeout(&mut self, timeout: u64) {
        self.double_tap_timeout = timeout;
    }

    pub fn set_long_press_timeout(&mut self, timeout: u64) {
        self.long_press_timeout = timeout;
    }

    pub fn set_swipe_threshold(&mut self, threshold: u32) {
        self.swipe_threshold = threshold;
    }

    pub fn set_swipe_timeout(&mut self, timeout: u64) {
        self.swipe_timeout = timeout;
    }

    pub fn set_pinch_threshold(&mut self, threshold: f32) {
        self.pinch_threshold = threshold;
    }

    pub fn set_zoom_threshold(&mut self, threshold: f32) {
        self.zoom_threshold = threshold;
    }
}

impl Default for GestureRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

// Gesture state
pub struct GestureState {
    touch_points: [Option<TouchPoint>; 10],
    num_points: usize,
    start_time: u64,
    last_time: u64,
    start_points: [Option<TouchPoint>; 10],
    last_tap_time: u64,
    tap_count: u8,
    is_long_pressing: bool,
    initial_distance: f32,
}

impl GestureState {
    pub const fn new() -> Self {
        GestureState {
            touch_points: [None; 10],
            num_points: 0,
            start_time: 0,
            last_time: 0,
            start_points: [None; 10],
            last_tap_time: 0,
            tap_count: 0,
            is_long_pressing: false,
            initial_distance: 0.0,
        }
    }

    pub fn update(&mut self, event: &TouchEvent) {
        self.last_time = event.timestamp;

        match event.event_type {
            super::touch_event::TouchEventType::Down => {
                self.start_time = event.timestamp;
                self.last_time = event.timestamp;
                self.num_points = event.num_points;
                for i in 0..10 {
                    self.touch_points[i] = event.points[i];
                    self.start_points[i] = event.points[i];
                }
                self.is_long_pressing = false;
                
                // Calculate initial distance for pinch/zoom
                if self.num_points == 2 {
                    if let (Some(p1), Some(p2)) = (self.touch_points[0], self.touch_points[1]) {
                        self.initial_distance = p1.distance(p2);
                    }
                }
            }
            super::touch_event::TouchEventType::Move => {
                self.num_points = event.num_points;
                for i in 0..10 {
                    self.touch_points[i] = event.points[i];
                }
            }
            super::touch_event::TouchEventType::Up => {
                self.num_points = event.num_points;
                for i in 0..10 {
                    self.touch_points[i] = event.points[i];
                }
            }
            _ => {}
        }
    }

    pub fn reset(&mut self) {
        for i in 0..10 {
            self.touch_points[i] = None;
            self.start_points[i] = None;
        }
        self.num_points = 0;
        self.start_time = 0;
        self.last_time = 0;
        self.last_tap_time = 0;
        self.tap_count = 0;
        self.is_long_pressing = false;
        self.initial_distance = 0.0;
    }

    pub fn get_duration(&self) -> u64 {
        self.last_time - self.start_time
    }

    pub fn get_movement(&self) -> f32 {
        if self.num_points == 0 {
            return 0.0;
        }

        let mut total_distance = 0.0;
        for i in 0..self.num_points {
            if let (Some(start), Some(current)) = (self.start_points[i], self.touch_points[i]) {
                total_distance += start.distance(current);
            }
        }

        total_distance / self.num_points as f32
    }

    pub fn get_center(&self) -> Option<(i32, i32)> {
        if self.num_points == 0 {
            return None;
        }

        let mut sum_x = 0i64;
        let mut sum_y = 0i64;
        let mut count = 0;

        for point in &self.touch_points {
            if let Some(p) = point {
                sum_x += p.x as i64;
                sum_y += p.y as i64;
                count += 1;
            }
        }

        Some(((sum_x / count) as i32, (sum_y / count) as i32))
    }

    pub fn get_distance(&self) -> f32 {
        if self.num_points == 2 {
            if let (Some(p1), Some(p2)) = (self.touch_points[0], self.touch_points[1]) {
                return p1.distance(p2);
            }
        }
        0.0
    }

    pub fn get_distance_change(&self) -> f32 {
        if self.num_points == 2 {
            if let (Some(p1), Some(p2)) = (self.start_points[0], self.start_points[1]) {
                let current_distance = p1.distance(p2);
                return (current_distance - self.initial_distance).abs();
            }
        }
        0.0
    }
}

impl Default for GestureState {
    fn default() -> Self {
        Self::new()
    }
}

// Gesture handler
pub type GestureHandler = fn(&GestureEvent);

// Gesture manager
pub struct GestureManager {
    recognizer: GestureRecognizer,
    state: GestureState,
    handlers: [Option<GestureHandler>; 20],
    num_handlers: usize,
}

impl GestureManager {
    pub const fn new() -> Self {
        GestureManager {
            recognizer: GestureRecognizer::new(),
            state: GestureState::new(),
            handlers: [None; 20],
            num_handlers: 0,
        }
    }

    pub fn add_handler(&mut self, handler: GestureHandler) -> Result<(), GestureError> {
        if self.num_handlers >= 20 {
            return Err(GestureError::TooManyHandlers);
        }

        for i in 0..20 {
            if self.handlers[i].is_none() {
                self.handlers[i] = Some(handler);
                self.num_handlers += 1;
                return Ok(());
            }
        }

        Err(GestureError::TooManyHandlers)
    }

    pub fn process_event(&mut self, event: &TouchEvent) {
        self.state.update(event);

        // Check for long press
        if !self.state.is_long_pressing && self.state.get_duration() > self.recognizer.long_press_timeout {
            self.state.is_long_pressing = true;
            self.dispatch_gesture(GestureType::LongPress);
        }

        match event.event_type {
            super::touch_event::TouchEventType::Up => {
                // Check for tap
                if self.state.get_movement() < self.recognizer.tap_threshold as f32 {
                    let now = event.timestamp;
                    
                    // Check for double tap
                    if now - self.state.last_tap_time < self.recognizer.double_tap_timeout {
                        self.state.tap_count += 1;
                        if self.state.tap_count == 2 {
                            self.dispatch_gesture(GestureType::DoubleTap);
                            self.state.tap_count = 0;
                        }
                    } else {
                        self.state.tap_count = 1;
                        self.dispatch_gesture(GestureType::Tap);
                    }
                    
                    self.state.last_tap_time = now;
                }

                // Check for swipe
                else if self.state.get_duration() < self.recognizer.swipe_timeout {
                    if let Some(start) = self.state.start_points[0] {
                        if let Some(end) = self.state.touch_points[0] {
                            let dx = (end.x - start.x).abs();
                            let dy = (end.y - start.y).abs();
                            
                            if dx > self.recognizer.swipe_threshold || dy > self.recognizer.swipe_threshold {
                                let gesture_event = GestureEvent::new(GestureType::Swipe, start, end);
                                self.dispatch_gesture_event(&gesture_event);
                            }
                        }
                    }
                }

                // Check for pinch/zoom
                else if self.state.num_points == 2 {
                    let distance_change = self.state.get_distance_change();
                    
                    if distance_change > self.recognizer.pinch_threshold {
                        let current_distance = self.state.get_distance();
                        let initial_distance = self.state.initial_distance;
                        
                        if current_distance < initial_distance {
                            self.dispatch_gesture(GestureType::Pinch);
                        } else {
                            self.dispatch_gesture(GestureType::Zoom);
                        }
                    }
                }

                self.state.reset();
            }
            _ => {}
        }
    }

    fn dispatch_gesture(&mut self, gesture_type: GestureType) {
        if let Some(center) = self.state.get_center() {
            let start = self.state.start_points[0].unwrap_or_else(|| {
                TouchPoint::new(0, center.0, center.1)
            });
            
            let end = self.state.touch_points[0].unwrap_or(start);
            
            let gesture_event = GestureEvent::new(gesture_type, start, end);
            self.dispatch_gesture_event(&gesture_event);
        }
    }

    fn dispatch_gesture_event(&mut self, gesture_event: &GestureEvent) {
        for i in 0..self.num_handlers {
            if let Some(handler) = self.handlers[i] {
                handler(gesture_event);
            }
        }
    }

    pub fn clear(&mut self) {
        for i in 0..20 {
            self.handlers[i] = None;
        }
        self.num_handlers = 0;
    }

    pub fn len(&self) -> usize {
        self.num_handlers
    }

    pub fn is_empty(&self) -> bool {
        self.num_handlers == 0
    }

    pub fn get_recognizer(&mut self) -> &mut GestureRecognizer {
        &mut self.recognizer
    }
}

impl Default for GestureManager {
    fn default() -> Self {
        Self::new()
    }
}

// Gesture errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureError {
    TooManyHandlers,
    InvalidGesture,
    Conflict,
}

// Gesture conflict resolver
pub struct GestureConflictResolver {
    priority_map: [(GestureType, u8); 6],
}

impl GestureConflictResolver {
    pub const fn new() -> Self {
        GestureConflictResolver {
            priority_map: [
                (GestureType::Tap, 1),
                (GestureType::DoubleTap, 2),
                (GestureType::LongPress, 3),
                (GestureType::Swipe, 4),
                (GestureType::Pinch, 5),
                (GestureType::Zoom, 6),
            ],
        }
    }

    pub fn resolve_conflict(&self, gesture1: GestureType, gesture2: GestureType) -> GestureType {
        let priority1 = self.get_priority(gesture1);
        let priority2 = self.get_priority(gesture2);

        if priority1 >= priority2 {
            gesture1
        } else {
            gesture2
        }
    }

    fn get_priority(&self, gesture_type: GestureType) -> u8 {
        for &(gt, priority) in &self.priority_map {
            if gt == gesture_type {
                return priority;
            }
        }
        0
    }
}

impl Default for GestureConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

// Gesture animation
pub struct GestureAnimation {
    gesture_type: GestureType,
    duration: u64,          // Animation duration in milliseconds
    start_time: u64,
    progress: f32,          // 0.0 to 1.0
    is_animating: bool,
}

impl GestureAnimation {
    pub fn new(gesture_type: GestureType, duration: u64) -> Self {
        GestureAnimation {
            gesture_type,
            duration,
            start_time: Self::get_timestamp(),
            progress: 0.0,
            is_animating: true,
        }
    }

    pub fn update(&mut self) -> bool {
        if !self.is_animating {
            return false;
        }

        let current_time = Self::get_timestamp();
        let elapsed = current_time - self.start_time;

        if elapsed >= self.duration {
            self.progress = 1.0;
            self.is_animating = false;
            return false;
        }

        self.progress = (elapsed as f32) / (self.duration as f32);
        true
    }

    pub fn get_progress(&self) -> f32 {
        self.progress
    }

    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    pub fn get_gesture_type(&self) -> GestureType {
        self.gesture_type
    }

    fn get_timestamp() -> u64 {
        use core::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }
}

// Gesture animation manager
pub struct GestureAnimationManager {
    animations: [Option<GestureAnimation>; 10],
    num_animations: usize,
}

impl GestureAnimationManager {
    pub const fn new() -> Self {
        GestureAnimationManager {
            animations: [None; 10],
            num_animations: 0,
        }
    }

    pub fn start_animation(&mut self, gesture_type: GestureType, duration: u64) {
        let animation = GestureAnimation::new(gesture_type, duration);

        if self.num_animations < 10 {
            for i in 0..10 {
                if self.animations[i].is_none() {
                    self.animations[i] = Some(animation);
                    self.num_animations += 1;
                    return;
                }
            }
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.num_animations {
            if let Some(ref mut animation) = self.animations[i] {
                if !animation.update() {
                    self.animations[i] = None;
                    self.num_animations -= 1;
                }
            }
        }
    }

    pub fn get_animations(&self) -> Vec<&GestureAnimation> {
        let mut result = Vec::new();
        for animation in &self.animations {
            if let Some(ref a) = animation {
                result.push(a);
            }
        }
        result
    }

    pub fn clear(&mut self) {
        for i in 0..10 {
            self.animations[i] = None;
        }
        self.num_animations = 0;
    }

    pub fn len(&self) -> usize {
        self.num_animations
    }

    pub fn is_empty(&self) -> bool {
        self.num_animations == 0
    }
}

impl Default for GestureAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}
