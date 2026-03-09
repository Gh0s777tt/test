// Graphics Primitives - VantisOS
//
// This module implements basic graphics primitives for
// drawing on framebuffer displays.

use alloc::vec::Vec;

/// Point
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Create a new point
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    /// Create a new size
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// Rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    /// Create a new rectangle
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
    
    /// Check if point is inside rectangle
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x &&
        point.x < self.x + self.width as i32 &&
        point.y >= self.y &&
        point.y < self.y + self.height as i32
    }
    
    /// Get intersection with another rectangle
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width as i32).min(other.x + other.width as i32);
        let y2 = (self.y + self.height as i32).min(other.y + other.height as i32);
        
        if x2 > x1 && y2 > y1 {
            Some(Rect::new(
                x1,
                y1,
                (x2 - x1) as u32,
                (y2 - y1) as u32,
            ))
        } else {
            None
        }
    }
}

/// Color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    /// Create a new color
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    /// Create RGB color (alpha = 255)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }
    
    /// Convert to u32 (ARGB)
    pub fn to_argb(&self) -> u32 {
        ((self.a as u32) << 24) |
        ((self.r as u32) << 16) |
        ((self.g as u32) << 8) |
        (self.b as u32)
    }
    
    /// Convert to u32 (RGB)
    pub fn to_rgb(&self) -> u32 {
        ((self.r as u32) << 16) |
        ((self.g as u32) << 8) |
        (self.b as u32)
    }
    
    /// Common colors
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
}

/// Graphics context
pub struct GraphicsContext {
    framebuffer: *mut u8,
    width: u32,
    height: u32,
    pitch: u32,
    bits_per_pixel: u8,
}

impl GraphicsContext {
    /// Create a new graphics context
    pub fn new(
        framebuffer: *mut u8,
        width: u32,
        height: u32,
        pitch: u32,
        bits_per_pixel: u8,
    ) -> Self {
        Self {
            framebuffer,
            width,
            height,
            pitch,
            bits_per_pixel,
        }
    }
    
    /// Get width
    pub fn get_width(&self) -> u32 {
        self.width
    }
    
    /// Get height
    pub fn get_height(&self) -> u32 {
        self.height
    }
    
    /// Put pixel
    pub fn put_pixel(&self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let offset = (y as usize * self.pitch as usize + x as usize * ((self.bits_per_pixel as usize + 7) / 8));
            unsafe {
                let pixel = self.framebuffer.add(offset);
                
                match self.bits_per_pixel {
                    32 => {
                        *(pixel as *mut u32) = color.to_argb();
                    }
                    24 => {
                        *(pixel.add(0)) = color.b;
                        *(pixel.add(1)) = color.g;
                        *(pixel.add(2)) = color.r;
                    }
                    16 => {
                        let r = (color.r as u16 >> 3) & 0x1F;
                        let g = (color.g as u16 >> 2) & 0x3F;
                        let b = (color.b as u16 >> 3) & 0x1F;
                        *(pixel as *mut u16) = (r << 11) | (g << 5) | b;
                    }
                    _ => {}
                }
            }
        }
    }
    
    /// Draw line (Bresenham's algorithm)
    pub fn draw_line(&self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut x = x0;
        let mut y = y0;
        
        loop {
            self.put_pixel(x, y, color);
            
            if x == x1 && y == y1 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
    
    /// Draw rectangle
    pub fn draw_rect(&self, rect: Rect, color: Color) {
        // Top edge
        for x in rect.x..rect.x + rect.width as i32 {
            self.put_pixel(x, rect.y, color);
        }
        
        // Bottom edge
        for x in rect.x..rect.x + rect.width as i32 {
            self.put_pixel(x, rect.y + rect.height as i32 - 1, color);
        }
        
        // Left edge
        for y in rect.y..rect.y + rect.height as i32 {
            self.put_pixel(rect.x, y, color);
        }
        
        // Right edge
        for y in rect.y..rect.y + rect.height as i32 {
            self.put_pixel(rect.x + rect.width as i32 - 1, y, color);
        }
    }
    
    /// Fill rectangle
    pub fn fill_rect(&self, rect: Rect, color: Color) {
        for y in rect.y..rect.y + rect.height as i32 {
            for x in rect.x..rect.x + rect.width as i32 {
                self.put_pixel(x, y, color);
            }
        }
    }
    
    /// Draw circle (midpoint circle algorithm)
    pub fn draw_circle(&self, cx: i32, cy: i32, radius: i32, color: Color) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 3 - 2 * radius;
        
        while y >= x {
            self.put_pixel(cx + x, cy + y, color);
            self.put_pixel(cx - x, cy + y, color);
            self.put_pixel(cx + x, cy - y, color);
            self.put_pixel(cx - x, cy - y, color);
            self.put_pixel(cx + y, cy + x, color);
            self.put_pixel(cx - y, cy + x, color);
            self.put_pixel(cx + y, cy - x, color);
            self.put_pixel(cx - y, cy - x, color);
            
            x += 1;
            if d > 0 {
                y -= 1;
                d += 4 * (x - y) + 10;
            } else {
                d += 4 * x + 6;
            }
        }
    }
    
    /// Fill circle
    pub fn fill_circle(&self, cx: i32, cy: i32, radius: i32, color: Color) {
        let mut y = radius;
        let mut x = 0;
        let mut d = 3 - 2 * radius;
        
        while y >= x {
            self.draw_line(cx - x, cy + y, cx + x, cy + y, color);
            self.draw_line(cx - x, cy - y, cx + x, cy - y, color);
            self.draw_line(cx - y, cy + x, cx + y, cy + x, color);
            self.draw_line(cx - y, cy - x, cx + y, cy - x, color);
            
            x += 1;
            if d > 0 {
                y -= 1;
                d += 4 * (x - y) + 10;
            } else {
                d += 4 * x + 6;
            }
        }
    }
    
    /// Clear screen
    pub fn clear(&self, color: Color) {
        self.fill_rect(Rect::new(0, 0, self.width, self.height), color);
    }
}

/// Initialize graphics primitives
pub fn init() {
    // TODO: Initialize graphics primitives
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point() {
        let point = Point::new(100, 200);
        assert_eq!(point.x, 100);
        assert_eq!(point.y, 200);
    }
    
    #[test]
    fn test_size() {
        let size = Size::new(800, 600);
        assert_eq!(size.width, 800);
        assert_eq!(size.height, 600);
    }
    
    #[test]
    fn test_rect() {
        let rect = Rect::new(10, 20, 100, 50);
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.width, 100);
        assert_eq!(rect.height, 50);
    }
    
    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10, 20, 100, 50);
        let point = Point::new(50, 30);
        assert!(rect.contains(point));
    }
    
    #[test]
    fn test_color() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color.to_rgb(), 0xFF8040);
    }
    
    #[test]
    fn test_graphics_context_creation() {
        let ctx = GraphicsContext::new(0xE0000000 as *mut u8, 1024, 768, 4096, 32);
        assert_eq!(ctx.get_width(), 1024);
        assert_eq!(ctx.get_height(), 768);
    }
}