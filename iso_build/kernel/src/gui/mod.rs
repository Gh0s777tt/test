//! GUI Subsystem
//! Complete graphical user interface for VantisOS

pub mod widgets;
pub mod theme;
pub mod window_manager;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::RefCell;

pub use widgets::*;
pub use theme::*;
pub use window_manager::*;

/// GUI Manager - Main entry point for GUI operations
pub struct GuiManager {
    /// Screen surface
    screen: Option<Box<dyn Surface>>,
    /// Root widget
    root: Option<Rc<RefCell<dyn Widget>>>,
    /// Theme
    theme: Theme,
    /// Cursor position
    cursor_pos: Position,
    /// Cursor visible
    cursor_visible: bool,
    /// Focused widget
    focused: Option<ElementId>,
    /// Mouse capture
    capture: Option<ElementId>,
    /// Animation frame
    frame: u64,
}

impl GuiManager {
    /// Create new GUI manager
    pub fn new() -> Self {
        Self {
            screen: None,
            root: None,
            theme: Theme::default(),
            cursor_pos: Position::default(),
            cursor_visible: true,
            focused: None,
            capture: None,
            frame: 0,
        }
    }
    
    /// Initialize with screen surface
    pub fn init(&mut self, surface: Box<dyn Surface>) {
        self.screen = Some(surface);
    }
    
    /// Set root widget
    pub fn set_root(&mut self, widget: Rc<RefCell<dyn Widget>>) {
        self.root = Some(widget);
    }
    
    /// Process event
    pub fn process_event(&mut self, event: &UiEvent) -> bool {
        if let Some(root) = &self.root {
            root.borrow_mut().handle_event(event)
        } else {
            false
        }
    }
    
    /// Render frame
    pub fn render(&mut self) {
        if let (Some(screen), Some(root)) = (&mut self.screen, &self.root) {
            // Clear screen
            screen.fill_rect(
                Rect::new(0, 0, screen.width(), screen.height()),
                self.theme.background,
            );
            
            // Render widget tree
            root.borrow_mut().render(screen.as_mut());
            
            // Render cursor
            if self.cursor_visible {
                screen.draw_cursor(self.cursor_pos.x, self.cursor_pos.y);
            }
        }
        
        self.frame += 1;
    }
    
    /// Get current frame
    pub fn frame(&self) -> u64 {
        self.frame
    }
}

impl Default for GuiManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Position
#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Rectangle
#[derive(Debug, Clone, Copy)]
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
        pos.x >= self.x 
            && pos.x < self.x + self.width as i32
            && pos.y >= self.y 
            && pos.y < self.y + self.height as i32
    }
    
    pub fn right(&self) -> i32 {
        self.x + self.width as i32
    }
    
    pub fn bottom(&self) -> i32 {
        self.y + self.height as i32
    }
}

/// Color (RGBA)
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 220, g: 20, b: 60, a: 255 };
    pub const GREEN: Color = Color { r: 34, g: 139, b: 34, a: 255 };
    pub const BLUE: Color = Color { r: 30, g: 144, b: 255, a: 255 };
    pub const GRAY: Color = Color { r: 128, g: 128, b: 128, a: 255 };
    pub const LIGHT_GRAY: Color = Color { r: 200, g: 200, b: 200, a: 255 };
    pub const DARK_GRAY: Color = Color { r: 64, g: 64, b: 64, a: 255 };
    pub const DESKTOP_BG: Color = Color { r: 0, g: 120, b: 212, a: 255 };
    pub const WINDOW_BG: Color = Color { r: 240, g: 240, b: 240, a: 255 };
    pub const TITLE_BAR: Color = Color { r: 32, g: 32, b: 32, a: 255 };
    pub const TITLE_BAR_ACTIVE: Color = Color { r: 0, g: 120, b: 212, a: 255 };
    pub const BUTTON: Color = Color { r: 225, g: 225, b: 225, a: 255 };
    pub const BUTTON_HOVER: Color = Color { r: 229, g: 243, b: 255, a: 255 };
    pub const BUTTON_PRESSED: Color = Color { r: 204, g: 228, b: 247, a: 255 };
    pub const SELECTED: Color = Color { r: 0, g: 120, b: 212, a: 120 };
    pub const ACCENT: Color = Color { r: 0, g: 120, b: 212, a: 255 };
    
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    pub fn with_alpha(mut self, a: u8) -> Self {
        self.a = a;
        self
    }
    
    pub fn to_rgb(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    
    pub fn to_rgba(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
}

/// Surface trait for drawing operations
pub trait Surface {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
    
    fn fill_rect(&mut self, rect: Rect, color: Color);
    fn fill_rounded_rect(&mut self, rect: Rect, radius: u32, color: Color);
    fn draw_rect(&mut self, rect: Rect, color: Color, thickness: u32);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color, thickness: u32);
    
    fn draw_text(&mut self, x: u32, y: u32, text: &str, color: Color);
    fn draw_text_sized(&mut self, x: u32, y: u32, text: &str, size: u32, color: Color);
    fn measure_text(&self, text: &str, size: u32) -> (u32, u32);
    
    fn draw_icon(&mut self, x: u32, y: u32, icon: Icon, size: u32);
    fn draw_cursor(&mut self, x: i32, y: i32);
    
    fn draw_image(&mut self, x: u32, y: u32, data: &[u8], width: u32, height: u32);
    fn copy_region(&mut self, src: Rect, dst: Position);
    
    fn clear(&mut self, color: Color) {
        self.fill_rect(Rect::new(0, 0, self.width(), self.height()), color);
    }
}

/// Widget trait for all GUI elements
pub trait Widget {
    fn id(&self) -> ElementId;
    fn bounds(&self) -> Rect;
    fn set_bounds(&mut self, rect: Rect);
    
    fn handle_event(&mut self, event: &UiEvent) -> bool;
    fn render(&mut self, surface: &mut dyn Surface);
    
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    
    fn can_focus(&self) -> bool;
    fn on_focus(&mut self);
    fn on_blur(&mut self);
}

/// Element ID type
pub type ElementId = u64;

/// Icon types
#[derive(Debug, Clone, Copy)]
pub enum Icon {
    None,
    Folder,
    File,
    Application,
    Settings,
    Computer,
    Network,
    User,
    Trash,
    Search,
    Close,
    Minimize,
    Maximize,
    Restore,
    Menu,
    Back,
    Forward,
    Up,
    Down,
    Left,
    Right,
    Play,
    Pause,
    Stop,
    Volume,
    Mute,
    Wifi,
    Battery,
    Clock,
    Calendar,
    Notification,
    Lock,
    Unlock,
    Power,
    Restart,
    Custom(u32),
}

/// Mouse button
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

/// UI Events
#[derive(Debug, Clone)]
pub enum UiEvent {
    // Mouse events
    MouseDown(Position, MouseButton),
    MouseUp(Position, MouseButton),
    MouseMove(Position),
    MouseWheel(i32),
    Click(Position, MouseButton),
    DoubleClick(Position, MouseButton),
    RightClick(Position),
    
    // Keyboard events
    KeyDown(u8),
    KeyUp(u8),
    Char(char),
    
    // Focus events
    Focus(ElementId),
    Blur(ElementId),
    
    // System events
    Resize(u32, u32),
    Close(ElementId),
    Minimize(ElementId),
    Maximize(ElementId),
    
    // Custom events
    Custom(u32, u64),
}