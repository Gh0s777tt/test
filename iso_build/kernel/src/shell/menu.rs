//! Menu Module
//! Start menu and context menus

use super::*;
use super::desktop::ContextMenuItem;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

/// Start menu item
#[derive(Debug, Clone)]
pub struct StartMenuItem {
    /// Item ID
    pub id: ElementId,
    /// Display name
    pub name: String,
    /// Icon
    pub icon: Icon,
    /// Application path
    pub path: String,
    /// Is pinned
    pub pinned: bool,
    /// Recent count
    pub recent_count: u32,
}

/// Start menu group
#[derive(Debug, Clone)]
pub struct StartMenuGroup {
    /// Group name
    pub name: String,
    /// Items in group
    pub items: Vec<StartMenuItem>,
}

/// Start menu configuration
#[derive(Debug, Clone)]
pub struct StartMenuConfig {
    /// Show most used apps
    pub show_most_used: bool,
    /// Show recently added
    pub show_recently_added: bool,
    /// Number of pinned apps to show
    pub pinned_count: usize,
    /// Number of recent apps to show
    pub recent_count: usize,
    /// Show power options
    pub show_power: bool,
    /// Show user profile
    pub show_user: bool,
    /// Full screen start menu
    pub full_screen: bool,
}

impl Default for StartMenuConfig {
    fn default() -> Self {
        Self {
            show_most_used: true,
            show_recently_added: true,
            pinned_count: 12,
            recent_count: 6,
            show_power: true,
            show_user: true,
            full_screen: false,
        }
    }
}

/// Start menu manager
pub struct StartMenuManager {
    /// Configuration
    config: StartMenuConfig,
    /// Pinned applications
    pinned: Vec<StartMenuItem>,
    /// All applications (alphabetical)
    all_apps: Vec<StartMenuItem>,
    /// Recent applications
    recent: Vec<StartMenuItem>,
    /// Power options visible
    power_menu_visible: bool,
    /// Search query
    search_query: String,
    /// Search results
    search_results: Vec<StartMenuItem>,
    /// Currently hovered item
    hovered_item: Option<ElementId>,
    /// Next ID
    next_id: ElementId,
    /// Visible state
    visible: bool,
    /// Position
    position: Position,
    /// Size
    size: Size,
}

impl StartMenuManager {
    /// Create new start menu manager
    pub fn new() -> Self {
        let mut menu = Self {
            config: StartMenuConfig::default(),
            pinned: Vec::new(),
            all_apps: Vec::new(),
            recent: Vec::new(),
            power_menu_visible: false,
            search_query: String::new(),
            search_results: Vec::new(),
            hovered_item: None,
            next_id: 1,
            visible: false,
            position: Position { x: 0, y: 0 },
            size: Size { width: 600, height: 500 },
        };
        
        menu.add_default_items();
        menu
    }
    
    /// Add default menu items
    fn add_default_items(&mut self) {
        // Pinned apps
        let pinned_apps = [
            ("File Explorer", Icon::Folder, "/apps/explorer"),
            ("Settings", Icon::Settings, "/apps/settings"),
            ("Terminal", Icon::Custom(1), "/apps/terminal"),
            ("Browser", Icon::Network, "/apps/browser"),
            ("Text Editor", Icon::File, "/apps/editor"),
            ("Calculator", Icon::Custom(2), "/apps/calculator"),
        ];
        
        for (name, icon, path) in pinned_apps {
            let id = self.next_id();
            self.pinned.push(StartMenuItem {
                id,
                name: String::from(name),
                icon,
                path: String::from(path),
                pinned: true,
                recent_count: 0,
            });
        }
        
        // All apps
        let all_apps = [
            ("Accessories", Icon::Folder, "/apps/category/accessories"),
            ("Games", Icon::Custom(3), "/apps/category/games"),
            ("Graphics", Icon::Custom(4), "/apps/category/graphics"),
            ("Internet", Icon::Network, "/apps/category/internet"),
            ("Multimedia", Icon::Custom(5), "/apps/category/multimedia"),
            ("Office", Icon::File, "/apps/category/office"),
            ("Programming", Icon::Custom(6), "/apps/category/programming"),
            ("System", Icon::Settings, "/apps/category/system"),
            ("Utilities", Icon::Custom(7), "/apps/category/utilities"),
        ];
        
        for (name, icon, path) in all_apps {
            let id = self.next_id();
            self.all_apps.push(StartMenuItem {
                id,
                name: String::from(name),
                icon,
                path: String::from(path),
                pinned: false,
                recent_count: 0,
            });
        }
    }
    
