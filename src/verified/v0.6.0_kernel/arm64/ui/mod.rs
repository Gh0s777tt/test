// UI module for VantisOS v0.6.0 ARM64 kernel
// Touch UI Framework

pub mod touch_event;
pub mod framework;
pub mod widgets;

// Re-export common types
pub use touch_event::{
    TouchPoint, TouchEvent, TouchEventType, GestureEvent, GestureType, GestureDirection,
    TouchEventError, TouchEventQueue, TouchEventDispatcher, TouchEventListener,
    TouchEventFilter, TouchEventManager,
};
