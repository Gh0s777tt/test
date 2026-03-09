//! Multi-Objective Optimization Module for VantisOS
//! 
//! This module implements advanced multi-objective optimization algorithms
//! including Pareto frontier analysis, NSGA-II, and weighted sum approaches.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, BTreeMap};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ============================================================================
// Core Types
// ============================================================================

/// Represents a single objective in multi-objective optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    /// Unique identifier for the objective
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of what this objective measures
    pub description: String,
    /// Whether this objective should be minimized or maximized
    pub direction: ObjectiveDirection,
    /// Weight for weighted sum approaches (0.0 to 1.0)
    pub weight: f64,
    /// Priority level for lexicographic optimization
    pub priority: Option<u32>,
    /// Target value for goal programming
    pub target: Option<f64>,
    /// Threshold for constraint handling
    pub threshold: Option<f64>,
    /// Whether this objective is active
    pub active: bool,
}

/// Direction of optimization for an objective
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectiveDirection {
    /// Minimize the objective value
    Minimize,
    /// Maximize the objective value
    Maximize,
}

/// Represents a solution in the objective space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    /// Unique identifier
    pub id: String,
    /// Decision variable values
    pub variables: HashMap<String, f64>,
    /// Objective function values
    pub objectives: HashMap<String, f64>,
    /// Whether this solution is Pareto optimal
    pub is_pareto_optimal: bool,
    /// Crowding distance (for NSGA-II)
    pub crowding_distance: f64,
    /// Rank in Pareto frontier
    pub pareto_rank: Option<u32>,
    /// Metadata about the solution
    pub metadata: SolutionMetadata,
}

/// Metadata about a solution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolutionMetadata {
    /// When this solution was generated
    pub created_at: DateTime<Utc>,
    /// Algorithm that generated this solution
    pub algorithm: String,
    /// Number of generations/iterations
    pub generations: u32,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
    /// Custom metadata
    pub custom: HashMap<String, String>,
}

/// Represents the Pareto frontier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParetoFrontier {
    /// Solutions on the Pareto frontier
    pub solutions: Vec<Solution>,
    /// Objectives considered
    pub objectives: Vec<String>,
    /// Hypervolume indicator
    pub hypervolume: f64,
    /// Spread indicator
    pub spread: f64,
    /// Convergence metric
    pub convergence: f64,
    /// When this frontier was computed
    pub computed_at: DateTime<Utc>,
}

/// Configuration for multi-objective optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiObjectiveConfig {
    /// Maximum number of solutions to maintain
    pub max_solutions: usize,
    /// Maximum number of generations
    pub max_generations: u32,
    /// Convergence threshold
    pub convergence_threshold: f64,
    /// Population size for evolutionary algorithms
    pub population_size: usize,
    /// Mutation probability
    pub mutation_probability: f64,
    /// Crossover probability
    pub crossover_probability: f64,
    /// Tournament size for selection
    pub tournament_size: usize,
    /// Whether to use adaptive operators
    pub adaptive_operators: bool,
    /// Reference point for hypervolume calculation
    pub reference_point: HashMap<String, f64>,
    /// Algorithm to use
    pub algorithm: MOOAlgorithm,
    /// Constraint handling method
    pub constraint_handling: ConstraintHandling,
}

/// Multi-objective optimization algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MOOAlgorithm {
    /// Non-dominated Sorting Genetic Algorithm II
    NSGA2,
    /// NSGA-III for many-objective optimization
    NSGA3,
    /// Strength Pareto Evolutionary Algorithm 2
    SPEA2,
    /// Pareto Archived Evolution Strategy
    PAES,
    /// Weighted Sum Approach
    WeightedSum,
    /// Epsilon-Constraint Method
    EpsilonConstraint,
    /// Goal Programming
    GoalProgramming,
    /// Normal Boundary Intersection
    NBI,
    /// Multi-Objective Evolutionary Algorithm based on Decomposition
    MOEAD,
    /// Indicator-Based Evolutionary Algorithm
    IBEA,
}

/// Constraint handling methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintHandling {
    /// Penalty function approach
    Penalty,
    /// Constraint domination
    ConstraintDomination,
    /// Stochastic ranking
    StochasticRanking,
    /// Feasibility rules
    FeasibilityRules,
    /// Epsilon level comparison
    EpsilonLevel,
}

/// Result of multi-objective optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MOOResult {
    /// Unique identifier
    pub id: String,
    /// Pareto frontier
    pub pareto_frontier: ParetoFrontier,
    /// Best compromise solution
    pub best_compromise: Option<Solution>,
    /// Selected solutions based on preferences
    pub selected_solutions: Vec<Solution>,
    /// Optimization statistics
    pub statistics: MOOStatistics,
    /// When this result was generated
    pub created_at: DateTime<Utc>,
}

/// Statistics for multi-objective optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MOOStatistics {
    /// Total number of evaluations
    pub evaluations: u64,
    /// Number of generations
    pub generations: u32,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
    /// Number of Pareto optimal solutions found
    pub pareto_solutions: usize,
    /// Final hypervolume
    pub final_hypervolume: f64,
    /// Hypervolume improvement over generations
    pub hypervolume_history: Vec<f64>,
    /// Number of non-dominated fronts
    pub num_fronts: usize,
    /// Average crowding distance
    pub avg_crowding_distance: f64,
    /// Convergence achieved
    pub converged: bool,
}

// ============================================================================
// Dominance Relations
// ============================================================================

/// Dominance relationship between solutions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DominanceRelation {
    /// First solution dominates the second
    Dominates,
    /// Second solution dominates the first
    Dominated,
    /// Solutions are incomparable (neither dominates)
    Incomparable,
    /// Solutions are equal in all objectives
    Equal,
}