    /// Get next ID
    fn next_id(&mut self) -> ElementId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    /// Show start menu
    pub fn show(&mut self, position: Position) {
        self.visible = true;
        self.position = position;
    }
    
    /// Hide start menu
    pub fn hide(&mut self) {
        self.visible = false;
        self.power_menu_visible = false;
        self.search_query.clear();
        self.search_results.clear();
    }
    
    /// Toggle visibility
    pub fn toggle(&mut self, position: Position) {
        if self.visible {
            self.hide();
        } else {
            self.show(position);
        }
    }
    
    /// Update search
    pub fn update_search(&mut self, query: &str) {
        self.search_query = String::from(query);
        
        if query.is_empty() {
            self.search_results.clear();
            return;
        }
        
        // Search in all apps
        self.search_results = self.all_apps.iter()
            .filter(|app| app.name.to_lowercase().contains(&query.to_lowercase()))
            .cloned()
            .collect();
    }
    
    /// Get item at position
    pub fn get_item_at(&self, pos: Position) -> Option<&StartMenuItem> {
        if !self.visible {
            return None;
        }
        
        // Check pinned items
        let item_height = 36;
        let item_width = 90;
        let start_x = self.position.x + 10;
        let start_y = self.position.y + 60;
        
        for (i, item) in self.pinned.iter().enumerate() {
            let row = i / 6;
            let col = i % 6;
            let x = start_x + (col as i32) * item_width;
            let y = start_y + (row as i32) * (item_height + 10);
            
            if pos.x >= x && pos.x < x + item_width &&
               pos.y >= y && pos.y < y + item_height {
                return Some(item);
            }
        }
        
        None
    }
    
    /// Handle event
    pub fn handle_event(&mut self, event: &UiEvent) -> MenuAction {
        if !self.visible {
            return MenuAction::None;
        }
        
        match event {
            UiEvent::Click(pos, button) => {
                if *button == MouseButton::Left {
                    // Check if click is outside menu
                    let rect = Rect::new(
                        self.position.x,
                        self.position.y,
                        self.size.width,
                        self.size.height,
                    );
                    
                    if !rect.contains(*pos) {
                        self.hide();
                        return MenuAction::Close;
                    }
                    
                    // Check item click
                    if let Some(item) = self.get_item_at(*pos) {
                        return MenuAction::LaunchApp(item.path.clone());
                    }
                    
                    // Check power button
                    let power_y = self.position.y + self.size.height as i32 - 50;
                    if pos.y >= power_y {
                        self.power_menu_visible = !self.power_menu_visible;
                        return MenuAction::TogglePowerMenu;
                    }
                }
            }
            
            UiEvent::KeyDown(key) => {
                if key.key == '\x1b' { // Escape
                    self.hide();
                    return MenuAction::Close;
                }
            }
            
            _ => {}
        }
        
        MenuAction::None
    }
    
