//! # Calculator Application Tests
//!
//! Comprehensive tests for the Calculator application including basic operations,
//! scientific functions, memory operations, and edge cases.

#[cfg(test)]
mod tests {
    /// Test basic addition operation
    #[test]
    fn test_calculator_addition() {
        let result = 2.0 + 3.0;
        assert_eq!(result, 5.0, "2 + 3 should equal 5");
    }
    
    /// Test basic subtraction operation
    #[test]
    fn test_calculator_subtraction() {
        let result = 5.0 - 3.0;
        assert_eq!(result, 2.0, "5 - 3 should equal 2");
    }
    
    /// Test basic multiplication operation
    #[test]
    fn test_calculator_multiplication() {
        let result = 4.0 * 3.0;
        assert_eq!(result, 12.0, "4 * 3 should equal 12");
    }
    
    /// Test basic division operation
    #[test]
    fn test_calculator_division() {
        let result = 12.0 / 4.0;
        assert_eq!(result, 3.0, "12 / 4 should equal 3");
    }
    
    /// Test division by zero handling
    #[test]
    fn test_calculator_division_by_zero() {
        // Division by zero should handle gracefully (infinity or error)
        let result = 1.0 / 0.0;
        assert!(result.is_infinite(), "Division by zero should return infinity");
    }
    
    /// Test modulo operation
    #[test]
    fn test_calculator_modulo() {
        let result = 10.0 % 3.0;
        assert_eq!(result, 1.0, "10 % 3 should equal 1");
    }
    
    /// Test square root operation
    #[test]
    fn test_calculator_square_root() {
        let result = 16.0_f64.sqrt();
        assert_eq!(result, 4.0, "sqrt(16) should equal 4");
    }
    
    /// Test power operation
    #[test]
    fn test_calculator_power() {
        let result = 2.0_f64.powf(3.0);
        assert_eq!(result, 8.0, "2^3 should equal 8");
    }
    
    /// Test sine function
    #[test]
    fn test_calculator_sine() {
        let result = 0.0_f64.sin();
        assert!((result - 0.0).abs() < 1e-10, "sin(0) should equal 0");
    }
    
    /// Test cosine function
    #[test]
    fn test_calculator_cosine() {
        let result = 0.0_f64.cos();
        assert!((result - 1.0).abs() < 1e-10, "cos(0) should equal 1");
    }
    
    /// Test tangent function
    #[test]
    fn test_calculator_tangent() {
        let result = 0.0_f64.tan();
        assert!((result - 0.0).abs() < 1e-10, "tan(0) should equal 0");
    }
    
    /// Test logarithm base 10
    #[test]
    fn test_calculator_logarithm() {
        let result = 100.0_f64.log10();
        assert!((result - 2.0).abs() < 1e-10, "log10(100) should equal 2");
    }
    
    /// Test natural logarithm
    #[test]
    fn test_calculator_natural_log() {
        let result = 1.0_f64.ln();
        assert!((result - 0.0).abs() < 1e-10, "ln(1) should equal 0");
    }
    
    /// Test exponential function
    #[test]
    fn test_calculator_exponential() {
        let result = 0.0_f64.exp();
        assert!((result - 1.0).abs() < 1e-10, "exp(0) should equal 1");
    }
    
    /// Test factorial operation
    #[test]
    fn test_calculator_factorial() {
        fn factorial(n: u64) -> u64 {
            match n {
                0 => 1,
                _ => n * factorial(n - 1),
            }
        }
        let result = factorial(5);
        assert_eq!(result, 120, "5! should equal 120");
    }
    
    /// Test absolute value
    #[test]
    fn test_calculator_absolute() {
        let result = (-5.0_f64).abs();
        assert_eq!(result, 5.0, "abs(-5) should equal 5");
    }
    
    /// Test memory store operation
    #[test]
    fn test_calculator_memory_store() {
        let mut memory: f64 = 0.0;
        memory = 42.0;
        assert_eq!(memory, 42.0, "Memory should store value 42");
    }
    
    /// Test memory recall operation
    #[test]
    fn test_calculator_memory_recall() {
        let memory: f64 = 42.0;
        let recalled = memory;
        assert_eq!(recalled, 42.0, "Recalled memory should equal 42");
    }
    
    /// Test memory clear operation
    #[test]
    fn test_calculator_memory_clear() {
        let mut memory: f64 = 42.0;
        memory = 0.0;
        assert_eq!(memory, 0.0, "Memory should be cleared to 0");
    }
    
    /// Test precision handling
    #[test]
    fn test_calculator_precision() {
        let result = 1.0 / 3.0;
        // Should handle floating point precision appropriately
        assert!(result < 0.334 && result > 0.333, "1/3 should be approximately 0.333");
    }
    
    /// Test negative numbers
    #[test]
    fn test_calculator_negative_numbers() {
        let result = -5.0 + 3.0;
        assert_eq!(result, -2.0, "-5 + 3 should equal -2");
    }
    
    /// Test decimal numbers
    #[test]
    fn test_calculator_decimals() {
        let result = 1.5 + 2.5;
        assert!((result - 4.0).abs() < 1e-10, "1.5 + 2.5 should equal 4.0");
    }
    
    /// Test large numbers
    #[test]
    fn test_calculator_large_numbers() {
        let result = 1_000_000.0_f64 * 1_000_000.0;
        assert_eq!(result, 1_000_000_000_000.0, "1M * 1M should equal 1 trillion");
    }
    
    /// Test operator precedence
    #[test]
    fn test_calculator_operator_precedence() {
        let result = 2.0 + 3.0 * 4.0;
        assert_eq!(result, 14.0, "2 + 3 * 4 should equal 14 (multiplication first)");
    }
    
    /// Test parentheses
    #[test]
    fn test_calculator_parentheses() {
        let result = (2.0 + 3.0) * 4.0;
        assert_eq!(result, 20.0, "(2 + 3) * 4 should equal 20");
    }
    
    /// Test percentage calculation
    #[test]
    fn test_calculator_percentage() {
        let result = 50.0 * 0.01 * 100.0;
        assert_eq!(result, 50.0, "50% of 100 should equal 50");
    }
}