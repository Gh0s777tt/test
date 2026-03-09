// Event Routing for VantisOS v0.6.0 ARM64 kernel
// Touch UI Framework - Event Routing

use super::touch_event::TouchEvent;
use super::framework::UIElementId;

// Event phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventPhase {
    Capturing,  // Event is being captured (root to target)
    AtTarget,   // Event has reached the target element
    Bubbling,   // Event is bubbling up (target to root)
}

// Event propagation flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventPropagationFlags {
    pub stop_propagation: bool,
    pub stop_immediate_propagation: bool,
    pub prevent_default: bool,
}

impl EventPropagationFlags {
    pub const fn new() -> Self {
        EventPropagationFlags {
            stop_propagation: false,
            stop_immediate_propagation: false,
            prevent_default: false,
        }
    }

    pub fn stop_propagation(&mut self) {
        self.stop_propagation = true;
    }

    pub fn stop_immediate_propagation(&mut self) {
        self.stop_immediate_propagation = true;
    }

    pub fn prevent_default(&mut self) {
        self.prevent_default = true;
    }

    pub fn is_propagation_stopped(&self) -> bool {
        self.stop_propagation || self.stop_immediate_propagation
    }
}

impl Default for EventPropagationFlags {
    fn default() -> Self {
        Self::new()
    }
}

// Event listener
pub type EventListener = fn(&mut UIEvent);

// UI Event
pub struct UIEvent {
    pub event_type: UIEventType,
    pub target_id: UIElementId,
    pub current_target_id: UIElementId,
    pub phase: EventPhase,
    pub propagation_flags: EventPropagationFlags,
    pub timestamp: u64,
    pub touch_event: Option<TouchEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UIEventType {
    TouchDown,
    TouchMove,
    TouchUp,
    TouchCancel,
    Click,
    Focus,
    Blur,
    KeyDown,
    KeyUp,
    Custom(u32),
}

impl UIEvent {
    pub fn new(event_type: UIEventType, target_id: UIElementId) -> Self {
        UIEvent {
            event_type,
            target_id,
            current_target_id: target_id,
            phase: EventPhase::AtTarget,
            propagation_flags: EventPropagationFlags::new(),
            timestamp: Self::get_timestamp(),
            touch_event: None,
        }
    }

    pub fn with_phase(mut self, phase: EventPhase) -> Self {
        self.phase = phase;
        self
    }

    pub fn with_current_target(mut self, current_target_id: UIElementId) -> Self {
        self.current_target_id = current_target_id;
        self
    }

    pub fn with_touch_event(mut self, touch_event: TouchEvent) -> Self {
        self.touch_event = Some(touch_event);
        self
    }

    fn get_timestamp() -> u64 {
        use core::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        COUNTER.fetch_add(1, Ordering::SeqCst)
    }

    pub fn stop_propagation(&mut self) {
        self.propagation_flags.stop_propagation();
    }

    pub fn stop_immediate_propagation(&mut self) {
        self.propagation_flags.stop_immediate_propagation();
    }

    pub fn prevent_default(&mut self) {
        self.propagation_flags.prevent_default();
    }

    pub fn is_propagation_stopped(&self) -> bool {
        self.propagation_flags.is_propagation_stopped()
    }

    pub fn is_default_prevented(&self) -> bool {
        self.propagation_flags.prevent_default
    }
}

// Event listener entry
pub struct EventListenerEntry {
    pub event_type: UIEventType,
    pub listener: EventListener,
    pub use_capture: bool,
    pub once: bool,
    pub passive: bool,
}

impl EventListenerEntry {
    pub fn new(event_type: UIEventType, listener: EventListener) -> Self {
        EventListenerEntry {
            event_type,
            listener,
            use_capture: false,
            once: false,
            passive: false,
        }
    }

    pub fn with_capture(mut self) -> Self {
        self.use_capture = true;
        self
    }

    pub fn with_once(mut self) -> Self {
        self.once = true;
        self
    }

    pub fn with_passive(mut self) -> Self {
        self.passive = true;
        self
    }
}

// Event listener manager
pub struct EventListenerManager {
    listeners: [Option<EventListenerEntry>; 50],
    num_listeners: usize,
}

impl EventListenerManager {
    pub const fn new() -> Self {
        EventListenerManager {
            listeners: [None; 50],
            num_listeners: 0,
        }
    }

    pub fn add_listener(&mut self, entry: EventListenerEntry) -> Result<(), EventRoutingError> {
        if self.num_listeners >= 50 {
            return Err(EventRoutingError::TooManyListeners);
        }

        for i in 0..50 {
            if self.listeners[i].is_none() {
                self.listeners[i] = Some(entry);
                self.num_listeners += 1;
                return Ok(());
            }
        }

        Err(EventRoutingError::TooManyListeners)
    }

