// VGA Text Mode Driver - VantisOS
//
// This module implements VGA text mode driver for
// basic text output on VGA-compatible displays.

use core::ptr::write_volatile;

/// VGA buffer address
const VGA_BUFFER: usize = 0xB8000;

/// VGA width (columns)
const VGA_WIDTH: usize = 80;

/// VGA height (rows)
const VGA_HEIGHT: usize = 25;

/// VGA color codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

/// VGA color entry
#[derive(Debug, Clone, Copy)]
pub struct VgaColorEntry {
    pub foreground: VgaColor,
    pub background: VgaColor,
}

impl VgaColorEntry {
    /// Create a new color entry
    pub fn new(foreground: VgaColor, background: VgaColor) -> Self {
        Self {
            foreground,
            background,
        }
    }
    
    /// Convert to u8
    pub fn to_u8(&self) -> u8 {
        (self.background as u8) << 4 | (self.foreground as u8)
    }
}

/// VGA screen character
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct VgaScreenChar {
    pub ascii_character: u8,
    pub color_code: u8,
}

/// VGA text mode writer
pub struct VgaTextWriter {
    column_position: usize,
    row_position: usize,
    color_code: VgaColorEntry,
    buffer: *mut VgaScreenChar,
}

impl VgaTextWriter {
    /// Create a new VGA text writer
    pub fn new() -> Self {
        Self {
            column_position: 0,
            row_position: 0,
            color_code: VgaColorEntry::new(VgaColor::LightGray, VgaColor::Black),
            buffer: VGA_BUFFER as *mut VgaScreenChar,
        }
    }
    
    /// Write a byte to the screen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.column_position = 0,
            byte => {
                if self.column_position >= VGA_WIDTH {
                    self.new_line();
                }
                
                let offset = self.row_position * VGA_WIDTH + self.column_position;
                unsafe {
                    write_volatile(
                        self.buffer.add(offset),
                        VgaScreenChar {
                            ascii_character: byte,
                            color_code: self.color_code.to_u8(),
                        },
                    );
                }
                
                self.column_position += 1;
            }
        }
    }
    
    /// Write a string to the screen
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
    
    /// Clear the screen
    pub fn clear(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let offset = row * VGA_WIDTH + col;
                unsafe {
                    write_volatile(
                        self.buffer.add(offset),
                        VgaScreenChar {
                            ascii_character: b' ',
                            color_code: self.color_code.to_u8(),
                        },
                    );
                }
            }
        }
        self.column_position = 0;
        self.row_position = 0;
    }
    
    /// Set color
    pub fn set_color(&mut self, color: VgaColorEntry) {
        self.color_code = color;
    }
    
    /// Move to new line
    fn new_line(&mut self) {
        self.column_position = 0;
        self.row_position += 1;
        
        if self.row_position >= VGA_HEIGHT {
            self.scroll_up();
            self.row_position = VGA_HEIGHT - 1;
        }
    }
    
    /// Scroll up one line
    fn scroll_up(&mut self) {
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let src_offset = row * VGA_WIDTH + col;
                let dst_offset = (row - 1) * VGA_WIDTH + col;
                unsafe {
                    let ch = self.buffer.add(src_offset).read_volatile();
                    write_volatile(self.buffer.add(dst_offset), ch);
                }
            }
        }
        
        // Clear last row
        for col in 0..VGA_WIDTH {
            let offset = (VGA_HEIGHT - 1) * VGA_WIDTH + col;
            unsafe {
                write_volatile(
                    self.buffer.add(offset),
                    VgaScreenChar {
                        ascii_character: b' ',
                        color_code: self.color_code.to_u8(),
                    },
                );
            }
        }
    }
}

/// Initialize VGA text mode driver
pub fn init() {
    let mut writer = VgaTextWriter::new();
    writer.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vga_color_entry() {
        let color = VgaColorEntry::new(VgaColor::White, VgaColor::Black);
        assert_eq!(color.to_u8(), 0x0F);
    }
    
    #[test]
    fn test_vga_text_writer_creation() {
        let writer = VgaTextWriter::new();
        assert_eq!(writer.column_position, 0);
        assert_eq!(writer.row_position, 0);
    }
}