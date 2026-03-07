//! File Explorer Module
//! Windows-like file explorer with:
//! - Navigation pane
//! - Address bar
//! - File listing
//! - Context menus

use super::*;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::format;
use alloc::vec;

/// File entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// Entry name
    pub name: String,
    /// Full path
    pub path: String,
    /// Is directory
    pub is_dir: bool,
    /// File size
    pub size: u64,
    /// Modified time
    pub modified: u64,
    /// Is hidden
    pub hidden: bool,
    /// Is system
    pub system: bool,
    /// Is read-only
    pub read_only: bool,
    /// Extension
    pub extension: String,
}

impl FileEntry {
    /// Create new file entry
    pub fn new(name: String, path: String, is_dir: bool) -> Self {
        let extension = if !is_dir {
            name.rfind('.')
                .map(|i| name[i + 1..].to_string())
                .unwrap_or_default()
        } else {
            String::new()
        };
        
        Self {
            name,
            path,
            is_dir,
            size: 0,
            modified: 0,
            hidden: false,
            system: false,
            read_only: false,
            extension,
        }
    }
    
    /// Get icon for file type
    pub fn get_icon(&self) -> Icon {
        if self.is_dir {
            return Icon::Folder;
        }
        
        match self.extension.to_lowercase().as_str() {
            "exe" | "bin" | "app" => Icon::App(0),
            "txt" | "doc" | "docx" | "rtf" => Icon::File,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" => Icon::Custom(10),
            "mp3" | "wav" | "ogg" | "flac" => Icon::Custom(11),
            "mp4" | "avi" | "mkv" | "mov" => Icon::Custom(12),
            "zip" | "rar" | "7z" | "tar" | "gz" => Icon::Custom(13),
            "pdf" => Icon::Custom(14),
            _ => Icon::File,
        }
    }
}

/// View mode
#[derive(Debug, Clone, Copy)]
pub enum ViewMode {
    /// Large icons
    LargeIcons,
    /// Medium icons
    MediumIcons,
    /// Small icons
    SmallIcons,
    /// List view
    List,
    /// Details view
    Details,
    /// Tiles view
    Tiles,
    /// Content view
    Content,
}

impl Default for ViewMode {
    fn default() -> Self {
        Self::Details
    }
}

/// Navigation history
#[derive(Debug, Clone)]
pub struct NavigationHistory {
    /// History items
    items: Vec<String>,
    /// Current position
    current: usize,
}

impl NavigationHistory {
    pub fn new() -> Self {
        Self {
            items: vec![String::from("/")],
            current: 0,
        }
    }
    
    pub fn current(&self) -> &str {
        &self.items[self.current]
    }
    
    pub fn navigate(&mut self, path: &str) {
        // Remove forward history
        self.items.truncate(self.current + 1);
        self.items.push(String::from(path));
        self.current = self.items.len() - 1;
    }
    
    pub fn can_go_back(&self) -> bool {
        self.current > 0
    }
    
    pub fn can_go_forward(&self) -> bool {
        self.current < self.items.len() - 1
    }
    
    pub fn go_back(&mut self) -> Option<&str> {
        if self.can_go_back() {
            self.current -= 1;
            Some(self.current())
        } else {
            None
        }
    }
    
    pub fn go_forward(&mut self) -> Option<&str> {
        if self.can_go_forward() {
            self.current += 1;
            Some(self.current())
        } else {
            None
        }
    }
}

impl Default for NavigationHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// File explorer configuration
#[derive(Debug, Clone)]
pub struct ExplorerConfig {
    /// Show hidden files
    pub show_hidden: bool,
    /// Show file extensions
    pub show_extensions: bool,
    /// Show preview pane
    pub show_preview: bool,
    /// Show navigation pane
    pub show_navigation: bool,
    /// View mode
    pub view_mode: ViewMode,
    /// Sort column
    pub sort_column: SortColumn,
    /// Sort ascending
    pub sort_ascending: bool,
}

impl Default for ExplorerConfig {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_extensions: true,
            show_preview: false,
            show_navigation: true,
            view_mode: ViewMode::Details,
            sort_column: SortColumn::Name,
            sort_ascending: true,
        }
    }
}

/// Sort column
#[derive(Debug, Clone, Copy)]
pub enum SortColumn {
    Name,
    Size,
    Modified,
    Type,
}

