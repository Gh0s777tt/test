//! AI Modules for VantisOS
//! 
//! This module contains sub-modules for the AI data pipeline:
//! - **data_collector**: Collects raw system metrics
//! - **data_processor**: Processes and transforms data for ML
//! - **trainer**: Trains and validates ML models
//! 
//! ## Architecture
//! 
//! ```text
//! ┌─────────────────┐    ┌──────────────────┐    ┌─────────────┐
//! │ Data Collector  │ -> │ Data Processor   │ -> │   Trainer   │
//! │                 │    │                  │    │             │
//! │ - CPU metrics   │    │ - Feature        │    │ - Train     │
//! │ - Memory stats  │    │   extraction     │    │ - Validate  │
//! │ - Disk I/O      │    │ - Normalization  │    │ - Fine-tune │
//! │ - Network stats │    │ - Aggregation    │    │             │
//! └─────────────────┘    └──────────────────┘    └─────────────┘
//! ```
//! 
//! ## Security
//! - All data is processed locally
//! - No external network access
//! - Differential privacy support
//! - Memory-bounded operations

pub mod data_collector;
pub mod data_processor;
pub mod trainer;

pub use data_collector::DataCollector;
pub use data_processor::DataProcessor;
pub use trainer::ModelTrainer;