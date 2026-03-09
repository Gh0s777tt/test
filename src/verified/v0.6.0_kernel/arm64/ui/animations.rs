// UI Animations for VantisOS vv0.6.0 ARM64 kernel
// Touch UI Framework - UI Animations

use super::framework::UIElementId;

// Animation curve types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationCurve {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
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
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
}

impl AnimationCurve {
    pub fn evaluate(&self, t: f32) -> f32 {
        match self {
            AnimationCurve::Linear => t,
            AnimationCurve::EaseIn => t * t,
            AnimationCurve::EaseOut => t * (2.0 - t),
            AnimationCurve::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            AnimationCurve::EaseInQuad => t * t,
            AnimationCurve::EaseOutQuad => t * (2.0 - t),
            AnimationCurve::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            AnimationCurve::EaseInCubic => t * t * t,
            AnimationCurve::EaseOutCubic => {
                let t1 = t - 1.0;
                t1 * t1 * t1 + 1.0
            }
            AnimationCurve::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t1 = t - 1.0;
                    1.0 + 4.0 * t1 * t1 * t1
                }
            }
            AnimationCurve::EaseInQuart => t * t * t * t,
            AnimationCurve::EaseOutQuart => {
                let t1 = t - 1.0;
                1.0 - t1 * t1 * t1 * t1
            }
            AnimationCurve::EaseInOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let t1 = t - 1.0;
                    1.0 - 8.0 * t1 * t1 * t1 * t1
                }
            }
            AnimationCurve::EaseInQuint => t * t * t * t * t,
            AnimationCurve::EaseOutQuint => {
                let t1 = t - 1.0;
                1.0 + t1 * t1 * t1 * t1 * t1
            }
            AnimationCurve::EaseInOutQuint => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    let t1 = t - 1.0;
                    1.0 + 16.0 * t1 * t1 * t1 * t1 * t1
                }
            }
            AnimationCurve::EaseInSine => 1.0 - (t * (2.0 * core::f32::consts::PI / 2.0)).cos(),
            AnimationCurve::EaseOutSine => (t * (2.0 * core::f32::consts::PI / 2.0)).sin(),
            AnimationCurve::EaseInOutSine => {
                -(core::f32::consts::PI / 2.0) * (1.0 - (t * core::f32::consts::PI).cos()) / 2.0
            }
            AnimationCurve::EaseInExpo => {
                if t == 0.0 { 0.0 } else { 2.0_f32.powf(10.0 * (t - 1.0)) }
            }
            AnimationCurve::EaseOutExpo => {
                if t == 1.0 { 1.0 } else { 1.0 + 2.0_f32.powf(10.0 * (t - 1.0)) }
            }
            AnimationCurve::EaseInOutExpo => {
                if t < 0.5 {
                    0.5 * 2.0_f32.powf(10.0 * (2.0 * t - 1.0))
                } else {
                    0.5 * (2.0_f32.powf(10.0 * (2.0 * t - 2.0)) + 2.0)
                }
            }
            AnimationCurve::EaseInCirc => {
                1.0 - (1.0 - t).sqrt()
            }
            AnimationCurve::EaseOutCirc => {
                let t1 = t - 1.0;
                t1.sqrt()
            }
            AnimationCurve::EaseInOutCirc => {
                if t < 0.5 {
                    1.0 - (1.0 - 2.0 * t).sqrt() / 2.0
                } else {
                    (1.0 + (1.0 + 2.0 * (t - 1.0)).sqrt() / 2.0
                }
            }
            AnimationCurve::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            AnimationCurve::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powf(3.0) + c1 * (t - 1.0).powf(2.0)
            }
            AnimationCurve::EaseInOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                if t < 0.5 {
                    0.5 * (2.0 * t).powf(3.0) - c1 * t * t
                } else {
                    0.5 * (2.0 * (t - 1.0)).powf(3.0) + c1 * (t - 1.0).powf(2.0) + 2.0
                }
            }
            AnimationCurve::EaseInElastic => {
                let c4 = (2.0 * core::f32::consts::PI) / 3.0;
                let s = c4 / 4.0;
                let c5 = c4 / 3.0;
                let c2 = c5 * 1.5;
                let c3 = c5 + 1.0;
                c2 * t * t * ((s + 1.0) * t - 1.0) + 1.0
            }
            AnimationCurve::EaseOutElastic => {
                let c4 = (2.0 * core::f32::consts::PI) / 3.0;
                let s = c4 / 4.0;
                let c5 = c4 / 3.0;
                let c2 = c5 + 1.0;
                let c3 = c5 + 1.0;
                c2 * (t - 1.0).powf(2.0) * ((s + 1.0) * (t - 1.0) + 1.0) + 1.0
            }
            AnimationCurve::EaseInOutElastic => {
                let c4 = (2.0 * core::f32::consts::PI) / 3.0;
                let s = c4 / 4.0;
                let c5 = c4 / 3.0;
                let c2 = c5 * 1.5;
                let c3 = c5 + 1.0;
                if t < 0.5 {
                    0.5 * (c2 * (2.0 * t).powf(3.0) - c3 * 2.0 * t * t + c4 * t)
                } else {
                    0.5 * (c2 * (2.0 * (t - 1.0)).powf(3.0) + c3 * 2.0 * (t - 1.0) * (t - 1.0) + c4 * (t - 1.0) + 2.0)
                }
            }
            AnimationCurve::EaseInBounce => {
                let c1 = 7.5625;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            AnimationCurve::EaseOutBounce => {
                let c1 = 7.5625;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powf(3.0) + c1 * (t - 1.0).powf(2.0)
            }
            AnimationCurve::EaseInOutBounce => {
                let c1 = 7.5625;
                let c3 = c1 + 1.0;
                if t < 0.5 {
                    0.5 * (c3 * (2.0 * t).powf(3.0) - c1 * 2.0 * t * t)
                } else {
                    0.5 * (c3 * (2.0 * (t - 1.0)).powf(3.0) + c1 * 2.0 * (t - 1.0) * (t - 1.0) + 2.0)
                }
            }
        }
    }
}