    /// Render start menu
    pub fn render(&self, surface: &mut dyn Surface) {
        if !self.visible {
            return;
        }
        
        let x = self.position.x;
        let y = self.position.y;
        let width = self.size.width;
        let height = self.size.height;
        
        // Background with rounded corners
        surface.fill_rounded_rect(
            Rect::new(x, y, width, height),
            8,
            Color { r: 32, g: 32, b: 32, a: 240 },
        );
        
        // Border
        surface.draw_rounded_rect(
            Rect::new(x, y, width, height),
            8,
            Color { r: 60, g: 60, b: 60, a: 255 },
        );
        
        // Search box
        let search_rect = Rect::new(x + 10, y + 10, width - 20, 36);
        surface.fill_rounded_rect(search_rect, 4, Color { r: 50, g: 50, b: 50, a: 255 });
        surface.draw_text(x as u32 + 20, y as u32 + 18, "Type here to search", Color { r: 120, g: 120, b: 120, a: 255 });
        
        // Pinned section header
        surface.draw_text(x as u32 + 10, y as u32 + 55, "Pinned", Color::WHITE);
        
        // Pinned apps
        let item_height = 36u32;
        let item_width = 90u32;
        let start_x = x + 10;
        let start_y = y + 80;
        
        for (i, item) in self.pinned.iter().enumerate() {
            let row = i / 6;
            let col = i % 6;
            let item_x = start_x + (col as i32) * (item_width as i32 + 10);
            let item_y = start_y + (row as i32) * (item_height as i32 + 10);
            
            // Item background
            let hovered = self.hovered_item == Some(item.id);
            if hovered {
                surface.fill_rounded_rect(
                    Rect::new(item_x, item_y, item_width, item_height),
                    4,
                    Color::HOVER,
                );
            }
            
            // Icon
            surface.draw_icon(item_x as u32 + 8, item_y as u32 + 6, item.icon, 24);
            
            // Name
            surface.draw_text(item_x as u32 + 36, item_y as u32 + 12, &item.name, Color::WHITE);
        }
        
        // All apps button
        let all_apps_y = start_y + 100;
        surface.fill_rounded_rect(
            Rect::new(x + 10, all_apps_y, width - 20, 32),
            4,
            Color { r: 45, g: 45, b: 45, a: 255 },
        );
        surface.draw_text(x as u32 + 20, all_apps_y as u32 + 8, "All apps >", Color::WHITE);
        
        // Recommended section
        let rec_y = all_apps_y + 50;
        surface.draw_text(x as u32 + 10, rec_y as u32, "Recommended", Color::WHITE);
        
        // Recent items
        let rec_item_y = rec_y + 25;
        for (i, item) in self.recent.iter().take(4).enumerate() {
            let item_y = rec_item_y + (i as i32) * 40;
            surface.fill_rounded_rect(
                Rect::new(x + 10, item_y, width - 20, 36),
                4,
                Color { r: 45, g: 45, b: 45, a: 255 },
            );
            surface.draw_icon(x as u32 + 20, item_y as u32 + 6, item.icon, 24);
            surface.draw_text(x as u32 + 52, item_y as u32 + 12, &item.name, Color::WHITE);
        }
        
        // User section at bottom
        let user_y = y + height as i32 - 50;
        surface.fill_rect(
            Rect::new(x, user_y, width, 50),
            Color { r: 28, g: 28, b: 28, a: 255 },
        );
        
        // User icon
        surface.draw_icon(x as u32 + 15, user_y as u32 + 10, Icon::User, 30);
        surface.draw_text(x as u32 + 55, user_y as u32 + 18, "User", Color::WHITE);
        
        // Power button
        let power_x = x + width as i32 - 45;
        surface.draw_icon(power_x as u32, user_y as u32 + 10, Icon::Power, 30);
    }
}

/// Menu action
#[derive(Debug, Clone)]
pub enum MenuAction {
    None,
    Close,
    LaunchApp(String),
    TogglePowerMenu,
    Shutdown,
    Reboot,
    Sleep,
    LogOut,
}

/// Context menu
pub struct ContextMenu {
    /// Menu items
    items: Vec<ContextMenuItem>,
    /// Visible
    visible: bool,
    /// Position
    position: Position,
    /// Hovered item index
    hovered_index: Option<usize>,
}

