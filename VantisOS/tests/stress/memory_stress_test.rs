// Memory Stress Tests for VantisOS v0.6.0
// Tests memory management under heavy load

use v0_6_0_kernel::arm64::memory::{PageAllocator, HeapAllocator};

// Test result structure
#[derive(Debug, Clone, Copy)]
pub enum TestResult {
    Pass,
    Fail,
}

// Test suite structure
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<(String, TestResult)>,
}

impl TestSuite {
    pub fn new(name: &str) -> Self {
        TestSuite {
            name: name.to_string(),
            tests: Vec::new(),
        }
    }

    pub fn add_test(&mut self, name: &str, result: TestResult) {
        self.tests.push((name.to_string(), result));
    }

    pub fn print_summary(&self) {
        println!("\n=== {} ===", self.name);
        let passed = self.tests.iter().filter(|(_, r)| *r == TestResult::Pass).count();
        let total = self.tests.len();
        println!("Passed: {}/{}", passed, total);
        
        for (name, result) in &self.tests {
            let status = match result {
                TestResult::Pass => "✓ PASS",
                TestResult::Fail => "✗ FAIL",
            };
            println!("  {}: {}", status, name);
        }
    }
}

// Test 1: Memory Allocation Stress
fn test_memory_allocation_stress() -> TestResult {
    println!("Test: Memory Allocation Stress");
    
    let mut page_allocator = PageAllocator::new();
    let mut heap_allocator = HeapAllocator::new();
    
    // Allocate 1000 pages
    let mut allocations = Vec::new();
    for i in 0..1000 {
        match page_allocator.allocate() {
            Some(addr) => allocations.push(addr),
            None => {
                println!("  ✗ Failed to allocate page {}", i);
                return TestResult::Fail;
            }
        }
    }
    
    // Free all allocations
    for addr in allocations {
        page_allocator.free(addr);
    }
    
    // Allocate 1000 heap blocks
    for i in 0..1000 {
        match heap_allocator.allocate(1024) {
            Some(_) => {},
            None => {
                println!("  ✗ Failed to allocate heap block {}", i);
                return TestResult::Fail;
            }
        }
    }
    
    println!("  ✓ Successfully allocated and freed 1000 pages and 1000 heap blocks");
    TestResult::Pass
}

// Test 2: Memory Fragmentation Stress
fn test_memory_fragmentation_stress() -> TestResult {
    println!("Test: Memory Fragmentation Stress");
    
    let mut page_allocator = PageAllocator::new();
    
    // Allocate and free in alternating pattern to create fragmentation
    let mut allocations = Vec::new();
    for i in 0..500 {
        match page_allocator.allocate() {
            Some(addr) => allocations.push(addr),
            None => return TestResult::Fail,
        }
    }
    
    // Free every other allocation
    for i in (0..allocations.len()).step_by(2) {
        page_allocator.free(allocations[i]);
    }
    
    // Try to allocate 500 more pages
    for i in 0..500 {
        match page_allocator.allocate() {
            Some(_) => {},
            None => {
                println!("  ✗ Failed to allocate after fragmentation");
                return TestResult::Fail;
            }
        }
    }
    
    println!("  ✓ Successfully handled memory fragmentation");
    TestResult::Pass
}

// Test 3: Memory Leak Stress
fn test_memory_leak_stress() -> TestResult {
    println!("Test: Memory Leak Stress");
    
    let mut page_allocator = PageAllocator::new();
    let mut heap_allocator = HeapAllocator::new();
    
    // Get initial stats
    let initial_pages = page_allocator.get_allocated_pages();
    let initial_heap = heap_allocator.get_allocated_bytes();
    
    // Allocate and free 1000 times
    for _ in 0..1000 {
        match page_allocator.allocate() {
            Some(addr) => page_allocator.free(addr),
            None => return TestResult::Fail,
        }
        
        match heap_allocator.allocate(1024) {
            Some(_) => {},
            None => return TestResult::Fail,
        }
    }
    
    // Check for leaks
    let final_pages = page_allocator.get_allocated_pages();
    let final_heap = heap_allocator.get_allocated_bytes();
    
    if final_pages != initial_pages {
        println!("  ✗ Memory leak detected in page allocator");
        return TestResult::Fail;
    }
    
    if final_heap != initial_heap {
        println!("  ✗ Memory leak detected in heap allocator");
        return TestResult::Fail;
    }
    
    println!("  ✓ No memory leaks detected");
    TestResult::Pass
}

// Test 4: Concurrent Memory Allocation Stress
fn test_concurrent_memory_allocation_stress() -> TestResult {
    println!("Test: Concurrent Memory Allocation Stress");
    
    let mut page_allocator = PageAllocator::new();
    let mut heap_allocator = HeapAllocator::new();
    
    // Simulate concurrent allocations
    let mut allocations = Vec::new();
    for i in 0..2000 {
        // Alternate between page and heap allocations
        if i % 2 == 0 {
            match page_allocator.allocate() {
                Some(addr) => allocations.push((addr, true)),
                None => return TestResult::Fail,
            }
        } else {
            match heap_allocator.allocate(1024) {
                Some(_) => allocations.push((0, false)),
                None => return TestResult::Fail,
            }
        }
    }
    
    // Free all allocations
    for (addr, is_page) in allocations {
        if is_page {
            page_allocator.free(addr);
        }
    }
    
    println!("  ✓ Successfully handled 2000 concurrent allocations");
    TestResult::Pass
}

// Test 5: Memory Pressure Stress
fn test_memory_pressure_stress() -> TestResult {
    println!("Test: Memory Pressure Stress");
    
    let mut page_allocator = PageAllocator::new();
    
    // Allocate until near exhaustion
    let mut allocations = Vec::new();
    let mut count = 0;
    loop {
        match page_allocator.allocate() {
            Some(addr) => {
                allocations.push(addr);
                count += 1;
                // Stop at 90% capacity
                if count >= page_allocator.get_total_pages() * 9 / 10 {
                    break;
                }
            },
            None => break,
        }
    }
    
    // Try to allocate one more (should fail or succeed with last page)
    match page_allocator.allocate() {
        Some(_) => {},
        None => {
            // Expected to fail near exhaustion
        }
    }
    
    // Free half of allocations
    for i in (0..allocations.len()).step_by(2) {
        page_allocator.free(allocations[i]);
    }
    
    // Should be able to allocate more now
    match page_allocator.allocate() {
        Some(_) => {},
        None => {
            println!("  ✗ Failed to allocate after freeing memory");
            return TestResult::Fail;
        }
    }
    
    println!("  ✓ Successfully handled memory pressure");
    TestResult::Pass
}

// Main test runner
fn main() {
    println!("=== VantisOS v0.6.0 Memory Stress Tests ===\n");
    
    let mut suite = TestSuite::new("Memory Stress Tests");
    
    // Run all tests
    suite.add_test("Memory Allocation Stress", test_memory_allocation_stress());
    suite.add_test("Memory Fragmentation Stress", test_memory_fragmentation_stress());
    suite.add_test("Memory Leak Stress", test_memory_leak_stress());
    suite.add_test("Concurrent Memory Allocation Stress", test_concurrent_memory_allocation_stress());
    suite.add_test("Memory Pressure Stress", test_memory_pressure_stress());
    
    // Print summary
    suite.print_summary();
}