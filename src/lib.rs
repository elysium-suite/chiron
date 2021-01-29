#![deny(missing_docs)]
#![feature(option_result_contains)]

//! Chiron is a vulnerability scoring engine designed for ease of use

/// Main directory for chiron and its data
#[macro_export]
macro_rules! chiron_dir {
	() => {
		"/opt/chiron/"
	};
}

/// Architecture-specific funtions
pub mod arch;

/// Scorable checks
pub mod checks;

/// Configuration parsing
pub mod config;

/// Crate-wide utility macros
pub mod macros;

/// Scoring utilities
pub mod scoring;
