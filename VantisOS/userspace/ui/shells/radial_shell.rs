//! VantisOS Radial Shell
//! 
//! A radial/circular menu desktop interface for touch and pen input.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Radial Shell - Circular menu desktop environment
pub struct RadialShell {
    /// Shell configuration
    config: RadialConfig,
    /// Shell state
    state: ShellState,
    /// Radial menu
    radial_menu: RadialMenu,
    /// Quick actions
    quick_actions: Vec<QuickAction>,
    /// Active application
    active_app: Option<ActiveApplication>,
    /// Notification center
    notifications: NotificationCenter,
    /// Recent items
    recent_items: VecDeque<RecentItem>,
    /// Shell is initialized
    initialized: bool,
}

/// Radial configuration
#[derive(Clone, Debug)]
pub struct RadialConfig {
    /// Menu radius
    pub menu_radius: f32,
    /// Inner radius (center circle)
    pub inner_radius: f32,
    /// Animation duration in ms
    pub animation_duration: u64,
    /// Auto-hide delay in ms
    pub auto_hide_delay: u64,
    /// Show labels
    pub show_labels: bool,
    /// Show icons
    pub show_icons: bool,
    /// Theme
    pub theme: RadialTheme,
    /// Haptic feedback
    pub haptic_feedback: bool,
    /// Sound feedback
    pub sound_feedback: bool,
    /// Gesture recognition
    pub gestures: GestureConfig,
}

impl Default for RadialConfig {
    fn default() -> Self {
        Self {
            menu_radius: 200.0,
            inner_radius: 60.0,
            animation_duration: 200,
            auto_hide_delay: 3000,
            show_labels: true,
            show_icons: true,
            theme: RadialTheme::default(),
            haptic_feedback: true,
            sound_feedback: true,
            gestures: GestureConfig::default(),
        }
    }
}

/// Radial theme
#[derive(Clone, Debug)]
pub struct RadialTheme {
    /// Background color
    pub background: Color,
    /// Menu background
    pub menu_background: Color,
    /// Menu border
    pub menu_border: Color,
    /// Icon color
    pub icon_color: Color,
    /// Label color
    pub label_color: Color,
    /// Highlight color
    pub highlight_color: Color,
    /// Shadow opacity
    pub shadow_opacity: f32,
}

impl Default for RadialTheme {
    fn default() -> Self {
        Self {
            background: Color::rgba(0, 0, 0, 200),
            menu_background: Color::rgba(30, 30, 30, 240),
            menu_border: Color::rgb(86, 156, 214),
            icon_color: Color::rgb(212, 212, 212),
            label_color: Color::rgb(212, 212, 212),
            highlight_color: Color::rgb(86, 156, 214),
            shadow_opacity: 0.5,
        }
    }
}

/// Gesture configuration
#[derive(Clone, Debug)]
pub struct GestureConfig {
    /// Enable swipe gestures
    pub swipe_enabled: bool,
    /// Enable pinch gestures
    pub pinch_enabled: bool,
    /// Swipe threshold in pixels
    pub swipe_threshold: f32,
    /// Pinch threshold
    pub pinch_threshold: f32,
    /// Long press duration in ms
    pub long_press_duration: u64,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            swipe_enabled: true,
            pinch_enabled: true,
            swipe_threshold: 50.0,
            pinch_threshold: 0.5,
            long_press_duration: 500,
        }
    }
}

/// Shell state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellState {
    /// Shell is hidden
    Hidden,
    /// Menu is showing
    MenuShowing,
    /// Menu is animating
    Animating,
    /// Application is running
    ApplicationRunning,
    /// Notification center is open
    NotificationCenter,
}

/// Radial menu
#[derive(Clone, Debug)]
pub struct RadialMenu {
    /// Menu items
    pub items: Vec<MenuItem>,
    /// Current selected item
    pub selected: Option<usize>,
    /// Submenu (if open)
    pub submenu: Option<Box<RadialMenu>>,
    /// Animation progress (0.0 - 1.0)
    pub animation_progress: f32,
    /// Menu center position
    pub center: Point,
    /// Is visible
    pub visible: bool,
}

/// Menu item
#[derive(Clone, Debug)]
pub struct MenuItem {
    /// Item ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Icon
    pub icon: String,
    /// Action
    pub action: MenuAction,
    /// Submenu items
    pub submenu: Vec<MenuItem>,
    /// Is enabled
    pub enabled: bool,
    /// Badge count
    pub badge: Option<u32>,
    /// Position angle (radians)
    pub angle: f32,
}

