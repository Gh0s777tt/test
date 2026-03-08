// Serial Port Driver
//
// This module provides serial port driver support for VantisOS, including:
// - Serial port initialization
// - Serial I/O operations
// - Console output via serial
// - Debug logging

#![no_std]

use core::fmt;

/// Serial port base addresses
const COM1_PORT: u16 = 0x3F8;
const COM2_PORT: u16 = 0x2F8;
const COM3_PORT: u16 = 0x3E8;
const COM4_PORT: u16 = 0x2E8;

/// Serial port registers
const SERIAL_DATA_REG: u16 = 0;   // Data register (read/write)
const SERIAL_INT_ENABLE: u16 = 1;  // Interrupt enable register
const SERIAL_FIFO_CTRL: u16 = 2;   // FIFO control register
const SERIAL_LINE_CTRL: u16 = 3;   // Line control register
const SERIAL_MODEM_CTRL: u16 = 4;  // Modem control register
const SERIAL_LINE_STATUS: u16 = 5; // Line status register
const SERIAL_MODEM_STATUS: u16 = 6;// Modem status register
const SERIAL_SCRATCH: u16 = 7;     // Scratch register

/// Line control register bits
const SERIAL_DLAB: u8 = 0x80;      // Divisor latch access bit

/// Line status register bits
const SERIAL_DATA_READY: u8 = 0x01;
const SERIAL_TRANSMIT_EMPTY: u8 = 0x20;

/// Baud rate divisors (for 115200 baud)
const BAUD_DIVISOR_LOW: u8 = 0x01;
const BAUD_DIVISOR_HIGH: u8 = 0x00;

/// Default serial port (COM1)
const DEFAULT_SERIAL_PORT: u16 = COM1_PORT;

/// Initialize serial port
pub fn init(port: u16) {
    unsafe {
        // Disable interrupts
        outb(port + SERIAL_INT_ENABLE, 0x00);

        // Enable DLAB (set baud rate divisor)
        outb(port + SERIAL_LINE_CTRL, SERIAL_DLAB);

        // Set divisor to 3 (lo byte) 38400 baud
        outb(port + SERIAL_DATA_REG, 0x03);
        outb(port + SERIAL_INT_ENABLE, 0x00); // (hi byte)

        // 8 bits, no parity, one stop bit
        outb(port + SERIAL_LINE_CTRL, 0x03);

        // Enable FIFO, clear them, with 14-byte threshold
        outb(port + SERIAL_FIFO_CTRL, 0xC7);

        // IRQs enabled, RTS/DSR set
        outb(port + SERIAL_MODEM_CTRL, 0x0B);

        // Set in loopback mode, test the serial chip
        outb(port + SERIAL_MODEM_CTRL, 0x1E);

        // Test serial chip (send byte 0xAE and check if serial returns same byte)
        outb(port + SERIAL_DATA_REG, 0xAE);

        // Check if serial is faulty (i.e., not same byte as sent)
        if inb(port + SERIAL_DATA_REG) != 0xAE {
            // Serial port is faulty
            return;
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        outb(port + SERIAL_MODEM_CTRL, 0x0F);
    }
}

/// Initialize default serial port (COM1)
pub fn init_default() {
    init(DEFAULT_SERIAL_PORT);
}

/// Write byte to serial port
pub fn write_byte(port: u16, byte: u8) {
    unsafe {
        // Wait for transmit buffer to be empty
        while (inb(port + SERIAL_LINE_STATUS) & SERIAL_TRANSMIT_EMPTY) == 0 {}
        
        // Write byte
        outb(port + SERIAL_DATA_REG, byte);
    }
}

/// Write byte to default serial port
pub fn write_byte_default(byte: u8) {
    write_byte(DEFAULT_SERIAL_PORT, byte);
}

/// Read byte from serial port
pub fn read_byte(port: u16) -> Option<u8> {
    unsafe {
        // Wait for data to be available
        while (inb(port + SERIAL_LINE_STATUS) & SERIAL_DATA_READY) == 0 {}
        
        // Read byte
        Some(inb(port + SERIAL_DATA_REG))
    }
}

/// Read byte from default serial port
pub fn read_byte_default() -> Option<u8> {
    read_byte(DEFAULT_SERIAL_PORT)
}

/// Check if data is available
pub fn data_available(port: u16) -> bool {
    unsafe {
        (inb(port + SERIAL_LINE_STATUS) & SERIAL_DATA_READY) != 0
    }
}

/// Check if data is available on default port
pub fn data_available_default() -> bool {
    data_available(DEFAULT_SERIAL_PORT)
}

/// Write string to serial port
pub fn write_str(port: u16, s: &str) {
    for byte in s.bytes() {
        write_byte(port, byte);
    }
}

/// Write string to default serial port
pub fn write_str_default(s: &str) {
    write_str(DEFAULT_SERIAL_PORT, s);
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

/// Serial writer for fmt::Write trait
pub struct SerialWriter {
    port: u16,
}

impl SerialWriter {
    pub fn new(port: u16) -> Self {
        SerialWriter { port }
    }

    pub fn default() -> Self {
        SerialWriter::new(DEFAULT_SERIAL_PORT)
    }
}

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write_str(self.port, s);
        Ok(())
    }
}

