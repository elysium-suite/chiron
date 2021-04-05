#![deny(missing_docs)]

//! Chiron is a vulnerability scoring engine designed for ease of use

/// Main directory for chiron and its data
const CHIRON_DIR: &str = "/opt/chiron/";

/// Architecture-specific funtions
pub mod sys;

/// Scorable checks
pub mod check;

/// Configuration parsing
pub mod config;

/// Scoring utilities
pub mod scoring;
