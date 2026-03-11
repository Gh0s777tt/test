//! AI-Powered Resource Prediction for VANTIS OS
//!
//! Uses Holt's double exponential smoothing for time series prediction
//! of system resource usage. Provides forecasting with confidence intervals
//! and auto-tuning recommendations for CPU, memory, disk I/O, and network.

use core::fmt;

// ============================================================================
// Resource Types
// ============================================================================

/// Types of system resources that can be predicted
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Cpu,
    Memory,
    DiskIo,
    NetworkBandwidth,
    GpuUtilization,
    PowerConsumption,
    ThreadCount,
    FileDescriptors,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::Cpu => write!(f, "CPU"),
            ResourceType::Memory => write!(f, "Memory"),
            ResourceType::DiskIo => write!(f, "DiskIO"),
            ResourceType::NetworkBandwidth => write!(f, "Network"),
            ResourceType::GpuUtilization => write!(f, "GPU"),
            ResourceType::PowerConsumption => write!(f, "Power"),
            ResourceType::ThreadCount => write!(f, "Threads"),
            ResourceType::FileDescriptors => write!(f, "FDs"),
        }
    }
}

/// Prediction time horizons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PredictionHorizon {
    /// Next 1 minute
    OneMinute,
    /// Next 5 minutes
    FiveMinutes,
    /// Next 15 minutes
    FifteenMinutes,
    /// Next 1 hour
    OneHour,
    /// Custom number of steps ahead
    Custom(u32),
}

impl PredictionHorizon {
    /// Convert to number of prediction steps
    pub fn steps(&self) -> u32 {
        match self {
            PredictionHorizon::OneMinute => 6,       // assuming 10s intervals
            PredictionHorizon::FiveMinutes => 30,
            PredictionHorizon::FifteenMinutes => 90,
            PredictionHorizon::OneHour => 360,
            PredictionHorizon::Custom(n) => *n,
        }
    }
}

// ============================================================================
// Holt's Double Exponential Smoothing
// ============================================================================

/// Holt's method state for trend-aware smoothing
#[derive(Debug, Clone)]
pub struct HoltState {
    /// Level smoothing factor (0 < α ≤ 1)
    pub alpha: f64,
    /// Trend smoothing factor (0 < β ≤ 1)
    pub beta: f64,
    /// Current level estimate
    pub level: f64,
    /// Current trend estimate
    pub trend: f64,
    /// Whether the model has been initialized
    pub initialized: bool,
    /// Number of observations processed
    pub observations: u64,
}

impl HoltState {
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self {
            alpha: alpha.clamp(0.01, 1.0),
            beta: beta.clamp(0.01, 1.0),
            level: 0.0,
            trend: 0.0,
            initialized: false,
            observations: 0,
        }
    }

    /// Update with a new observation
    pub fn update(&mut self, value: f64) {
        self.observations += 1;

        if !self.initialized {
            self.level = value;
            self.trend = 0.0;
            self.initialized = true;
            return;
        }

        let prev_level = self.level;
        // Level update: L_t = α * Y_t + (1 - α) * (L_{t-1} + T_{t-1})
        self.level = self.alpha * value + (1.0 - self.alpha) * (prev_level + self.trend);
        // Trend update: T_t = β * (L_t - L_{t-1}) + (1 - β) * T_{t-1}
        self.trend = self.beta * (self.level - prev_level) + (1.0 - self.beta) * self.trend;
    }

    /// Forecast k steps ahead
    pub fn forecast(&self, steps: u32) -> f64 {
        if !self.initialized {
            return 0.0;
        }
        self.level + self.trend * steps as f64
    }

    /// Forecast with confidence interval
    pub fn forecast_with_ci(&self, steps: u32, residual_std: f64, confidence: f64) -> (f64, f64, f64) {
        let point = self.forecast(steps);
        // z-score for confidence level (approximate)
        let z = match confidence {
            c if c >= 0.99 => 2.576,
            c if c >= 0.95 => 1.96,
            c if c >= 0.90 => 1.645,
            c if c >= 0.80 => 1.282,
            _ => 1.0,
        };
        let margin = z * residual_std * (steps as f64).sqrt();
        (point, point - margin, point + margin)
    }
}

// ============================================================================
// Prediction Result
// ============================================================================

/// A resource prediction with confidence bounds
#[derive(Debug, Clone)]
pub struct Prediction {
    pub resource: ResourceType,
    pub horizon: PredictionHorizon,
    pub point_estimate: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence: f64,
    pub trend_direction: TrendDirection,
}