/// Macro for serial printing
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut writer = $crate::verified::minimal_kernel::serial::SerialWriter::default();
            write!(writer, $($arg)*).unwrap();
        }
    };
}

/// Macro for serial printing with newline
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

/// Debug logging levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

/// Current log level
static mut LOG_LEVEL: LogLevel = LogLevel::Info;

/// Set log level
pub fn set_log_level(level: LogLevel) {
    unsafe {
        LOG_LEVEL = level;
    }
}

/// Get log level
pub fn get_log_level() -> LogLevel {
    unsafe { LOG_LEVEL }
}

/// Log message at specified level
pub fn log(level: LogLevel, args: fmt::Arguments) {
    if level <= get_log_level() {
        use core::fmt::Write;
        let mut writer = SerialWriter::default();
        
        // Write log level prefix
        let prefix = match level {
            LogLevel::Error => "[ERROR] ",
            LogLevel::Warn => "[WARN]  ",
            LogLevel::Info => "[INFO]  ",
            LogLevel::Debug => "[DEBUG] ",
            LogLevel::Trace => "[TRACE] ",
        };
        
        write!(writer, "{}", prefix).unwrap();
        write!(writer, "{}", args).unwrap();
        write!(writer, "\n").unwrap();
    }
}

/// Log error
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::verified::minimal_kernel::serial::log(
            $crate::verified::minimal_kernel::serial::LogLevel::Error,
            format_args!($($arg)*)
        );
    };
}

/// Log warning
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::verified::minimal_kernel::serial::log(
            $crate::verified::minimal_kernel::serial::LogLevel::Warn,
            format_args!($($arg)*)
        );
    };
}

/// Log info
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::verified::minimal_kernel::serial::log(
            $crate::verified::minimal_kernel::serial::LogLevel::Info,
            format_args!($($arg)*)
        );
    };
}

/// Log debug
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::verified::minimal_kernel::serial::log(
            $crate::verified::minimal_kernel::serial::LogLevel::Debug,
            format_args!($($arg)*)
        );
    };
}

/// Log trace
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::verified::minimal_kernel::serial::log(
            $crate::verified::minimal_kernel::serial::LogLevel::Trace,
            format_args!($($arg)*)
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_port_addresses() {
        assert_eq!(COM1_PORT, 0x3F8);
        assert_eq!(COM2_PORT, 0x2F8);
        assert_eq!(COM3_PORT, 0x3E8);
        assert_eq!(COM4_PORT, 0x2E8);
    }

    #[test]
    fn test_log_levels() {
        assert!(LogLevel::Error < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Trace);
    }

    #[test]
    fn test_serial_writer() {
        let writer = SerialWriter::default();
        assert_eq!(writer.port, COM1_PORT);
    }
}