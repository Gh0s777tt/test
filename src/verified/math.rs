//! Formally verified mathematical operations
//! 
//! This module provides mathematically proven implementations of
//! basic arithmetic operations with overflow protection.

#[cfg(feature = "verus-full")]
use builtin::*;
#[cfg(feature = "verus-full")]
use builtin_macros::*;
#[cfg(feature = "verus-full")]
use vstd::prelude::*;

/// Safely add two u32 numbers with overflow checking
/// 
/// # Formal Specification (Verus)
/// - Precondition: a + b <= u32::MAX
/// - Postcondition: result == a + b
/// 
/// # Examples
/// 
/// ```
/// use vantis::verified::math::safe_add;
/// 
/// let result = safe_add(10, 20);
/// assert_eq!(result, 30);
/// ```
#[cfg(feature = "verus-full")]
verus! {
    pub fn safe_add(a: u32, b: u32) -> (result: u32)
        requires a as u64 + b as u64 <= u32::MAX as u64,
        ensures result == a + b,
    {
        a + b
    }
}

#[cfg(not(feature = "verus-full"))]
pub fn safe_add(a: u32, b: u32) -> u32 {
    a.checked_add(b).expect("Addition overflow")
}

/// Safely subtract two u32 numbers with underflow checking
/// 
/// # Formal Specification (Verus)
/// - Precondition: a >= b
/// - Postcondition: result == a - b
#[cfg(feature = "verus-full")]
verus! {
    pub fn safe_sub(a: u32, b: u32) -> (result: u32)
        requires a >= b,
        ensures result == a - b,
    {
        a - b
    }
}

#[cfg(not(feature = "verus-full"))]
pub fn safe_sub(a: u32, b: u32) -> u32 {
    a.checked_sub(b).expect("Subtraction underflow")
}

/// Safely multiply two u32 numbers with overflow checking
/// 
/// # Formal Specification (Verus)
/// - Precondition: a * b <= u32::MAX
/// - Postcondition: result == a * b
#[cfg(feature = "verus-full")]
verus! {
    pub fn safe_mul(a: u32, b: u32) -> (result: u32)
        requires a as u64 * b as u64 <= u32::MAX as u64,
        ensures result == a * b,
    {
        a * b
    }
}

#[cfg(not(feature = "verus-full"))]
pub fn safe_mul(a: u32, b: u32) -> u32 {
    a.checked_mul(b).expect("Multiplication overflow")
}

/// Safely divide two u32 numbers with division by zero checking
/// 
/// # Formal Specification (Verus)
/// - Precondition: b != 0
/// - Postcondition: result == a / b
#[cfg(feature = "verus-full")]
verus! {
    pub fn safe_div(a: u32, b: u32) -> (result: u32)
        requires b != 0,
        ensures result == a / b,
    {
        a / b
    }
}

#[cfg(not(feature = "verus-full"))]
pub fn safe_div(a: u32, b: u32) -> u32 {
    a.checked_div(b).expect("Division by zero")
}

/// Find minimum of two numbers
/// 
/// # Formal Specification (Verus)
/// - Postcondition: result <= a && result <= b
/// - Postcondition: result == a || result == b
#[cfg(feature = "verus-full")]
verus! {
    pub fn min(a: u32, b: u32) -> (result: u32)
        ensures 
            result <= a && result <= b,
            result == a || result == b,
    {
        if a < b { a } else { b }
    }
}

