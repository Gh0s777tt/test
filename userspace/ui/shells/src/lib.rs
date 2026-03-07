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

pub mod implementation;

// Re-export main types
pub use self::implementation::*;
