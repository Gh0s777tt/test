//! File Manager Application
//! Complete file browser for VantisOS

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;
use crate::gui::*;

/// File entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: u64,
}

/// File Manager application
pub struct FileManager {
    /// Current path
    pub current_path: String,
    /// Files in current directory
    pub files: Vec<FileEntry>,
    /// Selected files
    pub selected: Vec<usize>,
    /// Navigation history
    pub history: Vec<String>,
    /// Current position in history
    pub history_pos: usize,
    /// Show hidden files
    pub show_hidden: bool,
    /// View mode
    pub view_mode: ViewMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Icons,
    List,
    Details,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            current_path: String::from("/"),
            files: vec![
                FileEntry { name: String::from("bin"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("boot"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("dev"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("etc"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("home"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("lib"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("opt"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("proc"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("root"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("sys"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("tmp"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("usr"), is_dir: true, size: 0, modified: 0 },
                FileEntry { name: String::from("var"), is_dir: true, size: 0, modified: 0 },
            ],
            selected: Vec::new(),
            history: vec![String::from("/")],
            history_pos: 0,
            show_hidden: false,
            view_mode: ViewMode::List,
        }
    }
    
    pub fn navigate_to(&mut self, path: &str) {
        self.current_path = String::from(path);
        // Truncate history after current position
        self.history.truncate(self.history_pos + 1);
        self.history.push(String::from(path));
        self.history_pos = self.history.len() - 1;
        self.selected.clear();
        self.refresh();
    }
    
    pub fn go_back(&mut self) {
        if self.history_pos > 0 {
            self.history_pos -= 1;
            self.current_path = self.history[self.history_pos].clone();
            self.selected.clear();
            self.refresh();
        }
    }
    
    pub fn go_forward(&mut self) {
        if self.history_pos < self.history.len() - 1 {
            self.history_pos += 1;
            self.current_path = self.history[self.history_pos].clone();
            self.selected.clear();
            self.refresh();
        }
    }
    
    pub fn go_up(&mut self) {
        if self.current_path != "/" {
            let parent = self.current_path.rfind('/').map(|i| {
                if i == 0 { String::from("/") } else { self.current_path[..i].to_string() }
            }).unwrap_or(String::from("/"));
            self.navigate_to(&parent);
        }
    }
    
    pub fn refresh(&mut self) {
        // In a real system, this would read the actual directory
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        // Toolbar
        surface.fill_rect(Rect::new(0, 0, width, 48), Color::new(45, 45, 45));
        
        // Navigation buttons
        let btn_y = 8;
        
        // Back button
        surface.fill_rounded_rect(Rect::new(12, btn_y, 32, 32), 4, Color::new(60, 60, 60));
        surface.draw_icon(20, btn_y as u32 + 8, Icon::Back, 16);
        
        // Forward button
        surface.fill_rounded_rect(Rect::new(52, btn_y, 32, 32), 4, Color::new(60, 60, 60));
        surface.draw_icon(60, btn_y as u32 + 8, Icon::Forward, 16);
        
        // Up button
        surface.fill_rounded_rect(Rect::new(92, btn_y, 32, 32), 4, Color::new(60, 60, 60));
        surface.draw_icon(100, btn_y as u32 + 8, Icon::Up, 16);
        
        // Path bar
        surface.fill_rect(Rect::new(140, btn_y, width - 200, 32), Color::new(35, 35, 35));
        surface.draw_rect(Rect::new(140, btn_y, width - 200, 32), Color::new(60, 60, 60), 1);
        surface.draw_text_sized(150, btn_y as u32 + 9, &self.current_path, 13, Color::WHITE);
        
        // Sidebar
        surface.fill_rect(Rect::new(0, 48, 180, height - 48), Color::new(38, 38, 38));
        
        let places = ["Home", "Desktop", "Documents", "Downloads", "Pictures", "Music", "Videos", "Trash"];
        for (i, place) in places.iter().enumerate() {
            surface.draw_text_sized(20, 68 + (i as u32) * 28, place, 12, Color::LIGHT_GRAY);
        }
        
        // File list
        let list_y = 56u32;
        let list_x = 190i32;
        
        // Column headers
        surface.fill_rect(Rect::new(list_x, 48, width - list_x as u32, 24), Color::new(45, 45, 45));
        surface.draw_text_sized(list_x as u32 + 10, 54, "Name", 11, Color::LIGHT_GRAY);
        surface.draw_text_sized(width - 180, 54, "Size", 11, Color::LIGHT_GRAY);
        surface.draw_text_sized(width - 100, 54, "Modified", 11, Color::LIGHT_GRAY);
        
        // Files
        for (i, file) in self.files.iter().enumerate() {
            let item_y = list_y + (i as u32) * 28;
            if item_y > height - 28 {
                break;
            }
            
            let is_selected = self.selected.contains(&i);
            if is_selected {
                surface.fill_rect(Rect::new(list_x, item_y as i32 - 4, width - list_x as u32, 26), Color::SELECTED);
            }
            
            let icon = if file.is_dir { Icon::Folder } else { Icon::File };
            surface.draw_icon(list_x as u32 + 10, item_y, icon, 16);
            
            surface.draw_text_sized(
                list_x as u32 + 32,
                item_y + 2,
                &file.name,
                12,
                if file.is_dir { Color::new(100, 180, 255) } else { Color::WHITE },
            );
            
            if !file.is_dir {
                let size_text = format_size(file.size);
                surface.draw_text_sized(width - 180, item_y + 2, &size_text, 11, Color::LIGHT_GRAY);
            }
        }
        
        // Status bar
        surface.fill_rect(Rect::new(0, (height - 28) as i32, width, 28), Color::new(35, 35, 35));
        surface.draw_text_sized(10, height - 22, &format!("{} items", self.files.len()), 11, Color::LIGHT_GRAY);
    }
}

fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{} KB", size / 1024)
    } else if size < 1024 * 1024 * 1024 {
        format!("{} MB", size / (1024 * 1024))
    } else {
        format!("{} GB", size / (1024 * 1024 * 1024))
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}