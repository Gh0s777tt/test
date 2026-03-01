// UI module for VantisOS v0.6.0 ARM64 kernel
// Touch UI Framework - Complete UI System

pub mod touch_event;
pub mod framework;
pub mod widgets;
pub mod event_routing;
pub mod system_ui;
pub mod app_framework;
pub mod gestures;
pub mod animations;

// Re-export touch event types
pub use touch_event::{
    TouchPoint, TouchEvent, TouchEventType, GestureEvent, GestureType, GestureDirection,
    TouchEventError, TouchEventQueue, TouchEventDispatcher, TouchEventListener,
    TouchEventFilter, TouchEventManager,
};

// Re-export framework types
pub use framework::{
    UIElement, UIElementId, UIElementType, UIElementState, UIRect, UIColor, BaseUIElement,
    UIContext, UIStateManager, UIRenderingPipeline, UIEventRouter,
};

// Re-export widget types
pub use widgets::{
    Button, Label, TextField, TextAlignment, WidgetStyle,
    LayoutManager, LayoutType, WidgetStyling,
};

// Re-export event routing types
pub use event_routing::{
    EventPhase, EventPropagationFlags, UIEvent, UIEventType,
    EventListener, EventListenerEntry, EventListenerManager,
    EventRoutingError, EventRouter, EventDelegation, EventSelector,
    EventFilter, EventPropagationController,
};

// UI framework version
pub const UI_FRAMEWORK_VERSION: &str = "0.1.0";
pub const UI_FRAMEWORK_NAME: &str = "VantisOS Touch UI Framework";
