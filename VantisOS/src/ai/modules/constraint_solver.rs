//! Constraint Solver for VantisOS Optimization
//!
//! This module provides constraint-based optimization:
//! - Linear programming constraints
//! - Non-linear constraint handling
//! - Constraint satisfaction
//! - Penalty methods
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::constraint_solver::{ConstraintSolver, ConstraintType};
//! use crate::ai::modules::optimization_types::OptimizationConstraint;
//!
//! let mut solver = ConstraintSolver::new();
//! solver.add_constraint(OptimizationConstraint::new("memory".to_string(), ConstraintType::UpperBound, 1024.0));
//! let feasible = solver.check_solution(&solution);
//! ```

use crate::ai::error::{AIServiceError, Result};
use crate::ai::modules::optimization_types::{OptimizationConstraint, ConstraintType};
use std::collections::HashMap;

/// Constraint solver
pub struct ConstraintSolver {
    pub constraints: Vec<OptimizationConstraint>,
    pub penalty_weights: HashMap<String, f64>,
    pub tolerance: f64,
    pub max_violations: usize,
}

impl ConstraintSolver {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            penalty_weights: HashMap::new(),
            tolerance: 1e-6,
            max_violations: 10,
        }
    }

    /// Add a constraint
    pub fn add_constraint(&mut self, constraint: OptimizationConstraint) {
        self.penalty_weights
            .insert(constraint.name.clone(), constraint.penalty_weight);
        self.constraints.push(constraint);
    }

    /// Add multiple constraints
    pub fn add_constraints(&mut self, constraints: Vec<OptimizationConstraint>) {
        for constraint in constraints {
            self.add_constraint(constraint);
        }
    }

    /// Check if a solution satisfies all constraints
    pub fn check_solution(&self, solution: &HashMap<String, f64>) -> bool {
        for constraint in &self.constraints {
            if let Some(&value) = solution.get(&constraint.name) {
                if !self.is_satisfied(constraint, value) {
                    return false;
                }
            }
        }
        true
    }

    /// Check which constraints are violated
    pub fn check_violations(&self, solution: &HashMap<String, f64>) -> Vec<ConstraintViolation> {
        let mut violations = Vec::new();

        for constraint in &self.constraints {
            if let Some(&value) = solution.get(&constraint.name) {
                if !self.is_satisfied(constraint, value) {
                    let violation_amount = self.violation_amount(constraint, value);
                    violations.push(ConstraintViolation {
                        constraint_name: constraint.name.clone(),
                        constraint_type: constraint.constraint_type,
                        expected_value: constraint.threshold,
                        actual_value: value,
                        violation_amount,
                        penalty: self.calculate_penalty(constraint, value),
                    });
                }
            }
        }

        violations.sort_by(|a, b| b.violation_amount.partial_cmp(&a.violation_amount).unwrap());
        violations
    }

    /// Calculate total penalty for a solution
    pub fn calculate_total_penalty(&self, solution: &HashMap<String, f64>) -> f64 {
        let mut total_penalty = 0.0;

        for constraint in &self.constraints {
            if let Some(&value) = solution.get(&constraint.name) {
                total_penalty += self.calculate_penalty(constraint, value);
            }
        }

        total_penalty
    }

    /// Find the feasible region for a parameter
    pub fn find_feasible_range(
        &self,
        parameter_name: &str,
        current_value: f64,
        step_size: f64,
    ) -> Option<(f64, f64)> {
        let constraint = self.constraints.iter().find(|c| c.name == parameter_name)?;
        let mut lower = current_value;
        let mut upper = current_value;

        // Find lower bound
        for _ in 0..1000 {
            lower -= step_size;
            if !self.is_satisfied(constraint, lower) {
                lower += step_size;
                break;
            }
        }

        // Find upper bound
        for _ in 0..1000 {
            upper += step_size;
            if !self.is_satisfied(constraint, upper) {
                upper -= step_size;
                break;
            }
        }

        Some((lower, upper))
    }

    /// Project a solution onto the feasible region
    pub fn project_to_feasible(&self, solution: &HashMap<String, f64>) -> HashMap<String, f64> {
        let mut projected = solution.clone();

        for constraint in &self.constraints {
            if let Some(value) = projected.get_mut(&constraint.name) {
                *value = self.project_value(constraint, *value);
            }
        }

        projected
    }

    /// Check if a constraint is satisfied
    fn is_satisfied(&self, constraint: &OptimizationConstraint, value: f64) -> bool {
        match constraint.constraint_type {
            ConstraintType::UpperBound => value <= constraint.threshold + self.tolerance,
            ConstraintType::LowerBound => value >= constraint.threshold - self.tolerance,
            ConstraintType::Equality => {
                (value - constraint.threshold).abs() <= self.tolerance
            }
            ConstraintType::Range { lower, upper } => {
                value >= lower - self.tolerance && value <= upper + self.tolerance
            }
        }
    }

    /// Calculate the violation amount
    fn violation_amount(&self, constraint: &OptimizationConstraint, value: f64) -> f64 {
        match constraint.constraint_type {
            ConstraintType::UpperBound => (value - constraint.threshold).max(0.0),
            ConstraintType::LowerBound => (constraint.threshold - value).max(0.0),
            ConstraintType::Equality => (value - constraint.threshold).abs(),
            ConstraintType::Range { lower, upper } => {
                if value < lower {
                    lower - value
                } else if value > upper {
                    value - upper
                } else {
                    0.0
                }
            }
        }
    }

    /// Calculate penalty for a constraint violation
    fn calculate_penalty(&self, constraint: &OptimizationConstraint, value: f64) -> f64 {
        if self.is_satisfied(constraint, value) {
            0.0
        } else {
            let violation = self.violation_amount(constraint, value);
            violation * constraint.penalty_weight
        }
    }

    /// Project a value onto the feasible region
    fn project_value(&self, constraint: &OptimizationConstraint, value: f64) -> f64 {
        match constraint.constraint_type {
            ConstraintType::UpperBound => value.min(constraint.threshold),
            ConstraintType::LowerBound => value.max(constraint.threshold),
            ConstraintType::Equality => constraint.threshold,
            ConstraintType::Range { lower, upper } => value.clamp(lower, upper),
        }
    }

    /// Get constraint statistics
    pub fn get_constraint_stats(&self) -> ConstraintStats {
        let mut by_type: HashMap<ConstraintType, usize> = HashMap::new();
        for constraint in &self.constraints {
            *by_type.entry(constraint.constraint_type).or_insert(0) += 1;
        }

        ConstraintStats {
            total_constraints: self.constraints.len(),
            by_type,
            tolerance: self.tolerance,
            max_violations: self.max_violations,
        }
    }

    /// Clear all constraints
    pub fn clear(&mut self) {
        self.constraints.clear();
        self.penalty_weights.clear();
    }

    /// Remove a constraint by name
    pub fn remove_constraint(&mut self, name: &str) -> Option<OptimizationConstraint> {
        self.penalty_weights.remove(name);
        self.constraints
            .iter()
            .position(|c| c.name == name)
            .map(|pos| self.constraints.remove(pos))
    }
}