/// Direction of the predicted trend
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendDirection {
    Rising,
    Falling,
    Stable,
}

impl fmt::Display for TrendDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrendDirection::Rising => write!(f, "↑ Rising"),
            TrendDirection::Falling => write!(f, "↓ Falling"),
            TrendDirection::Stable => write!(f, "→ Stable"),
        }
    }
}

// ============================================================================
// Auto-Tuning Recommendation
// ============================================================================

/// Recommended action based on prediction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuningAction {
    /// No action needed
    NoAction,
    /// Scale up resources
    ScaleUp { resource: ResourceType, amount_pct: u32 },
    /// Scale down resources
    ScaleDown { resource: ResourceType, amount_pct: u32 },
    /// Preemptive alert
    Alert { resource: ResourceType, message: String },
    /// Trigger garbage collection or cleanup
    Cleanup { resource: ResourceType },
}

impl fmt::Display for TuningAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TuningAction::NoAction => write!(f, "No action needed"),
            TuningAction::ScaleUp { resource, amount_pct } => {
                write!(f, "Scale up {} by {}%", resource, amount_pct)
            }
            TuningAction::ScaleDown { resource, amount_pct } => {
                write!(f, "Scale down {} by {}%", resource, amount_pct)
            }
            TuningAction::Alert { resource, message } => {
                write!(f, "Alert [{}]: {}", resource, message)
            }
            TuningAction::Cleanup { resource } => {
                write!(f, "Cleanup recommended for {}", resource)
            }
        }
    }
}

// ============================================================================
// Resource Predictor
// ============================================================================

/// Error types for the predictor
#[derive(Debug, Clone, PartialEq)]
pub enum PredictorError {
    InsufficientData { required: usize, available: usize },
    ResourceNotTracked(ResourceType),
    InvalidParameter(String),
}

impl fmt::Display for PredictorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PredictorError::InsufficientData { required, available } => {
                write!(f, "Need {} observations, have {}", required, available)
            }
            PredictorError::ResourceNotTracked(r) => write!(f, "Resource not tracked: {}", r),
            PredictorError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

/// Tracked resource state
struct ResourceTracker {
    resource: ResourceType,
    holt: HoltState,
    history: Vec<f64>,
    max_history: usize,
    residual_std: f64,
    capacity_threshold: f64,
}

impl ResourceTracker {
    fn new(resource: ResourceType, alpha: f64, beta: f64, capacity_threshold: f64) -> Self {
        Self {
            resource,
            holt: HoltState::new(alpha, beta),
            history: Vec::new(),
            max_history: 500,
            residual_std: 0.0,
            capacity_threshold,
        }
    }

    fn update(&mut self, value: f64) {
        // Compute residual before updating
        if self.holt.initialized {
            let predicted = self.holt.forecast(1);
            let residual = (value - predicted).abs();
            // Exponential moving average of residuals
            self.residual_std = 0.9 * self.residual_std + 0.1 * residual;
        }

        self.holt.update(value);

        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push(value);
    }

    fn observations(&self) -> u64 {
        self.holt.observations
    }
}

/// The main resource predictor managing multiple resource trackers
pub struct ResourcePredictor {
    trackers: Vec<ResourceTracker>,
    min_observations: usize,
    default_confidence: f64,
}

impl ResourcePredictor {
    /// Create a new resource predictor
    pub fn new(min_observations: usize) -> Self {
        Self {
            trackers: Vec::new(),
            min_observations,
            default_confidence: 0.95,
        }
    }

    /// Create with default settings and common resource trackers
    pub fn with_defaults() -> Self {
        let mut predictor = Self::new(30);
        predictor.track_resource(ResourceType::Cpu, 0.3, 0.1, 90.0);
        predictor.track_resource(ResourceType::Memory, 0.2, 0.05, 85.0);
        predictor.track_resource(ResourceType::DiskIo, 0.3, 0.1, 80.0);
        predictor.track_resource(ResourceType::NetworkBandwidth, 0.4, 0.15, 90.0);
        predictor
    }

    /// Start tracking a resource
    pub fn track_resource(
        &mut self,
        resource: ResourceType,
        alpha: f64,
        beta: f64,
        capacity_threshold: f64,
    ) {
        // Don't add duplicates
        if self.trackers.iter().any(|t| t.resource == resource) {
            return;
        }
        self.trackers.push(ResourceTracker::new(resource, alpha, beta, capacity_threshold));
    }

    /// Stop tracking a resource
    pub fn untrack_resource(&mut self, resource: ResourceType) {
        self.trackers.retain(|t| t.resource != resource);
    }

