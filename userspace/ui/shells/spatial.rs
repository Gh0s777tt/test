//! # Spatial Shell - 3D Room-Based Desktop Environment
//! 
//! Immersive 3D desktop environment with room-based navigation.
//! Applications and widgets are organized in a 3D space with depth and perspective.

use super::Shell;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

use crate::ui::flux::{InputManager, InputEvent, GestureEvent, ThemeManager, ColorPalette};

/// 3D position
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn distance(&self, other: &Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn translate(&self, dx: f32, dy: f32, dz: f32) -> Self {
        Self::new(self.x + dx, self.y + dy, self.z + dz)
    }
}

/// 3D rotation (Euler angles in degrees)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rotation {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Rotation {
    pub fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
        Self { pitch, yaw, roll }
    }

    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn to_radians(&self) -> Self {
        Self::new(
            self.pitch.to_radians(),
            self.yaw.to_radians(),
            self.roll.to_radians(),
        )
    }
}

/// 3D transform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform3D {
    pub position: Vec3,
    pub rotation: Rotation,
    pub scale: f32,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            position: Vec3::origin(),
            rotation: Rotation::identity(),
            scale: 1.0,
        }
    }
}

impl Transform3D {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn with_rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

/// Window/app in 3D space
#[derive(Debug, Clone)]
pub struct SpatialWindow {
    pub id: String,
    pub title: String,
    pub app_id: String,
    pub transform: Transform3D,
    pub size: Vec3, // width, height, depth
    pub visible: bool,
    pub focused: bool,
    pub minimized: bool,
    pub opacity: f32,
    pub created_at: Instant,
}

impl SpatialWindow {
    pub fn new(id: &str, title: &str, app_id: &str, transform: Transform3D) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            app_id: app_id.to_string(),
            transform,
            size: Vec3::new(400.0, 300.0, 10.0),
            visible: true,
            focused: false,
            minimized: false,
            opacity: 1.0,
            created_at: Instant::now(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32, depth: f32) -> Self {
        self.size = Vec3::new(width, height, depth);
        self
    }

    pub fn get_center(&self) -> Vec3 {
        self.transform.position
    }

    pub fn get_bounds(&self) -> (Vec3, Vec3) {
        let half_size = Vec3::new(
            self.size.x * 0.5,
            self.size.y * 0.5,
            self.size.z * 0.5,
        );
        let min = self.transform.position.translate(-half_size.x, -half_size.y, -half_size.z);
        let max = self.transform.position.translate(half_size.x, half_size.y, half_size.z);
        (min, max)
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        let (min, max) = self.get_bounds();
        point.x >= min.x && point.x <= max.x
            && point.y >= min.y && point.y <= max.y
            && point.z >= min.z && point.z <= max.z
    }
}

/// Room/camera view
#[derive(Debug, Clone)]
pub struct CameraView {
    pub position: Vec3,
    pub rotation: Rotation,
    pub fov: f32, // Field of view in degrees
    pub near_plane: f32,
    pub far_plane: f32,
}

impl Default for CameraView {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 500.0),
            rotation: Rotation::identity(),
            fov: 75.0,
            near_plane: 1.0,
            far_plane: 2000.0,
        }
    }
}

impl CameraView {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn look_at(&mut self, target: Vec3) {
        let dx = target.x - self.position.x;
        let dy = target.y - self.position.y;
        let dz = target.z - self.position.z;
        
        self.rotation.yaw = (-dx).atan2(-dz).to_degrees();
        self.rotation.pitch = dy.atan2((dx * dx + dz * dz).sqrt()).to_degrees();
    }

    pub fn move_forward(&mut self, distance: f32) {
        let rad = self.rotation.to_radians();
        self.position.x -= rad.yaw.sin() * distance;
        self.position.z -= rad.yaw.cos() * distance;
    }

    pub fn move_backward(&mut self, distance: f32) {
        self.move_forward(-distance);
    }

    pub fn move_left(&mut self, distance: f32) {
        let rad = self.rotation.to_radians();
        self.position.x -= rad.yaw.cos() * distance;
        self.position.z += rad.yaw.sin() * distance;
    }

    pub fn move_right(&mut self, distance: f32) {
        self.move_left(-distance);
    }

    pub fn move_up(&mut self, distance: f32) {
        self.position.y += distance;
    }

    pub fn move_down(&mut self, distance: f32) {
        self.position.y -= distance;
    }
}

/// Room layout type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomLayout {
    Grid,
    Circle,
    Freeform,
    Linear,
}

