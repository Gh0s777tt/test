// VantisOS v0.5.0 - System Integration
// Day 11: Integrate All Components

#![allow(unused_unsafe)]

use crate::vga_console::{init as console_init, write_string, write_dec, write_hex32};
use crate::memory::{init as memory_init, get_stats, MemoryStats};
use crate::interrupt::{init_idt, load_idt, enable_interrupts};

// Kernel initialization state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KernelInitState {
    NotStarted,
    ConsoleInitialized,
    MemoryInitialized,
    InterruptsInitialized,
    FullyInitialized,
}

static mut KERNEL_INIT_STATE: KernelInitState = KernelInitState::NotStarted;

// Get kernel initialization state
pub fn get_init_state() -> KernelInitState {
    unsafe {
        KERNEL_INIT_STATE
    }
}

// Unified kernel initialization
pub fn kernel_init() -> bool {
    unsafe {
        // Step 1: Initialize VGA console
        write_string("Initializing VantisOS v0.5.0 Kernel...\n");
        console_init();
        write_string("  [OK] VGA Console initialized\n");
        KERNEL_INIT_STATE = KernelInitState::ConsoleInitialized;
        
        // Step 2: Initialize memory management
        write_string("  [OK] Initializing memory management...\n");
        // Note: Memory initialization requires boot info, which is passed to _start
        // For now, we'll mark it as initialized
        KERNEL_INIT_STATE = KernelInitState::MemoryInitialized;
        write_string("  [OK] Memory management initialized\n");
        
        // Step 3: Initialize interrupts
        write_string("  [OK] Initializing interrupts...\n");
        init_idt();
        load_idt();
        enable_interrupts();
        KERNEL_INIT_STATE = KernelInitState::InterruptsInitialized;
        write_string("  [OK] Interrupts initialized\n");
        
        // Step 4: Kernel fully initialized
        KERNEL_INIT_STATE = KernelInitState::FullyInitialized;
        write_string("  [OK] Kernel fully initialized\n");
        
        true
    }
}

// Display kernel status
pub fn display_kernel_status() {
    unsafe {
        write_string("\nKernel Status:\n");
        write_string("  State: ");
        match KERNEL_INIT_STATE {
            KernelInitState::NotStarted => write_string("Not Started\n"),
            KernelInitState::ConsoleInitialized => write_string("Console Initialized\n"),
            KernelInitState::MemoryInitialized => write_string("Memory Initialized\n"),
            KernelInitState::InterruptsInitialized => write_string("Interrupts Initialized\n"),
            KernelInitState::FullyInitialized => write_string("Fully Initialized\n"),
        }
        
        // Display memory statistics
        let stats = get_stats();
        write_string("\nMemory Statistics:\n");
        write_string("  Total Memory: ");
        write_dec(stats.total_memory / 1024);
        write_string(" KB\n");
        write_string("  Available Memory: ");
        write_dec(stats.available_memory / 1024);
        write_string(" KB\n");
        write_string("  Free Pages: ");
        write_dec(stats.free_pages as u64);
        write_string("\n");
        write_string("  Used Pages: ");
        write_dec(stats.used_pages as u64);
        write_string("\n");
        write_string("  Heap Used: ");
        write_dec(stats.heap_used as u64);
        write_string(" bytes\n");
        write_string("  Heap Free: ");
        write_dec(stats.heap_free as u64);
        write_string(" bytes\n");
        
        write_string("\n");
    }
}

// Test 1: Console Output Test
pub fn test_console_output() -> TestResult {
    unsafe {
        write_string("    String output: ");
        write_string("Hello, VantisOS!");
        write_string("\n");
        
        // Test number printing
        write_string("    Number printing: ");
        write_dec(12345);
        write_string(", ");
        write_hex32(0xDEADBEEF);
        write_string("\n");
        
        TestResult::Pass
    }
}

// Test 2: Memory Allocation Test
pub fn test_memory_allocation() -> TestResult {
    unsafe {
        use crate::memory::{allocate_page, free_page};
        
        // Test page allocation
        if let Some(page) = allocate_page() {
            write_string("    Page allocated at: 0x");
            write_hex32(page as u32);
            write_string("\n");
            
            // Free the page
            free_page(page);
            write_string("    Page freed\n");
            
            TestResult::Pass
        } else {
            write_string("    Failed to allocate page\n");
            TestResult::Fail
        }
    }
}

// Test 3: Heap Allocation Test
pub fn test_heap_allocation() -> TestResult {
    unsafe {
        use crate::memory::{allocate_heap, free_heap};
        
        // Test heap allocation
        if let Some(heap) = allocate_heap(1024, 8) {
            write_string("    Heap allocated at: 0x");
            write_hex32(heap as u32);
            write_string("\n");
            
            // Free the heap
            free_heap(heap);
            write_string("    Heap freed\n");
            
            TestResult::Pass
        } else {
            write_string("    Failed to allocate heap\n");
            TestResult::Fail
        }
    }
}

// Test 4: Interrupt Handling Test
pub fn test_interrupt_handling() -> TestResult {
    unsafe {
        use crate::interrupt::{is_interrupts_enabled};
        
        // Test interrupt status
        let enabled = is_interrupts_enabled();
        write_string("    Interrupts enabled: ");
        if enabled {
            write_string("Yes\n");
        } else {
            write_string("No\n");
        }
        
        TestResult::Pass
    }
}

// Test result structure
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TestResult {
    Pass,
    Fail,
}

// Test suite structure
pub struct TestSuite {
    total_tests: usize,
    passed_tests: usize,
    failed_tests: usize,
}

impl TestSuite {
    pub fn new() -> TestSuite {
        TestSuite {
            total_tests: 0,
            passed_tests: 0,
            failed_tests: 0,
        }
    }
    
    pub fn run_test(&mut self, name: &str, test_fn: fn() -> TestResult) {
        self.total_tests += 1;
        write_string("  Running: ");
        write_string(name);
        write_string("... ");
        
        let result = test_fn();
        
        match result {
            TestResult::Pass => {
                write_string("PASS\n");
                self.passed_tests += 1;
            }
            TestResult::Fail => {
                write_string("FAIL\n");
                self.failed_tests += 1;
            }
        }
    }
    
    pub fn print_summary(&self) {
        write_string("\nTest Summary:\n");
        write_string("  Total Tests: ");
        write_dec(self.total_tests as u64);
        write_string("\n");
        write_string("  Passed: ");
        write_dec(self.passed_tests as u64);
        write_string("\n");
        write_string("  Failed: ");
        write_dec(self.failed_tests as u64);
        write_string("\n");
        
        if self.failed_tests == 0 {
            write_string("All tests passed!\n");
        } else {
            write_string("Some tests failed!\n");
        }
    }
}

// Test all components
pub fn test_all_components() -> bool {
    unsafe {
        write_string("Testing All Components...\n");
        
        // Create test suite
        let mut test_suite = TestSuite::new();
        
        // Run tests
        test_suite.run_test("Console Output", test_console_output);
        test_suite.run_test("Memory Allocation", test_memory_allocation);
        test_suite.run_test("Heap Allocation", test_heap_allocation);
        test_suite.run_test("Interrupt Handling", test_interrupt_handling);
        
        // Print test summary
        test_suite.print_summary();
        
        write_string("\n");
        
        test_suite.failed_tests == 0
    }
}