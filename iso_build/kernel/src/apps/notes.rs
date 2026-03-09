//! Notes Application
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use crate::gui::*;

pub struct Notes {
    notes: Vec<Note>,
    current: usize,
}

struct Note {
    title: String,
    content: String,
}

impl Notes {
    pub fn new() -> Self {
        Self {
            notes: vec![Note { title: String::from("Welcome"), content: String::from("Welcome to VantisOS Notes!") }],
            current: 0,
        }
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        surface.fill_rect(Rect::new(0, 0, width, height), Color::new(30, 30, 30));
        
        // Sidebar
        surface.fill_rect(Rect::new(0, 0, 200, height), Color::new(35, 35, 35));
        surface.draw_text_sized(20, 20, "Notes", 18, Color::WHITE);
        
        // Note list
        for (i, note) in self.notes.iter().enumerate() {
            let is_selected = i == self.current;
            if is_selected {
                surface.fill_rect(Rect::new(0, (50 + (i as u32) * 50) as i32, 200, 50), Color::ACCENT);
            }
            surface.draw_text_sized(20, 60 + (i as u32) * 50, &note.title, 13, Color::WHITE);
        }
        
        // Content area
        if let Some(note) = self.notes.get(self.current) {
            surface.draw_text_sized(220, 20, &note.title, 18, Color::WHITE);
            surface.draw_text_sized(220, 60, &note.content, 13, Color::LIGHT_GRAY);
        }
    }
}

impl Default for Notes {
    fn default() -> Self { Self::new() }
}