/// Calculate dominance relation between two solutions
pub fn calculate_dominance(
    sol1: &Solution,
    sol2: &Solution,
    objectives: &HashMap<String, Objective>,
) -> DominanceRelation {
    let mut dominates_in_any = false;
    let mut dominated_in_any = false;
    
    for (obj_id, objective) in objectives {
        if !objective.active {
            continue;
        }
        
        let val1 = sol1.objectives.get(obj_id).copied().unwrap_or(0.0);
        let val2 = sol2.objectives.get(obj_id).copied().unwrap_or(0.0);
        
        let (better1, better2) = match objective.direction {
            ObjectiveDirection::Minimize => (val1 < val2, val2 < val1),
            ObjectiveDirection::Maximize => (val1 > val2, val2 > val1),
        };
        
        if better1 {
            dominates_in_any = true;
        }
        if better2 {
            dominated_in_any = true;
        }
    }
    
    match (dominates_in_any, dominated_in_any) {
        (true, false) => DominanceRelation::Dominates,
        (false, true) => DominanceRelation::Dominated,
        (false, false) => DominanceRelation::Equal,
        (true, true) => DominanceRelation::Incomparable,
    }
}

// ============================================================================
// Multi-Objective Optimizer
// ============================================================================

/// Main multi-objective optimizer
pub struct MultiObjectiveOptimizer {
    /// Configuration
    config: MultiObjectiveConfig,
    /// Objectives to optimize
    objectives: HashMap<String, Objective>,
    /// Current population of solutions
    population: Vec<Solution>,
    /// Pareto frontier
    pareto_frontier: Option<ParetoFrontier>,
    /// Generation counter
    generation: u32,
    /// Statistics
    statistics: MOOStatistics,
}

impl MultiObjectiveOptimizer {
    /// Create a new multi-objective optimizer
    pub fn new(config: MultiObjectiveConfig) -> Self {
        Self {
            statistics: MOOStatistics {
                evaluations: 0,
                generations: 0,
                computation_time_ms: 0,
                pareto_solutions: 0,
                final_hypervolume: 0.0,
                hypervolume_history: Vec::new(),
                num_fronts: 0,
                avg_crowding_distance: 0.0,
                converged: false,
            },
            config,
            objectives: HashMap::new(),
            population: Vec::new(),
            pareto_frontier: None,
            generation: 0,
        }
    }
    
    /// Add an objective to the optimization problem
    pub fn add_objective(&mut self, objective: Objective) {
        self.objectives.insert(objective.id.clone(), objective);
    }
    
    /// Remove an objective from the optimization problem
    pub fn remove_objective(&mut self, objective_id: &str) -> Option<Objective> {
        self.objectives.remove(objective_id)
    }
    
    /// Initialize the population with random solutions
    pub fn initialize_population(&mut self, variable_bounds: &HashMap<String, (f64, f64)>) {
        self.population.clear();
        
        for _ in 0..self.config.population_size {
            let mut variables = HashMap::new();
            for (var_name, (min, max)) in variable_bounds {
                let value = min + (max - min) * rand_value();
                variables.insert(var_name.clone(), value);
            }
            
            let solution = Solution {
                id: Uuid::new_v4().to_string(),
                variables,
                objectives: HashMap::new(),
                is_pareto_optimal: false,
                crowding_distance: 0.0,
                pareto_rank: None,
                metadata: SolutionMetadata {
                    created_at: Utc::now(),
                    algorithm: format!("{:?}", self.config.algorithm),
                    generations: 0,
                    computation_time_ms: 0,
                    custom: HashMap::new(),
                },
            };
            
            self.population.push(solution);
        }
    }
    
    /// Run the optimization algorithm
    pub fn optimize(&mut self, 
        evaluate: &dyn Fn(&HashMap<String, f64>) -> HashMap<String, f64>,
        variable_bounds: &HashMap<String, (f64, f64)>,
    ) -> MOOResult {
        let start_time = std::time::Instant::now();
        
        // Initialize population if empty
        if self.population.is_empty() {
            self.initialize_population(variable_bounds);
        }
        
        // Evaluate initial population
        self.evaluate_population(evaluate);
        
        // Run algorithm-specific optimization
        match self.config.algorithm {
            MOOAlgorithm::NSGA2 => self.run_nsga2(evaluate, variable_bounds),
            MOOAlgorithm::NSGA3 => self.run_nsga3(evaluate, variable_bounds),
            MOOAlgorithm::SPEA2 => self.run_spea2(evaluate, variable_bounds),
            MOOAlgorithm::WeightedSum => self.run_weighted_sum(evaluate, variable_bounds),
            _ => self.run_nsga2(evaluate, variable_bounds), // Default to NSGA-II
        }
        
        // Compute final Pareto frontier
        self.compute_pareto_frontier();
        
        // Find best compromise solution
        let best_compromise = self.find_best_compromise();
        
        // Update statistics
        self.statistics.computation_time_ms = start_time.elapsed().as_millis() as u64;
        self.statistics.pareto_solutions = self.pareto_frontier
            .as_ref()
            .map(|pf| pf.solutions.len())
            .unwrap_or(0);
        
        MOOResult {
            id: Uuid::new_v4().to_string(),
            pareto_frontier: self.pareto_frontier.clone().unwrap_or_else(|| ParetoFrontier {
                solutions: Vec::new(),
                objectives: self.objectives.keys().cloned().collect(),
                hypervolume: 0.0,
                spread: 0.0,
                convergence: 0.0,
                computed_at: Utc::now(),
            }),
            best_compromise,
            selected_solutions: Vec::new(),
            statistics: self.statistics.clone(),
            created_at: Utc::now(),
        }
    }
    
