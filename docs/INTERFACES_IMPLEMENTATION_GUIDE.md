# Interfejsy (Interfaces) Implementation Guide

## Executive Summary

This guide provides a comprehensive implementation plan for the user interface system in VantisOS, including the Horizon UI framework, input handling, accessibility features, and multi-language support.

**Implementation Timeline**: 3 days  
**Complexity**: Medium  
**Dependencies**: Vantis Core, Flux Engine, Direct Metal  
**Security Level**: High (EAL 6+)

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Technical Requirements](#technical-requirements)
3. [Implementation Plan](#implementation-plan)
4. [Accessibility Features](#accessibility-features)
5. [Multi-language Support](#multi-language-support)
6. [Performance Targets](#performance-targets)
7. [Testing Strategy](#testing-strategy)
8. [Code Examples](#code-examples)
9. [Troubleshooting](#troubleshooting)

---

## Architecture Overview

### System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     VantisOS Kernel                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   IPC Core   │  │  Scheduler   │  │ Memory Mgmt  │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Horizon UI Framework                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Widget Tree │  │  Layout      │  │  Rendering   │      │
│  │              │  │  Engine      │  │  Pipeline    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Input & Accessibility Layer                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  Input       │  │  Accessibility│  │  I18n        │      │
│  │  Handler     │  │  Manager     │  │  Engine      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              User Applications                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  System UI   │  │  Apps        │  │  Settings    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### Key Components

1. **Widget Tree**: Hierarchical widget structure
2. **Layout Engine**: Flexible layout system
3. **Rendering Pipeline**: GPU-accelerated rendering
4. **Input Handler**: Unified input processing
5. **Accessibility Manager**: A11y features and screen reader support
6. **I18n Engine**: Internationalization and localization

---

## Technical Requirements

### Supported Input Devices

- **Mouse**: Pointer, scroll wheel, buttons
- **Keyboard**: Full keyboard support with shortcuts
- **Touch**: Multi-touch gestures (tap, swipe, pinch, rotate)
- **Gamepad**: Standard gamepad support
- **Pen/Stylus**: Pressure-sensitive input

### Accessibility Standards

- **WCAG 2.1**: Level AA compliance
- **Screen Reader**: NVDA, JAWS, VoiceOver compatibility
- **Keyboard Navigation**: Full keyboard accessibility
- **High Contrast**: High contrast mode support
- **Text Scaling**: 100%-200% text scaling

### Language Support

- **Languages**: 100+ languages
- **Scripts**: Latin, Cyrillic, Arabic, Hebrew, CJK, etc.
- **RTL**: Right-to-left text support
- **Unicode**: Unicode 16.0 full support

### Software Dependencies

```toml
[dependencies]
# UI Framework
horizon-ui = { version = "0.5.0", features = ["full"] }

# Graphics
vulkan = "1.3"
direct-metal = "0.4"

# Input
input-handler = { version = "0.3.0" }

# Accessibility
accessibility = { version = "0.2.0", features = ["screen-reader"] }

# Internationalization
i18n = { version = "0.4.0", features = ["full"] }
unicode-segmentation = "1.11"
```

---

## Implementation Plan

### Day 1: Widget System and Layout Engine

**Tasks:**
1. Implement widget tree structure
2. Create layout engine
3. Implement basic widgets
4. Add event handling

**Code Structure:**
```rust
// src/horizon/widget.rs
use std::sync::Arc;

pub trait Widget: Send + Sync {
    fn render(&self, renderer: &mut Renderer, rect: Rect) -> Result<(), RenderError>;
    fn handle_event(&mut self, event: &Event) -> Result<(), EventError>;
    fn layout(&mut self, constraints: LayoutConstraints) -> Size;
    fn children(&self) -> &[Arc<dyn Widget>];
}

pub struct Container {
    children: Vec<Arc<dyn Widget>>,
    layout: Layout,
    padding: Padding,
    margin: Margin,
}

impl Widget for Container {
    fn render(&self, renderer: &mut Renderer, rect: Rect) -> Result<(), RenderError> {
        // Render background
        renderer.fill_rect(rect, self.background_color)?;
        
        // Render children
        let child_rects = self.layout.calculate(rect, &self.children, self.padding);
        
        for (child, child_rect) in self.children.iter().zip(child_rects) {
            child.render(renderer, child_rect)?;
        }
        
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), EventError> {
        // Propagate event to children
        for child in &mut self.children {
            child.handle_event(event)?;
        }
        Ok(())
    }

    fn layout(&mut self, constraints: LayoutConstraints) -> Size {
        // Calculate size based on children
        let child_sizes: Vec<Size> = self.children
            .iter()
            .map(|c| c.layout(constraints))
            .collect();
        
        self.layout.calculate_size(&child_sizes, self.padding, self.margin)
    }

    fn children(&self) -> &[Arc<dyn Widget>] {
        &self.children
    }
}

pub struct Button {
    text: String,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    state: ButtonState,
    style: ButtonStyle,
}

#[derive(Clone, Copy)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

impl Widget for Button {
    fn render(&self, renderer: &mut Renderer, rect: Rect) -> Result<(), RenderError> {
        // Render button background
        let color = match self.state {
            ButtonState::Normal => self.style.normal_color,
            ButtonState::Hovered => self.style.hover_color,
            ButtonState::Pressed => self.style.pressed_color,
            ButtonState::Disabled => self.style.disabled_color,
        };
        
        renderer.fill_rect(rect, color)?;
        
        // Render border
        renderer.stroke_rect(rect, self.style.border_color, self.style.border_width)?;
        
        // Render text
        let text_rect = rect.inset(self.style.padding);
        renderer.render_text(&self.text, text_rect, self.style.text_color)?;
        
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), EventError> {
        match event {
            Event::MouseEnter => {
                self.state = ButtonState::Hovered;
            }
            Event::MouseLeave => {
                self.state = ButtonState::Normal;
            }
            Event::MouseDown(_) => {
                self.state = ButtonState::Pressed;
            }
            Event::MouseUp(_) => {
                if self.state == ButtonState::Pressed {
                    self.state = ButtonState::Hovered;
                    if let Some(ref callback) = self.on_click {
                        callback();
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn layout(&mut self, constraints: LayoutConstraints) -> Size {
        // Calculate button size
        let text_size = self.measure_text(&self.text);
        let padding = self.style.padding;
        
        Size {
            width: text_size.width + padding.left + padding.right,
            height: text_size.height + padding.top + padding.bottom,
        }
    }

    fn children(&self) -> &[Arc<dyn Widget>] {
        &[]
    }
}

impl Button {
    pub fn new(text: String) -> Self {
        Button {
            text,
            on_click: None,
            state: ButtonState::Normal,
            style: ButtonStyle::default(),
        }
    }

    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    fn measure_text(&self, text: &str) -> Size {
        // Measure text size
        // Implementation details
        Size { width: 100, height: 30 }
    }
}

// Layout Engine
pub enum Layout {
    Row { spacing: f32, alignment: Alignment },
    Column { spacing: f32, alignment: Alignment },
    Grid { columns: usize, spacing: f32 },
    Flex { direction: FlexDirection, wrap: bool },
}

impl Layout {
    pub fn calculate(&self, parent_rect: Rect, children: &[Arc<dyn Widget>], padding: Padding) -> Vec<Rect> {
        match self {
            Layout::Row { spacing, alignment } => {
                self.calculate_row(parent_rect, children, *spacing, *alignment, padding)
            }
            Layout::Column { spacing, alignment } => {
                self.calculate_column(parent_rect, children, *spacing, *alignment, padding)
            }
            Layout::Grid { columns, spacing } => {
                self.calculate_grid(parent_rect, children, *columns, *spacing, padding)
            }
            Layout::Flex { direction, wrap } => {
                self.calculate_flex(parent_rect, children, *direction, *wrap, padding)
            }
        }
    }

    fn calculate_row(
        &self,
        parent_rect: Rect,
        children: &[Arc<dyn Widget>],
        spacing: f32,
        alignment: Alignment,
        padding: Padding,
    ) -> Vec<Rect> {
        let mut rects = Vec::new();
        let mut x = parent_rect.x + padding.left;
        let available_width = parent_rect.width - padding.left - padding.right;
        
        // Calculate total width
        let total_spacing = spacing * (children.len() - 1) as f32;
        let child_width = (available_width - total_spacing) / children.len() as f32;
        
        for child in children {
            let rect = Rect {
                x,
                y: parent_rect.y + padding.top,
                width: child_width,
                height: parent_rect.height - padding.top - padding.bottom,
            };
            
            rects.push(rect);
            x += child_width + spacing;
        }
        
        rects
    }

    fn calculate_column(
        &self,
        parent_rect: Rect,
        children: &[Arc<dyn Widget>],
        spacing: f32,
        alignment: Alignment,
        padding: Padding,
    ) -> Vec<Rect> {
        let mut rects = Vec::new();
        let mut y = parent_rect.y + padding.top;
        let available_height = parent_rect.height - padding.top - padding.bottom;
        
        // Calculate total height
        let total_spacing = spacing * (children.len() - 1) as f32;
        let child_height = (available_height - total_spacing) / children.len() as f32;
        
        for child in children {
            let rect = Rect {
                x: parent_rect.x + padding.left,
                y,
                width: parent_rect.width - padding.left - padding.right,
                height: child_height,
            };
            
            rects.push(rect);
            y += child_height + spacing;
        }
        
        rects
    }

    fn calculate_grid(
        &self,
        parent_rect: Rect,
        children: &[Arc<dyn Widget>],
        columns: usize,
        spacing: f32,
        padding: Padding,
    ) -> Vec<Rect> {
        let mut rects = Vec::new();
        let cell_width = (parent_rect.width - padding.left - padding.right - spacing * (columns - 1) as f32) / columns as f32;
        let cell_height = cell_width; // Square cells
        
        for (i, child) in children.iter().enumerate() {
            let row = i / columns;
            let col = i % columns;
            
            let rect = Rect {
                x: parent_rect.x + padding.left + col as f32 * (cell_width + spacing),
                y: parent_rect.y + padding.top + row as f32 * (cell_height + spacing),
                width: cell_width,
                height: cell_height,
            };
            
            rects.push(rect);
        }
        
        rects
    }

    fn calculate_flex(
        &self,
        parent_rect: Rect,
        children: &[Arc<dyn Widget>],
        direction: FlexDirection,
        wrap: bool,
        padding: Padding,
    ) -> Vec<Rect> {
        // Flex layout implementation
        // Implementation details
        Vec::new()
    }

    pub fn calculate_size(&self, child_sizes: &[Size], padding: Padding, margin: Margin) -> Size {
        match self {
            Layout::Row { spacing, .. } => {
                let total_width: f32 = child_sizes.iter().map(|s| s.width).sum::<f32>()
                    + spacing * (child_sizes.len() - 1) as f32;
                let max_height: f32 = child_sizes.iter().map(|s| s.height).fold(0.0, f32::max);
                
                Size {
                    width: total_width + padding.left + padding.right + margin.left + margin.right,
                    height: max_height + padding.top + padding.bottom + margin.top + margin.bottom,
                }
            }
            Layout::Column { spacing, .. } => {
                let max_width: f32 = child_sizes.iter().map(|s| s.width).fold(0.0, f32::max);
                let total_height: f32 = child_sizes.iter().map(|s| s.height).sum::<f32>()
                    + spacing * (child_sizes.len() - 1) as f32;
                
                Size {
                    width: max_width + padding.left + padding.right + margin.left + margin.right,
                    height: total_height + padding.top + padding.bottom + margin.top + margin.bottom,
                }
            }
            _ => Size { width: 100.0, height: 100.0 },
        }
    }
}
```

### Day 2: Input Handling and Accessibility

**Tasks:**
1. Implement input handler
2. Add keyboard navigation
3. Implement screen reader support
4. Add high contrast mode

**Code Structure:**
```rust
// src/horizon/input.rs
use std::collections::HashMap;

pub struct InputHandler {
    keyboard_state: KeyboardState,
    mouse_state: MouseState,
    touch_state: TouchState,
    focus_manager: FocusManager,
    shortcuts: HashMap<Shortcut, Box<dyn ShortcutHandler>>,
}

impl InputHandler {
    pub fn new() -> Result<Self, InputError> {
        Ok(InputHandler {
            keyboard_state: KeyboardState::new(),
            mouse_state: MouseState::new(),
            touch_state: TouchState::new(),
            focus_manager: FocusManager::new(),
            shortcuts: HashMap::new(),
        })
    }

    pub fn handle_input(&mut self, input: Input) -> Result<Vec<Event>, InputError> {
        let mut events = Vec::new();
        
        match input {
            Input::Keyboard(keyboard_event) => {
                events.extend(self.handle_keyboard(keyboard_event)?);
            }
            Input::Mouse(mouse_event) => {
                events.extend(self.handle_mouse(mouse_event)?);
            }
            Input::Touch(touch_event) => {
                events.extend(self.handle_touch(touch_event)?);
            }
            Input::Gamepad(gamepad_event) => {
                events.extend(self.handle_gamepad(gamepad_event)?);
            }
        }
        
        events
    }

    fn handle_keyboard(&mut self, event: KeyboardEvent) -> Result<Vec<Event>, InputError> {
        let mut events = Vec::new();
        
        match event.kind {
            KeyEventKind::Pressed => {
                // Update keyboard state
                self.keyboard_state.press_key(event.key);
                
                // Check for shortcuts
                if let Some(shortcut_events) = self.check_shortcuts(&event)? {
                    events.extend(shortcut_events);
                }
                
                // Handle focus navigation
                if let Some(focus_event) = self.handle_focus_navigation(&event)? {
                    events.push(focus_event);
                }
                
                events.push(Event::KeyDown(event.key));
            }
            KeyEventKind::Released => {
                self.keyboard_state.release_key(event.key);
                events.push(Event::KeyUp(event.key));
            }
            KeyEventKind::Repeat => {
                events.push(Event::KeyRepeat(event.key));
            }
        }
        
        Ok(events)
    }

    fn handle_mouse(&mut self, event: MouseEvent) -> Result<Vec<Event>, InputError> {
        let mut events = Vec::new();
        
        match event.kind {
            MouseEventKind::Moved => {
                let old_position = self.mouse_state.position;
                self.mouse_state.position = event.position;
                
                events.push(Event::MouseMove {
                    old_position,
                    new_position: event.position,
                });
                
                // Check for hover
                if let Some(hover_event) = self.check_hover(event.position)? {
                    events.push(hover_event);
                }
            }
            MouseEventKind::Pressed(button) => {
                self.mouse_state.press_button(button);
                events.push(Event::MouseDown {
                    button,
                    position: event.position,
                });
            }
            MouseEventKind::Released(button) => {
                self.mouse_state.release_button(button);
                events.push(Event::MouseUp {
                    button,
                    position: event.position,
                });
                
                // Check for click
                if let Some(click_event) = self.check_click(button, event.position)? {
                    events.push(click_event);
                }
            }
            MouseEventKind::Wheel(delta) => {
                events.push(Event::MouseWheel {
                    delta,
                    position: event.position,
                });
            }
        }
        
        Ok(events)
    }

    fn handle_touch(&mut self, event: TouchEvent) -> Result<Vec<Event>, InputError> {
        let mut events = Vec::new();
        
        match event.kind {
            TouchEventKind::Started => {
                self.touch_state.start_touch(event.id, event.position);
                events.push(Event::TouchStart {
                    id: event.id,
                    position: event.position,
                });
            }
            TouchEventKind::Moved => {
                let old_position = self.touch_state.get_position(event.id);
                self.touch_state.move_touch(event.id, event.position);
                
                events.push(Event::TouchMove {
                    id: event.id,
                    old_position,
                    new_position: event.position,
                });
            }
            TouchEventKind::Ended => {
                let position = self.touch_state.get_position(event.id);
                self.touch_state.end_touch(event.id);
                
                events.push(Event::TouchEnd {
                    id: event.id,
                    position,
                });
                
                // Check for tap
                if let Some(tap_event) = self.check_tap(event.id, position)? {
                    events.push(tap_event);
                }
            }
            TouchEventKind::Cancelled => {
                self.touch_state.cancel_touch(event.id);
                events.push(Event::TouchCancel { id: event.id });
            }
        }
        
        Ok(events)
    }

    fn handle_gamepad(&mut self, event: GamepadEvent) -> Result<Vec<Event>, InputError> {
        let mut events = Vec::new();
        
        match event.kind {
            GamepadEventKind::ButtonPressed(button) => {
                events.push(Event::GamepadButtonDown { button });
            }
            GamepadEventKind::ButtonReleased(button) => {
                events.push(Event::GamepadButtonUp { button });
            }
            GamepadEventKind::AxisMoved(axis, value) => {
                events.push(Event::GamepadAxisMove { axis, value });
            }
        }
        
        Ok(events)
    }

    pub fn register_shortcut<F>(&mut self, shortcut: Shortcut, handler: F)
    where
        F: Fn() -> Vec<Event> + Send + Sync + 'static,
    {
        self.shortcuts.insert(shortcut, Box::new(handler));
    }

    fn check_shortcuts(&self, event: &KeyboardEvent) -> Result<Option<Vec<Event>>, InputError> {
        let shortcut = Shortcut::from_keyboard_event(event);
        
        if let Some(handler) = self.shortcuts.get(&shortcut) {
            Ok(Some(handler()))
        } else {
            Ok(None)
        }
    }

    fn handle_focus_navigation(&mut self, event: &KeyboardEvent) -> Result<Option<Event>, InputError> {
        match event.key {
            Key::Tab => {
                if event.modifiers.contains(KeyModifiers::SHIFT) {
                    Ok(Some(Event::FocusPrevious))
                } else {
                    Ok(Some(Event::FocusNext))
                }
            }
            Key::ArrowUp => Ok(Some(Event::FocusUp)),
            Key::ArrowDown => Ok(Some(Event::FocusDown)),
            Key::ArrowLeft => Ok(Some(Event::FocusLeft)),
            Key::ArrowRight => Ok(Some(Event::FocusRight)),
            Key::Enter | Key::Space => Ok(Some(Event::FocusActivate)),
            Key::Escape => Ok(Some(Event::FocusCancel)),
            _ => Ok(None),
        }
    }

    fn check_hover(&self, position: Point) -> Result<Option<Event>, InputError> {
        // Check if position is over any widget
        // Implementation details
        Ok(None)
    }

    fn check_click(&self, button: MouseButton, position: Point) -> Result<Option<Event>, InputError> {
        // Check if this is a click (press and release on same widget)
        // Implementation details
        Ok(None)
    }

    fn check_tap(&self, id: u64, position: Point) -> Result<Option<Event>, InputError> {
        // Check if this is a tap (quick touch start and end)
        // Implementation details
        Ok(None)
    }
}

// Focus Manager
pub struct FocusManager {
    focus_chain: Vec<Focusable>,
    current_focus: Option<usize>,
}

impl FocusManager {
    pub fn new() -> Self {
        FocusManager {
            focus_chain: Vec::new(),
            current_focus: None,
        }
    }

    pub fn add_focusable(&mut self, focusable: Focusable) {
        self.focus_chain.push(focusable);
    }

    pub fn focus_next(&mut self) -> Option<Focusable> {
        if self.focus_chain.is_empty() {
            return None;
        }
        
        let next = match self.current_focus {
            Some(index) => (index + 1) % self.focus_chain.len(),
            None => 0,
        };
        
        self.current_focus = Some(next);
        Some(self.focus_chain[next].clone())
    }

    pub fn focus_previous(&mut self) -> Option<Focusable> {
        if self.focus_chain.is_empty() {
            return None;
        }
        
        let prev = match self.current_focus {
            Some(index) => {
                if index == 0 {
                    self.focus_chain.len() - 1
                } else {
                    index - 1
                }
            }
            None => self.focus_chain.len() - 1,
        };
        
        self.current_focus = Some(prev);
        Some(self.focus_chain[prev].clone())
    }
}

// Accessibility Manager
pub struct AccessibilityManager {
    screen_reader_enabled: bool,
    high_contrast_enabled: bool,
    text_scale: f32,
    focus_announcer: FocusAnnouncer,
}

impl AccessibilityManager {
    pub fn new() -> Result<Self, AccessibilityError> {
        Ok(AccessibilityManager {
            screen_reader_enabled: false,
            high_contrast_enabled: false,
            text_scale: 1.0,
            focus_announcer: FocusAnnouncer::new()?,
        })
    }

    pub fn enable_screen_reader(&mut self) -> Result<(), AccessibilityError> {
        self.screen_reader_enabled = true;
        Ok(())
    }

    pub fn disable_screen_reader(&mut self) -> Result<(), AccessibilityError> {
        self.screen_reader_enabled = false;
        Ok(())
    }

    pub fn set_high_contrast(&mut self, enabled: bool) -> Result<(), AccessibilityError> {
        self.high_contrast_enabled = enabled;
        Ok(())
    }

    pub fn set_text_scale(&mut self, scale: f32) -> Result<(), AccessibilityError> {
        self.text_scale = scale.clamp(1.0, 2.0);
        Ok(())
    }

    pub fn announce(&self, text: &str) -> Result<(), AccessibilityError> {
        if self.screen_reader_enabled {
            self.focus_announcer.announce(text)?;
        }
        Ok(())
    }

    pub fn announce_focus_change(&self, widget: &dyn Widget) -> Result<(), AccessibilityError> {
        if self.screen_reader_enabled {
            let text = self.get_widget_description(widget)?;
            self.announce(&text)?;
        }
        Ok(())
    }

    fn get_widget_description(&self, widget: &dyn Widget) -> Result<String, AccessibilityError> {
        // Get accessible description of widget
        // Implementation details
        Ok("Widget".to_string())
    }
}

pub struct FocusAnnouncer {
    // Screen reader integration
}

impl FocusAnnouncer {
    pub fn new() -> Result<Self, AccessibilityError> {
        Ok(FocusAnnouncer {})
    }

    pub fn announce(&self, text: &str) -> Result<(), AccessibilityError> {
        // Send text to screen reader
        // Implementation details
        Ok(())
    }
}
```

### Day 3: Internationalization and Integration

**Tasks:**
1. Implement I18n engine
2. Add RTL support
3. Integrate all components
4. Add theme system

**Code Structure:**
```rust
// src/horizon/i18n.rs
use std::collections::HashMap;

pub struct I18nEngine {
    current_locale: String,
    translations: HashMap<String, TranslationSet>,
    rtl_locales: HashSet<String>,
}

pub struct TranslationSet {
    messages: HashMap<String, String>,
    plural_rules: PluralRules,
}

impl I18nEngine {
    pub fn new() -> Result<Self, I18nError> {
        let mut engine = I18nEngine {
            current_locale: "en".to_string(),
            translations: HashMap::new(),
            rtl_locales: HashSet::new(),
        };
        
        // Load RTL locales
        engine.load_rtl_locales()?;
        
        Ok(engine)
    }

    pub fn set_locale(&mut self, locale: &str) -> Result<(), I18nError> {
        self.current_locale = locale.to_string();
        Ok(())
    }

    pub fn get_translation(&self, key: &str) -> String {
        if let Some(translation_set) = self.translations.get(&self.current_locale) {
            if let Some(message) = translation_set.messages.get(key) {
                return message.clone();
            }
        }
        
        // Fallback to English
        if let Some(translation_set) = self.translations.get("en") {
            if let Some(message) = translation_set.messages.get(key) {
                return message.clone();
            }
        }
        
        // Return key as fallback
        key.to_string()
    }

    pub fn get_translation_plural(&self, key: &str, count: usize) -> String {
        let translation_set = self.translations.get(&self.current_locale)
            .or_else(|| self.translations.get("en"));
        
        if let Some(translation_set) = translation_set {
            let plural_form = translation_set.plural_rules.get_form(count);
            let plural_key = format!("{}.{}", key, plural_form);
            
            if let Some(message) = translation_set.messages.get(&plural_key) {
                return message.replace("{n}", &count.to_string());
            }
        }
        
        key.to_string()
    }

    pub fn is_rtl(&self) -> bool {
        self.rtl_locales.contains(&self.current_locale)
    }

    pub fn load_translations(&mut self, locale: &str, translations: HashMap<String, String>) -> Result<(), I18nError> {
        let plural_rules = PluralRules::for_locale(locale)?;
        
        self.translations.insert(
            locale.to_string(),
            TranslationSet {
                messages: translations,
                plural_rules,
            }
        );
        
        Ok(())
    }

    fn load_rtl_locales(&mut self) -> Result<(), I18nError> {
        // Arabic, Hebrew, Persian, Urdu, etc.
        self.rtl_locales.insert("ar".to_string());
        self.rtl_locales.insert("he".to_string());
        self.rtl_locales.insert("fa".to_string());
        self.rtl_locales.insert("ur".to_string());
        
        Ok(())
    }
}

pub enum PluralForm {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

pub struct PluralRules {
    locale: String,
}

impl PluralRules {
    pub fn for_locale(locale: &str) -> Result<Self, I18nError> {
        Ok(PluralRules {
            locale: locale.to_string(),
        })
    }

    pub fn get_form(&self, count: usize) -> PluralForm {
        match self.locale.as_str() {
            "en" | "de" | "fr" => {
                if count == 1 {
                    PluralForm::One
                } else {
                    PluralForm::Other
                }
            }
            "ar" => {
                if count == 0 {
                    PluralForm::Zero
                } else if count == 1 {
                    PluralForm::One
                } else if count == 2 {
                    PluralForm::Two
                } else if count >= 3 && count <= 10 {
                    PluralForm::Few
                } else if count >= 11 && count <= 99 {
                    PluralForm::Many
                } else {
                    PluralForm::Other
                }
            }
            "pl" => {
                if count == 1 {
                    PluralForm::One
                } else if count % 10 >= 2 && count % 10 <= 4 && (count % 100 < 10 || count % 100 >= 20) {
                    PluralForm::Few
                } else {
                    PluralForm::Many
                }
            }
            _ => PluralForm::Other,
        }
    }
}

// Theme System
pub struct Theme {
    colors: ColorPalette,
    typography: Typography,
    spacing: Spacing,
    effects: Effects,
}

pub struct ColorPalette {
    primary: Color,
    secondary: Color,
    background: Color,
    surface: Color,
    error: Color,
    warning: Color,
    success: Color,
    info: Color,
}

impl Theme {
    pub fn light() -> Self {
        Theme {
            colors: ColorPalette {
                primary: Color::rgb(0x2196F3),
                secondary: Color::rgb(0xFF9800),
                background: Color::rgb(0xFFFFFF),
                surface: Color::rgb(0xF5F5F5),
                error: Color::rgb(0xF44336),
                warning: Color::rgb(0xFF9800),
                success: Color::rgb(0x4CAF50),
                info: Color::rgb(0x2196F3),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            effects: Effects::default(),
        }
    }

    pub fn dark() -> Self {
        Theme {
            colors: ColorPalette {
                primary: Color::rgb(0x64B5F6),
                secondary: Color::rgb(0xFFB74D),
                background: Color::rgb(0x121212),
                surface: Color::rgb(0x1E1E1E),
                error: Color::rgb(0xEF5350),
                warning: Color::rgb(0xFFB74D),
                success: Color::rgb(0x66BB6A),
                info: Color::rgb(0x64B5F6),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            effects: Effects::default(),
        }
    }

    pub fn high_contrast() -> Self {
        Theme {
            colors: ColorPalette {
                primary: Color::rgb(0xFFFF00),
                secondary: Color::rgb(0x00FFFF),
                background: Color::rgb(0x000000),
                surface: Color::rgb(0x000000),
                error: Color::rgb(0xFF0000),
                warning: Color::rgb(0xFFFF00),
                success: Color::rgb(0x00FF00),
                info: Color::rgb(0x00FFFF),
            },
            typography: Typography::default(),
            spacing: Spacing::default(),
            effects: Effects::default(),
        }
    }
}
```

---

## Accessibility Features

### Screen Reader Support

```rust
// Example: Accessible widget
pub struct AccessibleButton {
    button: Button,
    accessible_label: String,
    accessible_description: String,
    accessible_role: AccessibleRole,
}

impl AccessibleButton {
    pub fn new(text: String, label: String) -> Self {
        AccessibleButton {
            button: Button::new(text),
            accessible_label: label,
            accessible_description: String::new(),
            accessible_role: AccessibleRole::Button,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.accessible_description = description;
        self
    }
}

impl Widget for AccessibleButton {
    fn render(&self, renderer: &mut Renderer, rect: Rect) -> Result<(), RenderError> {
        self.button.render(renderer, rect)
    }

    fn handle_event(&mut self, event: &Event) -> Result<(), EventError> {
        self.button.handle_event(event)
    }

    fn layout(&mut self, constraints: LayoutConstraints) -> Size {
        self.button.layout(constraints)
    }

    fn children(&self) -> &[Arc<dyn Widget>] {
        self.button.children()
    }
}

impl Accessible for AccessibleButton {
    fn accessible_label(&self) -> String {
        self.accessible_label.clone()
    }

    fn accessible_description(&self) -> String {
        self.accessible_description.clone()
    }

    fn accessible_role(&self) -> AccessibleRole {
        self.accessible_role
    }
}

pub trait Accessible {
    fn accessible_label(&self) -> String;
    fn accessible_description(&self) -> String;
    fn accessible_role(&self) -> AccessibleRole;
}

pub enum AccessibleRole {
    Button,
    TextField,
    CheckBox,
    RadioButton,
    ComboBox,
    ListBox,
    MenuItem,
    Tab,
    Window,
    Dialog,
    Alert,
}
```

---

## Multi-language Support

### RTL Text Rendering

```rust
// Example: RTL-aware text widget
pub struct TextWidget {
    text: String,
    direction: TextDirection,
    alignment: TextAlignment,
}

pub enum TextDirection {
    LTR,
    RTL,
    Auto,
}

impl TextWidget {
    pub fn new(text: String) -> Self {
        TextWidget {
            text,
            direction: TextDirection::Auto,
            alignment: TextAlignment::Start,
        }
    }

    pub fn with_direction(mut self, direction: TextDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn render(&self, renderer: &mut Renderer, rect: Rect) -> Result<(), RenderError> {
        let direction = match self.direction {
            TextDirection::LTR => TextDirection::LTR,
            TextDirection::RTL => TextDirection::RTL,
            TextDirection::Auto => {
                if self.is_rtl_text(&self.text) {
                    TextDirection::RTL
                } else {
                    TextDirection::LTR
                }
            }
        };

        renderer.render_text_with_direction(&self.text, rect, direction, self.alignment)
    }

    fn is_rtl_text(&self, text: &str) -> bool {
        // Check if text contains RTL characters
        text.chars().any(|c| {
            matches!(c,
                '\u{0590}'..='\u{05FF}' | // Hebrew
                '\u{0600}'..='\u{06FF}' | // Arabic
                '\u{0750}'..='\u{077F}' | // Arabic Supplement
                '\u{FB50}'..='\u{FDFF}' | // Arabic Presentation Forms-A
                '\u{FE70}'..='\u{FEFF}'   // Arabic Presentation Forms-B
            )
        })
    }
}
```

---

## Performance Targets

### Rendering Performance

| Metric | Target | Measurement |
|--------|--------|-------------|
| Frame Rate | 60 FPS | Smooth UI rendering |
| Frame Time | <16ms | Time per frame |
| Widget Layout | <1ms | Time to calculate layout |
| Event Processing | <100μs | Time to process events |

### Input Latency

| Metric | Target | Measurement |
|--------|--------|-------------|
| Touch Response | <16ms | Time from touch to response |
| Keyboard Response | <10ms | Time from key press to response |
| Mouse Response | <8ms | Time from mouse move to response |

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new("Click me".to_string());
        assert_eq!(button.text, "Click me");
    }

    #[test]
    fn test_layout_row() {
        let layout = Layout::Row {
            spacing: 10.0,
            alignment: Alignment::Start,
        };
        
        let parent_rect = Rect { x: 0.0, y: 0.0, width: 400.0, height: 100.0 };
        let children = vec![
            create_test_widget(),
            create_test_widget(),
            create_test_widget(),
        ];
        
        let rects = layout.calculate(parent_rect, &children, Padding::default());
        assert_eq!(rects.len(), 3);
    }

    #[test]
    fn test_i18n_translation() {
        let mut i18n = I18nEngine::new().unwrap();
        let mut translations = HashMap::new();
        translations.insert("hello".to_string(), "Hello".to_string());
        
        i18n.load_translations("en", translations).unwrap();
        i18n.set_locale("en").unwrap();
        
        assert_eq!(i18n.get_translation("hello"), "Hello");
    }
}
```

---

## Code Examples

### Creating a Simple UI

```rust
use horizon_ui::{Container, Button, Layout};

fn create_ui() -> Container {
    let button1 = Button::new("Button 1".to_string())
        .on_click(|| println!("Button 1 clicked"));
    
    let button2 = Button::new("Button 2".to_string())
        .on_click(|| println!("Button 2 clicked"));
    
    let button3 = Button::new("Button 3".to_string())
        .on_click(|| println!("Button 3 clicked"));
    
    Container::new()
        .with_layout(Layout::Row {
            spacing: 10.0,
            alignment: Alignment::Center,
        })
        .with_children(vec![
            Arc::new(button1),
            Arc::new(button2),
            Arc::new(button3),
        ])
}
```

### Setting Up Accessibility

```rust
use horizon_ui::AccessibilityManager;

fn setup_accessibility(manager: &mut AccessibilityManager) {
    manager.enable_screen_reader().unwrap();
    manager.set_high_contrast(true).unwrap();
    manager.set_text_scale(1.5).unwrap();
}
```

### Using Internationalization

```rust
use horizon_ui::I18nEngine;

fn setup_i18n(engine: &mut I18nEngine) {
    let mut translations = HashMap::new();
    translations.insert("welcome".to_string(), "Welcome".to_string());
    translations.insert("goodbye".to_string(), "Goodbye".to_string());
    
    engine.load_translations("en", translations).unwrap();
    engine.set_locale("en").unwrap();
}
```

---

## Troubleshooting

### Common Issues

**Issue: UI not rendering**
- **Solution**: Check graphics backend initialization
- **Command**: `vantis-horizon check-gpu`

**Issue: Input not responding**
- **Solution**: Verify input handler registration
- **Command**: `vantis-horizon check-input`

**Issue: Screen reader not working**
- **Solution**: Enable screen reader in accessibility settings
- **Command**: `vantis-horizon accessibility enable-screen-reader`

**Issue: Text not displaying correctly**
- **Solution**: Check font loading and text direction
- **Command**: `vantis-horizon check-fonts`

---

## Conclusion

This implementation guide provides a comprehensive plan for the user interface system in VantisOS. The 3-day timeline covers all critical components including widget system, layout engine, input handling, accessibility, and internationalization.

**Key Success Metrics:**
- ✅ 60 FPS rendering
- ✅ Full keyboard navigation
- ✅ Screen reader support
- ✅ 100+ language support
- ✅ RTL text support

**Next Steps:**
1. Begin implementation following the 3-day plan
2. Set up testing environment with sample UIs
3. Integrate with VantisOS build system
4. Conduct accessibility audit
5. Performance optimization

---

**Document Version**: 1.0  
**Last Updated**: February 24, 2025  
**Author**: SuperNinja AI Agent  
**Status**: Implementation Guide