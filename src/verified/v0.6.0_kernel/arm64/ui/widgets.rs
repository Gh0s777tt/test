// Widget System for VantisOS v0.6.0 ARM64 kernel
// Touch UI Framework - Widgets

use super::framework::{
    UIElement, UIElementId, UIElementType, UIElementState, UIRect, UIColor, BaseUIElement,
    TouchEvent,
};

// Widget styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetStyle {
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
}

// Button widget
pub struct Button {
    base: BaseUIElement,
    text: [u8; 256],
    text_len: usize,
    style: WidgetStyle,
    corner_radius: u8,
}

impl Button {
    pub fn new(id: UIElementId, rect: UIRect, text: &str) -> Self {
        let mut button = Button {
            base: BaseUIElement::new(id, UIElementType::Button, rect),
            text: [0; 256],
            text_len: 0,
            style: WidgetStyle::Default,
            corner_radius: 8,
        };
        
        button.set_text(text);
        button
    }

    pub fn set_text(&mut self, text: &str) {
        self.text_len = text.len().min(255);
        for (i, byte) in text.bytes().enumerate().take(255) {
            self.text[i] = byte;
        }
    }

    pub fn get_text(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.text[..self.text_len])
        }
    }

    pub fn set_style(&mut self, style: WidgetStyle) {
        self.style = style;
    }

    pub fn get_style(&self) -> WidgetStyle {
        self.style
    }

    pub fn set_corner_radius(&mut self, radius: u8) {
        self.corner_radius = radius;
    }

    fn get_background_color(&self) -> UIColor {
        match self.style {
            WidgetStyle::Default => UIColor::rgb(200, 200, 200),
            WidgetStyle::Primary => UIColor::rgb(0, 122, 255),
            WidgetStyle::Secondary => UIColor::rgb(142, 142, 147),
            WidgetStyle::Success => UIColor::rgb(52, 199, 89),
            WidgetStyle::Warning => UIColor::rgb(255, 149, 0),
            WidgetStyle::Danger => UIColor::rgb(255, 59, 48),
        }
    }

    fn get_text_color(&self) -> UIColor {
        match self.style {
            WidgetStyle::Default => UIColor::black(),
            _ => UIColor::white(),
        }
    }
}

impl UIElement for Button {
    fn get_id(&self) -> UIElementId {
        self.base.get_id()
    }

    fn get_type(&self) -> UIElementType {
        self.base.get_type()
    }

    fn get_rect(&self) -> UIRect {
        self.base.get_rect()
    }

    fn get_state(&self) -> UIElementState {
        self.base.get_state()
    }

    fn set_rect(&mut self, rect: UIRect) {
        self.base.set_rect(rect);
    }

    fn set_state(&mut self, state: UIElementState) {
        self.base.set_state(state);
    }

    fn render(&self) {
        if !self.is_visible() {
            return;
        }

        let rect = self.get_rect();
        let bg_color = self.get_background_color();
        let text_color = self.get_text_color();

        // TODO: Implement actual rendering with GPU
        // 1. Draw rounded rectangle background
        // 2. Draw text centered in button
    }

    fn handle_touch_event(&mut self, event: &TouchEvent) {
        if !self.is_enabled() {
            return;
        }

        match event.event_type {
            super::touch_event::TouchEventType::Down => {
                self.set_state(UIElementState::Pressed);
            }
            super::touch_event::TouchEventType::Up => {
                self.set_state(UIElementState::Normal);
                // TODO: Trigger button click action
            }
            _ => {}
        }

        self.base.handle_touch_event(event);
    }
}

