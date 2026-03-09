//! Classic Shell Implementation
//! 
//! Traditional desktop environment with taskbar, start menu, and window management
//! Similar to Windows/macOS desktop paradigm

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Classic Shell main structure
pub struct ClassicShell {
    pub taskbar: Arc<Mutex<Taskbar>>,
    pub start_menu: Arc<Mutex<StartMenu>>,
    pub window_manager: Arc<Mutex<WindowManager>>,
    pub desktop_icons: Arc<Mutex<DesktopIcons>>,
    pub notifications: Arc<Mutex<NotificationSystem>>,
    pub workspaces: Arc<Mutex<WorkspaceManager>>,
    pub system_tray: Arc<Mutex<SystemTray>>,
    pub clock: Arc<Mutex<Clock>>,
    pub state: Arc<Mutex<ShellState>>,
}

#[derive(Debug, Clone)]
pub struct Taskbar {
    pub position: TaskbarPosition,
    pub height: u32,
    pub auto_hide: bool,
    pub pinned_apps: Vec<PinnedApp>,
    pub running_apps: Vec<RunningApp>,
    pub show_desktop: bool,
    pub theme: TaskbarTheme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskbarPosition { Bottom, Top, Left, Right }

#[derive(Debug, Clone)]
pub struct PinnedApp {
    pub app_id: String,
    pub name: String,
    pub icon_path: String,
    pub command: String,
    pub position: usize,
}

#[derive(Debug, Clone)]
pub struct RunningApp {
    pub app_id: String,
    pub name: String,
    pub icon_path: String,
    pub window_count: usize,
    pub is_pinned: bool,
    pub active_window_id: Option<u64>,
    pub position: usize,
}

#[derive(Debug, Clone)]
pub struct TaskbarTheme {
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub accent_color: (u8, u8, u8, u8),
    pub translucency: f32,
    pub rounded_corners: bool,
    pub blur_effect: bool,
}

#[derive(Debug, Clone)]
pub struct StartMenu {
    pub is_open: bool,
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub pinned_apps: Vec<StartMenuApp>,
    pub all_apps: Vec<StartMenuApp>,
    pub recent_apps: Vec<StartMenuApp>,
    pub power_options: PowerOptions,
    pub settings_shortcut: bool,
    pub user_profile: UserProfile,
    pub theme: StartMenuTheme,
}

#[derive(Debug, Clone)]
pub struct StartMenuApp {
    pub app_id: String,
    pub name: String,
    pub description: String,
    pub icon_path: String,
    pub category: AppCategory,
    pub command: String,
    pub launch_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCategory {
    System, Utilities, Development, Office, Multimedia, Internet, Games, Other,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub result_type: SearchResultType,
    pub title: String,
    pub description: String,
    pub icon_path: String,
    pub action: String,
    pub relevance: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchResultType { Application, File, Setting, Web }

#[derive(Debug, Clone)]
pub struct PowerOptions {
    pub sleep: bool,
    pub restart: bool,
    pub shutdown: bool,
    pub hibernate: bool,
    pub lock: bool,
    pub sign_out: bool,
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub name: String,
    pub avatar_path: String,
    pub email: Option<String>,
    pub show_profile_button: bool,
}

#[derive(Debug, Clone)]
pub struct StartMenuTheme {
    pub background_color: (u8, u8, u8, u8),
    pub text_color: (u8, u8, u8, u8),
    pub accent_color: (u8, u8, u8, u8),
    pub translucency: f32,
    pub rounded_corners: bool,
    pub blur_effect: bool,
    pub show_recent: bool,
    pub show_pinned: bool,
}

#[derive(Debug, Clone)]
pub struct WindowManager {
    pub windows: HashMap<u64, Window>,
    pub next_window_id: u64,
    pub active_window_id: Option<u64>,
    pub focused_window_id: Option<u64>,
    pub stacking_order: VecDeque<u64>,
    pub config: WindowManagerConfig,
    pub theme: WindowTheme,
}

#[derive(Debug, Clone)]
pub struct Window {
    pub window_id: u64,
    pub title: String,
    pub app_id: String,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    pub state: WindowState,
    pub window_type: WindowType,
    pub resizable: bool,
    pub movable: bool,
    pub closable: bool,
    pub minimizable: bool,
    pub maximizable: bool,
    pub always_on_top: bool,
    pub is_modal: bool,
    pub parent_window_id: Option<u64>,
    pub decorations: WindowDecorations,
    pub opacity: f32,
    pub created_at: Instant,
    pub last_focused: Option<Instant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState { Normal, Minimized, Maximized, Fullscreen }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowType { Normal, Dialog, Splash, Utility, Toolbar, Menu, Popup, Tooltip }

#[derive(Debug, Clone)]
pub struct WindowDecorations {
    pub show_title_bar: bool,
    pub show_minimize: bool,
    pub show_maximize: bool,
    pub show_close: bool,
    pub show_resize_handles: bool,
    pub custom_decorations: bool,
}

#[derive(Debug, Clone)]
pub struct WindowManagerConfig {
    pub focus_follows_mouse: bool,
    pub auto_raise: bool,
    pub click_to_focus: bool,
    pub double_click_maximize: bool,
    pub snap_to_edges: bool,
    pub snap_distance: u32,
    pub show_all_windows: bool,
    pub group_windows: bool,
}

#[derive(Debug, Clone)]
pub struct WindowTheme {
    pub title_bar_color: (u8, u8, u8, u8),
    pub title_text_color: (u8, u8, u8, u8),
    pub border_color: (u8, u8, u8, u8),
    pub border_width: u32,
    pub active_border_color: (u8, u8, u8, u8),
    pub shadow_enabled: bool,
    pub rounded_corners: bool,
}

#[derive(Debug, Clone)]
pub struct DesktopIcons {
    pub icons: Vec<DesktopIcon>,
    pub grid: IconGrid,
    pub auto_arrange: bool,
    pub sort_order: SortOrder,
    pub show_icons: bool,
    pub icon_size: IconSize,
}

#[derive(Debug, Clone)]
pub struct DesktopIcon {
    pub icon_id: u64,
    pub name: String,
    pub icon_type: IconType,
    pub icon_path: String,
    pub target_path: Option<String>,
    pub command: Option<String>,
    pub position: (i32, i32),
    pub is_selected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconType { Application, File, Folder, Shortcut, Drive, Trash }

#[derive(Debug, Clone)]
pub struct IconGrid {
    pub cell_size: (u32, u32),
    pub padding: (u32, u32),
    pub start_position: (i32, i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder { Name, Type, Date, Size }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconSize { Small, Medium, Large, Extra }

#[derive(Debug, Clone)]
pub struct NotificationSystem {
    pub notifications: Vec<Notification>,
    pub history: Vec<Notification>,
    pub next_id: u64,
    pub do_not_disturb: bool,
    pub position: NotificationPosition,
    pub max_visible: usize,
    pub default_timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub notification_id: u64,
    pub app_name: String,
    pub app_icon_path: String,
    pub title: String,
    pub body: String,
    pub urgency: NotificationUrgency,
    pub actions: Vec<NotificationAction>,
    pub progress: Option<f32>,
    pub created_at: Instant,
    pub expires_at: Option<Instant>,
    pub is_persistent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency { Low, Normal, Critical }

#[derive(Debug, Clone)]
pub struct NotificationAction {
    pub action_id: String,
    pub label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationPosition { TopRight, TopLeft, BottomRight, BottomLeft }

#[derive(Debug, Clone)]
pub struct WorkspaceManager {
    pub workspaces: Vec<Workspace>,
    pub current_workspace: usize,
    pub num_workspaces: usize,
    pub workspace_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub index: usize,
    pub name: String,
    pub windows: Vec<u64>,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct SystemTray {
    pub icons: Vec<TrayIcon>,
    pub show_clock: bool,
    pub show_system_icons: bool,
}

#[derive(Debug, Clone)]
pub struct TrayIcon {
    pub icon_id: u64,
    pub app_name: String,
    pub icon_path: String,
    pub tooltip: String,
    pub click_action: Option<String>,
    pub context_menu: Vec<ContextMenuItem>,
}

#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    pub item_id: String,
    pub label: String,
    pub is_separator: bool,
    pub enabled: bool,
    pub action: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Clock {
    pub show_date: bool,
    pub show_seconds: bool,
    pub use_24_hour: bool,
    pub format: String,
}

#[derive(Debug, Clone)]
pub struct ShellState {
    pub is_running: bool,
    pub is_initialized: bool,
    pub start_time: Instant,
}
