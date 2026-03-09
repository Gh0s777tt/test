//! Multi-Objective Optimization for VantisOS
//!
//! This module provides multi-objective optimization methods:
//! - Pareto frontier computation
//! - NSGA-II (Non-dominated Sorting Genetic Algorithm)
//! - Weighted sum approach
//! - epsilon-constraint method
//! - Hypervolume indicator
//!
//! ## Example
//! ```rust
//! use crate::ai::modules::multi_objective::{MultiObjectiveOpt, MOConfig};
//!
//! let config = MOConfig::default();
//! let mut mo_opt = MultiObjectiveOpt::new(config);
//! let pareto_front = mo_opt.optimize(objectives);
//! ```

use crate::ai::error::{AIServiceError, Result};
use std::collections::HashMap;

/// Multi-objective optimization configuration
#[derive(Debug, Clone)]
pub struct MOConfig {
    /// Number of objectives
    pub n_objectives: usize,
    /// Population size
    pub population_size: usize,
    /// Number of generations
    pub n_generations: usize,
    /// Mutation rate
    pub mutation_rate: f64,
    /// Crossover rate
    pub crossover_rate: f64,
    /// Use archive
    pub use_archive: bool,
    /// Archive size
    pub archive_size: usize,
}

impl Default for MOConfig {
    fn default() -> Self {
        Self {
            n_objectives: 2,
            population_size: 100,
            n_generations: 100,
            mutation_rate: 0.1,
            crossover_rate: 0.9,
            use_archive: true,
            archive_size: 100,
        }
    }
}

/// Multi-objective solution
#[derive(Debug, Clone)]
pub struct MOSolution {
    pub params: HashMap<String, f64>,
    pub objectives: Vec<f64>,
    pub rank: usize,
    pub crowding_distance: f64,
}

impl MOSolution {
    pub fn new(params: HashMap<String, f64>, objectives: Vec<f64>) -> Self {
        Self {
            params,
            objectives,
            rank: 0,
            crowding_distance: 0.0,
        }
    }
    
    /// Check if this solution dominates another
    pub fn dominates(&self, other: &MOSolution) -> bool {
        let mut better_in_some = false;
        
        for (i, &obj) in self.objectives.iter().enumerate() {
            if obj > other.objectives[i] {
                return false; // Assuming minimization
            }
            if obj < other.objectives[i] {
                better_in_some = true;
            }
        }
        
        better_in_some
    }
}

/// Pareto frontier
#[derive(Debug, Clone)]
pub struct ParetoFront {
    pub solutions: Vec<MOSolution>,
}

impl ParetoFront {
    pub fn new() -> Self {
        Self { solutions: Vec::new() }
    }
    
    /// Add solution to frontier if non-dominated
    pub fn add_solution(&mut self, solution: MOSolution) {
        let mut dominated = false;
        
        // Remove solutions dominated by new solution
        self.solutions.retain(|s| !solution.dominates(s));
        
        // Check if new solution is dominated by any existing solution
        for s in &self.solutions {
            if s.dominates(&solution) {
                dominated = true;
                break;
            }
        }
        
        if !dominated {
            self.solutions.push(solution);
        }
    }
    
    /// Get number of solutions in frontier
    pub fn size(&self) -> usize {
        self.solutions.len()
    }
    
    /// Get solutions
    pub fn solutions(&self) -> &[MOSolution] {
        &self.solutions
    }
    
    /// Compute hypervolume indicator
    pub fn hypervolume(&self, reference_point: &[f64]) -> f64 {
        if self.solutions.is_empty() {
            return 0.0;
        }
        
        // Simplified hypervolume calculation
        // In production, would use proper algorithm
        let n_objectives = self.solutions[0].objectives.len();
        
        let mut volume = 0.0;
        
        for sol in &self.solutions {
            let mut sol_volume = 1.0;
            for i in 0..n_objectives {
                if sol.objectives[i] < reference_point[i] {
                    sol_volume *= reference_point[i] - sol.objectives[i];
                } else {
                    sol_volume = 0.0;
                    break;
                }
            }
            volume += sol_volume;
        }
        
        volume
    }
}

/// NSGA-II optimizer
#[derive(Debug, Clone)]
pub struct NSGAIIOptimizer {
    pub config: MOConfig,
    pub pareto_front: ParetoFront,
    pub archive: Vec<MOSolution>,
}

