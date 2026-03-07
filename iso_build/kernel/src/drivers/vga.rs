//! VGA text mode driver for VantisOS

use spin::Mutex;
use volatile::Volatile;
use lazy_static::lazy_static;

/// VGA buffer address
const VGA_BUFFER: *mut u8 = 0xB8000 as *mut u8;

/// Screen dimensions
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// VGA colors
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

/// VGA character
#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

/// VGA Writer
pub struct Writer {
    column_position: usize,
    color_code: u8,
    buffer: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Writer {
    /// Write a byte
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                
                self.buffer[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                
                self.column_position += 1;
            }
        }
    }
    
    /// Write a string
    pub fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
        Ok(())
    }
    
    /// New line
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer[row][col];
                self.buffer[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    
    /// Clear a row
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer[row][col] = blank;
        }
    }
    
    /// Set color
    pub fn set_color(&mut self, foreground: Color, background: Color) {
        self.color_code = (background as u8) << 4 | (foreground as u8);
    }
    
    /// Clear screen
    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.column_position = 0;
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str(s)
    }
}

/// Global VGA writer
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: Color::White as u8,
        buffer: [[ScreenChar {
            ascii_character: b' ',
            color_code: Color::White as u8,
        }; BUFFER_WIDTH]; BUFFER_HEIGHT],
    });
}

/// Initialize VGA
pub fn init() {
    WRITER.lock().clear_screen();
}

/// Print macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}