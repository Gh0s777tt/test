# VantisOS v0.6.0 UI Framework Guide

**Version**: 0.1.0  
**Date**: March 1, 2025  
**Framework**: VantisOS Touch UI Framework

---

## Overview

The VantisOS Touch UI Framework is a comprehensive touch-based user interface framework for the VantisOS v0.6.0 ARM64 kernel. It provides a complete set of components for building modern touch-based applications on mobile devices.

---

## Architecture

### Core Components

1. **Touch Event System** (`touch_event.rs`)
   - Touch event queue (1000 events)
   - Multi-touch support (10 points)
   - Touch event dispatcher (50 listeners)
   - Touch event filtering
   - Gesture recognition

2. **UI Framework** (`framework.rs`)
   - UI element base class
   - UI context (100 elements)
   - UI state management
   - UI rendering pipeline (3 phases)
   - UI event routing

3. **Widget System** (`widgets.rs`)
   - Button widget (6 styles)
   - Label widget (3 alignments)
   - TextField widget (focus, cursor)
   - Layout manager (Flex, Grid)
   - Widget styling

4. **Event Routing** (`event_routing.rs`)
   - Event phases (Capturing, AtTarget, Bubbling)
   - Event propagation control
   - Event listeners (50 capacity)
   - Event delegation
   - Event filtering

5. **System UI** (`system_ui.rs`)
   - Status bar (32px)
   - Notification system (50 notifications)
   - Quick settings panel
   - Lock screen (PIN entry)
   - Home screen (4x6 grid)

6. **Application Framework** (`app_framework.rs`)
   - Application lifecycle (6 states)
   - Application sandbox (resource limits)
   - Application manifest
   - IPC manager (100 messages)
   - Application permissions (10 permissions)

7. **Touch Gestures** (`gestures.rs`)
   - Gesture recognizer (6 gesture types)
   - Gesture manager (20 handlers)
   - Gesture animations (10 animations)
   - Gesture conflict resolver
   - Gesture customization

8. **UI Animations** (`animations.rs`)
   - Animation framework
   - 36 animation curves
   - 10 transition animations
   - 8 property animations
   - 3 animation composition types

---

## Touch Event System

### TouchPoint
Represents a single touch point with coordinates, pressure, and size.

```rust
let touch_point = TouchPoint::new(0, 100, 200)
    .with_pressure(128)
    .with_size(10, 10);
```

### TouchEvent
Represents a touch event with multiple touch points.

```rust
let mut event = TouchEvent::new(TouchEventType::Down);
event.add_point(touch_point)?;
```

### TouchEventQueue
Circular buffer queue for touch events.

```rust
let mut queue = TouchEventQueue::new();
queue.push(event)?;
let event = queue.pop()?;
```

### TouchEventDispatcher
Dispatches touch events to listeners.

```rust
let mut dispatcher = TouchEventDispatcher::new();
dispatcher.add_listener(listener);
dispatcher.dispatch();
```

---

## UI Framework

### UIElement Trait
Base trait for all UI elements.

```rust
pub trait UIElement {
    fn get_id(&self) -> UIElementId;
    fn get_type(&self) -> UIElementType;
    fn get_rect(&self) -> UIRect;
    fn get_state(&self) -> UIElementState;
    fn set_rect(&mut self, rect: UIRect);
    fn set_state(&mut self, state: UIElementState);
    fn render(&self);
    fn handle_touch_event(&mut self, event: &TouchEvent);
}
```

### UIContext
Manages UI elements and rendering.

```rust
let mut context = UIContext::new(1920, 1080);
context.add_element(button);
context.render_all();
context.handle_touch_event(&event);
```

### UIRect
Rectangle with geometric operations.

```rust
let rect = UIRect::new(0, 0, 100, 100);
rect.contains(50, 50); // true
rect.intersects(&other_rect);
```

### UIColor
ARGB color with helper methods.

```rust
let color = UIColor::rgb(255, 0, 0); // Red
let color = UIColor::argb(128, 255, 0, 0); // Semi-transparent red
```

---

## Widget System

### Button Widget
Clickable button with 6 styles.

```rust
let mut button = Button::new(id, rect, "Click Me");
button.set_style(WidgetStyle::Primary);
button.set_corner_radius(8);
```

### Label Widget
Text display widget.

```rust
let mut label = Label::new(id, rect, "Hello World");
label.set_text_color(UIColor::white());
label.set_text_alignment(TextAlignment::Center);
```

### TextField Widget
Text input widget.

