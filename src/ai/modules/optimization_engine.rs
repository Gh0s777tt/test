//! Optimization Engine for VantisOS
//!
//! This module provides the core optimization engine:
//! - Optimization suggestion generation
//! - Multiple optimization strategies
//! - Objective function optimization
//! - Convergence monitoring
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::optimization_engine::{OptimizationEngine, OptimizationStrategy};
//! use crate::ai::modules::optimization_types::OptimizationConfig;
//!
//! let mut engine = OptimizationEngine::new();
//! let config = OptimizationConfig::new(OptimizationStrategy::Adaptive);
//! let result = engine.optimize(&config, &objective_function);
//! ```

use crate::ai::error::{AIServiceError, Result};
use crate::ai::modules::optimization_types::{
    OptimizationConfig, OptimizationObjective, OptimizationResult, OptimizationStrategy,
    OptimizationSuggestion, OptimizationTarget, RiskLevel,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Objective function type
pub type ObjectiveFunction = Box<dyn Fn(&HashMap<String, f64>) -> f64 + Send + Sync>;

/// Optimization engine
pub struct OptimizationEngine {
    pub strategies: HashMap<OptimizationStrategy, Box<dyn OptimizationStrategyImpl>>,
    pub history: Vec<OptimizationResult>,
    pub max_history: usize,
}

impl OptimizationEngine {
    pub fn new() -> Self {
        let mut strategies: HashMap<OptimizationStrategy, Box<dyn OptimizationStrategyImpl>> =
            HashMap::new();

        strategies.insert(OptimizationStrategy::GradientBased, Box::new(GradientBasedOptimizer));
        strategies.insert(OptimizationStrategy::Bayesian, Box::new(BayesianOptimizer));
        strategies.insert(OptimizationStrategy::Evolutionary, Box::new(EvolutionaryOptimizer));
        strategies.insert(OptimizationStrategy::Adaptive, Box::new(AdaptiveOptimizer));
        strategies.insert(OptimizationStrategy::ReinforcementLearning, Box::new(RLOptimizer));
        strategies.insert(OptimizationStrategy::Ensemble, Box::new(EnsembleOptimizer));

        Self {
            strategies,
            history: Vec::new(),
            max_history: 1000,
        }
    }

    /// Run optimization
    pub fn optimize(
        &mut self,
        config: &OptimizationConfig,
        objective: &ObjectiveFunction,
    ) -> Result<OptimizationResult> {
        config.validate()?;

        let start_time = Instant::now();
        let strategy = self.strategies.get(&config.strategy)
            .ok_or_else(|| AIServiceError::ConfigurationError)?;

        let result = strategy.optimize(config, objective)?;
        let duration = start_time.elapsed();

        // Build result
        let suggestion = OptimizationSuggestion::new(config.target)
            .with_parameters(result.best_parameters.clone())
            .with_improvement(result.best_objective_value, 1.0 - result.convergence_value)
            .with_duration(duration.as_secs());

        let optimization_result = OptimizationResult::new(suggestion)
            .with_metrics(result.iterations, result.convergence_value, duration.as_millis() as u64)
            .with_solution(result.best_parameters.clone(), result.best_objective_value)
            .with_violations(result.constraint_violations);

        // Add to history
        self.history.push(optimization_result.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        Ok(optimization_result)
    }

    /// Generate optimization suggestions
    pub fn generate_suggestions(
        &self,
        target: OptimizationTarget,
        current_metrics: &HashMap<String, f64>,
    ) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        // Analyze current metrics
        for (metric_name, value) in current_metrics {
            if *value > 0.8 {
                // High usage - suggest optimization
                let suggestion = OptimizationSuggestion::new(target)
                    .with_parameters([(
                        format!("reduce_{}", metric_name),
                        *value * 0.8,
                    )]
                    .iter()
                    .cloned()
                    .collect())
                    .with_improvement(0.2, 0.7)
                    .with_duration(300);

                suggestions.push(suggestion);
            }
        }

        suggestions.sort_by(|a, b| b.expected_improvement.partial_cmp(&a.expected_improvement).unwrap());
        suggestions.truncate(5);
        suggestions
    }

    /// Get optimization history
    pub fn get_history(&self) -> &[OptimizationResult] {
        &self.history
    }

    /// Clear history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Get statistics
    pub fn get_statistics(&self) -> OptimizationStatistics {
        if self.history.is_empty() {
            return OptimizationStatistics::default();
        }

        let total_optimizations = self.history.len();
        let successful = self.history.iter().filter(|r| r.success).count();
        let total_iterations: usize = self.history.iter().map(|r| r.iterations).sum();
        let avg_iterations = total_iterations as f64 / total_optimizations as f64;
        let avg_improvement = self.history.iter()
            .map(|r| r.suggestion.expected_improvement)
            .sum::<f64>() / total_optimizations as f64;

        let mut strategy_counts: HashMap<OptimizationStrategy, usize> = HashMap::new();
        for result in &self.history {
            *strategy_counts.entry(result.suggestion.target).or_insert(0) += 1;
        }

        OptimizationStatistics {
            total_optimizations,
            successful,
            success_rate: successful as f64 / total_optimizations as f64,
            total_iterations,
            avg_iterations,
            avg_improvement,
            strategy_counts,
        }
    }
}

/// Optimization statistics
#[derive(Debug, Clone, Default)]
pub struct OptimizationStatistics {
    pub total_optimizations: usize,
    pub successful: usize,
    pub success_rate: f64,
    pub total_iterations: usize,
    pub avg_iterations: f64,
    pub avg_improvement: f64,
    pub strategy_counts: HashMap<OptimizationStrategy, usize>,
}

/// Optimization strategy implementation trait
pub trait OptimizationStrategyImpl {
    fn optimize(
        &self,
        config: &OptimizationConfig,
        objective: &ObjectiveFunction,
    ) -> Result<PartialOptimizationResult>;
}

/// Partial optimization result (internal)
struct PartialOptimizationResult {
    iterations: usize,
    convergence_value: f64,
    best_parameters: HashMap<String, f64>,
    best_objective_value: f64,
    constraint_violations: Vec<String>,
}

/// Gradient-based optimizer
struct GradientBasedOptimizer;

impl OptimizationStrategyImpl for GradientBasedOptimizer {
    fn optimize(
        &self,
        config: &OptimizationConfig,
        objective: &ObjectiveFunction,
    ) -> Result<PartialOptimizationResult> {
        let mut best_params: HashMap<String, f64> = config.parameters
            .iter()
            .map(|p| (p.name.clone(), p.value))
            .collect();

        let mut best_value = objective(&best_params);
        let mut convergence = f64::INFINITY;
        let learning_rate = 0.01;

        for iteration in 0..config.max_iterations {
            let mut gradients = HashMap::new();

            // Compute gradients
            for param in &config.parameters {
                if param.is_fixed {
                    continue;
                }

                let epsilon = 1e-6;
                let mut perturbed = best_params.clone();
                let current_value = perturbed.get(&param.name).copied().unwrap_or(param.value);
                perturbed.insert(param.name.clone(), current_value + epsilon);
                let value_plus = objective(&perturbed);
                
                perturbed.insert(param.name.clone(), current_value - epsilon);
                let value_minus = objective(&perturbed);

                let gradient = (value_plus - value_minus) / (2.0 * epsilon);
                gradients.insert(param.name.clone(), gradient);
            }

            // Update parameters
            for (name, gradient) in &gradients {
                let param = config.parameters.iter().find(|p| &p.name == name);
                if let Some(param) = param {
                    let current = best_params.get(name).copied().unwrap_or(param.value);
                    let new_value = if config.objective == OptimizationObjective::Minimize {
                        current - learning_rate * gradient
                    } else {
                        current + learning_rate * gradient
                    };

                    // Clamp to bounds
                    let clamped = new_value.clamp(param.min_value, param.max_value);
                    best_params.insert(name.clone(), clamped);
                }
            }

            // Evaluate objective
            let new_value = objective(&best_params);
            convergence = (new_value - best_value).abs();

            if new_value < best_value {
                best_value = new_value;
            }

            if convergence < config.convergence_threshold {
                break;
            }
        }

        let violations = check_constraints(config, &best_params);

        Ok(PartialOptimizationResult {
            iterations: config.max_iterations,
            convergence_value: convergence,
            best_parameters: best_params,
            best_objective_value: best_value,
            constraint_violations: violations,
        })
    }
}

/// Bayesian optimizer
struct BayesianOptimizer;

impl OptimizationStrategyImpl for BayesianOptimizer {
    fn optimize(
        &self,
        config: &OptimizationConfig,
        objective: &ObjectiveFunction,
    ) -> Result<PartialOptimizationResult> {
        let mut best_params: HashMap<String, f64> = config.parameters
            .iter()
            .map(|p| (p.name.clone(), p.value))
            .collect();

        let mut best_value = objective(&best_params);
        let mut convergence = f64::INFINITY;

        // Simplified Bayesian optimization using grid search + refinement
        for iteration in 0..config.max_iterations {
            // Sample new parameters
            let mut new_params = HashMap::new();
            for param in &config.parameters {
                if param.is_fixed {
                    new_params.insert(param.name.clone(), param.value);
                } else {
                    let range = param.max_value - param.min_value;
                    let sample = param.min_value + (rand::random::<f64>() * range);
                    new_params.insert(param.name.clone(), sample);
                }
            }

            let new_value = objective(&new_params);

            if new_value < best_value {
                best_value = new_value;
                best_params = new_params;
            }

            convergence = (new_value - best_value).abs();

            if convergence < config.convergence_threshold {
                break;
            }
        }

        let violations = check_constraints(config, &best_params);

        Ok(PartialOptimizationResult {
            iterations: config.max_iterations,
            convergence_value: convergence,
            best_parameters: best_params,
            best_objective_value: best_value,
            constraint_violations: violations,
        })
    }
}

/// Evolutionary optimizer
struct EvolutionaryOptimizer;

impl OptimizationStrategyImpl for EvolutionaryOptimizer {
    fn optimize(
        &self,
        config: &OptimizationConfig,
        objective: &ObjectiveFunction,
    ) -> Result<PartialOptimizationResult> {
        let population_size = 10;
        let mutation_rate = 0.1;

        let mut population: Vec<HashMap<String, f64>> = (0..population_size)
            .map(|_| {
                config.parameters
                    .iter()
                    .map(|p| {
                        if p.is_fixed {
                            (p.name.clone(), p.value)
                        } else {
                            let range = p.max_value - p.min_value;
                            let value = p.min_value + rand::random::<f64>() * range;
                            (p.name.clone(), value)
                        }
                    })
                    .collect()
            })
            .collect();

        let mut best_params = population[0].clone();
        let mut best_value = objective(&best_params);
        let mut convergence = f64::INFINITY;

        for iteration in 0..config.max_iterations {
            // Evaluate population
            let mut fitness: Vec<f64> = population.iter().map(|p| objective(p)).collect();

            // Find best
            let best_idx = fitness
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(i, _)| i)
                .unwrap();

            if fitness[best_idx] < best_value {
                best_value = fitness[best_idx];
                best_params = population[best_idx].clone();
            }

            // Selection and crossover
            let mut new_population = Vec::new();
            for _ in 0..population_size {
                let parent1 = &population[rand::random::<usize>() % population.len()];
                let parent2 = &population[rand::random::<usize>() % population.len()];
                
                let mut child = HashMap::new();
                for param in &config.parameters {
                    let p1 = parent1.get(&param.name).copied().unwrap_or(param.value);
                    let p2 = parent2.get(&param.name).copied().unwrap_or(param.value);
                    
                    let value = if rand::random::<f64>() < 0.5 {
                        p1
                    } else {
                        p2
                    };

                    // Mutation
                    let mutated = if rand::random::<f64>() < mutation_rate {
                        let range = param.max_value - param.min_value;
                        let delta = (rand::random::<f64>() - 0.5) * 2.0 * mutation_rate * range;
                        (value + delta).clamp(param.min_value, param.max_value)
                    } else {
                        value
                    };

                    child.insert(param.name.clone(), mutated);
                }
                new_population.push(child);
            }

            population = new_population;
            convergence = (best_value - fitness[best_idx]).abs();

            if convergence < config.convergence_threshold {
                break;
            }
        }

        let violations = check_constraints(config, &best_params);

        Ok(PartialOptimizationResult {
            iterations: config.max_iterations,
            convergence_value: convergence,
            best_parameters: best_params,
            best_objective_value: best_value,
            constraint_violations: violations,
        })
    }
}