/// Spatial room
#[derive(Debug, Clone)]
pub struct SpatialRoom {
    pub id: String,
    pub name: String,
    pub layout: RoomLayout,
    pub windows: Vec<SpatialWindow>,
    pub camera: CameraView,
    pub ambient_light: f32,
    pub background_color: String,
}

impl SpatialRoom {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            layout: RoomLayout::Grid,
            windows: Vec::new(),
            camera: CameraView::default(),
            ambient_light: 0.5,
            background_color: "#1a1a2e".to_string(),
        }
    }

    pub fn with_layout(mut self, layout: RoomLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn add_window(&mut self, window: SpatialWindow) {
        self.windows.push(window);
        self.arrange_windows();
    }

    pub fn remove_window(&mut self, id: &str) -> Option<SpatialWindow> {
        self.windows.iter().position(|w| w.id == id).map(|i| self.windows.remove(i))
    }

    pub fn get_window(&self, id: &str) -> Option<&SpatialWindow> {
        self.windows.iter().find(|w| w.id == id)
    }

    pub fn get_window_mut(&mut self, id: &str) -> Option<&mut SpatialWindow> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    pub fn focus_window(&mut self, id: &str) {
        for window in &mut self.windows {
            window.focused = window.id == id;
        }
        // Move focused window to front
        if let Some(pos) = self.windows.iter().position(|w| w.id == id) {
            let window = self.windows.remove(pos);
            self.windows.push(window);
        }
    }

    pub fn arrange_windows(&mut self) {
        match self.layout {
            RoomLayout::Grid => {
                let grid_size = (self.windows.len() as f32).sqrt().ceil() as usize;
                let spacing = 450.0;
                let start_x = -((grid_size - 1) as f32 * spacing) * 0.5;
                let start_y = ((grid_size - 1) as f32 * spacing) * 0.5;
                
                for (i, window) in self.windows.iter_mut().enumerate() {
                    let col = i % grid_size;
                    let row = i / grid_size;
                    window.transform.position.x = start_x + col as f32 * spacing;
                    window.transform.position.y = start_y - row as f32 * spacing;
                    window.transform.position.z = 0.0;
                    window.transform.rotation = Rotation::identity();
                }
            }
            RoomLayout::Circle => {
                let radius = 400.0;
                let angle_step = 2.0 * PI / self.windows.len() as f32;
                
                for (i, window) in self.windows.iter_mut().enumerate() {
                    let angle = i as f32 * angle_step;
                    window.transform.position.x = angle.cos() * radius;
                    window.transform.position.y = angle.sin() * radius;
                    window.transform.position.z = 0.0;
                    window.transform.rotation.yaw = (-angle).to_degrees();
                }
            }
            RoomLayout::Linear => {
                let spacing = 450.0;
                let start_x = -((self.windows.len() - 1) as f32 * spacing) * 0.5;
                
                for (i, window) in self.windows.iter_mut().enumerate() {
                    window.transform.position.x = start_x + i as f32 * spacing;
                    window.transform.position.y = 0.0;
                    window.transform.position.z = 0.0;
                    window.transform.rotation = Rotation::identity();
                }
            }
            RoomLayout::Freeform => {
                // Keep existing positions
            }
        }
    }

    pub fn get_visible_windows(&self) -> Vec<&SpatialWindow> {
        self.windows.iter().filter(|w| w.visible).collect()
    }

    pub fn get_focused_window(&self) -> Option<&SpatialWindow> {
        self.windows.iter().find(|w| w.focused)
    }
}

/// Navigation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationMode {
    Walk,
    Fly,
    Orbit,
}

/// Spatial shell configuration
#[derive(Debug, Clone)]
pub struct SpatialShellConfig {
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub mouse_sensitivity: f32,
    pub enable_gestures: bool,
    pub enable_haptics: bool,
    pub default_navigation_mode: NavigationMode,
    pub room_transition_duration: Duration,
}

impl Default for SpatialShellConfig {
    fn default() -> Self {
        Self {
            movement_speed: 500.0,
            rotation_speed: 90.0,
            mouse_sensitivity: 0.1,
            enable_gestures: true,
            enable_haptics: true,
            default_navigation_mode: NavigationMode::Walk,
            room_transition_duration: Duration::from_millis(500),
        }
    }
}

/// Spatial shell
pub struct SpatialShell {
    config: SpatialShellConfig,
    input_manager: InputManager,
    theme_manager: ThemeManager,
    rooms: HashMap<String, SpatialRoom>,
    current_room_id: Option<String>,
    navigation_mode: NavigationMode,
    running: bool,
}

