// VantisOS v0.5.0 - Simple VGA Test
// Minimal kernel to test VGA output

#![no_std]
#![allow(dead_code)]

use core::panic::PanicInfo;
use core::arch::asm;

// VGA constants
const VGA_BUFFER: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// VGA colors
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
static mut CONSOLE: Option<SimpleConsole> = None;

struct SimpleConsole {
    buffer: *mut VgaChar,
    cursor_position: usize,
    color_code: VgaColorCode,
}

impl SimpleConsole {
    fn new() -> SimpleConsole {
        SimpleConsole {
            buffer: VGA_BUFFER as *mut VgaChar,
            cursor_position: 0,
            color_code: VgaColorCode::new(VgaColor::White, VgaColor::Black),
        }
    }

    fn init(&mut self) {
        self.clear_screen();
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

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                let current_line = self.cursor_position / VGA_WIDTH;
                if current_line < VGA_HEIGHT - 1 {
                    self.cursor_position = (current_line + 1) * VGA_WIDTH;
                } else {
                    self.scroll_up();
                    self.cursor_position = (VGA_HEIGHT - 1) * VGA_WIDTH;
                }
            }
            b'\r' => {
                self.cursor_position = (self.cursor_position / VGA_WIDTH) * VGA_WIDTH;
            }
            byte => {
                if self.cursor_position < VGA_WIDTH * VGA_HEIGHT {
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
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    fn scroll_up(&mut self) {
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let src = row * VGA_WIDTH + col;
                let dst = (row - 1) * VGA_WIDTH + col;
                unsafe {
                    *self.buffer.add(dst) = *self.buffer.add(src);
                }
            }
        }
        // Clear last line
        for col in 0..VGA_WIDTH {
            let pos = (VGA_HEIGHT - 1) * VGA_WIDTH + col;
            unsafe {
                *self.buffer.add(pos) = VgaChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };
            }
        }
    }
}

// Public functions
pub fn init() {
    unsafe {
        CONSOLE = Some(SimpleConsole::new());
        if let Some(console) = CONSOLE.as_mut() {
            console.init();
        }
    }
}

pub fn print(s: &str) {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.write_string(s);
        }
    }
}

// Multiboot header
#[repr(C, packed)]
struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

#[link_section = ".multiboot"]
#[no_mangle]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0x1BADB002,
    flags: 0,
    checksum: 0xE4524FFE, // -(0x1BADB002 + 0)
};

// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize VGA console
    init();
    
    // Print test message
    print("VantisOS v0.5.0 - VGA Test\n");
    print("========================\n");
    print("VGA Console Working!\n");
    print("This is a test message.\n");
    print("Colors: ");
    
    // Test colors
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            for i in 0..16 {
                console.color_code = VgaColorCode::new(VgaColor::Black, VgaColor::Black);
                console.write_byte(b'[');
                console.color_code = VgaColorCode::new(unsafe { core::mem::transmute(i as u8) }, VgaColor::Black);
                console.write_byte(b'X');
                console.color_code = VgaColorCode::new(VgaColor::White, VgaColor::Black);
                console.write_byte(b']');
                console.write_byte(b' ');
            }
        }
    }
    
    print("\n\nSystem ready.\n");
    
    // Halt
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("\n\nKERNEL PANIC!\n");
    print("System halted.\n");
    
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}