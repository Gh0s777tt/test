//! GUI Widgets
//! All standard widgets for VantisOS GUI

use super::*;
use alloc::string::String;
use alloc::boxed::Box;

/// Button widget
pub struct Button {
    id: ElementId,
    bounds: Rect,
    text: String,
    visible: bool,
    enabled: bool,
    hovered: bool,
    pressed: bool,
    on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new(text: &str) -> Self {
        Self {
            id: 0,
            bounds: Rect::new(0, 0, 100, 32),
            text: String::from(text),
            visible: true,
            enabled: true,
            hovered: false,
            pressed: false,
            on_click: None,
        }
    }
    
    pub fn with_bounds(mut self, x: i32, y: i32, w: u32, h: u32) -> Self {
        self.bounds = Rect::new(x, y, w, h);
        self
    }
    
    pub fn with_on_click(mut self, handler: Box<dyn FnMut()>) -> Self {
        self.on_click = Some(handler);
        self
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }
}

impl Widget for Button {
    fn id(&self) -> ElementId { self.id }
    fn bounds(&self) -> Rect { self.bounds }
    fn set_bounds(&mut self, rect: Rect) { self.bounds = rect; }
    
    fn is_visible(&self) -> bool { self.visible }
    fn set_visible(&mut self, visible: bool) { self.visible = visible; }
    fn is_enabled(&self) -> bool { self.enabled }
    fn set_enabled(&mut self, enabled: bool) { self.enabled = enabled; }
    
    fn can_focus(&self) -> bool { true }
    fn on_focus(&mut self) {}
    fn on_blur(&mut self) {}
    
    fn handle_event(&mut self, event: &UiEvent) -> bool {
        if !self.visible || !self.enabled {
            return false;
        }
        
        match event {
            UiEvent::MouseMove(pos) => {
                let was_hovered = self.hovered;
                self.hovered = self.bounds.contains(*pos);
                was_hovered != self.hovered
            }
            
            UiEvent::MouseDown(pos, MouseButton::Left) => {
                if self.bounds.contains(*pos) {
                    self.pressed = true;
                    true
                } else {
                    false
                }
            }
            
            UiEvent::MouseUp(pos, MouseButton::Left) => {
                if self.pressed && self.bounds.contains(*pos) {
                    self.pressed = false;
                    if let Some(handler) = &mut self.on_click {
                        handler();
                    }
                    true
                } else {
                    self.pressed = false;
                    false
                }
            }
            
            _ => false
        }
    }
    
    fn render(&mut self, surface: &mut dyn Surface) {
        if !self.visible {
            return;
        }
        
        let bg_color = if !self.enabled {
            Color::LIGHT_GRAY
        } else if self.pressed {
            Color::BUTTON_PRESSED
        } else if self.hovered {
            Color::BUTTON_HOVER
        } else {
            Color::BUTTON
        };
        
        // Draw button background
        surface.fill_rounded_rect(self.bounds, 4, bg_color);
        
        // Draw border
        surface.draw_rect(self.bounds, Color::GRAY, 1);
        
        // Draw text centered
        let (tw, th) = surface.measure_text(&self.text, 12);
        let tx = self.bounds.x + (self.bounds.width as i32 - tw as i32) / 2;
        let ty = self.bounds.y + (self.bounds.height as i32 - th as i32) / 2;
        surface.draw_text_sized(tx as u32, ty as u32, &self.text, 12, 
            if self.enabled { Color::BLACK } else { Color::GRAY });
    }
}

/// Label widget
pub struct Label {
    id: ElementId,
    bounds: Rect,
    text: String,
    visible: bool,
    color: Color,
    font_size: u32,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Self {
            id: 0,
            bounds: Rect::new(0, 0, 100, 20),
            text: String::from(text),
            visible: true,
            color: Color::BLACK,
            font_size: 12,
        }
    }
    
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }
}

impl Widget for Label {
    fn id(&self) -> ElementId { self.id }
    fn bounds(&self) -> Rect { self.bounds }
    fn set_bounds(&mut self, rect: Rect) { self.bounds = rect; }
    
