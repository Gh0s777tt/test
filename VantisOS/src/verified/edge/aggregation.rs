//! Data Aggregation
//! 
//! This module provides data aggregation functionality for edge computing including
//! data collection, aggregation, and summarization.

use core::sync::atomic::{AtomicU32, Ordering};

/// Aggregation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationType {
    Sum,
    Average,
    Min,
    Max,
    Count,
    Median,
    StandardDeviation,
    Custom,
}

/// Aggregation window
#[derive(Debug, Clone, Copy)]
pub struct AggregationWindow {
    pub start_time: u64,
    pub end_time: u64,
    pub size_ms: u64,
}

/// Aggregation result
#[derive(Debug, Clone, Copy)]
pub struct AggregationResult {
    pub aggregation_type: AggregationType,
    pub value: f64,
    pub count: u32,
    pub window: AggregationWindow,
}

/// Data aggregator
pub struct DataAggregator {
    data_points: Vec<DataPoint>,
    current_window: AggregationWindow,
}

/// Data point
#[derive(Debug, Clone, Copy)]
pub struct DataPoint {
    pub timestamp: u64,
    pub value: f64,
}

impl DataAggregator {
    /// Create a new data aggregator
    pub fn new(window_size_ms: u64) -> Self {
        let current_time = 0; // Placeholder
        let window = AggregationWindow {
            start_time: current_time,
            end_time: current_time + window_size_ms,
            size_ms: window_size_ms,
        };
        
        Self {
            data_points: Vec::new(),
            current_window: window,
        }
    }
    
    /// Initialize data aggregator
    pub fn init(&mut self) {
        // Initialize hardware-specific data aggregation
        // This is a placeholder for hardware-specific code
    }
    
    /// Add data point
    pub fn add_data_point(&mut self, value: f64, timestamp: u64) {
        let data_point = DataPoint { timestamp, value };
        self.data_points.push(data_point);
        
        // Check if we need to start a new window
        if timestamp >= self.current_window.end_time {
            self.start_new_window(timestamp);
        }
    }
    
    /// Aggregate data
    pub fn aggregate(&self, aggregation_type: AggregationType) -> Result<AggregationResult, AggregationError> {
        if self.data_points.is_empty() {
            return Err(AggregationError::NoData);
        }
        
        let value = match aggregation_type {
            AggregationType::Sum => self.calculate_sum(),
            AggregationType::Average => self.calculate_average(),
            AggregationType::Min => self.calculate_min(),
            AggregationType::Max => self.calculate_max(),
            AggregationType::Count => self.calculate_count() as f64,
            AggregationType::Median => self.calculate_median(),
            AggregationType::StandardDeviation => self.calculate_std_dev(),
            AggregationType::Custom => 0.0,
        };
        
        Ok(AggregationResult {
            aggregation_type,
            value,
            count: self.data_points.len() as u32,
            window: self.current_window,
        })
    }
    
    /// Aggregate multiple types
    pub fn aggregate_multiple(&self, types: &[AggregationType]) -> Result<Vec<AggregationResult>, AggregationError> {
        let mut results = Vec::new();
        
        for &aggregation_type in types {
            let result = self.aggregate(aggregation_type)?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Clear data points
    pub fn clear(&mut self) {
        self.data_points.clear();
    }
    
    /// Get data points count
    pub fn get_count(&self) -> usize {
        self.data_points.len()
    }
    
    /// Get current window
    pub fn get_current_window(&self) -> AggregationWindow {
        self.current_window
    }
    
    /// Start new window
    fn start_new_window(&mut self, timestamp: u64) {
        self.current_window = AggregationWindow {
            start_time: timestamp,
            end_time: timestamp + self.current_window.size_ms,
            size_ms: self.current_window.size_ms,
        };
        self.data_points.clear();
    }
    
    /// Calculate sum
    fn calculate_sum(&self) -> f64 {
        self.data_points.iter().map(|dp| dp.value).sum()
    }
    
    /// Calculate average
    fn calculate_average(&self) -> f64 {
        let sum = self.calculate_sum();
        let count = self.data_points.len() as f64;
        sum / count
    }
    
    /// Calculate min
    fn calculate_min(&self) -> f64 {
        self.data_points.iter().map(|dp| dp.value).fold(f64::INFINITY, |a, b| a.min(b))
    }
    
    /// Calculate max
    fn calculate_max(&self) -> f64 {
        self.data_points.iter().map(|dp| dp.value).fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }
    
    /// Calculate count
    fn calculate_count(&self) -> u32 {
        self.data_points.len() as u32
    }
    
    /// Calculate median
    fn calculate_median(&self) -> f64 {
        let mut sorted = self.data_points.clone();
        sorted.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        
        let len = sorted.len();
        if len % 2 == 0 {
            (sorted[len / 2 - 1].value + sorted[len / 2].value) / 2.0
        } else {
            sorted[len / 2].value
        }
    }
    
    /// Calculate standard deviation
    fn calculate_std_dev(&self) -> f64 {
        let average = self.calculate_average();
        let variance = self.data_points.iter()
            .map(|dp| (dp.value - average).powi(2))
            .sum::<f64>() / self.data_points.len() as f64;
        variance.sqrt()
    }
}

/// Aggregation error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationError {
    NoData,
    InvalidAggregationType,
    CalculationError,
}

/// Data aggregation state
static DATA_AGGREGATION_INITIALIZED: AtomicU32 = AtomicU32::new(0);

/// Initialize data aggregation
pub fn init() {
    if DATA_AGGREGATION_INITIALIZED.load(Ordering::SeqCst) == 0 {
        // Initialize hardware-specific data aggregation
        // This is a placeholder for hardware-specific code
        
        DATA_AGGREGATION_INITIALIZED.store(1, Ordering::SeqCst);
    }
}

/// Check if data aggregation is initialized
pub fn is_initialized() -> bool {
    DATA_AGGREGATION_INITIALIZED.load(Ordering::SeqCst) != 0
}

/// Get data aggregation version
pub fn get_version() -> &'static str {
    "Data Aggregation v0.7.0"
}

/// Default aggregation window
impl Default for AggregationWindow {
    fn default() -> Self {
        Self {
            start_time: 0,
            end_time: 60000,  // 1 minute
            size_ms: 60000,
        }
    }
}

/// Default aggregation result
impl Default for AggregationResult {
    fn default() -> Self {
        Self {
            aggregation_type: AggregationType::Average,
            value: 0.0,
            count: 0,
            window: AggregationWindow::default(),
        }
    }
}