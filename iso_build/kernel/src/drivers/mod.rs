//! Device drivers for VantisOS
//! 
//! This module provides:
//! - VGA text mode driver
//! - Keyboard driver (PS/2)
//! - Mouse driver (PS/2)
//! - Timer driver
//! - Serial port driver

use crate::serial_println;
use crate::serial_print;

pub mod vga;

/// Initialize all drivers
pub fn init() {
    vga::init();
    serial_println!("[OK] Drivers initialized");
}

// ========== Keyboard Driver ==========

/// Keyboard scancode set 1 (US layout)
pub mod keyboard {
    use spin::Mutex;
    use alloc::collections::VecDeque;
    use crate::serial_println;
    
    extern crate alloc;
    
    /// Keyboard data port
    const KBD_DATA: u16 = 0x60;
    /// Keyboard status port
    const KBD_STATUS: u16 = 0x64;
    /// Keyboard command port
    const KBD_CMD: u16 = 0x64;
    
    /// Key state
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum KeyState {
        Released,
        Pressed,
    }
    
    /// Special keys
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum SpecialKey {
        Escape,
        Backspace,
        Tab,
        Enter,
        Control,
        LeftShift,
        RightShift,
        LeftAlt,
        CapsLock,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        ScrollLock,
        NumLock,
        PrintScreen,
        Pause,
        Insert,
        Home,
        PageUp,
        Delete,
        End,
        PageDown,
        Up,
        Down,
        Left,
        Right,
        RightAlt,
        RightControl,
    }
    
    /// Key event
    #[derive(Debug, Clone, Copy)]
    pub struct KeyEvent {
        pub scancode: u8,
        pub key: Option<char>,
        pub special: Option<SpecialKey>,
        pub state: KeyState,
        pub shift: bool,
        pub ctrl: bool,
        pub alt: bool,
    }
    
    /// Keyboard state
    pub struct KeyboardState {
        pub shift: bool,
        pub ctrl: bool,
        pub alt: bool,
        pub caps_lock: bool,
        pub num_lock: bool,
        pub scroll_lock: bool,
    }
    
    impl Default for KeyboardState {
        fn default() -> Self {
            KeyboardState {
                shift: false,
                ctrl: false,
                alt: false,
                caps_lock: false,
                num_lock: false,
                scroll_lock: false,
            }
        }
    }
    
    /// Global keyboard state
    pub static KEYBOARD_STATE: Mutex<KeyboardState> = Mutex::new(KeyboardState {
        shift: false,
        ctrl: false,
        alt: false,
        caps_lock: false,
        num_lock: false,
        scroll_lock: false,
    });
    
    /// Key buffer
    pub static KEY_BUFFER: Mutex<VecDeque<KeyEvent>> = Mutex::new(VecDeque::new());
    
