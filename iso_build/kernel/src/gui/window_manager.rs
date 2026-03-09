//! Window Manager
//! Complete window management for VantisOS GUI

use super::*;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::boxed::Box;

/// Window state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

/// Resize edge
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeEdge {
    None,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Window
pub struct Window {
    pub id: ElementId,
    pub title: String,
    pub bounds: Rect,
    pub normal_bounds: Rect,
    pub state: WindowState,
    pub visible: bool,
    pub active: bool,
    pub resizable: bool,
    pub minimizable: bool,
    pub maximizable: bool,
    pub closable: bool,
    pub content: Option<Box<dyn Widget>>,
    pub dragging: bool,
    pub drag_start: Position,
    pub resizing: bool,
    pub resize_edge: ResizeEdge,
    pub resize_start: Rect,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            id: 0,
            title: String::from(title),
            bounds: Rect::new(100, 100, width, height),
            normal_bounds: Rect::new(100, 100, width, height),
            state: WindowState::Normal,
            visible: true,
            active: false,
            resizable: true,
            minimizable: true,
            maximizable: true,
            closable: true,
            content: None,
            dragging: false,
            drag_start: Position::default(),
            resizing: false,
            resize_edge: ResizeEdge::None,
            resize_start: Rect::new(0, 0, 0, 0),
        }
    }
    
    pub fn with_position(mut self, x: i32, y: i32) -> Self {
        self.bounds.x = x;
        self.bounds.y = y;
        self.normal_bounds.x = x;
        self.normal_bounds.y = y;
        self
    }
    
    pub fn with_content(mut self, content: Box<dyn Widget>) -> Self {
        self.content = Some(content);
        self
    }
    
    pub fn set_content(&mut self, content: Box<dyn Widget>) {
        self.content = Some(content);
    }
    
    pub fn title_bar_rect(&self) -> Rect {
        Rect::new(self.bounds.x, self.bounds.y, self.bounds.width, 32)
    }
    
    pub fn content_rect(&self) -> Rect {
        Rect::new(
            self.bounds.x,
            self.bounds.y + 32,
            self.bounds.width,
            self.bounds.height - 32,
        )
    }
    
    pub fn close_button_rect(&self) -> Rect {
        Rect::new(
            self.bounds.x + self.bounds.width as i32 - 40,
            self.bounds.y + 4,
            32,
            24,
        )
    }
    
    pub fn minimize_button_rect(&self) -> Rect {
        Rect::new(
            self.bounds.x + self.bounds.width as i32 - 80,
            self.bounds.y + 4,
            32,
            24,
        )
    }
    
    pub fn maximize_button_rect(&self) -> Rect {
        Rect::new(
            self.bounds.x + self.bounds.width as i32 - 120,
            self.bounds.y + 4,
            32,
            24,
        )
    }
    
    pub fn get_resize_edge(&self, pos: Position) -> ResizeEdge {
        if self.state != WindowState::Normal || !self.resizable {
            return ResizeEdge::None;
        }
        
        let edge_size = 8;
        let in_left = pos.x >= self.bounds.x && pos.x < self.bounds.x + edge_size;
        let in_right = pos.x <= self.bounds.right() && pos.x > self.bounds.right() - edge_size;
        let in_top = pos.y >= self.bounds.y && pos.y < self.bounds.y + edge_size;
        let in_bottom = pos.y <= self.bounds.bottom() && pos.y > self.bounds.bottom() - edge_size;
        
        match (in_left, in_right, in_top, in_bottom) {
            (true, _, true, _) => ResizeEdge::TopLeft,
            (_, true, true, _) => ResizeEdge::TopRight,
            (true, _, _, true) => ResizeEdge::BottomLeft,
            (_, true, _, true) => ResizeEdge::BottomRight,
            (true, _, _, _) => ResizeEdge::Left,
            (_, true, _, _) => ResizeEdge::Right,
            (_, _, true, _) => ResizeEdge::Top,
            (_, _, _, true) => ResizeEdge::Bottom,
            _ => ResizeEdge::None,
        }
    }
    
    pub fn maximize(&mut self, screen_width: u32, screen_height: u32) {
        if self.state == WindowState::Normal {
            self.normal_bounds = self.bounds;
            self.bounds = Rect::new(0, 0, screen_width, screen_height - 48); // Leave room for taskbar
            self.state = WindowState::Maximized;
        } else if self.state == WindowState::Maximized {
            self.bounds = self.normal_bounds;
            self.state = WindowState::Normal;
        }
    }
    
    pub fn minimize(&mut self) {
        self.state = WindowState::Minimized;
        self.visible = false;
    }
    
    pub fn restore(&mut self) {
        self.state = WindowState::Normal;
        self.visible = true;
    }
}