impl SpatialShell {
    pub fn new() -> Self {
        let mut shell = Self {
            config: SpatialShellConfig::default(),
            input_manager: InputManager::new(),
            theme_manager: ThemeManager::new(),
            rooms: HashMap::new(),
            current_room_id: None,
            navigation_mode: NavigationMode::Walk,
            running: false,
        };

        // Create default room
        let default_room = SpatialRoom::new("default", "Main Room");
        shell.rooms.insert("default".to_string(), default_room);
        shell.current_room_id = Some("default".to_string());

        shell
    }

    pub fn with_config(mut self, config: SpatialShellConfig) -> Self {
        self.config = config;
        self
    }

    pub fn add_room(&mut self, room: SpatialRoom) {
        self.rooms.insert(room.id.clone(), room);
    }

    pub fn switch_room(&mut self, room_id: &str) -> Result<(), String> {
        if self.rooms.contains_key(room_id) {
            self.current_room_id = Some(room_id.to_string());
            Ok(())
        } else {
            Err(format!("Room '{}' not found", room_id))
        }
    }

    pub fn get_current_room(&self) -> Option<&SpatialRoom> {
        self.current_room_id.as_ref().and_then(|id| self.rooms.get(id))
    }

    pub fn get_current_room_mut(&mut self) -> Option<&mut SpatialRoom> {
        self.current_room_id.as_ref().and_then(|id| self.rooms.get_mut(id))
    }

    pub fn add_window(&mut self, window: SpatialWindow) {
        if let Some(room) = self.get_current_room_mut() {
            room.add_window(window);
        }
    }

    pub fn focus_window(&mut self, window_id: &str) {
        if let Some(room) = self.get_current_room_mut() {
            room.focus_window(window_id);
        }
    }

    pub fn set_room_layout(&mut self, layout: RoomLayout) {
        if let Some(room) = self.get_current_room_mut() {
            room.layout = layout;
            room.arrange_windows();
        }
    }

    pub fn update(&mut self, dt: Duration) {
        // Process input and update camera position
        if let Some(room) = self.get_current_room_mut() {
            // Camera movement logic would go here
            // This is a placeholder for input-driven camera updates
        }
    }

    pub fn get_window_count(&self) -> usize {
        self.get_current_room().map(|r| r.windows.len()).unwrap_or(0)
    }
}

impl Shell for SpatialShell {
    fn run(&self) {
        println!("Starting Spatial Shell - 3D Room-Based Desktop Environment");
        println!("Navigate the 3D space with mouse and keyboard");
        println!("Available layouts: Grid, Circle, Linear, Freeform");
        println!("Navigation modes: Walk, Fly, Orbit");
    }
}

impl Default for SpatialShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_creation() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vec3_distance() {
        let v1 = Vec3::origin();
        let v2 = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(v1.distance(&v2), 5.0);
    }

    #[test]
    fn test_rotation_creation() {
        let r = Rotation::new(90.0, 180.0, 270.0);
        assert_eq!(r.pitch, 90.0);
        assert_eq!(r.yaw, 180.0);
        assert_eq!(r.roll, 270.0);
    }

    #[test]
    fn test_spatial_window_creation() {
        let transform = Transform3D::new(Vec3::origin());
        let window = SpatialWindow::new("test", "Test", "app", transform);
        assert_eq!(window.id, "test");
        assert_eq!(window.title, "Test");
        assert_eq!(window.visible, true);
    }

    #[test]
    fn test_camera_view_creation() {
        let camera = CameraView::new(Vec3::new(0.0, 0.0, 500.0));
        assert_eq!(camera.position.z, 500.0);
        assert_eq!(camera.fov, 75.0);
    }

    #[test]
    fn test_spatial_room_creation() {
        let room = SpatialRoom::new("test", "Test Room");
        assert_eq!(room.id, "test");
        assert_eq!(room.name, "Test Room");
        assert_eq!(room.layout, RoomLayout::Grid);
    }

    #[test]
    fn test_spatial_room_add_window() {
        let mut room = SpatialRoom::new("test", "Test Room");
        let transform = Transform3D::new(Vec3::origin());
        let window = SpatialWindow::new("win1", "Window 1", "app", transform);
        room.add_window(window);
        assert_eq!(room.windows.len(), 1);
    }

    #[test]
    fn test_spatial_shell_creation() {
        let shell = SpatialShell::new();
        assert!(shell.current_room_id.is_some());
        assert_eq!(shell.get_window_count(), 0);
    }
}