    fn is_visible(&self) -> bool { self.visible }
    fn set_visible(&mut self, visible: bool) { self.visible = visible; }
    fn is_enabled(&self) -> bool { true }
    fn set_enabled(&mut self, _enabled: bool) {}
    
    fn can_focus(&self) -> bool { false }
    fn on_focus(&mut self) {}
    fn on_blur(&mut self) {}
    
    fn handle_event(&mut self, _event: &UiEvent) -> bool {
        false
    }
    
    fn render(&mut self, surface: &mut dyn Surface) {
        if !self.visible {
            return;
        }
        
        surface.draw_text_sized(
            self.bounds.x as u32,
            self.bounds.y as u32,
            &self.text,
            self.font_size,
            self.color,
        );
    }
}

/// TextBox widget
pub struct TextBox {
    id: ElementId,
    bounds: Rect,
    text: String,
    visible: bool,
    enabled: bool,
    focused: bool,
    cursor_pos: usize,
    scroll_offset: u32,
    placeholder: String,
    password: bool,
    max_length: usize,
    on_change: Option<Box<dyn FnMut(&str)>>,
    on_enter: Option<Box<dyn FnMut(&str)>>,
}

impl TextBox {
    pub fn new() -> Self {
        Self {
            id: 0,
            bounds: Rect::new(0, 0, 200, 28),
            text: String::new(),
            visible: true,
            enabled: true,
            focused: false,
            cursor_pos: 0,
            scroll_offset: 0,
            placeholder: String::new(),
            password: false,
            max_length: 256,
            on_change: None,
            on_enter: None,
        }
    }
    
    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = String::from(placeholder);
        self
    }
    
    pub fn with_password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }
    
    pub fn text(&self) -> &str {
        &self.text
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
        self.cursor_pos = self.text.len();
    }
}

impl Default for TextBox {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for TextBox {
    fn id(&self) -> ElementId { self.id }
    fn bounds(&self) -> Rect { self.bounds }
    fn set_bounds(&mut self, rect: Rect) { self.bounds = rect; }
    
    fn is_visible(&self) -> bool { self.visible }
    fn set_visible(&mut self, visible: bool) { self.visible = visible; }
    fn is_enabled(&self) -> bool { self.enabled }
    fn set_enabled(&mut self, enabled: bool) { self.enabled = enabled; }
    
    fn can_focus(&self) -> bool { true }
    fn on_focus(&mut self) { self.focused = true; }
    fn on_blur(&mut self) { self.focused = false; }
    
    fn handle_event(&mut self, event: &UiEvent) -> bool {
        if !self.visible || !self.enabled {
            return false;
        }
        
        match event {
            UiEvent::MouseDown(pos, MouseButton::Left) => {
                self.bounds.contains(*pos)
            }
            
            UiEvent::Char(c) => {
                if self.focused && self.text.len() < self.max_length {
                    self.text.insert(self.cursor_pos, *c);
                    self.cursor_pos += 1;
                    if let Some(handler) = &mut self.on_change {
                        handler(&self.text);
                    }
                    true
                } else {
                    false
                }
            }
            
            UiEvent::KeyDown(key) => {
                match *key {
                    8 => { // Backspace
                        if self.cursor_pos > 0 {
                            self.text.remove(self.cursor_pos - 1);
                            self.cursor_pos -= 1;
                            if let Some(handler) = &mut self.on_change {
                                handler(&self.text);
                            }
                        }
                        true
                    }
                    13 => { // Enter
                        if let Some(handler) = &mut self.on_enter {
                            handler(&self.text);
                        }
                        true
                    }
                    37 => { // Left arrow
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                        }
                        true
                    }
                    39 => { // Right arrow
                        if self.cursor_pos < self.text.len() {
                            self.cursor_pos += 1;
                        }
                        true
                    }
                    36 => { // Home
                        self.cursor_pos = 0;
                        true
                    }
                    35 => { // End
                        self.cursor_pos = self.text.len();
                        true
                    }
                    _ => false
                }
            }
            
            _ => false
        }
    }
    
