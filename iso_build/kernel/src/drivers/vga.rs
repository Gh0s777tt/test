//! VGA text mode driver for VantisOS
//! 
//! This module provides:
//! - VGA text mode (80x25) output
//! - Color support
//! - Cursor management
//! - Scrolling

use core::fmt;
use core::ptr;
use spin::Mutex;

use crate::serial_println;

/// VGA buffer address
const VGA_BUFFER: *mut VgaBuffer = 0xB8000 as *mut VgaBuffer;

/// Screen width (columns)
pub const SCREEN_WIDTH: usize = 80;

/// Screen height (rows)
pub const SCREEN_HEIGHT: usize = 25;

/// VGA Color enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
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
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl Color {
    /// Convert to RGB tuple (approximate)
    pub fn to_rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Black => (0, 0, 0),
            Color::Blue => (0, 0, 170),
            Color::Green => (0, 170, 0),
            Color::Cyan => (0, 170, 170),
            Color::Red => (170, 0, 0),
            Color::Magenta => (170, 0, 170),
            Color::Brown => (170, 85, 0),
            Color::LightGray => (170, 170, 170),
            Color::DarkGray => (85, 85, 85),
            Color::LightBlue => (85, 85, 255),
            Color::LightGreen => (85, 255, 85),
            Color::LightCyan => (85, 255, 255),
            Color::LightRed => (255, 85, 85),
            Color::Pink => (255, 85, 255),
            Color::Yellow => (255, 255, 85),
            Color::White => (255, 255, 255),
        }
    }
}

impl TryFrom<u8> for Color {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::Blue),
            2 => Ok(Color::Green),
            3 => Ok(Color::Cyan),
            4 => Ok(Color::Red),
            5 => Ok(Color::Magenta),
            6 => Ok(Color::Brown),
            7 => Ok(Color::LightGray),
            8 => Ok(Color::DarkGray),
            9 => Ok(Color::LightBlue),
            10 => Ok(Color::LightGreen),
            11 => Ok(Color::LightCyan),
            12 => Ok(Color::LightRed),
            13 => Ok(Color::Pink),
            14 => Ok(Color::Yellow),
            15 => Ok(Color::White),
            _ => Err(value),
        }
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        color as u8
    }
}

/// Color code (foreground + background)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    /// Create new color code
    pub const fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
    
    /// Get foreground color
    pub fn foreground(self) -> Color {
        Color::try_from(self.0 & 0x0F).unwrap_or(Color::White)
    }
    
    /// Get background color
    pub fn background(self) -> Color {
        Color::try_from((self.0 >> 4) & 0x0F).unwrap_or(Color::Black)
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        ColorCode::new(Color::White, Color::Black)
    }
}

/// VGA screen character
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    /// ASCII character
    pub ascii_char: u8,
    /// Color code
    pub color_code: ColorCode,
}

impl Default for ScreenChar {
    fn default() -> Self {
        ScreenChar {
            ascii_char: b' ',
            color_code: ColorCode::default(),
        }
    }
}

/// VGA buffer type
type VgaBuffer = [[ScreenChar; SCREEN_WIDTH]; SCREEN_HEIGHT];

/// VGA Writer
pub struct Writer {
    /// Column position
    column_position: usize,
    /// Current color code
    color_code: ColorCode,
    /// VGA buffer reference
    buffer: &'static mut VgaBuffer,
}

impl Writer {
    /// Create a new writer
    pub fn new() -> Self {
        Writer {
            column_position: 0,
            color_code: ColorCode::default(),
            buffer: unsafe { &mut *VGA_BUFFER },
        }
    }
    
