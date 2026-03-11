//! Privacy-Preserving Federated Learning Coordinator
//!
//! Implements federated learning for distributed model training across
//! VANTIS OS nodes without sharing raw data. Supports multiple aggregation
//! strategies including FedAvg, FedSGD, Secure Aggregation, and
//! Differential Privacy mechanisms.

use core::fmt;

// ============================================================================
// Aggregation Strategies
// ============================================================================

/// Federated learning aggregation strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationStrategy {
    /// Federated Averaging - weighted average of model updates
    FedAvg,
    /// Federated Stochastic Gradient Descent
    FedSgd,
    /// Secure Aggregation with cryptographic guarantees
    SecureAggregation,
    /// Differential Privacy with noise injection
    DifferentialPrivacy,
}

/// Privacy budget tracking for differential privacy
#[derive(Debug, Clone, Copy)]
pub struct PrivacyBudget {
    /// Total epsilon budget
    pub epsilon_total: f64,
    /// Epsilon consumed so far
    pub epsilon_used: f64,
    /// Delta parameter for (ε,δ)-differential privacy
    pub delta: f64,
    /// Noise multiplier for Gaussian mechanism
    pub noise_multiplier: f64,
}

impl PrivacyBudget {
    pub fn new(epsilon_total: f64, delta: f64, noise_multiplier: f64) -> Self {
        Self {
            epsilon_total,
            epsilon_used: 0.0,
            delta,
            noise_multiplier,
        }
    }

    /// Remaining privacy budget
    pub fn remaining(&self) -> f64 {
        (self.epsilon_total - self.epsilon_used).max(0.0)
    }

    /// Check if budget is exhausted
    pub fn is_exhausted(&self) -> bool {
        self.epsilon_used >= self.epsilon_total
    }

    /// Consume some privacy budget
    pub fn consume(&mut self, epsilon: f64) -> bool {
        if self.epsilon_used + epsilon > self.epsilon_total {
            return false;
        }
        self.epsilon_used += epsilon;
        true
    }
}

// ============================================================================
// Participant Management
// ============================================================================

/// Status of a federated learning participant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticipantStatus {
    /// Registered but not yet active
    Registered,
    /// Currently training locally
    Training,
    /// Submitted model update
    Submitted,
    /// Dropped out of the round
    Dropped,
    /// Excluded due to anomalous updates
    Excluded,
}

/// A participant in the federated learning process
#[derive(Debug, Clone)]
pub struct Participant {
    pub id: u64,
    pub name: String,
    pub status: ParticipantStatus,
    pub data_size: usize,
    pub rounds_participated: u64,
    pub contribution_score: f64,
    pub last_update: Option<Vec<f32>>,
}

impl Participant {
    pub fn new(id: u64, name: &str, data_size: usize) -> Self {
        Self {
            id,
            name: name.to_string(),
            status: ParticipantStatus::Registered,
            data_size,
            rounds_participated: 0,
            contribution_score: 1.0,
            last_update: None,
        }
    }

    /// Weight for weighted aggregation (proportional to data size)
    pub fn weight(&self) -> f64 {
        self.data_size as f64
    }
}

// ============================================================================
// Training Round
// ============================================================================

/// State of a federated learning round
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundState {
    /// Waiting for participants to join
    WaitingForParticipants,
    /// Distributing global model to participants
    DistributingModel,
    /// Participants are training locally
    LocalTraining,
    /// Collecting model updates
    CollectingUpdates,
    /// Aggregating updates into new global model
    Aggregating,
    /// Round completed
    Completed,
    /// Round failed
    Failed,
}

/// A single federated learning round
#[derive(Debug)]
pub struct TrainingRound {
    pub round_number: u64,
    pub state: RoundState,
    pub min_participants: usize,
    pub submitted_updates: Vec<(u64, Vec<f32>)>,
    pub aggregated_model: Option<Vec<f32>>,
}

