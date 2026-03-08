//! Shell Module for VantisOS
//! Windows-like desktop environment with:
//! - Desktop with icons
//! - Taskbar with start menu
//! - Context menus (right-click)
//! - File explorer
//! - Drive management

pub mod desktop;
pub mod taskbar;
pub mod menu;
pub mod explorer;
pub mod window;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;
use spin::Mutex;

/// Screen resolution
#[derive(Debug, Clone, Copy)]
pub struct ScreenResolution {
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
}

impl Default for ScreenResolution {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            bpp: 32,
        }
    }
}

/// Position on screen
#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// Size
#[derive(Debug, Clone, Copy, Default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

/// Rectangle
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, pos: Position) -> bool {
        pos.x >= self.x && pos.x < self.x + self.width as i32 &&
        pos.y >= self.y && pos.y < self.y + self.height as i32
    }
}

/// Color (RGBA)
#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    
    // Windows-like colors
    pub const TASKBAR_BG: Color = Color { r: 32, g: 32, b: 32, a: 255 };
    pub const WINDOW_BG: Color = Color { r: 240, g: 240, b: 240, a: 255 };
    pub const WINDOW_TITLE: Color = Color { r: 0, g: 120, b: 215, a: 255 };
    pub const DESKTOP_BG: Color = Color { r: 0, g: 78, b: 152, a: 255 };
    pub const START_BUTTON: Color = Color { r: 0, g: 120, b: 215, a: 255 };
    pub const HOVER: Color = Color { r: 229, g: 243, b: 255, a: 255 };
    pub const SELECTED: Color = Color { r: 204, g: 232, b: 255, a: 255 };
    
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn to_u32(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

/// Mouse button
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    None,
}

/// Mouse state
#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    pub position: Position,
    pub left_button: bool,
    pub right_button: bool,
    pub middle_button: bool,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            position: Position::default(),
            left_button: false,
            right_button: false,
            middle_button: false,
        }
    }
}

/// Keyboard event
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: char,
    pub scancode: u8,
    pub pressed: bool,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

/// UI Event
#[derive(Debug, Clone)]
pub enum UiEvent {
    /// Mouse moved
    MouseMove(Position),
    /// Mouse button pressed
    MouseDown(Position, MouseButton),
    /// Mouse button released
    MouseUp(Position, MouseButton),
    /// Mouse clicked
    Click(Position, MouseButton),
    /// Double click
    DoubleClick(Position, MouseButton),
    /// Right click (context menu)
    RightClick(Position),
    /// Key pressed
    KeyDown(KeyEvent),
    /// Key released
    KeyUp(KeyEvent),
    /// Mouse wheel
    MouseWheel(i32),
    /// Window resize
    Resize(u32, u32),
    /// Timer tick
    Tick,
    /// Custom event
    Custom(u32),
}

/// UI element ID
pub type ElementId = u64;

/// UI element trait
pub trait UiElement: Send + Sync {
    /// Get element ID
    fn id(&self) -> ElementId;
    
    /// Get bounds
    fn bounds(&self) -> Rect;
    
    /// Set bounds
    fn set_bounds(&mut self, rect: Rect);
    
    /// Check if point is inside
    fn hit_test(&self, pos: Position) -> bool {
        self.bounds().contains(pos)
    }
    
    /// Handle event
    fn handle_event(&mut self, event: &UiEvent) -> bool;
    
    /// Render
    fn render(&self, surface: &mut dyn Surface);
    
    /// Is visible
    fn is_visible(&self) -> bool;
    
    /// Set visible
    fn set_visible(&mut self, visible: bool);
    
    /// Is enabled
    fn is_enabled(&self) -> bool;
    
    /// Set enabled
    fn set_enabled(&mut self, enabled: bool);
}

/// Drawing surface trait
pub trait Surface {
    /// Get dimensions
    fn dimensions(&self) -> (u32, u32);
    
    /// Clear with color
    fn clear(&mut self, color: Color);
    
    /// Set pixel
    fn set_pixel(&mut self, x: u32, y: u32, color: Color);
    
    /// Draw line
    fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: Color);
    
    /// Draw rectangle
    fn draw_rect(&mut self, rect: Rect, color: Color);
    
    /// Fill rectangle
    fn fill_rect(&mut self, rect: Rect, color: Color);
    
    /// Draw rounded rectangle
    fn draw_rounded_rect(&mut self, rect: Rect, radius: u32, color: Color);
    
    /// Fill rounded rectangle
    fn fill_rounded_rect(&mut self, rect: Rect, radius: u32, color: Color);
    
    /// Draw text
    fn draw_text(&mut self, x: u32, y: u32, text: &str, color: Color);
    
    /// Draw text with font size
    fn draw_text_sized(&mut self, x: u32, y: u32, text: &str, size: u8, color: Color);
    
    /// Draw image
    fn draw_image(&mut self, x: u32, y: u32, image: &[u8], width: u32, height: u32);
    
    /// Draw icon
    fn draw_icon(&mut self, x: u32, y: u32, icon: Icon, size: u32);
}

/// Icon types
#[derive(Debug, Clone, Copy)]
pub enum Icon {
    /// File icon
    File,
    /// Folder icon
    Folder,
    /// Drive icon
    Drive,
    /// CD/DVD icon
    Optical,
    /// USB drive
    Usb,
    /// Network
    Network,
    /// Computer
    Computer,
    /// User
    User,
    /// Settings
    Settings,
    /// Search
    Search,
    /// Power
    Power,
    /// Close (X)
    Close,
    /// Minimize (-)
    Minimize,
    /// Maximize (□)
    Maximize,
    /// Restore
    Restore,
    /// Application
    App(u32),
    /// Custom (index)
    Custom(u32),
}

/// Global shell state
pub static SHELL_STATE: Mutex<ShellState> = Mutex::new(ShellState {
    initialized: false,
    resolution: ScreenResolution { width: 1024, height: 768, bpp: 32 },
    mouse: MouseState {
        position: Position { x: 0, y: 0 },
        left_button: false,
        right_button: false,
        middle_button: false,
    },
    focused_window: 0,
    active_menu: None,
});

/// Shell state
pub struct ShellState {
    pub initialized: bool,
    pub resolution: ScreenResolution,
    pub mouse: MouseState,
    pub focused_window: ElementId,
    pub active_menu: Option<ElementId>,
}

/// Initialize shell
pub fn init() {
    let mut state = SHELL_STATE.lock();
    state.initialized = true;
}

/// Set resolution
pub fn set_resolution(width: u32, height: u32) {
    let mut state = SHELL_STATE.lock();
    state.resolution.width = width;
    state.resolution.height = height;
}

/// Get mouse position
pub fn get_mouse_position() -> Position {
    SHELL_STATE.lock().mouse.position
}

/// Set mouse position
pub fn set_mouse_position(x: i32, y: i32) {
    let mut state = SHELL_STATE.lock();
    state.mouse.position.x = x;
    state.mouse.position.y = y;
}