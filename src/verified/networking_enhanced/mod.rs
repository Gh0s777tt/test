//! Advanced Networking Features for VANTIS OS v1.6.0
//!
//! This module provides enhanced networking capabilities including:
//! - Software-Defined Networking (SDN) controller with flow management
//! - QoS traffic shaping with token bucket rate limiting
//! - Zero-Trust Network Access (ZTNA) with trust score computation

pub mod sdn_controller;
pub mod traffic_shaper;
pub mod zero_trust_network;