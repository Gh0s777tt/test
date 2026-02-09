//! Verus Shim - Conditional Verus Support
//! 
//! This module provides conditional compilation support for Verus verification.
//! When the `verus` feature is disabled, it provides no-op replacements.

#[cfg(feature = "verus")]
pub use builtin::*;
#[cfg(feature = "verus")]
pub use builtin_macros::*;
#[cfg(feature = "verus")]
pub use vstd::prelude::*;

// When verus feature is disabled, provide empty macros
#[cfg(not(feature = "verus"))]
#[macro_export]
macro_rules! verus {
    ($($tt:tt)*) => {};
}

#[cfg(not(feature = "verus"))]
pub use verus;