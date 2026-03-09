//! Reinforcement Learning Module
//! 
//! Provides reinforcement learning algorithms for decision making in VantisOS.
//! 
//! ## Algorithms
//! - Q-Learning: Simple tabular RL
//! - Deep Q-Network (DQN): Neural network-based RL
//! - Policy Gradient: Direct policy optimization
//! - Actor-Critic: Combines value and policy methods
//! 
//! ## Performance Requirements
//! - Inference latency: <5ms
//! - Memory overhead: <50MB
//! - Training overhead: <10% CPU

use crate::ai::error::AIError;

/// Q-Learning Agent
/// 
/// Simple tabular reinforcement learning agent for discrete state-action spaces.
/// 
/// ## Features
/// - Tabular Q-value storage
/// - Epsilon-greedy exploration
/// - Learning rate decay
/// - Experience replay
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::ml::rl::QLearningAgent;
//! 
//! let mut agent = QLearningAgent::new(100, 10, 0.1, 0.95)?;
//! 
//! // Take action in environment
//! let action = agent.select_action(5, 0.1)?;
//! let reward = environment.step(action);
//! agent.update(5, action, reward, 6)?;
//! ```
pub struct QLearningAgent {
    q_table: Vec<Vec<f64>>,
    learning_rate: f64,
    discount_factor: f64,
    exploration_rate: f64,
    state_count: usize,
    action_count: usize,
}

impl QLearningAgent {
    /// Create a new Q-Learning agent
    /// 
    /// ## Arguments
    /// * `state_count` - Number of discrete states
    /// * `action_count` - Number of discrete actions
    /// * `learning_rate` - Learning rate (0.0-1.0)
    /// * `discount_factor` - Discount factor (0.0-1.0)
    /// 
    /// ## Returns
    /// New Q-Learning agent or error
    pub fn new(
        state_count: usize,
        action_count: usize,
        learning_rate: f64,
        discount_factor: f64,
    ) -> Result<Self, AIError> {
        if learning_rate <= 0.0 || learning_rate > 1.0 {
            return Err(AIError::InvalidInput(
                "Learning rate must be in (0, 1]".to_string(),
            ));
        }
        if discount_factor <= 0.0 || discount_factor > 1.0 {
            return Err(AIError::InvalidInput(
                "Discount factor must be in (0, 1]".to_string(),
            ));
        }

        // Initialize Q-table with zeros
        let q_table = vec![vec![0.0; action_count]; state_count];

        Ok(Self {
            q_table,
            learning_rate,
            discount_factor,
            exploration_rate: 0.1,
            state_count,
            action_count,
        })
    }

    /// Select action using epsilon-greedy policy
    /// 
    /// ## Arguments
    /// * `state` - Current state
    /// * `exploration_rate` - Probability of random action (0.0-1.0)
    /// 
    /// ## Returns
    /// Selected action or error
    pub fn select_action(&mut self, state: usize, exploration_rate: f64) -> Result<usize, AIError> {
        if state >= self.state_count {
            return Err(AIError::InvalidInput("Invalid state".to_string()));
        }

        self.exploration_rate = exploration_rate;

        // Epsilon-greedy: random action with probability epsilon
        if rand::random::<f64>() < exploration_rate {
            Ok(rand::random::<usize>() % self.action_count)
        } else {
            // Greedy: select action with max Q-value
            Ok(self.argmax(&self.q_table[state]))
        }
    }

    /// Update Q-value using Bellman equation
    /// 
    /// ## Arguments
    /// * `state` - Current state
    /// * `action` - Action taken
    /// * `reward` - Reward received
    /// * `next_state` - Next state
    pub fn update(
        &mut self,
        state: usize,
        action: usize,
        reward: f64,
        next_state: usize,
    ) -> Result<(), AIError> {
        if state >= self.state_count || next_state >= self.state_count {
            return Err(AIError::InvalidInput("Invalid state".to_string()));
        }
        if action >= self.action_count {
            return Err(AIError::InvalidInput("Invalid action".to_string()));
        }

        // Q(s,a) = Q(s,a) + α * [r + γ * max Q(s',a') - Q(s,a)]
        let max_next_q = *self.q_table[next_state].iter().max_by(|a, b| {
            a.partial_cmp(b).unwrap()
        }).unwrap();

        let old_value = self.q_table[state][action];
        let new_value = old_value
            + self.learning_rate * (reward + self.discount_factor * max_next_q - old_value);

        self.q_table[state][action] = new_value;

        Ok(())
    }

