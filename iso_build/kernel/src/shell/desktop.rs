//! Desktop Environment
//! Manages the desktop with icons, wallpaper, and context menus

use super::*;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;

/// Desktop icon
#[derive(Debug, Clone)]
pub struct DesktopIcon {
    /// Unique ID
    pub id: ElementId,
    /// Display name
    pub name: String,
    /// Position
    pub position: Position,
    /// Icon type
    pub icon: Icon,
    /// Target path (for files/folders)
    pub target: String,
    /// Is selected
    pub selected: bool,
    /// Double-click handler
    pub action: DesktopAction,
}

/// Desktop icon action
#[derive(Debug, Clone)]
pub enum DesktopAction {
    /// Open file/folder
    Open(String),
    /// Run application
    Run(String),
    /// Open URL
    Url(String),
    /// System action
    System(SystemAction),
    /// None
    None,
}

/// System actions
#[derive(Debug, Clone, Copy)]
pub enum SystemAction {
    Settings,
    PowerOff,
    Reboot,
    Sleep,
    LogOut,
    Lock,
    FileExplorer,
    Terminal,
}

/// Desktop configuration
#[derive(Debug, Clone)]
pub struct DesktopConfig {
    /// Wallpaper path
    pub wallpaper_path: String,
    /// Wallpaper style
    pub wallpaper_style: WallpaperStyle,
    /// Icon size
    pub icon_size: u32,
    /// Icon spacing
    pub icon_spacing: u32,
    /// Grid alignment
    pub grid_align: bool,
    /// Show hidden files
    pub show_hidden: bool,
    /// Auto arrange icons
    pub auto_arrange: bool,
    /// Sort by
    pub sort_by: SortBy,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            wallpaper_path: String::new(),
            wallpaper_style: WallpaperStyle::Stretch,
            icon_size: 48,
            icon_spacing: 10,
            grid_align: true,
            show_hidden: false,
            auto_arrange: true,
            sort_by: SortBy::Name,
        }
    }
}

/// Wallpaper style
#[derive(Debug, Clone, Copy)]
pub enum WallpaperStyle {
    Center,
    Tile,
    Stretch,
    Fit,
    Fill,
    Span,
}

/// Sort order
#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    Name,
    Size,
    Type,
    Modified,
}

/// Desktop manager
pub struct DesktopManager {
    /// Icons on desktop
    icons: Vec<DesktopIcon>,
    /// Configuration
    config: DesktopConfig,
    /// Selected icons
    selected: Vec<ElementId>,
    /// Dragging icon
    dragging: Option<ElementId>,
    /// Drag offset
    drag_offset: Position,
    /// Context menu visible
    context_menu_visible: bool,
    /// Context menu position
    context_menu_pos: Position,
    /// Next icon ID
    next_id: ElementId,
}

impl DesktopManager {
    /// Create new desktop manager
    pub fn new() -> Self {
        let mut desktop = Self {
            icons: Vec::new(),
            config: DesktopConfig::default(),
            selected: Vec::new(),
            dragging: None,
            drag_offset: Position::default(),
            context_menu_visible: false,
            context_menu_pos: Position::default(),
            next_id: 1,
        };
        
        // Add default icons
        desktop.add_default_icons();
        desktop
    }
    
    /// Add default desktop icons
    fn add_default_icons(&mut self) {
        let id1 = self.next_id();
        self.add_icon(DesktopIcon {
            id: id1,
            name: String::from("This PC"),
            position: Position { x: 20, y: 20 },
            icon: Icon::Computer,
            target: String::from("computer://"),
            selected: false,
            action: DesktopAction::System(SystemAction::FileExplorer),
        });
        
        let id2 = self.next_id();
        self.add_icon(DesktopIcon {
            id: id2,
            name: String::from("Recycle Bin"),
            position: Position { x: 20, y: 100 },
            icon: Icon::Custom(0),
            target: String::from("trash://"),
            selected: false,
            action: DesktopAction::Open(String::from("/trash")),
        });
        
        let id3 = self.next_id();
        self.add_icon(DesktopIcon {
            id: id3,
            name: String::from("Documents"),
            position: Position { x: 20, y: 180 },
            icon: Icon::Folder,
            target: String::from("/home/user/Documents"),
            selected: false,
            action: DesktopAction::Open(String::from("/home/user/Documents")),
        });
        
        let id4 = self.next_id();
        self.add_icon(DesktopIcon {
            id: id4,
            name: String::from("Settings"),
            position: Position { x: 20, y: 260 },
            icon: Icon::Settings,
            target: String::from("settings://"),
            selected: false,
            action: DesktopAction::System(SystemAction::Settings),
        });
    }
    