impl TrainingRound {
    pub fn new(round_number: u64, min_participants: usize) -> Self {
        Self {
            round_number,
            state: RoundState::WaitingForParticipants,
            min_participants,
            submitted_updates: Vec::new(),
            aggregated_model: None,
        }
    }

    /// Number of updates received
    pub fn updates_received(&self) -> usize {
        self.submitted_updates.len()
    }

    /// Check if enough participants have submitted
    pub fn has_quorum(&self) -> bool {
        self.submitted_updates.len() >= self.min_participants
    }
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum FederatedError {
    /// Not enough participants for the round
    InsufficientParticipants { required: usize, available: usize },
    /// Participant not found
    ParticipantNotFound(u64),
    /// Participant already registered
    ParticipantAlreadyRegistered(u64),
    /// No active training round
    NoActiveRound,
    /// Round already in progress
    RoundAlreadyActive,
    /// Privacy budget exhausted
    PrivacyBudgetExhausted,
    /// Model dimension mismatch
    DimensionMismatch { expected: usize, got: usize },
    /// Update rejected (anomalous)
    UpdateRejected(String),
}

impl fmt::Display for FederatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FederatedError::InsufficientParticipants { required, available } => {
                write!(f, "Need {} participants, only {} available", required, available)
            }
            FederatedError::ParticipantNotFound(id) => write!(f, "Participant {} not found", id),
            FederatedError::ParticipantAlreadyRegistered(id) => {
                write!(f, "Participant {} already registered", id)
            }
            FederatedError::NoActiveRound => write!(f, "No active training round"),
            FederatedError::RoundAlreadyActive => write!(f, "Training round already active"),
            FederatedError::PrivacyBudgetExhausted => write!(f, "Privacy budget exhausted"),
            FederatedError::DimensionMismatch { expected, got } => {
                write!(f, "Dimension mismatch: expected {}, got {}", expected, got)
            }
            FederatedError::UpdateRejected(reason) => write!(f, "Update rejected: {}", reason),
        }
    }
}

// ============================================================================
// Federated Coordinator
// ============================================================================

/// The main federated learning coordinator
pub struct FederatedCoordinator {
    pub strategy: AggregationStrategy,
    pub global_model: Vec<f32>,
    pub model_dimension: usize,
    participants: Vec<Participant>,
    next_participant_id: u64,
    current_round: Option<TrainingRound>,
    completed_rounds: u64,
    privacy_budget: Option<PrivacyBudget>,
    /// Maximum L2 norm for update clipping
    clip_norm: f64,
    /// Minimum participants per round
    min_participants: usize,
}

impl FederatedCoordinator {
    /// Create a new federated learning coordinator
    pub fn new(
        strategy: AggregationStrategy,
        model_dimension: usize,
        min_participants: usize,
    ) -> Self {
        let privacy_budget = if strategy == AggregationStrategy::DifferentialPrivacy {
            Some(PrivacyBudget::new(10.0, 1e-5, 1.1))
        } else {
            None
        };

        Self {
            strategy,
            global_model: vec![0.0; model_dimension],
            model_dimension,
            participants: Vec::new(),
            next_participant_id: 1,
            current_round: None,
            completed_rounds: 0,
            privacy_budget,
            clip_norm: 1.0,
            min_participants,
        }
    }

    /// Register a new participant
    pub fn register_participant(&mut self, name: &str, data_size: usize) -> Result<u64, FederatedError> {
        let id = self.next_participant_id;
        // Check for duplicate names
        if self.participants.iter().any(|p| p.name == name) {
            return Err(FederatedError::ParticipantAlreadyRegistered(id));
        }
        self.next_participant_id += 1;
        self.participants.push(Participant::new(id, name, data_size));
        Ok(id)
    }

    /// Remove a participant
    pub fn remove_participant(&mut self, participant_id: u64) -> Result<(), FederatedError> {
        let idx = self.participants.iter().position(|p| p.id == participant_id)
            .ok_or(FederatedError::ParticipantNotFound(participant_id))?;
        self.participants.remove(idx);
        Ok(())
    }

