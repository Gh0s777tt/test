//! # Flux Animation System
//! 
//! Comprehensive animation system supporting transitions, effects, and keyframe animations.
//! Provides smooth, performant animations for all Flux UI components.

use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Animation easing function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasingFunction {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
}

impl EasingFunction {
    /// Apply easing function to progress value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseInQuad => t * t,
            Self::EaseOutQuad => t * (2.0 - t),
            Self::EaseInOutQuad => if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t },
            Self::EaseInCubic => t * t * t,
            Self::EaseOutCubic => (t - 1.0).powi(3) + 1.0,
            Self::EaseInOutCubic => {
                if t < 0.5 { 4.0 * t * t * t } else { (t - 1.0).powi(3) * 4.0 + 1.0 }
            }
            Self::EaseInQuart => t * t * t * t,
            Self::EaseOutQuart => 1.0 - (t - 1.0).powi(4),
            Self::EaseInOutQuart => {
                if t < 0.5 { 8.0 * t * t * t * t } else { 1.0 - 8.0 * (t - 1.0).powi(4) }
            }
            Self::EaseInQuint => t * t * t * t * t,
            Self::EaseOutQuint => 1.0 + (t - 1.0).powi(5),
            Self::EaseInOutQuint => {
                if t < 0.5 { 16.0 * t * t * t * t * t } else { 1.0 + 16.0 * (t - 1.0).powi(5) }
            }
            Self::EaseInSine => 1.0 - (t * std::f32::consts::PI / 2.0).cos(),
            Self::EaseOutSine => (t * std::f32::consts::PI / 2.0).sin(),
            Self::EaseInOutSine => -(std::f32::consts::PI).cos() / 2.0 + 0.5,
            Self::EaseInExpo => if t == 0.0 { 0.0 } else { 2.0_f32.powf(10.0 * (t - 1.0)) - 0.001 },
            Self::EaseOutExpo => if t == 1.0 { 1.0 } else { -2.0_f32.powf(-10.0 * t) + 1.0 },
            Self::EaseInOutExpo => {
                if t == 0.0 || t == 1.0 { t }
                else if t < 0.5 { 0.5 * 2.0_f32.powf(10.0 * (2.0 * t - 2.0)) }
                else { 0.5 * (-2.0_f32.powf(-10.0 * (2.0 * t - 2.0)) + 2.0) }
            }
            Self::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            Self::EaseOutCirc => ((2.0 - t) * t).sqrt(),
            Self::EaseInOutCirc => {
                if t < 0.5 { 0.5 * (1.0 - (1.0 - 4.0 * t * t).sqrt()) }
                else { 0.5 * ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) }
            }
            Self::EaseInElastic => {
                const C4: f32 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 || t == 1.0 { t }
                else { -2.0_f32.powf(10.0 * t - 10.0) * ((t * 10.0 - 10.75) * C4).sin() }
            }
            Self::EaseOutElastic => {
                const C4: f32 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 || t == 1.0 { t }
                else { 2.0_f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * C4).sin() + 1.0 }
            }
            Self::EaseInOutElastic => {
                const C5: f32 = (2.0 * std::f32::consts::PI) / 4.5;
                if t == 0.0 || t == 1.0 { t }
                else if t < 0.5 {
                    -(2.0_f32.powf(20.0 * t - 10.0) * ((t * 20.0 - 11.125) * C5).sin()) / 2.0
                } else {
                    2.0_f32.powf(-20.0 * t + 10.0) * ((t * 20.0 - 11.125) * C5).sin() / 2.0 + 1.0
                }
            }
            Self::EaseInBack => {
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.0;
                C3 * t * t * t - C1 * t * t
            }
            Self::EaseOutBack => {
                const C1: f32 = 1.70158;
                const C3: f32 = C1 + 1.0;
                1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
            }
            Self::EaseInOutBack => {
                const C1: f32 = 1.70158;
                const C2: f32 = C1 * 1.525;
                if t < 0.5 {
                    (2.0 * t).powi(2) * ((C2 + 1.0) * 2.0 * t - C2) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((C2 + 1.0) * (t * 2.0 - 2.0) + C2) + 2.0) / 2.0
                }
            }
            Self::EaseInBounce => 1.0 - Self::EaseOutBounce.apply(1.0 - t),
            Self::EaseOutBounce => {
                const N1: f32 = 7.5625;
                const D1: f32 = 2.75;
                if t < 1.0 / D1 {
                    N1 * t * t
                } else if t < 2.0 / D1 {
                    N1 * (t - 1.5 / D1).powi(2) + 0.75
                } else if t < 2.5 / D1 {
                    N1 * (t - 2.25 / D1).powi(2) + 0.9375
                } else {
                    N1 * (t - 2.625 / D1).powi(2) + 0.984375
                }
            }
            Self::EaseInOutBounce => {
                if t < 0.5 {
                    (1.0 - Self::EaseOutBounce.apply(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + Self::EaseOutBounce.apply(2.0 * t - 1.0)) / 2.0
                }
            }
        }
    }
}

