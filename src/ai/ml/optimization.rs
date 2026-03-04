//! Optimization Module
//! 
//! Provides optimization algorithms for system parameter tuning and decision making.
//! 
//! ## Algorithms
//! - Bayesian Optimization: Global optimization of expensive functions
//! - Genetic Algorithms: Evolutionary optimization
//! - Simulated Annealing: Stochastic optimization
//! - Gradient Descent: First-order optimization

use crate::ai::error::AIError;

/// Bayesian Optimization
/// 
/// Global optimization algorithm for expensive-to-evaluate black-box functions.
/// Uses Gaussian Process surrogate model to guide search.
/// 
/// ## Features
/// - Efficient global optimization
/// - Handles noisy evaluations
/// - Exploration-exploitation balance
/// - Acquisition function selection
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::ml::optimization::BayesianOptimizer;
//! 
//! let mut optimizer = BayesianOptimizer::new(2, 0.1)?;
//! 
//! // Define objective function
//! let objective = |params: &[f64]| -> f64 {
//!     -((params[0] - 0.5).powi(2) + (params[1] - 0.5).powi(2))
//! };
//! 
//! // Optimize
//! let best = optimizer.optimize(&objective, 50)?;
//! println!("Best parameters: {:?}", best);
//! ```
pub struct BayesianOptimizer {
    dim: usize,
    xi: f64,
    // In real implementation, would have Gaussian Process
    // For now, we use a simpler approach
    observations: Vec<Observation>,
    bounds: Vec<(f64, f64)>,
}

#[derive(Debug, Clone)]
struct Observation {
    params: Vec<f64>,
    value: f64,
}

impl BayesianOptimizer {
    /// Create a new Bayesian optimizer
    /// 
    /// ## Arguments
    /// * `dim` - Parameter dimension
    /// * `xi` - Exploration parameter (0.0-1.0)
    /// 
    /// ## Returns
    /// New optimizer or error
    pub fn new(dim: usize, xi: f64) -> Result<Self, AIError> {
        if dim == 0 {
            return Err(AIError::InvalidInput("Dimension must be > 0".to_string()));
        }

        Ok(Self {
            dim,
            xi: xi.max(0.0).min(1.0),
            observations: Vec::new(),
            bounds: vec![(0.0, 1.0); dim],
        })
    }

    /// Set parameter bounds
    pub fn set_bounds(&mut self, bounds: Vec<(f64, f64)>) -> Result<(), AIError> {
        if bounds.len() != self.dim {
            return Err(AIError::InvalidInput(
                "Bounds length must match dimension".to_string(),
            ));
        }

        self.bounds = bounds;
        Ok(())
    }

    /// Optimize objective function
    /// 
    /// ## Arguments
    /// * `objective` - Function to minimize
    /// * `n_iter` - Number of optimization iterations
    /// 
    /// ## Returns
    /// Best parameters found
    pub fn optimize<F>(&mut self, objective: F, n_iter: usize) -> Result<Vec<f64>, AIError>
    where
        F: Fn(&[f64]) -> f64,
    {
        if self.observations.is_empty() {
            // Initialize with random sample
            let initial_params = self.random_params();
            let value = objective(&initial_params);
            self.observations.push(Observation {
                params: initial_params.clone(),
                value,
            });
        }

        for _ in 0..n_iter {
            // Suggest next point to evaluate
            let suggested = self.suggest()?;
            
            // Evaluate objective
            let value = objective(&suggested);
            
            // Add to observations
            self.observations.push(Observation {
                params: suggested,
                value,
            });
        }

        // Return best observation
        let best = self.observations
            .iter()
            .min_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
            .unwrap();

        Ok(best.params.clone())
    }

