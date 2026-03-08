// Keyboard Driver
//
// This module provides keyboard driver support for VantisOS, including:
// - PS/2 keyboard initialization
// - Keyboard interrupt handling
// - Key code translation
// - Keyboard buffer management

#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

/// Keyboard data port
const KEYBOARD_DATA_PORT: u16 = 0x60;

/// Keyboard command port
const KEYBOARD_COMMAND_PORT: u16 = 0x64;

/// Keyboard status register bits
const KEYBOARD_STATUS_OUTPUT_FULL: u8 = 0x01;
const KEYBOARD_STATUS_INPUT_FULL: u8 = 0x02;
const KEYBOARD_STATUS_SYSTEM_FLAG: u8 = 0x04;
const KEYBOARD_STATUS_COMMAND_DATA: u8 = 0x08;
const KEYBOARD_STATUS_LOCKED: u8 = 0x10;
const KEYBOARD_STATUS_AUX_OUTPUT: u8 = 0x20;
const KEYBOARD_STATUS_TIMEOUT: u8 = 0x40;
const KEYBOARD_STATUS_PARITY_ERROR: u8 = 0x80;

/// Keyboard commands
const KEYBOARD_CMD_SET_LEDS: u8 = 0xED;
const KEYBOARD_CMD_ECHO: u8 = 0xEE;
const KEYBOARD_CMD_SET_SCANCODE: u8 = 0xF0;
const KEYBOARD_CMD_SEND_TO_MOUSE: u8 = 0xD4;
const KEYBOARD_CMD_SYSTEM_RESET: u8 = 0xFF;
const KEYBOARD_CMD_DISABLE: u8 = 0xF5;
const KEYBOARD_CMD_ENABLE: u8 = 0xF4;
const KEYBOARD_CMD_SET_DEFAULT: u8 = 0xF6;
const KEYBOARD_CMD_RESEND: u8 = 0xFE;
const KEYBOARD_CMD_READ_ID: u8 = 0xF2;

/// Keyboard LED bits
const KEYBOARD_LED_SCROLL_LOCK: u8 = 0x01;
const KEYBOARD_LED_NUM_LOCK: u8 = 0x02;
const KEYBOARD_LED_CAPS_LOCK: u8 = 0x04;

/// Keyboard buffer size
const KEYBOARD_BUFFER_SIZE: usize = 256;

/// Keyboard buffer
static mut KEYBOARD_BUFFER: [u8; KEYBOARD_BUFFER_SIZE] = [0; KEYBOARD_BUFFER_SIZE];
static mut KEYBOARD_BUFFER_HEAD: AtomicUsize = AtomicUsize::new(0);
static mut KEYBOARD_BUFFER_TAIL: AtomicUsize = AtomicUsize::new(0);

/// Modifier key state
static mut MODIFIER_STATE: u8 = 0;

/// Modifier key flags
const MODIFIER_SHIFT: u8 = 0x01;
const MODIFIER_CTRL: u8 = 0x02;
const MODIFIER_ALT: u8 = 0x04;
const MODIFIER_CAPS_LOCK: u8 = 0x08;
const MODIFIER_NUM_LOCK: u8 = 0x10;
const MODIFIER_SCROLL_LOCK: u8 = 0x20;

/// LED state
static mut LED_STATE: u8 = 0;