/// Animation property type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationProperty {
    Opacity,
    PositionX,
    PositionY,
    ScaleX,
    ScaleY,
    Rotation,
    Width,
    Height,
    ColorR,
    ColorG,
    ColorB,
    ColorA,
}

/// Animation value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationValue {
    Float(f32),
    Color(u8, u8, u8, u8),
    Bool(bool),
}

impl AnimationValue {
    pub fn float(value: f32) -> Self {
        Self::Float(value)
    }

    pub fn color(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::Color(r, g, b, a)
    }

    pub fn interpolate(&self, target: &AnimationValue, progress: f32) -> AnimationValue {
        match (self, target) {
            (Self::Float(a), Self::Float(b)) => {
                Self::Float(a + (b - a) * progress)
            }
            (Self::Color(ar, ag, ab, aa), Self::Color(br, bg, bb, ba)) => {
                Self::Color(
                    (ar as f32 + (br as f32 - ar as f32) * progress) as u8,
                    (ag as f32 + (bg as f32 - ag as f32) * progress) as u8,
                    (ab as f32 + (bb as f32 - ab as f32) * progress) as u8,
                    (aa as f32 + (ba as f32 - aa as f32) * progress) as u8,
                )
            }
            _ => self.clone(),
        }
    }
}

/// Keyframe definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: f32, // 0.0 to 1.0
    pub value: AnimationValue,
    pub easing: EasingFunction,
}

impl Keyframe {
    pub fn new(time: f32, value: AnimationValue) -> Self {
        Self {
            time: time.clamp(0.0, 1.0),
            value,
            easing: EasingFunction::Linear,
        }
    }

    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }
}

/// Animation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationType {
    Once,
    Loop,
    PingPong,
}

/// Animation state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationState {
    Idle,
    Running,
    Paused,
    Completed,
    Cancelled,
}

/// Animation definition
#[derive(Debug, Clone)]
pub struct Animation {
    pub id: String,
    pub duration: Duration,
    pub delay: Duration,
    pub animation_type: AnimationType,
    pub keyframes: Vec<Keyframe>,
    pub state: AnimationState,
    pub start_time: Option<Instant>,
    pub pause_time: Option<Instant>,
    pub paused_duration: Duration,
    pub current_value: Option<AnimationValue>,
}