/// Menu action
#[derive(Clone, Debug)]
pub enum MenuAction {
    /// Launch application
    LaunchApp(String),
    /// Open submenu
    OpenSubmenu,
    /// Execute command
    Command(String),
    /// Open file
    OpenFile(String),
    /// System action
    System(SystemAction),
    /// Custom action ID
    Custom(String),
}

/// System actions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemAction {
    PowerOff,
    Restart,
    Sleep,
    Lock,
    Logout,
    Settings,
    FileBrowser,
    Terminal,
}

/// Quick action
#[derive(Clone, Debug)]
pub struct QuickAction {
    /// Action ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Icon
    pub icon: String,
    /// Action
    pub action: MenuAction,
    /// Position
    pub position: QuickActionPosition,
}

/// Quick action position
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuickActionPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Left,
    Right,
    Top,
    Bottom,
}

/// Active application
#[derive(Clone, Debug)]
pub struct ActiveApplication {
    /// Application ID
    pub app_id: String,
    /// Application name
    pub name: String,
    /// Window bounds
    pub bounds: Rect,
    /// Is fullscreen
    pub fullscreen: bool,
    /// Is minimized
    pub minimized: bool,
}

/// Notification center
#[derive(Clone, Debug)]
pub struct NotificationCenter {
    /// Notifications
    pub notifications: VecDeque<Notification>,
    /// Is visible
    pub visible: bool,
    /// Max notifications
    pub max_notifications: usize,
    /// Do not disturb
    pub do_not_disturb: bool,
}

/// Notification
#[derive(Clone, Debug)]
pub struct Notification {
    /// Notification ID
    pub id: String,
    /// Title
    pub title: String,
    /// Message
    pub message: String,
    /// Icon
    pub icon: Option<String>,
    /// Timestamp
    pub timestamp: Instant,
    /// Priority
    pub priority: NotificationPriority,
    /// Is read
    pub read: bool,
    /// Action
    pub action: Option<MenuAction>,
}

/// Notification priority
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Recent item
#[derive(Clone, Debug)]
pub struct RecentItem {
    /// Item path or ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Icon
    pub icon: String,
    /// Last accessed
    pub last_accessed: Instant,
    /// Item type
    pub item_type: RecentItemType,
}

/// Recent item type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecentItemType {
    File,
    Application,
    Folder,
    URL,
}

/// Point
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/// Rectangle
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
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
    
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Radial shell errors
#[derive(Debug, Clone)]
pub enum RadialShellError {
    /// Menu not found
    MenuNotFound,
    /// Item not found
    ItemNotFound(String),
    /// Application not found
    ApplicationNotFound(String),
    /// Invalid position
    InvalidPosition,
    /// Shell not initialized
    ShellNotInitialized,
    /// Animation in progress
    AnimationInProgress,
}

impl std::fmt::Display for RadialShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RadialShellError::MenuNotFound => write!(f, "Menu not found"),
            RadialShellError::ItemNotFound(id) => write!(f, "Item not found: {}", id),
            RadialShellError::ApplicationNotFound(id) => write!(f, "Application not found: {}", id),
            RadialShellError::InvalidPosition => write!(f, "Invalid position"),
            RadialShellError::ShellNotInitialized => write!(f, "Shell not initialized"),
            RadialShellError::AnimationInProgress => write!(f, "Animation in progress"),
        }
    }
}

impl std::error::Error for RadialShellError {}

impl RadialShell {
    /// Create a new radial shell
    pub fn new() -> Self {
        Self::with_config(RadialConfig::default())
    }
    
    /// Create with configuration
    pub fn with_config(config: RadialConfig) -> Self {
        let mut shell = Self {
            config,
            state: ShellState::Hidden,
            radial_menu: RadialMenu {
                items: Vec::new(),
                selected: None,
                submenu: None,
                animation_progress: 0.0,
                center: Point::default(),
                visible: false,
            },
            quick_actions: Vec::new(),
            active_app: None,
            notifications: NotificationCenter {
                notifications: VecDeque::with_capacity(20),
                visible: false,
                max_notifications: 20,
                do_not_disturb: false,
            },
            recent_items: VecDeque::with_capacity(10),
            initialized: false,
        };
        
        shell.init_default_menu();
        shell.init_quick_actions();
        shell.initialized = true;
        
        shell
    }
    
