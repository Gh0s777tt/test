//! ML Algorithms Module
//! 
//! Provides machine learning algorithm implementations for VantisOS AI.
//! 
//! ## Modules
//! - `rl`: Reinforcement learning algorithms
//! - `optimization`: Optimization algorithms
//! - `forecasting`: Time-series forecasting
//! - `classification`: Classification algorithms
//! - `clustering`: Clustering algorithms

pub mod rl;
pub mod optimization;
pub mod forecasting;
pub mod classification;
pub mod clustering;
pub mod metrics;

pub use rl::*;
pub use optimization::*;
pub use forecasting::*;
pub use classification::*;
pub use clustering::*;
pub use metrics::*;