    /// Evaluate all solutions in the population
    fn evaluate_population(&mut self, evaluate: &dyn Fn(&HashMap<String, f64>) -> HashMap<String, f64>) {
        for solution in &mut self.population {
            solution.objectives = evaluate(&solution.variables);
            self.statistics.evaluations += 1;
        }
    }
    
    /// Run NSGA-II algorithm
    fn run_nsga2(&mut self, 
        evaluate: &dyn Fn(&HashMap<String, f64>) -> HashMap<String, f64>,
        _variable_bounds: &HashMap<String, (f64, f64)>,
    ) {
        for gen in 0..self.config.max_generations {
            self.generation = gen;
            self.statistics.generations = gen + 1;
            
            // Non-dominated sorting
            let fronts = self.non_dominated_sort();
            
            // Calculate crowding distance for each front
            for front in &fronts {
                self.calculate_crowding_distance(&front.indices);
            }
            
            // Create offspring through selection, crossover, and mutation
            let offspring = self.create_offspring();
            
            // Evaluate offspring
            for mut child in offspring {
                child.objectives = evaluate(&child.variables);
                self.statistics.evaluations += 1;
                self.population.push(child);
            }
            
            // Non-dominated sorting of combined population
            let combined_fronts = self.non_dominated_sort();
            
            // Select new population
            self.select_new_population(combined_fronts);
            
            // Calculate and store hypervolume
            let hv = self.calculate_hypervolume();
            self.statistics.hypervolume_history.push(hv);
            
            // Check convergence
            if self.check_convergence() {
                self.statistics.converged = true;
                break;
            }
        }
    }
    
    /// Run NSGA-III algorithm
    fn run_nsga3(&mut self, 
        evaluate: &dyn Fn(&HashMap<String, f64>) -> HashMap<String, f64>,
        _variable_bounds: &HashMap<String, (f64, f64)>,
    ) {
        // Generate reference points for NSGA-III
        let _reference_points = self.generate_reference_points();
        
        for gen in 0..self.config.max_generations {
            self.generation = gen;
            self.statistics.generations = gen + 1;
            
            // Similar to NSGA-II but with reference point-based selection
            let fronts = self.non_dominated_sort();
            
            for front in &fronts {
                self.calculate_crowding_distance(&front.indices);
            }
            
            let offspring = self.create_offspring();
            
            for mut child in offspring {
                child.objectives = evaluate(&child.variables);
                self.statistics.evaluations += 1;
                self.population.push(child);
            }
            
            let combined_fronts = self.non_dominated_sort();
            self.select_new_population_nsga3(combined_fronts);
            
            let hv = self.calculate_hypervolume();
            self.statistics.hypervolume_history.push(hv);
            
            if self.check_convergence() {
                self.statistics.converged = true;
                break;
            }
        }
    }
    
    /// Run SPEA2 algorithm
    fn run_spea2(&mut self, 
        evaluate: &dyn Fn(&HashMap<String, f64>) -> HashMap<String, f64>,
        _variable_bounds: &HashMap<String, (f64, f64)>,
    ) {
        let mut archive: Vec<Solution> = Vec::new();
        let archive_size = self.config.population_size / 2;
        
        for gen in 0..self.config.max_generations {
            self.generation = gen;
            self.statistics.generations = gen + 1;
            
            // Calculate fitness (strength) for all solutions
            self.calculate_spea2_fitness(&mut archive);
            
            // Environmental selection
            archive = self.spea2_environmental_selection(archive, archive_size);
            
            // Mating selection
            let parents = self.spea2_mating_selection(&archive);
            
            // Create offspring
            let offspring = self.create_offspring_from_parents(&parents);
            
            // Evaluate offspring
            for mut child in offspring {
                child.objectives = evaluate(&child.variables);
                self.statistics.evaluations += 1;
            }
            
            // Combine archive and offspring
            self.population = archive.clone();
            for child in offspring {
                self.population.push(child);
            }
            
            let hv = self.calculate_hypervolume();
            self.statistics.hypervolume_history.push(hv);
            
            if self.check_convergence() {
                self.statistics.converged = true;
                break;
            }
        }
    }
    
    /// Run weighted sum approach
    fn run_weighted_sum(&mut self,
        evaluate: &dyn Fn(&HashMap<String, f64>) -> HashMap<String, f64>,
        _variable_bounds: &HashMap<String, (f64, f64)>,
    ) {
        let weight_vectors = self.generate_weight_vectors();
        
        for gen in 0..self.config.max_generations {
            self.generation = gen;
            self.statistics.generations = gen + 1;
            
            // For each weight vector, find best solution
            for weights in &weight_vectors {
                for solution in &mut self.population {
                    let weighted_sum = self.calculate_weighted_sum(solution, weights);
                    solution.metadata.custom.insert("weighted_sum".to_string(), weighted_sum.to_string());
                }
            }
            
            // Select best solutions for each weight combination
            self.population.sort_by(|a, b| {
                let ws_a: f64 = a.metadata.custom.get("weighted_sum")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                let ws_b: f64 = b.metadata.custom.get("weighted_sum")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                ws_a.partial_cmp(&ws_b).unwrap_or(std::cmp::Ordering::Equal)
            });
            
            // Create and evaluate offspring
            let offspring = self.create_offspring();
            for mut child in offspring {
                child.objectives = evaluate(&child.variables);
                self.statistics.evaluations += 1;
                self.population.push(child);
            }
            
            // Keep best solutions
            self.population.truncate(self.config.population_size);
            
            let hv = self.calculate_hypervolume();
            self.statistics.hypervolume_history.push(hv);
            
            if self.check_convergence() {
                self.statistics.converged = true;
                break;
            }
        }
    }
    
