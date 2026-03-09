//! VantisOS Spatial Shell
//! 
//! A 3D spatial desktop with room-based navigation.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Spatial Shell - 3D room-based desktop environment
pub struct SpatialShell {
    /// Shell configuration
    config: SpatialConfig,
    /// Shell state
    state: ShellState,
    /// Rooms (virtual spaces)
    rooms: Vec<Room>,
    /// Current room index
    current_room: usize,
    /// Camera
    camera: Camera,
    /// Spatial objects
    objects: Vec<SpatialObject>,
    /// Navigation history
    nav_history: VecDeque<usize>,
    /// Input state
    input: InputState,
    /// Hand tracking
    hand_tracking: HandTracking,
    /// Shell is initialized
    initialized: bool,
}

/// Spatial configuration
#[derive(Clone, Debug)]
pub struct SpatialConfig {
    /// Field of view in degrees
    pub fov: f32,
    /// Near clipping plane
    pub near_clip: f32,
    /// Far clipping plane
    pub far_clip: f32,
    /// Movement speed
    pub movement_speed: f32,
    /// Rotation speed
    pub rotation_speed: f32,
    /// Hand tracking enabled
    pub hand_tracking_enabled: bool,
    /// Voice control enabled
    pub voice_control_enabled: bool,
    /// Eye tracking enabled
    pub eye_tracking_enabled: bool,
    /// Theme
    pub theme: SpatialTheme,
}

impl Default for SpatialConfig {
    fn default() -> Self {
        Self {
            fov: 90.0,
            near_clip: 0.1,
            far_clip: 1000.0,
            movement_speed: 5.0,
            rotation_speed: 2.0,
            hand_tracking_enabled: true,
            voice_control_enabled: true,
            eye_tracking_enabled: false,
            theme: SpatialTheme::default(),
        }
    }
}

/// Spatial theme
#[derive(Clone, Debug)]
pub struct SpatialTheme {
    /// Sky color
    pub sky_color: Color,
    /// Floor color
    pub floor_color: Color,
    /// Wall color
    pub wall_color: Color,
    /// Ambient light color
    pub ambient_light: Color,
    /// Directional light color
    pub directional_light: Color,
    /// Light intensity
    pub light_intensity: f32,
    /// Shadow quality
    pub shadow_quality: ShadowQuality,
}

impl Default for SpatialTheme {
    fn default() -> Self {
        Self {
            sky_color: Color::rgb(30, 30, 40),
            floor_color: Color::rgb(60, 60, 70),
            wall_color: Color::rgb(50, 50, 60),
            ambient_light: Color::rgb(100, 100, 120),
            directional_light: Color::rgb(255, 255, 255),
            light_intensity: 1.0,
            shadow_quality: ShadowQuality::Medium,
        }
    }
}

/// Shadow quality
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadowQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Shell state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellState {
    /// Idle
    Idle,
    /// Navigating
    Navigating,
    /// Interacting with object
    Interacting,
    /// Grabbing object
    Grabbing,
    /// Voice command listening
    VoiceListening,
}

/// Room - Virtual space
#[derive(Clone, Debug)]
pub struct Room {
    /// Room ID
    pub id: String,
    /// Room name
    pub name: String,
    /// Room type
    pub room_type: RoomType,
    /// Position in 3D space
    pub position: Vec3,
    /// Room dimensions
    pub dimensions: RoomDimensions,
    /// Objects in room
    pub objects: Vec<SpatialObject>,
    /// Connected rooms
    pub connections: Vec<RoomConnection>,
    /// Lighting
    pub lighting: RoomLighting,
    /// Background
    pub background: RoomBackground,
}

/// Room type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoomType {
    /// Main workspace
    Workspace,
    /// Entertainment room
    Entertainment,
    /// Meeting room
    Meeting,
    /// Gallery room
    Gallery,
    /// Custom room
    Custom,
}