    /// Write a byte
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // Newline
            b'\n' => self.new_line(),
            // Carriage return
            b'\r' => self.column_position = 0,
            // Tab
            b'\t' => {
                let spaces = 4 - (self.column_position % 4);
                for _ in 0..spaces {
                    self.write_char(' ');
                }
            }
            // Backspace
            b'\x08' => {
                if self.column_position > 0 {
                    self.column_position -= 1;
                    self.write_char_at(' ', self.column_position, SCREEN_HEIGHT - 1);
                }
            }
            // Printable ASCII
            0x20..=0x7E => {
                if self.column_position >= SCREEN_WIDTH {
                    self.new_line();
                }
                
                self.write_char_at(byte as char, self.column_position, SCREEN_HEIGHT - 1);
                self.column_position += 1;
            }
            // Non-printable
            _ => {
                self.write_byte(b'?');
            }
        }
        
        self.update_cursor();
    }
    
    /// Write a character at specific position
    fn write_char_at(&mut self, c: char, col: usize, row: usize) {
        let char = ScreenChar {
            ascii_char: if c.is_ascii() { c as u8 } else { b'?' },
            color_code: self.color_code,
        };
        
        unsafe {
            ptr::write_volatile(&mut self.buffer[row][col], char);
        }
    }
    
    /// Write a character (handles position)
    fn write_char(&mut self, c: char) {
        if self.column_position >= SCREEN_WIDTH {
            self.new_line();
        }
        
        self.write_char_at(c, self.column_position, SCREEN_HEIGHT - 1);
        self.column_position += 1;
    }
    
    /// Write a string
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
    
    /// Set color
    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background);
    }
    
    /// Get current color
    pub fn get_color(&self) -> (Color, Color) {
        (self.color_code.foreground(), self.color_code.background())
    }
    
    /// New line
    pub fn new_line(&mut self) {
        // Scroll up
        for row in 1..SCREEN_HEIGHT {
            for col in 0..SCREEN_WIDTH {
                let character = unsafe { ptr::read_volatile(&self.buffer[row][col]) };
                unsafe { ptr::write_volatile(&mut self.buffer[row - 1][col], character); }
            }
        }
        
        // Clear last row
        self.clear_row(SCREEN_HEIGHT - 1);
        self.column_position = 0;
    }
    
    /// Clear a row
    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };
        
        for col in 0..SCREEN_WIDTH {
            unsafe { ptr::write_volatile(&mut self.buffer[row][col], blank); }
        }
    }
    
    /// Clear screen
    pub fn clear_screen(&mut self) {
        for row in 0..SCREEN_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
        self.update_cursor();
    }
    
    /// Update hardware cursor
    pub fn update_cursor(&mut self) {
        let pos = (SCREEN_HEIGHT - 1) * SCREEN_WIDTH + self.column_position;
        
        unsafe {
            // Cursor LOW port
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D4u16,
                in("al") 0x0Fu8,
                options(nostack, nomem)
            );
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D5u16,
                in("al") (pos & 0xFF) as u8,
                options(nostack, nomem)
            );
            
            // Cursor HIGH port
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D4u16,
                in("al") 0x0Eu8,
                options(nostack, nomem)
            );
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D5u16,
                in("al") ((pos >> 8) & 0xFF) as u8,
                options(nostack, nomem)
            );
        }
    }
    
    /// Hide cursor
    pub fn hide_cursor(&mut self) {
        unsafe {
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D4u16,
                in("al") 0x0Au8,
                options(nostack, nomem)
            );
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D5u16,
                in("al") 0x20u8,
                options(nostack, nomem)
            );
        }
    }
    
    /// Show cursor
    pub fn show_cursor(&mut self) {
        unsafe {
            // Set cursor start
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D4u16,
                in("al") 0x0Au8,
                options(nostack, nomem)
            );
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D5u16,
                in("al") 0x0Bu8,
                options(nostack, nomem)
            );
            // Set cursor end
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D4u16,
                in("al") 0x0Bu8,
                options(nostack, nomem)
            );
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x3D5u16,
                in("al") 0x0Cu8,
                options(nostack, nomem)
            );
        }
        self.update_cursor();
    }
    
    /// Set cursor position
    pub fn set_cursor(&mut self, row: usize, col: usize) {
        if row < SCREEN_HEIGHT && col < SCREEN_WIDTH {
            self.column_position = col;
            self.update_cursor();
        }
    }
    
    /// Get current position
    pub fn position(&self) -> (usize, usize) {
        (SCREEN_HEIGHT - 1, self.column_position)
    }
    
    /// Draw a box
    pub fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize) {
        // Top border
        self.write_char_at('┌', x, y);
        for i in 1..width-1 {
            self.write_char_at('─', x + i, y);
        }
        self.write_char_at('┐', x + width - 1, y);
        
        // Sides
        for row in 1..height-1 {
            self.write_char_at('│', x, y + row);
            self.write_char_at('│', x + width - 1, y + row);
        }
        
        // Bottom border
        self.write_char_at('└', x, y + height - 1);
        for i in 1..width-1 {
            self.write_char_at('─', x + i, y + height - 1);
        }
        self.write_char_at('┘', x + width - 1, y + height - 1);
    }
    
    /// Print at position
    pub fn print_at(&mut self, x: usize, y: usize, s: &str) {
        for (i, c) in s.chars().enumerate() {
            if x + i < SCREEN_WIDTH && y < SCREEN_HEIGHT {
                self.write_char_at(c, x + i, y);
            }
        }
    }
}

impl Default for Writer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Global VGA writer
lazy_static::lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *VGA_BUFFER },
    });
}

/// Initialize VGA
pub fn init() {
    let mut writer = WRITER.lock();
    writer.clear_screen();
    writer.show_cursor();
    
    serial_println!("[OK] VGA driver initialized");
}

/// Like print! but for VGA
#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::drivers::vga::_print(format_args!($($arg)*)));
}

/// Like println! but for VGA
#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_print!("\n"));
    ($fmt:expr) => ($crate::vga_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::vga_print!(concat!($fmt, "\n"), $($arg)*));
}

/// Internal print function for macro
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}