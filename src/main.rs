use anyhow::{ensure, Result};
use libchiron::{arch, config::Config};
use std::fs;

fn main() -> Result<()> {
	ensure!(
		matches!(arch::check_trace(), Ok(false)),
		"Detected engine tracing, killing engine!"
	);

	let raw = fs::read_to_string("examples/scoring.toml")?;
	let config = toml::from_str::<Config>(&raw)?;

	for scored in config.score() {
		println!("Check passed: {} - {} points", scored.name, scored.points);
	}
	Ok(())
}