/// Adaptive optimizer
struct AdaptiveOptimizer;

impl OptimizationStrategyImpl for AdaptiveOptimizer {
    fn optimize(
        &self,
        config: &OptimizationConfig,
        objective: &ObjectiveFunction,
    ) -> Result<PartialOptimizationResult> {
        // Adaptive: switch between strategies based on convergence
        let strategies = vec![
            Box::new(GradientBasedOptimizer) as Box<dyn OptimizationStrategyImpl>,
            Box::new(BayesianOptimizer),
            Box::new(EvolutionaryOptimizer),
        ];

        let mut best_result = strategies[0].optimize(config, objective)?;

        for strategy in strategies.iter().skip(1) {
            let result = strategy.optimize(config, objective)?;
            if result.best_objective_value < best_result.best_objective_value {
                best_result = result;
            }
        }

        Ok(best_result)
    }
}

/// RL-based optimizer (placeholder)
struct RLOptimizer;

impl OptimizationStrategyImpl for RLOptimizer {
    fn optimize(
        &self,
        config: &OptimizationConfig,
        objective: &ObjectiveFunction,
    ) -> Result<PartialOptimizationResult> {
        // Placeholder: use Bayesian for now
        BayesianOptimizer.optimize(config, objective)
    }
}

/// Ensemble optimizer
struct EnsembleOptimizer;

