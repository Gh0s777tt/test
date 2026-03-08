//! Taskbar Module
//! Windows-like taskbar with:
//! - Start button
//! - Pinned applications
//! - Running applications
//! - System tray
//! - Clock

use super::*;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

/// Taskbar position
#[derive(Debug, Clone, Copy)]
pub enum TaskbarPosition {
    Bottom,
    Top,
    Left,
    Right,
}

impl Default for TaskbarPosition {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Taskbar button
#[derive(Debug, Clone)]
pub struct TaskbarButton {
    /// Button ID
    pub id: ElementId,
    /// Application name
    pub name: String,
    /// Icon
    pub icon: Icon,
    /// Is pinned
    pub pinned: bool,
    /// Is running
    pub running: bool,
    /// Window ID if running
    pub window_id: Option<ElementId>,
    /// Is active
    pub active: bool,
    /// Tooltip
    pub tooltip: String,
}

/// System tray icon
#[derive(Debug, Clone)]
pub struct TrayIcon {
    /// Icon ID
    pub id: ElementId,
    /// Icon
    pub icon: Icon,
    /// Tooltip
    pub tooltip: String,
    /// Callback ID
    pub callback_id: u32,
}

/// Taskbar configuration
#[derive(Debug, Clone)]
pub struct TaskbarConfig {
    /// Height in pixels
    pub height: u32,
    /// Position
    pub position: TaskbarPosition,
    /// Auto-hide
    pub auto_hide: bool,
    /// Show clock
    pub show_clock: bool,
    /// Show date
    pub show_date: bool,
    /// Show desktop button
    pub show_desktop_button: bool,
    /// Combine taskbar buttons
    pub combine_buttons: bool,
    /// Use small icons
    pub small_icons: bool,
}

impl Default for TaskbarConfig {
    fn default() -> Self {
        Self {
            height: 40,
            position: TaskbarPosition::Bottom,
            auto_hide: false,
            show_clock: true,
            show_date: true,
            show_desktop_button: true,
            combine_buttons: true,
            small_icons: false,
        }
    }
}

/// Taskbar manager
pub struct TaskbarManager {
    /// Configuration
    config: TaskbarConfig,
    /// Pinned and running buttons
    buttons: Vec<TaskbarButton>,
    /// System tray icons
    tray_icons: Vec<TrayIcon>,
    /// Start menu visible
    start_menu_visible: bool,
    /// Search visible
    search_visible: bool,
    /// Next ID
    next_id: ElementId,
    /// Current time (HH:MM)
    current_time: String,
    /// Current date
    current_date: String,
    /// Hovered button
    hovered_button: Option<ElementId>,
}

impl TaskbarManager {
    /// Create new taskbar manager
    pub fn new() -> Self {
        let mut taskbar = Self {
            config: TaskbarConfig::default(),
            buttons: Vec::new(),
            tray_icons: Vec::new(),
            start_menu_visible: false,
            search_visible: false,
            next_id: 1,
            current_time: String::from("00:00"),
            current_date: String::from("Jan 1"),
            hovered_button: None,
        };
        
        taskbar.add_default_buttons();
        taskbar.add_default_tray_icons();
        taskbar
    }
    
    /// Add default taskbar buttons
    fn add_default_buttons(&mut self) {
        // Start button (ID 0 is reserved)
        
        // Pinned apps
        let id1 = self.next_id();
        self.add_button(TaskbarButton {
            id: id1,
            name: String::from("File Explorer"),
            icon: Icon::Folder,
            pinned: true,
            running: false,
            window_id: None,
            active: false,
            tooltip: String::from("File Explorer"),
        });
        
        let id2 = self.next_id();
        self.add_button(TaskbarButton {
            id: id2,
            name: String::from("Terminal"),
            icon: Icon::Custom(1),
            pinned: true,
            running: false,
            window_id: None,
            active: false,
            tooltip: String::from("Terminal"),
        });
        
        let id3 = self.next_id();
        self.add_button(TaskbarButton {
            id: id3,
            name: String::from("Settings"),
            icon: Icon::Settings,
            pinned: true,
            running: false,
            window_id: None,
            active: false,
            tooltip: String::from("Settings"),
        });
    }
    