    /// Feed a new observation for a resource
    pub fn observe(&mut self, resource: ResourceType, value: f64) -> Result<(), PredictorError> {
        let tracker = self.trackers.iter_mut()
            .find(|t| t.resource == resource)
            .ok_or(PredictorError::ResourceNotTracked(resource))?;
        tracker.update(value);
        Ok(())
    }

    /// Predict future resource usage
    pub fn predict(
        &self,
        resource: ResourceType,
        horizon: PredictionHorizon,
    ) -> Result<Prediction, PredictorError> {
        let tracker = self.trackers.iter()
            .find(|t| t.resource == resource)
            .ok_or(PredictorError::ResourceNotTracked(resource))?;

        if tracker.observations() < self.min_observations as u64 {
            return Err(PredictorError::InsufficientData {
                required: self.min_observations,
                available: tracker.observations() as usize,
            });
        }

        let steps = horizon.steps();
        let (point, lower, upper) = tracker.holt.forecast_with_ci(
            steps,
            tracker.residual_std,
            self.default_confidence,
        );

        let trend_direction = if tracker.holt.trend > 0.1 {
            TrendDirection::Rising
        } else if tracker.holt.trend < -0.1 {
            TrendDirection::Falling
        } else {
            TrendDirection::Stable
        };

        Ok(Prediction {
            resource,
            horizon,
            point_estimate: point,
            lower_bound: lower,
            upper_bound: upper,
            confidence: self.default_confidence,
            trend_direction,
        })
    }

    /// Generate auto-tuning recommendations based on predictions
    pub fn recommend(&self, resource: ResourceType) -> Result<TuningAction, PredictorError> {
        let tracker = self.trackers.iter()
            .find(|t| t.resource == resource)
            .ok_or(PredictorError::ResourceNotTracked(resource))?;

        if tracker.observations() < self.min_observations as u64 {
            return Ok(TuningAction::NoAction);
        }

        let prediction = self.predict(resource, PredictionHorizon::FiveMinutes)?;
        let threshold = tracker.capacity_threshold;

        // Check if predicted usage will exceed threshold
        if prediction.point_estimate > threshold {
            if prediction.point_estimate > threshold * 1.2 {
                return Ok(TuningAction::Alert {
                    resource,
                    message: format!(
                        "Predicted {:.1}% usage exceeds critical threshold {:.1}%",
                        prediction.point_estimate, threshold * 1.2
                    ),
                });
            }
            let scale_amount = ((prediction.point_estimate - threshold) / threshold * 100.0) as u32;
            return Ok(TuningAction::ScaleUp {
                resource,
                amount_pct: scale_amount.max(10),
            });
        }

        // Check if resources are significantly underutilized
        if prediction.point_estimate < threshold * 0.3 && prediction.trend_direction == TrendDirection::Falling {
            let scale_amount = ((threshold * 0.3 - prediction.point_estimate) / threshold * 100.0) as u32;
            return Ok(TuningAction::ScaleDown {
                resource,
                amount_pct: scale_amount.max(5),
            });
        }

        // Check if cleanup might help
        if prediction.trend_direction == TrendDirection::Rising
            && prediction.point_estimate > threshold * 0.7
        {
            return Ok(TuningAction::Cleanup { resource });
        }

        Ok(TuningAction::NoAction)
    }

    /// Get the number of tracked resources
    pub fn tracked_count(&self) -> usize {
        self.trackers.len()
    }

    /// Get current level for a resource
    pub fn current_level(&self, resource: ResourceType) -> Option<f64> {
        self.trackers.iter()
            .find(|t| t.resource == resource)
            .map(|t| t.holt.level)
    }

    /// Get current trend for a resource
    pub fn current_trend(&self, resource: ResourceType) -> Option<f64> {
        self.trackers.iter()
            .find(|t| t.resource == resource)
            .map(|t| t.holt.trend)
    }