    /// Initialize default menu items
    fn init_default_menu(&mut self) {
        let items = vec![
            MenuItem {
                id: "apps".to_string(),
                name: "Applications".to_string(),
                icon: "apps".to_string(),
                action: MenuAction::OpenSubmenu,
                submenu: self.create_apps_submenu(),
                enabled: true,
                badge: None,
                angle: 0.0,
            },
            MenuItem {
                id: "files".to_string(),
                name: "Files".to_string(),
                icon: "folder".to_string(),
                action: MenuAction::System(SystemAction::FileBrowser),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: std::f32::consts::PI / 4.0,
            },
            MenuItem {
                id: "terminal".to_string(),
                name: "Terminal".to_string(),
                icon: "terminal".to_string(),
                action: MenuAction::System(SystemAction::Terminal),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: std::f32::consts::PI / 2.0,
            },
            MenuItem {
                id: "settings".to_string(),
                name: "Settings".to_string(),
                icon: "settings".to_string(),
                action: MenuAction::System(SystemAction::Settings),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: 3.0 * std::f32::consts::PI / 4.0,
            },
            MenuItem {
                id: "recent".to_string(),
                name: "Recent".to_string(),
                icon: "history".to_string(),
                action: MenuAction::OpenSubmenu,
                submenu: Vec::new(), // Populated dynamically
                enabled: true,
                badge: None,
                angle: std::f32::consts::PI,
            },
            MenuItem {
                id: "notifications".to_string(),
                name: "Notifications".to_string(),
                icon: "bell".to_string(),
                action: MenuAction::Custom("notifications".to_string()),
                submenu: Vec::new(),
                enabled: true,
                badge: Some(3), // Example badge
                angle: 5.0 * std::f32::consts::PI / 4.0,
            },
            MenuItem {
                id: "search".to_string(),
                name: "Search".to_string(),
                icon: "search".to_string(),
                action: MenuAction::Custom("search".to_string()),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: 3.0 * std::f32::consts::PI / 2.0,
            },
            MenuItem {
                id: "power".to_string(),
                name: "Power".to_string(),
                icon: "power".to_string(),
                action: MenuAction::OpenSubmenu,
                submenu: self.create_power_submenu(),
                enabled: true,
                badge: None,
                angle: 7.0 * std::f32::consts::PI / 4.0,
            },
        ];
        
        self.radial_menu.items = items;
    }
    
    /// Create applications submenu
    fn create_apps_submenu(&self) -> Vec<MenuItem> {
        vec![
            MenuItem {
                id: "browser".to_string(),
                name: "Browser".to_string(),
                icon: "browser".to_string(),
                action: MenuAction::LaunchApp("browser".to_string()),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: 0.0,
            },
            MenuItem {
                id: "editor".to_string(),
                name: "Text Editor".to_string(),
                icon: "edit".to_string(),
                action: MenuAction::LaunchApp("text-editor".to_string()),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: std::f32::consts::PI / 2.0,
            },
            MenuItem {
                id: "files".to_string(),
                name: "Files".to_string(),
                icon: "folder".to_string(),
                action: MenuAction::LaunchApp("file-manager".to_string()),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: std::f32::consts::PI,
            },
            MenuItem {
                id: "monitor".to_string(),
                name: "System Monitor".to_string(),
                icon: "activity".to_string(),
                action: MenuAction::LaunchApp("system-monitor".to_string()),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: 3.0 * std::f32::consts::PI / 2.0,
            },
        ]
    }
    
    /// Create power submenu
    fn create_power_submenu(&self) -> Vec<MenuItem> {
        vec![
            MenuItem {
                id: "lock".to_string(),
                name: "Lock".to_string(),
                icon: "lock".to_string(),
                action: MenuAction::System(SystemAction::Lock),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: 0.0,
            },
            MenuItem {
                id: "sleep".to_string(),
                name: "Sleep".to_string(),
                icon: "moon".to_string(),
                action: MenuAction::System(SystemAction::Sleep),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: std::f32::consts::PI / 2.0,
            },
            MenuItem {
                id: "restart".to_string(),
                name: "Restart".to_string(),
                icon: "refresh".to_string(),
                action: MenuAction::System(SystemAction::Restart),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: std::f32::consts::PI,
            },
            MenuItem {
                id: "shutdown".to_string(),
                name: "Shut Down".to_string(),
                icon: "power".to_string(),
                action: MenuAction::System(SystemAction::PowerOff),
                submenu: Vec::new(),
                enabled: true,
                badge: None,
                angle: 3.0 * std::f32::consts::PI / 2.0,
            },
        ]
    }
    
