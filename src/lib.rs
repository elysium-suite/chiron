#![deny(missing_docs)]
#![feature(option_result_contains)]

//! Chiron is a vulnerability scoring engine designed for ease of use

/// Architecture-specific funtions
pub mod arch;

/// Scorable checks
pub mod checks;

/// Configuration parsing
pub mod config;

/// Crate-wide utility macros
pub mod macros;

/// Utility for Writing to Scoring Report
pub mod scoring;

/// Scoring Report File
pub const SCORING_REPORT: &str = "/opt/chiron/scoringreport.html";
