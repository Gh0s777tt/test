//! Window Manager Module
//! Manages windows, dialogs, and window decorations

use super::*;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::format;

/// Window style flags
#[derive(Debug, Clone, Copy)]
pub struct WindowStyle {
    /// Has title bar
    pub has_title_bar: bool,
    /// Has minimize button
    pub has_minimize: bool,
    /// Has maximize button
    pub has_maximize: bool,
    /// Has close button
    pub has_close: bool,
    /// Has system menu
    pub has_sys_menu: bool,
    /// Is resizable
    pub resizable: bool,
    /// Has thick frame
    pub thick_frame: bool,
    /// Is always on top
    pub top_most: bool,
    /// Has dialog frame
    pub dialog_frame: bool,
    /// Is tool window
    pub tool_window: bool,
}

impl Default for WindowStyle {
    fn default() -> Self {
        Self {
            has_title_bar: true,
            has_minimize: true,
            has_maximize: true,
            has_close: true,
            has_sys_menu: true,
            resizable: true,
            thick_frame: true,
            top_most: false,
            dialog_frame: false,
            tool_window: false,
        }
    }
}

impl WindowStyle {
    /// Standard window
    pub fn standard() -> Self {
        Self::default()
    }
    
    /// Dialog window
    pub fn dialog() -> Self {
        Self {
            has_title_bar: true,
            has_minimize: false,
            has_maximize: false,
            has_close: true,
            has_sys_menu: false,
            resizable: false,
            thick_frame: false,
            top_most: false,
            dialog_frame: true,
            tool_window: false,
        }
    }
    
    /// Tool window
    pub fn tool() -> Self {
        Self {
            has_title_bar: true,
            has_minimize: false,
            has_maximize: false,
            has_close: true,
            has_sys_menu: false,
            resizable: false,
            thick_frame: false,
            top_most: true,
            dialog_frame: false,
            tool_window: true,
        }
    }
    
    /// Borderless window
    pub fn borderless() -> Self {
        Self {
            has_title_bar: false,
            has_minimize: false,
            has_maximize: false,
            has_close: false,
            has_sys_menu: false,
            resizable: false,
            thick_frame: false,
            top_most: false,
            dialog_frame: false,
            tool_window: false,
        }
    }
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    FullScreen,
}

/// Window hit test result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitTest {
    Client,
    TitleBar,
    Minimize,
    Maximize,
    Close,
    LeftBorder,
    RightBorder,
    TopBorder,
    BottomBorder,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
}

/// Window
#[derive(Debug, Clone)]
pub struct Window {
    /// Window ID
    pub id: ElementId,
    /// Window title
    pub title: String,
    /// Window bounds
    pub rect: Rect,
    /// Normal bounds (before maximize)
    pub normal_rect: Rect,
    /// Window style
    pub style: WindowStyle,
    /// Window state
    pub state: WindowState,
    /// Is visible
    pub visible: bool,
    /// Is enabled
    pub enabled: bool,
    /// Is active
    pub active: bool,
    /// Z-order
    pub z_order: u32,
    /// Icon
    pub icon: Icon,
    /// Opacity (0-255)
    pub opacity: u8,
    /// Window data (for custom content)
    pub tag: u32,
}

impl Window {
    /// Create new window
    pub fn new(id: ElementId, title: String, rect: Rect) -> Self {
        Self {
            id,
            title,
            rect,
            normal_rect: rect,
            style: WindowStyle::default(),
            state: WindowState::Normal,
            visible: true,
            enabled: true,
            active: false,
            z_order: 0,
            icon: Icon::File,
            opacity: 255,
            tag: 0,
        }
    }
    
    /// Get client area
    pub fn client_rect(&self) -> Rect {
        let title_height = if self.style.has_title_bar { 32 } else { 0 };
        let border = if self.style.thick_frame { 1 } else { 0 };
        
        Rect::new(
            self.rect.x + border as i32,
            self.rect.y + title_height as i32,
            self.rect.width - 2 * border,
            self.rect.height - title_height - border,
        )
    }
    