    fn render(&mut self, surface: &mut dyn Surface) {
        if !self.visible {
            return;
        }
        
        // Background
        surface.fill_rect(self.bounds, Color::WHITE);
        
        // Border
        let border_color = if self.focused { Color::ACCENT } else { Color::GRAY };
        surface.draw_rect(self.bounds, border_color, 1);
        
        // Text or placeholder
        let display_text = if self.text.is_empty() && !self.focused {
            &self.placeholder
        } else if self.password {
            &"\u{2022}".repeat(self.text.len())
        } else {
            &self.text
        };
        
        let text_color = if self.text.is_empty() && !self.focused {
            Color::GRAY
        } else {
            Color::BLACK
        };
        
        surface.draw_text_sized(
            self.bounds.x as u32 + 8,
            self.bounds.y as u32 + 8,
            display_text,
            12,
            text_color,
        );
        
        // Cursor
        if self.focused {
            let cursor_x = self.bounds.x as u32 + 8 + 
                surface.measure_text(&display_text[..self.cursor_pos.min(display_text.len())], 12).0;
            surface.draw_line(
                cursor_x as i32,
                self.bounds.y + 6,
                cursor_x as i32,
                self.bounds.y + 22,
                Color::BLACK,
                1,
            );
        }
    }
}

/// Panel/Container widget
pub struct Panel {
    id: ElementId,
    bounds: Rect,
    visible: bool,
    children: Vec<Box<dyn Widget>>,
    background: Color,
    border: Option<Color>,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            id: 0,
            bounds: Rect::new(0, 0, 100, 100),
            visible: true,
            children: Vec::new(),
            background: Color::TRANSPARENT,
            border: None,
        }
    }
    
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }
    
    pub fn add_child(&mut self, child: Box<dyn Widget>) {
        self.children.push(child);
    }
}

impl Default for Panel {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Panel {
    fn id(&self) -> ElementId { self.id }
    fn bounds(&self) -> Rect { self.bounds }
    fn set_bounds(&mut self, rect: Rect) { self.bounds = rect; }
    
    fn is_visible(&self) -> bool { self.visible }
    fn set_visible(&mut self, visible: bool) { self.visible = visible; }
    fn is_enabled(&self) -> bool { true }
    fn set_enabled(&mut self, _enabled: bool) {}
    
    fn can_focus(&self) -> bool { false }
    fn on_focus(&mut self) {}
    fn on_blur(&mut self) {}
    
    fn handle_event(&mut self, event: &UiEvent) -> bool {
        if !self.visible {
            return false;
        }
        
        for child in &mut self.children {
            if child.handle_event(event) {
                return true;
            }
        }
        false
    }
    
    fn render(&mut self, surface: &mut dyn Surface) {
        if !self.visible {
            return;
        }
        
        // Background
        if self.background.a > 0 {
            surface.fill_rect(self.bounds, self.background);
        }
        
        // Border
        if let Some(color) = self.border {
            surface.draw_rect(self.bounds, color, 1);
        }
        
        // Children
        for child in &mut self.children {
            child.render(surface);
        }
    }
}

/// ProgressBar widget
pub struct ProgressBar {
    id: ElementId,
    bounds: Rect,
    visible: bool,
    value: u32,  // 0-100
    max: u32,
    show_text: bool,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self {
            id: 0,
            bounds: Rect::new(0, 0, 200, 24),
            visible: true,
            value: 0,
            max: 100,
            show_text: true,
        }
    }
    
    pub fn set_value(&mut self, value: u32) {
        self.value = value.min(self.max);
    }
    
