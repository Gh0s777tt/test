#!/bin/bash

# Create lib.rs files for all modules

# Kernel modules
for module in allocator process ipc scheduler syscall; do
    cat > kernel/$module/src/lib.rs << 'RUST'
//! VantisOS Kernel Module
//!
//! This module is part of the VantisOS kernel.

#![no_std]
#![allow(dead_code)]

#[cfg(feature = "verus-full")]
use builtin::*;
#[cfg(feature = "verus-full")]
use builtin_macros::*;
#[cfg(feature = "verus-full")]
use vstd::prelude::*;

pub mod verus_shim;

// Re-export main types
pub use self::main::*;
RUST

    # Create main.rs placeholder
    cat > kernel/$module/src/main.rs << 'RUST'
// Main module implementation
// This file will be populated with the actual implementation
RUST

    echo "Created kernel/$module/src/lib.rs"
done

# Create placeholder lib.rs for userspace modules
find userspace -type d -mindepth 2 -maxdepth 2 | while read dir; do
    # Skip if already has lib.rs
    if [ ! -f "$dir/src/lib.rs" ]; then
        cat > "$dir/src/lib.rs" << 'RUST'
//! VantisOS User Space Module
//!
//! This module is part of the VantisOS user space services.

#![no_std]
#![allow(dead_code)]

#[cfg(feature = "verus-full")]
use builtin::*;
#[cfg(feature = "verus-full")]
use builtin_macros::*;
#[cfg(feature = "verus-full")]
use vstd::prelude::*;

pub mod verus_shim;

// Re-export main types
pub use self::main::*;
RUST

        # Create main.rs placeholder
        cat > "$dir/src/main.rs" << 'RUST'
// Main module implementation
// This file will be populated with the actual implementation
RUST

        echo "Created $dir/src/lib.rs"
    fi
done

echo "All lib.rs files created successfully!"
