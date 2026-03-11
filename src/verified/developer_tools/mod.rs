//! Developer Tools for VANTIS OS v1.6.0
//!
//! This module provides kernel-level developer tooling including:
//! - Performance profiler with hierarchical span tracking
//! - Interactive kernel debugger with breakpoint management
//! - Build system configuration and dependency management

pub mod profiler;
pub mod debugger;
pub mod build_system;