impl ContextMenu {
    /// Create new context menu
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            visible: false,
            position: Position::default(),
            hovered_index: None,
        }
    }
    
    /// Set items
    pub fn set_items(&mut self, items: Vec<ContextMenuItem>) {
        self.items = items;
    }
    
    /// Show at position
    pub fn show(&mut self, pos: Position) {
        self.visible = true;
        self.position = pos;
    }
    
    /// Hide menu
    pub fn hide(&mut self) {
        self.visible = false;
        self.hovered_index = None;
    }
    
    /// Get item at position
    pub fn get_item_at(&self, pos: Position) -> Option<(usize, &ContextMenuItem)> {
        if !self.visible {
            return None;
        }
        
        let item_height = 28u32;
        let menu_width = 200u32;
        
        for (i, _item) in self.items.iter().enumerate() {
            let item_y = self.position.y + (i as i32 * item_height as i32);
            
            if pos.x >= self.position.x && pos.x < self.position.x + menu_width as i32 &&
               pos.y >= item_y && pos.y < item_y + item_height as i32 {
                return Some((i, &self.items[i]));
            }
        }
        
        None
    }
    
    /// Handle event
    pub fn handle_event(&mut self, event: &UiEvent) -> ContextMenuAction {
        if !self.visible {
            return ContextMenuAction::None;
        }
        
        match event {
            UiEvent::Click(pos, button) => {
                if *button == MouseButton::Left {
                    // First, extract the action we need before calling hide()
                    let action = if let Some((_index, item)) = self.get_item_at(*pos) {
                        match &item.action {
                            super::desktop::ContextAction::Command(cmd) => {
                                Some(ContextMenuAction::Command(cmd.clone()))
                            }
                            _ => None
                        }
                    } else {
                        // Click outside menu
                        Some(ContextMenuAction::Close)
                    };

                    if action.is_some() {
                        self.hide();
                        return action.unwrap();
                    }
                }
            }
            
            UiEvent::MouseMove(pos) => {
                self.hovered_index = self.get_item_at(*pos).map(|(i, _)| i);
            }
            
            _ => {}
        }
        
        ContextMenuAction::None
    }
    
    /// Render context menu
    pub fn render(&self, surface: &mut dyn Surface) {
        if !self.visible {
            return;
        }
        
        let item_height = 28u32;
        let menu_width = 200u32;
        let menu_height = (self.items.len() as u32 * item_height) + 4;
        
        // Background
        surface.fill_rounded_rect(
            Rect::new(self.position.x, self.position.y, menu_width, menu_height),
            4,
            Color { r: 45, g: 45, b: 45, a: 255 },
        );
        
        // Border
        surface.draw_rounded_rect(
            Rect::new(self.position.x, self.position.y, menu_width, menu_height),
            4,
            Color { r: 60, g: 60, b: 60, a: 255 },
        );
        
        // Items
        for (i, item) in self.items.iter().enumerate() {
            let item_y = self.position.y + 2 + (i as i32 * item_height as i32);
            
            // Hover background
            if self.hovered_index == Some(i) {
                surface.fill_rect(
                    Rect::new(self.position.x + 2, item_y, menu_width - 4, item_height),
                    Color::HOVER,
                );
            }
            
            // Label
            surface.draw_text(
                self.position.x as u32 + 12,
                item_y as u32 + 6,
                &item.label,
                Color::WHITE,
            );
            
            // Submenu arrow
            if matches!(item.action, super::desktop::ContextAction::Submenu(_)) {
                surface.draw_text(
                    self.position.x as u32 + menu_width - 20,
                    item_y as u32 + 6,
                    ">",
                    Color { r: 150, g: 150, b: 150, a: 255 },
                );
            }
        }
    }
}

/// Context menu action
#[derive(Debug, Clone)]
pub enum ContextMenuAction {
    None,
    Close,
    Command(String),
}

impl Default for StartMenuManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}