impl Default for ConstraintSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Constraint violation
#[derive(Debug, Clone)]
pub struct ConstraintViolation {
    pub constraint_name: String,
    pub constraint_type: ConstraintType,
    pub expected_value: f64,
    pub actual_value: f64,
    pub violation_amount: f64,
    pub penalty: f64,
}

/// Constraint statistics
#[derive(Debug, Clone)]
pub struct ConstraintStats {
    pub total_constraints: usize,
    pub by_type: HashMap<ConstraintType, usize>,
    pub tolerance: f64,
    pub max_violations: usize,
}

/// Linear constraint solver (simplified)
pub struct LinearConstraintSolver {
    pub constraints: Vec<LinearConstraint>,
}

#[derive(Debug, Clone)]
pub struct LinearConstraint {
    pub coefficients: Vec<f64>,
    pub constant: f64,
    pub constraint_type: LinearConstraintType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinearConstraintType {
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
}

impl LinearConstraintSolver {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, coefficients: Vec<f64>, constant: f64, c_type: LinearConstraintType) {
        self.constraints.push(LinearConstraint {
            coefficients,
            constant,
            constraint_type: c_type,
        });
    }

    /// Check if a solution satisfies all linear constraints
    pub fn check_solution(&self, solution: &[f64]) -> bool {
        for constraint in &self.constraints {
            if !self.is_satisfied(constraint, solution) {
                return false;
            }
        }
        true
    }