/// 3D Vector
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    
    pub fn distance_to(&self, other: &Vec3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Room dimensions
#[derive(Clone, Copy, Debug)]
pub struct RoomDimensions {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

impl Default for RoomDimensions {
    fn default() -> Self {
        Self {
            width: 10.0,
            height: 3.0,
            depth: 10.0,
        }
    }
}

/// Spatial object - 3D interactive element
#[derive(Clone, Debug)]
pub struct SpatialObject {
    /// Object ID
    pub id: String,
    /// Object type
    pub object_type: ObjectType,
    /// Position
    pub position: Vec3,
    /// Rotation (Euler angles)
    pub rotation: Vec3,
    /// Scale
    pub scale: Vec3,
    /// Is visible
    pub visible: bool,
    /// Is interactable
    pub interactable: bool,
    /// Is grabbed
    pub grabbed: bool,
    /// Action
    pub action: Option<ObjectAction>,
}

/// Object type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectType {
    /// Window/app
    Window(String),
    /// Icon
    Icon(String),
    /// File
    File(String),
    /// Portal to another room
    Portal(String),
    /// Decorative element
    Decorative,
    /// Light source
    Light,
    /// Sound source
    Sound,
}

/// Object action
#[derive(Clone, Debug)]
pub enum ObjectAction {
    /// Launch application
    LaunchApp(String),
    /// Open file
    OpenFile(String),
    /// Navigate to room
    GoToRoom(String),
    /// Execute command
    Command(String),
    /// Custom action
    Custom(String),
}

/// Room connection
#[derive(Clone, Debug)]
pub struct RoomConnection {
    /// Connected room ID
    pub room_id: String,
    /// Connection position
    pub position: Vec3,
    /// Connection type
    pub connection_type: ConnectionType,
}

/// Connection type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionType {
    Door,
    Portal,
    Stairs,
    Elevator,
}

/// Room lighting
#[derive(Clone, Debug)]
pub struct RoomLighting {
    /// Ambient light intensity
    pub ambient_intensity: f32,
    /// Directional light direction
    pub light_direction: Vec3,
    /// Light color temperature (Kelvin)
    pub color_temperature: u32,
}

impl Default for RoomLighting {
    fn default() -> Self {
        Self {
            ambient_intensity: 0.5,
            light_direction: Vec3::new(0.0, -1.0, 0.0),
            color_temperature: 6500,
        }
    }
}

/// Room background
#[derive(Clone, Debug)]
pub struct RoomBackground {
    /// Background type
    pub background_type: BackgroundType,
    /// Color or skybox
    pub color: Option<Color>,
    pub skybox: Option<String>,
}

/// Background type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackgroundType {
    SolidColor,
    Gradient,
    Skybox,
    EnvironmentMap,
}

impl Default for RoomBackground {
    fn default() -> Self {
        Self {
            background_type: BackgroundType::SolidColor,
            color: Some(Color::rgb(30, 30, 40)),
            skybox: None,
        }
    }
}

/// Camera
#[derive(Clone, Debug)]
pub struct Camera {
    /// Position
    pub position: Vec3,
    /// Rotation (yaw, pitch)
    pub rotation: Vec2,
    /// Field of view
    pub fov: f32,
    /// Target (for smooth camera movement)
    pub target: Option<Vec3>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 1.7, 5.0), // Eye level
            rotation: Vec2::new(0.0, 0.0),
            fov: 90.0,
            target: None,
        }
    }
    
    pub fn forward(&self) -> Vec3 {
        Vec3 {
            x: self.rotation.x.sin() * self.rotation.y.cos(),
            y: -self.rotation.y.sin(),
            z: -self.rotation.x.cos() * self.rotation.y.cos(),
        }
    }
    
    pub fn right(&self) -> Vec3 {
        Vec3 {
            x: self.rotation.x.cos(),
            y: 0.0,
            z: self.rotation.x.sin(),
        }
    }
}

/// 2D Vector for rotation
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Input state
#[derive(Clone, Debug, Default)]
pub struct InputState {
    /// Movement keys
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_up: bool,
    pub move_down: bool,
    /// Mouse delta
    pub mouse_delta: Vec2,
    /// Click position
    pub click_position: Option<Vec2>,
}