    /// US QWERTY scancode table (set 1)
    const SCANCODE_TABLE: [Option<char>; 131] = [
        None,                           // 0x00
        Some('\x1B'),                   // 0x01 - Escape
        Some('1'),                      // 0x02
        Some('2'),                      // 0x03
        Some('3'),                      // 0x04
        Some('4'),                      // 0x05
        Some('5'),                      // 0x06
        Some('6'),                      // 0x07
        Some('7'),                      // 0x08
        Some('8'),                      // 0x09
        Some('9'),                      // 0x0A
        Some('0'),                      // 0x0B
        Some('-'),                      // 0x0C
        Some('='),                      // 0x0D
        Some('\x08'),                   // 0x0E - Backspace
        Some('\t'),                     // 0x0F - Tab
        Some('q'),                      // 0x10
        Some('w'),                      // 0x11
        Some('e'),                      // 0x12
        Some('r'),                      // 0x13
        Some('t'),                      // 0x14
        Some('y'),                      // 0x15
        Some('u'),                      // 0x16
        Some('i'),                      // 0x17
        Some('o'),                      // 0x18
        Some('p'),                      // 0x19
        Some('['),                      // 0x1A
        Some(']'),                      // 0x1B
        Some('\n'),                     // 0x1C - Enter
        None,                           // 0x1D - Control
        Some('a'),                      // 0x1E
        Some('s'),                      // 0x1F
        Some('d'),                      // 0x20
        Some('f'),                      // 0x21
        Some('g'),                      // 0x22
        Some('h'),                      // 0x23
        Some('j'),                      // 0x24
        Some('k'),                      // 0x25
        Some('l'),                      // 0x26
        Some(';'),                      // 0x27
        Some('\''),                     // 0x28
        Some('`'),                      // 0x29
        None,                           // 0x2A - Left Shift
        Some('\\'),                     // 0x2B
        Some('z'),                      // 0x2C
        Some('x'),                      // 0x2D
        Some('c'),                      // 0x2E
        Some('v'),                      // 0x2F
        Some('b'),                      // 0x30
        Some('n'),                      // 0x31
        Some('m'),                      // 0x32
        Some(','),                      // 0x33
        Some('.'),                      // 0x34
        Some('/'),                      // 0x35
        None,                           // 0x36 - Right Shift
        Some('*'),                      // 0x37 - Keypad *
        None,                           // 0x38 - Left Alt
        Some(' '),                      // 0x39 - Space
        None,                           // 0x3A - CapsLock
        None,                           // 0x3B - F1
        None,                           // 0x3C - F2
        None,                           // 0x3D - F3
        None,                           // 0x3E - F4
        None,                           // 0x3F - F5
        None,                           // 0x40 - F6
        None,                           // 0x41 - F7
        None,                           // 0x42 - F8
        None,                           // 0x43 - F9
        None,                           // 0x44 - F10
        None,                           // 0x45 - NumLock
        None,                           // 0x46 - ScrollLock
        None,                           // 0x47 - Keypad 7/Home
        None,                           // 0x48 - Keypad 8/Up
        None,                           // 0x49 - Keypad 9/PgUp
        Some('-'),                      // 0x4A - Keypad -
        None,                           // 0x4B - Keypad 4/Left
        None,                           // 0x4C - Keypad 5
        None,                           // 0x4D - Keypad 6/Right
        Some('+'),                      // 0x4E - Keypad +
        None,                           // 0x4F - Keypad 1/End
        None,                           // 0x50 - Keypad 2/Down
        None,                           // 0x51 - Keypad 3/PgDn
        None,                           // 0x52 - Keypad 0/Ins
        None,                           // 0x53 - Keypad ./Del
        None, None, None,               // 0x54-0x56 - Reserved
        None,                           // 0x57 - F11
        None,                           // 0x58 - F12
        // Rest are undefined
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
    ];
    
    /// Shifted scancode table
    const SCANCODE_TABLE_SHIFT: [Option<char>; 123] = [
        None,                           // 0x00
        Some('\x1B'),                   // 0x01 - Escape
        Some('!'),                      // 0x02
        Some('@'),                      // 0x03
        Some('#'),                      // 0x04
        Some('$'),                      // 0x05
        Some('%'),                      // 0x06
        Some('^'),                      // 0x07
        Some('&'),                      // 0x08
        Some('*'),                      // 0x09
        Some('('),                      // 0x0A
        Some(')'),                      // 0x0B
        Some('_'),                      // 0x0C
        Some('+'),                      // 0x0D
        Some('\x08'),                   // 0x0E - Backspace
        Some('\t'),                     // 0x0F - Tab
        Some('Q'),                      // 0x10
        Some('W'),                      // 0x11
        Some('E'),                      // 0x12
        Some('R'),                      // 0x13
        Some('T'),                      // 0x14
        Some('Y'),                      // 0x15
        Some('U'),                      // 0x16
        Some('I'),                      // 0x17
        Some('O'),                      // 0x18
        Some('P'),                      // 0x19
        Some('{'),                      // 0x1A
        Some('}'),                      // 0x1B
        Some('\n'),                     // 0x1C - Enter
        None,                           // 0x1D - Control
        Some('A'),                      // 0x1E
        Some('S'),                      // 0x1F
        Some('D'),                      // 0x20
        Some('F'),                      // 0x21
        Some('G'),                      // 0x22
        Some('H'),                      // 0x23
        Some('J'),                      // 0x24
        Some('K'),                      // 0x25
        Some('L'),                      // 0x26
        Some(':'),                      // 0x27
        Some('"'),                      // 0x28
        Some('~'),                      // 0x29
        None,                           // 0x2A - Left Shift
        Some('|'),                      // 0x2B
        Some('Z'),                      // 0x2C
        Some('X'),                      // 0x2D
        Some('C'),                      // 0x2E
        Some('V'),                      // 0x2F
        Some('B'),                      // 0x30
        Some('N'),                      // 0x31
        Some('M'),                      // 0x32
        Some('<'),                      // 0x33
        Some('>'),                      // 0x34
        Some('?'),                      // 0x35
        None,                           // 0x36 - Right Shift
        Some('*'),                      // 0x37 - Keypad *
        None,                           // 0x38 - Left Alt
        Some(' '),                      // 0x39 - Space
        None,                           // 0x3A - CapsLock
        None, None, None, None, None, None, None, None, None, None, // F1-F10
        None,                           // NumLock
        None,                           // ScrollLock
        None, None, None, None, None, None, None, None, None, None, // Keypad
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None,
    ];
    