    /// Get next ID
    fn next_id(&mut self) -> ElementId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    
    /// Add icon
    pub fn add_icon(&mut self, icon: DesktopIcon) {
        self.icons.push(icon);
    }
    
    /// Remove icon
    pub fn remove_icon(&mut self, id: ElementId) {
        self.icons.retain(|i| i.id != id);
        self.selected.retain(|&i| i != id);
    }
    
    /// Get icon at position
    pub fn get_icon_at(&self, pos: Position) -> Option<&DesktopIcon> {
        let icon_size = self.config.icon_size as i32;
        
        for icon in self.icons.iter().rev() {
            let rect = Rect::new(
                icon.position.x,
                icon.position.y,
                icon_size as u32,
                icon_size as u32 + 20, // Icon + text height
            );
            if rect.contains(pos) {
                return Some(icon);
            }
        }
        None
    }
    
    /// Select icon
    pub fn select_icon(&mut self, id: ElementId, exclusive: bool) {
        if exclusive {
            for icon in &mut self.icons {
                icon.selected = false;
            }
            self.selected.clear();
        }
        
        if let Some(icon) = self.icons.iter_mut().find(|i| i.id == id) {
            icon.selected = true;
            self.selected.push(id);
        }
    }
    
    /// Clear selection
    pub fn clear_selection(&mut self) {
        for icon in &mut self.icons {
            icon.selected = false;
        }
        self.selected.clear();
    }
    
    /// Start drag
    pub fn start_drag(&mut self, id: ElementId, offset: Position) {
        self.dragging = Some(id);
        self.drag_offset = offset;
    }
    
    /// End drag
    pub fn end_drag(&mut self) {
        self.dragging = None;
    }
    
    /// Update drag position
    pub fn update_drag(&mut self, pos: Position) {
        if let Some(id) = self.dragging {
            if let Some(icon) = self.icons.iter_mut().find(|i| i.id == id) {
                icon.position.x = pos.x - self.drag_offset.x;
                icon.position.y = pos.y - self.drag_offset.y;
                
                // Grid alignment
                if self.config.grid_align {
                    let grid = self.config.icon_size + self.config.icon_spacing;
                    icon.position.x = (icon.position.x / grid as i32) * grid as i32;
                    icon.position.y = (icon.position.y / grid as i32) * grid as i32;
                }
            }
        }
    }
    
    /// Show context menu
    pub fn show_context_menu(&mut self, pos: Position) {
        self.context_menu_visible = true;
        self.context_menu_pos = pos;
    }
    
    /// Hide context menu
    pub fn hide_context_menu(&mut self) {
        self.context_menu_visible = false;
    }
    
    /// Handle event
    pub fn handle_event(&mut self, event: &UiEvent) -> bool {
        match event {
            UiEvent::MouseDown(pos, button) => {
                if *button == MouseButton::Left {
                    if let Some(icon) = self.get_icon_at(*pos) {
                        let id = icon.id;
                        let offset = Position {
                            x: pos.x - icon.position.x,
                            y: pos.y - icon.position.y,
                        };
                        self.select_icon(id, true);
                        self.start_drag(id, offset);
                        return true;
                    } else {
                        self.clear_selection();
                    }
                } else if *button == MouseButton::Right {
                    if self.get_icon_at(*pos).is_some() {
                        // Icon context menu
                        self.show_context_menu(*pos);
                    } else {
                        // Desktop context menu
                        self.clear_selection();
                        self.show_context_menu(*pos);
                    }
                    return true;
                }
            }
            
            UiEvent::MouseUp(pos, button) => {
                if *button == MouseButton::Left {
                    self.end_drag();
                }
            }
            
            UiEvent::MouseMove(pos) => {
                self.update_drag(*pos);
            }
            
            UiEvent::DoubleClick(pos, button) => {
                if *button == MouseButton::Left {
                    if let Some(icon) = self.get_icon_at(*pos) {
                        // Execute action
                        match &icon.action {
                            DesktopAction::Open(path) => {
                                // Open file/folder
                            }
                            DesktopAction::Run(app) => {
                                // Run application
                            }
                            DesktopAction::Url(url) => {
                                // Open URL
                            }
                            DesktopAction::System(action) => {
                                // System action
                            }
                            DesktopAction::None => {}
                        }
                        return true;
                    }
                }
            }
            
            UiEvent::RightClick(pos) => {
                self.show_context_menu(*pos);
                return true;
            }
            
            _ => {}
        }
        
        false
    }
    