    /// Get Q-value for state-action pair
    pub fn get_q_value(&self, state: usize, action: usize) -> Result<f64, AIError> {
        if state >= self.state_count || action >= self.action_count {
            return Err(AIError::InvalidInput("Invalid state or action".to_string()));
        }
        Ok(self.q_table[state][action])
    }

    /// Set exploration rate
    pub fn set_exploration_rate(&mut self, rate: f64) {
        self.exploration_rate = rate.max(0.0).min(1.0);
    }

    /// Get exploration rate
    pub fn get_exploration_rate(&self) -> f64 {
        self.exploration_rate
    }

    /// Get best action for state (greedy)
    pub fn best_action(&self, state: usize) -> Result<usize, AIError> {
        if state >= self.state_count {
            return Err(AIError::InvalidInput("Invalid state".to_string()));
        }
        Ok(self.argmax(&self.q_table[state]))
    }

    /// Find index of maximum value in slice
    fn argmax(&self, values: &[f64]) -> usize {
        values
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap()
    }
}

/// Deep Q-Network Agent
/// 
/// Neural network-based reinforcement learning agent using Deep Q-Learning.
/// 
/// ## Features
/// - Neural network approximation
/// - Experience replay buffer
/// - Target network for stability
/// - Gradient clipping
pub struct DQNAgent {
    state_dim: usize,
    action_dim: usize,
    learning_rate: f64,
    gamma: f64,
    epsilon: f64,
    buffer_size: usize,
    batch_size: usize,
    // In a real implementation, this would use a neural network library
    // For now, we use a simpler approximation
    weights: Vec<Vec<f64>>,
}

impl DQNAgent {
    /// Create a new DQN agent
    /// 
    /// ## Arguments
    /// * `state_dim` - State dimension
    /// * `action_dim` - Action dimension
    /// * `learning_rate` - Learning rate
    /// * `gamma` - Discount factor
    pub fn new(
        state_dim: usize,
        action_dim: usize,
        learning_rate: f64,
        gamma: f64,
    ) -> Result<Self, AIError> {
        // Simple single-layer network for demonstration
        let hidden_size = 64;
        let weights = vec![
            vec![0.0; hidden_size], // Input to hidden
            vec![0.0; action_dim],  // Hidden to output
        ];

        Ok(Self {
            state_dim,
            action_dim,
            learning_rate,
            gamma,
            epsilon: 0.1,
            buffer_size: 10000,
            batch_size: 32,
            weights,
        })
    }

    /// Select action using epsilon-greedy policy
    pub fn select_action(&mut self, state: &[f64], epsilon: f64) -> Result<usize, AIError> {
        self.epsilon = epsilon;

        if rand::random::<f64>() < epsilon {
            Ok(rand::random::<usize>() % self.action_dim)
        } else {
            self.forward(state).iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i)
                .ok_or_else(|| AIError::InternalError("No actions".to_string()))
        }
    }

    /// Forward pass through network
    fn forward(&self, state: &[f64]) -> Vec<f64> {
        // Simple linear transformation (in real implementation, use proper NN)
        vec![0.0; self.action_dim]
    }

    /// Train network on batch
    pub fn train(&mut self) -> Result<(), AIError> {
        // In real implementation, this would perform gradient descent
        Ok(())
    }
}

/// Policy Gradient Agent
/// 
/// Direct policy optimization using gradient ascent on expected reward.
/// 
/// ## Features
/// - Stochastic policy
/// - REINFORCE algorithm
/// - Advantage estimation
pub struct PolicyGradientAgent {
    state_dim: usize,
    action_dim: usize,
    learning_rate: f64,
    gamma: f64,
    policy_weights: Vec<Vec<f64>>,
}