impl Animation {
    pub fn new(id: &str, duration: Duration) -> Self {
        Self {
            id: id.to_string(),
            duration,
            delay: Duration::ZERO,
            animation_type: AnimationType::Once,
            keyframes: Vec::new(),
            state: AnimationState::Idle,
            start_time: None,
            pause_time: None,
            paused_duration: Duration::ZERO,
            current_value: None,
        }
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn with_type(mut self, animation_type: AnimationType) -> Self {
        self.animation_type = animation_type;
        self
    }

    pub fn add_keyframe(mut self, keyframe: Keyframe) -> Self {
        self.keyframes.push(keyframe);
        self.keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        self
    }

    pub fn start(&mut self) {
        if self.state == AnimationState::Idle || self.state == AnimationState::Completed {
            self.state = AnimationState::Running;
            self.start_time = Some(Instant::now());
            self.paused_duration = Duration::ZERO;
        }
    }

    pub fn pause(&mut self) {
        if self.state == AnimationState::Running {
            self.state = AnimationState::Paused;
            self.pause_time = Some(Instant::now());
        }
    }

    pub fn resume(&mut self) {
        if self.state == AnimationState::Paused {
            if let Some(pause_time) = self.pause_time {
                self.paused_duration += pause_time.elapsed();
            }
            self.state = AnimationState::Running;
            self.pause_time = None;
        }
    }

    pub fn cancel(&mut self) {
        self.state = AnimationState::Cancelled;
    }

    pub fn reset(&mut self) {
        self.state = AnimationState::Idle;
        self.start_time = None;
        self.pause_time = None;
        self.paused_duration = Duration::ZERO;
        self.current_value = None;
    }

    pub fn update(&mut self) -> bool {
        if self.state != AnimationState::Running {
            return false;
        }

        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed() + self.paused_duration;
            
            if elapsed < self.delay {
                return false;
            }

            let animation_elapsed = elapsed - self.delay;
            let progress = (animation_elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);

            // Calculate current value based on keyframes
            if !self.keyframes.is_empty() {
                if progress <= self.keyframes[0].time {
                    self.current_value = Some(self.keyframes[0].value.clone());
                } else if progress >= self.keyframes.last().unwrap().time {
                    self.current_value = Some(self.keyframes.last().unwrap().value.clone());
                } else {
                    // Find the two keyframes to interpolate between
                    for i in 0..self.keyframes.len() - 1 {
                        let current = &self.keyframes[i];
                        let next = &self.keyframes[i + 1];
                        
                        if progress >= current.time && progress <= next.time {
                            let local_progress = (progress - current.time) / (next.time - current.time);
                            let eased_progress = current.easing.apply(local_progress);
                            self.current_value = Some(current.value.interpolate(&next.value, eased_progress));
                            break;
                        }
                    }
                }
            }

            // Check if animation is complete
            if progress >= 1.0 {
                match self.animation_type {
                    AnimationType::Once => {
                        self.state = AnimationState::Completed;
                    }
                    AnimationType::Loop => {
                        self.start_time = Some(Instant::now());
                        self.paused_duration = Duration::ZERO;
                    }
                    AnimationType::PingPong => {
                        // Implement ping-pong logic by reversing keyframes
                        self.keyframes.reverse();
                        for keyframe in &mut self.keyframes {
                            keyframe.time = 1.0 - keyframe.time;
                        }
                        self.start_time = Some(Instant::now());
                        self.paused_duration = Duration::ZERO;
                    }
                }
            }
        }

        true
    }

    pub fn get_current_value(&self) -> Option<&AnimationValue> {
        self.current_value.as_ref()
    }

    pub fn get_progress(&self) -> f32 {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed() + self.paused_duration;
            if elapsed < self.delay {
                return 0.0;
            }
            let animation_elapsed = elapsed - self.delay;
            (animation_elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}

/// Transition type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransitionType {
    Fade,
    SlideLeft,
    SlideRight,
    SlideUp,
    SlideDown,
    Scale,
    Rotate,
    FlipX,
    FlipY,
}

/// Transition effect
#[derive(Debug, Clone)]
pub struct Transition {
    pub transition_type: TransitionType,
    pub duration: Duration,
    pub easing: EasingFunction,
    pub state: AnimationState,
    pub start_time: Option<Instant>,
}

impl Transition {
    pub fn new(transition_type: TransitionType, duration: Duration) -> Self {
        Self {
            transition_type,
            duration,
            easing: EasingFunction::EaseInOutQuad,
            state: AnimationState::Idle,
            start_time: None,
        }
    }

    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    pub fn start(&mut self) {
        self.state = AnimationState::Running;
        self.start_time = Some(Instant::now());
    }