/// File explorer manager
pub struct ExplorerManager {
    /// Configuration
    config: ExplorerConfig,
    /// Current path
    current_path: String,
    /// Current files
    files: Vec<FileEntry>,
    /// Selected files
    selected: Vec<usize>,
    /// Navigation history
    history: NavigationHistory,
    /// Address bar content
    address_bar: String,
    /// Search query
    search_query: String,
    /// Quick access locations
    quick_access: Vec<QuickAccessItem>,
    /// Drives
    drives: Vec<DriveInfo>,
    /// Column widths for details view
    column_widths: [u32; 4],
    /// Scroll position
    scroll_y: u32,
}

/// Quick access item
#[derive(Debug, Clone)]
pub struct QuickAccessItem {
    pub name: String,
    pub path: String,
    pub icon: Icon,
}

/// Drive information
#[derive(Debug, Clone)]
pub struct DriveInfo {
    pub letter: char,
    pub label: String,
    pub total_size: u64,
    pub free_space: u64,
    pub drive_type: DriveType,
    pub icon: Icon,
}

/// Drive type
#[derive(Debug, Clone, Copy)]
pub enum DriveType {
    Fixed,
    Removable,
    Optical,
    Network,
    RamDisk,
}

impl ExplorerManager {
    /// Create new explorer manager
    pub fn new() -> Self {
        let mut explorer = Self {
            config: ExplorerConfig::default(),
            current_path: String::from("/"),
            files: Vec::new(),
            selected: Vec::new(),
            history: NavigationHistory::new(),
            address_bar: String::from("/"),
            search_query: String::new(),
            quick_access: Vec::new(),
            drives: Vec::new(),
            column_widths: [200, 100, 120, 80],
            scroll_y: 0,
        };
        
        explorer.add_default_items();
        explorer
    }
    
    /// Add default quick access items and drives
    fn add_default_items(&mut self) {
        // Quick access
        self.quick_access.push(QuickAccessItem {
            name: String::from("Desktop"),
            path: String::from("/home/user/Desktop"),
            icon: Icon::Custom(20),
        });
        
        self.quick_access.push(QuickAccessItem {
            name: String::from("Downloads"),
            path: String::from("/home/user/Downloads"),
            icon: Icon::Custom(21),
        });
        
        self.quick_access.push(QuickAccessItem {
            name: String::from("Documents"),
            path: String::from("/home/user/Documents"),
            icon: Icon::Folder,
        });
        
        self.quick_access.push(QuickAccessItem {
            name: String::from("Pictures"),
            path: String::from("/home/user/Pictures"),
            icon: Icon::Custom(22),
        });
        
        self.quick_access.push(QuickAccessItem {
            name: String::from("Music"),
            path: String::from("/home/user/Music"),
            icon: Icon::Custom(23),
        });
        
        self.quick_access.push(QuickAccessItem {
            name: String::from("Videos"),
            path: String::from("/home/user/Videos"),
            icon: Icon::Custom(24),
        });
        
        // Drives
        self.drives.push(DriveInfo {
            letter: 'C',
            label: String::from("System"),
            total_size: 256 * 1024 * 1024 * 1024, // 256 GB
            free_space: 128 * 1024 * 1024 * 1024, // 128 GB
            drive_type: DriveType::Fixed,
            icon: Icon::Drive,
        });
        
        self.drives.push(DriveInfo {
            letter: 'D',
            label: String::from("Data"),
            total_size: 1024 * 1024 * 1024 * 1024, // 1 TB
            free_space: 512 * 1024 * 1024 * 1024, // 512 GB
            drive_type: DriveType::Fixed,
            icon: Icon::Drive,
        });
    }
    
    /// Navigate to path
    pub fn navigate(&mut self, path: &str) {
        self.current_path = String::from(path);
        self.address_bar = String::from(path);
        self.history.navigate(path);
        self.selected.clear();
        self.scroll_y = 0;
        self.refresh();
    }
    
    /// Refresh current directory
    pub fn refresh(&mut self) {
        // In a real implementation, this would read from the filesystem
        self.files.clear();
        
        // Add sample files
        self.files.push(FileEntry::new(
            String::from(".."),
            String::from("/"),
            true,
        ));
        
        self.files.push(FileEntry::new(
            String::from("Documents"),
            format!("{}/Documents", self.current_path),
            true,
        ));
        
        self.files.push(FileEntry::new(
            String::from("Pictures"),
            format!("{}/Pictures", self.current_path),
            true,
        ));
        
        self.files.push(FileEntry::new(
            String::from("readme.txt"),
            format!("{}/readme.txt", self.current_path),
            false,
        ));
    }
    
    /// Go back
    pub fn go_back(&mut self) {
        if let Some(path) = self.history.go_back() {
            self.current_path = String::from(path);
            self.address_bar = String::from(path);
            self.refresh();
        }
    }
    