impl OptimizationStrategyImpl for EnsembleOptimizer {
    fn optimize(
        &self,
        config: &OptimizationConfig,
        objective: &ObjectiveFunction,
    ) -> Result<PartialOptimizationResult> {
        // Run multiple strategies and combine results
        let strategies = vec![
            Box::new(GradientBasedOptimizer) as Box<dyn OptimizationStrategyImpl>,
            Box::new(BayesianOptimizer),
            Box::new(EvolutionaryOptimizer),
        ];

        let mut all_params = Vec::new();

        for strategy in &strategies {
            let result = strategy.optimize(config, objective)?;
            all_params.push(result.best_parameters);
        }

        // Average parameters
        let mut averaged_params = HashMap::new();
        for params in &all_params {
            for (name, value) in params {
                *averaged_params.entry(name.clone()).or_insert(0.0) += value / all_params.len() as f64;
            }
        }

        let best_value = objective(&averaged_params);
        let violations = check_constraints(config, &averaged_params);

        Ok(PartialOptimizationResult {
            iterations: config.max_iterations,
            convergence_value: 0.0,
            best_parameters: averaged_params,
            best_objective_value: best_value,
            constraint_violations: violations,
        })
    }
}

/// Check constraint violations
fn check_constraints(
    config: &OptimizationConfig,
    params: &HashMap<String, f64>,
) -> Vec<String> {
    let mut violations = Vec::new();

    for constraint in &config.constraints {
        if let Some(&value) = params.get(&constraint.name) {
            if !constraint.is_satisfied(value) {
                violations.push(format!(
                    "{} violated: {}",
                    constraint.name,
                    constraint.penalty(value)
                ));
            }
        }
    }

    violations
}

