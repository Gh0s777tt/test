// Timer Management
//
// This module provides timer management for VantisOS, including:
// - Programmable Interval Timer (PIT) configuration
// - Timer interrupts
// - Time tracking
// - Sleep/delay functions

#![no_std]

use core::arch::asm;
use core::sync::atomic::{AtomicU64, Ordering};

/// PIT frequency (1.193182 MHz)
const PIT_FREQUENCY: u64 = 1_193_182;

/// Default timer frequency (1000 Hz = 1 ms per tick)
const TIMER_FREQUENCY: u64 = 1000;

/// PIT divisor for default frequency
const PIT_DIVISOR: u16 = (PIT_FREQUENCY / TIMER_FREQUENCY) as u16;

/// PIT I/O ports
const PIT_COMMAND: u16 = 0x43;
const PIT_CHANNEL0: u16 = 0x40;
const PIT_CHANNEL1: u16 = 0x41;
const PIT_CHANNEL2: u16 = 0x42;

/// PIT commands
const PIT_CHANNEL0_SELECT: u8 = 0x00;
const PIT_CHANNEL1_SELECT: u8 = 0x40;
const PIT_CHANNEL2_SELECT: u8 = 0x80;
const PIT_READ_BACK: u8 = 0xC0;

const PIT_LATCH_COUNT: u8 = 0x00;
const PIT_LOW_BYTE_ONLY: u8 = 0x10;
const PIT_HIGH_BYTE_ONLY: u8 = 0x20;
const PIT_LOW_THEN_HIGH_BYTE: u8 = 0x30;

const PIT_MODE0: u8 = 0x00; // Interrupt on terminal count
const PIT_MODE1: u8 = 0x02; // Hardware re-triggerable one-shot
const PIT_MODE2: u8 = 0x04; // Rate generator
const PIT_MODE3: u8 = 0x06; // Square wave generator
const PIT_MODE4: u8 = 0x08; // Software triggered strobe
const PIT_MODE5: u8 = 0x0A; // Hardware triggered strobe

const PIT_BINARY_MODE: u8 = 0x00;
const PIT_BCD_MODE: u8 = 0x01;

/// Global tick counter
static TICKS: AtomicU64 = AtomicU64::new(0);

/// Timer callback type
pub type TimerCallback = fn();

/// Timer callbacks (up to 16)
static mut TIMER_CALLBACKS: [Option<TimerCallback>; 16] = [None; 16];

/// Get current tick count
pub fn get_ticks() -> u64 {
    TICKS.load(Ordering::SeqCst)
}

/// Get milliseconds since boot
pub fn get_milliseconds() -> u64 {
    get_ticks() * (1000 / TIMER_FREQUENCY)
}

/// Get seconds since boot
pub fn get_seconds() -> u64 {
    get_ticks() / TIMER_FREQUENCY
}

/// Initialize PIT
pub fn init() {
    unsafe {
        // Configure PIT channel 0 for square wave mode
        let command = PIT_CHANNEL0_SELECT | PIT_LOW_THEN_HIGH_BYTE | PIT_MODE3 | PIT_BINARY_MODE;
        outb(PIT_COMMAND, command);

        // Set divisor
        let divisor_low = (PIT_DIVISOR & 0xFF) as u8;
        let divisor_high = ((PIT_DIVISOR >> 8) & 0xFF) as u8;
        
        outb(PIT_CHANNEL0, divisor_low);
        outb(PIT_CHANNEL0, divisor_high);
    }
}

/// Set timer frequency
pub fn set_frequency(freq: u64) {
    let divisor = (PIT_FREQUENCY / freq) as u16;
    
    unsafe {
        let command = PIT_CHANNEL0_SELECT | PIT_LOW_THEN_HIGH_BYTE | PIT_MODE3 | PIT_BINARY_MODE;
        outb(PIT_COMMAND, command);

        let divisor_low = (divisor & 0xFF) as u8;
        let divisor_high = ((divisor >> 8) & 0xFF) as u8;
        
        outb(PIT_CHANNEL0, divisor_low);
        outb(PIT_CHANNEL0, divisor_high);
    }
}

/// Timer interrupt handler
pub fn timer_interrupt_handler() {
    // Increment tick counter
    TICKS.fetch_add(1, Ordering::SeqCst);

    // Call registered callbacks
    unsafe {
        for i in 0..16 {
            if let Some(callback) = TIMER_CALLBACKS[i] {
                callback();
            }
        }
    }
}

/// Register timer callback
pub fn register_callback(index: usize, callback: TimerCallback) -> bool {
    if index < 16 {
        unsafe {
            TIMER_CALLBACKS[index] = Some(callback);
            true
        }
    } else {
        false
    }
}