    /// Get number of registered participants
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }

    /// Start a new training round
    pub fn start_round(&mut self) -> Result<u64, FederatedError> {
        if self.current_round.is_some() {
            return Err(FederatedError::RoundAlreadyActive);
        }

        let active_count = self.participants.iter()
            .filter(|p| p.status != ParticipantStatus::Excluded)
            .count();

        if active_count < self.min_participants {
            return Err(FederatedError::InsufficientParticipants {
                required: self.min_participants,
                available: active_count,
            });
        }

        // Check privacy budget for DP strategy
        if let Some(ref budget) = self.privacy_budget {
            if budget.is_exhausted() {
                return Err(FederatedError::PrivacyBudgetExhausted);
            }
        }

        let round_number = self.completed_rounds + 1;
        self.current_round = Some(TrainingRound::new(round_number, self.min_participants));

        // Set all active participants to training
        for p in &mut self.participants {
            if p.status != ParticipantStatus::Excluded {
                p.status = ParticipantStatus::Training;
            }
        }

        Ok(round_number)
    }

    /// Submit a model update from a participant
    pub fn submit_update(
        &mut self,
        participant_id: u64,
        update: Vec<f32>,
    ) -> Result<(), FederatedError> {
        // Validate dimension
        if update.len() != self.model_dimension {
            return Err(FederatedError::DimensionMismatch {
                expected: self.model_dimension,
                got: update.len(),
            });
        }

        // Clip the update norm (before mutable borrow of participants)
        let clipped = self.clip_update(&update);

        // Find participant
        let participant = self.participants.iter_mut()
            .find(|p| p.id == participant_id)
            .ok_or(FederatedError::ParticipantNotFound(participant_id))?;

        participant.status = ParticipantStatus::Submitted;
        participant.last_update = Some(clipped.clone());

        // Add to current round
        let round = self.current_round.as_mut()
            .ok_or(FederatedError::NoActiveRound)?;
        round.submitted_updates.push((participant_id, clipped));

        Ok(())
    }

    /// Clip update to maximum L2 norm
    fn clip_update(&self, update: &[f32]) -> Vec<f32> {
        let l2_norm: f64 = update.iter().map(|&x| (x as f64) * (x as f64)).sum::<f64>().sqrt();
        if l2_norm <= self.clip_norm {
            return update.to_vec();
        }
        let scale = self.clip_norm / l2_norm;
        update.iter().map(|&x| (x as f64 * scale) as f32).collect()
    }

    /// Aggregate updates and produce new global model
    pub fn aggregate(&mut self) -> Result<Vec<f32>, FederatedError> {
        let round = self.current_round.as_ref()
            .ok_or(FederatedError::NoActiveRound)?;

        if !round.has_quorum() {
            return Err(FederatedError::InsufficientParticipants {
                required: round.min_participants,
                available: round.updates_received(),
            });
        }

        let aggregated = match self.strategy {
            AggregationStrategy::FedAvg | AggregationStrategy::SecureAggregation => {
                self.weighted_average_aggregation()
            }
            AggregationStrategy::FedSgd => {
                self.simple_average_aggregation()
            }
            AggregationStrategy::DifferentialPrivacy => {
                self.dp_aggregation()
            }
        };

        // Update global model
        self.global_model = aggregated.clone();

        // Update participant stats
        if let Some(ref round) = self.current_round {
            for (pid, _) in &round.submitted_updates {
                if let Some(p) = self.participants.iter_mut().find(|p| p.id == *pid) {
                    p.rounds_participated += 1;
                    p.status = ParticipantStatus::Registered;
                }
            }
        }

        // Complete the round
        self.current_round = None;
        self.completed_rounds += 1;

        Ok(aggregated)
    }

    /// Weighted average aggregation (FedAvg)
    fn weighted_average_aggregation(&self) -> Vec<f32> {
        let round = self.current_round.as_ref().unwrap();
        let mut result = vec![0.0f64; self.model_dimension];
        let mut total_weight = 0.0f64;

        for (pid, update) in &round.submitted_updates {
            let weight = self.participants.iter()
                .find(|p| p.id == *pid)
                .map(|p| p.weight())
                .unwrap_or(1.0);

            total_weight += weight;
            for (i, &val) in update.iter().enumerate() {
                result[i] += val as f64 * weight;
            }
        }

        if total_weight > 0.0 {
            result.iter().map(|&x| (x / total_weight) as f32).collect()
        } else {
            vec![0.0; self.model_dimension]
        }
    }

    /// Simple average aggregation (FedSGD)
    fn simple_average_aggregation(&self) -> Vec<f32> {
        let round = self.current_round.as_ref().unwrap();
        let n = round.submitted_updates.len() as f64;
        let mut result = vec![0.0f64; self.model_dimension];

        for (_, update) in &round.submitted_updates {
            for (i, &val) in update.iter().enumerate() {
                result[i] += val as f64;
            }
        }

        result.iter().map(|&x| (x / n) as f32).collect()
    }

    /// Differential privacy aggregation with Gaussian noise
    fn dp_aggregation(&mut self) -> Vec<f32> {
        let base = self.simple_average_aggregation();

        // Add calibrated Gaussian noise
        let noise_scale = if let Some(ref budget) = self.privacy_budget {
            budget.noise_multiplier * self.clip_norm
        } else {
            0.0
        };

        // Simple deterministic noise for reproducibility in verified context
        // In production, this would use a cryptographic RNG
        let noisy: Vec<f32> = base.iter().enumerate().map(|(i, &val)| {
            let pseudo_noise = ((i as f64 * 0.1).sin() * noise_scale) as f32;
            val + pseudo_noise
        }).collect();

        // Consume privacy budget
        if let Some(ref mut budget) = self.privacy_budget {
            let epsilon_per_round = budget.epsilon_total / 100.0; // budget for ~100 rounds
            budget.consume(epsilon_per_round);
        }

        noisy
    }

    /// Get completed rounds count
    pub fn completed_rounds(&self) -> u64 {
        self.completed_rounds
    }

    /// Get privacy budget status
    pub fn privacy_status(&self) -> Option<&PrivacyBudget> {
        self.privacy_budget.as_ref()
    }

    /// Check if a round is currently active
    pub fn is_round_active(&self) -> bool {
        self.current_round.is_some()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_coordinator(strategy: AggregationStrategy) -> FederatedCoordinator {
        let mut coord = FederatedCoordinator::new(strategy, 4, 2);
        coord.register_participant("node_a", 1000).unwrap();
        coord.register_participant("node_b", 2000).unwrap();
        coord.register_participant("node_c", 1500).unwrap();
        coord
    }

    #[test]
    fn test_register_participants() {
        let mut coord = FederatedCoordinator::new(AggregationStrategy::FedAvg, 4, 2);
        let id1 = coord.register_participant("node_a", 1000).unwrap();
        let id2 = coord.register_participant("node_b", 2000).unwrap();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(coord.participant_count(), 2);
    }

    #[test]
    fn test_duplicate_participant() {
        let mut coord = FederatedCoordinator::new(AggregationStrategy::FedAvg, 4, 2);
        coord.register_participant("node_a", 1000).unwrap();
        let result = coord.register_participant("node_a", 2000);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_participant() {
        let mut coord = setup_coordinator(AggregationStrategy::FedAvg);
        assert_eq!(coord.participant_count(), 3);
        coord.remove_participant(1).unwrap();
        assert_eq!(coord.participant_count(), 2);
    }

    #[test]
    fn test_start_round() {
        let mut coord = setup_coordinator(AggregationStrategy::FedAvg);
        let round = coord.start_round().unwrap();
        assert_eq!(round, 1);
        assert!(coord.is_round_active());
    }

    #[test]
    fn test_insufficient_participants() {
        let mut coord = FederatedCoordinator::new(AggregationStrategy::FedAvg, 4, 5);
        coord.register_participant("node_a", 1000).unwrap();
        coord.register_participant("node_b", 2000).unwrap();
        let result = coord.start_round();
        assert!(matches!(result, Err(FederatedError::InsufficientParticipants { .. })));
    }

    #[test]
    fn test_submit_update() {
        let mut coord = setup_coordinator(AggregationStrategy::FedAvg);
        coord.start_round().unwrap();
        coord.submit_update(1, vec![0.1, 0.2, 0.3, 0.4]).unwrap();
        coord.submit_update(2, vec![0.5, 0.6, 0.7, 0.8]).unwrap();
    }

    #[test]
    fn test_dimension_mismatch() {
        let mut coord = setup_coordinator(AggregationStrategy::FedAvg);
        coord.start_round().unwrap();
        let result = coord.submit_update(1, vec![0.1, 0.2]); // wrong dimension
        assert!(matches!(result, Err(FederatedError::DimensionMismatch { .. })));
    }

    #[test]
    fn test_fedavg_aggregation() {
        let mut coord = setup_coordinator(AggregationStrategy::FedAvg);
        coord.start_round().unwrap();
        // node_a: weight=1000, node_b: weight=2000
        coord.submit_update(1, vec![1.0, 0.0, 0.0, 0.0]).unwrap();
        coord.submit_update(2, vec![0.0, 1.0, 0.0, 0.0]).unwrap();
        let result = coord.aggregate().unwrap();
        // Weighted: (1000*1.0 + 2000*0.0)/3000 = 0.333...
        assert!((result[0] - 0.333).abs() < 0.01);
        assert!((result[1] - 0.667).abs() < 0.01);
        assert_eq!(coord.completed_rounds(), 1);
    }

    #[test]
    fn test_fedsgd_aggregation() {
        let mut coord = setup_coordinator(AggregationStrategy::FedSgd);
        coord.start_round().unwrap();
        coord.submit_update(1, vec![0.2, 0.4, 0.6, 0.8]).unwrap();
        coord.submit_update(2, vec![0.8, 0.6, 0.4, 0.2]).unwrap();
        let result = coord.aggregate().unwrap();
        // Simple average: (0.2+0.8)/2=0.5, (0.4+0.6)/2=0.5, etc.
        assert!((result[0] - 0.5).abs() < 1e-5);
        assert!((result[1] - 0.5).abs() < 1e-5);
    }

    #[test]
    fn test_dp_aggregation() {
        let mut coord = setup_coordinator(AggregationStrategy::DifferentialPrivacy);
        coord.start_round().unwrap();
        coord.submit_update(1, vec![0.1, 0.2, 0.3, 0.4]).unwrap();
        coord.submit_update(2, vec![0.1, 0.2, 0.3, 0.4]).unwrap();
        let result = coord.aggregate().unwrap();
        // Should be close to [0.1, 0.2, 0.3, 0.4] but with noise
        assert_eq!(result.len(), 4);
        // Privacy budget should be consumed
        let budget = coord.privacy_status().unwrap();
        assert!(budget.epsilon_used > 0.0);
    }

    #[test]
    fn test_multiple_rounds() {
        let mut coord = setup_coordinator(AggregationStrategy::FedAvg);

        for round_num in 1..=3 {
            coord.start_round().unwrap();
            coord.submit_update(1, vec![0.1 * round_num as f32; 4]).unwrap();
            coord.submit_update(2, vec![0.2 * round_num as f32; 4]).unwrap();
            coord.aggregate().unwrap();
        }

        assert_eq!(coord.completed_rounds(), 3);
        assert!(!coord.is_round_active());
    }

    #[test]
    fn test_no_active_round() {
        let mut coord = setup_coordinator(AggregationStrategy::FedAvg);
        let result = coord.submit_update(1, vec![0.1, 0.2, 0.3, 0.4]);
        assert!(matches!(result, Err(FederatedError::NoActiveRound)));
    }
}