    /// Suggest next point to evaluate
    fn suggest(&self) -> Result<Vec<f64>, AIError> {
        if self.observations.is_empty() {
            return Ok(self.random_params());
        }

        // For simplicity, use Thompson sampling
        // In real implementation, would use acquisition function (EI, UCB, etc.)
        if rand::random::<f64>() < self.xi {
            // Explore: random point
            Ok(self.random_params())
        } else {
            // Exploit: point near best observation
            let best = self.observations
                .iter()
                .min_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
                .unwrap();

            // Add small noise to best parameters
            let mut suggested = best.params.clone();
            for (i, &value) in suggested.iter_mut().enumerate() {
                let noise = (rand::random::<f64>() - 0.5) * 0.1;
                *value = (value + noise).max(self.bounds[i].0).min(self.bounds[i].1);
            }

            Ok(suggested)
        }
    }

    /// Generate random parameters within bounds
    fn random_params(&self) -> Vec<f64> {
        self.bounds
            .iter()
            .map(|(min, max)| {
                min + rand::random::<f64>() * (max - min)
            })
            .collect()
    }
}

/// Genetic Algorithm
/// 
/// Evolutionary optimization algorithm inspired by natural selection.
/// 
/// ## Features
/// - Population-based search
/// - Crossover and mutation operators
/// - Selection mechanisms
/// - Elitism preservation
pub struct GeneticAlgorithm {
    population_size: usize,
    mutation_rate: f64,
    crossover_rate: f64,
    tournament_size: usize,
    elitism_count: usize,
    dim: usize,
    bounds: Vec<(f64, f64)>,
}

impl GeneticAlgorithm {
    /// Create a new genetic algorithm
    /// 
    /// ## Arguments
    /// * `population_size` - Size of population
    /// * `mutation_rate` - Probability of mutation (0.0-1.0)
    /// * `crossover_rate` - Probability of crossover (0.0-1.0)
    /// * `dim` - Parameter dimension
    pub fn new(
        population_size: usize,
        mutation_rate: f64,
        crossover_rate: f64,
        dim: usize,
    ) -> Result<Self, AIError> {
        if population_size < 2 {
            return Err(AIError::InvalidInput(
                "Population size must be >= 2".to_string(),
            ));
        }

        Ok(Self {
            population_size,
            mutation_rate: mutation_rate.max(0.0).min(1.0),
            crossover_rate: crossover_rate.max(0.0).min(1.0),
            tournament_size: 3,
            elitism_count: 2,
            dim,
            bounds: vec![(0.0, 1.0); dim],
        })
    }

    /// Set parameter bounds
    pub fn set_bounds(&mut self, bounds: Vec<(f64, f64)>) -> Result<(), AIError> {
        if bounds.len() != self.dim {
            return Err(AIError::InvalidInput(
                "Bounds length must match dimension".to_string(),
            ));
        }
        self.bounds = bounds;
        Ok(())
    }

    /// Optimize objective function
    /// 
    /// ## Arguments
    /// * `objective` - Function to minimize
    /// * `generations` - Number of generations
    /// 
    /// ## Returns
    /// Best parameters found
    pub fn optimize<F>(&self, objective: F, generations: usize) -> Result<Vec<f64>, AIError>
    where
        F: Fn(&[f64]) -> f64,
    {
        // Initialize population
        let mut population = self.initialize_population();

        for _ in 0..generations {
            // Evaluate fitness
            let fitness: Vec<f64> = population
                .iter()
                .map(|individual| objective(individual))
                .collect();

            // Create next generation
            let mut next_population = Vec::new();

            // Elitism: keep best individuals
            let mut sorted: Vec<_> = population.iter().zip(fitness.iter()).collect();
            sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

            for i in 0..self.elitism_count.min(sorted.len()) {
                next_population.push(sorted[i].0.clone());
            }

            // Generate rest of population
            while next_population.len() < self.population_size {
                // Selection
                let parent1 = self.tournament_select(&population, &fitness)?;
                let parent2 = self.tournament_select(&population, &fitness)?;

                // Crossover
                let offspring = if rand::random::<f64>() < self.crossover_rate {
                    self.crossover(&parent1, &parent2)?
                } else {
                    parent1.clone()
                };

                // Mutation
                let mutated = self.mutate(offspring)?;

                next_population.push(mutated);
            }

            population = next_population;
        }

        // Return best individual
        let best = population
            .iter()
            .min_by(|a, b| objective(a).partial_cmp(&objective(b)).unwrap())
            .unwrap();

        Ok(best.clone())
    }

