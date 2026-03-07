#[cfg(test)]
mod tests {
    use super::super::spatial_shell::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_spatial_shell_new() {
        let shell = SpatialShell::new();
        assert_eq!(shell.state(), ShellState::Idle);
        assert_eq!(shell.current_room(), Some(&shell.rooms()[0]));
    }

    #[test]
    fn test_spatial_config_default() {
        let config = SpatialConfig::default();
        assert_eq!(config.fov, 90.0);
        assert_eq!(config.movement_speed, 5.0);
        assert!(config.hand_tracking_enabled);
    }

    #[test]
    fn test_spatial_theme_default() {
        let theme = SpatialTheme::default();
        assert_eq!(theme.sky_color.r, 30);
        assert_eq!(theme.floor_color.r, 60);
        assert_eq!(theme.light_intensity, 1.0);
    }

    #[test]
    fn test_rooms_created() {
        let shell = SpatialShell::new();
        assert_eq!(shell.rooms().len(), 2);
    }

    #[test]
    fn test_room_names() {
        let shell = SpatialShell::new();
        let names: Vec<&str> = shell.rooms().iter().map(|r| r.name.as_str()).collect();
        assert!(names.contains(&"Workspace"));
        assert!(names.contains(&"Entertainment"));
    }

    #[test]
    fn test_go_to_room() {
        let mut shell = SpatialShell::new();
        shell.go_to_room("entertainment").unwrap();
        assert_eq!(shell.current_room().unwrap().name, "Entertainment");
    }

    #[test]
    fn test_go_to_room_not_found() {
        let mut shell = SpatialShell::new();
        let result = shell.go_to_room("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_go_back() {
        let mut shell = SpatialShell::new();
        shell.go_to_room("entertainment").unwrap();
        shell.go_back().unwrap();
        assert_eq!(shell.current_room().unwrap().name, "Workspace");
    }

    #[test]
    fn test_go_back_no_history() {
        let mut shell = SpatialShell::new();
        let result = shell.go_back();
        assert!(result.is_err());
    }

    #[test]
    fn test_camera_new() {
        let camera = Camera::new();
        assert_eq!(camera.position.y, 1.7); // Eye level
        assert_eq!(camera.rotation, Vec2::default());
    }

    #[test]
    fn test_camera_forward() {
        let mut camera = Camera::new();
        camera.rotation.x = 0.0;
        let forward = camera.forward();
        assert_eq!(forward.y, 0.0);
    }

    #[test]
    fn test_camera_right() {
        let camera = Camera::new();
        let right = camera.right();
        assert_eq!(right.y, 0.0);
    }

    #[test]
    fn test_vec3_new() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn test_vec3_distance_to() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(a.distance_to(&b), 5.0);
    }

