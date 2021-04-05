use anyhow::{ensure, Context, Result};
use libchiron::config::Config;
use libchiron::scoring::report;
use libchiron::sys;
use std::fs;

fn main() -> Result<()> {
	ensure!(
		matches!(sys::check_trace(), Ok(false)),
		"Detected engine tracing, killing engine!"
	);

	let raw = fs::read_to_string("examples/scoring.toml")
		.context("Failed to read scoring config!")?;
	let config = toml::from_str::<Config>(&raw)
		.context("Failed to parse scoring config!")?;

	report::generate(config.score())
}
