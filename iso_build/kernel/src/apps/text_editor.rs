//! Text Editor Application
use alloc::string::String;
use alloc::format;
use crate::gui::*;

pub struct TextEditor {
    content: String,
    cursor_pos: usize,
    filename: String,
    modified: bool,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            cursor_pos: 0,
            filename: String::from("untitled.txt"),
            modified: false,
        }
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        surface.fill_rect(Rect::new(0, 0, width, height), Color::new(30, 30, 30));
        
        surface.fill_rect(Rect::new(0, 0, width, 40), Color::new(40, 40, 40));
        surface.draw_text_sized(10, 10, &format!("{} {}", self.filename, if self.modified { "*" } else { "" }), 14, Color::WHITE);
        
        surface.draw_text_sized(10, 60, &self.content, 12, Color::WHITE);
    }
}

impl Default for TextEditor {
    fn default() -> Self { Self::new() }
}