    /// Render desktop
    pub fn render(&self, surface: &mut dyn Surface) {
        let (width, height) = surface.dimensions();
        
        // Draw background
        surface.fill_rect(
            Rect::new(0, 0, width, height),
            Color::DESKTOP_BG
        );
        
        // Draw icons
        let icon_size = self.config.icon_size;
        for icon in &self.icons {
            // Draw selection background
            if icon.selected {
                surface.fill_rounded_rect(
                    Rect::new(
                        icon.position.x - 2,
                        icon.position.y - 2,
                        icon_size + 4,
                        icon_size + 24,
                    ),
                    4,
                    Color::SELECTED,
                );
            }
            
            // Draw icon
            surface.draw_icon(
                icon.position.x as u32,
                icon.position.y as u32,
                icon.icon,
                icon_size,
            );
            
            // Draw text
            surface.draw_text_sized(
                icon.position.x as u32,
                icon.position.y as u32 + icon_size + 2,
                &icon.name,
                10,
                if icon.selected { Color::WHITE } else { Color::WHITE },
            );
        }
    }
    
    /// Get context menu items for desktop
    pub fn get_context_menu_items(&self) -> Vec<ContextMenuItem> {
        vec![
            ContextMenuItem {
                label: String::from("View"),
                action: ContextAction::Submenu(vec![
                    ContextMenuItem {
                        label: String::from("Large icons"),
                        action: ContextAction::None,
                    },
                    ContextMenuItem {
                        label: String::from("Medium icons"),
                        action: ContextAction::None,
                    },
                    ContextMenuItem {
                        label: String::from("Small icons"),
                        action: ContextAction::None,
                    },
                ]),
            },
            ContextMenuItem {
                label: String::from("Sort by"),
                action: ContextAction::Submenu(vec![
                    ContextMenuItem {
                        label: String::from("Name"),
                        action: ContextAction::None,
                    },
                    ContextMenuItem {
                        label: String::from("Size"),
                        action: ContextAction::None,
                    },
                    ContextMenuItem {
                        label: String::from("Type"),
                        action: ContextAction::None,
                    },
                    ContextMenuItem {
                        label: String::from("Modified"),
                        action: ContextAction::None,
                    },
                ]),
            },
            ContextMenuItem {
                label: String::from("Refresh"),
                action: ContextAction::Command(String::from("refresh")),
            },
            ContextMenuItem {
                label: String::from("New"),
                action: ContextAction::Submenu(vec![
                    ContextMenuItem {
                        label: String::from("Folder"),
                        action: ContextAction::Command(String::from("new_folder")),
                    },
                    ContextMenuItem {
                        label: String::from("Text Document"),
                        action: ContextAction::Command(String::from("new_text")),
                    },
                    ContextMenuItem {
                        label: String::from("Shortcut"),
                        action: ContextAction::Command(String::from("new_shortcut")),
                    },
                ]),
            },
            ContextMenuItem {
                label: String::from("Display settings"),
                action: ContextAction::Command(String::from("display_settings")),
            },
            ContextMenuItem {
                label: String::from("Personalize"),
                action: ContextAction::Command(String::from("personalize")),
            },
        ]
    }
}

/// Context menu item
#[derive(Debug, Clone)]
pub struct ContextMenuItem {
    pub label: String,
    pub action: ContextAction,
}

/// Context menu action
#[derive(Debug, Clone)]
pub enum ContextAction {
    Command(String),
    Submenu(Vec<ContextMenuItem>),
    None,
}

impl Default for DesktopManager {
    fn default() -> Self {
        Self::new()
    }
}