    pub fn value(&self) -> u32 {
        self.value
    }
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for ProgressBar {
    fn id(&self) -> ElementId { self.id }
    fn bounds(&self) -> Rect { self.bounds }
    fn set_bounds(&mut self, rect: Rect) { self.bounds = rect; }
    
    fn is_visible(&self) -> bool { self.visible }
    fn set_visible(&mut self, visible: bool) { self.visible = visible; }
    fn is_enabled(&self) -> bool { true }
    fn set_enabled(&mut self, _enabled: bool) {}
    
    fn can_focus(&self) -> bool { false }
    fn on_focus(&mut self) {}
    fn on_blur(&mut self) {}
    
    fn handle_event(&mut self, _event: &UiEvent) -> bool {
        false
    }
    
    fn render(&mut self, surface: &mut dyn Surface) {
        if !self.visible {
            return;
        }
        
        // Background
        surface.fill_rect(self.bounds, Color::LIGHT_GRAY);
        surface.draw_rect(self.bounds, Color::GRAY, 1);
        
        // Progress fill
        if self.value > 0 {
            let fill_width = (self.bounds.width as u32 * self.value / self.max) as u32;
            surface.fill_rect(
                Rect::new(self.bounds.x, self.bounds.y, fill_width, self.bounds.height),
                Color::ACCENT,
            );
        }
        
        // Text
        if self.show_text {
            use alloc::format;
            let text = format!("{}%", self.value * 100 / self.max);
            let (tw, th) = surface.measure_text(&text, 11);
            surface.draw_text_sized(
                self.bounds.x as u32 + (self.bounds.width - tw) / 2,
                self.bounds.y as u32 + (self.bounds.height - th) / 2,
                &text,
                11,
                Color::BLACK,
            );
        }
    }
}

/// Checkbox widget
pub struct Checkbox {
    id: ElementId,
    bounds: Rect,
    text: String,
    visible: bool,
    enabled: bool,
    checked: bool,
    on_change: Option<Box<dyn FnMut(bool)>>,
}

impl Checkbox {
    pub fn new(text: &str) -> Self {
        Self {
            id: 0,
            bounds: Rect::new(0, 0, 150, 24),
            text: String::from(text),
            visible: true,
            enabled: true,
            checked: false,
            on_change: None,
        }
    }
    
    pub fn checked(&self) -> bool {
        self.checked
    }
    
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
}

impl Widget for Checkbox {
    fn id(&self) -> ElementId { self.id }
    fn bounds(&self) -> Rect { self.bounds }
    fn set_bounds(&mut self, rect: Rect) { self.bounds = rect; }
    
    fn is_visible(&self) -> bool { self.visible }
    fn set_visible(&mut self, visible: bool) { self.visible = visible; }
    fn is_enabled(&self) -> bool { self.enabled }
    fn set_enabled(&mut self, enabled: bool) { self.enabled = enabled; }
    
    fn can_focus(&self) -> bool { true }
    fn on_focus(&mut self) {}
    fn on_blur(&mut self) {}
    
    fn handle_event(&mut self, event: &UiEvent) -> bool {
        if !self.visible || !self.enabled {
            return false;
        }
        
        match event {
            UiEvent::Click(pos, MouseButton::Left) => {
                if self.bounds.contains(*pos) {
                    self.checked = !self.checked;
                    if let Some(handler) = &mut self.on_change {
                        handler(self.checked);
                    }
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    }
    
    fn render(&mut self, surface: &mut dyn Surface) {
        if !self.visible {
            return;
        }
        
        // Checkbox box
        let box_rect = Rect::new(self.bounds.x, self.bounds.y, 18, 18);
        surface.fill_rect(box_rect, Color::WHITE);
        surface.draw_rect(box_rect, Color::GRAY, 1);
        
        // Checkmark
        if self.checked {
            surface.draw_line(
                box_rect.x + 4, box_rect.y + 9,
                box_rect.x + 7, box_rect.y + 13,
                Color::ACCENT, 2,
            );
            surface.draw_line(
                box_rect.x + 7, box_rect.y + 13,
                box_rect.x + 14, box_rect.y + 4,
                Color::ACCENT, 2,
            );
        }
        
        // Label
        surface.draw_text_sized(
            self.bounds.x as u32 + 24,
            self.bounds.y as u32 + 4,
            &self.text,
            12,
            if self.enabled { Color::BLACK } else { Color::GRAY },
        );
    }
}