/// US QWERTY scancode set 1 to ASCII translation table
static SCANCODE_TO_ASCII: [u8; 128] = [
    0, 0, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8,
    '7' as u8, '8' as u8, '9' as u8, '0' as u8, '-' as u8, '=' as u8, 0x08, 0x09,
    'q' as u8, 'w' as u8, 'e' as u8, 'r' as u8, 't' as u8, 'y' as u8, 'u' as u8, 'i' as u8,
    'o' as u8, 'p' as u8, '[' as u8, ']' as u8, 0x0A, 0, 'a' as u8, 's' as u8,
    'd' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'j' as u8, 'k' as u8, 'l' as u8, ';' as u8,
    '\'' as u8, '`' as u8, 0, '\\' as u8, 'z' as u8, 'x' as u8, 'c' as u8, 'v' as u8,
    'b' as u8, 'n' as u8, 'm' as u8, ',' as u8, '.' as u8, '/' as u8, 0, '*',
    0, ' ' as u8, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, '7' as u8,
    '8' as u8, '9' as u8, '-' as u8, '4' as u8, '5' as u8, '6' as u8, '+' as u8, '1' as u8,
    '2' as u8, '3' as u8, '0' as u8, '.' as u8, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

/// Shifted ASCII translation table
static SCANCODE_TO_ASCII_SHIFTED: [u8; 128] = [
    0, 0, '!' as u8, '@' as u8, '#' as u8, '$' as u8, '%' as u8, '^' as u8,
    '&' as u8, '*' as u8, '(' as u8, ')' as u8, '_' as u8, '+' as u8, 0x08, 0x09,
    'Q' as u8, 'W' as u8, 'E' as u8, 'R' as u8, 'T' as u8, 'Y' as u8, 'U' as u8, 'I' as u8,
    'O' as u8, 'P' as u8, '{' as u8, '}' as u8, 0x0A, 0, 'A' as u8, 'S' as u8,
    'D' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'J' as u8, 'K' as u8, 'L' as u8, ':' as u8,
    '"' as u8, '~' as u8, 0, '|' as u8, 'Z' as u8, 'X' as u8, 'C' as u8, 'V' as u8,
    'B' as u8, 'N' as u8, 'M' as u8, '<' as u8, '>' as u8, '?' as u8, 0, '*',
    0, ' ' as u8, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, '7' as u8,
    '8' as u8, '9' as u8, '-' as u8, '4' as u8, '5' as u8, '6' as u8, '+' as u8, '1' as u8,
    '2' as u8, '3' as u8, '0' as u8, '.' as u8, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

/// Special scancodes
const SCANCODE_ESCAPE: u8 = 0xE0;
const SCANCODE_RELEASE: u8 = 0xF0;

/// Initialize keyboard
pub fn init() {
    // Disable keyboard
    send_command(KEYBOARD_CMD_DISABLE);
    
    // Flush output buffer
    while read_status() & KEYBOARD_STATUS_OUTPUT_FULL != 0 {
        read_data();
    }
    
    // Set scancode set 1
    send_command(KEYBOARD_CMD_SET_SCANCODE);
    send_data(0x01);
    
    // Enable keyboard
    send_command(KEYBOARD_CMD_ENABLE);
    
    // Clear keyboard buffer
    unsafe {
        KEYBOARD_BUFFER_HEAD.store(0, Ordering::SeqCst);
        KEYBOARD_BUFFER_TAIL.store(0, Ordering::SeqCst);
    }
}

/// Send command to keyboard
fn send_command(cmd: u8) {
    unsafe {
        // Wait for input buffer to be empty
        while read_status() & KEYBOARD_STATUS_INPUT_FULL != 0 {}
        
        // Send command
        outb(KEYBOARD_COMMAND_PORT, cmd);
    }
}

/// Send data to keyboard
fn send_data(data: u8) {
    unsafe {
        // Wait for input buffer to be empty
        while read_status() & KEYBOARD_STATUS_INPUT_FULL != 0 {}
        
        // Send data
        outb(KEYBOARD_DATA_PORT, data);
    }
}

/// Read keyboard status
fn read_status() -> u8 {
    unsafe {
        inb(KEYBOARD_COMMAND_PORT)
    }
}

/// Read keyboard data
fn read_data() -> u8 {
    unsafe {
        inb(KEYBOARD_DATA_PORT)
    }
}

/// Write byte to I/O port
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

/// Read byte from I/O port
unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    core::arch::asm!("in al, dx", inlateout("dx") port => result, options(nomem, nostack));
    result
}

/// Keyboard interrupt handler
pub fn keyboard_interrupt_handler() {
    let scancode = read_data();
    
    // Handle special scancodes
    if scancode == SCANCODE_ESCAPE {
        // Extended scancode - read next byte
        // TODO: Handle extended scancodes
        return;
    }
    
    if scancode == SCANCODE_RELEASE {
        // Key release - read next byte and clear modifier
        let released_scancode = read_data();
        handle_key_release(released_scancode);
        return;
    }
    
    // Regular key press
    handle_key_press(scancode);
}

/// Handle key press
fn handle_key_press(scancode: u8) {
    match scancode {
        0x2A | 0x36 => unsafe { MODIFIER_STATE |= MODIFIER_SHIFT; }, // Left/Right Shift
        0x1D => unsafe { MODIFIER_STATE |= MODIFIER_CTRL; }, // Ctrl
        0x38 => unsafe { MODIFIER_STATE |= MODIFIER_ALT; }, // Alt
        0x3A => { // Caps Lock
            unsafe {
                MODIFIER_STATE ^= MODIFIER_CAPS_LOCK;
                LED_STATE ^= KEYBOARD_LED_CAPS_LOCK;
                update_leds();
            }
        }
        0x45 => { // Num Lock
            unsafe {
                MODIFIER_STATE ^= MODIFIER_NUM_LOCK;
                LED_STATE ^= KEYBOARD_LED_NUM_LOCK;
                update_leds();
            }
        }
        0x46 => { // Scroll Lock
            unsafe {
                MODIFIER_STATE ^= MODIFIER_SCROLL_LOCK;
                LED_STATE ^= KEYBOARD_LED_SCROLL_LOCK;
                update_leds();
            }
        }
        _ => {
            // Regular key - translate and add to buffer
            let ascii = translate_scancode(scancode);
            if ascii != 0 {
                add_to_buffer(ascii);
            }
        }
    }
}

/// Handle key release
fn handle_key_release(scancode: u8) {
    match scancode {
        0x2A | 0x36 => unsafe { MODIFIER_STATE &= !MODIFIER_SHIFT; }, // Left/Right Shift
        0x1D => unsafe { MODIFIER_STATE &= !MODIFIER_CTRL; }, // Ctrl
        0x38 => unsafe { MODIFIER_STATE &= !MODIFIER_ALT; }, // Alt
        _ => {}
    }
}

/// Translate scancode to ASCII
fn translate_scancode(scancode: u8) -> u8 {
    if scancode >= 128 {
        return 0;
    }
    
    unsafe {
        let shifted = (MODIFIER_STATE & MODIFIER_SHIFT) != 0;
        let caps_lock = (MODIFIER_STATE & MODIFIER_CAPS_LOCK) != 0;
        
        let base = if shifted {
            SCANCODE_TO_ASCII_SHIFTED[scancode as usize]
        } else {
            SCANCODE_TO_ASCII[scancode as usize]
        };
        
        // Apply caps lock to letters
        if base >= b'a' && base <= b'z' {
            if caps_lock ^ shifted {
                base - 32 // Convert to uppercase
            } else {
                base
            }
        } else {
            base
        }
    }
}

/// Add character to keyboard buffer
fn add_to_buffer(c: u8) {
    unsafe {
        let head = KEYBOARD_BUFFER_HEAD.load(Ordering::SeqCst);
        let tail = KEYBOARD_BUFFER_TAIL.load(Ordering::SeqCst);
        
        let next_head = (head + 1) % KEYBOARD_BUFFER_SIZE;
        
        // Check if buffer is full
        if next_head != tail {
            KEYBOARD_BUFFER[head] = c;
            KEYBOARD_BUFFER_HEAD.store(next_head, Ordering::SeqCst);
        }
    }
}

/// Read character from keyboard buffer
pub fn read_char() -> Option<u8> {
    unsafe {
        let head = KEYBOARD_BUFFER_HEAD.load(Ordering::SeqCst);
        let tail = KEYBOARD_BUFFER_TAIL.load(Ordering::SeqCst);
        
        if head == tail {
            return None;
        }
        
        let c = KEYBOARD_BUFFER[tail];
        KEYBOARD_BUFFER_TAIL.store((tail + 1) % KEYBOARD_BUFFER_SIZE, Ordering::SeqCst);
        
        Some(c)
    }
}

/// Check if character is available in buffer
pub fn char_available() -> bool {
    unsafe {
        let head = KEYBOARD_BUFFER_HEAD.load(Ordering::SeqCst);
        let tail = KEYBOARD_BUFFER_TAIL.load(Ordering::SeqCst);
        head != tail
    }
}

/// Update keyboard LEDs
fn update_leds() {
    unsafe {
        send_command(KEYBOARD_CMD_SET_LEDS);
        send_data(LED_STATE);
    }
}

/// Get modifier state
pub fn get_modifier_state() -> u8 {
    unsafe { MODIFIER_STATE }
}

/// Check if shift is pressed
pub fn is_shift_pressed() -> bool {
    unsafe { (MODIFIER_STATE & MODIFIER_SHIFT) != 0 }
}

/// Check if ctrl is pressed
pub fn is_ctrl_pressed() -> bool {
    unsafe { (MODIFIER_STATE & MODIFIER_CTRL) != 0 }
}

/// Check if alt is pressed
pub fn is_alt_pressed() -> bool {
    unsafe { (MODIFIER_STATE & MODIFIER_ALT) != 0 }
}

/// Check if caps lock is on
pub fn is_caps_lock_on() -> bool {
    unsafe { (MODIFIER_STATE & MODIFIER_CAPS_LOCK) != 0 }
}

/// Check if num lock is on
pub fn is_num_lock_on() -> bool {
    unsafe { (MODIFIER_STATE & MODIFIER_NUM_LOCK) != 0 }
}

/// Check if scroll lock is on
pub fn is_scroll_lock_on() -> bool {
    unsafe { (MODIFIER_STATE & MODIFIER_SCROLL_LOCK) != 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_buffer_size() {
        assert_eq!(KEYBOARD_BUFFER_SIZE, 256);
    }

    #[test]
    fn test_modifier_flags() {
        assert_eq!(MODIFIER_SHIFT, 0x01);
        assert_eq!(MODIFIER_CTRL, 0x02);
        assert_eq!(MODIFIER_ALT, 0x04);
        assert_eq!(MODIFIER_CAPS_LOCK, 0x08);
    }

    #[test]
    fn test_led_flags() {
        assert_eq!(KEYBOARD_LED_SCROLL_LOCK, 0x01);
        assert_eq!(KEYBOARD_LED_NUM_LOCK, 0x02);
        assert_eq!(KEYBOARD_LED_CAPS_LOCK, 0x04);
    }

    #[test]
    fn test_scancode_translation() {
        assert_eq!(SCANCODE_TO_ASCII[0x02], b'1');
        assert_eq!(SCANCODE_TO_ASCII[0x10], b'q');
        assert_eq!(SCANCODE_TO_ASCII[0x1E], b'a');
    }

    #[test]
    fn test_scancode_translation_shifted() {
        assert_eq!(SCANCODE_TO_ASCII_SHIFTED[0x02], b'!');
        assert_eq!(SCANCODE_TO_ASCII_SHIFTED[0x10], b'Q');
        assert_eq!(SCANCODE_TO_ASCII_SHIFTED[0x1E], b'A');
    }
}