    /// Go forward
    pub fn go_forward(&mut self) {
        if let Some(path) = self.history.go_forward() {
            self.current_path = String::from(path);
            self.address_bar = String::from(path);
            self.refresh();
        }
    }
    
    /// Go up one level
    pub fn go_up(&mut self) {
        if self.current_path != "/" {
            let parent = self.current_path.rfind('/')
                .map(|i| if i == 0 { String::from("/") } else { self.current_path[..i].to_string() })
                .unwrap_or_else(|| String::from("/"));
            self.navigate(&parent);
        }
    }
    
    /// Select item
    pub fn select(&mut self, index: usize, exclusive: bool) {
        if exclusive {
            self.selected.clear();
        }
        if !self.selected.contains(&index) {
            self.selected.push(index);
        }
    }
    
    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected.clear();
    }
    
    /// Select all
    pub fn select_all(&mut self) {
        self.selected = (0..self.files.len()).collect();
    }
    
    /// Delete selected
    pub fn delete_selected(&mut self) -> bool {
        // Sort selected indices in reverse order
        let mut to_delete: Vec<usize> = self.selected.iter().cloned().collect();
        to_delete.sort_by(|a, b| b.cmp(a));
        
        for index in to_delete {
            if index < self.files.len() {
                // In real implementation, delete from filesystem
                self.files.remove(index);
            }
        }
        
        self.selected.clear();
        true
    }
    
    /// Create new folder
    pub fn create_folder(&mut self, name: &str) {
        let path = format!("{}/{}", self.current_path, name);
        self.files.push(FileEntry::new(String::from(name), path, true));
    }
    
    /// Get file at position
    pub fn get_file_at(&self, pos: Position, rect: Rect) -> Option<usize> {
        let header_height = 80;
        let item_height = 24;
        let nav_width = if self.config.show_navigation { 200 } else { 0 };
        
        let list_x = rect.x + nav_width as i32;
        let list_y = rect.y + header_height as i32;
        
        if pos.x >= list_x && pos.x < rect.x + rect.width as i32 &&
           pos.y >= list_y && pos.y < rect.y + rect.height as i32 {
            let index = ((pos.y - list_y) / item_height) as usize;
            if index < self.files.len() {
                return Some(index);
            }
        }
        
        None
    }
    
    /// Handle event
    pub fn handle_event(&mut self, event: &UiEvent, rect: Rect) -> ExplorerAction {
        match event {
            UiEvent::Click(pos, button) => {
                if *button == MouseButton::Left {
                    // Check navigation buttons
                    let btn_y = rect.y + 10;
                    let btn_h = 28;
                    
                    // Back button
                    if pos.x >= rect.x + 10 && pos.x < rect.x + 38 &&
                       pos.y >= btn_y && pos.y < btn_y + btn_h {
                        self.go_back();
                        return ExplorerAction::Navigate;
                    }
                    
                    // Forward button
                    if pos.x >= rect.x + 42 && pos.x < rect.x + 70 &&
                       pos.y >= btn_y && pos.y < btn_y + btn_h {
                        self.go_forward();
                        return ExplorerAction::Navigate;
                    }
                    
                    // Up button
                    if pos.x >= rect.x + 74 && pos.x < rect.x + 102 &&
                       pos.y >= btn_y && pos.y < btn_y + btn_h {
                        self.go_up();
                        return ExplorerAction::Navigate;
                    }
                    
                    // File list click
                    if let Some(index) = self.get_file_at(*pos, rect) {
                        self.select(index, true);
                        return ExplorerAction::Select;
                    }
                }
            }
            
            UiEvent::DoubleClick(pos, button) => {
                if *button == MouseButton::Left {
                    if let Some(index) = self.get_file_at(*pos, rect) {
                        let (is_dir, path) = {
                            let file = &self.files[index];
                            (file.is_dir, file.path.clone())
                        };
                        if is_dir {
                            self.navigate(&path);
                            return ExplorerAction::Navigate;
                        } else {
                            return ExplorerAction::OpenFile(path);
                        }
                    }
                }
            }
            
            UiEvent::RightClick(pos) => {
                // Show context menu
                return ExplorerAction::ShowContextMenu(*pos);
            }
            
            _ => {}
        }
        
        ExplorerAction::None
    }
    
    /// Render explorer
    pub fn render(&self, surface: &mut dyn Surface, rect: Rect) {
        let nav_width = if self.config.show_navigation { 200 } else { 0 };
        
        // Toolbar area
        surface.fill_rect(
            Rect::new(rect.x, rect.y, rect.width, 80),
            Color::WINDOW_BG,
        );
        
        // Navigation buttons
        let btn_y = rect.y + 10;
        let btn_h = 28;
        let btn_w = 28;
        
        // Back button
        surface.fill_rounded_rect(
            Rect::new(rect.x + 10, btn_y, btn_w, btn_h),
            4,
            if self.history.can_go_back() { Color::HOVER } else { Color { r: 230, g: 230, b: 230, a: 255 } },
        );
        surface.draw_text(rect.x as u32 + 18, btn_y as u32 + 6, "<", Color::BLACK);
        
        // Forward button
        surface.fill_rounded_rect(
            Rect::new(rect.x + 42, btn_y, btn_w, btn_h),
            4,
            if self.history.can_go_forward() { Color::HOVER } else { Color { r: 230, g: 230, b: 230, a: 255 } },
        );
        surface.draw_text(rect.x as u32 + 50, btn_y as u32 + 6, ">", Color::BLACK);
        
        // Up button
        surface.fill_rounded_rect(
            Rect::new(rect.x + 74, btn_y, btn_w, btn_h),
            4,
            Color::HOVER,
        );
        surface.draw_text(rect.x as u32 + 82, btn_y as u32 + 6, "^", Color::BLACK);
        
        // Address bar
        let addr_y = btn_y + 32;
        surface.fill_rounded_rect(
            Rect::new(rect.x + 10, addr_y, rect.width - 20, 28),
            4,
            Color::WHITE,
        );
        surface.draw_text(rect.x as u32 + 20, addr_y as u32 + 6, &self.address_bar, Color::BLACK);
        
        // Navigation pane
        if self.config.show_navigation {
            let nav_rect = Rect::new(rect.x, rect.y + 80, nav_width, rect.height - 80);
            surface.fill_rect(nav_rect, Color { r: 245, g: 245, b: 245, a: 255 });
            
            // Quick access header
            surface.draw_text(rect.x as u32 + 10, rect.y as u32 + 90, "Quick access", Color { r: 100, g: 100, b: 100, a: 255 });
            
            // Quick access items
            let mut item_y = rect.y + 110;
            for item in &self.quick_access {
                surface.draw_icon(rect.x as u32 + 12, item_y as u32, item.icon, 16);
                surface.draw_text(rect.x as u32 + 32, item_y as u32 + 2, &item.name, Color::BLACK);
                item_y += 24;
            }
            
            // This PC header
            surface.draw_text(rect.x as u32 + 10, item_y as u32 + 10, "This PC", Color { r: 100, g: 100, b: 100, a: 255 });
            item_y += 30;
            
            // Drives
            for drive in &self.drives {
                surface.draw_icon(rect.x as u32 + 12, item_y as u32, drive.icon, 16);
                surface.draw_text(
                    rect.x as u32 + 32,
                    item_y as u32 + 2,
                    &format!("{}: {}", drive.letter, drive.label),
                    Color::BLACK,
                );
                item_y += 24;
            }
        }
        
        // File list area
        let list_x = rect.x + nav_width as i32;
        let list_width = rect.width - nav_width;
        let header_height = 80u32;
        
        // Details view header
        if matches!(self.config.view_mode, ViewMode::Details) {
            surface.fill_rect(
                Rect::new(list_x, rect.y + header_height as i32, list_width, 24),
                Color { r: 240, g: 240, b: 240, a: 255 },
            );
            
            let mut col_x = list_x as u32 + 10;
            let headers = ["Name", "Size", "Modified", "Type"];
            for (i, header) in headers.iter().enumerate() {
                surface.draw_text(col_x, rect.y as u32 + header_height + 6, header, Color { r: 80, g: 80, b: 80, a: 255 });
                col_x += self.column_widths[i];
            }
        }
        
        // File list
        let item_height = 24u32;
        let mut item_y = rect.y as u32 + header_height as u32 + 24;
        
        for (i, file) in self.files.iter().enumerate() {
            let is_selected = self.selected.contains(&i);
            
            if is_selected {
                surface.fill_rect(
                    Rect::new(list_x, item_y as i32, list_width, item_height),
                    Color::SELECTED,
                );
            }
            
            // Icon
            surface.draw_icon(list_x as u32 + 4, item_y + 4, file.get_icon(), 16);
            
            // Name
            surface.draw_text(list_x as u32 + 24, item_y + 4, &file.name, Color::BLACK);
            
            item_y += item_height;
        }
    }
}

/// Explorer action
#[derive(Debug, Clone)]
pub enum ExplorerAction {
    None,
    Navigate,
    Select,
    OpenFile(String),
    ShowContextMenu(Position),
}

impl Default for ExplorerManager {
    fn default() -> Self {
        Self::new()
    }
}