impl NSGAIIOptimizer {
    pub fn new(config: MOConfig) -> Self {
        Self {
            config,
            pareto_front: ParetoFront::new(),
            archive: Vec::new(),
        }
    }
    
    /// Non-dominated sort
    fn non_dominated_sort(&self, population: &[MOSolution]) -> Vec<Vec<MOSolution>> {
        let mut fronts = Vec::new();
        let mut remaining: Vec<MOSolution> = population.to_vec();
        
        while !remaining.is_empty() {
            let mut front = Vec::new();
            let mut to_remove = Vec::new();
            
            for (i, sol) in remaining.iter().enumerate() {
                let mut dominated = false;
                
                for (j, other) in remaining.iter().enumerate() {
                    if i != j && other.dominates(sol) {
                        dominated = true;
                        break;
                    }
                }
                
                if !dominated {
                    front.push(sol.clone());
                    to_remove.push(i);
                }
            }
            
            fronts.push(front);
            
            // Remove selected solutions
            to_remove.sort();
            to_remove.reverse();
            for idx in to_remove {
                remaining.remove(idx);
            }
        }
        
        fronts
    }
    
    /// Calculate crowding distance
    fn calculate_crowding_distance(&self, front: &mut [MOSolution]) {
        if front.is_empty() {
            return;
        }
        
        let n_objectives = front[0].objectives.len();
        
        // Initialize distances
        for sol in front.iter_mut() {
            sol.crowding_distance = 0.0;
        }
        
        // Calculate distance for each objective
        for obj_idx in 0..n_objectives {
            // Sort by objective value
            front.sort_by(|a, b| {
                a.objectives[obj_idx].partial_cmp(&b.objectives[obj_idx]).unwrap()
            });
            
            // Boundary solutions have infinite distance
            if !front.is_empty() {
                front[0].crowding_distance = f64::INFINITY;
                front[front.len() - 1].crowding_distance = f64::INFINITY;
            }
            
            // Calculate distance for interior solutions
            if front.len() > 2 {
                let min_val = front[0].objectives[obj_idx];
                let max_val = front[front.len() - 1].objectives[obj_idx];
                
                if max_val > min_val {
                    for i in 1..(front.len() - 1) {
                        let distance = (front[i + 1].objectives[obj_idx] - front[i - 1].objectives[obj_idx]) 
                            / (max_val - min_val);
                        front[i].crowding_distance += distance;
                    }
                }
            }
        }
    }
    
    /// Tournament selection
    fn tournament_selection(&self, population: &[MOSolution]) -> MOSolution {
        // Select 2 random individuals
        let idx1 = self.random_index(population.len());
        let mut idx2 = self.random_index(population.len());
        
        while idx1 == idx2 {
            idx2 = self.random_index(population.len());
        }
        
        let sol1 = &population[idx1];
        let sol2 = &population[idx2];
        
        // Compare based on rank and crowding distance
        if sol1.rank != sol2.rank {
            if sol1.rank < sol2.rank {
                sol1.clone()
            } else {
                sol2.clone()
            }
        } else {
            if sol1.crowding_distance > sol2.crowding_distance {
                sol1.clone()
            } else {
                sol2.clone()
            }
        }
    }
    
    fn random_index(&self, n: usize) -> usize {
        let hash = (n as f64 * 0.618033988749895).fract() * n as f64;
        (hash as usize) % n
    }
    
    /// Crossover
    fn crossover(&self, parent1: &MOSolution, parent2: &MOSolution) -> (MOSolution, MOSolution) {
        // Simple blend crossover
        let alpha = self.random_index(100) as f64 / 100.0;
        
        let mut child1_params = HashMap::new();
        let mut child2_params = HashMap::new();
        
        for (key, &val1) in &parent1.params {
            if let Some(&val2) = parent2.params.get(key) {
                let c1 = alpha * val1 + (1.0 - alpha) * val2;
                let c2 = (1.0 - alpha) * val1 + alpha * val2;
                child1_params.insert(key.clone(), c1);
                child2_params.insert(key.clone(), c2);
            }
        }
        
        let child1_obj: Vec<f64> = parent1.objectives.iter()
            .zip(parent2.objectives.iter())
            .map(|(o1, o2)| alpha * o1 + (1.0 - alpha) * o2)
            .collect();
        
        let child2_obj: Vec<f64> = parent1.objectives.iter()
            .zip(parent2.objectives.iter())
            .map(|(o1, o2)| (1.0 - alpha) * o1 + alpha * o2)
            .collect();
        
        let child1 = MOSolution::new(child1_params, child1_obj);
        let child2 = MOSolution::new(child2_params, child2_obj);
        
        (child1, child2)
    }
    