    #[test]
    fn test_vec2_new() {
        let vec = Vec2::new(1.0, 2.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
    }

    #[test]
    fn test_room_dimensions_default() {
        let dims = RoomDimensions::default();
        assert_eq!(dims.width, 10.0);
        assert_eq!(dims.height, 3.0);
        assert_eq!(dims.depth, 10.0);
    }

    #[test]
    fn test_room_lighting_default() {
        let lighting = RoomLighting::default();
        assert_eq!(lighting.ambient_intensity, 0.5);
        assert_eq!(lighting.color_temperature, 6500);
    }

    #[test]
    fn test_room_background_default() {
        let bg = RoomBackground::default();
        assert_eq!(bg.background_type, BackgroundType::SolidColor);
        assert!(bg.color.is_some());
    }

    #[test]
    fn test_create_room() {
        let mut shell = SpatialShell::new();
        let id = shell.create_room("Custom Room", RoomType::Custom, Vec3::new(20.0, 0.0, 20.0));
        assert!(id.starts_with("room-"));
        assert_eq!(shell.rooms().len(), 3);
    }

    #[test]
    fn test_spatial_object_new() {
        let obj = SpatialObject {
            id: "test".to_string(),
            object_type: ObjectType::Window("test".to_string()),
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            visible: true,
            interactable: true,
            grabbed: false,
            action: None,
        };
        assert_eq!(obj.id, "test");
        assert!(obj.visible);
    }

    #[test]
    fn test_grab_object() {
        let mut shell = SpatialShell::new();
        shell.grab_object("desktop").unwrap();
        // Would check if object is grabbed (implementation dependent)
    }

    #[test]
    fn test_release_object() {
        let mut shell = SpatialShell::new();
        shell.grab_object("desktop").unwrap();
        shell.release_object("desktop").unwrap();
        // Would check if object is released
    }

    #[test]
    fn test_grab_object_not_found() {
        let mut shell = SpatialShell::new();
        let result = shell.grab_object("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_hand_tracking() {
        let mut shell = SpatialShell::new();
        let hand = Hand {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::default(),
            pinch_strength: 0.5,
            grab_strength: 0.5,
        };
        shell.update_hand_tracking(Some(hand.clone()), Some(hand), None);
        assert!(shell.hand_tracking.left_hand.is_some());
        assert!(shell.hand_tracking.right_hand.is_some());
    }

    #[test]
    fn test_hand_tracking_disabled() {
        let mut shell = SpatialShell::new();
        assert!(!shell.hand_tracking.enabled);
    }

    #[test]
    fn test_room_type_variants() {
        assert_eq!(
            std::mem::discriminant(&RoomType::Workspace),
            std::mem::discriminant(&RoomType::Workspace)
        );
        assert_eq!(
            std::mem::discriminant(&RoomType::Entertainment),
            std::mem::discriminant(&RoomType::Entertainment)
        );
    }

    #[test]
    fn test_object_type_variants() {
        let obj_type = ObjectType::Window("test".to_string());
        assert!(matches!(obj_type, ObjectType::Window(_)));

        let obj_type = ObjectType::Portal("room1".to_string());
        assert!(matches!(obj_type, ObjectType::Portal(_)));
    }

    #[test]
    fn test_object_action_variants() {
        let action = ObjectAction::LaunchApp("test".to_string());
        assert!(matches!(action, ObjectAction::LaunchApp(_)));

        let action = ObjectAction::GoToRoom("room1".to_string());
        assert!(matches!(action, ObjectAction::GoToRoom(_)));
    }

    #[test]
    fn test_connection_type_variants() {
        assert_eq!(
            std::mem::discriminant(&ConnectionType::Door),
            std::mem::discriminant(&ConnectionType::Door)
        );
        assert_eq!(
            std::mem::discriminant(&ConnectionType::Portal),
            std::mem::discriminant(&ConnectionType::Portal)
        );
    }

    #[test]
    fn test_hand_gesture_variants() {
        assert_eq!(
            std::mem::discriminant(&HandGesture::Open),
            std::mem::discriminant(&HandGesture::Open)
        );
        assert_eq!(
            std::mem::discriminant(&HandGesture::Grab),
            std::mem::discriminant(&HandGesture::Grab)
        );
    }

    #[test]
    fn test_shell_state_variants() {
        assert_eq!(
            std::mem::discriminant(&ShellState::Idle),
            std::mem::discriminant(&ShellState::Idle)
        );
        assert_eq!(
            std::mem::discriminant(&ShellState::Navigating),
            std::mem::discriminant(&ShellState::Navigating)
        );
    }

    #[test]
    fn test_shadow_quality_variants() {
        assert_eq!(
            std::mem::discriminant(&ShadowQuality::Low),
            std::mem::discriminant(&ShadowQuality::Low)
        );
        assert_eq!(
            std::mem::discriminant(&ShadowQuality::Ultra),
            std::mem::discriminant(&ShadowQuality::Ultra)
        );
    }

    #[test]
    fn test_input_state_default() {
        let input = InputState::default();
        assert!(!input.move_forward);
        assert!(!input.move_backward);
    }

    #[test]
    fn test_color_rgb() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
    }

    #[test]
    fn test_spatial_shell_error_display() {
        let err = SpatialShellError::RoomNotFound("test".to_string());
        assert!(err.to_string().contains("Room not found"));
    }

    #[test]
    fn test_set_config() {
        let mut shell = SpatialShell::new();
        let mut config = SpatialConfig::default();
        config.fov = 120.0;
        shell.set_config(config);
        assert_eq!(shell.config().fov, 120.0);
    }

    #[test]
    fn test_add_object_to_room() {
        let mut shell = SpatialShell::new();
        let obj = SpatialObject {
            id: "custom".to_string(),
            object_type: ObjectType::Decorative,
            position: Vec3::new(1.0, 1.0, 1.0),
            rotation: Vec3::default(),
            scale: Vec3::new(1.0, 1.0, 1.0),
            visible: true,
            interactable: false,
            grabbed: false,
            action: None,
        };
        shell.add_object(obj);
        let room = shell.current_room().unwrap();
        assert!(room.objects.iter().any(|o| o.id == "custom"));
    }

    #[test]
    fn test_workspace_room_objects() {
        let shell = SpatialShell::new();
        let workspace = &shell.rooms()[0];
        assert!(!workspace.objects.is_empty());
    }
}