impl Default for OptimizationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::modules::optimization_types::{
        OptimizationConstraint, ConstraintType, OptimizationParameter,
    };

    #[test]
    fn test_optimization_engine() {
        let mut engine = OptimizationEngine::new();
        let config = OptimizationConfig::new(OptimizationStrategy::Adaptive);

        let objective: ObjectiveFunction = Box::new(|params| {
            let x = *params.get("x").unwrap_or(&0.0);
            let y = *params.get("y").unwrap_or(&0.0);
            (x - 2.0).powi(2) + (y - 3.0).powi(2)
        });

        let result = engine.optimize(&config, &objective);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_generate_suggestions() {
        let engine = OptimizationEngine::new();
        
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_string(), 0.95);
        metrics.insert("memory_usage".to_string(), 0.85);

        let suggestions = engine.generate_suggestions(OptimizationTarget::CPU, &metrics);
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_optimization_statistics() {
        let mut engine = OptimizationEngine::new();
        
        let config = OptimizationConfig::new(OptimizationStrategy::Adaptive);
        let objective: ObjectiveFunction = Box::new(|params| {
            let x = *params.get("x").unwrap_or(&0.0);
            x.powi(2)
        });

        for _ in 0..5 {
            let _ = engine.optimize(&config, &objective);
        }

        let stats = engine.get_statistics();
        assert_eq!(stats.total_optimizations, 5);
        assert!(stats.success_rate > 0.0);
    }

    #[test]
    fn test_constraint_checking() {
        let mut config = OptimizationConfig::new(OptimizationStrategy::Adaptive);
        
        config.with_constraint(OptimizationConstraint::new(
            "x".to_string(),
            ConstraintType::UpperBound,
            10.0,
        ));

        config.with_parameter(OptimizationParameter::new("x".to_string(), 5.0, 0.0, 20.0));

        let mut params = HashMap::new();
        params.insert("x".to_string(), 15.0);

        let violations = check_constraints(&config, &params);
        assert!(!violations.is_empty());
    }
}