/// Hand tracking
#[derive(Clone, Debug)]
pub struct HandTracking {
    /// Is enabled
    pub enabled: bool,
    /// Left hand position
    pub left_hand: Option<Hand>,
    /// Right hand position
    pub right_hand: Option<Hand>,
    /// Gesture detection
    pub gesture: Option<HandGesture>,
}

/// Hand
#[derive(Clone, Debug)]
pub struct Hand {
    /// Position
    pub position: Vec3,
    /// Rotation
    pub rotation: Vec3,
    /// Pinch strength (0.0 - 1.0)
    pub pinch_strength: f32,
    /// Grab strength (0.0 - 1.0)
    pub grab_strength: f32,
}

/// Hand gesture
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandGesture {
    /// Open palm
    Open,
    /// Pointing
    Point,
    /// Pinch
    Pinch,
    /// Grab
    Grab,
    /// Fist
    Fist,
    /// Wave
    Wave,
}

/// Color
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

/// Spatial shell errors
#[derive(Debug, Clone)]
pub enum SpatialShellError {
    /// Room not found
    RoomNotFound(String),
    /// Object not found
    ObjectNotFound(String),
    /// Invalid position
    InvalidPosition,
    /// Shell not initialized
    ShellNotInitialized,
    /// Camera movement failed
    CameraMoveFailed(String),
    /// Navigation failed
    NavigationFailed(String),
}

impl std::fmt::Display for SpatialShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SpatialShellError::RoomNotFound(id) => write!(f, "Room not found: {}", id),
            SpatialShellError::ObjectNotFound(id) => write!(f, "Object not found: {}", id),
            SpatialShellError::InvalidPosition => write!(f, "Invalid position"),
            SpatialShellError::ShellNotInitialized => write!(f, "Shell not initialized"),
            SpatialShellError::CameraMoveFailed(msg) => write!(f, "Camera move failed: {}", msg),
            SpatialShellError::NavigationFailed(msg) => write!(f, "Navigation failed: {}", msg),
        }
    }
}

impl std::error::Error for SpatialShellError {}

impl SpatialShell {
    /// Create a new spatial shell
    pub fn new() -> Self {
        Self::with_config(SpatialConfig::default())
    }
    
    /// Create with configuration
    pub fn with_config(config: SpatialConfig) -> Self {
        let mut shell = Self {
            config,
            state: ShellState::Idle,
            rooms: Vec::new(),
            current_room: 0,
            camera: Camera::new(),
            objects: Vec::new(),
            nav_history: VecDeque::with_capacity(20),
            input: InputState::default(),
            hand_tracking: HandTracking {
                enabled: false,
                left_hand: None,
                right_hand: None,
                gesture: None,
            },
            initialized: false,
        };
        
        shell.init_default_rooms();
        shell.initialized = true;
        
        shell
    }
    
    /// Initialize default rooms
    fn init_default_rooms(&mut self) {
        // Main workspace room
        let workspace = Room {
            id: "workspace".to_string(),
            name: "Workspace".to_string(),
            room_type: RoomType::Workspace,
            position: Vec3::new(0.0, 0.0, 0.0),
            dimensions: RoomDimensions::default(),
            objects: self.create_workspace_objects(),
            connections: vec![
                RoomConnection {
                    room_id: "entertainment".to_string(),
                    position: Vec3::new(5.0, 0.0, 0.0),
                    connection_type: ConnectionType::Door,
                },
            ],
            lighting: RoomLighting::default(),
            background: RoomBackground::default(),
        };
        
        // Entertainment room
        let entertainment = Room {
            id: "entertainment".to_string(),
            name: "Entertainment".to_string(),
            room_type: RoomType::Entertainment,
            position: Vec3::new(10.0, 0.0, 0.0),
            dimensions: RoomDimensions::default(),
            objects: self.create_entertainment_objects(),
            connections: vec![
                RoomConnection {
                    room_id: "workspace".to_string(),
                    position: Vec3::new(-5.0, 0.0, 0.0),
                    connection_type: ConnectionType::Door,
                },
            ],
            lighting: RoomLighting::default(),
            background: RoomBackground {
                background_type: BackgroundType::Skybox,
                color: None,
                skybox: Some("skybox_night.png".to_string()),
            },
        };
        
        self.rooms.push(workspace);
        self.rooms.push(entertainment);
    }
    