    pub fn update(&mut self) -> f32 {
        if self.state != AnimationState::Running {
            return if self.state == AnimationState::Completed { 1.0 } else { 0.0 };
        }

        if let Some(start_time) = self.start_time {
            let progress = (start_time.elapsed().as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
            let eased = self.easing.apply(progress);

            if progress >= 1.0 {
                self.state = AnimationState::Completed;
            }

            eased
        } else {
            0.0
        }
    }
}

/// Animation manager
pub struct AnimationManager {
    animations: HashMap<String, Animation>,
    running_animations: Vec<String>,
}

impl AnimationManager {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            running_animations: Vec::new(),
        }
    }

    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.insert(animation.id.clone(), animation);
    }

    pub fn start_animation(&mut self, id: &str) -> Result<(), String> {
        if let Some(animation) = self.animations.get_mut(id) {
            animation.start();
            if !self.running_animations.contains(&id.to_string()) {
                self.running_animations.push(id.to_string());
            }
            Ok(())
        } else {
            Err(format!("Animation '{}' not found", id))
        }
    }

    pub fn pause_animation(&mut self, id: &str) -> Result<(), String> {
        if let Some(animation) = self.animations.get_mut(id) {
            animation.pause();
            Ok(())
        } else {
            Err(format!("Animation '{}' not found", id))
        }
    }

    pub fn resume_animation(&mut self, id: &str) -> Result<(), String> {
        if let Some(animation) = self.animations.get_mut(id) {
            animation.resume();
            Ok(())
        } else {
            Err(format!("Animation '{}' not found", id))
        }
    }

    pub fn cancel_animation(&mut self, id: &str) -> Result<(), String> {
        if let Some(animation) = self.animations.get_mut(id) {
            animation.cancel();
            self.running_animations.retain(|x| x != id);
            Ok(())
        } else {
            Err(format!("Animation '{}' not found", id))
        }
    }

    pub fn update(&mut self) {
        let mut to_remove = Vec::new();
        
        for id in self.running_animations.clone() {
            if let Some(animation) = self.animations.get_mut(&id) {
                animation.update();
                if animation.state == AnimationState::Completed || animation.state == AnimationState::Cancelled {
                    to_remove.push(id);
                }
            }
        }
        
        for id in to_remove {
            self.running_animations.retain(|x| x != &id);
        }
    }

    pub fn get_animation_value(&self, id: &str) -> Option<&AnimationValue> {
        self.animations.get(id)?.get_current_value()
    }

    pub fn get_animation_progress(&self, id: &str) -> Option<f32> {
        Some(self.animations.get(id)?.get_progress())
    }

    pub fn remove_animation(&mut self, id: &str) {
        self.animations.remove(id);
        self.running_animations.retain(|x| x != id);
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let eased = EasingFunction::Linear.apply(0.5);
        assert_eq!(eased, 0.5);
    }

    #[test]
    fn test_easing_in_quad() {
        let eased = EasingFunction::EaseInQuad.apply(0.5);
        assert_eq!(eased, 0.25);
    }

    #[test]
    fn test_keyframe_creation() {
        let keyframe = Keyframe::new(0.5, AnimationValue::float(100.0));
        assert_eq!(keyframe.time, 0.5);
    }

    #[test]
    fn test_animation_creation() {
        let animation = Animation::new("test", Duration::from_millis(1000));
        assert_eq!(animation.id, "test");
        assert_eq!(animation.state, AnimationState::Idle);
    }

    #[test]
    fn test_animation_start() {
        let mut animation = Animation::new("test", Duration::from_millis(1000));
        animation.start();
        assert_eq!(animation.state, AnimationState::Running);
    }

    #[test]
    fn test_animation_value_interpolation() {
        let start = AnimationValue::float(0.0);
        let end = AnimationValue::float(100.0);
        let interpolated = start.interpolate(&end, 0.5);
        assert_eq!(interpolated, AnimationValue::float(50.0));
    }

    #[test]
    fn test_animation_manager() {
        let mut manager = AnimationManager::new();
        let animation = Animation::new("test", Duration::from_millis(1000));
        manager.add_animation(animation);
        manager.start_animation("test").unwrap();
        assert!(manager.get_animation_progress("test").is_some());
    }
}