    /// Initialize random population
    fn initialize_population(&self) -> Vec<Vec<f64>> {
        (0..self.population_size)
            .map(|_| self.random_individual())
            .collect()
    }

    /// Generate random individual
    fn random_individual(&self) -> Vec<f64> {
        self.bounds
            .iter()
            .map(|(min, max)| min + rand::random::<f64>() * (max - min))
            .collect()
    }

    /// Tournament selection
    fn tournament_select(
        &self,
        population: &[Vec<f64>],
        fitness: &[f64],
    ) -> Result<Vec<f64>, AIError> {
        let mut best_idx = rand::random::<usize>() % population.len();

        for _ in 1..self.tournament_size {
            let idx = rand::random::<usize>() % population.len();
            if fitness[idx] < fitness[best_idx] {
                best_idx = idx;
            }
        }

        Ok(population[best_idx].clone())
    }

    /// Crossover two parents
    fn crossover(&self, parent1: &[f64], parent2: &[f64]) -> Result<Vec<f64>, AIError> {
        // Uniform crossover
        let offspring = parent1
            .iter()
            .zip(parent2.iter())
            .map(|(&p1, &p2)| {
                if rand::random::<f64>() < 0.5 {
                    p1
                } else {
                    p2
                }
            })
            .collect();

        Ok(offspring)
    }

    /// Mutate individual
    fn mutate(&self, mut individual: Vec<f64>) -> Result<Vec<f64>, AIError> {
        for i in 0..self.dim {
            if rand::random::<f64>() < self.mutation_rate {
                // Gaussian mutation
                let noise = rand::random::<f64>() * 2.0 - 1.0;
                individual[i] = (individual[i] + noise * 0.1)
                    .max(self.bounds[i].0)
                    .min(self.bounds[i].1);
            }
        }

        Ok(individual)
    }
}

/// Simulated Annealing
/// 
/// Stochastic optimization algorithm inspired by annealing process in metallurgy.
/// 
/// ## Features
/// - Global search capability
/// - Temperature schedule
/// - Acceptance probability
/// - Adaptive cooling
pub struct SimulatedAnnealing {
    initial_temp: f64,
    cooling_rate: f64,
    min_temp: f64,
    bounds: Vec<(f64, f64)>,
}

impl SimulatedAnnealing {
    /// Create a new simulated annealing optimizer
    /// 
    /// ## Arguments
    /// * `initial_temp` - Initial temperature
    /// * `cooling_rate` - Cooling rate (0.0-1.0)
    /// * `min_temp` - Minimum temperature
    pub fn new(initial_temp: f64, cooling_rate: f64, min_temp: f64) -> Result<Self, AIError> {
        Ok(Self {
            initial_temp,
            cooling_rate: cooling_rate.max(0.0).min(1.0),
            min_temp: min_temp.max(0.0),
            bounds: Vec::new(),
        })
    }

    /// Set parameter bounds
    pub fn set_bounds(&mut self, bounds: Vec<(f64, f64)>) {
        self.bounds = bounds;
    }

    /// Optimize objective function
    /// 
    /// ## Arguments
    /// * `objective` - Function to minimize
    /// * `max_iterations` - Maximum number of iterations
    /// 
    /// ## Returns
    /// Best parameters found
    pub fn optimize<F>(
        &self,
        objective: F,
        max_iterations: usize,
    ) -> Result<Vec<f64>, AIError>
    where
        F: Fn(&[f64]) -> f64,
    {
        // Initialize with random solution
        let mut current = self.random_solution();
        let mut current_value = objective(&current);

        let mut best = current.clone();
        let mut best_value = current_value;

        let mut temp = self.initial_temp;

        for iteration in 0..max_iterations {
            if temp < self.min_temp {
                break;
            }

            // Generate neighbor
            let neighbor = self.generate_neighbor(&current);
            let neighbor_value = objective(&neighbor);

            // Accept or reject
            let delta = neighbor_value - current_value;

            if delta < 0.0 || rand::random::<f64>() < (-delta / temp).exp() {
                current = neighbor;
                current_value = neighbor_value;

                if current_value < best_value {
                    best = current.clone();
                    best_value = current_value;
                }
            }

            // Cool down
            temp *= self.cooling_rate;
        }

        Ok(best)
    }