// Label widget
pub struct Label {
    base: BaseUIElement,
    text: [u8; 512],
    text_len: usize,
    text_color: UIColor,
    font_size: u16,
    alignment: TextAlignment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl Label {
    pub fn new(id: UIElementId, rect: UIRect, text: &str) -> Self {
        let mut label = Label {
            base: BaseUIElement::new(id, UIElementType::Label, rect),
            text: [0; 512],
            text_len: 0,
            text_color: UIColor::black(),
            font_size: 16,
            alignment: TextAlignment::Left,
        };
        
        label.set_text(text);
        label
    }

    pub fn set_text(&mut self, text: &str) {
        self.text_len = text.len().min(511);
        for (i, byte) in text.bytes().enumerate().take(511) {
            self.text[i] = byte;
        }
    }

    pub fn get_text(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.text[..self.text_len])
        }
    }

    pub fn set_text_color(&mut self, color: UIColor) {
        self.text_color = color;
    }

    pub fn get_text_color(&self) -> UIColor {
        self.text_color
    }

    pub fn set_font_size(&mut self, size: u16) {
        self.font_size = size;
    }

    pub fn get_font_size(&self) -> u16 {
        self.font_size
    }

    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        self.alignment = alignment;
    }

    pub fn get_alignment(&self) -> TextAlignment {
        self.alignment
    }
}

impl UIElement for Label {
    fn get_id(&self) -> UIElementId {
        self.base.get_id()
    }

    fn get_type(&self) -> UIElementType {
        self.base.get_type()
    }

    fn get_rect(&self) -> UIRect {
        self.base.get_rect()
    }

    fn get_state(&self) -> UIElementState {
        self.base.get_state()
    }

    fn set_rect(&mut self, rect: UIRect) {
        self.base.set_rect(rect);
    }

    fn set_state(&mut self, state: UIElementState) {
        self.base.set_state(state);
    }

    fn render(&self) {
        if !self.is_visible() {
            return;
        }

        // TODO: Implement actual rendering with GPU
        // 1. Draw text with specified color, font size, and alignment
    }

    fn handle_touch_event(&mut self, event: &TouchEvent) {
        // Labels don't handle touch events
    }
}

// TextField widget
pub struct TextField {
    base: BaseUIElement,
    text: [u8; 256],
    text_len: usize,
    placeholder: [u8; 128],
    placeholder_len: usize,
    text_color: UIColor,
    placeholder_color: UIColor,
    background_color: UIColor,
    font_size: u16,
    is_focused: bool,
    cursor_position: usize,
}

impl TextField {
    pub fn new(id: UIElementId, rect: UIRect, placeholder: &str) -> Self {
        let mut text_field = TextField {
            base: BaseUIElement::new(id, UIElementType::TextField, rect),
            text: [0; 256],
            text_len: 0,
            placeholder: [0; 128],
            placeholder_len: 0,
            text_color: UIColor::black(),
            placeholder_color: UIColor::rgb(150, 150, 150),
            background_color: UIColor::white(),
            font_size: 16,
            is_focused: false,
            cursor_position: 0,
        };
        
        text_field.set_placeholder(placeholder);
        text_field
    }

    pub fn set_text(&mut self, text: &str) {
        self.text_len = text.len().min(255);
        for (i, byte) in text.bytes().enumerate().take(255) {
            self.text[i] = byte;
        }
        self.cursor_position = self.text_len;
    }

    pub fn get_text(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.text[..self.text_len])
        }
    }

    pub fn set_placeholder(&mut self, placeholder: &str) {
        self.placeholder_len = placeholder.len().min(127);
        for (i, byte) in placeholder.bytes().enumerate().take(127) {
            self.placeholder[i] = byte;
        }
    }

    pub fn get_placeholder(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.placeholder[..self.placeholder_len])
        }
    }

    pub fn set_text_color(&mut self, color: UIColor) {
        self.text_color = color;
    }

    pub fn set_placeholder_color(&mut self, color: UIColor) {
        self.placeholder_color = color;
    }

    pub fn set_background_color(&mut self, color: UIColor) {
        self.background_color = color;
    }

    pub fn set_font_size(&mut self, size: u16) {
        self.font_size = size;
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn set_cursor_position(&mut self, position: usize) {
        self.cursor_position = position.min(self.text_len);
    }

    pub fn insert_char(&mut self, c: char) {
        if self.text_len < 255 {
            let bytes = c.to_string().into_bytes();
            for byte in bytes {
                if self.text_len < 255 {
                    self.text[self.text_len] = byte;
                    self.text_len += 1;
                }
            }
            self.cursor_position = self.text_len;
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.text_len = self.cursor_position;
            self.text[self.text_len] = 0;
        }
    }
}

