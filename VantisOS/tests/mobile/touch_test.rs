// VantisOS Mobile Touch Interaction Tests
// Copyright 2025 VantisOS Team
// Licensed under MPL-2.0

use vantis_mobile::touch::*;
use vantis_ui::flux::*;

#[cfg(test)]
mod touch_system_initialization_tests {
    use super::*;

    #[test]
    fn test_touch_manager_initialization() {
        let manager = TouchManager::new();
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_touch_screen_detection() {
        let manager = TouchManager::new();
        let has_touch = manager.has_touch_screen();
        assert!(has_touch.is_ok());
    }

    #[test]
    fn test_multi_touch_support() {
        let manager = TouchManager::new();
        let max_touches = manager.get_max_touch_points();
        assert!(max_touches > 0);
    }

    #[test]
    fn test_touch_pressure_support() {
        let manager = TouchManager::new();
        let supported = manager.supports_pressure();
        assert!(supported.is_ok());
    }

    #[test]
    fn test_touch_haptic_support() {
        let manager = TouchManager::new();
        let supported = manager.supports_haptic_feedback();
        assert!(supported.is_ok());
    }

    #[test]
    fn test_stylus_support() {
        let manager = TouchManager::new();
        let supported = manager.supports_stylus();
        assert!(supported.is_ok());
    }

    #[test]
    fn test_palm_rejection_support() {
        let manager = TouchManager::new();
        let supported = manager.supports_palm_rejection();
        assert!(supported.is_ok());
    }

    #[test]
    fn test_touch_sensitivity() {
        let manager = TouchManager::new();
        let sensitivity = manager.get_touch_sensitivity();
        assert!(sensitivity > 0.0 && sensitivity <= 1.0);
    }

    #[test]
    fn test_touch_sensitivity_setting() {
        let manager = TouchManager::new();
        let result = manager.set_touch_sensitivity(0.7);
        assert!(result.is_ok());
        assert_eq!(manager.get_touch_sensitivity(), 0.7);
    }

    #[test]
    fn test_gesture_recognition_initialization() {
        let manager = TouchManager::new();
        let result = manager.initialize_gesture_recognition();
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_sampling_rate() {
        let manager = TouchManager::new();
        let rate = manager.get_touch_sampling_rate();
        assert!(rate > 0);
    }

    #[test]
    fn test_touch_latency() {
        let manager = TouchManager::new();
        let latency = manager.get_touch_latency();
        assert!(latency >= 0);
    }

    #[test]
    fn test_touch_buffer_size() {
        let manager = TouchManager::new();
        let size = manager.get_touch_buffer_size();
        assert!(size > 0);
    }
}

#[cfg(test)]
mod touch_event_tests {
    use super::*;

    #[test]
    fn test_touch_event_creation() {
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        assert_eq!(event.get_type(), TouchType::Down);
    }

    #[test]
    fn test_touch_event_timestamp() {
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        assert!(event.get_timestamp() > 0);
    }

    #[test]
    fn test_touch_event_position() {
        let point = TouchPoint { x: 150.0, y: 250.0, id: 1 };
        let event = TouchEvent::new(TouchType::Down, point);
        assert_eq!(event.get_position(), point);
    }

    #[test]
    fn test_touch_event_pressure() {
        let mut event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        event.set_pressure(0.7);
        assert_eq!(event.get_pressure(), Some(0.7));
    }

    #[test]
    fn test_touch_event_radius() {
        let mut event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        event.set_radius(10.0);
        assert_eq!(event.get_radius(), Some(10.0));
    }

    #[test]
    fn test_touch_event_velocity() {
        let mut event = TouchEvent::new(
            TouchType::Move,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        event.set_velocity(Velocity { x: 5.0, y: 3.0 });
        assert_eq!(event.get_velocity(), Some(Velocity { x: 5.0, y: 3.0 }));
    }

    #[test]
    fn test_touch_event_phase() {
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        assert_eq!(event.get_phase(), TouchPhase::Began);
    }

    #[test]
    fn test_multi_touch_event_creation() {
        let points = vec![
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
            TouchPoint { x: 300.0, y: 400.0, id: 2 },
        ];
        let event = MultiTouchEvent::new(TouchType::Down, points);
        assert_eq!(event.get_touch_count(), 2);
    }

    #[test]
    fn test_multi_touch_event_points() {
        let points = vec![
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
            TouchPoint { x: 300.0, y: 400.0, id: 2 },
        ];
        let event = MultiTouchEvent::new(TouchType::Down, points);
        assert_eq!(event.get_points().len(), 2);
    }

    #[test]
    fn test_touch_event_bubbling() {
        let manager = TouchManager::new();
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_event_capturing() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        manager.set_capture_handler(handler);
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_event_cancellation() {
        let manager = TouchManager::new();
        let event = TouchEvent::new(
            TouchType::Cancel,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_event_pass_through() {
        let manager = TouchManager::new();
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        manager.set_pass_through(true);
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_event_consumption() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        handler.set_consumes_events(true);
        manager.add_touch_handler(handler);
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod gesture_recognition_tests {
    use super::*;

    #[test]
    fn test_tap_gesture_recognition() {
        let manager = TouchManager::new();
        let recognizer = TapGestureRecognizer::new();
        recognizer.set_taps_required(1);
        recognizer.set_touches_required(1);
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_double_tap_gesture_recognition() {
        let manager = TouchManager::new();
        let recognizer = TapGestureRecognizer::new();
        recognizer.set_taps_required(2);
        recognizer.set_touches_required(1);
        recognizer.set_max_interval(300);
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_long_press_gesture_recognition() {
        let manager = TouchManager::new();
        let recognizer = LongPressGestureRecognizer::new();
        recognizer.set_min_duration(500);
        recognizer.set_touches_required(1);
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pan_gesture_recognition() {
        let manager = TouchManager::new();
        let recognizer = PanGestureRecognizer::new();
        recognizer.set_minimum_distance(10);
        recognizer.set_touches_required(1);
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pinch_gesture_recognition() {
        let manager = TouchManager::new();
        let recognizer = PinchGestureRecognizer::new();
        recognizer.set_touches_required(2);
        recognizer.set_min_scale(0.1);
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rotation_gesture_recognition() {
        let manager = TouchManager::new();
        let recognizer = RotationGestureRecognizer::new();
        recognizer.set_touches_required(2);
        recognizer.set_min_rotation(0.1);
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_swipe_gesture_recognition() {
        let manager = TouchManager::new();
        let recognizer = SwipeGestureRecognizer::new();
        recognizer.set_direction(SwipeDirection::Right);
        recognizer.set_touches_required(1);
        recognizer.set_min_distance(50);
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_swipe_direction_recognition() {
        let recognizer = SwipeGestureRecognizer::new();
        recognizer.set_direction(SwipeDirection::Up);
        assert_eq!(recognizer.get_direction(), SwipeDirection::Up);
    }

    #[test]
    fn test_edge_pan_gesture_recognition() {
        let manager = TouchManager::new();
        let recognizer = EdgePanGestureRecognizer::new();
        recognizer.set_edge(Edge::Left);
        recognizer.set_min_distance(50);
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_screen_edge_pan_gesture() {
        let recognizer = EdgePanGestureRecognizer::new();
        recognizer.set_edge(Edge::Right);
        recognizer.set_min_distance(100);
        assert_eq!(recognizer.get_edge(), Edge::Right);
    }

    #[test]
    fn test_gesture_recognizer_delegate() {
        let manager = TouchManager::new();
        let recognizer = TapGestureRecognizer::new();
        let delegate = GestureDelegate::new();
        recognizer.set_delegate(delegate);
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gesture_recognizer_state() {
        let recognizer = TapGestureRecognizer::new();
        assert_eq!(recognizer.get_state(), GestureState::Possible);
    }

    #[test]
    fn test_gesture_recognizer_failure_requirements() {
        let tap = TapGestureRecognizer::new();
        let double_tap = TapGestureRecognizer::new();
        double_tap.set_taps_required(2);
        tap.require_to_fail(double_tap);
        assert!(tap.has_failure_requirements());
    }

    #[test]
    fn test_simultaneous_gesture_recognition() {
        let pan = PanGestureRecognizer::new();
        let pinch = PinchGestureRecognizer::new();
        pan.set_simultaneous(true);
        pinch.set_simultaneous(true);
        assert!(pan.is_simultaneous() && pinch.is_simultaneous());
    }

    #[test]
    fn test_custom_gesture_recognizer() {
        let manager = TouchManager::new();
        let recognizer = CustomGestureRecognizer::new("custom_gesture");
        recognizer.on_recognize(|_| {
            // Custom recognition logic
        });
        let result = manager.add_gesture_recognizer(recognizer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gesture_recognizer_priority() {
        let tap = TapGestureRecognizer::new();
        let long_press = LongPressGestureRecognizer::new();
        tap.set_priority(100);
        long_press.set_priority(90);
        assert!(tap.get_priority() > long_press.get_priority());
    }

    #[test]
    fn test_gesture_recognizer_enabled() {
        let recognizer = TapGestureRecognizer::new();
        recognizer.set_enabled(false);
        assert!(!recognizer.is_enabled());
    }

    #[test]
    fn test_gesture_recognizer_cancels_touches() {
        let recognizer = LongPressGestureRecognizer::new();
        recognizer.set_cancels_touches(true);
        assert!(recognizer.cancels_touches());
    }

    #[test]
    fn test_gesture_recognizer_delays_touches() {
        let recognizer = LongPressGestureRecognizer::new();
        recognizer.set_delays_touches(true);
        assert!(recognizer.delays_touches());
    }

    #[test]
    fn test_gesture_recognizer_require_failure_of() {
        let tap = TapGestureRecognizer::new();
        let double_tap = TapGestureRecognizer::new();
        double_tap.set_taps_required(2);
        tap.require_to_fail(double_tap);
        let failures = tap.get_failure_requirements();
        assert!(!failures.is_empty());
    }

    #[test]
    fn test_gesture_recognizer_began() {
        let recognizer = TapGestureRecognizer::new();
        recognizer.on_began(|_| {
            // Gesture began callback
        });
        assert!(recognizer.has_began_callback());
    }

    #[test]
    fn test_gesture_recognizer_changed() {
        let recognizer = PanGestureRecognizer::new();
        recognizer.on_changed(|_| {
            // Gesture changed callback
        });
        assert!(recognizer.has_changed_callback());
    }

    #[test]
    fn test_gesture_recognizer_ended() {
        let recognizer = TapGestureRecognizer::new();
        recognizer.on_ended(|_| {
            // Gesture ended callback
        });
        assert!(recognizer.has_ended_callback());
    }

    #[test]
    fn test_gesture_recognizer_failed() {
        let recognizer = LongPressGestureRecognizer::new();
        recognizer.on_failed(|_| {
            // Gesture failed callback
        });
        assert!(recognizer.has_failed_callback());
    }

    #[test]
    fn test_gesture_recognizer_cancelled() {
        let recognizer = PanGestureRecognizer::new();
        recognizer.on_cancelled(|_| {
            // Gesture cancelled callback
        });
        assert!(recognizer.has_cancelled_callback());
    }
}

#[cfg(test)]
mod touch_handling_tests {
    use super::*;

    #[test]
    fn test_touch_handler_creation() {
        let handler = TouchHandler::new();
        assert!(handler.is_created());
    }

    #[test]
    fn test_touch_handler_registration() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        let result = manager.add_touch_handler(handler);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_handler_removal() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        let id = manager.add_touch_handler(handler).unwrap();
        let result = manager.remove_touch_handler(id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_handler_priority() {
        let handler = TouchHandler::new();
        handler.set_priority(100);
        assert_eq!(handler.get_priority(), 100);
    }

    #[test]
    fn test_touch_handler_zone() {
        let handler = TouchHandler::new();
        handler.set_zone(TouchZone {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        });
        assert!(handler.has_zone());
    }

    #[test]
    fn test_touch_handler_on_touch_down() {
        let handler = TouchHandler::new();
        handler.on_touch_down(|event| {
            assert_eq!(event.get_type(), TouchType::Down);
        });
        assert!(handler.has_touch_down_callback());
    }

    #[test]
    fn test_touch_handler_on_touch_move() {
        let handler = TouchHandler::new();
        handler.on_touch_move(|event| {
            assert_eq!(event.get_type(), TouchType::Move);
        });
        assert!(handler.has_touch_move_callback());
    }

    #[test]
    fn test_touch_handler_on_touch_up() {
        let handler = TouchHandler::new();
        handler.on_touch_up(|event| {
            assert_eq!(event.get_type(), TouchType::Up);
        });
        assert!(handler.has_touch_up_callback());
    }

    #[test]
    fn test_touch_handler_on_touch_cancel() {
        let handler = TouchHandler::new();
        handler.on_touch_cancel(|event| {
            assert_eq!(event.get_type(), TouchType::Cancel);
        });
        assert!(handler.has_touch_cancel_callback());
    }

    #[test]
    fn test_touch_handler_consumes_events() {
        let handler = TouchHandler::new();
        handler.set_consumes_events(true);
        assert!(handler.consumes_events());
    }

    #[test]
    fn test_touch_handler_enabled() {
        let handler = TouchHandler::new();
        handler.set_enabled(false);
        assert!(!handler.is_enabled());
    }

    #[test]
    fn test_touch_handler_hit_test() {
        let handler = TouchHandler::new();
        handler.set_zone(TouchZone {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        });
        let point = TouchPoint { x: 50.0, y: 50.0, id: 1 };
        assert!(handler.hit_test(point));
    }

    #[test]
    fn test_touch_handler_hit_test_fail() {
        let handler = TouchHandler::new();
        handler.set_zone(TouchZone {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        });
        let point = TouchPoint { x: 150.0, y: 150.0, id: 1 };
        assert!(!handler.hit_test(point));
    }

    #[test]
    fn test_touch_handler_multiple_handlers() {
        let manager = TouchManager::new();
        let handler1 = TouchHandler::new();
        let handler2 = TouchHandler::new();
        manager.add_touch_handler(handler1).ok();
        manager.add_touch_handler(handler2).ok();
        assert_eq!(manager.get_handler_count(), 2);
    }

    #[test]
    fn test_touch_handler_with_gesture() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        let recognizer = TapGestureRecognizer::new();
        handler.add_gesture_recognizer(recognizer);
        let result = manager.add_touch_handler(handler);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_handler_exclusive() {
        let handler = TouchHandler::new();
        handler.set_exclusive(true);
        assert!(handler.is_exclusive());
    }

    #[test]
    fn test_touch_handler_propagation() {
        let manager = TouchManager::new();
        let handler1 = TouchHandler::new();
        let handler2 = TouchHandler::new();
        handler1.set_propagates(false);
        manager.add_touch_handler(handler1).ok();
        manager.add_touch_handler(handler2).ok();
        assert!(!handler1.propagates());
    }

    #[test]
    fn test_touch_handler_delegation() {
        let handler = TouchHandler::new();
        let delegate = TouchHandlerDelegate::new();
        handler.set_delegate(delegate);
        assert!(handler.has_delegate());
    }
}

#[cfg(test)]
mod touch_feedback_tests {
    use super::*;

    #[test]
    fn test_haptic_feedback_on_tap() {
        let manager = TouchManager::new();
        let result = manager.trigger_haptic_feedback(HapticType::Light);
        assert!(result.is_ok());
    }

    #[test]
    fn test_haptic_feedback_on_long_press() {
        let manager = TouchManager::new();
        let result = manager.trigger_haptic_feedback(HapticType::Medium);
        assert!(result.is_ok());
    }

    #[test]
    fn test_haptic_feedback_custom_pattern() {
        let manager = TouchManager::new();
        let pattern = vec![10, 20, 30];
        let result = manager.trigger_haptic_pattern(pattern);
        assert!(result.is_ok());
    }

    #[test]
    fn test_visual_feedback_ripple() {
        let manager = TouchManager::new();
        let result = manager.show_ripple_effect(TouchPoint { x: 100.0, y: 200.0, id: 1 });
        assert!(result.is_ok());
    }

    #[test]
    fn test_visual_feedback_highlight() {
        let manager = TouchManager::new();
        let result = manager.highlight_zone(TouchZone {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_visual_feedback_animation() {
        let manager = TouchManager::new();
        let result = manager.animate_feedback(TouchPoint { x: 100.0, y: 200.0, id: 1 });
        assert!(result.is_ok());
    }

    #[test]
    fn test_feedback_duration() {
        let manager = TouchManager::new();
        manager.set_feedback_duration(500);
        assert_eq!(manager.get_feedback_duration(), 500);
    }

    #[test]
    fn test_feedback_intensity() {
        let manager = TouchManager::new();
        manager.set_feedback_intensity(0.7);
        assert_eq!(manager.get_feedback_intensity(), 0.7);
    }

    #[test]
    fn test_feedback_enabled() {
        let manager = TouchManager::new();
        manager.set_feedback_enabled(false);
        assert!(!manager.is_feedback_enabled());
    }

    #[test]
    fn test_sound_feedback_on_tap() {
        let manager = TouchManager::new();
        let result = manager.play_sound_feedback(SoundType::Tap);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sound_feedback_on_click() {
        let manager = TouchManager::new();
        let result = manager.play_sound_feedback(SoundType::Click);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sound_feedback_custom() {
        let manager = TouchManager::new();
        let result = manager.play_custom_sound("tap_sound.mp3");
        assert!(result.is_ok());
    }

    #[test]
    fn test_sound_feedback_volume() {
        let manager = TouchManager::new();
        manager.set_sound_feedback_volume(0.5);
        assert_eq!(manager.get_sound_feedback_volume(), 0.5);
    }

    #[test]
    fn test_feedback_combination() {
        let manager = TouchManager::new();
        manager.trigger_haptic_feedback(HapticType::Light).ok();
        manager.show_ripple_effect(TouchPoint { x: 100.0, y: 200.0, id: 1 }).ok();
        manager.play_sound_feedback(SoundType::Tap).ok();
        assert!(manager.is_feedback_enabled());
    }

    #[test]
    fn test_feedback_delay() {
        let manager = TouchManager::new();
        manager.set_feedback_delay(50);
        assert_eq!(manager.get_feedback_delay(), 50);
    }

    #[test]
    fn test_feedback_curve() {
        let manager = TouchManager::new();
        manager.set_feedback_curve(FeedbackCurve::EaseOut);
        assert_eq!(manager.get_feedback_curve(), FeedbackCurve::EaseOut);
    }
}

#[cfg(test)]
mod touch_performance_tests {
    use super::*;

    #[test]
    fn test_touch_event_processing_performance() {
        let manager = TouchManager::new();
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let event = TouchEvent::new(
                TouchType::Down,
                TouchPoint { x: 100.0, y: 200.0, id: i as i32 },
            );
            manager.dispatch_event(event).ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_gesture_recognition_performance() {
        let manager = TouchManager::new();
        let recognizer = TapGestureRecognizer::new();
        manager.add_gesture_recognizer(recognizer).ok();
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let event = TouchEvent::new(
                TouchType::Down,
                TouchPoint { x: 100.0, y: 200.0, id: i as i32 },
            );
            manager.dispatch_event(event).ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn test_multi_touch_performance() {
        let manager = TouchManager::new();
        let points = vec![
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
            TouchPoint { x: 300.0, y: 400.0, id: 2 },
            TouchPoint { x: 500.0, y: 600.0, id: 3 },
        ];
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let event = MultiTouchEvent::new(TouchType::Move, points.clone());
            manager.dispatch_multi_touch_event(event).ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn test_haptic_feedback_performance() {
        let manager = TouchManager::new();
        let start = std::time::Instant::now();
        for _ in 0..100 {
            manager.trigger_haptic_feedback(HapticType::Light).ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn test_visual_feedback_performance() {
        let manager = TouchManager::new();
        let start = std::time::Instant::now();
        for i in 0..100 {
            manager.show_ripple_effect(TouchPoint { x: 100.0, y: 200.0, id: i as i32 }).ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn test_handler_dispatch_performance() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        handler.on_touch_down(|_| {});
        manager.add_touch_handler(handler).ok();
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let event = TouchEvent::new(
                TouchType::Down,
                TouchPoint { x: 100.0, y: 200.0, id: i as i32 },
            );
            manager.dispatch_event(event).ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn test_memory_usage() {
        let manager = TouchManager::new();
        let memory_before = manager.get_memory_usage();
        
        for i in 0..1000 {
            let event = TouchEvent::new(
                TouchType::Down,
                TouchPoint { x: 100.0, y: 200.0, id: i as i32 },
            );
            manager.dispatch_event(event).ok();
        }
        
        let memory_after = manager.get_memory_usage();
        let increase = memory_after - memory_before;
        assert!(increase < 5_000_000); // Less than 5MB increase
    }

    #[test]
    fn test_concurrent_touch_events() {
        let manager = TouchManager::new();
        let start = std::time::Instant::now();
        
        let handles: Vec<_> = (0..10).map(|i| {
            std::thread::spawn(move || {
                let local_manager = TouchManager::new();
                for j in 0..100 {
                    let event = TouchEvent::new(
                        TouchType::Down,
                        TouchPoint { x: 100.0, y: 200.0, id: (i * 100 + j) as i32 },
                    );
                    local_manager.dispatch_event(event).ok();
                }
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let duration = start.elapsed();
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn test_touch_buffer_optimization() {
        let manager = TouchManager::new();
        manager.set_touch_buffer_size(100);
        let start = std::time::Instant::now();
        for i in 0..200 {
            let event = TouchEvent::new(
                TouchType::Down,
                TouchPoint { x: 100.0, y: 200.0, id: i as i32 },
            );
            manager.dispatch_event(event).ok();
        }
        let duration = start.elapsed();
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn test_gesture_recognizer_pool() {
        let manager = TouchManager::new();
        for _ in 0..100 {
            let recognizer = TapGestureRecognizer::new();
            manager.add_gesture_recognizer(recognizer).ok();
        }
        let count = manager.get_gesture_recognizer_count();
        assert_eq!(count, 100);
    }
}

#[cfg(test)]
mod touch_integration_tests {
    use super::*;

    #[test]
    fn test_touch_with_gesture_recognition() {
        let manager = TouchManager::new();
        let recognizer = TapGestureRecognizer::new();
        manager.add_gesture_recognizer(recognizer).ok();
        
        let handler = TouchHandler::new();
        handler.on_touch_down(|event| {
            assert_eq!(event.get_type(), TouchType::Down);
        });
        manager.add_touch_handler(handler).ok();
        
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multi_touch_with_pinch() {
        let manager = TouchManager::new();
        let recognizer = PinchGestureRecognizer::new();
        manager.add_gesture_recognizer(recognizer).ok();
        
        let points = vec![
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
            TouchPoint { x: 300.0, y: 400.0, id: 2 },
        ];
        let event = MultiTouchEvent::new(TouchType::Down, points);
        let result = manager.dispatch_multi_touch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_swipe_with_navigation() {
        let manager = TouchManager::new();
        let recognizer = SwipeGestureRecognizer::new();
        recognizer.set_direction(SwipeDirection::Left);
        manager.add_gesture_recognizer(recognizer).ok();
        
        recognizer.on_ended(|_| {
            // Navigate back
        });
        
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_long_press_with_context_menu() {
        let manager = TouchManager::new();
        let recognizer = LongPressGestureRecognizer::new();
        recognizer.set_min_duration(500);
        manager.add_gesture_recognizer(recognizer).ok();
        
        recognizer.on_ended(|_| {
            // Show context menu
        });
        
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_double_tap_with_zoom() {
        let manager = TouchManager::new();
        let recognizer = TapGestureRecognizer::new();
        recognizer.set_taps_required(2);
        manager.add_gesture_recognizer(recognizer).ok();
        
        recognizer.on_ended(|_| {
            // Zoom in
        });
        
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_edge_pan_with_drawer() {
        let manager = TouchManager::new();
        let recognizer = EdgePanGestureRecognizer::new();
        recognizer.set_edge(Edge::Left);
        manager.add_gesture_recognizer(recognizer).ok();
        
        recognizer.on_changed(|_| {
            // Open drawer
        });
        
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 10.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_with_haptic_feedback() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        handler.on_touch_down(|_| {
            manager.trigger_haptic_feedback(HapticType::Light).ok();
        });
        manager.add_touch_handler(handler).ok();
        
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        manager.dispatch_event(event).ok();
    }

    #[test]
    fn test_touch_with_visual_feedback() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        handler.on_touch_down(|event| {
            manager.show_ripple_effect(event.get_position()).ok();
        });
        manager.add_touch_handler(handler).ok();
        
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        manager.dispatch_event(event).ok();
    }

    #[test]
    fn test_pan_with_scroll() {
        let manager = TouchManager::new();
        let recognizer = PanGestureRecognizer::new();
        recognizer.set_minimum_distance(10);
        manager.add_gesture_recognizer(recognizer).ok();
        
        recognizer.on_changed(|gesture| {
            // Scroll content
            let translation = gesture.get_translation();
            assert!(translation.x.abs() > 0.0 || translation.y.abs() > 0.0);
        });
        
        let event = TouchEvent::new(
            TouchType::Move,
            TouchPoint { x: 110.0, y: 210.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pinch_with_rotation() {
        let manager = TouchManager::new();
        let pinch = PinchGestureRecognizer::new();
        let rotation = RotationGestureRecognizer::new();
        pinch.set_simultaneous(true);
        rotation.set_simultaneous(true);
        manager.add_gesture_recognizer(pinch).ok();
        manager.add_gesture_recognizer(rotation).ok();
        
        let points = vec![
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
            TouchPoint { x: 300.0, y: 400.0, id: 2 },
        ];
        let event = MultiTouchEvent::new(TouchType::Move, points);
        let result = manager.dispatch_multi_touch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_with_sound_feedback() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        handler.on_touch_up(|_| {
            manager.play_sound_feedback(SoundType::Tap).ok();
        });
        manager.add_touch_handler(handler).ok();
        
        let event = TouchEvent::new(
            TouchType::Up,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        manager.dispatch_event(event).ok();
    }

    #[test]
    fn test_custom_gesture_with_action() {
        let manager = TouchManager::new();
        let recognizer = CustomGestureRecognizer::new("draw_circle");
        recognizer.on_recognize(|gesture| {
            // Perform circle action
            assert_eq!(gesture.get_name(), "draw_circle");
        });
        manager.add_gesture_recognizer(recognizer).ok();
        
        let event = TouchEvent::new(
            TouchType::Up,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_handler_priority_order() {
        let manager = TouchManager::new();
        let handler1 = TouchHandler::new();
        let handler2 = TouchHandler::new();
        handler1.set_priority(100);
        handler2.set_priority(50);
        
        handler1.on_touch_down(|_| {
            // High priority handler
        });
        
        handler2.on_touch_down(|_| {
            // Low priority handler
        });
        
        manager.add_touch_handler(handler1).ok();
        manager.add_touch_handler(handler2).ok();
        
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_zone_filtering() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        handler.set_zone(TouchZone {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        });
        
        handler.on_touch_down(|_| {
            // Zone specific handler
        });
        
        manager.add_touch_handler(handler).ok();
        
        let event_inside = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 50.0, y: 50.0, id: 1 },
        );
        
        let event_outside = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 150.0, y: 150.0, id: 2 },
        );
        
        manager.dispatch_event(event_inside).ok();
        manager.dispatch_event(event_outside).ok();
    }

    #[test]
    fn test_touch_with_delegation() {
        let manager = TouchManager::new();
        let delegate = TouchHandlerDelegate::new();
        let handler = TouchHandler::new();
        handler.set_delegate(delegate);
        
        handler.on_touch_down(|_| {
            // Handler logic
        });
        
        manager.add_touch_handler(handler).ok();
        
        let event = TouchEvent::new(
            TouchType::Down,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_touch_cancellation_propagation() {
        let manager = TouchManager::new();
        let handler = TouchHandler::new();
        handler.set_propagates(false);
        
        handler.on_touch_cancel(|_| {
            // Handle cancellation
        });
        
        manager.add_touch_handler(handler).ok();
        
        let event = TouchEvent::new(
            TouchType::Cancel,
            TouchPoint { x: 100.0, y: 200.0, id: 1 },
        );
        let result = manager.dispatch_event(event);
        assert!(result.is_ok());
    }
}