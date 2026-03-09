// VantisOS v0.5.0 - Performance Profiling
// Day 13: Performance Optimization

#![allow(unused_unsafe)]

use crate::vga_console::{write_string, write_dec, write_hex32};

// Performance counters
#[derive(Clone, Copy)]
pub struct PerformanceCounters {
    pub boot_time: u64,
    pub memory_allocations: u64,
    pub memory_allocation_time: u64,
    pub interrupt_count: u64,
    pub interrupt_time: u64,
    pub console_chars: u64,
    pub console_time: u64,
}

static mut COUNTERS: PerformanceCounters = PerformanceCounters {
    boot_time: 0,
    memory_allocations: 0,
    memory_allocation_time: 0,
    interrupt_count: 0,
    interrupt_time: 0,
    console_chars: 0,
    console_time: 0,
};

// Get performance counters
pub fn get_counters() -> PerformanceCounters {
    unsafe { COUNTERS }
}

// Reset performance counters
pub fn reset_counters() {
    unsafe {
        COUNTERS = PerformanceCounters {
            boot_time: 0,
            memory_allocations: 0,
            memory_allocation_time: 0,
            interrupt_count: 0,
            interrupt_time: 0,
            console_chars: 0,
            console_time: 0,
        };
    }
}

// Record boot time
pub fn record_boot_time(time: u64) {
    unsafe {
        COUNTERS.boot_time = time;
    }
}

// Record memory allocation
pub fn record_memory_allocation(time: u64) {
    unsafe {
        COUNTERS.memory_allocations += 1;
        COUNTERS.memory_allocation_time += time;
    }
}

// Record interrupt
pub fn record_interrupt(time: u64) {
    unsafe {
        COUNTERS.interrupt_count += 1;
        COUNTERS.interrupt_time += time;
    }
}

// Record console output
pub fn record_console_output(chars: u64, time: u64) {
    unsafe {
        COUNTERS.console_chars += chars;
        COUNTERS.console_time += time;
    }
}

// Display performance statistics
pub fn display_performance_stats() {
    unsafe {
        let counters = COUNTERS;
        
        write_string("\n=== Performance Statistics ===\n");
        
        write_string("Boot Time: ");
        write_dec(counters.boot_time);
        write_string(" ms\n");
        
        write_string("Memory Allocations: ");
        write_dec(counters.memory_allocations);
        write_string("\n");
        
        if counters.memory_allocations > 0 {
            write_string("Avg Memory Allocation Time: ");
            write_dec(counters.memory_allocation_time / counters.memory_allocations);
            write_string(" μs\n");
        }
        
        write_string("Interrupts: ");
        write_dec(counters.interrupt_count);
        write_string("\n");
        
        if counters.interrupt_count > 0 {
            write_string("Avg Interrupt Time: ");
            write_dec(counters.interrupt_time / counters.interrupt_count);
            write_string(" μs\n");
        }
        
        write_string("Console Characters: ");
        write_dec(counters.console_chars);
        write_string("\n");
        
        if counters.console_chars > 0 {
            write_string("Avg Console Time: ");
            write_dec(counters.console_time / counters.console_chars);
            write_string(" μs\n");
        }
        
        write_string("\n");
    }
}

// Simple timestamp function (using RDTSC)
#[inline(always)]
pub fn rdtsc() -> u64 {
    unsafe {
        let mut high: u32;
        let mut low: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") low,
            out("edx") high,
            options(nomem, nostack)
        );
        ((high as u64) << 32) | (low as u64)
    }
}

// Convert RDTSC cycles to microseconds (assuming 2.5 GHz CPU)
#[inline(always)]
pub fn cycles_to_us(cycles: u64) -> u64 {
    cycles / 2500
}

// Convert RDTSC cycles to milliseconds
#[inline(always)]
pub fn cycles_to_ms(cycles: u64) -> u64 {
    cycles / 2500000
}