impl UIElement for TextField {
    fn get_id(&self) -> UIElementId {
        self.base.get_id()
    }

    fn get_type(&self) -> UIElementType {
        self.base.get_type()
    }

    fn get_rect(&self) -> UIRect {
        self.base.get_rect()
    }

    fn get_state(&self) -> UIElementState {
        self.base.get_state()
    }

    fn set_rect(&mut self, rect: UIRect) {
        self.base.set_rect(rect);
    }

    fn set_state(&mut self, state: UIElementState) {
        self.base.set_state(state);
    }

    fn render(&self) {
        if !self.is_visible() {
            return;
        }

        // TODO: Implement actual rendering with GPU
        // 1. Draw background rectangle
        // 2. Draw text or placeholder
        // 3. Draw cursor if focused
    }

    fn handle_touch_event(&mut self, event: &TouchEvent) {
        if !self.is_enabled() {
            return;
        }

        match event.event_type {
            super::touch_event::TouchEventType::Down => {
                self.set_focused(true);
            }
            _ => {}
        }

        self.base.handle_touch_event(event);
    }
}

// Layout types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutType {
    Absolute,
    Flex,
    Grid,
}

// Layout manager
pub struct LayoutManager {
    layout_type: LayoutType,
}

impl LayoutManager {
    pub const fn new(layout_type: LayoutType) -> Self {
        LayoutManager { layout_type }
    }

    pub fn layout(&self, elements: &mut [&mut dyn UIElement], container: UIRect) {
        match self.layout_type {
            LayoutType::Absolute => {
                // Elements already have absolute positions
            }
            LayoutType::Flex => {
                self.layout_flex(elements, container);
            }
            LayoutType::Grid => {
                self.layout_grid(elements, container);
            }
        }
    }

    fn layout_flex(&self, elements: &mut [&mut dyn UIElement], container: UIRect) {
        // Simple horizontal flex layout
        let num_elements = elements.len();
        if num_elements == 0 {
            return;
        }

        let element_width = container.width / num_elements as u32;
        let mut x = container.x;

        for element in elements {
            let mut rect = element.get_rect();
            rect.x = x;
            rect.y = container.y;
            rect.width = element_width;
            rect.height = container.height;
            element.set_rect(rect);
            x += element_width as i32;
        }
    }

    fn layout_grid(&self, elements: &mut [&mut dyn UIElement], container: UIRect) {
        // Simple 2-column grid layout
        let num_elements = elements.len();
        if num_elements == 0 {
            return;
        }

        let cols = 2;
        let rows = (num_elements + cols - 1) / cols;
        let element_width = container.width / cols as u32;
        let element_height = container.height / rows as u32;

        for (i, element) in elements.iter_mut().enumerate() {
            let row = i / cols;
            let col = i % cols;

            let mut rect = element.get_rect();
            rect.x = container.x + (col * element_width) as i32;
            rect.y = container.y + (row * element_height) as i32;
            rect.width = element_width;
            rect.height = element_height;
            element.set_rect(rect);
        }
    }
}

// Widget styling
pub struct WidgetStyling {
    font_family: [u8; 64],
    font_family_len: usize,
    default_font_size: u16,
    default_text_color: UIColor,
    default_background_color: UIColor,
}

impl WidgetStyling {
    pub const fn new() -> Self {
        WidgetStyling {
            font_family: [0; 64],
            font_family_len: 0,
            default_font_size: 16,
            default_text_color: UIColor::black(),
            default_background_color: UIColor::white(),
        }
    }

    pub fn set_font_family(&mut self, family: &str) {
        self.font_family_len = family.len().min(63);
        for (i, byte) in family.bytes().enumerate().take(63) {
            self.font_family[i] = byte;
        }
    }

    pub fn get_font_family(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.font_family[..self.font_family_len])
        }
    }

    pub fn set_default_font_size(&mut self, size: u16) {
        self.default_font_size = size;
    }

    pub fn set_default_text_color(&mut self, color: UIColor) {
        self.default_text_color = color;
    }

    pub fn set_default_background_color(&mut self, color: UIColor) {
        self.default_background_color = color;
    }
}

impl Default for WidgetStyling {
    fn default() -> Self {
        Self::new()
    }
}
