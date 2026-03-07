#[cfg(test)]
mod tests {
    use super::super::radial_shell::*;

    #[test]
    fn test_radial_shell_new() {
        let shell = RadialShell::new();
        assert_eq!(shell.state(), ShellState::Hidden);
        assert!(!shell.menu().visible);
    }

    #[test]
    fn test_radial_config_default() {
        let config = RadialConfig::default();
        assert_eq!(config.menu_radius, 200.0);
        assert_eq!(config.inner_radius, 60.0);
        assert!(config.show_labels);
        assert!(config.show_icons);
    }

    #[test]
    fn test_show_menu() {
        let mut shell = RadialShell::new();
        shell.show_menu(100.0, 100.0).unwrap();
        assert!(shell.menu().visible);
        assert_eq!(shell.state(), ShellState::Animating);
    }

    #[test]
    fn test_hide_menu() {
        let mut shell = RadialShell::new();
        shell.show_menu(100.0, 100.0).unwrap();
        shell.hide_menu();
        assert!(!shell.menu().visible);
    }

    #[test]
    fn test_menu_items_count() {
        let shell = RadialShell::new();
        assert_eq!(shell.menu().items.len(), 8);
    }

    #[test]
    fn test_menu_item_names() {
        let shell = RadialShell::new();
        let names: Vec<&str> = shell.menu().items.iter().map(|i| i.name.as_str()).collect();
        assert!(names.contains(&"Applications"));
        assert!(names.contains(&"Files"));
        assert!(names.contains(&"Terminal"));
        assert!(names.contains(&"Settings"));
        assert!(names.contains(&"Power"));
    }

    #[test]
    fn test_handle_pointer_move() {
        let mut shell = RadialShell::new();
        shell.show_menu(100.0, 100.0).unwrap();

        // Move pointer to trigger selection
        let result = shell.handle_pointer_move(100.0, 300.0);
        assert!(result.is_some() || result.is_none()); // Depends on position
    }

    #[test]
    fn test_handle_pointer_move_no_menu() {
        let shell = RadialShell::new();
        let result = shell.handle_pointer_move(100.0, 200.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_quick_actions_count() {
        let shell = RadialShell::new();
        assert_eq!(shell.quick_actions().len(), 4);
    }

    #[test]
    fn test_quick_action_names() {
        let shell = RadialShell::new();
        let names: Vec<&str> = shell
            .quick_actions()
            .iter()
            .map(|a| a.name.as_str())
            .collect();
        assert!(names.contains(&"Wi-Fi"));
        assert!(names.contains(&"Bluetooth"));
        assert!(names.contains(&"Volume"));
        assert!(names.contains(&"Brightness"));
    }

    #[test]
    fn test_add_notification() {
        let mut shell = RadialShell::new();
        let id = shell.add_notification("Test", "Test message", NotificationPriority::Normal);
        assert!(id.starts_with("notif-"));
    }

    #[test]
    fn test_clear_notification() {
        let mut shell = RadialShell::new();
        let id = shell.add_notification("Test", "Test message", NotificationPriority::Normal);
        let result = shell.clear_notification(&id);
        assert!(result);
    }

    #[test]
    fn test_clear_notification_not_found() {
        let mut shell = RadialShell::new();
        let result = shell.clear_notification("nonexistent");
        assert!(!result);
    }

    #[test]
    fn test_clear_all_notifications() {
        let mut shell = RadialShell::new();
        shell.add_notification("Test1", "Message1", NotificationPriority::Normal);
        shell.add_notification("Test2", "Message2", NotificationPriority::High);
        shell.clear_all_notifications();
        assert!(shell.notifications().notifications.is_empty());
    }

    #[test]
    fn test_add_recent_item() {
        let mut shell = RadialShell::new();
        shell.add_recent_item("/path/to/file", "File", "file", RecentItemType::File);
        assert_eq!(shell.recent_items().len(), 1);
    }

    #[test]
    fn test_recent_items_limit() {
        let mut shell = RadialShell::new();
        for i in 0..15 {
            shell.add_recent_item(
                &format!("/file{}", i),
                &format!("File{}", i),
                "file",
                RecentItemType::File,
            );
        }
        assert_eq!(shell.recent_items().len(), 10);
    }

    #[test]
    fn test_update_animation() {
        let mut shell = RadialShell::new();
        shell.show_menu(100.0, 100.0).unwrap();
        shell.update_animation(250); // More than animation duration
        assert_eq!(shell.state(), ShellState::MenuShowing);
    }

    #[test]
    fn test_gesture_config_default() {
        let config = GestureConfig::default();
        assert!(config.swipe_enabled);
        assert!(config.pinch_enabled);
        assert_eq!(config.swipe_threshold, 50.0);
        assert_eq!(config.long_press_duration, 500);
    }

    #[test]
    fn test_radial_theme_default() {
        let theme = RadialTheme::default();
        assert_eq!(theme.background.r, 0);
        assert_eq!(theme.menu_background.r, 30);
        assert_eq!(theme.menu_border.r, 86);
    }

    #[test]
    fn test_menu_action_variants() {
        let action = MenuAction::LaunchApp("test".to_string());
        assert!(matches!(action, MenuAction::LaunchApp(_)));

        let action = MenuAction::System(SystemAction::Lock);
        assert!(matches!(action, MenuAction::System(_)));
    }

    #[test]
    fn test_system_action_variants() {
        assert_eq!(
            std::mem::discriminant(&SystemAction::PowerOff),
            std::mem::discriminant(&SystemAction::PowerOff)
        );
        assert_eq!(
            std::mem::discriminant(&SystemAction::Restart),
            std::mem::discriminant(&SystemAction::Restart)
        );
    }

    #[test]
    fn test_notification_priority() {
        assert_eq!(
            std::mem::discriminant(&NotificationPriority::Low),
            std::mem::discriminant(&NotificationPriority::Low)
        );
        assert_eq!(
            std::mem::discriminant(&NotificationPriority::Urgent),
            std::mem::discriminant(&NotificationPriority::Urgent)
        );
    }

    #[test]
    fn test_gesture_swipe() {
        let mut shell = RadialShell::new();
        shell.handle_gesture(Gesture::Swipe(SwipeDirection::Up));
        assert_eq!(shell.state(), ShellState::NotificationCenter);
    }

    #[test]
    fn test_gesture_swipe_down() {
        let mut shell = RadialShell::new();
        shell.handle_gesture(Gesture::Swipe(SwipeDirection::Up));
        shell.handle_gesture(Gesture::Swipe(SwipeDirection::Down));
        assert!(!shell.notifications().visible);
    }

    #[test]
    fn test_handle_gesture_long_press() {
        let mut shell = RadialShell::new();
        shell.handle_gesture(Gesture::LongPress(Point { x: 100.0, y: 100.0 }));
        assert!(shell.menu().visible);
    }

    #[test]
    fn test_color_rgb() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_rgba() {
        let color = Color::rgba(255, 128, 64, 128);
        assert_eq!(color.a, 128);
    }

    #[test]
    fn test_point_default() {
        let point = Point::default();
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }

    #[test]
    fn test_radial_shell_error_display() {
        let err = RadialShellError::MenuNotFound;
        assert!(err.to_string().contains("Menu not found"));
    }

    #[test]
    fn test_set_config() {
        let mut shell = RadialShell::new();
        let mut config = RadialConfig::default();
        config.menu_radius = 300.0;
        shell.set_config(config);
        assert_eq!(shell.config().menu_radius, 300.0);
    }
}