// Animation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    Fade,
    Slide,
    Scale,
    Rotate,
    Translate,
    Custom(u32),
}

// Transition types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    FadeIn,
    FadeOut,
    SlideInLeft,
    SlideInRight,
    SlideInUp,
    SlideInDown,
    ScaleIn,
    ScaleOut,
    RotateIn,
    RotateOut,
    Custom(u32),
}

// Animation
pub struct Animation {
    id: u32,
    element_id: UIElementId,
    animation_type: AnimationType,
    duration: u64,          // Duration in milliseconds
    delay: u64,             // Delay before starting in milliseconds
    curve: AnimationCurve,
    start_value: f32,
    end_value: f32,
    start_time: u64,
    is_running: bool,
    is_paused: bool,
    is_complete: bool,
    progress: f32,          // 0.0 to 1.0
}

impl Animation {
    pub fn new(id: u32, element_id: UIElementId, animation_type: AnimationType, duration: u64) -> Self {
        Animation {
            id,
            element_id,
            animation_type,
            duration,
            delay: 0,
            curve: AnimationCurve::EaseInOut,
            start_value: 0.0,
            end_value: 1.0,
            start_time: 0,
            is_running: false,
            is_paused: false,
            is_complete: false,
            progress: 0.0,
        }
    }

    pub fn with_curve(mut self, curve: AnimationCurve) -> Self {
        self.curve = curve;
        self
    }

    pub fn with_delay(mut self, delay: u64) -> Self {
        self.delay = delay;
        self
    }

    pub fn with_values(mut self, start: f32, end: f32) -> Self {
        self.start_value = start;
        self.end_value = end;
        self
    }

