// VantisOS v0.5.0 - VGA Text Mode Console
// Day 8: Initialize Early Console
#![allow(unused_unsafe)]

// VGA constants
const VGA_BUFFER: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// VGA colors
#[allow(dead_code)]
#[repr(u8)]
enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Clone, Copy)]
#[repr(transparent)]
struct VgaColorCode(u8);

impl VgaColorCode {
    fn new(foreground: VgaColor, background: VgaColor) -> VgaColorCode {
        VgaColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct VgaChar {
    ascii_character: u8,
    color_code: VgaColorCode,
}

// Console state
static mut CONSOLE: Option<Console> = None;

struct Console {
    buffer: *mut VgaChar,
    cursor_position: usize,
    color_code: VgaColorCode,
    initialized: bool,
}

impl Console {
    fn new() -> Console {
        Console {
            buffer: VGA_BUFFER as *mut VgaChar,
            cursor_position: 0,
            color_code: VgaColorCode::new(VgaColor::White, VgaColor::Black),
            initialized: false,
        }
    }

    fn init(&mut self) {
        self.clear_screen();
        self.initialized = true;
    }

    fn clear_screen(&mut self) {
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            unsafe {
                *self.buffer.add(i) = VgaChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };
            }
        }
        self.cursor_position = 0;
    }

    fn set_color(&mut self, foreground: VgaColor, background: VgaColor) {
        self.color_code = VgaColorCode::new(foreground, background);
    }

    fn set_cursor(&mut self, position: usize) {
        self.cursor_position = position;
    }

    fn new_line(&mut self) {
        let current_line = self.cursor_position / VGA_WIDTH;
        if current_line < VGA_HEIGHT - 1 {
            self.cursor_position = (current_line + 1) * VGA_WIDTH;
        } else {
            self.scroll_up();
            self.cursor_position = (VGA_HEIGHT - 1) * VGA_WIDTH;
        }
    }

    fn scroll_up(&mut self) {
        for row in 0..(VGA_HEIGHT - 1) {
            for col in 0..VGA_WIDTH {
                let src = (row + 1) * VGA_WIDTH + col;
                let dst = row * VGA_WIDTH + col;
                unsafe {
                    *self.buffer.add(dst) = *self.buffer.add(src);
                }
            }
        }
        
        // Clear the last line
        for col in 0..VGA_WIDTH {
            let dst = (VGA_HEIGHT - 1) * VGA_WIDTH + col;
            unsafe {
                *self.buffer.add(dst) = VgaChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => {
                // Carriage return - move to beginning of current line
                let current_line = self.cursor_position / VGA_WIDTH;
                self.cursor_position = current_line * VGA_WIDTH;
            }
            b'\t' => {
                // Tab - move to next tab stop (every 8 characters)
                let current_col = self.cursor_position % VGA_WIDTH;
                let spaces = 8 - (current_col % 8);
                for _ in 0..spaces {
                    self.write_byte(b' ');
                }
            }
            0x08 => {
                // Backspace
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    unsafe {
                        *self.buffer.add(self.cursor_position) = VgaChar {
                            ascii_character: b' ',
                            color_code: self.color_code,
                        };
                    }
                }
            }
            byte => {
                if self.cursor_position >= VGA_WIDTH * VGA_HEIGHT {
                    self.new_line();
                }
                
                unsafe {
                    *self.buffer.add(self.cursor_position) = VgaChar {
                        ascii_character: byte,
                        color_code: self.color_code,
                    };
                }
                self.cursor_position += 1;
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            unsafe {
                self.write_byte(*bytes.as_ptr().add(i));
            }
        }
    }

    fn write_dec(&mut self, mut value: u64) {
        if value == 0 {
            self.write_byte(b'0');
            return;
        }

        let mut buffer = [0u8; 20];
        let mut pos = 0usize;

        while value > 0 {
            unsafe {
                *buffer.as_mut_ptr().add(pos) = (value % 10) as u8 + b'0';
            }
            value /= 10;
            pos += 1;
        }

        for i in (0..pos).rev() {
            unsafe {
                self.write_byte(*buffer.as_ptr().add(i));
            }
        }
    }

    fn write_hex(&mut self, value: u64) {
        self.write_string("0x");
        
        let hex_chars = b"0123456789ABCDEF";
        
        for i in 0..16 {
            let shift = (15 - i) * 4;
            let digit = (value >> shift) & 0xF;
            unsafe {
                self.write_byte(*hex_chars.as_ptr().add(digit as usize));
            }
        }
    }

    fn write_hex32(&mut self, value: u32) {
        self.write_string("0x");
        
        let hex_chars = b"0123456789ABCDEF";
        
        for i in 0..8 {
            let shift = (7 - i) * 4;
            let digit = (value >> shift) & 0xF;
            unsafe {
                self.write_byte(*hex_chars.as_ptr().add(digit as usize));
            }
        }
    }

    fn write_bool(&mut self, value: bool) {
        let s = if value { "true" } else { "false" };
        let bytes = s.as_bytes();
        for i in 0..bytes.len() {
            unsafe {
                self.write_byte(*bytes.as_ptr().add(i));
            }
        }
    }
}

// Public API
pub fn init() {
    unsafe {
        if CONSOLE.is_none() {
            CONSOLE = Some(Console::new());
        }
        CONSOLE.as_mut().unwrap().init();
    }
}

pub fn set_color(foreground: VgaColor, background: VgaColor) {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.set_color(foreground, background);
        }
    }
}

pub fn clear_screen() {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.clear_screen();
        }
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.write_byte(byte);
        }
    }
}

pub fn write_string(s: &str) {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.write_string(s);
        }
    }
}

pub fn write_dec(value: u64) {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.write_dec(value);
        }
    }
}

pub fn write_hex(value: u64) {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.write_hex(value);
        }
    }
}

pub fn write_hex32(value: u32) {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.write_hex32(value);
        }
    }
}

pub fn write_bool(value: bool) {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.write_bool(value);
        }
    }
}