    /// Non-dominated sorting
    fn non_dominated_sort(&mut self) -> Vec<Front> {
        let n = self.population.len();
        let mut domination_count = vec![0usize; n];
        let mut dominated_solutions: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut fronts: Vec<Front> = Vec::new();
        
        // Calculate domination counts
        for i in 0..n {
            for j in (i + 1)..n {
                let relation = calculate_dominance(
                    &self.population[i],
                    &self.population[j],
                    &self.objectives,
                );
                
                match relation {
                    DominanceRelation::Dominates => {
                        dominated_solutions[i].push(j);
                        domination_count[j] += 1;
                    }
                    DominanceRelation::Dominated => {
                        dominated_solutions[j].push(i);
                        domination_count[i] += 1;
                    }
                    _ => {}
                }
            }
        }
        
        // Find first front
        let mut current_front: Vec<usize> = domination_count
            .iter()
            .enumerate()
            .filter(|(_, &count)| count == 0)
            .map(|(i, _)| i)
            .collect();
        
        let mut front_rank = 0u32;
        
        while !current_front.is_empty() {
            for &idx in &current_front {
                self.population[idx].pareto_rank = Some(front_rank);
            }
            
            fronts.push(Front {
                indices: current_front.clone(),
                rank: front_rank,
            });
            
            let mut next_front = Vec::new();
            for &idx in &current_front {
                for &dominated_idx in &dominated_solutions[idx] {
                    domination_count[dominated_idx] -= 1;
                    if domination_count[dominated_idx] == 0 {
                        next_front.push(dominated_idx);
                    }
                }
            }
            
            current_front = next_front;
            front_rank += 1;
        }
        
        self.statistics.num_fronts = fronts.len();
        fronts
    }
    
    /// Calculate crowding distance for solutions in a front
    fn calculate_crowding_distance(&mut self, front_indices: &[usize]) {
        if front_indices.len() <= 2 {
            for &idx in front_indices {
                self.population[idx].crowding_distance = f64::INFINITY;
            }
            return;
        }
        
        // Initialize distances to zero
        for &idx in front_indices {
            self.population[idx].crowding_distance = 0.0;
        }
        
        // Calculate distance for each objective
        for (obj_id, objective) in &self.objectives {
            if !objective.active {
                continue;
            }
            
            // Sort front by this objective
            let mut sorted_indices = front_indices.to_vec();
            sorted_indices.sort_by(|&a, &b| {
                let val_a = self.population[a].objectives.get(obj_id).copied().unwrap_or(0.0);
                let val_b = self.population[b].objectives.get(obj_id).copied().unwrap_or(0.0);
                val_a.partial_cmp(&val_b).unwrap_or(std::cmp::Ordering::Equal)
            });
            
            // Boundary solutions get infinite distance
            self.population[sorted_indices[0]].crowding_distance = f64::INFINITY;
            self.population[sorted_indices[sorted_indices.len() - 1]].crowding_distance = f64::INFINITY;
            
            // Calculate normalized distance
            let min_val = self.population[sorted_indices[0]]
                .objectives.get(obj_id)
                .copied()
                .unwrap_or(0.0);
            let max_val = self.population[sorted_indices[sorted_indices.len() - 1]]
                .objectives.get(obj_id)
                .copied()
                .unwrap_or(0.0);
            
            let range = if (max_val - min_val).abs() < 1e-10 {
                1.0
            } else {
                max_val - min_val
            };
            
            for i in 1..(sorted_indices.len() - 1) {
                let prev_val = self.population[sorted_indices[i - 1]]
                    .objectives.get(obj_id)
                    .copied()
                    .unwrap_or(0.0);
                let next_val = self.population[sorted_indices[i + 1]]
                    .objectives.get(obj_id)
                    .copied()
                    .unwrap_or(0.0);
                
                self.population[sorted_indices[i]].crowding_distance +=
                    (next_val - prev_val) / range;
            }
        }
    }
    