    pub fn start(&mut self) {
        self.start_time = Self::get_timestamp();
        self.is_running = true;
        self.is_paused = false;
        self.is_complete = false;
        self.progress = 0.0;
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    pub fn resume(&mut self) {
        if self.is_running && self.is_paused {
            self.is_paused = false;
            // Adjust start_time to account for pause duration
            let elapsed = self.get_elapsed();
            self.start_time = Self::get_timestamp() - elapsed;
        }
    }

    pub fn stop(&mut self) {
        self.is_running = false;
        self.is_complete = true;
        self.progress = 1.0;
    }

    pub fn reset(&mut self) {
        self.start_time = 0;
        self.is_running = false;
        is_paused = false;
        self.is_complete = false;
        self.progress = 0.0;
    }

    pub fn update(&mut self) -> bool {
        if !self.is_running || self.is_paused || self.is_complete {
            return false;
        }

        let elapsed = self.get_elapsed();

        if elapsed < self.delay {
            return true; // Still waiting for delay
        }

        let animation_time = elapsed - self.delay;

        if animation_time >= self.duration {
            self.progress = 1.0;
            self.is_complete = true;
            self.is_running = false;
            return false;
        }

        let t = animation_time as f32 / self.duration as f32;
        let eased_t = self.curve.evaluate(t);
        self.progress = eased_t;

        true
    }

    pub fn get_elapsed(&self) -> u64 {
        if self.start_time == 0 {
            0
        } else {
            Self::get_timestamp() - self.start_time
        }
    }

    pub fn get_progress(&self) -> f32 {
        self.progress
    }

    pub fn get_current_value(&self) -> f32 {
        let range = self.end_value - self.start_value;
        self.start_value + range * self.progress
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    fn get_timestamp() -> u64 {
        use core::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }
}

// Animation manager
pub struct AnimationManager {
    animations: [Option<Animation>; 50],
    num_animations: usize,
    next_id: u32,
}

impl AnimationManager {
    pub const fn new() -> Self {
        AnimationManager {
            animations: [None; 50],
            num_animations: 0,
            next_id: 1,
        }
    }

    pub fn create_animation(&mut self, element_id: UIElementId, animation_type: AnimationType, duration: u64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let animation = Animation::new(id, element_id, animation_type, duration);

        if self.num_animations < 50 {
            for i in 0..50 {
                if self.animations[i].is_none() {
                    self.animations[i] = Some(animation);
                    self.num_animations += 1;
                    return id;
                }
            }
        }

        id
    }

    pub fn start_animation(&mut self, id: u32) -> bool {
        if let Some(animation) = self.get_animation_mut(id) {
            animation.start();
            true
        } else {
            false
        }
    }

    pub fn pause_animation(&mut self, id: u32) -> bool {
        if let Some(animation) = self.get_animation_mut(id) {
            animation.pause();
            true
        } else {
            false
        }
    }

    pub fn resume_animation(&mut self, id: u32) -> bool {
        if let Some(animation) =_animation.get_animation_mut(id) {
            animation.resume();
            true
        } else {
            false
        }
    }

    pub fn stop_animation(&mut self, id: u32) -> bool {
        if let Some(animation) = self.get_animation_mut(id) {
            animation.stop();
            true
        } else {
            false
        }
    }

    pub fn remove_animation(&mut self, id: u32) -> bool {
        for i in 0..self.num_animations {
            if let Some(ref animation) = self.animations[i] {
                if animation.id == id {
                    self.animations[i] = None;
                    self.num_animations -= 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn get_animation(&self, id: u32) -> Option<&Animation> {
        for animation in &self.animations {
            if let Some(ref a) = animation {
                if a.id == id {
                    return Some(a);
                }
            }
        }
        None
    }

    pub fn get_animation_mut(&mut self, id: u32) -> Option<&mut Animation> {
        for animation in &mut self.animations {
            if let Some(ref mut a) = animation {
                if a.id == id {
                    return Some(a);
                }
            }
        }
        None
    }

    pub fn get_animations_for_element(&self, element_id: UIElementId) -> Vec<&Animation> {
        let mut result = Vec::new();
        for animation in &self.animations {
            if let Some(ref a) = animation {
                if a.element_id == element_id {
                    result.push(a);
                }
            }
        }
        result
    }

    pub fn update(&mut self) {
        for i in 0..self.num_animations {
            if let Some(ref mut animation) = self.animations[i] {
                animation.update();
            }
        }
    }

    pub fn clear(&mut self) {
        for i in 0..50 {
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

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

// Transition animation
pub struct TransitionAnimation {
    animation: Animation,
    transition_type: TransitionType,
    from_rect: super::framework::UIRect,
    to_rect: super::framework::UIRect,
}

impl TransitionAnimation {
    pub fn new(id: u32, element_id: UIElementId, transition_type: TransitionType, duration: u64, from_rect: super::framework::UIRect, to_rect: super::framework::UIRect) -> Self {
        let animation_type = match transition_type {
            TransitionType::FadeIn => AnimationType::Fade,
            TransitionType::FadeOut => AnimationType::Fade,
            TransitionType::SlideInLeft => AnimationType::Translate,
            TransitionType::SlideInRight => AnimationType::Translate,
            TransitionType::SlideInUp => AnimationType::Translate,
            TransitionType::SlideInDown => AnimationType::Translate,
            TransitionType::ScaleIn => AnimationType::Scale,
            TransitionType::ScaleOut => AnimationType::Scale,
            TransitionType::RotateIn => AnimationType::Rotate,
            TransitionType::RotateOut => AnimationType::Rotate,
            TransitionType::Custom(_) => AnimationType::Custom(0),
        };

        let animation = Animation::new(id, element_id, animation_type, duration);
        
        TransitionAnimation {
            animation,
            transition_type,
            from_rect,
            to_rect,
        }
    }

    pub fn get_animation(&self) -> &Animation {
        &self.animation
    }

    pub fn get_animation_mut(&mut self) -> &mut Animation {
        &mut self.animation
    }

    pub fn get_transition_type(&self) -> TransitionType {
        self.transition_type
    }

    pub fn get_from_rect(&self) -> super::framework::UIRect {
        self.from_rect
    }

    pub fn get_to_rect(&self) -> super::framework::UIRect {
        self.to_rect
    }

    pub fn start(&mut self) {
        self.animation.start();
    }

    pub fn update(&mut self) -> bool {
        self.animation.update()
    }

    pub fn get_progress(&self) -> f32 {
        self.animation.get_progress()
    }

    pub fn is_complete(&self) -> bool {
        self.animation.is_complete()
    }
}

// Property animation
pub struct PropertyAnimation {
    animation: Animation,
    property_type: PropertyType,
    element_id: UIElementId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyType {
    Opacity,
    PositionX,
    PositionY,
    Width,
    Height,
    Rotation,
    Scale,
    Color,
    Custom(u32),
}

impl PropertyAnimation {
    pub fn new(id: u32, element_id: UIElementId, property_type: PropertyType, duration: u64, start_value: f32, end_value: f32) -> Self {
        let animation_type = match property_type {
            PropertyType::Opacity => AnimationType::Fade,
            PropertyType::PositionX => AnimationType::Translate,
            PropertyType::PositionY => AnimationType::Translate,
            PropertyType::Width => AnimationType::Scale,
            PropertyType::Height => AnimationType::Scale,
            PropertyType::Rotation => AnimationType::Rotate,
            PropertyType::Scale => AnimationType::Scale,
            PropertyType::Color => AnimationType::Custom(0),
            PropertyType::Custom(_) => AnimationType::Custom(0),
        };

        let animation = Animation::new(id, element_id, animation_type, duration)
            .with_values(start_value, end_value);
        
        PropertyAnimation {
            animation,
            property_type,
            element_id,
        }
    }

    pub fn get_animation(&self) -> &Animation {
        &self.animation
    }

    pub fn get_animation_mut(&mut self) -> &mut Animation {
        &mut self.animation
    }

    pub fn get_property_type(&self) -> PropertyType {
        self.property_type
    }

    pub fn get_element_id(&self) -> UIElementId {
        self.element_id
    }

    pub fn start(&mut self) {
        self.animation.start();
    }

    pub fn update(&mut self) -> bool {
        self.animation.update()
    }

    pub fn get_progress(&self) -> f32 {
        self.animation.get_progress()
    }

    pub fn get_current_value(&self) -> f32 {
        self.animation.get_current_value()
    }

    pub fn is_complete(&self) -> bool {
        self.animation.is_complete()
    }
}

// Animation composition
pub struct AnimationComposition {
    animations: [Option<u32>; 20],
    num_animations: usize,
    composition_type: CompositionType,
    is_running: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositionType {
    Sequential,    // Run animations one after another
    Parallel,      // Run all animations simultaneously
    Staggered,     // Run animations with delays
}

impl AnimationComposition {
    pub fn new(composition_type: CompositionType) -> Self {
        AnimationComposition {
            animations: [None; 20],
            num_animations: 0,
            composition_type,
            is_running: false,
        }
    }

    pub fn add_animation(&mut self, animation_id: u32) -> Result<(), AnimationError> {
        if self.num_animations >= 20 {
            return Err(AnimationError::TooManyAnimations);
        }

        for i in 0..20 {
            if self.animations[i].is_none() {
                self.animations[i] = Some(animation_id);
                self.num_animations += 1;
                return Ok(());
            }
        }

        Err(AnimationError::TooManyAnimations)
    }

    pub fn start(&mut self, animation_manager: &mut AnimationManager) {
        self.is_running = true;

        match self.composition_type {
            CompositionType::Sequential => {
                // Start first animation
                if let Some(id) = self.animations[0] {
                    animation_manager.start_animation(id);
                }
            }
            CompositionType::Parallel => {
                // Start all animations simultaneously
                for i in 0..self.num_animations {
                    if let Some(id) = self.animations[i] {
                        animation_manager.start_animation(id);
                    }
                }
            }
            CompositionType::Staggered => {
                // Start animations with staggered delays
                let delay_step = 100; // 100ms between animations
                for i in 0..self.num_animations {
                    if let Some(id) = self.animations[i] {
                        let mut animation = animation_manager.get_animation_mut(id).unwrap();
                        animation.with_delay((i as u64) * delay_step);
                        animation_manager.start_animation(id);
                    }
                }
            }
        }
    }

    pub fn update(&mut self, animation_manager: &mut AnimationManager) -> bool {
        if !self.is_running {
            return false;
        }

        match self.composition_type {
            CompositionType::Sequential => {
                // Start next animation when current completes
                for i in 0..self.num_animations {
                    if let Some(id) = self.animations[i] {
                        if let Some(animation) = animation_manager.get_animation(id) {
                            if animation.is_complete() {
                                // Start next animation
                                if i + 1 < self.num_animations {
                                    if let Some(next_id) = self.animations[i + 1] {
                                        animation_manager.start_animation(next_id);
                                    }
                                }
                            } else {
                                animation.update();
                            }
                        }
                    }
                }
            }
            CompositionType::Parallel => {
                // Update all animations
                let all_complete = true;
                for i in 0..self.num_animations {
                    if let Some(id) = self.animations[i] {
                        if let Some(animation) = animation_manager.get_animation(id) {
                            if !animation.is_complete() {
                                animation.update();
                                all_complete = false;
                            }
                        }
                    }
                }

                if all_complete {
                    self.is_running = false;
                }
            }
            CompositionType::Staggered => {
                // Update all animations
                let all_complete = true;
                for i in 0..self.num_animations {
                    if let Some(id) = self.animations[i] {
                        if let Some(animation) = animation_manager.get_animation(id) {
                            if !animation.is_complete() {
                                animation.update();
                                all_complete = false;
                            }
                        }
                    }
                }

                if all_complete {
                    self.is_running = false;
                }
            }
        }

        self.is_running
    }

    pub fn is_complete(&self) -> bool {
        !self.is_running
    }

    pub fn clear(&mut self) {
        for i in 0..20 {
            self.animations[i] = None;
        }
        self.num_animations = 0;
        self.is_running = false;
    }

    pub fn len(&self) -> usize {
        self.num_animations
    }

    pub fn is_empty(&self) -> bool {
        self.num_animations == 0
    }
}

// Animation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationError {
    TooManyAnimations,
    AnimationNotFound,
    InvalidAnimation,
}

impl Default for AnimationComposition {
    fn default() -> Self {
        Self::new(CompositionType::Sequential)
    }
}
