//! Calculator Application
use alloc::string::String;
use alloc::format;
use crate::gui::*;

pub struct Calculator {
    display: String,
    operand: f64,
    operator: Option<char>,
    waiting: bool,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            display: String::from("0"),
            operand: 0.0,
            operator: None,
            waiting: false,
        }
    }
    
    pub fn render(&self, surface: &mut dyn Surface) {
        let width = surface.width();
        let height = surface.height();
        
        surface.fill_rect(Rect::new(0, 0, width, height), Color::new(30, 30, 30));
        
        // Display
        surface.fill_rect(Rect::new(20, 20, width - 40, 60), Color::new(40, 40, 40));
        surface.draw_text_sized(width - 50, 40, &self.display, 28, Color::WHITE);
        
        // Buttons
        let buttons = ["C", "±", "%", "÷", "7", "8", "9", "×", "4", "5", "6", "-", "1", "2", "3", "+", "0", ".", "="];
        for (i, btn) in buttons.iter().enumerate() {
            let row = i / 4;
            let col = i % 4;
            let x = 20 + (col as u32) * 70;
            let y = 100 + (row as u32) * 60;
            
            let bg = match *btn {
                "÷" | "×" | "-" | "+" | "=" => Color::ACCENT,
                "C" | "±" | "%" => Color::new(60, 60, 60),
                _ => Color::new(80, 80, 80),
            };
            
            surface.fill_rounded_rect(Rect::new(x as i32, y as i32, 60, 50), 8, bg);
            surface.draw_text_sized(x + 25, y + 15, btn, 18, Color::WHITE);
        }
    }
}

impl Default for Calculator {
    fn default() -> Self { Self::new() }
}