    /// Create offspring through genetic operators
    fn create_offspring(&self) -> Vec<Solution> {
        let mut offspring = Vec::new();
        
        while offspring.len() < self.config.population_size {
            // Tournament selection
            let parent1 = self.tournament_selection();
            let parent2 = self.tournament_selection();
            
            // Crossover
            if rand_value() < self.config.crossover_probability {
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
        
        offspring.truncate(self.config.population_size);
        offspring
    }
    
    /// Tournament selection
    fn tournament_selection(&self) -> &Solution {
        let tournament_indices: Vec<usize> = (0..self.config.tournament_size)
            .map(|_| (rand_value() * self.population.len() as f64) as usize)
            .collect();
        
        let best_idx = tournament_indices
            .iter()
            .copied()
            .min_by(|&a, &b| {
                let rank_a = self.population[a].pareto_rank.unwrap_or(u32::MAX);
                let rank_b = self.population[b].pareto_rank.unwrap_or(u32::MAX);
                
                match rank_a.cmp(&rank_b) {
                    std::cmp::Ordering::Equal => {
                        let dist_a = self.population[a].crowding_distance;
                        let dist_b = self.population[b].crowding_distance;
                        dist_b.partial_cmp(&dist_a).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    other => other,
                }
            })
            .unwrap_or(0);
        
        &self.population[best_idx]
    }
    
    /// Simulated binary crossover
    fn crossover(&self, parent1: &Solution, parent2: &Solution) -> (Solution, Solution) {
        let mut child1_vars = HashMap::new();
        let mut child2_vars = HashMap::new();
        
        let eta = 20.0; // Distribution index
        
        for (key, val1) in &parent1.variables {
            let val2 = parent2.variables.get(key).copied().unwrap_or(*val1);
            
            if rand_value() < 0.5 {
                child1_vars.insert(key.clone(), *val1);
                child2_vars.insert(key.clone(), val2);
            } else {
                let u = rand_value();
                let beta = if u <= 0.5 {
                    (2.0 * u).powf(1.0 / (eta + 1.0))
                } else {
                    (1.0 / (2.0 * (1.0 - u))).powf(1.0 / (eta + 1.0))
                };
                
                let avg = (val1 + val2) / 2.0;
                let diff = (val2 - val1).abs() / 2.0;
                
                child1_vars.insert(key.clone(), avg + beta * diff);
                child2_vars.insert(key.clone(), avg - beta * diff);
            }
        }
        
        let now = Utc::now();
        (
            Solution {
                id: Uuid::new_v4().to_string(),
                variables: child1_vars,
                objectives: HashMap::new(),
                is_pareto_optimal: false,
                crowding_distance: 0.0,
                pareto_rank: None,
                metadata: SolutionMetadata {
                    created_at: now,
                    algorithm: format!("{:?}", self.config.algorithm),
                    generations: self.generation,
                    computation_time_ms: 0,
                    custom: HashMap::new(),
                },
            },
            Solution {
                id: Uuid::new_v4().to_string(),
                variables: child2_vars,
                objectives: HashMap::new(),
                is_pareto_optimal: false,
                crowding_distance: 0.0,
                pareto_rank: None,
                metadata: SolutionMetadata {
                    created_at: now,
                    algorithm: format!("{:?}", self.config.algorithm),
                    generations: self.generation,
                    computation_time_ms: 0,
                    custom: HashMap::new(),
                },
            },
        )
    }
    
    /// Polynomial mutation
    fn mutate(&self, solution: &mut Solution) {
        let eta = 20.0; // Distribution index
        
        for (_key, value) in solution.variables.iter_mut() {
            if rand_value() < self.config.mutation_probability {
                let u = rand_value();
                let delta = if u < 0.5 {
                    (2.0 * u).powf(1.0 / (eta + 1.0)) - 1.0
                } else {
                    1.0 - (2.0 * (1.0 - u)).powf(1.0 / (eta + 1.0))
                };
                
                *value += delta;
            }
        }
    }
    
    /// Select new population from combined fronts
    fn select_new_population(&mut self, fronts: Vec<Front>) {
        let mut new_population = Vec::new();
        let mut count = 0;
        
        for front in fronts {
            if count + front.indices.len() <= self.config.population_size {
                // Add entire front
                for idx in front.indices {
                    new_population.push(self.population[idx].clone());
                }
                count += front.indices.len();
            } else {
                // Sort front by crowding distance and add remaining
                let mut sorted_front = front.indices.clone();
                sorted_front.sort_by(|&a, &b| {
                    let dist_a = self.population[a].crowding_distance;
                    let dist_b = self.population[b].crowding_distance;
                    dist_b.partial_cmp(&dist_a).unwrap_or(std::cmp::Ordering::Equal)
                });
                
                for idx in sorted_front {
                    if count >= self.config.population_size {
                        break;
                    }
                    new_population.push(self.population[idx].clone());
                    count += 1;
                }
                break;
            }
        }
        
        self.population = new_population;
    }
    
    /// Select new population for NSGA-III
    fn select_new_population_nsga3(&mut self, fronts: Vec<Front>) {
        // Simplified NSGA-III selection - uses crowding distance as fallback
        self.select_new_population(fronts);
    }
    
    /// Generate reference points for NSGA-III
    fn generate_reference_points(&self) -> Vec<HashMap<String, f64>> {
        let num_objectives = self.objectives.len();
        let mut points = Vec::new();
        
        // Generate Das-Dennis reference points
        let divisions = 12; // Number of divisions per objective
        
        for i in 0..=divisions {
            for j in 0..=(divisions - i) {
                let mut point = HashMap::new();
                let mut idx = 0;
                
                for obj_id in self.objectives.keys() {
                    let value = if idx == 0 {
                        i as f64 / divisions as f64
                    } else if idx == 1 {
                        j as f64 / divisions as f64
                    } else {
                        (divisions - i - j) as f64 / divisions as f64
                    };
                    point.insert(obj_id.clone(), value);
                    idx += 1;
                }
                
                points.push(point);
            }
        }
        
        points
    }
    
    /// Generate weight vectors for weighted sum approach
    fn generate_weight_vectors(&self) -> Vec<HashMap<String, f64>> {
        let num_objectives = self.objectives.len();
        let num_weights = 10; // Number of weight combinations
        let mut vectors = Vec::new();
        
        for i in 0..num_weights {
            let mut weights = HashMap::new();
            let t = i as f64 / (num_weights - 1) as f64;
            
            for (idx, obj_id) in self.objectives.keys().enumerate() {
                let weight = if idx == 0 {
                    t
                } else if idx == 1 && num_objectives == 2 {
                    1.0 - t
                } else {
                    (1.0 - t) / (num_objectives - 1) as f64
                };
                weights.insert(obj_id.clone(), weight);
            }
            
            vectors.push(weights);
        }
        
        vectors
    }
    
    /// Calculate weighted sum for a solution
    fn calculate_weighted_sum(&self, solution: &Solution, weights: &HashMap<String, f64>) -> f64 {
        let mut sum = 0.0;
        
        for (obj_id, objective) in &self.objectives {
            if !objective.active {
                continue;
            }
            
            let value = solution.objectives.get(obj_id).copied().unwrap_or(0.0);
            let weight = weights.get(obj_id).copied().unwrap_or(objective.weight);
            
            let normalized = match objective.direction {
                ObjectiveDirection::Minimize => -value * weight,
                ObjectiveDirection::Maximize => value * weight,
            };
            
            sum += normalized;
        }
        
        sum
    }
    
    /// Calculate hypervolume indicator
    fn calculate_hypervolume(&self) -> f64 {
        if self.population.is_empty() {
            return 0.0;
        }
        
        // Get Pareto optimal solutions
        let pareto_solutions: Vec<&Solution> = self.population
            .iter()
            .filter(|s| s.pareto_rank == Some(0))
            .collect();
        
        if pareto_solutions.is_empty() {
            return 0.0;
        }
        
        // Simplified 2D hypervolume calculation
        if self.objectives.len() == 2 {
            let mut volume = 0.0;
            
            let obj_ids: Vec<&String> = self.objectives.keys().collect();
            let ref_x = self.config.reference_point.get(obj_ids[0]).copied().unwrap_or(0.0);
            let ref_y = self.config.reference_point.get(obj_ids[1]).copied().unwrap_or(0.0);
            
            let mut points: Vec<(f64, f64)> = pareto_solutions
                .iter()
                .map(|s| {
                    let x = s.objectives.get(obj_ids[0]).copied().unwrap_or(0.0);
                    let y = s.objectives.get(obj_ids[1]).copied().unwrap_or(0.0);
                    (x, y)
                })
                .collect();
            
            points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            
            let mut prev_x = 0.0;
            for (x, y) in points {
                let dx = x - prev_x.max(ref_x);
                let dy = y - ref_y;
                
                if dx > 0.0 && dy > 0.0 {
                    volume += dx * dy;
                }
                
                prev_x = x;
            }
            
            return volume;
        }
        
        // For higher dimensions, use Monte Carlo estimation
        let num_samples = 10000;
        let mut dominated_count = 0;
        
        // Find bounding box
        let mut mins = HashMap::new();
        let mut maxs = HashMap::new();
        
        for obj_id in self.objectives.keys() {
            let values: Vec<f64> = pareto_solutions
                .iter()
                .filter_map(|s| s.objectives.get(obj_id).copied())
                .collect();
            
            mins.insert(obj_id.clone(), values.iter().cloned().fold(f64::INFINITY, f64::min));
            maxs.insert(obj_id.clone(), values.iter().cloned::fold(f64::NEG_INFINITY, f64::max));
        }
        
        // Monte Carlo sampling
        for _ in 0..num_samples {
            let mut sample = HashMap::new();
            for obj_id in self.objectives.keys() {
                let min = mins.get(obj_id).copied().unwrap_or(0.0);
                let max = maxs.get(obj_id).copied().unwrap_or(1.0);
                sample.insert(obj_id.clone(), min + (max - min) * rand_value());
            }
            
            // Check if sample is dominated by any Pareto solution
            for sol in &pareto_solutions {
                let mut dominated = true;
                for (obj_id, objective) in &self.objectives {
                    let sol_val = sol.objectives.get(obj_id).copied().unwrap_or(0.0);
                    let sample_val = sample.get(obj_id).copied().unwrap_or(0.0);
                    
                    match objective.direction {
                        ObjectiveDirection::Minimize => {
                            if sample_val < sol_val {
                                dominated = false;
                                break;
                            }
                        }
                        ObjectiveDirection::Maximize => {
                            if sample_val > sol_val {
                                dominated = false;
                                break;
                            }
                        }
                    }
                }
                
                if dominated {
                    dominated_count += 1;
                    break;
                }
            }
        }
        
        // Calculate hypervolume from Monte Carlo estimate
        let volume_ratio = dominated_count as f64 / num_samples as f64;
        
        let total_volume: f64 = self.objectives.keys()
            .map(|obj_id| {
                let min = mins.get(obj_id).copied().unwrap_or(0.0);
                let max = maxs.get(obj_id).copied().unwrap_or(1.0);
                (max - min).abs()
            })
            .product();
        
        volume_ratio * total_volume
    }
    
    /// Check convergence
    fn check_convergence(&self) -> bool {
        let history = &self.statistics.hypervolume_history;
        
        if history.len() < 10 {
            return false;
        }
        
        // Check if hypervolume improvement is below threshold
        let recent: Vec<f64> = history.iter().rev().take(10).cloned().collect();
        let improvement = recent.first().unwrap_or(&0.0) - recent.last().unwrap_or(&0.0);
        
        improvement < self.config.convergence_threshold
    }
    
    /// Compute the Pareto frontier
    fn compute_pareto_frontier(&mut self) {
        let pareto_solutions: Vec<Solution> = self.population
            .iter()
            .filter(|s| s.pareto_rank == Some(0))
            .cloned()
            .map(|mut s| {
                s.is_pareto_optimal = true;
                s
            })
            .collect();
        
        let hypervolume = self.calculate_hypervolume();
        
        self.pareto_frontier = Some(ParetoFrontier {
            solutions: pareto_solutions,
            objectives: self.objectives.keys().cloned().collect(),
            hypervolume,
            spread: 0.0, // Would calculate spread metric
            convergence: self.statistics.hypervolume_history.last().copied().unwrap_or(0.0),
            computed_at: Utc::now(),
        });
        
        self.statistics.final_hypervolume = hypervolume;
    }
    
    /// Find best compromise solution using TOPSIS-like approach
    fn find_best_compromise(&self) -> Option<Solution> {
        let frontier = self.pareto_frontier.as_ref()?;
        
        if frontier.solutions.is_empty() {
            return None;
        }
        
        // Calculate ideal point
        let mut ideal = HashMap::new();
        for obj_id in &frontier.objectives {
            let values: Vec<f64> = frontier.solutions
                .iter()
                .filter_map(|s| s.objectives.get(obj_id).copied())
                .collect();
            
            let objective = self.objectives.get(obj_id)?;
            let ideal_val = match objective.direction {
                ObjectiveDirection::Minimize => values.iter().cloned().fold(f64::INFINITY, f64::min),
                ObjectiveDirection::Maximize => values.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            };
            ideal.insert(obj_id.clone(), ideal_val);
        }
        
        // Calculate distance to ideal for each solution
        let mut best_solution = None;
        let mut min_distance = f64::INFINITY;
        
        for solution in &frontier.solutions {
            let mut distance = 0.0;
            
            for (obj_id, ideal_val) in &ideal {
                let value = solution.objectives.get(obj_id).copied().unwrap_or(0.0);
                let objective = self.objectives.get(obj_id)?;
                let weight = objective.weight;
                
                let diff = match objective.direction {
                    ObjectiveDirection::Minimize => value - ideal_val,
                    ObjectiveDirection::Maximize => ideal_val - value,
                };
                
                distance += weight * diff * diff;
            }
            
            distance = distance.sqrt();
            
            if distance < min_distance {
                min_distance = distance;
                best_solution = Some(solution.clone());
            }
        }
        
        best_solution
    }
    
    /// SPEA2 fitness calculation
    fn calculate_spea2_fitness(&self, archive: &mut Vec<Solution>) {
        let combined: Vec<&Solution> = self.population.iter()
            .chain(archive.iter())
            .collect();
        
        // Calculate strength (number of solutions dominated)
        let mut strengths = vec![0usize; combined.len()];
        
        for i in 0..combined.len() {
            for j in 0..combined.len() {
                if i != j {
                    let relation = calculate_dominance(combined[i], combined[j], &self.objectives);
                    if relation == DominanceRelation::Dominates {
                        strengths[i] += 1;
                    }
                }
            }
        }
        
        // Calculate raw fitness
        for i in 0..combined.len() {
            let mut raw_fitness = 0.0;
            
            for j in 0..combined.len() {
                if i != j {
                    let relation = calculate_dominance(combined[j], combined[i], &self.objectives);
                    if relation == DominanceRelation::Dominates {
                        raw_fitness += strengths[j] as f64;
                    }
                }
            }
            
            // Add density
            let k = 1;
            let mut distances: Vec<f64> = combined.iter()
                .enumerate()
                .filter(|&(idx, _)| idx != i)
                .map(|(_, s)| {
                    let mut dist = 0.0;
                    for obj_id in self.objectives.keys() {
                        let v1 = combined[i].objectives.get(obj_id).copied().unwrap_or(0.0);
                        let v2 = s.objectives.get(obj_id).copied().unwrap_or(0.0);
                        dist += (v1 - v2).powi(2);
                    }
                    dist.sqrt()
                })
                .collect();
            
            distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            
            let density = if distances.len() > k {
                1.0 / (distances[k] + 2.0)
            } else {
                1.0
            };
            
            // Store fitness in metadata (simplified)
            if i < self.population.len() {
                self.population[i].metadata.custom.insert(
                    "spea2_fitness".to_string(),
                    (raw_fitness + density).to_string(),
                );
            }
        }
    }
    
    /// SPEA2 environmental selection
    fn spea2_environmental_selection(&self, mut archive: Vec<Solution>, archive_size: usize) -> Vec<Solution> {
        // Select non-dominated solutions
        let mut new_archive: Vec<Solution> = archive.into_iter()
            .filter(|s| {
                s.metadata.custom.get("spea2_fitness")
                    .and_then(|v| v.parse::<f64>().ok())
                    .map(|f| f < 1.0)
                    .unwrap_or(false)
            })
            .collect();
        
        // Truncate if too many
        if new_archive.len() > archive_size {
            // Use truncation based on distances
            while new_archive.len() > archive_size {
                let mut min_dist = f64::INFINITY;
                let mut min_idx = 0;
                
                for i in 0..new_archive.len() {
                    let mut nearest_dist = f64::INFINITY;
                    
                    for j in 0..new_archive.len() {
                        if i != j {
                            let mut dist = 0.0;
                            for obj_id in self.objectives.keys() {
                                let v1 = new_archive[i].objectives.get(obj_id).copied().unwrap_or(0.0);
                                let v2 = new_archive[j].objectives.get(obj_id).copied().unwrap_or(0.0);
                                dist += (v1 - v2).powi(2);
                            }
                            nearest_dist = nearest_dist.min(dist.sqrt());
                        }
                    }
                    
                    if nearest_dist < min_dist {
                        min_dist = nearest_dist;
                        min_idx = i;
                    }
                }
                
                new_archive.remove(min_idx);
            }
        }
        
        new_archive
    }
    
    /// SPEA2 mating selection
    fn spea2_mating_selection(&self, archive: &[Solution]) -> Vec<&Solution> {
        let mut selected = Vec::new();
        
        for _ in 0..self.config.population_size / 2 {
            let i = (rand_value() * archive.len() as f64) as usize;
            let j = (rand_value() * archive.len() as f64) as usize;
            
            let fitness_i = archive[i].metadata.custom.get("spea2_fitness")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(f64::INFINITY);
            let fitness_j = archive[j].metadata.custom.get("spea2_fitness")
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(f64::INFINITY);
            
            selected.push(if fitness_i < fitness_j { &archive[i] } else { &archive[j] });
        }
        
        selected
    }
    
    /// Create offspring from selected parents
    fn create_offspring_from_parents(&self, parents: &[&Solution]) -> Vec<Solution> {
        let mut offspring = Vec::new();
        
        for i in (0..parents.len()).step_by(2) {
            if i + 1 < parents.len() {
                let (child1, child2) = self.crossover(parents[i], parents[i + 1]);
                offspring.push(child1);
                offspring.push(child2);
            }
        }
        
        for child in &mut offspring {
            self.mutate(child);
        }
        
        offspring
    }
}

/// Helper function to generate random value (placeholder)
fn rand_value() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 / u32::MAX as f64)
}

/// Represents a front in non-dominated sorting
#[derive(Debug, Clone)]
struct Front {
    indices: Vec<usize>,
    rank: u32,
}

// ============================================================================
// Preference Handling
// ============================================================================

/// Preference information for decision making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preference {
    /// Type of preference
    pub preference_type: PreferenceType,
    /// Objective weights
    pub weights: HashMap<String, f64>,
    /// Aspiration levels
    pub aspiration_levels: HashMap<String, f64>,
    /// Reservation levels
    pub reservation_levels: HashMap<String, f64>,
    /// Reference point for achievement functions
    pub reference_point: HashMap<String, f64>,
}