    /// Add default tray icons
    fn add_default_tray_icons(&mut self) {
        let id1 = self.next_id();
        self.add_tray_icon(TrayIcon {
            id: id1,
            icon: Icon::Network,
            tooltip: String::from("Network: Connected"),
            callback_id: 1,
        });
        
        let id2 = self.next_id();
        self.add_tray_icon(TrayIcon {
            id: id2,
            icon: Icon::Usb,
            tooltip: String::from("Safely Remove Hardware"),
            callback_id: 2,
        });
        
        let id3 = self.next_id();
        self.add_tray_icon(TrayIcon {
            id: id3,
            icon: Icon::Power,
            tooltip: String::from("Power options"),
            callback_id: 3,
        });
    }
    
    /// Get next ID
    fn next_id(&mut self) -> ElementId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    /// Add button
    pub fn add_button(&mut self, button: TaskbarButton) {
        self.buttons.push(button);
    }
    
    /// Remove button
    pub fn remove_button(&mut self, id: ElementId) {
        self.buttons.retain(|b| b.id != id || b.pinned);
        // If pinned, just mark as not running
        if let Some(button) = self.buttons.iter_mut().find(|b| b.id == id) {
            button.running = false;
            button.window_id = None;
        }
    }
    
    /// Add tray icon
    pub fn add_tray_icon(&mut self, icon: TrayIcon) {
        self.tray_icons.push(icon);
    }
    
    /// Remove tray icon
    pub fn remove_tray_icon(&mut self, id: ElementId) {
        self.tray_icons.retain(|i| i.id != id);
    }
    
    /// Toggle start menu
    pub fn toggle_start_menu(&mut self) {
        self.start_menu_visible = !self.start_menu_visible;
        if self.start_menu_visible {
            self.search_visible = false;
        }
    }
    
    /// Show start menu
    pub fn show_start_menu(&mut self) {
        self.start_menu_visible = true;
        self.search_visible = false;
    }
    
    /// Hide start menu
    pub fn hide_start_menu(&mut self) {
        self.start_menu_visible = false;
    }
    
    /// Toggle search
    pub fn toggle_search(&mut self) {
        self.search_visible = !self.search_visible;
        if self.search_visible {
            self.start_menu_visible = false;
        }
    }
    
    /// Update time
    pub fn update_time(&mut self, hours: u8, minutes: u8) {
        self.current_time = format!("{:02}:{:02}", hours, minutes);
    }
    
    /// Update date
    pub fn update_date(&mut self, month: &str, day: u8) {
        self.current_date = format!("{} {}", month, day);
    }
    
    /// Set active window
    pub fn set_active_window(&mut self, window_id: Option<ElementId>) {
        for button in &mut self.buttons {
            button.active = button.window_id == window_id;
        }
    }
    
    /// Register running application
    pub fn register_app(&mut self, name: &str, icon: Icon, window_id: ElementId) -> ElementId {
        // Check if already pinned
        if let Some(button) = self.buttons.iter_mut().find(|b| b.name == name && b.pinned) {
            button.running = true;
            button.window_id = Some(window_id);
            return button.id;
        }
        
        // Add new button
        let id = self.next_id();
        self.add_button(TaskbarButton {
            id,
            name: String::from(name),
            icon,
            pinned: false,
            running: true,
            window_id: Some(window_id),
            active: true,
            tooltip: String::from(name),
        });
        id
    }
    
    /// Get button at position
    pub fn get_button_at(&self, pos: Position) -> Option<&TaskbarButton> {
        let button_width = 44i32;
        let start_button_width = 44i32;
        
        let taskbar_y = if let TaskbarPosition::Bottom = self.config.position {
            768 - self.config.height as i32
        } else {
            0
        };
        
        // Check if in taskbar area
        if pos.y < taskbar_y || pos.y >= taskbar_y + self.config.height as i32 {
            return None;
        }
        
        // Check start button
        if pos.x >= 0 && pos.x < start_button_width {
            return None; // Start button handled separately
        }
        
        // Check taskbar buttons
        let mut x = start_button_width;
        for button in &self.buttons {
            if pos.x >= x && pos.x < x + button_width {
                return Some(button);
            }
            x += button_width;
        }
        
        None
    }
    
