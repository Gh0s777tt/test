//! Serial Port Driver for Debugging
//! Provides output to COM1 port

use spin::Mutex;
use uart_16550::SerialPort;

/// Serial port address
pub const SERIAL_PORT: u16 = 0x3F8; // COM1

/// Global serial port
pub static SERIAL: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(SERIAL_PORT) });

/// Initialize serial port
pub fn init() {
    let mut serial = SERIAL.lock();
    serial.init();
}

/// Print a string to serial
pub fn print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL.lock().write_fmt(args).unwrap();
}

/// Print a line to serial
pub fn println() {
    print(format_args!("\n"));
}

/// Print formatted to serial
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::arch::serial::print(format_args!($($arg)*))
    };
}

/// Print formatted line to serial
#[macro_export]
macro_rules! serial_println {
    () => {
        $crate::arch::serial::print(format_args!("\n"))
    };
    ($fmt:expr) => {
        $crate::arch::serial::print(format_args!(concat!($fmt, "\n")))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::arch::serial::print(format_args!(concat!($fmt, "\n"), $($arg)*))
    };
}