#[cfg(not(feature = "verus-full"))]
pub fn min(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

/// Find maximum of two numbers
/// 
/// # Formal Specification (Verus)
/// - Postcondition: result >= a && result >= b
/// - Postcondition: result == a || result == b
#[cfg(feature = "verus-full")]
verus! {
    pub fn max(a: u32, b: u32) -> (result: u32)
        ensures 
            result >= a && result >= b,
            result == a || result == b,
    {
        if a > b { a } else { b }
    }
}

#[cfg(not(feature = "verus-full"))]
pub fn max(a: u32, b: u32) -> u32 {
    if a > b { a } else { b }
}

// Kani verification harnesses
#[cfg(kani)]
mod verification {
    use super::*;
    
    #[kani::proof]
    fn verify_safe_add() {
        let a: u32 = kani::any();
        let b: u32 = kani::any();
        
        // Assume precondition
        kani::assume(a as u64 + b as u64 <= u32::MAX as u64);
        
        // Call function
        let result = safe_add(a, b);
        
        // Assert postcondition
        assert!(result == a.wrapping_add(b));
        assert!(result >= a);
        assert!(result >= b);
    }
    
    #[kani::proof]
    fn verify_safe_sub() {
        let a: u32 = kani::any();
        let b: u32 = kani::any();
        
        kani::assume(a >= b);
        
        let result = safe_sub(a, b);
        
        assert!(result == a.wrapping_sub(b));
        assert!(result <= a);
    }
    
    #[kani::proof]
    fn verify_safe_mul() {
        let a: u32 = kani::any();
        let b: u32 = kani::any();
        
        kani::assume(a as u64 * b as u64 <= u32::MAX as u64);
        
        let result = safe_mul(a, b);
        
        assert!(result == a.wrapping_mul(b));
    }
    
    #[kani::proof]
    fn verify_safe_div() {
        let a: u32 = kani::any();
        let b: u32 = kani::any();
        
        kani::assume(b != 0);
        
        let result = safe_div(a, b);
        
        assert!(result == a / b);
        assert!(result <= a);
    }
    
    #[kani::proof]
    fn verify_min() {
        let a: u32 = kani::any();
        let b: u32 = kani::any();
        
        let result = min(a, b);
        
        assert!(result <= a);
        assert!(result <= b);
        assert!(result == a || result == b);
    }
    
    #[kani::proof]
    fn verify_max() {
        let a: u32 = kani::any();
        let b: u32 = kani::any();
        
        let result = max(a, b);
        
        assert!(result >= a);
        assert!(result >= b);
        assert!(result == a || result == b);
    }
    
    #[kani::proof]
    fn verify_min_max_relationship() {
        let a: u32 = kani::any();
        let b: u32 = kani::any();
        
        let min_val = min(a, b);
        let max_val = max(a, b);
        
        assert!(min_val <= max_val);
        assert!(min_val == a || min_val == b);
        assert!(max_val == a || max_val == b);
    }
}

#[cfg(all(test, feature = "verus-full"))]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_add() {
        assert_eq!(safe_add(10, 20), 30);
        assert_eq!(safe_add(0, 0), 0);
        assert_eq!(safe_add(u32::MAX - 1, 1), u32::MAX);
    }
    
    #[test]
    #[should_panic(expected = "Addition overflow")]
    fn test_safe_add_overflow() {
        safe_add(u32::MAX, 1);
    }
    
    #[test]
    fn test_safe_sub() {
        assert_eq!(safe_sub(20, 10), 10);
        assert_eq!(safe_sub(10, 10), 0);
        assert_eq!(safe_sub(u32::MAX, 1), u32::MAX - 1);
    }
    
    #[test]
    #[should_panic(expected = "Subtraction underflow")]
    fn test_safe_sub_underflow() {
        safe_sub(10, 20);
    }
    
    #[test]
    fn test_safe_mul() {
        assert_eq!(safe_mul(10, 20), 200);
        assert_eq!(safe_mul(0, 100), 0);
        assert_eq!(safe_mul(1, u32::MAX), u32::MAX);
    }
    
    #[test]
    #[should_panic(expected = "Multiplication overflow")]
    fn test_safe_mul_overflow() {
        safe_mul(u32::MAX, 2);
    }
    
    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(20, 10), 2);
        assert_eq!(safe_div(100, 3), 33);
        assert_eq!(safe_div(u32::MAX, 1), u32::MAX);
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_safe_div_by_zero() {
        safe_div(10, 0);
    }
    
    #[test]
    fn test_min() {
        assert_eq!(min(10, 20), 10);
        assert_eq!(min(20, 10), 10);
        assert_eq!(min(10, 10), 10);
        assert_eq!(min(0, u32::MAX), 0);
    }
    
    #[test]
    fn test_max() {
        assert_eq!(max(10, 20), 20);
        assert_eq!(max(20, 10), 20);
        assert_eq!(max(10, 10), 10);
        assert_eq!(max(0, u32::MAX), u32::MAX);
    }
}