    /// Initialize quick actions
    fn init_quick_actions(&mut self) {
        self.quick_actions = vec![
            QuickAction {
                id: "wifi".to_string(),
                name: "Wi-Fi".to_string(),
                icon: "wifi".to_string(),
                action: MenuAction::Custom("toggle-wifi".to_string()),
                position: QuickActionPosition::TopLeft,
            },
            QuickAction {
                id: "bluetooth".to_string(),
                name: "Bluetooth".to_string(),
                icon: "bluetooth".to_string(),
                action: MenuAction::Custom("toggle-bluetooth".to_string()),
                position: QuickActionPosition::TopRight,
            },
            QuickAction {
                id: "volume".to_string(),
                name: "Volume".to_string(),
                icon: "volume".to_string(),
                action: MenuAction::Custom("volume-control".to_string()),
                position: QuickActionPosition::BottomLeft,
            },
            QuickAction {
                id: "brightness".to_string(),
                name: "Brightness".to_string(),
                icon: "sun".to_string(),
                action: MenuAction::Custom("brightness-control".to_string()),
                position: QuickActionPosition::BottomRight,
            },
        ];
    }
    
    /// Show radial menu at position
    pub fn show_menu(&mut self, x: f32, y: f32) -> Result<(), RadialShellError> {
        if !self.initialized {
            return Err(RadialShellError::ShellNotInitialized);
        }
        
        self.radial_menu.center = Point { x, y };
        self.radial_menu.visible = true;
        self.radial_menu.animation_progress = 0.0;
        self.state = ShellState::Animating;
        
        Ok(())
    }
    
    /// Hide radial menu
    pub fn hide_menu(&mut self) {
        self.radial_menu.visible = false;
        self.radial_menu.selected = None;
        self.radial_menu.submenu = None;
        
        if self.active_app.is_some() {
            self.state = ShellState::ApplicationRunning;
        } else {
            self.state = ShellState::Hidden;
        }
    }
    
    /// Handle pointer move
    pub fn handle_pointer_move(&mut self, x: f32, y: f32) -> Option<usize> {
        if !self.radial_menu.visible {
            return None;
        }
        
        // Calculate angle from center
        let dx = x - self.radial_menu.center.x;
        let dy = y - self.radial_menu.center.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // Check if within menu radius
        if distance < self.config.inner_radius {
            // Inside center circle - close menu
            self.radial_menu.selected = None;
            return None;
        }
        
        if distance > self.config.menu_radius {
            // Outside menu - no selection
            self.radial_menu.selected = None;
            return None;
        }
        
        // Calculate angle and find closest item
        let angle = dy.atan2(dx);
        let normalized_angle = if angle < 0.0 { angle + 2.0 * std::f32::consts::PI } else { angle };
        
        // Find closest item by angle
        let mut closest_idx = None;
        let mut closest_diff = std::f32::MAX;
        
        for (idx, item) in self.radial_menu.items.iter().enumerate() {
            let diff = (item.angle - normalized_angle).abs();
            let diff = diff.min(2.0 * std::f32::consts::PI - diff);
            
            if diff < closest_diff {
                closest_diff = diff;
                closest_idx = Some(idx);
            }
        }
        
        self.radial_menu.selected = closest_idx;
        closest_idx
    }
    
    /// Handle pointer click
    pub fn handle_click(&mut self, x: f32, y: f32) -> Result<MenuAction, RadialShellError> {
        if !self.radial_menu.visible {
            return Err(RadialShellError::MenuNotFound);
        }
        
        if let Some(idx) = self.radial_menu.selected {
            if let Some(item) = self.radial_menu.items.get(idx).cloned() {
                // Check if it has a submenu
                if !item.submenu.is_empty() && item.action == MenuAction::OpenSubmenu {
                    self.radial_menu.submenu = Some(Box::new(RadialMenu {
                        items: item.submenu,
                        selected: None,
                        submenu: None,
                        animation_progress: 0.0,
                        center: self.radial_menu.center,
                        visible: true,
                    }));
                }
                
                return Ok(item.action);
            }
        }
        
        Err(RadialShellError::ItemNotFound("none selected".to_string()))
    }
    