/// Unregister timer callback
pub fn unregister_callback(index: usize) {
    if index < 16 {
        unsafe {
            TIMER_CALLBACKS[index] = None;
        }
    }
}

/// Sleep for specified milliseconds
pub fn sleep_ms(ms: u64) {
    let start = get_milliseconds();
    while get_milliseconds() - start < ms {
        // Busy wait (TODO: Use sleep scheduling when scheduler is available)
        unsafe { asm!("hlt"); }
    }
}

/// Sleep for specified seconds
pub fn sleep(seconds: u64) {
    sleep_ms(seconds * 1000);
}

/// Read byte from I/O port
unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

/// Read byte from I/O port
unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", inlateout("dx") port => result, options(nomem, nostack));
    result
}

/// Read current PIT count (for precise timing)
pub fn read_pit_count() -> u16 {
    unsafe {
        // Latch count
        outb(PIT_COMMAND, PIT_CHANNEL0_SELECT | PIT_LATCH_COUNT);
        
        // Read low byte
        let low = inb(PIT_CHANNEL0);
        
        // Read high byte
        let high = inb(PIT_CHANNEL0);
        
        ((high as u16) << 8) | (low as u16)
    }
}

/// Get microseconds since boot (approximate)
pub fn get_microseconds() -> u64 {
    let ticks = get_ticks();
    let pit_count = read_pit_count();
    
    // Calculate microseconds based on ticks and current PIT count
    let ticks_us = ticks * (1_000_000 / TIMER_FREQUENCY);
    let count_us = ((PIT_DIVISOR - pit_count as u64) * 1_000_000) / PIT_FREQUENCY;
    
    ticks_us + count_us
}

/// Performance counter for high-resolution timing
pub struct PerformanceCounter {
    start_ticks: u64,
}

impl PerformanceCounter {
    /// Create a new performance counter
    pub fn new() -> Self {
        PerformanceCounter {
            start_ticks: get_ticks(),
        }
    }

    /// Get elapsed milliseconds
    pub fn elapsed_ms(&self) -> u64 {
        (get_ticks() - self.start_ticks) * (1000 / TIMER_FREQUENCY)
    }

    /// Get elapsed microseconds
    pub fn elapsed_us(&self) -> u64 {
        let current_us = get_microseconds();
        let start_us = self.start_ticks * (1_000_000 / TIMER_FREQUENCY);
        current_us - start_us
    }

    /// Reset the counter
    pub fn reset(&mut self) {
        self.start_ticks = get_ticks();
    }
}

impl Default for PerformanceCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// CPU timestamp counter (RDTSC)
pub fn rdtsc() -> u64 {
    let high: u32;
    let low: u32;
    unsafe {
        asm!("rdtsc", lateout("eax") low, lateout("edx") high);
    }
    ((high as u64) << 32) | (low as u64)
}

/// Get CPU frequency in Hz (approximate)
pub fn get_cpu_frequency() -> u64 {
    // Measure CPU cycles over a known time period
    let start = rdtsc();
    sleep_ms(100); // Sleep for 100ms
    let end = rdtsc();
    
    let cycles = end - start;
    let seconds = 100; // 100ms = 0.1s
    
    cycles * 10 // Multiply by 10 to get cycles per second
}

/// Calibrate timer against CPU timestamp counter
pub fn calibrate_timer() {
    let cpu_freq = get_cpu_frequency();
    // TODO: Use CPU frequency to adjust timer if needed
    let _ = cpu_freq;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pit_frequency() {
        assert_eq!(PIT_FREQUENCY, 1_193_182);
    }

    #[test]
    fn test_timer_frequency() {
        assert_eq!(TIMER_FREQUENCY, 1000);
    }

    #[test]
    fn test_pit_divisor() {
        assert_eq!(PIT_DIVISOR, 1193);
    }

    #[test]
    fn test_performance_counter() {
        let counter = PerformanceCounter::new();
        sleep_ms(10);
        let elapsed = counter.elapsed_ms();
        assert!(elapsed >= 10);
        assert!(elapsed < 20); // Should be close to 10ms
    }

    #[test]
    fn test_rdtsc() {
        let tsc1 = rdtsc();
        let tsc2 = rdtsc();
        assert!(tsc2 > tsc1);
    }

    #[test]
    fn test_ticks_increment() {
        let ticks1 = get_ticks();
        sleep_ms(10);
        let ticks2 = get_ticks();
        assert!(ticks2 > ticks1);
    }
}