    pub fn remove_listener(&mut self, event_type: UIEventType, listener: EventListener) -> bool {
        for i in 0..self.num_listeners {
            if let Some(ref entry) = self.listeners[i] {
                if entry.event_type == event_type && (entry.listener as *const fn(&mut UIEvent)) == (listener as *const fn(&mut UIEvent)) {
                    self.listeners[i] = None;
                    self.num_listeners -= 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn get_listeners(&self, event_type: UIEventType, phase: EventPhase) -> Vec<&EventListenerEntry> {
        let mut result = Vec::new();

        for i in 0..self.num_listeners {
            if let Some(ref entry) = self.listeners[i] {
                if entry.event_type == event_type {
                    match phase {
                        EventPhase::Capturing => {
                            if entry.use_capture {
                                result.push(entry);
                            }
                        }
                        EventPhase::AtTarget | EventPhase::Bubbling => {
                            if !entry.use_capture {
                                result.push(entry);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    pub fn clear(&mut self) {
        for i in 0..50 {
            self.listeners[i] = None;
        }
        self.num_listeners = 0;
    }

    pub fn len(&self) -> usize {
        self.num_listeners
    }

    pub fn is_empty(&self) -> bool {
        self.num_listeners == 0
    }
}

impl Default for EventListenerManager {
    fn default() -> Self {
        Self::new()
    }
}

// Event routing errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventRoutingError {
    TooManyListeners,
    InvalidElement,
    InvalidEvent,
}

// Event router
pub struct EventRouter {
    listener_manager: EventListenerManager,
}

impl EventRouter {
    pub const fn new() -> Self {
        EventRouter {
            listener_manager: EventListenerManager::new(),
        }
    }

    pub fn add_listener(&mut self, entry: EventListenerEntry) -> Result<(), EventRoutingError> {
        self.listener_manager.add_listener(entry)
    }

    pub fn remove_listener(&mut self, event_type: UIEventType, listener: EventListener) -> bool {
        self.listener_manager.remove_listener(event_type, listener)
    }

    pub fn dispatch_event(&mut self, event: &mut UIEvent) {
        // Phase 1: Capturing phase (root to target)
        event.phase = EventPhase::Capturing;
        let capturing_listeners = self.listener_manager.get_listeners(event.event_type, EventPhase::Capturing);
        
        for listener in capturing_listeners {
            (listener.listener)(event);
            if event.is_propagation_stopped() {
                return;
            }
        }

        // Phase 2: At target phase
        event.phase = EventPhase::AtTarget;
        let at_target_listeners = self.listener_manager.get_listeners(event.event_type, EventPhase::AtTarget);
        
        for listener in at_target_listeners {
            (listener.listener)(event);
            if event.is_propagation_stopped() {
                return;
            }
        }

        // Phase 3: Bubbling phase (target to root)
        event.phase = EventPhase::Bubbling;
        let bubbling_listeners = self.listener_manager.get_listeners(event.event_type, EventPhase::Bubbling);
        
        for listener in bubbling_listeners {
            (listener.listener)(event);
            if event.is_propagation_stopped() {
                return;
            }
        }
    }

    pub fn dispatch_event_immediate(&mut self, event: &mut UIEvent) {
        // Dispatch only to current target, no propagation
        event.phase = EventPhase::AtTarget;
        let listeners = self.listener_manager.get_listeners(event.event_type, EventPhase::AtTarget);
        
        for listener in listeners {
            (listener.listener)(event);
            if event.is_propagation_stopped() {
                return;
            }
        }
    }

    pub fn clear(&mut self) {
        self.listener_manager.clear();
    }

    pub fn len(&self) -> usize {
        self.listener_manager.len()
    }

    pub fn is_empty(&self) -> bool {
        self.listener_manager.is_empty()
    }
}

impl Default for EventRouter {
    fn default() -> Self {
        Self::new()
    }
}

// Event delegation
pub struct EventDelegation {
    delegate_id: UIElementId,
    selector: EventSelector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSelector {
    All,
    ById(UIElementId),
    ByType(u32),  // UIElementType as u32
    ByClass(u32), // Custom class ID
}

impl EventDelegation {
    pub fn new(delegate_id: UIElementId, selector: EventSelector) -> Self {
        EventDelegation {
            delegate_id,
            selector,
        }
    }

    pub fn matches(&self, element_id: UIElementId, element_type: u32) -> bool {
        match self.selector {
            EventSelector::All => true,
            EventSelector::ById(id) => id == element_id,
            EventSelector::ByType(t) => t == element_type,
            EventSelector::ByClass(_) => false, // TODO: Implement class matching
        }
    }
}

// Event filter
pub struct EventFilter {
    enabled: bool,
    filter_fn: Option<fn(&UIEvent) -> bool>,
}

impl EventFilter {
    pub const fn new() -> Self {
        EventFilter {
            enabled: true,
            filter_fn: None,
        }
    }

    pub fn with_filter(mut self, filter_fn: fn(&UIEvent) -> bool) -> Self {
        self.filter_fn = Some(filter_fn);
        self
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn filter(&self, event: &UIEvent) -> bool {
        if !self.enabled {
            return true;
        }

        if let Some(filter_fn) = self.filter_fn {
            filter_fn(event)
        } else {
            true
        }
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new()
    }
}

// Event propagation controller
pub struct EventPropagationController {
    router: EventRouter,
    filter: EventFilter,
}

impl EventPropagationController {
    pub const fn new() -> Self {
        EventPropagationController {
            router: EventRouter::new(),
            filter: EventFilter::new(),
        }
    }

    pub fn add_listener(&mut self, entry: EventListenerEntry) -> Result<(), EventRoutingError> {
        self.router.add_listener(entry)
    }

    pub fn remove_listener(&mut self, event_type: UIEventType, listener: EventListener) -> bool {
        self.router.remove_listener(event_type, listener)
    }

    pub fn dispatch_event(&mut self, event: &mut UIEvent) {
        if self.filter.filter(event) {
            self.router.dispatch_event(event);
        }
    }

    pub fn get_filter(&mut self) -> &mut EventFilter {
        &mut self.filter
    }

    pub fn get_router(&mut self) -> &mut EventRouter {
        &mut self.router
    }
}

impl Default for EventPropagationController {
    fn default() -> Self {
        Self::new()
    }
}