    /// Get observation count for a resource
    pub fn observation_count(&self, resource: ResourceType) -> Option<u64> {
        self.trackers.iter()
            .find(|t| t.resource == resource)
            .map(|t| t.observations())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holt_initialization() {
        let mut holt = HoltState::new(0.3, 0.1);
        assert!(!holt.initialized);
        holt.update(10.0);
        assert!(holt.initialized);
        assert!((holt.level - 10.0).abs() < 1e-10);
        assert!((holt.trend - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_holt_trend_detection() {
        let mut holt = HoltState::new(0.5, 0.3);
        // Feed increasing values
        for i in 0..20 {
            holt.update(i as f64 * 2.0);
        }
        assert!(holt.trend > 0.0); // Should detect upward trend
        let forecast = holt.forecast(5);
        assert!(forecast > holt.level); // Forecast should be above current level
    }

    #[test]
    fn test_holt_forecast_with_ci() {
        let mut holt = HoltState::new(0.3, 0.1);
        for i in 0..30 {
            holt.update(50.0 + i as f64);
        }
        let (point, lower, upper) = holt.forecast_with_ci(10, 2.0, 0.95);
        assert!(lower < point);
        assert!(upper > point);
        assert!(upper - lower > 0.0); // CI should have width
    }

    #[test]
    fn test_predictor_track_and_observe() {
        let mut predictor = ResourcePredictor::new(5);
        predictor.track_resource(ResourceType::Cpu, 0.3, 0.1, 90.0);
        
        for i in 0..10 {
            predictor.observe(ResourceType::Cpu, 50.0 + i as f64).unwrap();
        }

        assert_eq!(predictor.observation_count(ResourceType::Cpu), Some(10));
    }

    #[test]
    fn test_predictor_untracked_resource() {
        let predictor = ResourcePredictor::new(5);
        let result = predictor.predict(ResourceType::Cpu, PredictionHorizon::OneMinute);
        assert!(matches!(result, Err(PredictorError::ResourceNotTracked(_))));
    }

    #[test]
    fn test_predictor_insufficient_data() {
        let mut predictor = ResourcePredictor::new(30);
        predictor.track_resource(ResourceType::Cpu, 0.3, 0.1, 90.0);
        predictor.observe(ResourceType::Cpu, 50.0).unwrap();

        let result = predictor.predict(ResourceType::Cpu, PredictionHorizon::OneMinute);
        assert!(matches!(result, Err(PredictorError::InsufficientData { .. })));
    }

    #[test]
    fn test_predictor_rising_trend() {
        let mut predictor = ResourcePredictor::new(10);
        predictor.track_resource(ResourceType::Memory, 0.5, 0.3, 85.0);

        for i in 0..30 {
            predictor.observe(ResourceType::Memory, 40.0 + i as f64 * 1.5).unwrap();
        }

        let prediction = predictor.predict(ResourceType::Memory, PredictionHorizon::FiveMinutes).unwrap();
        assert_eq!(prediction.trend_direction, TrendDirection::Rising);
        assert!(prediction.point_estimate > 40.0);
    }

    #[test]
    fn test_predictor_stable_trend() {
        let mut predictor = ResourcePredictor::new(10);
        predictor.track_resource(ResourceType::Cpu, 0.3, 0.1, 90.0);

        for _ in 0..30 {
            predictor.observe(ResourceType::Cpu, 50.0).unwrap();
        }

        let prediction = predictor.predict(ResourceType::Cpu, PredictionHorizon::OneMinute).unwrap();
        assert_eq!(prediction.trend_direction, TrendDirection::Stable);
        assert!((prediction.point_estimate - 50.0).abs() < 5.0);
    }

    #[test]
    fn test_recommend_scale_up() {
        let mut predictor = ResourcePredictor::new(10);
        predictor.track_resource(ResourceType::Cpu, 0.5, 0.3, 80.0);

        // Feed rapidly increasing data that will exceed threshold
        for i in 0..30 {
            predictor.observe(ResourceType::Cpu, 60.0 + i as f64 * 2.0).unwrap();
        }

        let action = predictor.recommend(ResourceType::Cpu).unwrap();
        assert!(matches!(action, TuningAction::ScaleUp { .. } | TuningAction::Alert { .. }));
    }

    #[test]
    fn test_recommend_no_action() {
        let mut predictor = ResourcePredictor::new(10);
        predictor.track_resource(ResourceType::Cpu, 0.3, 0.1, 90.0);

        for _ in 0..30 {
            predictor.observe(ResourceType::Cpu, 50.0).unwrap();
        }

        let action = predictor.recommend(ResourceType::Cpu).unwrap();
        assert_eq!(action, TuningAction::NoAction);
    }

    #[test]
    fn test_with_defaults() {
        let predictor = ResourcePredictor::with_defaults();
        assert_eq!(predictor.tracked_count(), 4);
    }

    #[test]
    fn test_prediction_horizons() {
        assert_eq!(PredictionHorizon::OneMinute.steps(), 6);
        assert_eq!(PredictionHorizon::FiveMinutes.steps(), 30);
        assert_eq!(PredictionHorizon::FifteenMinutes.steps(), 90);
        assert_eq!(PredictionHorizon::OneHour.steps(), 360);
        assert_eq!(PredictionHorizon::Custom(42).steps(), 42);
    }
}