/// Types of preference information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreferenceType {
    /// Weights for objectives
    Weighted,
    /// Reference point
    ReferencePoint,
    /// Aspiration/Reservation levels
    AspirationReservation,
    /// Lexicographic ordering
    Lexicographic,
}

/// Decision support for selecting solutions from Pareto frontier
pub struct DecisionSupport {
    /// Preferences
    preferences: Preference,
}

impl DecisionSupport {
    /// Create new decision support
    pub fn new(preferences: Preference) -> Self {
        Self { preferences }
    }
    
    /// Rank solutions based on preferences
    pub fn rank_solutions(&self, solutions: &[Solution]) -> Vec<(Solution, f64)> {
        let mut ranked: Vec<(Solution, f64)> = solutions
            .iter()
            .map(|s| (s.clone(), self.calculate_achievement(s)))
            .collect();
        
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked
    }
    
    /// Calculate achievement function value
    fn calculate_achievement(&self, solution: &Solution) -> f64 {
        match self.preferences.preference_type {
            PreferenceType::Weighted => {
                let mut achievement = 0.0;
                for (obj_id, weight) in &self.preferences.weights {
                    let value = solution.objectives.get(obj_id).copied().unwrap_or(0.0);
                    achievement += weight * value;
                }
                achievement
            }
            PreferenceType::ReferencePoint => {
                let mut distance = 0.0;
                for (obj_id, ref_val) in &self.preferences.reference_point {
                    let value = solution.objectives.get(obj_id).copied().unwrap_or(0.0);
                    distance += (value - ref_val).powi(2);
                }
                -distance.sqrt() // Negative distance as achievement
            }
            _ => 0.0,
        }
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_objective_creation() {
        let objective = Objective {
            id: "obj1".to_string(),
            name: "Cost".to_string(),
            description: "Minimize cost".to_string(),
            direction: ObjectiveDirection::Minimize,
            weight: 0.5,
            priority: None,
            target: None,
            threshold: None,
            active: true,
        };
        
        assert_eq!(objective.id, "obj1");
        assert_eq!(objective.direction, ObjectiveDirection::Minimize);
    }
    
    #[test]
    fn test_dominance_calculation() {
        let objectives = HashMap::new();
        
        let sol1 = Solution {
            id: "s1".to_string(),
            variables: HashMap::new(),
            objectives: vec![("obj1".to_string(), 1.0)].into_iter().collect(),
            is_pareto_optimal: false,
            crowding_distance: 0.0,
            pareto_rank: None,
            metadata: SolutionMetadata {
                created_at: Utc::now(),
                algorithm: "Test".to_string(),
                generations: 0,
                computation_time_ms: 0,
                custom: HashMap::new(),
            },
        };
        
        let sol2 = Solution {
            id: "s2".to_string(),
            variables: HashMap::new(),
            objectives: vec![("obj1".to_string(), 2.0)].into_iter().collect(),
            is_pareto_optimal: false,
            crowding_distance: 0.0,
            pareto_rank: None,
            metadata: SolutionMetadata {
                created_at: Utc::now(),
                algorithm: "Test".to_string(),
                generations: 0,
                computation_time_ms: 0,
                custom: HashMap::new(),
            },
        };
        
        let mut obj_map = HashMap::new();
        obj_map.insert("obj1".to_string(), Objective {
            id: "obj1".to_string(),
            name: "Objective 1".to_string(),
            description: "Test objective".to_string(),
            direction: ObjectiveDirection::Minimize,
            weight: 1.0,
            priority: None,
            target: None,
            threshold: None,
            active: true,
        });
        
        let relation = calculate_dominance(&sol1, &sol2, &obj_map);
        assert_eq!(relation, DominanceRelation::Dominates);
    }
    
    #[test]
    fn test_optimizer_creation() {
        let config = MultiObjectiveConfig {
            max_solutions: 100,
            max_generations: 50,
            convergence_threshold: 0.001,
            population_size: 50,
            mutation_probability: 0.1,
            crossover_probability: 0.9,
            tournament_size: 2,
            adaptive_operators: true,
            reference_point: HashMap::new(),
            algorithm: MOOAlgorithm::NSGA2,
            constraint_handling: ConstraintHandling::Penalty,
        };
        
        let optimizer = MultiObjectiveOptimizer::new(config);
        assert_eq!(optimizer.generation, 0);
        assert!(optimizer.population.is_empty());
    }
    
    #[test]
    fn test_solution_metadata() {
        let metadata = SolutionMetadata {
            created_at: Utc::now(),
            algorithm: "NSGA2".to_string(),
            generations: 100,
            computation_time_ms: 5000,
            custom: HashMap::new(),
        };
        
        assert_eq!(metadata.algorithm, "NSGA2");
        assert_eq!(metadata.generations, 100);
    }
    
    #[test]
    fn test_pareto_frontier() {
        let frontier = ParetoFrontier {
            solutions: Vec::new(),
            objectives: vec!["obj1".to_string(), "obj2".to_string()],
            hypervolume: 0.85,
            spread: 0.5,
            convergence: 0.01,
            computed_at: Utc::now(),
        };
        
        assert_eq!(frontier.objectives.len(), 2);
        assert_eq!(frontier.hypervolume, 0.85);
    }
}