    /// Get special key from scancode
    fn get_special_key(scancode: u8) -> Option<SpecialKey> {
        match scancode {
            0x01 => Some(SpecialKey::Escape),
            0x1D => Some(SpecialKey::Control),
            0x2A => Some(SpecialKey::LeftShift),
            0x36 => Some(SpecialKey::RightShift),
            0x38 => Some(SpecialKey::LeftAlt),
            0x3A => Some(SpecialKey::CapsLock),
            0x3B..=0x44 => Some(match scancode {
                0x3B => SpecialKey::F1,
                0x3C => SpecialKey::F2,
                0x3D => SpecialKey::F3,
                0x3E => SpecialKey::F4,
                0x3F => SpecialKey::F5,
                0x40 => SpecialKey::F6,
                0x41 => SpecialKey::F7,
                0x42 => SpecialKey::F8,
                0x43 => SpecialKey::F9,
                0x44 => SpecialKey::F10,
                _ => unreachable!(),
            }),
            0x45 => Some(SpecialKey::NumLock),
            0x46 => Some(SpecialKey::ScrollLock),
            0x47 => Some(SpecialKey::Home),
            0x48 => Some(SpecialKey::Up),
            0x49 => Some(SpecialKey::PageUp),
            0x4B => Some(SpecialKey::Left),
            0x4D => Some(SpecialKey::Right),
            0x4F => Some(SpecialKey::End),
            0x50 => Some(SpecialKey::Down),
            0x51 => Some(SpecialKey::PageDown),
            0x52 => Some(SpecialKey::Insert),
            0x53 => Some(SpecialKey::Delete),
            0x57 => Some(SpecialKey::F11),
            0x58 => Some(SpecialKey::F12),
            _ => None,
        }
    }
    
    /// Process a scancode and return a key event
    pub fn process_scancode(scancode: u8) -> Option<KeyEvent> {
        let mut state = KEYBOARD_STATE.lock();
        
        // Check for release (high bit set)
        let released = scancode & 0x80 != 0;
        let code = scancode & 0x7F;
        
        // Handle extended scancodes (0xE0 prefix)
        // For simplicity, we'll handle basic scancodes here
        
        // Update modifier state
        match code {
            0x1D => state.ctrl = !released,
            0x2A | 0x36 => state.shift = !released,
            0x38 => state.alt = !released,
            0x3A => {
                if !released {
                    state.caps_lock = !state.caps_lock;
                }
            }
            0x45 => {
                if !released {
                    state.num_lock = !state.num_lock;
                }
            }
            _ => {}
        }
        
        // Get character
        let key = if state.shift {
            SCANCODE_TABLE_SHIFT.get(code as usize).and_then(|&c| c)
        } else {
            SCANCODE_TABLE.get(code as usize).and_then(|&c| c)
        };
        
        // Apply caps lock for letters
        let key = if state.caps_lock && key.is_some() {
            let c = key.unwrap();
            if c.is_ascii_alphabetic() {
                if state.shift {
                    Some(c.to_ascii_lowercase())
                } else {
                    Some(c.to_ascii_uppercase())
                }
            } else {
                Some(c)
            }
        } else {
            key
        };
        
        let special = get_special_key(code);
        
        Some(KeyEvent {
            scancode,
            key,
            special,
            state: if released { KeyState::Released } else { KeyState::Pressed },
            shift: state.shift,
            ctrl: state.ctrl,
            alt: state.alt,
        })
    }
    
