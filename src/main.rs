use anyhow::{ensure, Result};
use libchiron::{arch, config::Config, scoring, SCORING_REPORT};
use std::fs;

fn main() -> Result<()> {
	ensure!(
		matches!(arch::check_trace(), Ok(false)),
		"Detected engine tracing, killing engine!"
	);

	let raw = fs::read_to_string("examples/scoring.toml")?;
	let config = toml::from_str::<Config>(&raw)?;

	scoring::write_to_scoring_report(SCORING_REPORT, config.score())
}