    /// Mutation
    fn mutate(&self, solution: &mut MOSolution) {
        for (_, val) in solution.params.iter_mut() {
            if self.rand() < self.config.mutation_rate {
                // Gaussian mutation
                let perturbation = (self.rand() * 2.0 - 1.0) * 0.1;
                *val = (*val + perturbation).max(0.0);
            }
        }
    }
    
    fn rand(&self) -> f64 {
        let hash = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as f64;
        (hash * 0.618033988749895).fract().abs()
    }
    
    /// Run NSGA-II optimization
    pub fn optimize<F>(&mut self, objectives: F) -> Result<&ParetoFront>
    where
        F: Fn(&HashMap<String, f64>) -> Vec<f64>,
    {
        // Initialize population
        let mut population: Vec<MOSolution> = (0..self.config.population_size)
            .map(|i| {
                let params = HashMap::new();
                let obj_values = objectives(&params);
                MOSolution::new(params, obj_values)
            })
            .collect();
        
        // Evolution
        for _generation in 0..self.config.n_generations {
            // Non-dominated sort
            let fronts = self.non_dominated_sort(&population);
            
            // Assign ranks
            for (rank, front) in fronts.iter().enumerate() {
                for sol in front {
                    sol.rank = rank;
                }
            }
            
            // Calculate crowding distance for each front
            for front in &mut fronts.clone() {
                self.calculate_crowding_distance(&mut fronts.clone()[fronts.iter().position(|f| std::ptr::eq(f, front)).unwrap()]);
            }
            
            // Selection
            let mut offspring = Vec::new();
            
            while offspring.len() < self.config.population_size {
                let parent1 = self.tournament_selection(&population);
                let parent2 = self.tournament_selection(&population);
                
                // Crossover
                if self.rand() < self.config.crossover_rate {
                    let (child1, child2) = self.crossover(&parent1, &parent2);
                    offspring.push(child1);
                    offspring.push(child2);
                } else {
                    offspring.push(parent1.clone());
                    offspring.push(parent2.clone());
                }
            }
            
            // Mutation
            for child in &mut offspring {
                self.mutate(child);
            }
            
            // Evaluate offspring
            for child in &mut offspring {
                child.objectives = objectives(&child.params);
            }
            
            // Combine population and offspring
            let mut combined = population.clone();
            combined.extend(offspring);
            
            // Non-dominated sort combined population
            let fronts = self.non_dominated_sort(&combined);
            
            // Select next generation
            let mut next_population = Vec::new();
            let mut front_idx = 0;
            
            while next_population.len() + fronts[front_idx].len() <= self.config.population_size {
                next_population.extend(fronts[front_idx].clone());
                front_idx += 1;
            }
            
            // Fill remaining spots from last front based on crowding distance
            if next_population.len() < self.config.population_size && front_idx < fronts.len() {
                let mut last_front = fronts[front_idx].clone();
                self.calculate_crowding_distance(&mut last_front);
                
                last_front.sort_by(|a, b| {
                    b.crowding_distance.partial_cmp(&a.crowding_distance).unwrap()
                });
                
                let remaining = self.config.population_size - next_population.len();
                next_population.extend(last_front.into_iter().take(remaining));
            }
            
            population = next_population;
        }
        
        // Final non-dominated sort
        let fronts = self.non_dominated_sort(&population);
        
        if !fronts.is_empty() {
            for sol in &fronts[0] {
                self.pareto_front.add_solution(sol.clone());
            }
            
            if self.config.use_archive {
                self.archive.extend(fronts[0].iter().cloned());
                
                // Trim archive if needed
                if self.archive.len() > self.config.archive_size {
                    self.archive.truncate(self.config.archive_size);
                }
            }
        }
        
        Ok(&self.pareto_front)
    }
}

/// Weighted sum approach
#[derive(Debug, Clone)]
pub struct WeightedSum {
    pub weights: Vec<f64>,
}

impl WeightedSum {
    pub fn new(weights: Vec<f64>) -> Self {
        Self { weights }
    }
    