    /// Hit test at position
    pub fn hit_test(&self, pos: Position) -> HitTest {
        if !self.rect.contains(pos) {
            return HitTest::None;
        }
        
        let border_size = 5;
        let title_height = 32;
        let button_size = 32;
        let button_y = self.rect.y;
        
        // Check title bar buttons
        if self.style.has_title_bar && pos.y < self.rect.y + title_height {
            let button_x = self.rect.x + self.rect.width as i32 - button_size;
            
            // Close button
            if self.style.has_close && pos.x >= button_x {
                return HitTest::Close;
            }
            
            // Maximize button
            if self.style.has_maximize && pos.x >= button_x - button_size {
                return HitTest::Maximize;
            }
            
            // Minimize button
            if self.style.has_minimize && pos.x >= button_x - 2 * button_size {
                return HitTest::Minimize;
            }
            
            // Title bar drag area
            return HitTest::TitleBar;
        }
        
        // Check borders for resizing
        if self.style.resizable && self.state == WindowState::Normal {
            let in_left = pos.x < self.rect.x + border_size;
            let in_right = pos.x >= self.rect.x + self.rect.width as i32 - border_size;
            let in_top = pos.y < self.rect.y + border_size;
            let in_bottom = pos.y >= self.rect.y + self.rect.height as i32 - border_size;
            
            if in_top && in_left { return HitTest::TopLeft; }
            if in_top && in_right { return HitTest::TopRight; }
            if in_bottom && in_left { return HitTest::BottomLeft; }
            if in_bottom && in_right { return HitTest::BottomRight; }
            if in_left { return HitTest::LeftBorder; }
            if in_right { return HitTest::RightBorder; }
            if in_top { return HitTest::TopBorder; }
            if in_bottom { return HitTest::BottomBorder; }
        }
        
        HitTest::Client
    }
    
    /// Minimize window
    pub fn minimize(&mut self) {
        self.state = WindowState::Minimized;
        self.visible = false;
    }
    
    /// Maximize window
    pub fn maximize(&mut self, screen_rect: Rect) {
        if self.state == WindowState::Maximized {
            // Restore
            self.rect = self.normal_rect;
            self.state = WindowState::Normal;
        } else {
            // Maximize
            self.normal_rect = self.rect;
            self.rect = screen_rect;
            self.state = WindowState::Maximized;
        }
    }
    
    /// Restore window
    pub fn restore(&mut self) {
        if self.state == WindowState::Minimized {
            self.visible = true;
        }
        self.rect = self.normal_rect;
        self.state = WindowState::Normal;
    }
}

/// Window manager
pub struct WindowManager {
    /// All windows
    windows: Vec<Window>,
    /// Active window ID
    active_window: Option<ElementId>,
    /// Dragging window
    dragging: Option<ElementId>,
    /// Drag offset
    drag_offset: Position,
    /// Resizing window
    resizing: Option<ElementId>,
    /// Resize edge
    resize_edge: HitTest,
    /// Next window ID
    next_id: ElementId,
    /// Desktop bounds
    desktop_rect: Rect,
}