    /// Create workspace objects
    fn create_workspace_objects(&self) -> Vec<SpatialObject> {
        vec![
            SpatialObject {
                id: "desktop".to_string(),
                object_type: ObjectType::Window("desktop".to_string()),
                position: Vec3::new(0.0, 0.8, -2.0),
                rotation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(2.0, 1.5, 0.1),
                visible: true,
                interactable: true,
                grabbed: false,
                action: Some(ObjectAction::LaunchApp("file-manager".to_string())),
            },
            SpatialObject {
                id: "terminal".to_string(),
                object_type: ObjectType::Window("terminal".to_string()),
                position: Vec3::new(-2.5, 0.8, -2.0),
                rotation: Vec3::new(0.0, 0.3, 0.0),
                scale: Vec3::new(1.5, 1.0, 0.1),
                visible: true,
                interactable: true,
                grabbed: false,
                action: Some(ObjectAction::LaunchApp("terminal".to_string())),
            },
            SpatialObject {
                id: "portal-entertainment".to_string(),
                object_type: ObjectType::Portal("entertainment".to_string()),
                position: Vec3::new(5.0, 1.0, 0.0),
                rotation: Vec3::new(0.0, std::f32::consts::PI / 2.0, 0.0),
                scale: Vec3::new(2.0, 2.5, 0.2),
                visible: true,
                interactable: true,
                grabbed: false,
                action: Some(ObjectAction::GoToRoom("entertainment".to_string())),
            },
        ]
    }
    
    /// Create entertainment room objects
    fn create_entertainment_objects(&self) -> Vec<SpatialObject> {
        vec![
            SpatialObject {
                id: "portal-workspace".to_string(),
                object_type: ObjectType::Portal("workspace".to_string()),
                position: Vec3::new(-5.0, 1.0, 0.0),
                rotation: Vec3::new(0.0, std::f32::consts::PI / 2.0, 0.0),
                scale: Vec3::new(2.0, 2.5, 0.2),
                visible: true,
                interactable: true,
                grabbed: false,
                action: Some(ObjectAction::GoToRoom("workspace".to_string())),
            },
        ]
    }
    
    /// Navigate to room
    pub fn go_to_room(&mut self, room_id: &str) -> Result<(), SpatialShellError> {
        let idx = self.rooms.iter()
            .position(|r| r.id == room_id)
            .ok_or_else(|| SpatialShellError::RoomNotFound(room_id.to_string()))?;
        
        // Add to history
        self.nav_history.push_back(self.current_room);
        
        // Limit history
        if self.nav_history.len() > 20 {
            self.nav_history.pop_front();
        }
        
        self.current_room = idx;
        self.state = ShellState::Navigating;
        
        // Move camera to room entrance
        self.camera.target = Some(Vec3::new(0.0, 1.7, 5.0));
        
        Ok(())
    }
    
    /// Go back
    pub fn go_back(&mut self) -> Result<(), SpatialShellError> {
        if let Some(prev) = self.nav_history.pop_back() {
            let room = self.rooms.get(prev).unwrap();
            self.go_to_room(&room.id)
        } else {
            Err(SpatialShellError::NavigationFailed("No history".to_string()))
        }
    }
    