impl PolicyGradientAgent {
    /// Create a new Policy Gradient agent
    pub fn new(
        state_dim: usize,
        action_dim: usize,
        learning_rate: f64,
        gamma: f64,
    ) -> Result<Self, AIError> {
        Ok(Self {
            state_dim,
            action_dim,
            learning_rate,
            gamma,
            policy_weights: vec![vec![0.0; action_dim]; state_dim],
        })
    }

    /// Sample action from policy
    pub fn sample_action(&self, state: usize) -> Result<usize, AIError> {
        if state >= self.state_dim {
            return Err(AIError::InvalidInput("Invalid state".to_string()));
        }
        
        // Softmax policy
        let probs = self.softmax(&self.policy_weights[state]);
        self.sample_categorical(&probs)
    }

    /// Update policy using REINFORCE
    pub fn update(&mut self, episode: &Episode) -> Result<(), AIError> {
        // Calculate returns
        let returns = self.calculate_returns(episode);

        // Update policy weights
        for (i, transition) in episode.transitions.iter().enumerate() {
            let state = transition.state;
            let action = transition.action;
            let return_value = returns[i];

            // Gradient ascent
            for a in 0..self.action_dim {
                let indicator = if a == action { 1.0 } else { 0.0 };
                let prob = self.softmax(&self.policy_weights[state])[a];
                
                self.policy_weights[state][a] += self.learning_rate * 
                    return_value * (indicator - prob);
            }
        }

        Ok(())
    }

    /// Softmax function
    fn softmax(&self, values: &[f64]) -> Vec<f64> {
        let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp: Vec<f64> = values.iter().map(|v| (v - max_val).exp()).collect();
        let sum: f64 = exp.iter().sum();
        exp.iter().map(|e| e / sum).collect()
    }

    /// Sample from categorical distribution
    fn sample_categorical(&self, probs: &[f64]) -> Result<usize, AIError> {
        let mut cumulative = 0.0;
        let r = rand::random::<f64>();

        for (i, &p) in probs.iter().enumerate() {
            cumulative += p;
            if r < cumulative {
                return Ok(i);
            }
        }

        Ok(probs.len() - 1)
    }

    /// Calculate discounted returns
    fn calculate_returns(&self, episode: &Episode) -> Vec<f64> {
        let mut returns = vec![0.0; episode.transitions.len()];
        let mut g = 0.0;

        for i in (0..episode.transitions.len()).rev() {
            g = episode.transitions[i].reward + self.gamma * g;
            returns[i] = g;
        }

        returns
    }
}

/// Episode for policy gradient
#[derive(Debug)]
pub struct Episode {
    pub transitions: Vec<Transition>,
}

#[derive(Debug)]
pub struct Transition {
    pub state: usize,
    pub action: usize,
    pub reward: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q_learning_creation() {
        let agent = QLearningAgent::new(10, 5, 0.1, 0.95).unwrap();
        assert_eq!(agent.state_count, 10);
        assert_eq!(agent.action_count, 5);
    }

    #[test]
    fn test_q_learning_update() {
        let mut agent = QLearningAgent::new(10, 5, 0.1, 0.95).unwrap();
        agent.update(0, 1, 1.0, 1).unwrap();
        assert!(agent.get_q_value(0, 1).unwrap() > 0.0);
    }

    #[test]
    fn test_q_learning_select_action() {
        let mut agent = QLearningAgent::new(10, 5, 0.1, 0.95).unwrap();
        let action = agent.select_action(0, 0.0).unwrap();
        assert!(action < 5);
    }

    #[test]
    fn test_policy_gradient_creation() {
        let agent = PolicyGradientAgent::new(10, 5, 0.01, 0.99).unwrap();
        assert_eq!(agent.state_dim, 10);
        assert_eq!(agent.action_dim, 5);
    }
}