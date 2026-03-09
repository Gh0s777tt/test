//! Calendar Application
use alloc::format;
use crate::gui::*;

pub struct Calendar {
    month: u32,
    year: u32,
}

impl Calendar {
    pub fn new() -> Self {
        Self { month: 3, year: 2025 }
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        
        surface.fill_rect(Rect::new(0, 0, width, surface.height()), Color::new(30, 30, 30));
        
        let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
        
        surface.draw_text_sized(width/2 - 60, 20, &format!("{} {}", months[self.month as usize - 1], self.year), 20, Color::WHITE);
        
        // Day headers
        let days = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
        for (i, day) in days.iter().enumerate() {
            surface.draw_text_sized(30 + (i as u32) * 80, 60, day, 12, Color::LIGHT_GRAY);
        }
        
        // Calendar grid
        let start_day = 6; // March 2025 starts on Saturday
        let days_in_month = 31;
        
        for day in 1..=days_in_month {
            let pos = start_day + day - 1;
            let row = pos / 7;
            let col = pos % 7;
            let x = 30 + col * 80;
            let y = 90 + row * 40;
            
            let is_today = day == 8;
            if is_today {
                surface.fill_rounded_rect(Rect::new(x as i32 - 5, y as i32 - 5, 35, 35), 4, Color::ACCENT);
            }
            
            surface.draw_text_sized(x, y, &format!("{}", day), 14, if is_today { Color::WHITE } else { Color::WHITE });
        }
    }
}

impl Default for Calendar {
    fn default() -> Self { Self::new() }
}