    /// Handle event
    pub fn handle_event(&mut self, event: &UiEvent, resolution: &ScreenResolution) -> TaskbarAction {
        match event {
            UiEvent::Click(pos, button) => {
                if *button == MouseButton::Left {
                    let taskbar_y = resolution.height as i32 - self.config.height as i32;
                    
                    // Check if click is in taskbar area
                    if pos.y >= taskbar_y {
                        // Start button area
                        if pos.x < 44 {
                            self.toggle_start_menu();
                            return TaskbarAction::ToggleStartMenu;
                        }
                        
                        // Check button clicks
                        if let Some(btn) = self.get_button_at(*pos) {
                            let id = btn.id;
                            if btn.running {
                                // Switch to window
                                return TaskbarAction::ActivateWindow(id);
                            } else {
                                // Launch app
                                return TaskbarAction::LaunchApp(btn.name.clone());
                            }
                        }
                        
                        // System tray area (right side)
                        let tray_start = resolution.width as i32 - 100;
                        if pos.x >= tray_start {
                            return TaskbarAction::ShowTrayMenu;
                        }
                    }
                }
            }
            
            UiEvent::MouseDown(pos, button) => {
                if *button == MouseButton::Right {
                    let taskbar_y = resolution.height as i32 - self.config.height as i32;
                    if pos.y >= taskbar_y {
                        return TaskbarAction::ShowTaskbarMenu;
                    }
                }
            }
            
            _ => {}
        }
        
        TaskbarAction::None
    }
    
    /// Render taskbar
    pub fn render(&self, surface: &mut dyn Surface) {
        let (width, height) = surface.dimensions();
        let taskbar_height = self.config.height;
        let taskbar_y = height - taskbar_height;
        
        // Draw taskbar background
        surface.fill_rect(
            Rect::new(0, taskbar_y as i32, width, taskbar_height),
            Color::TASKBAR_BG,
        );
        
        // Draw top line
        surface.draw_line(
            0, taskbar_y, width, taskbar_y,
            Color { r: 60, g: 60, b: 60, a: 255 },
        );
        
        // Draw start button
        let start_rect = Rect::new(0, taskbar_y as i32, 44, taskbar_height);
        let start_color = if self.start_menu_visible {
            Color { r: 30, g: 30, b: 30, a: 255 }
        } else if self.hovered_button == Some(0) {
            Color::HOVER
        } else {
            Color::TRANSPARENT
        };
        surface.fill_rect(start_rect, start_color);
        
        // Draw start icon (simplified Windows logo)
        surface.draw_icon(12, taskbar_y as u32 + 10, Icon::Computer, 20);
        
        // Draw taskbar buttons
        let mut x = 44u32;
        let button_width = 44u32;
        
        for button in &self.buttons {
            let btn_rect = Rect::new(x as i32, taskbar_y as i32, button_width, taskbar_height);
            
            // Background color
            let bg_color = if button.active {
                Color { r: 50, g: 50, b: 50, a: 255 }
            } else if self.hovered_button == Some(button.id) {
                Color::HOVER
            } else {
                Color::TRANSPARENT
            };
            surface.fill_rect(btn_rect, bg_color);
            
            // Active indicator line
            if button.active || button.running {
                let indicator_y = taskbar_y + taskbar_height - 3;
                surface.fill_rect(
                    Rect::new(x as i32, indicator_y as i32, button_width, 3),
                    Color::START_BUTTON,
                );
            }
            
            // Icon
            let icon_size = if self.config.small_icons { 16 } else { 24 };
            surface.draw_icon(
                x + (button_width - icon_size) / 2,
                taskbar_y as u32 + (taskbar_height - icon_size) / 2,
                button.icon,
                icon_size,
            );
            
            x += button_width;
        }
        
        // Draw system tray area
        let tray_x = width - 100;
        surface.fill_rect(
            Rect::new(tray_x as i32, taskbar_y as i32, 100, taskbar_height),
            Color::TASKBAR_BG,
        );
        
        // Draw clock
        surface.draw_text(
            tray_x + 10,
            taskbar_y as u32 + 8,
            &self.current_time,
            Color::WHITE,
        );
        
        if self.config.show_date {
            surface.draw_text(
                tray_x + 10,
                taskbar_y as u32 + 22,
                &self.current_date,
                Color { r: 180, g: 180, b: 180, a: 255 },
            );
        }
        
        // Show desktop button
        if self.config.show_desktop_button {
            let desktop_btn_x = width - 6;
            surface.fill_rect(
                Rect::new(desktop_btn_x as i32, taskbar_y as i32, 6, taskbar_height),
                Color { r: 60, g: 60, b: 60, a: 255 },
            );
        }
    }
}

/// Taskbar action
#[derive(Debug, Clone)]
pub enum TaskbarAction {
    None,
    ToggleStartMenu,
    ShowStartMenu,
    HideStartMenu,
    LaunchApp(String),
    ActivateWindow(ElementId),
    ShowTrayMenu,
    ShowTaskbarMenu,
    ShowDesktop,
}

impl Default for TaskbarManager {
    fn default() -> Self {
        Self::new()
    }
}