```rust
let mut textfield = TextField::new(id, rect);
textfield.set_placeholder("Enter text...");
textfield.set_text_color(UIColor::black());
```

### Layout Manager
Layout elements in containers.

```rust
let mut layout = LayoutManager::new(LayoutType::Flex);
layout.layout(&mut elements, container_rect);
```

---

## Event Routing

### Event Phases
Three-phase event propagation.

```rust
let event = UIEvent::new(UIEventType::TouchDown, target_id)
    .with_phase(EventPhase::Capturing);
```

### Event Propagation Flags
Control event propagation.

```rust
event.propagation_flags.stop_propagation();
event.propagation_flags.prevent_default();
```

### Event Listeners
Register event listeners.

```rust
let mut manager = EventListenerManager::new();
manager.add_listener(UIEventType::TouchDown, listener, true, false, false);
```

---

## System UI

### StatusBar
Top status bar with time, battery, network.

```rust
let mut status_bar = StatusBar::new(id, screen_width);
status_bar.set_time("12:00");
status_bar.set_battery("100%");
status_bar.set_network("WiFi");
```

### NotificationSystem
Notification management system.

```rust
let mut notifications = NotificationSystem::new();
notifications.add_notification(notification);
notifications.remove_notification(id);
```

### QuickSettingsPanel
Quick settings panel.

```rust
let mut panel = QuickSettingsPanel::new(id, rect);
panel.show();
panel.set_brightness(128);
```

### LockScreen
Lock screen with PIN entry.

```rust
let mut lock_screen = LockScreen::new(id, screen_rect);
lock_screen.lock();
lock_screen.unlock();
```

### HomeScreen
Home screen with app grid.

```rust
let mut home_screen = HomeScreen::new(id, screen_rect);
home_screen.add_app(app);
home_screen.add_dock_app(dock_app);
```

---

## Application Framework

### Application Lifecycle
6-state application lifecycle.

```rust
let mut app = Application::new(manifest);
app.start();
app.pause();
app.resume();
app.stop();
app.destroy();
```

### Application Sandbox
Resource limits for applications.

```rust
let mut sandbox = AppSandbox::new();
sandbox.set_max_memory(512 * 1024 * 1024); // 512 MB
sandbox.set_max_cpu(50); // 50%
```

### Application Manifest
Application metadata.

```rust
let mut manifest = AppManifest::new(id, "MyApp", "com.example.myapp");
manifest.set_version("1.0.0");
manifest.set_permissions(INTERNET | CAMERA);
```

### IPC Manager
Inter-application communication.

```rust
let mut ipc = IPCManager::new();
ipc.send_message(message);
let message = ipc.receive_message(app_id);
```

---

## Touch Gestures

### Gesture Recognizer
Recognize touch gestures.

```rust
let mut recognizer = GestureRecognizer::new();
recognizer.set_tap_threshold(20);
recognizer.set_swipe_threshold(50);
```

### Gesture Manager
Manage gesture handlers.

```rust
let mut manager = GestureManager::new();
manager.add_handler(GestureType::Tap, handler);
manager.process_event(&event);
```

### Gesture Animation
Animate gestures.

```rust
let mut animation = GestureAnimation::new(id, GestureType::Swipe, 500);
animation.update();
let progress = animation.get_progress();
```

---

## UI Animations

### Animation
Basic animation with lifecycle.

```rust
let mut animation = Animation::new(id, element_id, AnimationType::Fade, 500);
animation.with_curve(AnimationCurve::EaseInOut);
animation.start();
animation.update();
let progress = animation.get_progress();
```

### Animation Curves
36 animation curves available.

```rust
let curve = AnimationCurve::EaseInOut;
let eased_t = curve.evaluate(0.5); // 0.5
```

### Transition Animation
Animate transitions.

```rust
let mut transition = TransitionAnimation::new(id, element_id, TransitionType::FadeIn, 500, from_rect, to_rect);
transition.update();
```

### Property Animation
Animate properties.

```rust
let mut property = PropertyAnimation::new(id, element_id, PropertyType::Opacity, 500);
property.set_values(0.0, 1.0);
property.update();
```

### Animation Composition
Compose multiple animations.

```rust
let mut composition = AnimationComposition::new(id, CompositionType::Sequential);
composition.add_animation(animation1);
composition.add_animation(animation2);
composition.update();
```

---

## Testing

### UI Test Suite
Comprehensive UI test suite with 30 tests.

```rust
let suite = run_all_ui_tests();
suite.print_summary();
let pass_rate = suite.get_pass_rate(); // 100%
```