    /// Handle keyboard interrupt
    pub fn handle_interrupt() {
        // Read scancode from keyboard
        let scancode = unsafe {
            let mut val: u8;
            core::arch::asm!(
                "in al, dx",
                out("al") val,
                in("dx") KBD_DATA,
                options(nostack, nomem)
            );
            val
        };
        
        // Process and buffer
        if let Some(event) = process_scancode(scancode) {
            KEY_BUFFER.lock().push_back(event);
        }
    }
    
    /// Read a key from the buffer
    pub fn read_key() -> Option<KeyEvent> {
        KEY_BUFFER.lock().pop_front()
    }
    
    /// Check if a key is available
    pub fn has_key() -> bool {
        !KEY_BUFFER.lock().is_empty()
    }
    
    /// Initialize keyboard
    pub fn init() {
        serial_println!("[KBD] Initializing keyboard...");
        
        // Enable keyboard interrupts
        unsafe {
            // Read current mask
            let mut mask: u8;
            core::arch::asm!(
                "in al, dx",
                out("al") mask,
                in("dx") 0x21u16,
                options(nostack, nomem)
            );
            // Enable IRQ1 (keyboard)
            mask &= !0x02;
            core::arch::asm!(
                "out dx, al",
                in("dx") 0x21u16,
                in("al") mask,
                options(nostack, nomem)
            );
        }
        
        serial_println!("[OK] Keyboard initialized");
    }
}

// ========== Timer Driver ==========

pub mod timer {
    use spin::Mutex;
    use crate::serial_println;
    
    /// PIT frequency (1193182 Hz)
    const PIT_FREQUENCY: u64 = 1193182;
    
    /// PIT channel 0 data port
    const PIT_CH0_DATA: u16 = 0x40;
    /// PIT channel 1 data port
    const PIT_CH1_DATA: u16 = 0x41;
    /// PIT channel 2 data port
    const PIT_CH2_DATA: u16 = 0x42;
    /// PIT mode/command register
    const PIT_MODE: u16 = 0x43;
    
    /// Timer tick counter
    pub static TICKS: Mutex<u64> = Mutex::new(0);
    
    /// Timer frequency
    pub static FREQUENCY: Mutex<u32> = Mutex::new(100);
    
    /// Initialize PIT timer
    pub fn init(frequency: u32) {
        *FREQUENCY.lock() = frequency;
        
        let divisor = (PIT_FREQUENCY / frequency as u64) as u16;
        
        unsafe {
            // Channel 0, Access mode: lobyte/hibyte, Mode 3 (square wave), Binary
            core::arch::asm!(
                "out dx, al",
                in("dx") PIT_MODE,
                in("al") 0x36u8,
                options(nostack, nomem)
            );
            
            // Set divisor
            let low = (divisor & 0xFF) as u8;
            let high = ((divisor >> 8) & 0xFF) as u8;
            
            core::arch::asm!(
                "out dx, al",
                in("dx") PIT_CH0_DATA,
                in("al") low,
                options(nostack, nomem)
            );
            core::arch::asm!(
                "out dx, al",
                in("dx") PIT_CH0_DATA,
                in("al") high,
                options(nostack, nomem)
            );
        }
        
        serial_println!("[TIMER] Initialized at {} Hz", frequency);
    }
    
    /// Handle timer interrupt
    pub fn handle_interrupt() {
        *TICKS.lock() += 1;
    }
    
    /// Get current tick count
    pub fn get_ticks() -> u64 {
        *TICKS.lock()
    }
    
    /// Sleep for specified ticks
    pub fn sleep_ticks(ticks: u64) {
        let start = get_ticks();
        while get_ticks() - start < ticks {
            core::hint::spin_loop();
        }
    }
    
    /// Sleep for specified milliseconds (approximate)
    pub fn sleep_ms(ms: u64) {
        let freq = *FREQUENCY.lock() as u64;
        let ticks = (ms * freq) / 1000;
        sleep_ticks(ticks);
    }
}