    /// Handle input
    pub fn handle_input(&mut self, delta_time: f32) {
        if self.state == ShellState::Idle || self.state == ShellState::Navigating {
            let speed = self.config.movement_speed * delta_time;
            let forward = self.camera.forward();
            let right = self.camera.right();
            
            if self.input.move_forward {
                self.camera.position.x += forward.x * speed;
                self.camera.position.y += forward.y * speed;
                self.camera.position.z += forward.z * speed;
            }
            if self.input.move_backward {
                self.camera.position.x -= forward.x * speed;
                self.camera.position.y -= forward.y * speed;
                self.camera.position.z -= forward.z * speed;
            }
            if self.input.move_left {
                self.camera.position.x -= right.x * speed;
                self.camera.position.z -= right.z * speed;
            }
            if self.input.move_right {
                self.camera.position.x += right.x * speed;
                self.camera.position.z += right.z * speed;
            }
            if self.input.move_up {
                self.camera.position.y += speed;
            }
            if self.input.move_down {
                self.camera.position.y -= speed;
            }
            
            // Mouse look
            let rot_speed = self.config.rotation_speed * delta_time;
            self.camera.rotation.x += self.input.mouse_delta.x * rot_speed;
            self.camera.rotation.y = (self.camera.rotation.y + self.input.mouse_delta.y * rot_speed)
                .clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);
            
            // Reset mouse delta
            self.input.mouse_delta = Vec2::new(0.0, 0.0);
        }
    }
    
    /// Handle click
    pub fn handle_click(&self, screen_x: f32, screen_y: f32) -> Option<ObjectAction> {
        // Raycast from camera
        // (simplified - would need proper raycasting implementation)
        
        // Check if clicking on any interactable object in current room
        if let Some(room) = self.rooms.get(self.current_room) {
            for obj in &room.objects {
                if obj.interactable && obj.visible {
                    // Check if ray hits object (simplified)
                    let dist = self.camera.position.distance_to(&obj.position);
                    if dist < 5.0 {
                        return obj.action.clone();
                    }
                }
            }
        }
        
        None
    }
    
    /// Grab object
    pub fn grab_object(&mut self, object_id: &str) -> Result<(), SpatialShellError> {
        if let Some(room) = self.rooms.get_mut(self.current_room) {
            if let Some(obj) = room.objects.iter_mut().find(|o| o.id == object_id) {
                if obj.interactable {
                    obj.grabbed = true;
                    self.state = ShellState::Grabbing;
                    return Ok(());
                }
            }
        }
        Err(SpatialShellError::ObjectNotFound(object_id.to_string()))
    }
    
    /// Release object
    pub fn release_object(&mut self, object_id: &str) -> Result<(), SpatialShellError> {
        if let Some(room) = self.rooms.get_mut(self.current_room) {
            if let Some(obj) = room.objects.iter_mut().find(|o| o.id == object_id) {
                obj.grabbed = false;
                self.state = ShellState::Idle;
                return Ok(());
            }
        }
        Err(SpatialShellError::ObjectNotFound(object_id.to_string()))
    }
    
    /// Update hand tracking
    pub fn update_hand_tracking(&mut self, left: Option<Hand>, right: Option<Hand>, gesture: Option<HandGesture>) {
        self.hand_tracking.left_hand = left;
        self.hand_tracking.right_hand = right;
        self.hand_tracking.gesture = gesture;
        
        if gesture.is_some() && self.state == ShellState::Idle {
            self.state = ShellState::Interacting;
        }
    }
    
    /// Create room
    pub fn create_room(&mut self, name: &str, room_type: RoomType, position: Vec3) -> String {
        let id = format!("room-{}", self.rooms.len());
        
        let room = Room {
            id: id.clone(),
            name: name.to_string(),
            room_type,
            position,
            dimensions: RoomDimensions::default(),
            objects: Vec::new(),
            connections: Vec::new(),
            lighting: RoomLighting::default(),
            background: RoomBackground::default(),
        };
        
        self.rooms.push(room);
        id
    }
    
    /// Add object to current room
    pub fn add_object(&mut self, object: SpatialObject) {
        if let Some(room) = self.rooms.get_mut(self.current_room) {
            room.objects.push(object);
        }
    }
    
    /// Get current room
    pub fn current_room(&self) -> Option<&Room> {
        self.rooms.get(self.current_room)
    }
    
    /// Get camera
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    
    /// Get state
    pub fn state(&self) -> ShellState {
        self.state
    }
    
    /// Get configuration
    pub fn config(&self) -> &SpatialConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: SpatialConfig) {
        self.config = config;
    }
}

impl Default for SpatialShell {
    fn default() -> Self {
        Self::new()
    }
}