//! # Calculator Application
//!
//! A comprehensive scientific calculator application for VantisOS with support for
//! basic arithmetic, scientific functions, memory operations, and history.

use serde::{Deserialize, Serialize};

/// Calculator operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    SquareRoot,
    Sine,
    Cosine,
    Tangent,
    Logarithm,
    NaturalLog,
    Exponential,
    Factorial,
    Absolute,
}

/// Calculator state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calculator {
    pub display: String,
    pub current_value: f64,
    pub previous_value: f64,
    pub operation: Option<Operation>,
    pub memory: f64,
    pub history: Vec<HistoryEntry>,
    pub new_entry: bool,
}

/// History entry for calculator operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub expression: String,
    pub result: f64,
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculator {
    /// Create a new calculator instance
    pub fn new() -> Self {
        Calculator {
            display: "0".to_string(),
            current_value: 0.0,
            previous_value: 0.0,
            operation: None,
            memory: 0.0,
            history: Vec::new(),
            new_entry: true,
        }
    }

    /// Enter a digit
    pub fn input_digit(&mut self, digit: u8) {
        if self.new_entry {
            self.display = digit.to_string();
            self.new_entry = false;
        } else {
            if self.display != "0" || digit != 0 {
                self.display.push_str(&digit.to_string());
            }
        }
        self.update_current_value();
    }

    /// Enter a decimal point
    pub fn input_decimal(&mut self) {
        if self.new_entry {
            self.display = "0.".to_string();
            self.new_entry = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
        self.update_current_value();
    }

    /// Clear the display
    pub fn clear(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
        self.operation = None;
        self.new_entry = true;
    }

    /// Clear everything including history
    pub fn clear_all(&mut self) {
        self.clear();
        self.previous_value = 0.0;
        self.history.clear();
    }

    /// Perform a calculation
    pub fn calculate(&mut self) -> Result<f64, String> {
        if let Some(op) = self.operation.take() {
            let result = match op {
                Operation::Add => self.previous_value + self.current_value,
                Operation::Subtract => self.previous_value - self.current_value,
                Operation::Multiply => self.previous_value * self.current_value,
                Operation::Divide => {
                    if self.current_value == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    self.previous_value / self.current_value
                }
                Operation::Modulo => {
                    if self.current_value == 0.0 {
                        return Err("Modulo by zero".to_string());
                    }
                    self.previous_value % self.current_value
                }
                Operation::Power => self.previous_value.powf(self.current_value),
                _ => self.current_value,
            };

            self.add_to_history(op, result);
            self.display = Self::format_number(result);
            self.current_value = result;
            self.new_entry = true;
            Ok(result)
        } else {
            Ok(self.current_value)
        }
    }

    /// Perform a scientific operation
    pub fn scientific_operation(&mut self, op: Operation) -> Result<f64, String> {
        let result = match op {
            Operation::SquareRoot => {
                if self.current_value < 0.0 {
                    return Err("Square root of negative number".to_string());
                }
                self.current_value.sqrt()
            }
            Operation::Sine => self.current_value.sin(),
            Operation::Cosine => self.current_value.cos(),
            Operation::Tangent => {
                if self.current_value.cos() == 0.0 {
                    return Err("Tangent undefined".to_string());
                }
                self.current_value.tan()
            }
            Operation::Logarithm => {
                if self.current_value <= 0.0 {
                    return Err("Logarithm of non-positive number".to_string());
                }
                self.current_value.log10()
            }
            Operation::NaturalLog => {
                if self.current_value <= 0.0 {
                    return Err("Natural log of non-positive number".to_string());
                }
                self.current_value.ln()
            }
            Operation::Exponential => self.current_value.exp(),
            Operation::Factorial => {
                if self.current_value < 0.0 || self.current_value.fract() != 0.0 {
                    return Err("Factorial requires non-negative integer".to_string());
                }
                Self::factorial(self.current_value as u64)
            }
            Operation::Absolute => self.current_value.abs(),
            _ => return Err("Invalid operation".to_string()),
        };

        self.display = Self::format_number(result);
        self.current_value = result;
        self.new_entry = true;
        Ok(result)
    }

    /// Set an operation
    pub fn set_operation(&mut self, op: Operation) {
        if self.new_entry {
            self.previous_value = self.current_value;
        } else {
            if let Ok(_) = self.calculate() {
                self.previous_value = self.current_value;
            }
        }
        self.operation = Some(op);
        self.new_entry = true;
    }

    /// Memory store
    pub fn memory_store(&mut self) {
        self.memory = self.current_value;
    }

    /// Memory recall
    pub fn memory_recall(&mut self) {
        self.current_value = self.memory;
        self.display = Self::format_number(self.memory);
        self.new_entry = true;
    }

    /// Memory clear
    pub fn memory_clear(&mut self) {
        self.memory = 0.0;
    }

    /// Memory add
    pub fn memory_add(&mut self) {
        self.memory += self.current_value;
    }

    /// Memory subtract
    pub fn memory_subtract(&mut self) {
        self.memory -= self.current_value;
    }

    /// Get history
    pub fn get_history(&self) -> &[HistoryEntry] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Update current value from display
    fn update_current_value(&mut self) {
        self.current_value = self.display.parse().unwrap_or(0.0);
    }

    /// Format number for display
    fn format_number(num: f64) -> String {
        if num.fract() == 0.0 {
            num.to_string()
        } else {
            format!("{:.10}", num).trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }

    /// Calculate factorial
    fn factorial(n: u64) -> f64 {
        match n {
            0 | 1 => 1.0,
            _ => n as f64 * Self::factorial(n - 1),
        }
    }

    /// Add entry to history
    fn add_to_history(&mut self, op: Operation, result: f64) {
        let op_symbol = match op {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "×",
            Operation::Divide => "÷",
            Operation::Modulo => "%",
            Operation::Power => "^",
            _ => "?",
        };
        
        let expression = format!("{} {} {} = {}", 
            Self::format_number(self.previous_value),
            op_symbol,
            Self::format_number(self.current_value),
            Self::format_number(result)
        );
        
        self.history.push(HistoryEntry { expression, result });
        
        // Keep only last 100 entries
        if self.history.len() > 100 {
            self.history.remove(0);
        }
    }

    /// Calculate percentage
    pub fn percentage(&mut self) {
        self.current_value = self.current_value / 100.0;
        self.display = Self::format_number(self.current_value);
    }

    /// Toggle positive/negative
    pub fn toggle_sign(&mut self) {
        self.current_value = -self.current_value;
        self.display = Self::format_number(self.current_value);
    }

    /// Backspace
    pub fn backspace(&mut self) {
        if !self.new_entry && self.display.len() > 1 {
            self.display.pop();
            self.update_current_value();
        } else {
            self.display = "0".to_string();
            self.current_value = 0.0;
            self.new_entry = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_creation() {
        let calc = Calculator::new();
        assert_eq!(calc.display, "0");
        assert_eq!(calc.current_value, 0.0);
    }

    #[test]
    fn test_digit_input() {
        let mut calc = Calculator::new();
        calc.input_digit(1);
        calc.input_digit(2);
        calc.input_digit(3);
        assert_eq!(calc.display, "123");
    }

    #[test]
    fn test_addition() {
        let mut calc = Calculator::new();
        calc.input_digit(5);
        calc.set_operation(Operation::Add);
        calc.input_digit(3);
        calc.calculate().unwrap();
        assert_eq!(calc.current_value, 8.0);
    }

    #[test]
    fn test_division_by_zero() {
        let mut calc = Calculator::new();
        calc.input_digit(5);
        calc.set_operation(Operation::Divide);
        calc.input_digit(0);
        assert!(calc.calculate().is_err());
    }

    #[test]
    fn test_memory_operations() {
        let mut calc = Calculator::new();
        calc.input_digit(4);
        calc.input_digit(2);
        calc.memory_store();
        calc.clear();
        calc.memory_recall();
        assert_eq!(calc.current_value, 42.0);
    }

    #[test]
    fn test_percentage() {
        let mut calc = Calculator::new();
        calc.input_digit(5);
        calc.input_digit(0);
        calc.percentage();
        assert!((calc.current_value - 0.5).abs() < 0.01);
    }
}