    pub fn optimize<F>(&self, objectives: F) -> Result<MOSolution>
    where
        F: Fn(&HashMap<String, f64>) -> Vec<f64>,
    {
        // Simplified optimization
        let params = HashMap::new();
        let obj_values = objectives(&params);
        
        let weighted_sum: f64 = self.weights.iter()
            .zip(obj_values.iter())
            .map(|(&w, &o)| w * o)
            .sum();
        
        Ok(MOSolution::new(params, vec![weighted_sum]))
    }
}

/// Epsilon-constraint method
#[derive(Debug, Clone)]
pub struct EpsilonConstraint {
    pub primary_objective: usize,
    pub epsilon_values: Vec<f64>,
}

impl EpsilonConstraint {
    pub fn new(primary_objective: usize, epsilon_values: Vec<f64>) -> Self {
        Self {
            primary_objective,
            epsilon_values,
        }
    }
    
    pub fn optimize<F>(&self, objectives: F) -> Result<MOSolution>
    where
        F: Fn(&HashMap<String, f64>) -> Vec<f64>,
    {
        // Simplified optimization
        let params = HashMap::new();
        let obj_values = objectives(&params);
        
        // Check constraints
        for (i, &epsilon) in self.epsilon_values.iter().enumerate() {
            if i != self.primary_objective && obj_values[i] > epsilon {
                return Err(AIServiceError::InvalidInput {
                    message: "Constraint violated".to_string()
                });
            }
        }
        
        Ok(MOSolution::new(params, obj_values))
    }
}

/// Multi-objective optimizer wrapper
#[derive(Debug, Clone)]
pub struct MultiObjectiveOpt {
    pub config: MOConfig,
    pub method: MOOptimizationMethod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MOOptimizationMethod {
    NSGAII,
    WeightedSum,
    EpsilonConstraint,
}

impl MultiObjectiveOpt {
    pub fn new(config: MOConfig, method: MOOptimizationMethod) -> Self {
        Self { config, method }
    }
    
    pub fn optimize<F>(&mut self, objectives: F) -> Result<&ParetoFront>
    where
        F: Fn(&HashMap<String, f64>) -> Vec<f64>,
    {
        match self.method {
            MOOptimizationMethod::NSGAII => {
                let mut nsga2 = NSGAIIOptimizer::new(self.config.clone());
                nsga2.optimize(objectives)
            }
            MOOptimizationMethod::WeightedSum => {
                let weights = vec![1.0 / self.config.n_objectives as f64; self.config.n_objectives];
                let ws = WeightedSum::new(weights);
                let sol = ws.optimize(objectives)?;
                let mut pf = ParetoFront::new();
                pf.add_solution(sol);
                Ok(&pf)
            }
            MOOptimizationMethod::EpsilonConstraint => {
                let ec = EpsilonConstraint::new(0, vec![1.0; self.config.n_objectives - 1]);
                let sol = ec.optimize(objectives)?;
                let mut pf = ParetoFront::new();
                pf.add_solution(sol);
                Ok(&pf)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pareto_front() {
        let mut pf = ParetoFront::new();
        
        let sol1 = MOSolution::new(HashMap::new(), vec![1.0, 5.0]);
        let sol2 = MOSolution::new(HashMap::new(), vec![3.0, 2.0]);
        let sol3 = MOSolution::new(HashMap::new(), vec![2.0, 4.0]);
        
        pf.add_solution(sol1);
        pf.add_solution(sol2);
        pf.add_solution(sol3);
        
        assert_eq!(pf.size(), 2); // sol3 is dominated
    }

    #[test]
    fn test_domination() {
        let sol1 = MOSolution::new(HashMap::new(), vec![1.0, 5.0]);
        let sol2 = MOSolution::new(HashMap::new(), vec![2.0, 4.0]);
        let sol3 = MOSolution::new(HashMap::new(), vec![1.0, 5.0]);
        
        assert!(sol1.dominates(&sol2));
        assert!(!sol2.dominates(&sol1));
        assert!(!sol1.dominates(&sol3));
    }

    #[test]
    fn test_hypervolume() {
        let mut pf = ParetoFront::new();
        pf.add_solution(MOSolution::new(HashMap::new(), vec![1.0, 1.0]));
        pf.add_solution(MOSolution::new(HashMap::new(), vec![2.0, 0.5]));
        
        let hv = pf.hypervolume(&[3.0, 3.0]);
        assert!(hv > 0.0);
    }
}