/// Window Manager
pub struct WindowManager {
    windows: Vec<Window>,
    next_id: ElementId,
    active_window: Option<ElementId>,
    screen_width: u32,
    screen_height: u32,
}

impl WindowManager {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            windows: Vec::new(),
            next_id: 1,
            active_window: None,
            screen_width,
            screen_height,
        }
    }
    
    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> ElementId {
        let id = self.next_id;
        self.next_id += 1;
        
        let mut window = Window::new(title, width, height);
        window.id = id;
        
        // Center window
        window.bounds.x = (self.screen_width as i32 - width as i32) / 2;
        window.bounds.y = (self.screen_height as i32 - height as i32) / 2;
        window.normal_bounds = window.bounds;
        
        self.windows.push(window);
        self.set_active(id);
        
        id
    }
    
    pub fn get_window(&self, id: ElementId) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == id)
    }
    
    pub fn get_window_mut(&mut self, id: ElementId) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }
    
    pub fn set_active(&mut self, id: ElementId) {
        // Deactivate all windows
        for window in &mut self.windows {
            window.active = false;
        }
        
        // Activate the specified window
        if let Some(window) = self.get_window_mut(id) {
            window.active = true;
            window.visible = true;
            if window.state == WindowState::Minimized {
                window.state = WindowState::Normal;
            }
            self.active_window = Some(id);
        }
    }
    
    pub fn active_window(&self) -> Option<ElementId> {
        self.active_window
    }
    
    pub fn close_window(&mut self, id: ElementId) {
        self.windows.retain(|w| w.id != id);
        if self.active_window == Some(id) {
            self.active_window = self.windows.last().map(|w| w.id);
            if let Some(active_id) = self.active_window {
                if let Some(window) = self.get_window_mut(active_id) {
                    window.active = true;
                }
            }
        }
    }
    
    pub fn bring_to_front(&mut self, id: ElementId) {
        let pos = self.windows.iter().position(|w| w.id == id);
        if let Some(pos) = pos {
            let window = self.windows.remove(pos);
            self.windows.push(window);
        }
    }
    
    pub fn window_at(&self, pos: Position) -> Option<ElementId> {
        // Iterate in reverse order (top to bottom)
        for window in self.windows.iter().rev() {
            if window.visible && window.bounds.contains(pos) {
                return Some(window.id);
            }
        }
        None
    }
    
    pub fn handle_event(&mut self, event: &UiEvent) -> bool {
        match event {
            UiEvent::MouseDown(pos, MouseButton::Left) => {
                // Check if clicking on a window
                if let Some(window_id) = self.window_at(*pos) {
                    self.bring_to_front(window_id);
                    self.set_active(window_id);
                    
                    // Get screen dimensions first
                    let (sw, sh) = (self.screen_width, self.screen_height);
                    
                    let window = self.get_window_mut(window_id).unwrap();
                    
                    // Check close button
                    if window.closable && window.close_button_rect().contains(*pos) {
                        let id = window.id;
                        self.close_window(id);
                        return true;
                    }
                    
                    // Check minimize button
                    if window.minimizable && window.minimize_button_rect().contains(*pos) {
                        window.minimize();
                        return true;
                    }
                    
                    // Check maximize button
                    if window.maximizable && window.maximize_button_rect().contains(*pos) {
                        window.maximize(sw, sh);
                        return true;
                    }
                    
                    // Check title bar for dragging
                    if window.title_bar_rect().contains(*pos) && window.state == WindowState::Normal {
                        window.dragging = true;
                        window.drag_start = Position::new(pos.x - window.bounds.x, pos.y - window.bounds.y);
                        return true;
                    }
                    
                    // Check for resize
                    let edge = window.get_resize_edge(*pos);
                    if edge != ResizeEdge::None {
                        window.resizing = true;
                        window.resize_edge = edge;
                        window.resize_start = window.bounds;
                        return true;
                    }
                    
                    // Forward to content
                    if let Some(content) = &mut window.content {
                        return content.handle_event(event);
                    }
                    
                    return true;
                }
                false
            }
            
            UiEvent::MouseUp(_, MouseButton::Left) => {
                for window in &mut self.windows {
                    window.dragging = false;
                    window.resizing = false;
                    window.resize_edge = ResizeEdge::None;
                }
                false
            }
            
            UiEvent::MouseMove(pos) => {
                for window in &mut self.windows {
                    if window.dragging {
                        window.bounds.x = pos.x - window.drag_start.x;
                        window.bounds.y = pos.y - window.drag_start.y;
                        window.normal_bounds = window.bounds;
                        return true;
                    }
                    
                    if window.resizing {
                        let dx = pos.x - window.resize_start.x;
                        let dy = pos.y - window.resize_start.y;
                        
                        let mut new_bounds = window.resize_start;
                        
                        match window.resize_edge {
                            ResizeEdge::Left => {
                                new_bounds.x = window.resize_start.x + dx;
                                new_bounds.width = (window.resize_start.width as i32 - dx) as u32;
                            }
                            ResizeEdge::Right => {
                                new_bounds.width = (window.resize_start.width as i32 + dx) as u32;
                            }
                            ResizeEdge::Top => {
                                new_bounds.y = window.resize_start.y + dy;
                                new_bounds.height = (window.resize_start.height as i32 - dy) as u32;
                            }
                            ResizeEdge::Bottom => {
                                new_bounds.height = (window.resize_start.height as i32 + dy) as u32;
                            }
                            ResizeEdge::TopLeft => {
                                new_bounds.x = window.resize_start.x + dx;
                                new_bounds.y = window.resize_start.y + dy;
                                new_bounds.width = (window.resize_start.width as i32 - dx) as u32;
                                new_bounds.height = (window.resize_start.height as i32 - dy) as u32;
                            }
                            ResizeEdge::TopRight => {
                                new_bounds.y = window.resize_start.y + dy;
                                new_bounds.width = (window.resize_start.width as i32 + dx) as u32;
                                new_bounds.height = (window.resize_start.height as i32 - dy) as u32;
                            }
                            ResizeEdge::BottomLeft => {
                                new_bounds.x = window.resize_start.x + dx;
                                new_bounds.width = (window.resize_start.width as i32 - dx) as u32;
                                new_bounds.height = (window.resize_start.height as i32 + dy) as u32;
                            }
                            ResizeEdge::BottomRight => {
                                new_bounds.width = (window.resize_start.width as i32 + dx) as u32;
                                new_bounds.height = (window.resize_start.height as i32 + dy) as u32;
                            }
                            ResizeEdge::None => {}
                        }
                        
                        // Enforce minimum size
                        if new_bounds.width >= 200 && new_bounds.height >= 100 {
                            window.bounds = new_bounds;
                            window.normal_bounds = new_bounds;
                        }
                        
                        return true;
                    }
                }
                false
            }
            
            _ => {
                // Forward to active window content
                if let Some(active_id) = self.active_window {
                    if let Some(window) = self.get_window_mut(active_id) {
                        if let Some(content) = &mut window.content {
                            return content.handle_event(event);
                        }
                    }
                }
                false
            }
        }
    }
    
    pub fn render(&mut self, surface: &mut dyn Surface) {
        for window in &mut self.windows {
            if !window.visible {
                continue;
            }
            
            // Window background
            surface.fill_rect(window.bounds, Color::WINDOW_BG);
            
            // Title bar
            let title_bar_color = if window.active {
                Color::TITLE_BAR_ACTIVE
            } else {
                Color::TITLE_BAR
            };
            surface.fill_rect(window.title_bar_rect(), title_bar_color);
            
            // Title text
            surface.draw_text_sized(
                window.bounds.x as u32 + 12,
                window.bounds.y as u32 + 8,
                &window.title,
                14,
                Color::WHITE,
            );
            
            // Window border
            surface.draw_rect(window.bounds, Color::GRAY, 1);
            
            // Window buttons
            if window.closable {
                let close_rect = window.close_button_rect();
                surface.fill_rounded_rect(close_rect, 4, Color::RED);
                surface.draw_text_sized(
                    close_rect.x as u32 + 10,
                    close_rect.y as u32 + 4,
                    "X",
                    14,
                    Color::WHITE,
                );
            }
            
            if window.minimizable {
                let min_rect = window.minimize_button_rect();
                surface.fill_rounded_rect(min_rect, 4, Color::GRAY);
                surface.draw_text_sized(
                    min_rect.x as u32 + 10,
                    min_rect.y as u32 + 4,
                    "_",
                    14,
                    Color::WHITE,
                );
            }
            
            if window.maximizable {
                let max_rect = window.maximize_button_rect();
                surface.fill_rounded_rect(max_rect, 4, Color::GRAY);
                surface.draw_text_sized(
                    max_rect.x as u32 + 8,
                    max_rect.y as u32 + 4,
                    "[]",
                    14,
                    Color::WHITE,
                );
            }
            
            // Content
            if let Some(content) = &mut window.content {
                content.render(surface);
            }
        }
    }
    
    pub fn windows(&self) -> &[Window] {
        &self.windows
    }
    
    pub fn windows_mut(&mut self) -> &mut [Window] {
        &mut self.windows
    }
}