    /// Handle gesture
    pub fn handle_gesture(&mut self, gesture: Gesture) -> Option<MenuAction> {
        match gesture {
            Gesture::Swipe(direction) => {
                if self.config.gestures.swipe_enabled {
                    match direction {
                        SwipeDirection::Up => {
                            // Show notification center
                            self.notifications.visible = true;
                            self.state = ShellState::NotificationCenter;
                        }
                        SwipeDirection::Down => {
                            // Hide notification center
                            self.notifications.visible = false;
                            self.state = ShellState::Hidden;
                        }
                        SwipeDirection::Left => {
                            // Previous workspace
                        }
                        SwipeDirection::Right => {
                            // Next workspace
                        }
                    }
                }
                None
            }
            Gesture::Pinch(scale) => {
                if self.config.gestures.pinch_enabled {
                    if scale < 1.0 - self.config.gestures.pinch_threshold {
                        // Pinch in - close menu
                        self.hide_menu();
                    } else if scale > 1.0 + self.config.gestures.pinch_threshold {
                        // Pinch out - show menu
                    }
                }
                None
            }
            Gesture::LongPress(point) => {
                // Show menu at position
                let _ = self.show_menu(point.x, point.y);
                None
            }
            Gesture::Tap(point) => {
                // Handle tap on quick action
                for action in &self.quick_actions {
                    // Check if tap is on quick action area
                    // (simplified - would need actual position calculation)
                }
                None
            }
        }
    }
    
    /// Add notification
    pub fn add_notification(&mut self, title: &str, message: &str, priority: NotificationPriority) -> String {
        let id = format!("notif-{}", self.notifications.notifications.len());
        
        let notification = Notification {
            id: id.clone(),
            title: title.to_string(),
            message: message.to_string(),
            icon: None,
            timestamp: Instant::now(),
            priority,
            read: false,
            action: None,
        };
        
        self.notifications.notifications.push_back(notification);
        
        // Trim if exceeded max
        while self.notifications.notifications.len() > self.notifications.max_notifications {
            self.notifications.notifications.pop_front();
        }
        
        id
    }
    
    /// Clear notification
    pub fn clear_notification(&mut self, id: &str) -> bool {
        if let Some(pos) = self.notifications.notifications.iter().position(|n| n.id == id) {
            self.notifications.notifications.remove(pos);
            true
        } else {
            false
        }
    }
    
    /// Clear all notifications
    pub fn clear_all_notifications(&mut self) {
        self.notifications.notifications.clear();
    }
    
    /// Add recent item
    pub fn add_recent_item(&mut self, id: &str, name: &str, icon: &str, item_type: RecentItemType) {
        let item = RecentItem {
            id: id.to_string(),
            name: name.to_string(),
            icon: icon.to_string(),
            last_accessed: Instant::now(),
            item_type,
        };
        
        // Remove if already exists
        self.recent_items.retain(|i| i.id != id);
        
        // Add to front
        self.recent_items.push_front(item);
        
        // Trim to max 10
        while self.recent_items.len() > 10 {
            self.recent_items.pop_back();
        }
    }
    
    /// Get recent items
    pub fn recent_items(&self) -> &[RecentItem] {
        &self.recent_items
    }
    
    /// Update animation
    pub fn update_animation(&mut self, delta_ms: u64) {
        if self.state == ShellState::Animating {
            let progress = self.radial_menu.animation_progress + 
                (delta_ms as f32 / self.config.animation_duration as f32);
            
            if progress >= 1.0 {
                self.radial_menu.animation_progress = 1.0;
                self.state = ShellState::MenuShowing;
            } else {
                self.radial_menu.animation_progress = progress;
            }
        }
    }
    
    /// Get state
    pub fn state(&self) -> ShellState {
        self.state
    }
    
    /// Get radial menu
    pub fn menu(&self) -> &RadialMenu {
        &self.radial_menu
    }
    
    /// Get quick actions
    pub fn quick_actions(&self) -> &[QuickAction] {
        &self.quick_actions
    }
    
    /// Get notifications
    pub fn notifications(&self) -> &NotificationCenter {
        &self.notifications
    }
    
    /// Get configuration
    pub fn config(&self) -> &RadialConfig {
        &self.config
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: RadialConfig) {
        self.config = config;
    }
}

impl Default for RadialShell {
    fn default() -> Self {
        Self::new()
    }
}

/// Gesture types
#[derive(Clone, Debug)]
pub enum Gesture {
    Swipe(SwipeDirection),
    Pinch(f32),
    LongPress(Point),
    Tap(Point),
}

/// Swipe direction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}