impl WindowManager {
    /// Create new window manager
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_window: None,
            dragging: None,
            drag_offset: Position::default(),
            resizing: None,
            resize_edge: HitTest::None,
            next_id: 1,
            desktop_rect: Rect::new(0, 0, 1024, 768),
        }
    }
    
    /// Set desktop bounds
    pub fn set_desktop_rect(&mut self, rect: Rect) {
        self.desktop_rect = rect;
    }
    
    /// Create window
    pub fn create_window(&mut self, title: &str, rect: Rect, style: WindowStyle) -> ElementId {
        let id = self.next_id;
        self.next_id += 1;
        
        let mut window = Window::new(id, String::from(title), rect);
        window.style = style;
        window.z_order = self.windows.len() as u32;
        
        self.windows.push(window);
        self.set_active(id);
        
        id
    }
    
    /// Destroy window
    pub fn destroy_window(&mut self, id: ElementId) {
        self.windows.retain(|w| w.id != id);
        
        if self.active_window == Some(id) {
            self.active_window = self.windows.last().map(|w| w.id);
        }
    }
    
    /// Get window by ID
    pub fn get_window(&self, id: ElementId) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == id)
    }
    
    /// Get mutable window by ID
    pub fn get_window_mut(&mut self, id: ElementId) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }
    
    /// Set active window
    pub fn set_active(&mut self, id: ElementId) {
        // Deactivate previous
        if let Some(active) = self.active_window {
            if let Some(window) = self.get_window_mut(active) {
                window.active = false;
            }
        }
        
        // Get z-order and max before mutating
        let (z, max_z) = if let Some(window) = self.get_window(id) {
            (window.z_order, self.windows.len() - 1)
        } else {
            return;
        };
        
        // Activate new
        if let Some(window) = self.get_window_mut(id) {
            window.active = true;
        }
        
        // Update z-orders
        for w in &mut self.windows {
            if w.z_order > z {
                w.z_order -= 1;
            }
        }
        
        if let Some(window) = self.get_window_mut(id) {
            window.z_order = max_z as u32;
        }
        
        self.active_window = Some(id);
    }
    
    /// Get window at position (top-most first)
    pub fn get_window_at(&self, pos: Position) -> Option<ElementId> {
        let mut sorted: Vec<&Window> = self.windows.iter()
            .filter(|w| w.visible && w.state != WindowState::Minimized)
            .collect();
        sorted.sort_by(|a, b| b.z_order.cmp(&a.z_order));
        
        for window in sorted {
            if window.rect.contains(pos) {
                return Some(window.id);
            }
        }
        None
    }
    
    /// Handle event
    pub fn handle_event(&mut self, event: &UiEvent) -> WindowAction {
        match event {
            UiEvent::MouseDown(pos, button) => {
                if *button == MouseButton::Left {
                    // Find window under cursor
                    if let Some(id) = self.get_window_at(*pos) {
                        if let Some(window) = self.get_window(id) {
                            let hit = window.hit_test(*pos);
                            
                            match hit {
                                HitTest::Close => {
                                    return WindowAction::Close(id);
                                }
                                HitTest::Minimize => {
                                    if let Some(w) = self.get_window_mut(id) {
                                        w.minimize();
                                    }
                                    return WindowAction::Minimize(id);
                                }
                                HitTest::Maximize => {
                                    let desktop_rect = self.desktop_rect;
                                    if let Some(w) = self.get_window_mut(id) {
                                        w.maximize(desktop_rect);
                                    }
                                    return WindowAction::Maximize(id);
                                }
                                HitTest::TitleBar => {
                                    // Extract values from window before modifying self
                                    let offset_x = pos.x - window.rect.x;
                                    let offset_y = pos.y - window.rect.y;
                                    self.dragging = Some(id);
                                    self.drag_offset = Position {
                                        x: offset_x,
                                        y: offset_y,
                                    };
                                    self.set_active(id);
                                }
                                HitTest::Client => {
                                    self.set_active(id);
                                }
                                _ => {
                                    if hit != HitTest::None {
                                        self.resizing = Some(id);
                                        self.resize_edge = hit;
                                        self.set_active(id);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            UiEvent::MouseUp(_, _) => {
                self.dragging = None;
                self.resizing = None;
            }
            
            UiEvent::MouseMove(pos) => {
                if let Some(id) = self.dragging {
                    // Extract drag_offset before mutable borrow
                    let offset_x = self.drag_offset.x;
                    let offset_y = self.drag_offset.y;
                    if let Some(window) = self.get_window_mut(id) {
                        if window.state == WindowState::Normal {
                            window.rect.x = pos.x - offset_x;
                            window.rect.y = pos.y - offset_y;
                        }
                    }
                }
                
                if let Some(id) = self.resizing {
                    // Extract resize_edge before mutable borrow
                    let resize_edge = self.resize_edge;
                    if let Some(window) = self.get_window_mut(id) {
                        let min_width = 200;
                        let min_height = 100;
                        
                        match resize_edge {
                            HitTest::LeftBorder | HitTest::TopLeft | HitTest::BottomLeft => {
                                let new_x = pos.x;
                                let new_width = window.rect.x + window.rect.width as i32 - new_x;
                                if new_width >= min_width as i32 {
                                    window.rect.x = new_x;
                                    window.rect.width = new_width as u32;
                                }
                            }
                            HitTest::RightBorder | HitTest::TopRight | HitTest::BottomRight => {
                                let new_width = pos.x - window.rect.x;
                                if new_width >= min_width as i32 {
                                    window.rect.width = new_width as u32;
                                }
                            }
                            _ => {}
                        }
                        
                        match resize_edge {
                            HitTest::TopBorder | HitTest::TopLeft | HitTest::TopRight => {
                                let new_y = pos.y;
                                let new_height = window.rect.y + window.rect.height as i32 - new_y;
                                if new_height >= min_height as i32 {
                                    window.rect.y = new_y;
                                    window.rect.height = new_height as u32;
                                }
                            }
                            HitTest::BottomBorder | HitTest::BottomLeft | HitTest::BottomRight => {
                                let new_height = pos.y - window.rect.y;
                                if new_height >= min_height as i32 {
                                    window.rect.height = new_height as u32;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            
            _ => {}
        }
        
        WindowAction::None
    }
    
    /// Render all windows
    pub fn render(&self, surface: &mut dyn Surface) {
        // Sort windows by z-order
        let mut sorted: Vec<&Window> = self.windows.iter()
            .filter(|w| w.visible && w.state != WindowState::Minimized)
            .collect();
        sorted.sort_by_key(|w| w.z_order);
        
        for window in sorted {
            self.render_window(window, surface);
        }
    }
    
    /// Render single window
    fn render_window(&self, window: &Window, surface: &mut dyn Surface) {
        let rect = window.rect;
        let title_height = if window.style.has_title_bar { 32 } else { 0 };
        
        // Window shadow
        surface.fill_rounded_rect(
            Rect::new(rect.x + 4, rect.y + 4, rect.width, rect.height),
            8,
            Color { r: 0, g: 0, b: 0, a: 40 },
        );
        
        // Window background
        surface.fill_rounded_rect(
            Rect::new(rect.x, rect.y, rect.width, rect.height),
            8,
            Color::WINDOW_BG,
        );
        
        // Title bar
        if window.style.has_title_bar {
            let title_color = if window.active {
                Color::WINDOW_TITLE
            } else {
                Color { r: 100, g: 100, b: 100, a: 255 }
            };
            
            // Title bar background (rounded top corners only)
            surface.fill_rect(
                Rect::new(rect.x + 1, rect.y + 1, rect.width - 2, title_height - 2),
                title_color,
            );
            
            // Icon
            surface.draw_icon(rect.x as u32 + 8, rect.y as u32 + 6, window.icon, 20);
            
            // Title text
            surface.draw_text(
                rect.x as u32 + 32,
                rect.y as u32 + 8,
                &window.title,
                Color::WHITE,
            );
            
            // Window buttons
            let btn_size = 32u32;
            let btn_x = rect.x + rect.width as i32 - btn_size as i32;
            let btn_y = rect.y;
            
            // Close button
            if window.style.has_close {
                let close_color = if window.active {
                    Color { r: 232, g: 17, b: 35, a: 255 }
                } else {
                    Color { r: 150, g: 150, b: 150, a: 255 }
                };
                surface.fill_rect(
                    Rect::new(btn_x, btn_y, btn_size, btn_size),
                    close_color,
                );
                surface.draw_text(btn_x as u32 + 10, btn_y as u32 + 8, "X", Color::WHITE);
            }
            
            // Maximize button
            if window.style.has_maximize {
                let max_x = btn_x - btn_size as i32;
                surface.fill_rect(
                    Rect::new(max_x, btn_y, btn_size, btn_size),
                    if window.active { Color { r: 0, g: 90, b: 158, a: 255 } } else { Color { r: 100, g: 100, b: 100, a: 255 } },
                );
                surface.draw_text(max_x as u32 + 10, btn_y as u32 + 8, "□", Color::WHITE);
            }
            
            // Minimize button
            if window.style.has_minimize {
                let min_x = btn_x - 2 * btn_size as i32;
                surface.fill_rect(
                    Rect::new(min_x, btn_y, btn_size, btn_size),
                    if window.active { Color { r: 0, g: 90, b: 158, a: 255 } } else { Color { r: 100, g: 100, b: 100, a: 255 } },
                );
                surface.draw_text(min_x as u32 + 10, btn_y as u32 + 8, "—", Color::WHITE);
            }
        }
        
        // Window border
        let border_color = if window.active {
            Color { r: 0, g: 120, b: 215, a: 255 }
        } else {
            Color { r: 150, g: 150, b: 150, a: 255 }
        };
        surface.draw_rounded_rect(rect, 8, border_color);
    }
    
    /// Get all windows
    pub fn get_windows(&self) -> &[Window] {
        &self.windows
    }
    
    /// Get active window
    pub fn get_active_window(&self) -> Option<&Window> {
        self.active_window.and_then(|id| self.get_window(id))
    }
}

/// Window action
#[derive(Debug, Clone)]
pub enum WindowAction {
    None,
    Close(ElementId),
    Minimize(ElementId),
    Maximize(ElementId),
    Restore(ElementId),
    Move(ElementId, Position),
    Resize(ElementId, Size),
    Activate(ElementId),
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}