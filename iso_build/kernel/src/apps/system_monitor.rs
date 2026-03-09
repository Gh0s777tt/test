//! System Monitor Application
use alloc::string::String;
use crate::gui::*;

pub struct SystemMonitor {
    cpu_usage: u32,
    memory_usage: u32,
    disk_usage: u32,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            cpu_usage: 15,
            memory_usage: 42,
            disk_usage: 35,
        }
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        surface.fill_rect(Rect::new(0, 0, width, height), Color::new(30, 30, 30));
        
        surface.draw_text_sized(20, 20, "System Monitor", 18, Color::WHITE);
        
        surface.draw_text_sized(20, 70, "CPU Usage:", 13, Color::LIGHT_GRAY);
        self.draw_progress(surface, 20, 95, 300, self.cpu_usage);
        
        surface.draw_text_sized(20, 140, "Memory Usage:", 13, Color::LIGHT_GRAY);
        self.draw_progress(surface, 20, 165, 300, self.memory_usage);
        
        surface.draw_text_sized(20, 210, "Disk Usage:", 13, Color::LIGHT_GRAY);
        self.draw_progress(surface, 20, 235, 300, self.disk_usage);
    }
    
    fn draw_progress(&self, surface: &mut dyn Surface, x: u32, y: u32, w: u32, value: u32) {
        surface.fill_rect(Rect::new(x as i32, y as i32, w, 20), Color::new(50, 50, 50));
        surface.fill_rect(Rect::new(x as i32, y as i32, w * value / 100, 20), Color::ACCENT);
        surface.draw_text_sized(x + w + 10, y + 3, &alloc::format!("{}%", value), 12, Color::WHITE);
    }
}

impl Default for SystemMonitor {
    fn default() -> Self { Self::new() }
}