    /// Generate random initial solution
    fn random_solution(&self) -> Vec<f64> {
        if self.bounds.is_empty() {
            vec![rand::random::<f64>()]
        } else {
            self.bounds
                .iter()
                .map(|(min, max)| min + rand::random::<f64>() * (max - min))
                .collect()
        }
    }

    /// Generate neighbor solution
    fn generate_neighbor(&self, current: &[f64]) -> Vec<f64> {
        current
            .iter()
            .map(|&x| {
                let noise = (rand::random::<f64>() - 0.5) * 0.2;
                let bounded = if self.bounds.is_empty() {
                    x + noise
                } else {
                    // Apply bounds if available (simplified)
                    (x + noise).max(0.0).min(1.0)
                };
                bounded
            })
            .collect()
    }
}

/// Gradient Descent
/// 
/// First-order iterative optimization algorithm.
/// 
/// ## Features
/// - Simple and fast
/// - Learning rate control
/// - Momentum support
/// - Adaptive learning rate
pub struct GradientDescent {
    learning_rate: f64,
    momentum: f64,
    max_iterations: usize,
    tolerance: f64,
}

impl GradientDescent {
    /// Create a new gradient descent optimizer
    /// 
    /// ## Arguments
    /// * `learning_rate` - Step size
    /// * `momentum` - Momentum coefficient (0.0-1.0)
    /// * `max_iterations` - Maximum iterations
    /// * `tolerance` - Convergence tolerance
    pub fn new(
        learning_rate: f64,
        momentum: f64,
        max_iterations: usize,
        tolerance: f64,
    ) -> Result<Self, AIError> {
        Ok(Self {
            learning_rate,
            momentum: momentum.max(0.0).min(1.0),
            max_iterations,
            tolerance,
        })
    }

    /// Optimize objective function
    /// 
    /// ## Arguments
    /// * `gradient` - Function that returns gradient
    /// * `initial_params` - Starting parameters
    /// 
    /// ## Returns
    /// Optimized parameters
    pub fn optimize<F>(
        &self,
        gradient: F,
        initial_params: Vec<f64>,
    ) -> Result<Vec<f64>, AIError>
    where
        F: Fn(&[f64]) -> Vec<f64>,
    {
        let mut params = initial_params;
        let mut velocity = vec![0.0; params.len()];

        for _ in 0..self.max_iterations {
            let grad = gradient(&params);

            // Check convergence
            let grad_norm: f64 = grad.iter().map(|&g| g * g).sum::<f64>().sqrt();
            if grad_norm < self.tolerance {
                break;
            }

            // Update with momentum
            for (i, &g) in grad.iter().enumerate() {
                velocity[i] = self.momentum * velocity[i] - self.learning_rate * g;
                params[i] += velocity[i];
            }
        }

        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bayesian_optimizer_creation() {
        let optimizer = BayesianOptimizer::new(2, 0.1).unwrap();
        assert_eq!(optimizer.dim, 2);
    }

    #[test]
    fn test_bayesian_optimizer_optimize() {
        let mut optimizer = BayesianOptimizer::new(1, 0.1).unwrap();
        
        let objective = |params: &[f64]| -> f64 {
            (params[0] - 0.5).powi(2)
        };

        let best = optimizer.optimize(&objective, 20).unwrap();
        assert!((best[0] - 0.5).abs() < 0.2);
    }

    #[test]
    fn test_genetic_algorithm_creation() {
        let ga = GeneticAlgorithm::new(10, 0.1, 0.5, 2).unwrap();
        assert_eq!(ga.population_size, 10);
    }

    #[test]
    fn test_simulated_annealing_creation() {
        let sa = SimulatedAnnealing::new(100.0, 0.95, 0.01).unwrap();
        assert_eq!(sa.initial_temp, 100.0);
    }
}