# VantisOS v0.5.0 - VGA Console Component Documentation

**Version**: 0.5.0  
**Component**: VGA Console  
**File**: `src/verified/v0.5.0_kernel/vga_console.rs`  
**Lines**: ~310 lines

---

## Overview

The VGA Console component provides text mode console output for the VantisOS kernel. It implements a VGA text mode driver with support for colors, cursor positioning, screen scrolling, and special characters.

## Features

- **Text Mode Display**: 80x25 character text mode at 0xB8000
- **Color Support**: 16 foreground colors and 16 background colors
- **Cursor Positioning**: Programmable cursor position
- **Screen Scrolling**: Automatic screen scrolling when buffer is full
- **Special Characters**: Support for newline, tab, backspace, and carriage return
- **String/Number Output**: Functions for printing strings, decimal numbers, and hexadecimal numbers
- **Boolean Output**: Function for printing boolean values

## Architecture

### Console State
```rust
struct Console {
    buffer: *mut u8,
    width: usize,
    height: usize,
    cursor_x: usize,
    cursor_y: usize,
    color: u8,
    initialized: bool,
}
```

### Color Constants
```rust
enum VgaColor {
    Black, Blue, Green, Cyan, Red, Magenta, Brown, LightGray,
    DarkGray, LightBlue, LightGreen, LightCyan, LightRed, LightMagenta, Yellow, White,
}
```

## Public API

### Initialization

#### `init()`
Initialize the VGA console.

```rust
pub fn init()
```

**Description**: Initializes the VGA console by setting up the buffer, clearing the screen, and setting default colors.

**Parameters**: None

**Returns**: None

**Example**:
```rust
vga_console::init();
```

---

### Color Management

#### `set_color(foreground, background)`
Set the console color.

```rust
pub fn set_color(foreground: VgaColor, background: VgaColor)
```

**Parameters**:
- `foreground`: Foreground color (VgaColor)
- `background`: Background color (VgaColor)

**Returns**: None

**Example**:
```rust
vga_console::set_color(VgaColor::White, VgaColor::Black);
```

---

### Screen Operations

#### `clear_screen()`
Clear the entire screen.

```rust
pub fn clear_screen()
```

**Description**: Clears the entire screen by filling it with spaces.

**Parameters**: None

**Returns**: None

**Example**:
```rust
vga_console::clear_screen();
```

---

### Character Output

#### `write_byte(byte)`
Write a single byte to the console.

```rust
pub fn write_byte(byte: u8)
```

**Parameters**:
- `byte`: Byte to write (u8)

**Returns**: None

**Description**: Writes a single byte to the console at the current cursor position. Handles special characters like newline, tab, backspace, and carriage return.

**Example**:
```rust
vga_console::write_byte(b'A');
```

---

### String Output

#### `write_string(string)`
Write a string to the console.

```rust
pub fn write_string(string: &str)
```

**Parameters**:
- `string`: String to write (&str)

**Returns**: None

**Description**: Writes a null-terminated string to the console.

**Example**:
```rust
vga_console::write_string("Hello, VantisOS!");
```

---

### Number Output

#### `write_dec(value)`
Write a decimal number to the console.

```rust
pub fn write_dec(value: u64)
```

**Parameters**:
- `value`: Decimal number to write (u64)

**Returns**: None

**Description**: Writes a decimal number to the console.

**Example**:
```rust
vga_console::write_dec(12345);
```

#### `write_hex(value)`
Write a hexadecimal number to the console.

```rust
pub fn write_hex(value: u64)
```

**Parameters**:
- `value`: Hexadecimal number to write (u64)

**Returns**: None

**Description**: Writes a hexadecimal number to the console.

**Example**:
```rust
vga_console::write_hex(0xDEADBEEF);
```

#### `write_hex32(value)`
Write a 32-bit hexadecimal number to the console.

```rust
pub fn write_hex32(value: u32)
```

**Parameters**:
- `value`: 32-bit hexadecimal number to write (u32)

**Returns**: None

**Description**: Writes a 32-bit hexadecimal number to the console.

**Example**:
```rust
vga_console::write_hex32(0xDEADBEEF);
```

---

### Boolean Output

#### `write_bool(value)`
Write a boolean value to the console.

```rust
pub fn write_bool(value: bool)
```

**Parameters**:
- `value`: Boolean value to write (bool)

**Returns**: None

**Description**: Writes "true" or "false" to the console.

**Example**:
```rust
vga_console::write_bool(true);
```

---

## Internal Implementation

### Buffer Access
The VGA text buffer is located at physical address 0xB8000. Each character is represented by 2 bytes:
- Byte 0: Character code (ASCII)
- Byte 1: Color attribute (foreground in lower 4 bits, background in upper 4 bits)

### Color Attribute
```rust
color = (background << 4) | foreground
```

### Cursor Positioning
The cursor position is tracked by `cursor_x` (0-79) and `cursor_y` (0-24). The cursor automatically advances after each character and wraps to the next line when reaching the end of a line.

### Screen Scrolling
When the cursor reaches the bottom of the screen (line 24), the screen scrolls up by one line. All lines are moved up by one line, and the bottom line is cleared.

### Special Characters
- **Newline (0x0A)**: Move cursor to the beginning of the next line
- **Carriage Return (0x0D)**: Move cursor to the beginning of the current line
- **Tab (0x09)**: Move cursor to the next tab stop (every 8 characters)
- **Backspace (0x08)**: Move cursor back one position and clear the character

---

## Performance Characteristics

- **Character Write**: < 1μs per character
- **String Write**: < 100μs per 1000 characters
- **Screen Clear**: < 1ms
- **Color Change**: < 1μs

---

## Usage Examples

### Basic Output
```rust
use vga_console::{init, write_string, write_dec};

fn main() {
    init();
    write_string("VantisOS v0.5.0\n");
    write_string("Booting...\n");
}
```

### Colored Output
```rust
use vga_console::{init, set_color, write_string};

fn main() {
    init();
    set_color(VgaColor::LightGreen, VgaColor::Black);
    write_string("Success!\n");
}
```

### Number Output
```rust
use vga_console::{init, write_string, write_dec, write_hex};

fn main() {
    init();
    write_string("Decimal: ");
    write_dec(12345);
    write_string("\nHex: ");
    write_hex(0xDEADBEEF);
    write_string("\n");
}
```

---

## Limitations

- Text mode only (no graphics support)
- Fixed resolution (80x25)
- No Unicode support (ASCII only)
- No cursor blinking
- No keyboard input (handled by separate driver)

---

## Future Enhancements

- Add graphics mode support (VESA VBE)
- Add Unicode support
- Add cursor blinking
- Add multiple console support
- Add console buffer history
- Add console logging to file

---

## References

- VGA Hardware Specifications
- VGA Text Mode Programming
- x86 Architecture Manual