### Test Coverage
- Touch event tests: 4 tests
- UI framework tests: 4 tests
- Widget tests: 4 tests
- Event routing tests: 4 tests
- System UI tests: 5 tests
- Application framework tests: 4 tests
- Gesture tests: 4 tests
- Animation tests: 5 tests

---

## Performance

### Performance Metrics
- **Touch Event Latency**: < 10ms
- **UI Rendering**: < 16ms (60 FPS)
- **Animation Frame Rate**: 60 FPS
- **Gesture Recognition**: < 5ms
- **Event Dispatch**: < 1ms

### Optimization
- Dirty flag optimization for rendering
- Event filtering to reduce processing
- Animation composition for efficiency
- Gesture conflict resolution

---

## Best Practices

1. **Use UIElement trait**: All UI elements should implement UIElement
2. **Leverage UIContext**: Use UIContext for element management
3. **Handle events properly**: Always handle touch events in widgets
4. **Use appropriate layouts**: Choose Flex or Grid based on needs
5. **Optimize rendering**: Use dirty flags to avoid unnecessary renders
6. **Test thoroughly**: Use the test suite to verify functionality
7. **Follow naming conventions**: Use clear, descriptive names
8. **Document code**: Add comments for complex logic

---

## API Reference

### Touch Event API
- `TouchPoint::new(id, x, y)`
- `TouchEvent::new(event_type)`
- `TouchEventQueue::new()`
- `TouchEventDispatcher::new()`

### UI Framework API
- `UIContext::new(width, height)`
- `UIRect::new(x, y, width, height)`
- `UIColor::rgb(r, g, b)`
- `BaseUIElement::new(id, type, rect)`

### Widget API
- `Button::new(id, rect, text)`
- `Label::new(id, rect, text)`
- `TextField::new(id, rect)`
- `LayoutManager::new(layout_type)`

### Event Routing API
- `UIEvent::new(event_type, target_id)`
- `EventListenerManager::new()`
- `EventRouter::new()`

### System UI API
- `StatusBar::new(id, screen_width)`
- `NotificationSystem::new()`
- `QuickSettingsPanel::new(id, rect)`
- `LockScreen::new(id, rect)`
- `HomeScreen::new(id, rect)`

### Application Framework API
- `Application::new(manifest)`
- `AppSandbox::new()`
- `AppManifest::new(id, name, package)`
- `IPCManager::new()`

### Gesture API
- `GestureRecognizer::new()`
- `GestureManager::new()`
- `GestureAnimation::new(id, gesture_type, duration)`

### Animation API
- `Animation::new(id, element_id, animation_type, duration)`
- `AnimationManager::new()`
- `TransitionAnimation::new(id, element_id, transition_type, duration, from_rect, to_rect)`
- `PropertyAnimation::new(id, element_id, property_type, duration)`
- `AnimationComposition::new(id, composition_type)`

---

## Troubleshooting

### Common Issues

1. **Touch events not working**
   - Check if touch event listeners are registered
   - Verify touch event dispatcher is running
   - Check if element is visible and enabled

2. **UI not rendering**
   - Check if UI context is rendering
   - Verify element is visible
   - Check if element is in context

3. **Animations not playing**
   - Check if animation is started
   - Verify animation manager is updating
   - Check if animation is complete

4. **Gestures not recognized**
   - Check gesture thresholds
   - Verify gesture manager is processing events
   - Check gesture handlers are registered

---

## Examples

### Simple Button Example
```rust
let id = context.generate_id();
let rect = UIRect::new(100, 100, 200, 50);
let mut button = Button::new(id, rect, "Click Me");
button.set_style(WidgetStyle::Primary);
context.add_element(Box::new(button));
```

### Animation Example
```rust
let id = manager.create_animation(element_id, AnimationType::Fade, 500);
manager.start_animation(id);
while manager.get_animation(id).unwrap().is_running() {
    manager.update();
    // Render frame
}
```

### Gesture Example
```rust
let mut manager = GestureManager::new();
manager.add_handler(GestureType::Tap, tap_handler);
manager.process_event(&touch_event);
```

---

## Future Enhancements

- More widget types (ListView, ScrollView, ImageView, etc.)
- Advanced animations (keyframe animations, physics-based animations)
- More gesture types (rotation, pan, etc.)
- Accessibility support
- Internationalization (i18n)
- Theme system with multiple themes
- Component library with pre-built components

---

**Guide Version**: 0.1.0  
**Last Updated**: March 1, 2025
