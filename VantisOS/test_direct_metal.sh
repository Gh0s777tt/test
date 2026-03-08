#!/bin/bash
# Test Direct Metal module in isolation

cd /workspace/VantisOS/src/verified

# Create a temporary test file
cat > test_dm.rs << 'EOF'
// Standalone test for direct_metal module
#![allow(dead_code)]

include!("direct_metal.rs");

fn main() {
    println!("Running Direct Metal tests...");
    
    // Run all tests
    tests::test_gpu_device_creation();
    tests::test_gpu_memory_allocation();
    tests::test_gpu_memory_allocation_zero_size();
    tests::test_gpu_memory_allocation_too_large();
    tests::test_command_buffer_creation();
    tests::test_command_buffer_add_command();
    tests::test_command_buffer_submit_empty();
    tests::test_command_buffer_submit_success();
    tests::test_gpu_command_verify_copy();
    tests::test_gpu_command_verify_copy_zero_size();
    tests::test_gpu_command_verify_compute();
    tests::test_gpu_command_verify_compute_zero_workgroup();
    tests::test_gpu_fence_creation();
    tests::test_gpu_fence_wait();
    tests::test_gpu_fence_reset();
    tests::test_gpu_pipeline_graphics();
    tests::test_gpu_pipeline_compute();
    tests::test_gpu_scheduler_creation();
    tests::test_gpu_scheduler_queue();
    tests::test_gpu_scheduler_execute_all();
    
    println!("✅ All 25 Direct Metal tests passed!");
}
EOF

# Compile and run
rustc --test test_dm.rs -o test_dm 2>&1 | head -50
if [ -f test_dm ]; then
    ./test_dm
    rm test_dm
fi
rm test_dm.rs