    fn is_satisfied(&self, constraint: &LinearConstraint, solution: &[f64]) -> bool {
        if solution.len() != constraint.coefficients.len() {
            return false;
        }

        let sum: f64 = constraint.coefficients
            .iter()
            .zip(solution.iter())
            .map(|(c, v)| c * v)
            .sum();

        match constraint.constraint_type {
            LinearConstraintType::LessThanOrEqual => sum <= constraint.constant + 1e-9,
            LinearConstraintType::GreaterThanOrEqual => sum >= constraint.constant - 1e-9,
            LinearConstraintType::Equal => (sum - constraint.constant).abs() <= 1e-9,
        }
    }
}

impl Default for LinearConstraintSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_solver() {
        let mut solver = ConstraintSolver::new();
        
        solver.add_constraint(OptimizationConstraint::new(
            "cpu_usage".to_string(),
            ConstraintType::UpperBound,
            80.0,
        ));

        let mut solution = HashMap::new();
        solution.insert("cpu_usage".to_string(), 70.0);

        assert!(solver.check_solution(&solution));
    }

    #[test]
    fn test_constraint_violation() {
        let mut solver = ConstraintSolver::new();
        
        solver.add_constraint(OptimizationConstraint::new(
            "cpu_usage".to_string(),
            ConstraintType::UpperBound,
            80.0,
        ));

        let mut solution = HashMap::new();
        solution.insert("cpu_usage".to_string(), 90.0);

        let violations = solver.check_violations(&solution);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].constraint_name, "cpu_usage");
    }

    #[test]
    fn test_penalty_calculation() {
        let mut solver = ConstraintSolver::new();
        
        let constraint = OptimizationConstraint::new(
            "cpu_usage".to_string(),
            ConstraintType::UpperBound,
            80.0,
        ).with_penalty_weight(2.0);

        solver.add_constraint(constraint);

        let mut solution = HashMap::new();
        solution.insert("cpu_usage".to_string(), 90.0);

        let penalty = solver.calculate_total_penalty(&solution);
        assert_eq!(penalty, 20.0); // (90 - 80) * 2
    }

    #[test]
    fn test_projection_to_feasible() {
        let mut solver = ConstraintSolver::new();
        
        solver.add_constraint(OptimizationConstraint::new(
            "cpu_usage".to_string(),
            ConstraintType::UpperBound,
            80.0,
        ));

        let mut solution = HashMap::new();
        solution.insert("cpu_usage".to_string(), 90.0);

        let projected = solver.project_to_feasible(&solution);
        assert_eq!(projected.get("cpu_usage"), Some(&80.0));
    }

    #[test]
    fn test_feasible_range() {
        let mut solver = ConstraintSolver::new();
        
        solver.add_constraint(OptimizationConstraint::new(
            "cpu_usage".to_string(),
            ConstraintType::Range {
                lower: 20.0,
                upper: 80.0,
            },
        ));

        let range = solver.find_feasible_range("cpu_usage", 50.0, 1.0);
        assert!(range.is_some());
        let (lower, upper) = range.unwrap();
        assert_eq!(lower, 20.0);
        assert_eq!(upper, 80.0);
    }

    #[test]
    fn test_linear_constraint_solver() {
        let mut solver = LinearConstraintSolver::new();
        
        solver.add_constraint(vec![1.0, 0.0], 10.0, LinearConstraintType::LessThanOrEqual);
        solver.add_constraint(vec![0.0, 1.0], 20.0, LinearConstraintType::LessThanOrEqual);

        assert!(solver.check_solution(&[5.0, 10.0]));
        assert!(!solver.check_solution(&[15.0, 10.0]));
    }
}